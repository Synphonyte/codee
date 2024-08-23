# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-08-23

### Breaking Changes

- Trait `IsBinary` has been removed.
- Instead, the traits `HybridEncoder` and `HybridDecoder` now provide the methods `is_binary_encoder` and
  `is_binary_decoder` respectively.

## [0.1.2] - 2024-07-08

### New Codecs

- Added `MiniserdeCodec`
- Added `SerdeLite` wrapper for serde based codecs
- Added `JsonSerdeWasmCodec`

## [0.1.1] - 2024-07-07

### New Codec

- Added `RkyvCodec`

## [0.1.0] - 2024-07-07

Initial release.
