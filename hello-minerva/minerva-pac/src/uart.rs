#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub rxtx: RXTX,
    #[doc = "0x04 - TX FIFO Full."]
    pub txfull: TXFULL,
    #[doc = "0x08 - RX FIFO Empty."]
    pub rxempty: RXEMPTY,
    #[doc = "0x0c - This register contains the current raw level of the rx event trigger. Writes to this register have no effect."]
    pub ev_status: EV_STATUS,
    #[doc = "0x10 - When a rx event occurs, the corresponding bit will be set in this register. To clear the Event, set the corresponding bit in this register."]
    pub ev_pending: EV_PENDING,
    #[doc = "0x14 - This register enables the corresponding rx events. Write a ``0`` to this register to disable individual events."]
    pub ev_enable: EV_ENABLE,
    #[doc = "0x18 - TX FIFO Empty."]
    pub txempty: TXEMPTY,
    #[doc = "0x1c - RX FIFO Full."]
    pub rxfull: RXFULL,
}
#[doc = "RXTX (rw) register accessor: an alias for `Reg<RXTX_SPEC>`"]
pub type RXTX = crate::Reg<rxtx::RXTX_SPEC>;
#[doc = ""]
pub mod rxtx;
#[doc = "TXFULL (rw) register accessor: an alias for `Reg<TXFULL_SPEC>`"]
pub type TXFULL = crate::Reg<txfull::TXFULL_SPEC>;
#[doc = "TX FIFO Full."]
pub mod txfull;
#[doc = "RXEMPTY (rw) register accessor: an alias for `Reg<RXEMPTY_SPEC>`"]
pub type RXEMPTY = crate::Reg<rxempty::RXEMPTY_SPEC>;
#[doc = "RX FIFO Empty."]
pub mod rxempty;
#[doc = "EV_STATUS (rw) register accessor: an alias for `Reg<EV_STATUS_SPEC>`"]
pub type EV_STATUS = crate::Reg<ev_status::EV_STATUS_SPEC>;
#[doc = "This register contains the current raw level of the rx event trigger. Writes to this register have no effect."]
pub mod ev_status;
#[doc = "EV_PENDING (rw) register accessor: an alias for `Reg<EV_PENDING_SPEC>`"]
pub type EV_PENDING = crate::Reg<ev_pending::EV_PENDING_SPEC>;
#[doc = "When a rx event occurs, the corresponding bit will be set in this register. To clear the Event, set the corresponding bit in this register."]
pub mod ev_pending;
#[doc = "EV_ENABLE (rw) register accessor: an alias for `Reg<EV_ENABLE_SPEC>`"]
pub type EV_ENABLE = crate::Reg<ev_enable::EV_ENABLE_SPEC>;
#[doc = "This register enables the corresponding rx events. Write a ``0`` to this register to disable individual events."]
pub mod ev_enable;
#[doc = "TXEMPTY (rw) register accessor: an alias for `Reg<TXEMPTY_SPEC>`"]
pub type TXEMPTY = crate::Reg<txempty::TXEMPTY_SPEC>;
#[doc = "TX FIFO Empty."]
pub mod txempty;
#[doc = "RXFULL (rw) register accessor: an alias for `Reg<RXFULL_SPEC>`"]
pub type RXFULL = crate::Reg<rxfull::RXFULL_SPEC>;
#[doc = "RX FIFO Full."]
pub mod rxfull;
