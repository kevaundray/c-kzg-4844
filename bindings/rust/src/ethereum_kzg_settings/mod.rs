use crate::KzgSettings;
use alloc::{boxed::Box, sync::Arc};
use once_cell::race::OnceBox;

/// Default G1 Lagrange bytes.
const ETH_G1_MONOMIAL_POINTS: &[u8] = include_bytes!("./g1_monomial_bytes.bin");
/// Default G1 Lagrange bytes.
const ETH_G1_LAGRANGE_POINTS: &[u8] = include_bytes!("./g1_lagrange_bytes.bin");
/// Default G2 monomial bytes.
const ETH_G2_MONOMIAL_POINTS: &[u8] = include_bytes!("./g2_monomial_bytes.bin");

/// Returns default Ethereum mainnet KZG settings.
///
/// If you need a cloneable settings use `ethereum_kzg_settings_arc` instead.
pub fn ethereum_kzg_settings(precompute: usize) -> &'static KzgSettings {
    ethereum_kzg_settings_inner(precompute).as_ref()
}

/// Returns default Ethereum mainnet KZG settings as an `Arc`.
///
/// It is useful for sharing the settings in multiple places.
pub fn ethereum_kzg_settings_arc(precompute: usize) -> Arc<KzgSettings> {
    ethereum_kzg_settings_inner(precompute).clone()
}

fn ethereum_kzg_settings_inner(precompute: usize) -> &'static Arc<KzgSettings> {
    static DEFAULT: OnceBox<Arc<KzgSettings>> = OnceBox::new();
    DEFAULT.get_or_init(|| {
        let settings = KzgSettings::load_trusted_setup(
            ETH_G1_MONOMIAL_POINTS,
            ETH_G1_LAGRANGE_POINTS,
            ETH_G2_MONOMIAL_POINTS,
            precompute,
        )
        .expect("failed to load default trusted setup");
        Box::new(Arc::new(settings))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bindings::BYTES_PER_BLOB, Blob, KzgCommitment, KzgProof, KzgSettings};
    use std::path::Path;

    #[test]
    pub fn compare_default_with_file() {
        let precompute = 0;
        let ts_settings =
            KzgSettings::load_trusted_setup_file(Path::new("src/trusted_setup.txt"), precompute)
                .unwrap();
        let eth_settings = ethereum_kzg_settings(precompute);
        let blob = Blob::new([1u8; BYTES_PER_BLOB]);

        // generate commitment
        let ts_commitment = KzgCommitment::blob_to_kzg_commitment(&blob, &ts_settings)
            .unwrap()
            .to_bytes();
        let eth_commitment = KzgCommitment::blob_to_kzg_commitment(&blob, &eth_settings)
            .unwrap()
            .to_bytes();
        assert_eq!(ts_commitment, eth_commitment);

        // generate proof
        let ts_proof = KzgProof::compute_blob_kzg_proof(&blob, &ts_commitment, &ts_settings)
            .unwrap()
            .to_bytes();
        let eth_proof = KzgProof::compute_blob_kzg_proof(&blob, &eth_commitment, &eth_settings)
            .unwrap()
            .to_bytes();
        assert_eq!(ts_proof, eth_proof);
    }
}
