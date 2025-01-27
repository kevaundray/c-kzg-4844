/* automatically generated by rust-bindgen 0.69.4 */

use libc::FILE;

pub const BYTES_PER_COMMITMENT: usize = 48;
pub const BYTES_PER_PROOF: usize = 48;
pub const BYTES_PER_FIELD_ELEMENT: usize = 32;
pub const BITS_PER_FIELD_ELEMENT: usize = 255;
pub const FIELD_ELEMENTS_PER_BLOB: usize = 4096;
pub const BYTES_PER_BLOB: usize = 131072;
pub const FIELD_ELEMENTS_PER_EXT_BLOB: usize = 8192;
pub const FIELD_ELEMENTS_PER_CELL: usize = 64;
pub const CELLS_PER_EXT_BLOB: usize = 128;
pub const BYTES_PER_CELL: usize = 2048;
pub type limb_t = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct blst_fr {
    l: [limb_t; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct blst_fp {
    l: [limb_t; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct blst_fp2 {
    fp: [blst_fp; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct blst_p1 {
    x: blst_fp,
    y: blst_fp,
    z: blst_fp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct blst_p1_affine {
    x: blst_fp,
    y: blst_fp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct blst_p2 {
    x: blst_fp2,
    y: blst_fp2,
    z: blst_fp2,
}
pub type g1_t = blst_p1;
pub type g2_t = blst_p2;
pub type fr_t = blst_fr;
#[doc = " An array of 32 bytes. Represents an untrusted\n (potentially invalid) field element."]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bytes32 {
    bytes: [u8; 32usize],
}
#[doc = " An array of 48 bytes. Represents an untrusted\n (potentially invalid) commitment/proof."]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bytes48 {
    bytes: [u8; 48usize],
}
#[doc = " A basic blob data."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Blob {
    bytes: [u8; 131072usize],
}
#[doc = " A single cell for a blob."]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Cell {
    bytes: [u8; 2048usize],
}
#[repr(C)]
#[doc = " The common return type for all routines in which something can go wrong."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum C_KZG_RET {
    #[doc = "< Success!"]
    C_KZG_OK = 0,
    #[doc = "< The supplied data is invalid in some way."]
    C_KZG_BADARGS = 1,
    #[doc = "< Internal error - this should never occur."]
    C_KZG_ERROR = 2,
    #[doc = "< Could not allocate memory."]
    C_KZG_MALLOC = 3,
}
#[doc = " Stores the setup and parameters needed for computing KZG proofs."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct KZGSettings {
    #[doc = " The length of `roots_of_unity`, a power of 2."]
    max_width: u64,
    #[doc = " Powers of the primitive root of unity determined by\n `SCALE2_ROOT_OF_UNITY` in bit-reversal permutation order,\n length `max_width`."]
    roots_of_unity: *mut fr_t,
    #[doc = " The expanded roots of unity."]
    expanded_roots_of_unity: *mut fr_t,
    #[doc = " The bit-reversal permuted roots of unity."]
    reverse_roots_of_unity: *mut fr_t,
    #[doc = " G1 group elements from the trusted setup,\n in monomial form."]
    g1_values_monomial: *mut g1_t,
    #[doc = " G1 group elements from the trusted setup,\n in Lagrange form bit-reversal permutation."]
    g1_values_lagrange_brp: *mut g1_t,
    #[doc = " G2 group elements from the trusted setup,\n in monomial form."]
    g2_values_monomial: *mut g2_t,
    #[doc = " Data used during FK20 proof generation."]
    x_ext_fft_columns: *mut *mut g1_t,
    #[doc = " The precomputed tables for fixed-base MSM."]
    tables: *mut *mut blst_p1_affine,
    #[doc = " The window size for the fixed-base MSM."]
    wbits: usize,
    #[doc = " The scratch size for the fixed-base MSM."]
    scratch_size: usize,
}
extern "C" {
    pub fn load_trusted_setup(
        out: *mut KZGSettings,
        g1_monomial_bytes: *const u8,
        num_g1_monomial_bytes: usize,
        g1_lagrange_bytes: *const u8,
        num_g1_lagrange_bytes: usize,
        g2_monomial_bytes: *const u8,
        num_g2_monomial_bytes: usize,
        precompute: usize,
    ) -> C_KZG_RET;
    pub fn load_trusted_setup_file(
        out: *mut KZGSettings,
        in_: *mut FILE,
        precompute: usize,
    ) -> C_KZG_RET;
    pub fn free_trusted_setup(s: *mut KZGSettings);
    pub fn blob_to_kzg_commitment(
        out: *mut KZGCommitment,
        blob: *const Blob,
        s: *const KZGSettings,
    ) -> C_KZG_RET;
    pub fn compute_kzg_proof(
        proof_out: *mut KZGProof,
        y_out: *mut Bytes32,
        blob: *const Blob,
        z_bytes: *const Bytes32,
        s: *const KZGSettings,
    ) -> C_KZG_RET;
    pub fn compute_blob_kzg_proof(
        out: *mut KZGProof,
        blob: *const Blob,
        commitment_bytes: *const Bytes48,
        s: *const KZGSettings,
    ) -> C_KZG_RET;
    pub fn verify_kzg_proof(
        ok: *mut bool,
        commitment_bytes: *const Bytes48,
        z_bytes: *const Bytes32,
        y_bytes: *const Bytes32,
        proof_bytes: *const Bytes48,
        s: *const KZGSettings,
    ) -> C_KZG_RET;
    pub fn verify_blob_kzg_proof(
        ok: *mut bool,
        blob: *const Blob,
        commitment_bytes: *const Bytes48,
        proof_bytes: *const Bytes48,
        s: *const KZGSettings,
    ) -> C_KZG_RET;
    pub fn verify_blob_kzg_proof_batch(
        ok: *mut bool,
        blobs: *const Blob,
        commitments_bytes: *const Bytes48,
        proofs_bytes: *const Bytes48,
        n: usize,
        s: *const KZGSettings,
    ) -> C_KZG_RET;
    pub fn compute_cells_and_kzg_proofs(
        cells: *mut Cell,
        proofs: *mut KZGProof,
        blob: *const Blob,
        s: *const KZGSettings,
    ) -> C_KZG_RET;
    pub fn recover_cells_and_kzg_proofs(
        recovered_cells: *mut Cell,
        recovered_proofs: *mut KZGProof,
        cell_indices: *const u64,
        cells: *const Cell,
        num_cells: usize,
        s: *const KZGSettings,
    ) -> C_KZG_RET;
    pub fn verify_cell_kzg_proof_batch(
        ok: *mut bool,
        commitments_bytes: *const Bytes48,
        cell_indices: *const u64,
        cells: *const Cell,
        proofs_bytes: *const Bytes48,
        num_cells: usize,
        s: *const KZGSettings,
    ) -> C_KZG_RET;
}
