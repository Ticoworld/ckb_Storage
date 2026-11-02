# WO02 — Radical alternatives: research boundary, not a fork implementation

## What current consensus cannot change at the application layer

The unchanged occupied-capacity rule charges the standard empty-data 20-byte-args
cell **61 CKB**. An application cannot create the same valid output with only one
CKB by adding a Merkle proof, type script, witness, off-chain agreement or fee
sponsor. ACP avoids a *new* cell's requirement by using existing capital; shared
claims change the representation. Neither alters that output-validity rule.

This is a specific consensus boundary, not proof that all current-consensus
small-balance self-custody is impossible. WO02 itself demonstrates conditional
self-custodial shared claims. Protocol economics matters if the target additionally
requires cheap, fresh, directly spendable L1 state rather than such representations.

The reference rules/economic rationale are the pinned CKB CapacityVerifier,
occupied-capacity extensions, mainnet specification, DAO calculation and RFC
0015 recorded in WO01. Token-denominated storage rights are an economic choice;
none of these sources proves that choice optimal. No consensus code, network
configuration or fork was changed for this research.

## Candidate 1: decoupled monetary balances and storage rights

**Rule changes.** Separate conserved native monetary amount from the right to
keep live bytes. Cell validity would require both sufficient monetary input/output
accounting and sufficient independently represented storage rights. Specify whether
rights are perpetual scarce byte units or expiring byte-epochs. Define issuance,
transfer, retirement, renewal and which chain state enforces the rights. Change
serialization/syscalls/indexers/SDKs and existing capacity-dependent scripts as
needed. A new application token cannot override today's global occupied-capacity
check, so merely minting a storage-credit token is not this proposal.

**Potential outcome.** A 1 CKB monetary cell could exist without also buying 60
additional native CKB if its bytes were paid by storage credits. Those credits
still have real scarcity and cost. Separating the markets does **not** prove that
credit prices stay affordable when demand rises, or that total system capital
and operating cost fall. Compare the price/collateral of credits, not just the
native-balance field.

**Migration.** Every existing CKB has an implicit storage option, including liquid
coins and DAO holdings, not merely currently occupied cells. Granting new rights
only to live-cell users would ignore others' existing rights. A conversion must
specify how all existing native holders receive equivalent options or redemption,
how locked contracts/DAO positions control them, how shared-pool beneficiaries
are represented, and how legacy scripts continue to operate. Giving credits to
a custodian while removing beneficiaries' guarantees is not rights preservation.
Avoid charging twice for the same grandfathered bytes or issuing multiple claims
on the same grandfathered entitlement. Model a staged, explicit legacy/new-mode
transition rather than silently changing field meanings for old contracts.

**Security.** Credit lending/reuse must not overbook bytes. Free issuance per
account is Sybil-subsidized unless a scarce resource pays for it. Credit price
oracles introduce manipulation/governance risks. A separated monetary market can
weaken the existing state-demand/miner-revenue relationship. Reserve the ability
to exit and retain monetary ownership if a credit provider disappears.

**Miner/node incentives.** Allocate charges to a specified security budget and
state-service mechanism; burning a fee or paying miners does not automatically
pay every validating or archival node. Bound aggregate live-state growth,
validation bandwidth and history-service costs. Explain why nodes continue to
validate/serve data when transaction volume or native-token price changes.
Provider competition or voluntary node operation is an assumption to test, not
a new incentive created by naming a credit.

## Candidate 2: term storage leases

**Rule changes.** Introduce authenticated paid-until epochs, byte-time accounting,
renewal and expiry handling. Consensus must specify whether expired state is
invalid, reclaimable, compacted into a commitment or merely unavailable to new
operations. Expiry cannot silently destroy a user's monetary principal and be
called equivalent custody. A lease contract under current rules may finance
capacity, but somebody still locks today's native capacity; that is capital
rental, not a change in the aggregate storage rule.

**Potential outcome.** Replacing a perpetual native-token bond with a limited
term's payment can lower upfront capital. It changes duration and renewal risk.
A perpetual unattended claim is not equivalent to a renewable 90-day claim.
An escape receipt/accumulator still needs persistent consensus representation,
anti-replay, backing, available preimages and an economically funded exit.
Replacing old cells with an opaque expired-state root does not solve availability.

**Migration.** Preserve legacy indefinite rights rather than expiring existing
cells by surprise. Voluntary new-format leases could coexist with old cells,
but all holders' native storage options and DAO accounting still require explicit
treatment. Specify address behavior for late deposits, recovery credentials,
notification/offline periods and who may pay renewal. An expiry date does not
execute a transaction or publish a missing exit proof automatically.

**Security/incentives.** Lease squatting, renewal censorship, fee spikes and
expiry auctions can turn rent collection into asset seizure. Long-offline users,
contract-held money and unavailable archives are hard cases. Rent rates must fund
real service/security costs under congestion and falling usage, and prevent
cheap short-term flood attacks. A landlord charging a fee while miners/nodes
bear the externalized storage cost is not an adequate incentive model.

## Candidate 3: alternative occupied-capacity conversion

**Rule changes.** Replace today's 1 CKB per occupied byte with an explicit
conversion `lambda` bytes per CKB (including deterministic rounding), or a
separate time/rent schedule. Update every consensus calculation dependent on
occupied capacity, DAO counted capacity, rewards/issuance treatment, RPC/tooling,
cell-creation and withdrawal assumptions. A monetary input/output conservation
rule must remain intact; changing byte pricing is not permission to mint native
balances. No such implementation was authorized or attempted here.

**Concrete arithmetic.** An unchanged standard layout would need
`61/lambda CKB` of minimum capacity before integer rounding. `lambda >= 61`
would let a 1 CKB balance underwrite that layout. This is a rule-change
calculation, not a safe economic parameter recommendation: all native storage
options could then support approximately 61 times more live bytes at the same
token supply if fully used. Token prices would also respond; holding price fixed
is not a monetary-equilibrium proof. At $10/CKB, a genuine 1 CKB balance itself
is still $10; removing the extra 60 CKB bond does not make its principal cheaper.

**Migration/security.** Increasing the ratio can expand legacy holders' usable
rights but dilutes the scarcity/security economics of previously priced capacity.
Decreasing it can make existing cells invalid unless grandfathered, which must
never happen through an undocumented accounting change. Avoid abrupt state-growth
shocks, unstable governance repricing and oracle-driven liquidation of old state.
Protocol changes cannot assume all full nodes suddenly have 61 times the resources.

**Incentives.** Evaluate native price/security budget, secondary issuance and DAO
in addition to raw disk. A high hypothetical native price might increase fiat
security income, but it does not prove sufficient decentralized node resources
or justify a permanent multiplier. Use bounded state growth and explicit service
cost/security assumptions, including downturns, rather than relying on “adoption
will pay for it.”

## Next falsifiable economic experiment

Before any fork design, build a parameterized **economic/state-growth model**,
with named assumptions and sensitivity ranges rather than a free-price claim:

1. Specify fresh direct-L1 1 CKB control and duration (perpetual versus renewable),
   the existing rights to preserve, admissible live-state growth, and security/
   archive-service budget requirements.
2. Compare legacy capacity, separate credits, lease schedules and conversion
   multipliers under identical demand, node-resource and price/security-budget
   stress cases. Count all collateral, rent, financing, renewal and exit costs.
3. Try to find a nonempty feasible range where upfront user capital falls,
   outstanding monetary/legacy rights remain intact, and specified state-growth
   and service/security constraints hold. Then attack Sybil demand, price crashes,
   data-provider failure, renewal censorship and mass expiry/exit.
4. Reject a candidate if affordability relies only on moving someone else's
   collateral off the balance sheet, confiscating legacy options, assuming free
   perpetual archives, unbounded hardware growth or an untested price oracle.

A simulated feasible range would justify a separate, explicitly authorized design
study; it would not prove market equilibrium, decentralization or fork readiness.
If no credible range exists under stated constraints, stop this direction. This
is the credible next falsifiable hypothesis behind the pivot, not a claim that
a different economic model already solves the problem.
