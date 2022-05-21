#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PLL\\[i\\]
configuration register 0"]
    pub pll0_cfg0: crate::Reg<pll_cfg0::PLL_CFG0_SPEC>,
    #[doc = "0x04 - PLL\\[i\\]
configuration register 1"]
    pub pll0_cfg1: crate::Reg<pll_cfg1::PLL_CFG1_SPEC>,
    #[doc = "0x08 - PLL\\[i\\]
control register"]
    pub pll0_ctl: crate::Reg<pll_ctl::PLL_CTL_SPEC>,
    #[doc = "0x0c - PLL\\[i\\]
status register"]
    pub pll0_stat: crate::Reg<pll_stat::PLL_STAT_SPEC>,
    #[doc = "0x10 - PLL\\[i\\]
configuration register 0"]
    pub pll1_cfg0: crate::Reg<pll_cfg0::PLL_CFG0_SPEC>,
    #[doc = "0x14 - PLL\\[i\\]
configuration register 1"]
    pub pll1_cfg1: crate::Reg<pll_cfg1::PLL_CFG1_SPEC>,
    #[doc = "0x18 - PLL\\[i\\]
control register"]
    pub pll1_ctl: crate::Reg<pll_ctl::PLL_CTL_SPEC>,
    #[doc = "0x1c - PLL\\[i\\]
status register"]
    pub pll1_stat: crate::Reg<pll_stat::PLL_STAT_SPEC>,
    #[doc = "0x20 - PLL\\[i\\]
configuration register 0"]
    pub pll2_cfg0: crate::Reg<pll_cfg0::PLL_CFG0_SPEC>,
    #[doc = "0x24 - PLL\\[i\\]
configuration register 1"]
    pub pll2_cfg1: crate::Reg<pll_cfg1::PLL_CFG1_SPEC>,
    #[doc = "0x28 - PLL\\[i\\]
control register"]
    pub pll2_ctl: crate::Reg<pll_ctl::PLL_CTL_SPEC>,
    #[doc = "0x2c - PLL\\[i\\]
status register"]
    pub pll2_stat: crate::Reg<pll_stat::PLL_STAT_SPEC>,
    #[doc = "0x30 - PLL\\[i\\]
configuration register 0"]
    pub pll3_cfg0: crate::Reg<pll_cfg0::PLL_CFG0_SPEC>,
    #[doc = "0x34 - PLL\\[i\\]
configuration register 1"]
    pub pll3_cfg1: crate::Reg<pll_cfg1::PLL_CFG1_SPEC>,
    #[doc = "0x38 - PLL\\[i\\]
control register"]
    pub pll3_ctl: crate::Reg<pll_ctl::PLL_CTL_SPEC>,
    #[doc = "0x3c - PLL\\[i\\]
status register"]
    pub pll3_stat: crate::Reg<pll_stat::PLL_STAT_SPEC>,
    #[doc = "0x40 - SoC boot control register"]
    pub soc_boot_ctl: crate::Reg<soc_boot_ctl::SOC_BOOT_CTL_SPEC>,
    #[doc = "0x44 - Reset status register"]
    pub reset_status: crate::Reg<reset_status::RESET_STATUS_SPEC>,
    #[doc = "0x48 - OSC 25M clock off register"]
    pub osc_25m_off: crate::Reg<osc_25m_off::OSC_25M_OFF_SPEC>,
    _reserved19: [u8; 0x1010],
    #[doc = "0x105c..0x106c - UART\\[i\\]
host module serial interface clock configure register"]
    pub uart_sclk_cfg: [crate::Reg<uart_sclk_cfg::UART_SCLK_CFG_SPEC>; 4],
    _reserved20: [u8; 0x0fec],
    #[doc = "0x2058..0x2068 - UART\\[i\\]
host module reset control register"]
    pub uart_rst_ctl: [crate::Reg<uart_rst_ctl::UART_RST_CTL_SPEC>; 4],
}
#[doc = "PLL_CFG0 register accessor: an alias for `Reg<PLL_CFG0_SPEC>`"]
pub type PLL_CFG0 = crate::Reg<pll_cfg0::PLL_CFG0_SPEC>;
#[doc = "PLL\\[i\\]
configuration register 0"]
pub mod pll_cfg0;
#[doc = "PLL_CFG1 register accessor: an alias for `Reg<PLL_CFG1_SPEC>`"]
pub type PLL_CFG1 = crate::Reg<pll_cfg1::PLL_CFG1_SPEC>;
#[doc = "PLL\\[i\\]
configuration register 1"]
pub mod pll_cfg1;
#[doc = "PLL_CTL register accessor: an alias for `Reg<PLL_CTL_SPEC>`"]
pub type PLL_CTL = crate::Reg<pll_ctl::PLL_CTL_SPEC>;
#[doc = "PLL\\[i\\]
control register"]
pub mod pll_ctl;
#[doc = "PLL_STAT register accessor: an alias for `Reg<PLL_STAT_SPEC>`"]
pub type PLL_STAT = crate::Reg<pll_stat::PLL_STAT_SPEC>;
#[doc = "PLL\\[i\\]
status register"]
pub mod pll_stat;
#[doc = "SOC_BOOT_CTL register accessor: an alias for `Reg<SOC_BOOT_CTL_SPEC>`"]
pub type SOC_BOOT_CTL = crate::Reg<soc_boot_ctl::SOC_BOOT_CTL_SPEC>;
#[doc = "SoC boot control register"]
pub mod soc_boot_ctl;
#[doc = "RESET_STATUS register accessor: an alias for `Reg<RESET_STATUS_SPEC>`"]
pub type RESET_STATUS = crate::Reg<reset_status::RESET_STATUS_SPEC>;
#[doc = "Reset status register"]
pub mod reset_status;
#[doc = "OSC_25M_OFF register accessor: an alias for `Reg<OSC_25M_OFF_SPEC>`"]
pub type OSC_25M_OFF = crate::Reg<osc_25m_off::OSC_25M_OFF_SPEC>;
#[doc = "OSC 25M clock off register"]
pub mod osc_25m_off;
#[doc = "UART_SCLK_CFG register accessor: an alias for `Reg<UART_SCLK_CFG_SPEC>`"]
pub type UART_SCLK_CFG = crate::Reg<uart_sclk_cfg::UART_SCLK_CFG_SPEC>;
#[doc = "UART\\[i\\]
host module serial interface clock configure register"]
pub mod uart_sclk_cfg;
#[doc = "UART_RST_CTL register accessor: an alias for `Reg<UART_RST_CTL_SPEC>`"]
pub type UART_RST_CTL = crate::Reg<uart_rst_ctl::UART_RST_CTL_SPEC>;
#[doc = "UART\\[i\\]
host module reset control register"]
pub mod uart_rst_ctl;
