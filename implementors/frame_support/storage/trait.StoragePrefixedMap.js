(function() {var implementors = {};
implementors["dock_runtime"] = [{"text":"impl&lt;T:&nbsp;Trait&gt; StoragePrefixedMap&lt;(KeyDetail, &lt;T as Trait&gt;::BlockNumber)&gt; for Dids&lt;T&gt;","synthetic":false,"types":[]}];
implementors["forked_pallet_democracy"] = [{"text":"impl&lt;T:&nbsp;Trait&gt; StoragePrefixedMap&lt;(Vec&lt;&lt;T as Trait&gt;::AccountId&gt;, &lt;&lt;T as Trait&gt;::Currency as Currency&lt;&lt;T as Trait&gt;::AccountId&gt;&gt;::Balance)&gt; for DepositOf&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Trait&gt; StoragePrefixedMap&lt;PreimageStatus&lt;&lt;T as Trait&gt;::AccountId, &lt;&lt;T as Trait&gt;::Currency as Currency&lt;&lt;T as Trait&gt;::AccountId&gt;&gt;::Balance, &lt;T as Trait&gt;::BlockNumber&gt;&gt; for Preimages&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Trait&gt; StoragePrefixedMap&lt;ReferendumInfo&lt;&lt;T as Trait&gt;::BlockNumber, &lt;T as Trait&gt;::Hash, &lt;&lt;T as Trait&gt;::Currency as Currency&lt;&lt;T as Trait&gt;::AccountId&gt;&gt;::Balance&gt;&gt; for ReferendumInfoOf&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Trait&gt; StoragePrefixedMap&lt;Voting&lt;&lt;&lt;T as Trait&gt;::Currency as Currency&lt;&lt;T as Trait&gt;::AccountId&gt;&gt;::Balance, &lt;T as Trait&gt;::AccountId, &lt;T as Trait&gt;::BlockNumber&gt;&gt; for VotingOf&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Trait&gt; StoragePrefixedMap&lt;&lt;T as Trait&gt;::BlockNumber&gt; for Locks&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Trait&gt; StoragePrefixedMap&lt;(&lt;T as Trait&gt;::BlockNumber, Vec&lt;&lt;T as Trait&gt;::AccountId&gt;)&gt; for Blacklist&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Trait&gt; StoragePrefixedMap&lt;bool&gt; for Cancellations&lt;T&gt;","synthetic":false,"types":[]}];
implementors["token_migration"] = [{"text":"impl&lt;T:&nbsp;Trait&gt; StoragePrefixedMap&lt;Bonus&lt;&lt;&lt;T as Trait&gt;::Currency as Currency&lt;&lt;T as Trait&gt;::AccountId&gt;&gt;::Balance, &lt;T as Trait&gt;::BlockNumber&gt;&gt; for Bonuses&lt;T&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()