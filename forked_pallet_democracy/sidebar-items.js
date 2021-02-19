initSidebarItems({"constant":[["MAX_VETOERS","The maximum number of vetoers on a single proposal used to compute Weight."]],"enum":[["AccountVote","A vote for a referendum of a particular account."],["Call","Dispatchable calls."],["Conviction","A value denoting the strength of conviction of a vote."],["Error",""],["PreimageStatus",""],["RawEvent","Events for this module."],["ReferendumInfo","Info regarding a referendum, present or past."],["UnvoteScope","Whether an `unvote` operation is able to make actions that are not strictly always in the interest of an account."],["VoteThreshold","A means of determining if a vote is past pass threshold."],["Voting","An indicator for what an account is doing; it can either be delegating or voting."]],"struct":[["Blacklist","A record of who vetoed what. Maps proposal hash to a possible existent block number (until when it may not be resubmitted) and who vetoed it."],["Cancellations","Record of all proposals that have been subject to emergency cancellation."],["Delegations","Amount of votes and capital placed in delegation for an account."],["DepositOf","Those who have locked a deposit."],["GenesisConfig","Genesis config for the module, allow to build genesis storage."],["LastTabledWasExternal","True if the last referendum tabled was submitted externally. False if it was a public proposal."],["Locks","Accounts for which there are locks in action which may be removed at some point in the future. The value is the block number at which the lock expires and may be removed."],["LowestUnbaked","The lowest referendum index representing an unbaked referendum. Equal to `ReferendumCount` if there isn't an unbaked referendum."],["Module",""],["NextExternal","The referendum to be tabled whenever it would be valid to table an external proposal. This happens when a referendum needs to be tabled and one of two conditions are met:"],["Preimages","Map of hashes to the proposal preimage, along with who registered it and their deposit. The block number is the block at which it was deposited."],["PublicPropCount","The number of (public) proposals that have been made so far."],["PublicProps","The public proposals. Unsorted. The second item is the proposal's hash."],["ReferendumCount","The next free referendum index, aka the number of referenda started so far."],["ReferendumInfoOf","Information concerning any given referendum."],["ReferendumStatus","Info regarding an ongoing referendum."],["Tally","Info regarding an ongoing referendum."],["Vote","A number of lock periods, plus a vote, one way or the other."],["VotingOf","All votes for a particular voter. We store the balance for the number of votes that we have recorded. The second item is the total amount of delegations, that will be added."]],"trait":[["Approved",""],["Trait",""],["WeightInfo",""]],"type":[["BalanceOf",""],["Event","`RawEvent` specialized for the configuration `Trait`"],["NegativeImbalanceOf",""],["PropIndex","A proposal index."],["ReferendumIndex","A referendum index."]]});