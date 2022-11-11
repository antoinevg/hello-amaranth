#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Control."]
    pub control: CONTROL,
    #[doc = "0x04 - SPI Status."]
    pub status: STATUS,
    #[doc = "0x08 - SPI MOSI data (MSB-first serialization)."]
    pub mosi: MOSI,
    #[doc = "0x0c - SPI MISO data (MSB-first de-serialization)."]
    pub miso: MISO,
    #[doc = "0x10 - SPI CS Chip-Select and Mode."]
    pub cs: CS,
    #[doc = "0x14 - SPI Loopback Mode."]
    pub loopback: LOOPBACK,
    #[doc = "0x18 - SPI Clk Divider."]
    pub clk_divider: CLK_DIVIDER,
}
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "SPI Control."]
pub mod control;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "SPI Status."]
pub mod status;
#[doc = "MOSI (rw) register accessor: an alias for `Reg<MOSI_SPEC>`"]
pub type MOSI = crate::Reg<mosi::MOSI_SPEC>;
#[doc = "SPI MOSI data (MSB-first serialization)."]
pub mod mosi;
#[doc = "MISO (rw) register accessor: an alias for `Reg<MISO_SPEC>`"]
pub type MISO = crate::Reg<miso::MISO_SPEC>;
#[doc = "SPI MISO data (MSB-first de-serialization)."]
pub mod miso;
#[doc = "CS (rw) register accessor: an alias for `Reg<CS_SPEC>`"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "SPI CS Chip-Select and Mode."]
pub mod cs;
#[doc = "LOOPBACK (rw) register accessor: an alias for `Reg<LOOPBACK_SPEC>`"]
pub type LOOPBACK = crate::Reg<loopback::LOOPBACK_SPEC>;
#[doc = "SPI Loopback Mode."]
pub mod loopback;
#[doc = "CLK_DIVIDER (rw) register accessor: an alias for `Reg<CLK_DIVIDER_SPEC>`"]
pub type CLK_DIVIDER = crate::Reg<clk_divider::CLK_DIVIDER_SPEC>;
#[doc = "SPI Clk Divider."]
pub mod clk_divider;
