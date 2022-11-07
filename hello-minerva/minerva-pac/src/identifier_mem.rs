#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 8 x 39-bit memory"]
    pub identifier_mem: IDENTIFIER_MEM,
}
#[doc = "IDENTIFIER_MEM (rw) register accessor: an alias for `Reg<IDENTIFIER_MEM_SPEC>`"]
pub type IDENTIFIER_MEM = crate::Reg<identifier_mem::IDENTIFIER_MEM_SPEC>;
#[doc = "8 x 39-bit memory"]
pub mod identifier_mem;
