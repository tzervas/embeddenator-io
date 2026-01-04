# embeddenator-io

Binary envelope format and serialization for Embeddenator engrams.

## Features

### Core
- **Binary Envelope Format**: EDN1 magic-number based envelope with header metadata
- **Payload Types**: EngramBincode, SubEngramBincode
- **Legacy Support**: Automatic detection and unwrapping of legacy (non-enveloped) data

### Optional Compression Codecs

Enable optional compression via feature flags:

```toml
[dependencies]
embeddenator-io = { version = "0.2", features = ["compression-zstd"] }
```

**Available Features:**
- `compression-zstd`: Zstandard compression (configurable levels -7 to 22)
- `compression-lz4`: LZ4 compression (fast, moderate ratio)
- `full-compression`: Enable all compression codecs

## Usage

### Basic Envelope (No Compression)

```rust
use embeddenator_io::*;

let data = b"my engram data";

// Wrap without compression
let wrapped = wrap_or_legacy(
    PayloadKind::EngramBincode,
    BinaryWriteOptions::default(),
    data,
)?;

// Unwrap automatically detects format
let unwrapped = unwrap_auto(PayloadKind::EngramBincode, &wrapped)?;
assert_eq!(unwrapped, data);
```

### With Compression (Zstd)

```rust
let opts = BinaryWriteOptions {
    codec: CompressionCodec::Zstd,
    level: Some(10), // Higher = better compression, slower
};

let wrapped = wrap_or_legacy(
    PayloadKind::EngramBincode,
    opts,
    data,
)?;

// Compressed size will be smaller for repetitive data
println!("Original: {} bytes", data.len());
println!("Compressed: {} bytes", wrapped.len());

// Unwrap automatically decompresses
let unwrapped = unwrap_auto(PayloadKind::EngramBincode, &wrapped)?;
```

### With Compression (LZ4)

```rust
let opts = BinaryWriteOptions {
    codec: CompressionCodec::Lz4,
    level: None, // LZ4 doesn't use compression levels
};

let wrapped = wrap_or_legacy(PayloadKind::SubEngramBincode, opts, data)?;
let unwrapped = unwrap_auto(PayloadKind::SubEngramBincode, &wrapped)?;
```

## Envelope Format

```
[4 bytes] Magic: "EDN1"
[1 byte]  Payload Kind (1=EngramBincode, 2=SubEngramBincode)
[1 byte]  Compression Codec (0=None, 1=Zstd, 2=Lz4)
[2 bytes] Reserved (0x0000)
[8 bytes] Uncompressed Size (little-endian u64)
[N bytes] Payload (compressed or raw)
```

## Performance

- **No Compression**: Zero overhead (returns raw bytes)
- **Zstd Level 3**: ~10-50% size reduction, fast
- **Zstd Level 10**: ~30-70% reduction, moderate speed
- **Zstd Level 22**: ~40-80% reduction, slow
- **LZ4**: ~20-40% reduction, very fast

## Testing

```bash
# Test without compression
cargo test --no-default-features

# Test with Zstd
cargo test --features compression-zstd

# Test with all codecs
cargo test --features full-compression
```

## Security Audit

**Status:** ✅ **No `unsafe` code**

All envelope and compression operations use safe Rust abstractions provided by:
- `std::io` for I/O operations
- `zstd` crate for Zstandard compression
- `lz4_flex` crate for LZ4 compression

## License

MIT

## Part of Embeddenator

This is a component of the [Embeddenator](https://github.com/tzervas/embeddenator) holographic computing substrate.
