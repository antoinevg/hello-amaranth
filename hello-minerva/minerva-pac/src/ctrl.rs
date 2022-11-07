#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub reset: RESET,
    #[doc = "0x04 - Use this register as a scratch space to verify that software read/write accesses to the Wishbone/CSR bus are working correctly. The initial reset value of 0x1234578 can be used to verify endianness."]
    pub scratch: SCRATCH,
    #[doc = "0x08 - Total number of Wishbone bus errors (timeouts) since start."]
    pub bus_errors: BUS_ERRORS,
}
#[doc = "RESET (rw) register accessor: an alias for `Reg<RESET_SPEC>`"]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = ""]
pub mod reset;
#[doc = "SCRATCH (rw) register accessor: an alias for `Reg<SCRATCH_SPEC>`"]
pub type SCRATCH = crate::Reg<scratch::SCRATCH_SPEC>;
#[doc = "Use this register as a scratch space to verify that software read/write accesses to the Wishbone/CSR bus are working correctly. The initial reset value of 0x1234578 can be used to verify endianness."]
pub mod scratch;
#[doc = "BUS_ERRORS (rw) register accessor: an alias for `Reg<BUS_ERRORS_SPEC>`"]
pub type BUS_ERRORS = crate::Reg<bus_errors::BUS_ERRORS_SPEC>;
#[doc = "Total number of Wishbone bus errors (timeouts) since start."]
pub mod bus_errors;
