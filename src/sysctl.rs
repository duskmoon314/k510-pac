#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x105c],
    #[doc = "0x105c..0x106c - UART\\[i\\]
host module serial interface clock configure register"]
    pub uart_sclk_cfg: [crate::Reg<uart_sclk_cfg::UART_SCLK_CFG_SPEC>; 4],
    _reserved1: [u8; 0x0fec],
    #[doc = "0x2058..0x2068 - UART\\[i\\]
host module reset control register"]
    pub uart_rst_ctl: [crate::Reg<uart_rst_ctl::UART_RST_CTL_SPEC>; 4],
}
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
