# WO01 — First-principles questions

Each answer below distinguishes the necessary property from its current encoding.
Evidence: [consensus boundaries](WO01_CONSENSUS_BOUNDARIES.md), [experiments](WO01_EXPERIMENT_RESULTS.md)
and [pinned primary sources](WO01_SOURCES.md).

## A. Must each user own persistent state?

**IF** “independently authenticated” means a transaction can prove one user's
claim, **THEN** a single authenticated root can represent many users. Consensus
charges bytes, not users. Four different Ed25519 keys exit one root without an
operator signature in the executable pool. **WHY** doesn't this settle the
mission? Authentication alone does not establish funding, current proof
availability, concurrency or complete recovery semantics.

**IF** users demand different L1 transactions concurrently consume the same
root, **ELSE** only one wins that outpoint. This is a UTXO consensus rule, not a
Merkle cryptography failure. A coordinator can aggregate exits into one atomic
update, but its failure cannot be allowed to remove a user's unilateral route.
Our one-exit prototype rejects multi-input flows and does not implement batching.
Sharding into S cells trades O(S) state/deployment details for up to S independent
state lanes; it does not establish a fixed per-user cell requirement.

**WHAT IF** each user holds a proof rather than the whole database? For a fixed
withdrawal-only tree, public exit proofs disclose enough information to update
other users' proofs at their index divergence. We implemented this after stale
proofs failed. It works against the real VM, and 240 distinct index pairs match
full-tree recomputation. Thus “any changed root always needs an operator database”
is too strong. **HOW** does the user get the initial proof, missed history and
valid funding? Those remain explicit prerequisites.

## B. Must recovery create storage?

**IF** a recovery key is precommitted in the original authorization policy,
**THEN** a spend can use it directly and put the funds under a new policy. The
existing system 1-of-2 multisig does this with a 20-byte policy hash and 61 CKB
asset cell. Our custom 32-byte commitment costs 73 CKB and is worse for the same
narrow authority model. This is an implementation convention we can eliminate,
not an architectural invention.

**ELSE IF** recovery is expected to revoke a device across unspent cells and
unknown future deposits, **THEN** the verifier needs a current authenticated fact
that those cells' original scripts consult. Possibilities include an existing
asset's state, a shared registry root, updating every affected cell, or a policy
with scheduled expiry. An attacker cannot be forced to supply a new witness
simply because a wallet UI considers an old key revoked.

**WHAT IF** the owner key is compromised before recovery? Both 1-of-2 and our
custom policy let it spend immediately. Both also let a compromised recovery key
spend. They provide loss recovery, not theft-resistant delayed social recovery.
Loss of all authorized secrets has no bypass in these contracts. Delay,
challenge and threshold policies have different semantics and must be compared
with equivalent implementations; their extra state is not automatically waste.

## C. Must application policy be separate?

A lock can commit static policy; a type can validate an asset state transition;
a witness can reveal authenticated policy preimages. There is no general need
for a stand-alone policy cell. An existing CKB balance may already underwrite
the bytes, so shrinking occupied state need not change required token holdings.

A **per-transaction** spending ceiling is stateless. A **per-day aggregate** ceiling
must also prevent spending through multiple cells, signatures or transactions.
Without an authenticated shared counter, preallocated disjoint allowances,
nonces/nullifiers, or equivalent state, each transaction cannot determine what
other accepted transactions already spent. This is a statement about the named
security property, not a proof that each user needs a cell. A root can contain
many counters, with proof-availability and concurrency costs.

Retiring a policy is safe only when all obligations it mediates have ended or a
successor policy is enforced by every affected asset. Late deposits to an old
lock are a counterexample to “delete the registry and revoke everything.” Our
executable test accepts an old owner's spend of such a deposit after another
cell was recovered. Global application limits/revocation were **not** implemented.

## D. What must remain represented?

For this pool: the current unspent outpoint, funding capacity, immutable script
identity, and a binding root remain live. Public keys, balances and proofs are
witness-supplied; the root authenticates them. Every exit atomically replaces the
leaf by a domain-separated spent marker and preserves the other subtrees.
Double claims on the successor fail; old-input replay fails in the resolver.

A fixed-size hash can authenticate a large random dataset under collision
resistance. It cannot losslessly reconstruct that dataset. If every holder loses
its proof and all archives/providers disappear, the root does not recover it.
The available-information requirement can be distributed or made publicly
reconstructible, rather than duplicated in every live cell. Public availability
is a separate condition from valid execution and proof soundness.

Channels retain latest signed states and timeout/challenge machinery; rollups
retain roots plus their publication/exit mechanisms; validiums can verify state
transitions while withholding the data users need. None gets “self-custody under
operator disappearance” for free from its label. We do not equate a SNARK of a
valid transition with availability or unilateral exit liveness.

## E. Can identity be cheaper?

An empty-argument script can authenticate a key compiled into its code: our
41 CKB live cell passes real signatures. **WHY** isn't that a 20 CKB/user saving?
For independent keys the identity has moved to separate code deployments, about
44,677 CKB each in this implementation. Reusing one embedded key shares authority,
not independent user ownership. Reusing one code cell but storing different
identities in its type/dependency moves the state elsewhere again.

Shorter hashes trade security for bytes unless another authenticated field
already binds the identity. A b-bit truncated identifier has generic preimage
cost about 2^b, but adversarial multi-target or collision settings can demand
less work; one cannot label a 4-byte address secure just because it fits a cell.
The precise binding and attacker goals matter. No universal information-theoretic
20-byte-args lower bound was established: the code hash and other state may carry
that binding. Witness-only self-declared keys with no such anchor authenticate
whoever the attacker chooses.

## F. Is token-denominated capacity the real constraint?

At $0.01/$1/$10, a 61 CKB cell ties up $0.61/$61/$610 before fees; price changes
neither its bytes nor its authorization. Pooling 100 CKB/user does not remove
100 CKB/user of native monetary principal. Pooling tiny claims changes storage
capital, but cannot by itself fund simultaneous new independent 61 CKB exits.
Depositing into an **existing** ACP cell or shared settlement target can avoid a
new output's incremental floor; the existing recipient infrastructure must then
be counted. Our prototype does not implement that route.

Separate storage credits, term leases, expiry and repricing are economic/protocol
hypotheses, not cryptographic shortcuts. A credit system must specify issuance,
collateral, node compensation, growth limits, renewal/eviction, forced exits and
existing holders' rights. Free per-person quotas additionally require a scarce
identity resource or an explicit Sybil subsidy. We model tradeoffs without
claiming a proved optimal economic design.
