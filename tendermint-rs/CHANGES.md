## [0.9.0] (2019-06-24)

This release is compatible with [tendermint v0.31]

- Reject low order points in Secret Connection handshake ([#279])
- Add `RemoteErrorCode` enum ([#272])
- Add `msg_type()` accessor for signature types ([#271])

## [0.8.0] (2019-06-20)

This release is compatible with [tendermint v0.31]

- `/block_results` RPC endpoint and related types ([#267], [#268])
- Upgrade to Signatory v0.12 ([#259])

## [0.7.0] (2019-04-24)

This release is compatible with [tendermint v0.31]

- Initial JSONRPC over HTTP client + `/broadcast_tx_*` endpoints ([#243])
- Initial RPC support ([#235])
- Disallow a block height of 0 ([#234])

## [0.6.0] (2019-04-16)

This release is compatible with [tendermint v0.31]

- Add `tendermint::Address`, `tendermint::account::Id`, `tendermint::Moniker`,
  and improve `serde` serializer support ([#228]).

## [0.5.0] (2019-03-13)

This release is compatible with [tendermint v0.30]

- Rename `SecretConnectionKey` to `secret_connection::PublicKey`, add
  `secret_connection::PeerId` ([#219])
- Move `ConsensusState` under `chain::state` ([#205])

## 0.4.0 (N/A)

- Skipped to synchronize versions with `tmkms`

## 0.3.0 (2019-03-05)

- Support for secp256k1 keys ([#181])

## 0.2.0 (2019-01-23)

This release is compatible with [tendermint v0.29]

- Update to x25519-dalek v0.4.4 (#158)
- Consistent ordering of `BlockID` and `Timestamps` in vote and proposal messages (#159)
- Remove `PoisonPillMsg` previously used to shut-down the kms (#162)

## 0.1.5 (2019-01-18)

This release is compatible with [tendermint v0.28]

- Split `PubKeyMsg` into `PubKeyRequest` and `PubKeyResponse` (#141)
- Migrate to Rust 2018 edition (#138)
 
## 0.1.4 (2018-12-02)

- Allow empty BlockIds in validation method (#131)

## 0.1.3 (2018-12-01)

- Prefix bech32 encoding of consensus keys with amino prefix (#128)

## 0.1.2 (2018-11-27)

- Update to subtle-encoding v0.3 (#124)
- Introduce same validation logic as Tendermint (#110)
- Remove heartbeat (#105)

## 0.1.1 (2018-11-20)

- Minor clarifications/fixes (#103)

## 0.1.0 (2018-11-13)

- Initial release

[0.9.0]: https://github.com/tendermint/kms/pull/280
[#279]: https://github.com/tendermint/kms/pull/279
[#272]: https://github.com/tendermint/kms/pull/272
[#271]: https://github.com/tendermint/kms/pull/271
[0.8.0]: https://github.com/tendermint/kms/pull/269
[#268]: https://github.com/tendermint/kms/pull/268
[#267]: https://github.com/tendermint/kms/pull/267
[#259]: https://github.com/tendermint/kms/pull/259
[0.7.0]: https://github.com/tendermint/kms/pull/247
[#243]: https://github.com/tendermint/kms/pull/243
[#235]: https://github.com/tendermint/kms/pull/235
[#234]: https://github.com/tendermint/kms/pull/234
[0.6.0]: https://github.com/tendermint/kms/pull/229
[tendermint v0.31]: https://github.com/tendermint/tendermint/blob/master/CHANGELOG.md#v0310
[#228]: https://github.com/tendermint/kms/pull/228
[0.5.0]: https://github.com/tendermint/kms/pull/220
[tendermint v0.30]: https://github.com/tendermint/tendermint/blob/master/CHANGELOG.md#v0300
[#219]: https://github.com/tendermint/kms/pull/219
[#205]: https://github.com/tendermint/kms/pull/219
[#181]: https://github.com/tendermint/kms/pull/181
[tendermint v0.29]: https://github.com/tendermint/tendermint/blob/master/CHANGELOG.md#v0290
[tendermint v0.28]: https://github.com/tendermint/tendermint/blob/master/CHANGELOG.md#v0280
