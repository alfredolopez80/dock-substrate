// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use futures::prelude::*;
use sc_client_api::backend::{Backend as ClientBackend, Finalizer};
use sc_consensus_manual_seal::{rpc, rpc::CreatedBlock, rpc::EngineCommand, Error};
use sc_transaction_pool::txpool;
use sp_blockchain::HeaderBackend;
use sp_consensus::{
    self, import_queue::BoxBlockImport, BlockImport, BlockImportParams, BlockOrigin, Environment,
    ForkChoiceStrategy, ImportResult, Proposer, SelectChain,
};
use sp_inherents::InherentDataProviders;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, Header as HeaderT},
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// max duration for creating a proposal in secs
const MAX_PROPOSAL_DURATION: u64 = 10;

/// params for sealing a new block
pub struct SealBlockParams<'a, B: BlockT, SC, HB, E, T, P: txpool::ChainApi> {
    /// if true, empty blocks(without extrinsics) will be created.
    /// otherwise, will return Error::EmptyTransactionPool.
    pub create_empty: bool,
    /// instantly finalize this block?
    pub finalize: bool,
    /// specify the parent hash of the about-to-created block
    pub parent_hash: Option<<B as BlockT>::Hash>,
    /// sender to report errors/success to the rpc.
    pub sender: rpc::Sender<CreatedBlock<<B as BlockT>::Hash>>,
    /// transaction pool
    pub pool: Arc<txpool::Pool<P>>,
    /// header backend
    pub client: Arc<HB>,
    /// Environment trait object for creating a proposer
    pub env: &'a mut E,
    /// SelectChain object
    pub select_chain: &'a SC,
    /// block import object
    pub block_import: &'a mut BoxBlockImport<B, T>,
    /// inherent data provider
    pub inherent_data_provider: &'a InherentDataProviders,
}

// use sp_inherents::InherentDataProviders;
// use sp_runtime::{traits::Block as BlockT, Justification};
// use std::{marker::PhantomData, sync::Arc};

/// Creates the background authorship task for the manual seal engine.
pub async fn run_manual_seal<B, CB, E, C, A, SC, S, T>(
    mut block_import: BoxBlockImport<B, T>,
    mut env: E,
    client: Arc<C>,
    pool: Arc<txpool::Pool<A>>,
    mut commands_stream: S,
    select_chain: SC,
    inherent_data_providers: InherentDataProviders,
) where
    A: txpool::ChainApi<Block = B> + 'static,
    B: BlockT + 'static,
    C: HeaderBackend<B> + Finalizer<B, CB> + 'static,
    CB: ClientBackend<B> + 'static,
    E: Environment<B> + 'static,
    E::Error: std::fmt::Display,
    <E::Proposer as Proposer<B>>::Error: std::fmt::Display,
    S: Stream<Item = EngineCommand<<B as BlockT>::Hash>> + Unpin + 'static,
    SC: SelectChain<B> + 'static,
{
    while let Some(command) = commands_stream.next().await {
        match command {
            EngineCommand::SealNewBlock {
                create_empty,
                finalize,
                parent_hash,
                sender,
            } => {
                seal_new_block(SealBlockParams {
                    sender,
                    parent_hash,
                    finalize,
                    create_empty,
                    env: &mut env,
                    select_chain: &select_chain,
                    block_import: &mut block_import,
                    inherent_data_provider: &inherent_data_providers,
                    pool: pool.clone(),
                    client: client.clone(),
                })
                .await;
            }
            EngineCommand::FinalizeBlock {
                hash: _,
                sender: _,
                justification: _,
            } => unimplemented!(),
        }
    }
}

/// seals a new block with the given params
pub async fn seal_new_block<B, SC, HB, E, T, P>(
    SealBlockParams {
        create_empty,
        finalize,
        pool,
        parent_hash,
        client,
        select_chain,
        block_import,
        env,
        inherent_data_provider,
        mut sender,
        ..
    }: SealBlockParams<'_, B, SC, HB, E, T, P>,
) where
    B: BlockT,
    HB: HeaderBackend<B>,
    E: Environment<B>,
    <E as Environment<B>>::Error: std::fmt::Display,
    <E::Proposer as Proposer<B>>::Error: std::fmt::Display,
    P: txpool::ChainApi<Block = B>,
    SC: SelectChain<B>,
{
    let future = async {
        if pool.validated_pool().status().ready == 0 && !create_empty {
            return Err(Error::EmptyTransactionPool);
        }

        // get the header to build this new block on.
        // use the parent_hash supplied via `EngineCommand`
        // or fetch the best_block.
        let header = match parent_hash {
            Some(hash) => match client.header(BlockId::Hash(hash))? {
                Some(header) => header,
                None => return Err(Error::BlockNotFound(format!("{}", hash))),
            },
            None => select_chain.best_chain()?,
        };

        let proposer = env
            .init(&header)
            .map_err(|err| Error::StringError(format!("{}", err)))
            .await?;
        let id = inherent_data_provider.create_inherent_data()?;
        let inherents_len = id.len();
        // assert_eq!(inherents_len, 2);
        assert_eq!(pool.validated_pool().status().ready, 1);
        let proposal = proposer
            .propose(
                id,
                Default::default(),
                Duration::from_secs(MAX_PROPOSAL_DURATION),
                false.into(),
            )
            .map_err(|err| Error::StringError(format!("{}", err)))
            .await?;
        dbg!(&proposal.block);

        // assert_eq!(inherents_len, 2);
        // if proposal.block.extrinsics().len() == inherents_len && !create_empty {
        //     dbg!(pool.validated_pool().status());
        //     return Err(Error::EmptyTransactionPool);
        // }

        let (header, body) = proposal.block.deconstruct();
        let mut params = BlockImportParams::new(BlockOrigin::Own, header.clone());
        params.body = Some(body);
        params.finalized = finalize;
        params.fork_choice = Some(ForkChoiceStrategy::LongestChain);

        match block_import.import_block(params, HashMap::new())? {
            ImportResult::Imported(aux) => Ok(CreatedBlock {
                hash: <B as BlockT>::Header::hash(&header),
                aux,
            }),
            other => Err(other.into()),
        }
    };

    rpc::send_result(&mut sender, future.await)
}
