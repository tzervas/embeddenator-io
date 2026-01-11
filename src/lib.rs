//! # embeddenator-io
//!
//! Envelope format and serialization for Embeddenator.
//!
//! Extracted from embeddenator core as part of Phase 2A component decomposition.

pub mod io;
pub use io::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_loads() {
        // Verify core types are accessible
        let _ = PayloadKind::EngramBincode;
        let _ = CompressionCodec::None;
        let opts = BinaryWriteOptions::default();
        assert_eq!(opts.codec, CompressionCodec::None);
    }
}
