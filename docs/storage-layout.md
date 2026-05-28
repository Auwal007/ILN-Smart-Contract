# Storage Layout

## 1. Overview
The ILN-Smart-Contract uses a unified, strongly typed storage architecture via the `DataKey` enum to guarantee storage slot isolation and prevent key collisions. By utilizing a single comprehensive enum for all persistent and instance storage needs across the contract ecosystem, we ensure:
- **Collision Safety**: Each enum variant serializes as a deterministic tuple `[Symbol(VariantName), ...args]`, preventing accidental overwrites.
- **Strong Typing**: Compiler guarantees ensure all key parameters are perfectly matching the required types.
- **Centralized Management**: All keys and storage helper access methods reside entirely in `storage.rs`.

## 2. Storage Categories

### Persistent Storage
Used for data that must survive indefinitely until explicitly deleted. In Soroban, persistent storage has dedicated TTL (Time-To-Live) extensions to prevent archival. We use persistent storage for Invoices, Reputations, Funding Queues, Appeals, and Statistics.

### Instance Storage
Used for high-frequency, global contract configurations that live as long as the contract instance itself. We use instance storage for Admin configurations, Fee Rates, Max Discount Rates, and the Paused switch.

### Temporary Storage
Used for short-lived data that is cheap but can be cleared out automatically by the network. Currently not used, but available for future ephemeral non-critical caching.

## 3. DataKey Enum Layout

| Variant | Storage Type | Stored Value | Purpose |
| ------- | ------------ | ------------ | ------- |
| `Admin` | Instance | `Address` | Stores the contract administrator |
| `Config` | Instance | `Config` | Stores system parameters (fees, decay logic) |
| `FeeRate` | Instance | `u32` | Stores the protocol fee rate |
| `MaxDiscountRate` | Instance | `u32` | Maximum allowed discount rate for an invoice |
| `DistributionContract` | Instance | `Address` | The external distribution contract |
| `Paused` | Instance | `bool` | Emergency pause flag |
| `Invoice(u64)` | Persistent | `Invoice` | Stores individual invoice states |
| `InvoiceCount` | Persistent | `u64` | Auto-incrementing counter for new invoices |
| `Token` | Persistent | `Address` | Primary token address |
| `PayerScore(Address)` | Persistent | `ReputationScore`| Tracks payer reputation and activity ledger |
| `InvoiceFunders(u64)` | Persistent | `Vec<(Address, i128)>` | Tracks LP funders for partial funding |
| `ApprovedToken(Address)` | Persistent | `bool` | Whitelisted tokens |
| `TokenList` | Persistent | `Vec<Address>` | Iteratable list of approved tokens |
| `Appeal(u64)` | Persistent | `AppealRecord` | Stores evidence hashes for defaults |
| `PreDefaultPayerScore(u64)`| Persistent| `u32` | Snapshot of payer score before a default penalty |
| `LpScore(Address)` | Persistent | `u32` | Tracks LP reputation |
| `FundQueue(u64)` | Persistent | `Vec<LpFundRequest>` | LPs waiting to fund an invoice |
| `QueueResolution(u64)`| Persistent | `Address` | Selected LP from a funding queue |
| `TotalInvoices` | Persistent | `u64` | Global protocol stat |
| `TotalFunded` | Persistent | `u64` | Global protocol stat |
| `TotalPaid` | Persistent | `u64` | Global protocol stat |
| `TotalVolumeUsdc` | Persistent | `i128` | Global protocol stat |
| `TotalVolumeEurc` | Persistent | `i128` | Global protocol stat |
| `TotalVolumeXlm` | Persistent | `i128` | Global protocol stat |

## 4. Collision Prevention Strategy
The unification into the `DataKey` enum inherently prevents key collision:
- In Soroban SDK, custom enum variants serialize into `ScVal::Vec` where the first element is the string literal (Symbol) of the variant name.
- By strictly disallowing the usage of raw strings `symbol_short!("foo")`, all keys are mapped to their respective enum variants.
- For dynamic keys like `Invoice(u64)`, the ID guarantees distinct vectors: `[Symbol("Invoice"), u64(1)]` vs `[Symbol("Invoice"), u64(2)]`.
- As a result, no overlap can ever occur between different keys.

## 5. Upgrade/Migration Notes
All previously used enums (`ConfigKey`, `StorageKey`) have been successfully merged into `DataKey` during this update. Data serialization maintains backwards compatibility since the names of the enum variants (e.g. `Admin`, `Invoice(u64)`) remain identical.
