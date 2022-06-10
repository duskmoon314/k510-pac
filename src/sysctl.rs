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
    _reserved19: [u8; 0x14],
    #[doc = "0x60 - SoC global reset control register"]
    pub soc_glb_rst: crate::Reg<soc_glb_rst::SOC_GLB_RST_SPEC>,
    #[doc = "0x64 - SoC reset timing configuration register"]
    pub soc_reset_tim: crate::Reg<soc_reset_tim::SOC_RESET_TIM_SPEC>,
    #[doc = "0x68 - SoC sleep mode timing configuration register"]
    pub soc_sleep_tim: crate::Reg<soc_sleep_tim::SOC_SLEEP_TIM_SPEC>,
    #[doc = "0x6c - SoC sleep mode control register"]
    pub soc_sleep_ctl: crate::Reg<soc_sleep_ctl::SOC_SLEEP_CTL_SPEC>,
    #[doc = "0x70 - Clock stable timing configuration register"]
    pub clk_stable_tim: crate::Reg<clk_stable_tim::CLK_STABLE_TIM_SPEC>,
    #[doc = "0x74 - CPU wake-up timing configuration register"]
    pub cpu_wakup_tim: crate::Reg<cpu_wakup_tim::CPU_WAKUP_TIM_SPEC>,
    #[doc = "0x78 - SoC wake-up cause status register"]
    pub soc_wakup_src: crate::Reg<soc_wakup_src::SOC_WAKUP_SRC_SPEC>,
    #[doc = "0x7c - CPU wake-up (when SoC core is woken up) configuration register"]
    pub cpu_wakup_cfg: crate::Reg<cpu_wakup_cfg::CPU_WAKUP_CFG_SPEC>,
    #[doc = "0x80 - SoC internal Timer module's timer pause control register"]
    pub timer_pause_ctl: crate::Reg<timer_pause_ctl::TIMER_PAUSE_CTL_SPEC>,
    _reserved28: [u8; 0x0c],
    #[doc = "0x90 - Sysctl module interrupt 0 raw status register"]
    pub sys_ctl_int0_raw: crate::Reg<sys_ctl_int0_raw::SYS_CTL_INT0_RAW_SPEC>,
    #[doc = "0x94 - Sysctl module interrupt 0 interrupt enable register"]
    pub sys_ctl_int0_en: crate::Reg<sys_ctl_int0_en::SYS_CTL_INT0_EN_SPEC>,
    #[doc = "0x98 - Sysctl module interrupt 0 interrupt status register"]
    pub sys_ctl_int0_stat: crate::Reg<sys_ctl_int0_stat::SYS_CTL_INT0_STAT_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0xa0 - Sysctl module interrupt 1 raw status register"]
    pub sys_ctl_int1_raw: crate::Reg<sys_ctl_int1_raw::SYS_CTL_INT1_RAW_SPEC>,
    #[doc = "0xa4 - Sysctl module interrupt 1 interrupt enable register"]
    pub sys_ctl_int1_en: crate::Reg<sys_ctl_int1_en::SYS_CTL_INT1_EN_SPEC>,
    #[doc = "0xa8 - Sysctl module interrupt 1 interrupt status register"]
    pub sys_ctl_int1_stat: crate::Reg<sys_ctl_int1_stat::SYS_CTL_INT1_STAT_SPEC>,
    _reserved34: [u8; 0x04],
    #[doc = "0xb0 - Sysctl module interrupt 2 raw status register"]
    pub sys_ctl_int2_raw: crate::Reg<sys_ctl_int2_raw::SYS_CTL_INT2_RAW_SPEC>,
    #[doc = "0xb4 - Sysctl module interrupt 2 interrupt enable register"]
    pub sys_ctl_int2_en: crate::Reg<sys_ctl_int2_en::SYS_CTL_INT2_EN_SPEC>,
    #[doc = "0xb8 - Sysctl module interrupt 2 interrupt status register"]
    pub sys_ctl_int2_stat: crate::Reg<sys_ctl_int2_stat::SYS_CTL_INT2_STAT_SPEC>,
    _reserved37: [u8; 0x44],
    #[doc = "0x100..0x108 - AX25M dual-core CPU hart\\[i\\]
reset vector register."]
    pub ax25m_hart_rstvec: [crate::Reg<ax25m_hart_rstvec::AX25M_HART_RSTVEC_SPEC>; 2],
    #[doc = "0x108 - AX25P processor CPU core reset vector register."]
    pub ax25p_core_rstvec: crate::Reg<ax25p_core_rstvec::AX25P_CORE_RSTVEC_SPEC>,
    _reserved39: [u8; 0x0c],
    #[doc = "0x118 - SoC sleep mode control register"]
    pub soc_sleep_mask: crate::Reg<soc_sleep_mask::SOC_SLEEP_MASK_SPEC>,
    #[doc = "0x11c - Test pin group select register"]
    pub test_pin_sel: crate::Reg<test_pin_sel::TEST_PIN_SEL_SPEC>,
    _reserved41: [u8; 0x0ee0],
    #[doc = "0x1000 - AX25M dual-core RISCV core clock Division configure register"]
    pub ax25m_clk_cfg: crate::Reg<ax25m_clk_cfg::AX25M_CLK_CFG_SPEC>,
    #[doc = "0x1004 - AX25M dual-core RISCV core Machine Timer clock Division configure register"]
    pub ax25m_mtimer_clk_cfg: crate::Reg<ax25m_mtimer_clk_cfg::AX25M_MTIMER_CLK_CFG_SPEC>,
    _reserved43: [u8; 0x08],
    #[doc = "0x1010 - AX25P single-core RISCV core clock Division configure register"]
    pub ax25p_clk_cfg: crate::Reg<ax25p_clk_cfg::AX25P_CLK_CFG_SPEC>,
    #[doc = "0x1014 - AX25P single-core RISCV core Machine Timer clock Division configure register"]
    pub ax25p_mtimer_clk_cfg: crate::Reg<ax25p_mtimer_clk_cfg::AX25P_MTIMER_CLK_CFG_SPEC>,
    _reserved45: [u8; 0x08],
    #[doc = "0x1020 - GNNE axi clock configure register"]
    pub gnne_aclk_cfg: crate::Reg<gnne_aclk_cfg::GNNE_ACLK_CFG_SPEC>,
    _reserved46: [u8; 0x04],
    #[doc = "0x1028 - GNNE system clock configure register"]
    pub gnne_sysclk_cfg: crate::Reg<gnne_sysclk_cfg::GNNE_SYSCLK_CFG_SPEC>,
    _reserved47: [u8; 0x10],
    #[doc = "0x103c - I2C2AXI clock configure register"]
    pub i2c2axi_clk_cfg: crate::Reg<i2c2axi_clk_cfg::I2C2AXI_CLK_CFG_SPEC>,
    _reserved48: [u8; 0x1c],
    #[doc = "0x105c..0x106c - UART\\[i\\]
host module serial interface clock configure register"]
    pub uart_sclk_cfg: [crate::Reg<uart_sclk_cfg::UART_SCLK_CFG_SPEC>; 4],
    _reserved49: [u8; 0x0fec],
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
#[doc = "SOC_GLB_RST register accessor: an alias for `Reg<SOC_GLB_RST_SPEC>`"]
pub type SOC_GLB_RST = crate::Reg<soc_glb_rst::SOC_GLB_RST_SPEC>;
#[doc = "SoC global reset control register"]
pub mod soc_glb_rst;
#[doc = "SOC_RESET_TIM register accessor: an alias for `Reg<SOC_RESET_TIM_SPEC>`"]
pub type SOC_RESET_TIM = crate::Reg<soc_reset_tim::SOC_RESET_TIM_SPEC>;
#[doc = "SoC reset timing configuration register"]
pub mod soc_reset_tim;
#[doc = "SOC_SLEEP_TIM register accessor: an alias for `Reg<SOC_SLEEP_TIM_SPEC>`"]
pub type SOC_SLEEP_TIM = crate::Reg<soc_sleep_tim::SOC_SLEEP_TIM_SPEC>;
#[doc = "SoC sleep mode timing configuration register"]
pub mod soc_sleep_tim;
#[doc = "SOC_SLEEP_CTL register accessor: an alias for `Reg<SOC_SLEEP_CTL_SPEC>`"]
pub type SOC_SLEEP_CTL = crate::Reg<soc_sleep_ctl::SOC_SLEEP_CTL_SPEC>;
#[doc = "SoC sleep mode control register"]
pub mod soc_sleep_ctl;
#[doc = "CLK_STABLE_TIM register accessor: an alias for `Reg<CLK_STABLE_TIM_SPEC>`"]
pub type CLK_STABLE_TIM = crate::Reg<clk_stable_tim::CLK_STABLE_TIM_SPEC>;
#[doc = "Clock stable timing configuration register"]
pub mod clk_stable_tim;
#[doc = "CPU_WAKUP_TIM register accessor: an alias for `Reg<CPU_WAKUP_TIM_SPEC>`"]
pub type CPU_WAKUP_TIM = crate::Reg<cpu_wakup_tim::CPU_WAKUP_TIM_SPEC>;
#[doc = "CPU wake-up timing configuration register"]
pub mod cpu_wakup_tim;
#[doc = "SOC_WAKUP_SRC register accessor: an alias for `Reg<SOC_WAKUP_SRC_SPEC>`"]
pub type SOC_WAKUP_SRC = crate::Reg<soc_wakup_src::SOC_WAKUP_SRC_SPEC>;
#[doc = "SoC wake-up cause status register"]
pub mod soc_wakup_src;
#[doc = "CPU_WAKUP_CFG register accessor: an alias for `Reg<CPU_WAKUP_CFG_SPEC>`"]
pub type CPU_WAKUP_CFG = crate::Reg<cpu_wakup_cfg::CPU_WAKUP_CFG_SPEC>;
#[doc = "CPU wake-up (when SoC core is woken up) configuration register"]
pub mod cpu_wakup_cfg;
#[doc = "TIMER_PAUSE_CTL register accessor: an alias for `Reg<TIMER_PAUSE_CTL_SPEC>`"]
pub type TIMER_PAUSE_CTL = crate::Reg<timer_pause_ctl::TIMER_PAUSE_CTL_SPEC>;
#[doc = "SoC internal Timer module's timer pause control register"]
pub mod timer_pause_ctl;
#[doc = "SYS_CTL_INT0_RAW register accessor: an alias for `Reg<SYS_CTL_INT0_RAW_SPEC>`"]
pub type SYS_CTL_INT0_RAW = crate::Reg<sys_ctl_int0_raw::SYS_CTL_INT0_RAW_SPEC>;
#[doc = "Sysctl module interrupt 0 raw status register"]
pub mod sys_ctl_int0_raw;
#[doc = "SYS_CTL_INT0_EN register accessor: an alias for `Reg<SYS_CTL_INT0_EN_SPEC>`"]
pub type SYS_CTL_INT0_EN = crate::Reg<sys_ctl_int0_en::SYS_CTL_INT0_EN_SPEC>;
#[doc = "Sysctl module interrupt 0 interrupt enable register"]
pub mod sys_ctl_int0_en;
#[doc = "SYS_CTL_INT0_STAT register accessor: an alias for `Reg<SYS_CTL_INT0_STAT_SPEC>`"]
pub type SYS_CTL_INT0_STAT = crate::Reg<sys_ctl_int0_stat::SYS_CTL_INT0_STAT_SPEC>;
#[doc = "Sysctl module interrupt 0 interrupt status register"]
pub mod sys_ctl_int0_stat;
#[doc = "SYS_CTL_INT1_RAW register accessor: an alias for `Reg<SYS_CTL_INT1_RAW_SPEC>`"]
pub type SYS_CTL_INT1_RAW = crate::Reg<sys_ctl_int1_raw::SYS_CTL_INT1_RAW_SPEC>;
#[doc = "Sysctl module interrupt 1 raw status register"]
pub mod sys_ctl_int1_raw;
#[doc = "SYS_CTL_INT1_EN register accessor: an alias for `Reg<SYS_CTL_INT1_EN_SPEC>`"]
pub type SYS_CTL_INT1_EN = crate::Reg<sys_ctl_int1_en::SYS_CTL_INT1_EN_SPEC>;
#[doc = "Sysctl module interrupt 1 interrupt enable register"]
pub mod sys_ctl_int1_en;
#[doc = "SYS_CTL_INT1_STAT register accessor: an alias for `Reg<SYS_CTL_INT1_STAT_SPEC>`"]
pub type SYS_CTL_INT1_STAT = crate::Reg<sys_ctl_int1_stat::SYS_CTL_INT1_STAT_SPEC>;
#[doc = "Sysctl module interrupt 1 interrupt status register"]
pub mod sys_ctl_int1_stat;
#[doc = "SYS_CTL_INT2_RAW register accessor: an alias for `Reg<SYS_CTL_INT2_RAW_SPEC>`"]
pub type SYS_CTL_INT2_RAW = crate::Reg<sys_ctl_int2_raw::SYS_CTL_INT2_RAW_SPEC>;
#[doc = "Sysctl module interrupt 2 raw status register"]
pub mod sys_ctl_int2_raw;
#[doc = "SYS_CTL_INT2_EN register accessor: an alias for `Reg<SYS_CTL_INT2_EN_SPEC>`"]
pub type SYS_CTL_INT2_EN = crate::Reg<sys_ctl_int2_en::SYS_CTL_INT2_EN_SPEC>;
#[doc = "Sysctl module interrupt 2 interrupt enable register"]
pub mod sys_ctl_int2_en;
#[doc = "SYS_CTL_INT2_STAT register accessor: an alias for `Reg<SYS_CTL_INT2_STAT_SPEC>`"]
pub type SYS_CTL_INT2_STAT = crate::Reg<sys_ctl_int2_stat::SYS_CTL_INT2_STAT_SPEC>;
#[doc = "Sysctl module interrupt 2 interrupt status register"]
pub mod sys_ctl_int2_stat;
#[doc = "AX25M_HART_RSTVEC register accessor: an alias for `Reg<AX25M_HART_RSTVEC_SPEC>`"]
pub type AX25M_HART_RSTVEC = crate::Reg<ax25m_hart_rstvec::AX25M_HART_RSTVEC_SPEC>;
#[doc = "AX25M dual-core CPU hart\\[i\\]
reset vector register."]
pub mod ax25m_hart_rstvec;
#[doc = "AX25P_CORE_RSTVEC register accessor: an alias for `Reg<AX25P_CORE_RSTVEC_SPEC>`"]
pub type AX25P_CORE_RSTVEC = crate::Reg<ax25p_core_rstvec::AX25P_CORE_RSTVEC_SPEC>;
#[doc = "AX25P processor CPU core reset vector register."]
pub mod ax25p_core_rstvec;
#[doc = "SOC_SLEEP_MASK register accessor: an alias for `Reg<SOC_SLEEP_MASK_SPEC>`"]
pub type SOC_SLEEP_MASK = crate::Reg<soc_sleep_mask::SOC_SLEEP_MASK_SPEC>;
#[doc = "SoC sleep mode control register"]
pub mod soc_sleep_mask;
#[doc = "TEST_PIN_SEL register accessor: an alias for `Reg<TEST_PIN_SEL_SPEC>`"]
pub type TEST_PIN_SEL = crate::Reg<test_pin_sel::TEST_PIN_SEL_SPEC>;
#[doc = "Test pin group select register"]
pub mod test_pin_sel;
#[doc = "AX25M_CLK_CFG register accessor: an alias for `Reg<AX25M_CLK_CFG_SPEC>`"]
pub type AX25M_CLK_CFG = crate::Reg<ax25m_clk_cfg::AX25M_CLK_CFG_SPEC>;
#[doc = "AX25M dual-core RISCV core clock Division configure register"]
pub mod ax25m_clk_cfg;
#[doc = "AX25M_MTIMER_CLK_CFG register accessor: an alias for `Reg<AX25M_MTIMER_CLK_CFG_SPEC>`"]
pub type AX25M_MTIMER_CLK_CFG = crate::Reg<ax25m_mtimer_clk_cfg::AX25M_MTIMER_CLK_CFG_SPEC>;
#[doc = "AX25M dual-core RISCV core Machine Timer clock Division configure register"]
pub mod ax25m_mtimer_clk_cfg;
#[doc = "AX25P_CLK_CFG register accessor: an alias for `Reg<AX25P_CLK_CFG_SPEC>`"]
pub type AX25P_CLK_CFG = crate::Reg<ax25p_clk_cfg::AX25P_CLK_CFG_SPEC>;
#[doc = "AX25P single-core RISCV core clock Division configure register"]
pub mod ax25p_clk_cfg;
#[doc = "AX25P_MTIMER_CLK_CFG register accessor: an alias for `Reg<AX25P_MTIMER_CLK_CFG_SPEC>`"]
pub type AX25P_MTIMER_CLK_CFG = crate::Reg<ax25p_mtimer_clk_cfg::AX25P_MTIMER_CLK_CFG_SPEC>;
#[doc = "AX25P single-core RISCV core Machine Timer clock Division configure register"]
pub mod ax25p_mtimer_clk_cfg;
#[doc = "GNNE_ACLK_CFG register accessor: an alias for `Reg<GNNE_ACLK_CFG_SPEC>`"]
pub type GNNE_ACLK_CFG = crate::Reg<gnne_aclk_cfg::GNNE_ACLK_CFG_SPEC>;
#[doc = "GNNE axi clock configure register"]
pub mod gnne_aclk_cfg;
#[doc = "GNNE_SYSCLK_CFG register accessor: an alias for `Reg<GNNE_SYSCLK_CFG_SPEC>`"]
pub type GNNE_SYSCLK_CFG = crate::Reg<gnne_sysclk_cfg::GNNE_SYSCLK_CFG_SPEC>;
#[doc = "GNNE system clock configure register"]
pub mod gnne_sysclk_cfg;
#[doc = "I2C2AXI_CLK_CFG register accessor: an alias for `Reg<I2C2AXI_CLK_CFG_SPEC>`"]
pub type I2C2AXI_CLK_CFG = crate::Reg<i2c2axi_clk_cfg::I2C2AXI_CLK_CFG_SPEC>;
#[doc = "I2C2AXI clock configure register"]
pub mod i2c2axi_clk_cfg;
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
