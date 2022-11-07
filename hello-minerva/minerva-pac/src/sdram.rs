#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control DFI signals common to all phases"]
    pub dfii_control: DFII_CONTROL,
    #[doc = "0x04 - Control DFI signals on a single phase"]
    pub dfii_pi0_command: DFII_PI0_COMMAND,
    #[doc = "0x08 - "]
    pub dfii_pi0_command_issue: DFII_PI0_COMMAND_ISSUE,
    #[doc = "0x0c - DFI address bus"]
    pub dfii_pi0_address: DFII_PI0_ADDRESS,
    #[doc = "0x10 - DFI bank address bus"]
    pub dfii_pi0_baddress: DFII_PI0_BADDRESS,
    #[doc = "0x14 - DFI write data bus"]
    pub dfii_pi0_wrdata: DFII_PI0_WRDATA,
    #[doc = "0x18 - DFI read data bus"]
    pub dfii_pi0_rddata: DFII_PI0_RDDATA,
}
#[doc = "DFII_CONTROL (rw) register accessor: an alias for `Reg<DFII_CONTROL_SPEC>`"]
pub type DFII_CONTROL = crate::Reg<dfii_control::DFII_CONTROL_SPEC>;
#[doc = "Control DFI signals common to all phases"]
pub mod dfii_control;
#[doc = "DFII_PI0_COMMAND (rw) register accessor: an alias for `Reg<DFII_PI0_COMMAND_SPEC>`"]
pub type DFII_PI0_COMMAND = crate::Reg<dfii_pi0_command::DFII_PI0_COMMAND_SPEC>;
#[doc = "Control DFI signals on a single phase"]
pub mod dfii_pi0_command;
#[doc = "DFII_PI0_COMMAND_ISSUE (rw) register accessor: an alias for `Reg<DFII_PI0_COMMAND_ISSUE_SPEC>`"]
pub type DFII_PI0_COMMAND_ISSUE = crate::Reg<dfii_pi0_command_issue::DFII_PI0_COMMAND_ISSUE_SPEC>;
#[doc = ""]
pub mod dfii_pi0_command_issue;
#[doc = "DFII_PI0_ADDRESS (rw) register accessor: an alias for `Reg<DFII_PI0_ADDRESS_SPEC>`"]
pub type DFII_PI0_ADDRESS = crate::Reg<dfii_pi0_address::DFII_PI0_ADDRESS_SPEC>;
#[doc = "DFI address bus"]
pub mod dfii_pi0_address;
#[doc = "DFII_PI0_BADDRESS (rw) register accessor: an alias for `Reg<DFII_PI0_BADDRESS_SPEC>`"]
pub type DFII_PI0_BADDRESS = crate::Reg<dfii_pi0_baddress::DFII_PI0_BADDRESS_SPEC>;
#[doc = "DFI bank address bus"]
pub mod dfii_pi0_baddress;
#[doc = "DFII_PI0_WRDATA (rw) register accessor: an alias for `Reg<DFII_PI0_WRDATA_SPEC>`"]
pub type DFII_PI0_WRDATA = crate::Reg<dfii_pi0_wrdata::DFII_PI0_WRDATA_SPEC>;
#[doc = "DFI write data bus"]
pub mod dfii_pi0_wrdata;
#[doc = "DFII_PI0_RDDATA (rw) register accessor: an alias for `Reg<DFII_PI0_RDDATA_SPEC>`"]
pub type DFII_PI0_RDDATA = crate::Reg<dfii_pi0_rddata::DFII_PI0_RDDATA_SPEC>;
#[doc = "DFI read data bus"]
pub mod dfii_pi0_rddata;
