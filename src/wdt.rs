#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub wdt_cr: crate::Reg<wdt_cr::WDT_CR_SPEC>,
    #[doc = "0x04 - timeout rage"]
    pub wdt_torr: crate::Reg<wdt_torr::WDT_TORR_SPEC>,
    #[doc = "0x08 - current counter"]
    pub wdt_ccvr: crate::Reg<wdt_ccvr::WDT_CCVR_SPEC>,
    #[doc = "0x0c - counter restart"]
    pub wdt_crr: crate::Reg<wdt_crr::WDT_CRR_SPEC>,
    #[doc = "0x10 - interrupt status"]
    pub wdt_stat: crate::Reg<wdt_stat::WDT_STAT_SPEC>,
    #[doc = "0x14 - interrupt clear"]
    pub wdt_eoi: crate::Reg<wdt_eoi::WDT_EOI_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - protection level"]
    pub wdt_prot_level: crate::Reg<wdt_prot_level::WDT_PROT_LEVEL_SPEC>,
    _reserved7: [u8; 0xc4],
    #[doc = "0xe4..0xf8 - parameters info"]
    pub wdt_comp_param_: [crate::Reg<wdt_comp_param_::WDT_COMP_PARAM__SPEC>; 5],
    #[doc = "0xf8 - version info"]
    pub wdt_comp_version: crate::Reg<wdt_comp_version::WDT_COMP_VERSION_SPEC>,
    #[doc = "0xfc - type info"]
    pub wdt_comp_type: crate::Reg<wdt_comp_type::WDT_COMP_TYPE_SPEC>,
}
impl RegisterBlock {
    #[doc = "0xe4 - parameters info"]
    #[inline(always)]
    pub fn wdt_comp_param_5(&self) -> &crate::Reg<wdt_comp_param_::WDT_COMP_PARAM__SPEC> {
        &self.wdt_comp_param_[0]
    }
    #[doc = "0xe8 - parameters info"]
    #[inline(always)]
    pub fn wdt_comp_param_4(&self) -> &crate::Reg<wdt_comp_param_::WDT_COMP_PARAM__SPEC> {
        &self.wdt_comp_param_[1]
    }
    #[doc = "0xec - parameters info"]
    #[inline(always)]
    pub fn wdt_comp_param_3(&self) -> &crate::Reg<wdt_comp_param_::WDT_COMP_PARAM__SPEC> {
        &self.wdt_comp_param_[2]
    }
    #[doc = "0xf0 - parameters info"]
    #[inline(always)]
    pub fn wdt_comp_param_2(&self) -> &crate::Reg<wdt_comp_param_::WDT_COMP_PARAM__SPEC> {
        &self.wdt_comp_param_[3]
    }
    #[doc = "0xf4 - parameters info"]
    #[inline(always)]
    pub fn wdt_comp_param_1(&self) -> &crate::Reg<wdt_comp_param_::WDT_COMP_PARAM__SPEC> {
        &self.wdt_comp_param_[4]
    }
}
#[doc = "WDT_CR register accessor: an alias for `Reg<WDT_CR_SPEC>`"]
pub type WDT_CR = crate::Reg<wdt_cr::WDT_CR_SPEC>;
#[doc = "control register"]
pub mod wdt_cr;
#[doc = "WDT_TORR register accessor: an alias for `Reg<WDT_TORR_SPEC>`"]
pub type WDT_TORR = crate::Reg<wdt_torr::WDT_TORR_SPEC>;
#[doc = "timeout rage"]
pub mod wdt_torr;
#[doc = "WDT_CCVR register accessor: an alias for `Reg<WDT_CCVR_SPEC>`"]
pub type WDT_CCVR = crate::Reg<wdt_ccvr::WDT_CCVR_SPEC>;
#[doc = "current counter"]
pub mod wdt_ccvr;
#[doc = "WDT_CRR register accessor: an alias for `Reg<WDT_CRR_SPEC>`"]
pub type WDT_CRR = crate::Reg<wdt_crr::WDT_CRR_SPEC>;
#[doc = "counter restart"]
pub mod wdt_crr;
#[doc = "WDT_STAT register accessor: an alias for `Reg<WDT_STAT_SPEC>`"]
pub type WDT_STAT = crate::Reg<wdt_stat::WDT_STAT_SPEC>;
#[doc = "interrupt status"]
pub mod wdt_stat;
#[doc = "WDT_EOI register accessor: an alias for `Reg<WDT_EOI_SPEC>`"]
pub type WDT_EOI = crate::Reg<wdt_eoi::WDT_EOI_SPEC>;
#[doc = "interrupt clear"]
pub mod wdt_eoi;
#[doc = "WDT_PROT_LEVEL register accessor: an alias for `Reg<WDT_PROT_LEVEL_SPEC>`"]
pub type WDT_PROT_LEVEL = crate::Reg<wdt_prot_level::WDT_PROT_LEVEL_SPEC>;
#[doc = "protection level"]
pub mod wdt_prot_level;
#[doc = "WDT_COMP_PARAM_ register accessor: an alias for `Reg<WDT_COMP_PARAM__SPEC>`"]
pub type WDT_COMP_PARAM_ = crate::Reg<wdt_comp_param_::WDT_COMP_PARAM__SPEC>;
#[doc = "parameters info"]
pub mod wdt_comp_param_;
#[doc = "WDT_COMP_VERSION register accessor: an alias for `Reg<WDT_COMP_VERSION_SPEC>`"]
pub type WDT_COMP_VERSION = crate::Reg<wdt_comp_version::WDT_COMP_VERSION_SPEC>;
#[doc = "version info"]
pub mod wdt_comp_version;
#[doc = "WDT_COMP_TYPE register accessor: an alias for `Reg<WDT_COMP_TYPE_SPEC>`"]
pub type WDT_COMP_TYPE = crate::Reg<wdt_comp_type::WDT_COMP_TYPE_SPEC>;
#[doc = "type info"]
pub mod wdt_comp_type;
