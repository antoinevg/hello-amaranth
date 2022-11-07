#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Load value when Timer is (re-)enabled. In One-Shot mode, the value written to this register specifies the Timer's duration in clock cycles."]
    pub load: LOAD,
    #[doc = "0x04 - Reload value when Timer reaches ``0``. In Periodic mode, the value written to this register specify the Timer's period in clock cycles."]
    pub reload: RELOAD,
    #[doc = "0x08 - Enable flag of the Timer. Set this flag to ``1`` to enable/start the Timer. Set to ``0`` to disable the Timer."]
    pub en: EN,
    #[doc = "0x0c - Update trigger for the current countdown value. A write to this register latches the current countdown value to ``value`` register."]
    pub update_value: UPDATE_VALUE,
    #[doc = "0x10 - Latched countdown value. This value is updated by writing to ``update_value``."]
    pub value: VALUE,
    #[doc = "0x14 - This register contains the current raw level of the zero event trigger. Writes to this register have no effect."]
    pub ev_status: EV_STATUS,
    #[doc = "0x18 - When a zero event occurs, the corresponding bit will be set in this register. To clear the Event, set the corresponding bit in this register."]
    pub ev_pending: EV_PENDING,
    #[doc = "0x1c - This register enables the corresponding zero events. Write a ``0`` to this register to disable individual events."]
    pub ev_enable: EV_ENABLE,
}
#[doc = "LOAD (rw) register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "Load value when Timer is (re-)enabled. In One-Shot mode, the value written to this register specifies the Timer's duration in clock cycles."]
pub mod load;
#[doc = "RELOAD (rw) register accessor: an alias for `Reg<RELOAD_SPEC>`"]
pub type RELOAD = crate::Reg<reload::RELOAD_SPEC>;
#[doc = "Reload value when Timer reaches ``0``. In Periodic mode, the value written to this register specify the Timer's period in clock cycles."]
pub mod reload;
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "Enable flag of the Timer. Set this flag to ``1`` to enable/start the Timer. Set to ``0`` to disable the Timer."]
pub mod en;
#[doc = "UPDATE_VALUE (rw) register accessor: an alias for `Reg<UPDATE_VALUE_SPEC>`"]
pub type UPDATE_VALUE = crate::Reg<update_value::UPDATE_VALUE_SPEC>;
#[doc = "Update trigger for the current countdown value. A write to this register latches the current countdown value to ``value`` register."]
pub mod update_value;
#[doc = "VALUE (rw) register accessor: an alias for `Reg<VALUE_SPEC>`"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "Latched countdown value. This value is updated by writing to ``update_value``."]
pub mod value;
#[doc = "EV_STATUS (rw) register accessor: an alias for `Reg<EV_STATUS_SPEC>`"]
pub type EV_STATUS = crate::Reg<ev_status::EV_STATUS_SPEC>;
#[doc = "This register contains the current raw level of the zero event trigger. Writes to this register have no effect."]
pub mod ev_status;
#[doc = "EV_PENDING (rw) register accessor: an alias for `Reg<EV_PENDING_SPEC>`"]
pub type EV_PENDING = crate::Reg<ev_pending::EV_PENDING_SPEC>;
#[doc = "When a zero event occurs, the corresponding bit will be set in this register. To clear the Event, set the corresponding bit in this register."]
pub mod ev_pending;
#[doc = "EV_ENABLE (rw) register accessor: an alias for `Reg<EV_ENABLE_SPEC>`"]
pub type EV_ENABLE = crate::Reg<ev_enable::EV_ENABLE_SPEC>;
#[doc = "This register enables the corresponding zero events. Write a ``0`` to this register to disable individual events."]
pub mod ev_enable;
