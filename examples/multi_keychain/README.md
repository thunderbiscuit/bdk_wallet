# Multi-keychain examples

A `Wallet<K>` tracks any number of keychains, each identified by a value of type `K`. The default
is `KeychainKind`, which gives the familiar two-keychain wallet; supplying your own `K` gives you
as many keychains as you need.

The wallet requires `K: Ord + Clone + Debug` and nothing more, so `K` can carry whatever metadata
your application wants to attach to a keychain. `Ord` is required because keychains are held in a
`BTreeMap`, which also means it decides the order they iterate in.

| Example | Shows |
| --- | --- |
| [`custom_keychain.rs`](./custom_keychain.rs) | A keychain identifier that carries application metadata, and using it to choose which keychain to reveal from |
| [`address_generation.rs`](./address_generation.rs) | Per-keychain derivation indices, `reveal_next_address` vs `next_unused_address`, reading back the last revealed index |
| [`persistence.rs`](./persistence.rs) | Persisting a multi-keychain wallet to sqlite and loading it back, including load-time descriptor checks |

Run them with:

```shell
cargo run --example multi_keychain_custom_keychain
cargo run --example multi_keychain_address_generation
cargo run --example multi_keychain_persistence --features rusqlite
```

## Building a wallet

Keychains are added one at a time on the `CreateParams` builder returned by `Wallet::create`:

```rust,ignore
let mut wallet = Wallet::create(Network::Signet)
    .descriptor(Keychain::A, DESC_A)
    .descriptor(Keychain::B, DESC_B)
    .create_wallet_no_persist()?;
```

Each descriptor is validated against the network as it is added, and duplicate keychains and
duplicate descriptors are rejected. The builder chain itself never fails: the first error is kept
and returned as a `CreateError` when the wallet is created.

The conventional two-keychain wallet is the same two calls with `KeychainKind`:

```rust,ignore
let mut wallet = Wallet::create(Network::Signet)
    .descriptor(KeychainKind::External, external_desc)
    .descriptor(KeychainKind::Internal, internal_desc)
    .create_wallet_no_persist()?;
```

## Current limitations

One thing a multi-keychain wallet cannot do yet:

- **No transaction building.** `TxBuilder` lives on `impl Wallet<KeychainKind>`, because building a
  transaction has to pick a change keychain and a wallet generic over `K` has no canonical one.
  Relatedly, `balance()` counts nothing as trusted until it is mined.

To persist a custom keychain type, implement `rusqlite`'s `ToSql` and `FromSql` for it (see
[`persistence.rs`](./persistence.rs)); for the file store, implement `serde::Serialize` and
`serde::Deserialize`.
