# Issuance, storage rights, and the missing live snapshot

## Verified schedule values

The pinned CKB mainnet genesis configuration has 665 issued cells totaling **33,598,735,037 CKB**. Separately listed genesis system cells total about **1,264,617 CKB**. The values are read from the pinned `resource/specs/mainnet.toml`; the genesis cells together are about 33.6 billion CKB, subject to the distinction between issued and system-cell capacities.

The pinned consensus source defines a secondary epoch reward of 61,369,863,013,698 shannons and a four-hour target epoch. That yields 2,190 target epochs per year and the source's stated **1.344 billion CKB secondary issuance per year**. The initial primary issuance rate is 4.2 billion CKB/year and the halving interval is four years; the finite primary schedule totals 33.6 billion CKB. These are protocol schedule values, not current circulating supply or miner revenue observations.

WO03 used a **45 billion CKB legacy entitlement proxy** as an explicit placeholder. WO04 does not carry that number forward: no pinned source or accessible mainnet snapshot verified it as current outstanding supply. The new machine-readable model sets current total supply and exercisable legacy capacity to unknown. Legacy entitlement must be bounded from an actual mainnet snapshot or an exact issuance-height calculation, not a rounded proxy.

## What a capacity conversion would change

If capacity for new outputs changes from one CKB per byte to `r` bytes per CKB, the same one CKB monetary output may require only `ceil(61/r)` CKB total capacity. At `r=61`, the capital collateral beyond the 1 CKB principal disappears for this output. The existing native-capacity guarantee is also the holder's right to create future cells under current rules. A fork must specify whether that right persists for all existing CKB, is exchanged for credits, or is repriced. Preserving all old rights while selling the same bytes more cheaply duplicates claims unless the global maximum is reserved or a priority rule changes access.

The miner-side state incentive is not a separate free budget. RFC 0015 ties secondary issuance allocation to occupied versus unoccupied capacity and explains DAO compensation for unoccupied capacity. Current DAO code subtracts occupied capacity before applying its exchange-rate calculation. A new conversion could change occupied accounting, state-linked miner income, or DAO rewards. WO04 verifies the fixed issuance schedules but cannot compute today's occupied share or USD security budget without the actual chain state and market price. The model therefore does not infer the required miner income or assume a rent/credit payment can replace it.

## Rights-preservation boundary

A static snapshot of currently occupied bytes would fail to preserve unoccupied holders' future output option. Reserving all capacity implied by every outstanding CKB preserves more of that option, but can consume a large part of the safe global byte budget even when holders are dormant. A cheaper token class, a term lease, or a market storage credit does not remove this conflict. It allocates it through an exchange rule, an expiry rule, or a market-clearing price. No migration that preserves independent redemption by long-offline holders was tested here.

The exact genesis totals and schedule constants replace the WO03 toy supply input where source facts exist. They do not replace current total supply, current occupied capacity, live cells, global storage demand, or current miner revenue; those require an accessible and verifiable mainnet state snapshot.
