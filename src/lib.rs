//! # embeddenator-io
//!
//! Envelope format and serialization for Embeddenator.
//!
//! Extracted from embeddenator core as part of Phase 2A component decomposition.

pub mod io;
pub use io::*;

#[cfg(test)]
mod tests {
    #[test]
    fn component_loads() {
        assert!(true);
    }
}
