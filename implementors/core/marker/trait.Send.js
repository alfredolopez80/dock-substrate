(function() {var implementors = {};
implementors["dock_node"] = [{"text":"impl&lt;B&gt; Send for GrandpaDeps&lt;B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;C, P, B&gt; Send for FullDeps&lt;C, P, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Send + Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for Executor","synthetic":true,"types":[]}];
implementors["dock_runtime"] = [{"text":"impl Send for BlockHashCount","synthetic":true,"types":[]},{"text":"impl Send for MaximumBlockWeight","synthetic":true,"types":[]},{"text":"impl Send for AvailableBlockRatio","synthetic":true,"types":[]},{"text":"impl Send for MaximumExtrinsicWeight","synthetic":true,"types":[]},{"text":"impl Send for BlockExecutionWeight","synthetic":true,"types":[]},{"text":"impl Send for MaximumBlockLength","synthetic":true,"types":[]},{"text":"impl Send for Version","synthetic":true,"types":[]},{"text":"impl Send for MinimumPeriod","synthetic":true,"types":[]},{"text":"impl Send for ExistentialDeposit","synthetic":true,"types":[]},{"text":"impl Send for MaxLocks","synthetic":true,"types":[]},{"text":"impl Send for TransactionByteFee","synthetic":true,"types":[]},{"text":"impl Send for MaxBlobSize","synthetic":true,"types":[]},{"text":"impl Send for StorageWeight","synthetic":true,"types":[]},{"text":"impl Send for VestingMilestones","synthetic":true,"types":[]},{"text":"impl Send for VestingDuration","synthetic":true,"types":[]},{"text":"impl Send for UncleGenerations","synthetic":true,"types":[]},{"text":"impl Send for CouncilMotionDuration","synthetic":true,"types":[]},{"text":"impl Send for CouncilMaxProposals","synthetic":true,"types":[]},{"text":"impl Send for CouncilMaxMembers","synthetic":true,"types":[]},{"text":"impl Send for TechnicalMotionDuration","synthetic":true,"types":[]},{"text":"impl Send for TechnicalMaxProposals","synthetic":true,"types":[]},{"text":"impl Send for TechnicalMaxMembers","synthetic":true,"types":[]},{"text":"impl Send for MaxScheduledPerBlock","synthetic":true,"types":[]},{"text":"impl Send for EnactmentPeriod","synthetic":true,"types":[]},{"text":"impl Send for LaunchPeriod","synthetic":true,"types":[]},{"text":"impl Send for VotingPeriod","synthetic":true,"types":[]},{"text":"impl Send for FastTrackVotingPeriod","synthetic":true,"types":[]},{"text":"impl Send for MinimumDeposit","synthetic":true,"types":[]},{"text":"impl Send for PreimageByteDeposit","synthetic":true,"types":[]},{"text":"impl Send for MaxVotes","synthetic":true,"types":[]},{"text":"impl Send for MaxProposals","synthetic":true,"types":[]},{"text":"impl Send for InstantAllowed","synthetic":true,"types":[]},{"text":"impl Send for BaseFilter","synthetic":true,"types":[]},{"text":"impl Send for Runtime","synthetic":true,"types":[]},{"text":"impl !Send for Origin","synthetic":true,"types":[]},{"text":"impl Send for PalletInfo","synthetic":true,"types":[]},{"text":"impl Send for GenesisConfig","synthetic":true,"types":[]},{"text":"impl Send for RuntimeApi","synthetic":true,"types":[]},{"text":"impl Send for StateChange","synthetic":true,"types":[]},{"text":"impl Send for Event","synthetic":true,"types":[]},{"text":"impl Send for OriginCaller","synthetic":true,"types":[]},{"text":"impl Send for Call","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Module&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Error&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;AccountId, BlockNumber, Hash&gt; Send for RawEvent&lt;AccountId, BlockNumber, Hash&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Hash: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for Attestation","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Module&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Error&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for Blob","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Module&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for BlobError&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for Bytes32","synthetic":true,"types":[]},{"text":"impl Send for Bytes33","synthetic":true,"types":[]},{"text":"impl Send for Bytes64","synthetic":true,"types":[]},{"text":"impl Send for Bytes65","synthetic":true,"types":[]},{"text":"impl Send for KeyDetail","synthetic":true,"types":[]},{"text":"impl Send for KeyUpdate","synthetic":true,"types":[]},{"text":"impl Send for DidRemoval","synthetic":true,"types":[]},{"text":"impl Send for GenesisConfig","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Dids&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Module&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Error&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for PublicKey","synthetic":true,"types":[]},{"text":"impl Send for DidSignature","synthetic":true,"types":[]},{"text":"impl Send for Event","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for Payload","synthetic":true,"types":[]},{"text":"impl Send for Membership","synthetic":true,"types":[]},{"text":"impl Send for GenesisConfig","synthetic":true,"types":[]},{"text":"impl Send for Members","synthetic":true,"types":[]},{"text":"impl Send for Round","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Module&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for MasterError&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;Call&gt; Send for RawEvent&lt;Call&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Call: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::Call: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for Registry","synthetic":true,"types":[]},{"text":"impl Send for Revoke","synthetic":true,"types":[]},{"text":"impl Send for UnRevoke","synthetic":true,"types":[]},{"text":"impl Send for RemoveRegistry","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Module&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for Policy","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for RevErr&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for TxnFee&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for SessionKeys","synthetic":true,"types":[]},{"text":"impl&lt;Block:&nbsp;BlockT, C:&nbsp;CallApiAt&lt;Block&gt;&gt; Send for RuntimeApiImpl&lt;Block, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C::StateBackend: StateBackend&lt;HashFor&lt;Block&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["forked_pallet_democracy"] = [{"text":"impl&lt;Balance&gt; Send for Delegations&lt;Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;BlockNumber, Hash, Balance&gt; Send for ReferendumStatus&lt;BlockNumber, Hash, Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Hash: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;Balance&gt; Send for Tally&lt;Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for Vote","synthetic":true,"types":[]},{"text":"impl Send for GenesisConfig","synthetic":true,"types":[]},{"text":"impl Send for PublicPropCount","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for PublicProps&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for DepositOf&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Preimages&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for ReferendumCount","synthetic":true,"types":[]},{"text":"impl Send for LowestUnbaked","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for ReferendumInfoOf&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for VotingOf&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Locks&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for LastTabledWasExternal","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for NextExternal&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Blacklist&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Cancellations&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Module&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for Conviction","synthetic":true,"types":[]},{"text":"impl&lt;BlockNumber, Hash, Balance&gt; Send for ReferendumInfo&lt;BlockNumber, Hash, Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Hash: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for UnvoteScope","synthetic":true,"types":[]},{"text":"impl&lt;Balance&gt; Send for AccountVote&lt;Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;Balance, AccountId, BlockNumber&gt; Send for Voting&lt;Balance, AccountId, BlockNumber&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for VoteThreshold","synthetic":true,"types":[]},{"text":"impl&lt;AccountId, Balance, BlockNumber&gt; Send for PreimageStatus&lt;AccountId, Balance, BlockNumber&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;Balance, AccountId, Hash, BlockNumber&gt; Send for RawEvent&lt;Balance, AccountId, Hash, BlockNumber&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Hash: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Error&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Trait&gt;::Currency as Currency&lt;&lt;T as Trait&gt;::AccountId&gt;&gt;::Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::BlockNumber: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::Currency: Currency&lt;&lt;T as Trait&gt;::AccountId&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::Hash: Send,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["poa"] = [{"text":"impl&lt;Balance&gt; Send for EpochDetail&lt;Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;Balance&gt; Send for ValidatorStatsPerEpoch&lt;Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for GenesisConfig&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Trait&gt;::Currency as Currency&lt;&lt;T as Trait&gt;::AccountId&gt;&gt;::Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::Currency: Currency&lt;&lt;T as Trait&gt;::AccountId&gt;,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Module&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;AccountId, BlockNumber, Balance&gt; Send for RawEvent&lt;AccountId, BlockNumber, Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Error&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Trait&gt;::Currency as Currency&lt;&lt;T as Trait&gt;::AccountId&gt;&gt;::Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::Currency: Currency&lt;&lt;T as Trait&gt;::AccountId&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::Keys: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::Lookup: StaticLookup,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Trait&gt;::Lookup as StaticLookup&gt;::Source: Send,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["poa_rpc"] = [{"text":"impl&lt;BlockHash, AccountId, Balance&gt; Send for Client&lt;BlockHash, AccountId, Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockHash: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;C, P&gt; Send for PoA&lt;C, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Send + Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Send,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["simple_democracy"] = [{"text":"impl&lt;T&gt; Send for Module&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for Event","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Error&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Trait&gt;::Currency as Currency&lt;&lt;T as Trait&gt;::AccountId&gt;&gt;::Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::BlockNumber: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::Hash: Send,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["token_migration"] = [{"text":"impl&lt;Balance, BlockNumber&gt; Send for Bonus&lt;Balance, BlockNumber&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Bonuses&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Module&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for OnlyMigrator&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;AccountId, Balance, BlockNumber&gt; Send for RawEvent&lt;AccountId, Balance, BlockNumber&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Error&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::AccountId: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Trait&gt;::Currency as Currency&lt;&lt;T as Trait&gt;::AccountId&gt;&gt;::Balance: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::Currency: Currency&lt;&lt;T as Trait&gt;::AccountId&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Trait&gt;::Lookup: StaticLookup,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Trait&gt;::Lookup as StaticLookup&gt;::Source: Send,&nbsp;</span>","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()