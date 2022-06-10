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
    #[doc = "0x1040 - NOC bus clock Division and configure register"]
    pub noc_clk_cfg: crate::Reg<noc_clk_cfg::NOC_CLK_CFG_SPEC>,
    _reserved49: [u8; 0x0c],
    #[doc = "0x1050 - Peripheral subsystem modules bus IF clock configure register"]
    pub peri_sys_bus_clk_cfg: crate::Reg<peri_sys_bus_clk_cfg::PERI_SYS_BUS_CLK_CFG_SPEC>,
    #[doc = "0x1054 - Peripheral subsystem modules bus IF clock enable register"]
    pub peri_sys_bus_clk_en: crate::Reg<peri_sys_bus_clk_en::PERI_SYS_BUS_CLK_EN_SPEC>,
    _reserved51: [u8; 0x04],
    #[doc = "0x105c..0x106c - UART\\[i\\]
host module serial interface clock configure register"]
    pub uart_sclk_cfg: [crate::Reg<uart_sclk_cfg::UART_SCLK_CFG_SPEC>; 4],
    _reserved52: [u8; 0x04],
    #[doc = "0x1070 - I2S 2 slave module serial interface clock configure register"]
    pub i2s2_sclk_cfg: crate::Reg<i2s2_sclk_cfg::I2S2_SCLK_CFG_SPEC>,
    #[doc = "0x1074..0x1084 - SPI\\[i\\]
host module serial interface clock configure register"]
    pub spi_sclk_cfg: [crate::Reg<spi_sclk_cfg::SPI_SCLK_CFG_SPEC>; 4],
    _reserved54: [u8; 0x04],
    #[doc = "0x1088 - Audio interface module serial clock configure register"]
    pub audif_sclk_cfg: crate::Reg<audif_sclk_cfg::AUDIF_SCLK_CFG_SPEC>,
    #[doc = "0x108c - Audio device clock configure register"]
    pub audif_devclk_cfg: crate::Reg<audif_devclk_cfg::AUDIF_DEVCLK_CFG_SPEC>,
    #[doc = "0x1090 - Security Subsystem bus interface clock configure register"]
    pub sec_sys_bus_clk_cfg: crate::Reg<sec_sys_bus_clk_cfg::SEC_SYS_BUS_CLK_CFG_SPEC>,
    #[doc = "0x1094 - Security Subsystem modules bus interface clock control register"]
    pub sec_sys_bus_clk_en: crate::Reg<sec_sys_bus_clk_en::SEC_SYS_BUS_CLK_EN_SPEC>,
    #[doc = "0x1098 - OTP modules clock control register"]
    pub otp_clk_en: crate::Reg<otp_clk_en::OTP_CLK_EN_SPEC>,
    _reserved59: [u8; 0x04],
    #[doc = "0x10a0 - SRAM block 0/1 bus clock enable control register"]
    pub sram_bus_clk_en: crate::Reg<sram_bus_clk_en::SRAM_BUS_CLK_EN_SPEC>,
    _reserved60: [u8; 0x0c],
    #[doc = "0x10b0 - SOC Control Units slave APB clock enable control register"]
    pub soc_ctl_pclk_en: crate::Reg<soc_ctl_pclk_en::SOC_CTL_PCLK_EN_SPEC>,
    #[doc = "0x10b4 - SOC Control Units slave APB clock enable control register"]
    pub soc_ctl_pclk_en1: crate::Reg<soc_ctl_pclk_en1::SOC_CTL_PCLK_EN1_SPEC>,
    #[doc = "0x10b8..0x10d4 - I2C\\[i\\]
host module serial clock configure register"]
    pub i2c_icclk_cfg: [crate::Reg<i2c_icclk_cfg::I2C_ICCLK_CFG_SPEC>; 7],
    #[doc = "0x10d4..0x10e0 - WDT\\[i\\]
module tick clock configure register"]
    pub wdt_tclk_cfg: [crate::Reg<wdt_tclk_cfg::WDT_TCLK_CFG_SPEC>; 3],
    #[doc = "0x10e0 - System Timer module tick clocks source configure register"]
    pub timer_tclk_src: crate::Reg<timer_tclk_src::TIMER_TCLK_SRC_SPEC>,
    #[doc = "0x10e4 - System Timer module tick clocks division configure register"]
    pub timer_tclk_cfg: crate::Reg<timer_tclk_cfg::TIMER_TCLK_CFG_SPEC>,
    #[doc = "0x10e8 - System Timer module tick clocks division configure register"]
    pub timer_tclk_cfg1: crate::Reg<timer_tclk_cfg1::TIMER_TCLK_CFG1_SPEC>,
    #[doc = "0x10ec - VAD module audio data serial clock configure register"]
    pub vad_sclk_cfg: crate::Reg<vad_sclk_cfg::VAD_SCLK_CFG_SPEC>,
    _reserved68: [u8; 0x10],
    #[doc = "0x1100 - Storage subsystem modules bus IF clock enable register"]
    pub stor_sys_bus_clk_en: crate::Reg<stor_sys_bus_clk_en::STOR_SYS_BUS_CLK_EN_SPEC>,
    #[doc = "0x1104 - EMAC-PHY interface transceiver clock configure register"]
    pub emac_trx_clk_cfg: crate::Reg<emac_trx_clk_cfg::EMAC_TRX_CLK_CFG_SPEC>,
    #[doc = "0x1108 - EMAC-PHY interface transceiver clock configure register"]
    pub sd_card_clk_cfg: crate::Reg<sd_card_clk_cfg::SD_CARD_CLK_CFG_SPEC>,
    #[doc = "0x110c - SENSOR clock enable register"]
    pub sensor_clk_en: crate::Reg<sensor_clk_en::SENSOR_CLK_EN_SPEC>,
    #[doc = "0x1110 - ISP system APB clock enable control register"]
    pub isp_sys_pclk_en: crate::Reg<isp_sys_pclk_en::ISP_SYS_PCLK_EN_SPEC>,
    #[doc = "0x1114 - ISP system APB clock enable control register"]
    pub isp_sys_aclk_en: crate::Reg<isp_sys_aclk_en::ISP_SYS_ACLK_EN_SPEC>,
    #[doc = "0x1118 - DISP system APB clock enable control register"]
    pub disp_sys_pclk_en: crate::Reg<disp_sys_pclk_en::DISP_SYS_PCLK_EN_SPEC>,
    #[doc = "0x111c - DISP system APB clock enable control register"]
    pub disp_sys_aclk_en: crate::Reg<disp_sys_aclk_en::DISP_SYS_ACLK_EN_SPEC>,
    #[doc = "0x1120 - VI module tpg pixel clock configure register"]
    pub tpg_pixel_clk_cfg: crate::Reg<tpg_pixel_clk_cfg::TPG_PIXEL_CLK_CFG_SPEC>,
    #[doc = "0x1124..0x112c - CSI\\[i\\]
module pixel clock configure register"]
    pub csi_pixel_clk_cfg: [crate::Reg<csi_pixel_clk_cfg::CSI_PIXEL_CLK_CFG_SPEC>; 2],
    _reserved78: [u8; 0x08],
    #[doc = "0x1134 - DISP module pixel clock configure register"]
    pub disp_pixel_clk_cfg: crate::Reg<disp_pixel_clk_cfg::DISP_PIXEL_CLK_CFG_SPEC>,
    _reserved79: [u8; 0x04],
    #[doc = "0x113c..0x1144 - CSI\\[i\\]
module system clock configure register"]
    pub csi_sys_clk_cfg: [crate::Reg<csi_sys_clk_cfg::CSI_SYS_CLK_CFG_SPEC>; 2],
    _reserved80: [u8; 0x08],
    #[doc = "0x114c - DSI module system clock configure register"]
    pub dsi_sys_clk_cfg: crate::Reg<dsi_sys_clk_cfg::DSI_SYS_CLK_CFG_SPEC>,
    #[doc = "0x1150 - H264 module AXI clock enable control register"]
    pub h264_aclk_en: crate::Reg<h264_aclk_en::H264_ACLK_EN_SPEC>,
    #[doc = "0x1154 - USB host module clock enable control register"]
    pub usb_clk_en: crate::Reg<usb_clk_en::USB_CLK_EN_SPEC>,
    #[doc = "0x1158 - MIPI TXDPHY clock enable control register"]
    pub txdphy_clk_en: crate::Reg<txdphy_clk_en::TXDPHY_CLK_EN_SPEC>,
    #[doc = "0x115c - MIPI RXDPHY clock enable control register"]
    pub rxdphy_clk_en: crate::Reg<rxdphy_clk_en::RXDPHY_CLK_EN_SPEC>,
    #[doc = "0x1160 - Memory Controller DFS Command FIFO"]
    pub mem_ctl_cmd_fifo: crate::Reg<mem_ctl_cmd_fifo::MEM_CTL_CMD_FIFO_SPEC>,
    #[doc = "0x1164 - Memory Controller DFS Command FIFO STATUS"]
    pub mem_ctl_cmd_fifo_sts: crate::Reg<mem_ctl_cmd_fifo_sts::MEM_CTL_CMD_FIFO_STS_SPEC>,
    #[doc = "0x1168 - Memory Controller clock configuration register"]
    pub mem_ctl_clk_cfg: crate::Reg<mem_ctl_clk_cfg::MEM_CTL_CLK_CFG_SPEC>,
    #[doc = "0x116c - Memory Controller clock DFS configuration register"]
    pub mem_ctl_dfs_cfg: crate::Reg<mem_ctl_dfs_cfg::MEM_CTL_DFS_CFG_SPEC>,
    #[doc = "0x1170 - Memory Controller DFS Command FIFO FLUSH"]
    pub mem_ctl_cmd_fifo_flush: crate::Reg<mem_ctl_cmd_fifo_flush::MEM_CTL_CMD_FIFO_FLUSH_SPEC>,
    _reserved90: [u8; 0x0e8c],
    #[doc = "0x2000 - AX25M dual-core RISCV reset timing control register"]
    pub ax25m_rst_tim: crate::Reg<ax25m_rst_tim::AX25M_RST_TIM_SPEC>,
    _reserved91: [u8; 0x0c],
    #[doc = "0x2010 - AX25P single-core RISCV reset timing control register"]
    pub ax25p_rst_tim: crate::Reg<ax25p_rst_tim::AX25P_RST_TIM_SPEC>,
    #[doc = "0x2014 - AX25P single-core RISCV reset control register"]
    pub ax25p_rst_ctl: crate::Reg<ax25p_rst_ctl::AX25P_RST_CTL_SPEC>,
    _reserved93: [u8; 0x10],
    #[doc = "0x2028 - GNNE reset timing control register"]
    pub gnne_rst_tim: crate::Reg<gnne_rst_tim::GNNE_RST_TIM_SPEC>,
    #[doc = "0x202c - GNNE reset control register"]
    pub gnne_rst_ctl: crate::Reg<gnne_rst_ctl::GNNE_RST_CTL_SPEC>,
    _reserved95: [u8; 0x10],
    #[doc = "0x2040 - SOC Control subsystem reseting control register"]
    pub soc_ctl_rst_ctl: crate::Reg<soc_ctl_rst_ctl::SOC_CTL_RST_CTL_SPEC>,
    #[doc = "0x2044 - SOC Control subsystem reseting control register"]
    pub soc_ctl_rst_ctl1: crate::Reg<soc_ctl_rst_ctl1::SOC_CTL_RST_CTL1_SPEC>,
    _reserved97: [u8; 0x08],
    #[doc = "0x2050 - Peripheral subsystem reset timing control register"]
    pub peri_sys_rst_tim: crate::Reg<peri_sys_rst_tim::PERI_SYS_RST_TIM_SPEC>,
    #[doc = "0x2054 - UART host module reset timing control register"]
    pub uart_rst_tim: crate::Reg<uart_rst_tim::UART_RST_TIM_SPEC>,
    #[doc = "0x2058..0x2068 - UART\\[i\\]
host module reset control register"]
    pub uart_rst_ctl: [crate::Reg<uart_rst_ctl::UART_RST_CTL_SPEC>; 4],
    #[doc = "0x2068 - I2S slave module reset timing control register"]
    pub i2s_rst_tim: crate::Reg<i2s_rst_tim::I2S_RST_TIM_SPEC>,
    #[doc = "0x206c - I2S 2 slave module reset control register"]
    pub i2s2_rst_ctl: crate::Reg<i2s2_rst_ctl::I2S2_RST_CTL_SPEC>,
    #[doc = "0x2070 - SPI module reset timing control register"]
    pub spi_rst_tim: crate::Reg<spi_rst_tim::SPI_RST_TIM_SPEC>,
    #[doc = "0x2074 - SPI 0 host module reset control register"]
    pub spi0_rst_ctl: crate::Reg<spi0_rst_ctl::SPI0_RST_CTL_SPEC>,
    #[doc = "0x2078 - SPI 1 host module reset control register"]
    pub spi1_rst_ctl: crate::Reg<spi1_rst_ctl::SPI1_RST_CTL_SPEC>,
    #[doc = "0x207c - SPI 2 host module reset control register"]
    pub spi2_rst_ctl: crate::Reg<spi2_rst_ctl::SPI2_RST_CTL_SPEC>,
    #[doc = "0x2080 - SPI 3 slave module reset control register"]
    pub spi3_rst_ctl: crate::Reg<spi3_rst_ctl::SPI3_RST_CTL_SPEC>,
    #[doc = "0x2084 - Audio Interface module reset timing control register"]
    pub audif_rst_tim: crate::Reg<audif_rst_tim::AUDIF_RST_TIM_SPEC>,
    #[doc = "0x2088 - Audio Interface module reset control register"]
    pub audif_rst_ctl: crate::Reg<audif_rst_ctl::AUDIF_RST_CTL_SPEC>,
    #[doc = "0x208c - Security subsystem reset timing control register"]
    pub sec_sys_rst_tim: crate::Reg<sec_sys_rst_tim::SEC_SYS_RST_TIM_SPEC>,
    #[doc = "0x2090 - SHA module reset timing control register"]
    pub sha_rst_tim: crate::Reg<sha_rst_tim::SHA_RST_TIM_SPEC>,
    #[doc = "0x2094 - SHA module reset control register"]
    pub sha_rst_ctl: crate::Reg<sha_rst_ctl::SHA_RST_CTL_SPEC>,
    #[doc = "0x2098 - AES module reset timing control register"]
    pub aes_rst_tim: crate::Reg<aes_rst_tim::AES_RST_TIM_SPEC>,
    #[doc = "0x209c - AES module reset control register"]
    pub aes_rst_ctl: crate::Reg<aes_rst_ctl::AES_RST_CTL_SPEC>,
    #[doc = "0x20a0 - RSA module reset timing control register"]
    pub rsa_rst_tim: crate::Reg<rsa_rst_tim::RSA_RST_TIM_SPEC>,
    #[doc = "0x20a4 - RSA module reset control register"]
    pub rsa_rst_ctl: crate::Reg<rsa_rst_ctl::RSA_RST_CTL_SPEC>,
    #[doc = "0x20a8 - ROM module reset timing control register"]
    pub rom_rst_tim: crate::Reg<rom_rst_tim::ROM_RST_TIM_SPEC>,
    #[doc = "0x20ac - ROM module reset control register"]
    pub rom_rst_ctl: crate::Reg<rom_rst_ctl::ROM_RST_CTL_SPEC>,
    #[doc = "0x20b0 - OTP module reset timing control register"]
    pub otp_rst_tim: crate::Reg<otp_rst_tim::OTP_RST_TIM_SPEC>,
    #[doc = "0x20b4 - OTP module reset control register"]
    pub otp_rst_ctl: crate::Reg<otp_rst_ctl::OTP_RST_CTL_SPEC>,
    _reserved120: [u8; 0x08],
    #[doc = "0x20c0 - Storage subsystem reset timing control register"]
    pub stor_sys_rst_tim: crate::Reg<stor_sys_rst_tim::STOR_SYS_RST_TIM_SPEC>,
    #[doc = "0x20c4 - SD host controllers reset timing control register"]
    pub sdctl_rst_tim: crate::Reg<sdctl_rst_tim::SDCTL_RST_TIM_SPEC>,
    #[doc = "0x20c8 - SD host controller 0 reset control register"]
    pub sdc0_rst_ctl: crate::Reg<sdc0_rst_ctl::SDC0_RST_CTL_SPEC>,
    #[doc = "0x20cc - SD host controller 1 reset control register"]
    pub sdc1_rst_ctl: crate::Reg<sdc1_rst_ctl::SDC1_RST_CTL_SPEC>,
    #[doc = "0x20d0 - SD host controller 2 reset control register"]
    pub sdc2_rst_ctl: crate::Reg<sdc2_rst_ctl::SDC2_RST_CTL_SPEC>,
    #[doc = "0x20d4 - DMA controllers reset timing control register"]
    pub dmac_rst_tim: crate::Reg<dmac_rst_tim::DMAC_RST_TIM_SPEC>,
    #[doc = "0x20d8 - Peripheral DMAC reset control register"]
    pub peri_dma_rst_ctl: crate::Reg<peri_dma_rst_ctl::PERI_DMA_RST_CTL_SPEC>,
    #[doc = "0x20dc - Memory DMAC reset control register"]
    pub sys_dma_rst_ctl: crate::Reg<sys_dma_rst_ctl::SYS_DMA_RST_CTL_SPEC>,
    #[doc = "0x20e0 - EMAC host controllers reset timing control register"]
    pub emac_rst_tim: crate::Reg<emac_rst_tim::EMAC_RST_TIM_SPEC>,
    #[doc = "0x20e4 - EMAC host controllers reset control register"]
    pub emac_rst_ctl: crate::Reg<emac_rst_ctl::EMAC_RST_CTL_SPEC>,
    #[doc = "0x20e8 - SDIO slave controller reset timing control register"]
    pub sdio_rst_tim: crate::Reg<sdio_rst_tim::SDIO_RST_TIM_SPEC>,
    #[doc = "0x20ec - SD slave controllers reset control register"]
    pub sdio_rst_ctl: crate::Reg<sdio_rst_ctl::SDIO_RST_CTL_SPEC>,
    #[doc = "0x20f0 - Memory controller reset timing control register"]
    pub mctl_rst_tim: crate::Reg<mctl_rst_tim::MCTL_RST_TIM_SPEC>,
    #[doc = "0x20f4 - Memory controller module reset control register"]
    pub mctl_rst_ctl: crate::Reg<mctl_rst_ctl::MCTL_RST_CTL_SPEC>,
    _reserved134: [u8; 0x08],
    #[doc = "0x2100 - SRAM block 0 reset timing control register"]
    pub sram0_rst_tim: crate::Reg<sram0_rst_tim::SRAM0_RST_TIM_SPEC>,
    #[doc = "0x2104 - SRAM block 0 module reset control register"]
    pub sram0_rst_ctl: crate::Reg<sram0_rst_ctl::SRAM0_RST_CTL_SPEC>,
    _reserved136: [u8; 0x08],
    #[doc = "0x2110 - SRAM block 1 reset timing control register"]
    pub sram1_rst_tim: crate::Reg<sram1_rst_tim::SRAM1_RST_TIM_SPEC>,
    #[doc = "0x2114 - SRAM block 1 module reset control register"]
    pub sram1_rst_ctl: crate::Reg<sram1_rst_ctl::SRAM1_RST_CTL_SPEC>,
    #[doc = "0x2118 - Video subsystem reset timing control register"]
    pub isp_sys_rst_tim: crate::Reg<isp_sys_rst_tim::ISP_SYS_RST_TIM_SPEC>,
    #[doc = "0x211c - ISP module reset timing control register"]
    pub isp_rst_tim: crate::Reg<isp_rst_tim::ISP_RST_TIM_SPEC>,
    _reserved140: [u8; 0x04],
    #[doc = "0x2124 - ISP_F2K module reset control register"]
    pub isp_f2k_rst_ctl: crate::Reg<isp_f2k_rst_ctl::ISP_F2K_RST_CTL_SPEC>,
    #[doc = "0x2128 - ISP_R2K module reset control register"]
    pub isp_r2k_rst_ctl: crate::Reg<isp_r2k_rst_ctl::ISP_R2K_RST_CTL_SPEC>,
    #[doc = "0x212c - ISP_TOF module reset control register"]
    pub isp_tof_rst_ctl: crate::Reg<isp_tof_rst_ctl::ISP_TOF_RST_CTL_SPEC>,
    #[doc = "0x2130 - CSI module reset timing control register"]
    pub csi_rst_tim: crate::Reg<csi_rst_tim::CSI_RST_TIM_SPEC>,
    #[doc = "0x2134 - CSI0 module reset control register"]
    pub csi0_rst_ctl: crate::Reg<csi0_rst_ctl::CSI0_RST_CTL_SPEC>,
    #[doc = "0x2138 - CSI1 module reset control register"]
    pub csi1_rst_ctl: crate::Reg<csi1_rst_ctl::CSI1_RST_CTL_SPEC>,
    _reserved146: [u8; 0x08],
    #[doc = "0x2144 - SENSOR reset timing control register"]
    pub sensor_rst_tim: crate::Reg<sensor_rst_tim::SENSOR_RST_TIM_SPEC>,
    #[doc = "0x2148 - SENSOR reset control register"]
    pub sensor_rst_ctl: crate::Reg<sensor_rst_ctl::SENSOR_RST_CTL_SPEC>,
    #[doc = "0x214c - VI module reset timing control register"]
    pub vi_rst_tim: crate::Reg<vi_rst_tim::VI_RST_TIM_SPEC>,
    #[doc = "0x2150 - VI module reset control register"]
    pub vi_rst_ctl: crate::Reg<vi_rst_ctl::VI_RST_CTL_SPEC>,
    #[doc = "0x2154 - MFBC module reset timing control register"]
    pub mfbc_rst_tim: crate::Reg<mfbc_rst_tim::MFBC_RST_TIM_SPEC>,
    #[doc = "0x2158 - MFBC module reset control register"]
    pub mfbc_rst_ctl: crate::Reg<mfbc_rst_ctl::MFBC_RST_CTL_SPEC>,
    #[doc = "0x215c - Display subsystem reset timing control register"]
    pub disp_sys_rst_tim: crate::Reg<disp_sys_rst_tim::DISP_SYS_RST_TIM_SPEC>,
    #[doc = "0x2160 - DSI module reset timing control register"]
    pub dsi_rst_tim: crate::Reg<dsi_rst_tim::DSI_RST_TIM_SPEC>,
    #[doc = "0x2164 - DSI module reset control register"]
    pub dsi_rst_ctl: crate::Reg<dsi_rst_ctl::DSI_RST_CTL_SPEC>,
    #[doc = "0x2168 - BT1120 module reset timing control register"]
    pub bt1120_rst_tim: crate::Reg<bt1120_rst_tim::BT1120_RST_TIM_SPEC>,
    #[doc = "0x216c - BT1120 module reset control register"]
    pub bt1120_rst_ctl: crate::Reg<bt1120_rst_ctl::BT1120_RST_CTL_SPEC>,
    #[doc = "0x2170 - TWOD module reset timing control register"]
    pub twod_rst_tim: crate::Reg<twod_rst_tim::TWOD_RST_TIM_SPEC>,
    #[doc = "0x2174 - TWOD module reset control register"]
    pub twod_rst_ctl: crate::Reg<twod_rst_ctl::TWOD_RST_CTL_SPEC>,
    #[doc = "0x2178 - VO module reset timing control register"]
    pub vo_rst_tim: crate::Reg<vo_rst_tim::VO_RST_TIM_SPEC>,
    #[doc = "0x217c - VO module reset control register"]
    pub vo_rst_ctl: crate::Reg<vo_rst_ctl::VO_RST_CTL_SPEC>,
    #[doc = "0x2180 - H264 module reset timing control register"]
    pub h264_rst_tim: crate::Reg<h264_rst_tim::H264_RST_TIM_SPEC>,
    #[doc = "0x2184 - H264 module reset control register"]
    pub h264_rst_ctl: crate::Reg<h264_rst_ctl::H264_RST_CTL_SPEC>,
    #[doc = "0x2188 - USB module reset timing control register"]
    pub usb_rst_tim: crate::Reg<usb_rst_tim::USB_RST_TIM_SPEC>,
    #[doc = "0x218c - USB module reset control register"]
    pub usb_rst_ctl: crate::Reg<usb_rst_ctl::USB_RST_CTL_SPEC>,
    #[doc = "0x2190 - MIPI CORNER module reset timing control register"]
    pub mipi_corner_rst_tim: crate::Reg<mipi_corner_rst_tim::MIPI_CORNER_RST_TIM_SPEC>,
    #[doc = "0x2194 - MIPI CORNER module reset control register"]
    pub mipi_corner_rst_ctl: crate::Reg<mipi_corner_rst_ctl::MIPI_CORNER_RST_CTL_SPEC>,
    #[doc = "0x2198 - DDR PHY reset control register"]
    pub ddr_phy_rst_ctl: crate::Reg<ddr_phy_rst_ctl::DDR_PHY_RST_CTL_SPEC>,
    _reserved168: [u8; 0x0e64],
    #[doc = "0x3000 - AX25M dual-core processor power controller timing register"]
    pub ax25m_pwr_tim: crate::Reg<ax25m_pwr_tim::AX25M_PWR_TIM_SPEC>,
    #[doc = "0x3004 - AX25M domain NOC power controller LPI timing register"]
    pub ax25m_lpi_tim: crate::Reg<ax25m_lpi_tim::AX25M_LPI_TIM_SPEC>,
    #[doc = "0x3008 - AX25M NOC power controller low-power interface control register"]
    pub ax25m_lpi_ctl: crate::Reg<ax25m_lpi_ctl::AX25M_LPI_CTL_SPEC>,
    #[doc = "0x300c - AX25M power domain power status control register"]
    pub ax25m_pwr_ctl: crate::Reg<ax25m_pwr_ctl::AX25M_PWR_CTL_SPEC>,
    #[doc = "0x3010 - AX25M NOC power controller low-power interface status register"]
    pub ax25m_lpi_stat: crate::Reg<ax25m_lpi_stat::AX25M_LPI_STAT_SPEC>,
    #[doc = "0x3014 - AX25M power domain current power status register"]
    pub ax25m_pwr_stat: crate::Reg<ax25m_pwr_stat::AX25M_PWR_STAT_SPEC>,
    #[doc = "0x3018 - AX25M dual-core processor WFI signal waiting timeout configure register"]
    pub ax25m_wfi_tim: crate::Reg<ax25m_wfi_tim::AX25M_WFI_TIM_SPEC>,
    _reserved175: [u8; 0x04],
    #[doc = "0x3020 - AX25P single-core processor power controller timing register"]
    pub ax25p_pwr_tim: crate::Reg<ax25p_pwr_tim::AX25P_PWR_TIM_SPEC>,
    #[doc = "0x3024 - AX25P domain NOC power controller LPI timing register"]
    pub ax25p_lpi_tim: crate::Reg<ax25p_lpi_tim::AX25P_LPI_TIM_SPEC>,
    #[doc = "0x3028 - AX25P NOC power controller low-power interface control register"]
    pub ax25p_lpi_ctl: crate::Reg<ax25p_lpi_ctl::AX25P_LPI_CTL_SPEC>,
    #[doc = "0x302c - AX25P power domain power state control register"]
    pub ax25p_pwr_ctl: crate::Reg<ax25p_pwr_ctl::AX25P_PWR_CTL_SPEC>,
    #[doc = "0x3030 - AX25P NOC power controller low-power interface status register"]
    pub ax25p_lpi_stat: crate::Reg<ax25p_lpi_stat::AX25P_LPI_STAT_SPEC>,
    #[doc = "0x3034 - AX25P power domain current power status register"]
    pub ax25p_pwr_stat: crate::Reg<ax25p_pwr_stat::AX25P_PWR_STAT_SPEC>,
    #[doc = "0x3038 - AX25P processor WFI signal waiting timeout configure register"]
    pub ax25p_wfi_tim: crate::Reg<ax25p_wfi_tim::AX25P_WFI_TIM_SPEC>,
    _reserved182: [u8; 0x24],
    #[doc = "0x3060 - GNNE power controller timing register"]
    pub gnne_pwr_tim: crate::Reg<gnne_pwr_tim::GNNE_PWR_TIM_SPEC>,
    #[doc = "0x3064 - GNNE power domain NOC power controller LPI timing register"]
    pub gnne_lpi_tim: crate::Reg<gnne_lpi_tim::GNNE_LPI_TIM_SPEC>,
    #[doc = "0x3068 - GNNE power domain NOC power controller low- power interface control register"]
    pub gnne_lpi_ctl: crate::Reg<gnne_lpi_ctl::GNNE_LPI_CTL_SPEC>,
    #[doc = "0x306c - GNNE power domain power state control register"]
    pub gnne_pwr_ctl: crate::Reg<gnne_pwr_ctl::GNNE_PWR_CTL_SPEC>,
    #[doc = "0x3070 - GNNE NOC power controller low-power interface status register"]
    pub gnne_lpi_stat: crate::Reg<gnne_lpi_stat::GNNE_LPI_STAT_SPEC>,
    #[doc = "0x3074 - GNNE power domain current power status register"]
    pub gnne_pwr_stat: crate::Reg<gnne_pwr_stat::GNNE_PWR_STAT_SPEC>,
    _reserved188: [u8; 0x08],
    #[doc = "0x3080 - Security subsystem domain power controller timing register"]
    pub sec_pwr_tim: crate::Reg<sec_pwr_tim::SEC_PWR_TIM_SPEC>,
    #[doc = "0x3084 - Security subsystem domain NOC power controller LPI timing register"]
    pub sec_lpi_tim: crate::Reg<sec_lpi_tim::SEC_LPI_TIM_SPEC>,
    #[doc = "0x3088 - Security subsystem NOC power controller low- power interface control register"]
    pub sec_lpi_ctl: crate::Reg<sec_lpi_ctl::SEC_LPI_CTL_SPEC>,
    #[doc = "0x308c - Security subsys power domain power state control register"]
    pub sec_pwr_ctl: crate::Reg<sec_pwr_ctl::SEC_PWR_CTL_SPEC>,
    #[doc = "0x3090 - Security subsystem NOC power controller low- power interface status register"]
    pub sec_lpi_stat: crate::Reg<sec_lpi_stat::SEC_LPI_STAT_SPEC>,
    #[doc = "0x3094 - Security subsystem power domain current power status register"]
    pub sec_pwr_stat: crate::Reg<sec_pwr_stat::SEC_PWR_STAT_SPEC>,
    _reserved194: [u8; 0x08],
    #[doc = "0x30a0 - Storage subsystem domain power controller timing register"]
    pub stor_pwr_tim: crate::Reg<stor_pwr_tim::STOR_PWR_TIM_SPEC>,
    #[doc = "0x30a4 - Storage subsystem domain NOC power controller LPI timing register"]
    pub stor_lpi_tim: crate::Reg<stor_lpi_tim::STOR_LPI_TIM_SPEC>,
    #[doc = "0x30a8 - Storage subsystem NOC power controller low- power interface control register"]
    pub stor_lpi_ctl: crate::Reg<stor_lpi_ctl::STOR_LPI_CTL_SPEC>,
    #[doc = "0x30ac - Storage subsystem power domain power state control register"]
    pub stor_pwr_ctl: crate::Reg<stor_pwr_ctl::STOR_PWR_CTL_SPEC>,
    #[doc = "0x30b0 - Storage subsystem NOC power controller low- power interface status register"]
    pub stor_lpi_stat: crate::Reg<stor_lpi_stat::STOR_LPI_STAT_SPEC>,
    #[doc = "0x30b4 - Storage subsystem power domain current power status register"]
    pub stor_pwr_stat: crate::Reg<stor_pwr_stat::STOR_PWR_STAT_SPEC>,
    _reserved200: [u8; 0x08],
    #[doc = "0x30c0 - Peripheral subsystem domain power controller timing register"]
    pub peri_pwr_tim: crate::Reg<peri_pwr_tim::PERI_PWR_TIM_SPEC>,
    #[doc = "0x30c4 - Peripheral subsystem domain NOC power controller LPI timing register"]
    pub peri_lpi_tim: crate::Reg<peri_lpi_tim::PERI_LPI_TIM_SPEC>,
    #[doc = "0x30c8 - Peripheral subsystem NOC power controller low- power interface control register"]
    pub peri_lpi_ctl: crate::Reg<peri_lpi_ctl::PERI_LPI_CTL_SPEC>,
    #[doc = "0x30cc - Peripheral subsystem power domain power state control register"]
    pub peri_pwr_ctl: crate::Reg<peri_pwr_ctl::PERI_PWR_CTL_SPEC>,
    #[doc = "0x30d0 - Peripheral subsystem NOC power controller low- power interface status register"]
    pub peri_lpi_stat: crate::Reg<peri_lpi_stat::PERI_LPI_STAT_SPEC>,
    #[doc = "0x30d4 - Peripheral subsystem power domain current power status register"]
    pub peri_pwr_stat: crate::Reg<peri_pwr_stat::PERI_PWR_STAT_SPEC>,
    _reserved206: [u8; 0x08],
    #[doc = "0x30e0 - System Memory contorller power controller timing register"]
    pub mctl_pwr_tim0: crate::Reg<mctl_pwr_tim0::MCTL_PWR_TIM0_SPEC>,
    #[doc = "0x30e4 - System Memory contorller power controller timing register"]
    pub mctl_pwr_tim1: crate::Reg<mctl_pwr_tim1::MCTL_PWR_TIM1_SPEC>,
    #[doc = "0x30e8 - Memory Controller domain NOC power controller LPI timing register"]
    pub mctl_lpi_tim: crate::Reg<mctl_lpi_tim::MCTL_LPI_TIM_SPEC>,
    #[doc = "0x30ec - Memory Controller domain NOC power controller low-power interface control register"]
    pub mctl_lpi_ctl: crate::Reg<mctl_lpi_ctl::MCTL_LPI_CTL_SPEC>,
    #[doc = "0x30f0 - Memory Controller power domain power status control register"]
    pub mctl_pwr_ctl: crate::Reg<mctl_pwr_ctl::MCTL_PWR_CTL_SPEC>,
    #[doc = "0x30f4 - Memory Controller NOC power controller low- power interface status register"]
    pub mctl_lpi_stat: crate::Reg<mctl_lpi_stat::MCTL_LPI_STAT_SPEC>,
    #[doc = "0x30f8 - Memory Controller power domain current power status register"]
    pub mctl_pwr_stat: crate::Reg<mctl_pwr_stat::MCTL_PWR_STAT_SPEC>,
    _reserved213: [u8; 0x04],
    #[doc = "0x3100 - System SRAM block 0 power controller timing register"]
    pub sram0_pwr_tim: crate::Reg<sram0_pwr_tim::SRAM0_PWR_TIM_SPEC>,
    #[doc = "0x3104 - System SRAM block 0 domain NOC power controller LPI timing register"]
    pub sram0_lpi_tim: crate::Reg<sram0_lpi_tim::SRAM0_LPI_TIM_SPEC>,
    #[doc = "0x3108 - System SRAM block 0 domain NOC power controller low-power interface control register"]
    pub sram0_lpi_ctl: crate::Reg<sram0_lpi_ctl::SRAM0_LPI_CTL_SPEC>,
    #[doc = "0x310c - System SRAM block 0 power domain power status control register"]
    pub sram0_pwr_ctl: crate::Reg<sram0_pwr_ctl::SRAM0_PWR_CTL_SPEC>,
    #[doc = "0x3110 - System SRAM block 0 NOC power controller low- power interface status register"]
    pub sram0_lpi_stat: crate::Reg<sram0_lpi_stat::SRAM0_LPI_STAT_SPEC>,
    #[doc = "0x3114 - System SRAM block 0 power domain current power status register"]
    pub sram0_pwr_stat: crate::Reg<sram0_pwr_stat::SRAM0_PWR_STAT_SPEC>,
    _reserved219: [u8; 0x08],
    #[doc = "0x3120 - System SRAM block 1 power controller timing register"]
    pub sram1_pwr_tim: crate::Reg<sram1_pwr_tim::SRAM1_PWR_TIM_SPEC>,
    #[doc = "0x3124 - System SRAM block 1 domain NOC power controller LPI timing register"]
    pub sram1_lpi_tim: crate::Reg<sram1_lpi_tim::SRAM1_LPI_TIM_SPEC>,
    #[doc = "0x3128 - System SRAM block 1 domain NOC power controller low-power interface control register"]
    pub sram1_lpi_ctl: crate::Reg<sram1_lpi_ctl::SRAM1_LPI_CTL_SPEC>,
    #[doc = "0x312c - System SRAM block 1 power domain power status control register"]
    pub sram1_pwr_ctl: crate::Reg<sram1_pwr_ctl::SRAM1_PWR_CTL_SPEC>,
    #[doc = "0x3130 - System SRAM block 1 NOC power controller low- power interface status register"]
    pub sram1_lpi_stat: crate::Reg<sram1_lpi_stat::SRAM1_LPI_STAT_SPEC>,
    #[doc = "0x3134 - System SRAM block 1 power domain current power status register"]
    pub sram1_pwr_stat: crate::Reg<sram1_pwr_stat::SRAM1_PWR_STAT_SPEC>,
    _reserved225: [u8; 0x28],
    #[doc = "0x3160 - Display System power controller timing register"]
    pub disp_pwr_tim: crate::Reg<disp_pwr_tim::DISP_PWR_TIM_SPEC>,
    #[doc = "0x3164 - System DISP domain NOC power controller LPI timing register"]
    pub disp_lpi_tim: crate::Reg<disp_lpi_tim::DISP_LPI_TIM_SPEC>,
    #[doc = "0x3168 - System DISP domain NOC power controller low- power interface control register"]
    pub disp_lpi_ctl: crate::Reg<disp_lpi_ctl::DISP_LPI_CTL_SPEC>,
    #[doc = "0x316c - System DISP power domain power status control register"]
    pub disp_pwr_ctl: crate::Reg<disp_pwr_ctl::DISP_PWR_CTL_SPEC>,
    #[doc = "0x3170 - System DISP NOC power controller low-power interface status register"]
    pub disp_lpi_stat: crate::Reg<disp_lpi_stat::DISP_LPI_STAT_SPEC>,
    #[doc = "0x3174 - System DISP power domain current power status register"]
    pub disp_pwr_stat: crate::Reg<disp_pwr_stat::DISP_PWR_STAT_SPEC>,
    _reserved231: [u8; 0x08],
    #[doc = "0x3180 - System H264 power controller timing register"]
    pub h264_pwr_tim: crate::Reg<h264_pwr_tim::H264_PWR_TIM_SPEC>,
    #[doc = "0x3184 - System H264 domain NOC power controller LPI timing register"]
    pub h264_lpi_tim: crate::Reg<h264_lpi_tim::H264_LPI_TIM_SPEC>,
    #[doc = "0x3188 - System H264 domain NOC power controller low- power interface control register"]
    pub h264_lpi_ctl: crate::Reg<h264_lpi_ctl::H264_LPI_CTL_SPEC>,
    #[doc = "0x318c - System H264 power domain power status control register"]
    pub h264_pwr_ctl: crate::Reg<h264_pwr_ctl::H264_PWR_CTL_SPEC>,
    #[doc = "0x3190 - System H264 NOC power controller low-power interface status register"]
    pub h264_lpi_stat: crate::Reg<h264_lpi_stat::H264_LPI_STAT_SPEC>,
    #[doc = "0x3194 - System H264 power domain current power status register"]
    pub h264_pwr_stat: crate::Reg<h264_pwr_stat::H264_PWR_STAT_SPEC>,
    _reserved237: [u8; 0x08],
    #[doc = "0x31a0 - System USB power controller timing register"]
    pub usb_pwr_tim: crate::Reg<usb_pwr_tim::USB_PWR_TIM_SPEC>,
    #[doc = "0x31a4 - System USB domain NOC power controller LPI timing register"]
    pub usb_lpi_tim: crate::Reg<usb_lpi_tim::USB_LPI_TIM_SPEC>,
    #[doc = "0x31a8 - System USB domain NOC power controller low- power interface control register"]
    pub usb_lpi_ctl: crate::Reg<usb_lpi_ctl::USB_LPI_CTL_SPEC>,
    #[doc = "0x31ac - System USB power domain power status control register"]
    pub usb_pwr_ctl: crate::Reg<usb_pwr_ctl::USB_PWR_CTL_SPEC>,
    #[doc = "0x31b0 - System USB NOC power controller low-power interface status register"]
    pub usb_lpi_stat: crate::Reg<usb_lpi_stat::USB_LPI_STAT_SPEC>,
    #[doc = "0x31b4 - System USB power domain current power status register"]
    pub usb_pwr_stat: crate::Reg<usb_pwr_stat::USB_PWR_STAT_SPEC>,
    _reserved243: [u8; 0x08],
    #[doc = "0x31c0 - System ISP power controller timing register"]
    pub isp_pwr_tim: crate::Reg<isp_pwr_tim::ISP_PWR_TIM_SPEC>,
    #[doc = "0x31c4 - System ISP domain NOC power controller LPI timing register"]
    pub isp_lpi_tim: crate::Reg<isp_lpi_tim::ISP_LPI_TIM_SPEC>,
    #[doc = "0x31c8 - System ISP domain NOC power controller low- power interface control register"]
    pub isp_lpi_ctl: crate::Reg<isp_lpi_ctl::ISP_LPI_CTL_SPEC>,
    #[doc = "0x31cc - System ISP power domain power status control register"]
    pub isp_pwr_ctl: crate::Reg<isp_pwr_ctl::ISP_PWR_CTL_SPEC>,
    #[doc = "0x31d0 - ISP NOC power controller low-power interface status register"]
    pub isp_lpi_stat: crate::Reg<isp_lpi_stat::ISP_LPI_STAT_SPEC>,
    #[doc = "0x31d4 - ISP power domain current power status register"]
    pub isp_pwr_stat: crate::Reg<isp_pwr_stat::ISP_PWR_STAT_SPEC>,
    _reserved249: [u8; 0x80],
    #[doc = "0x3258 - power domain group repair en"]
    pub pwr_dom_grp_en: crate::Reg<pwr_dom_grp_en::PWR_DOM_GRP_EN_SPEC>,
    #[doc = "0x325c - power domain group repair status"]
    pub repair_status: crate::Reg<repair_status::REPAIR_STATUS_SPEC>,
    #[doc = "0x3260 - AX25M power domain repair timer register"]
    pub ax25m_repair_tim: crate::Reg<ax25m_repair_tim::AX25M_REPAIR_TIM_SPEC>,
    #[doc = "0x3264 - AX25P power domain repair timer register"]
    pub ax25p_repair_tim: crate::Reg<ax25p_repair_tim::AX25P_REPAIR_TIM_SPEC>,
    #[doc = "0x3268 - GNNE power domain repair timer register"]
    pub gnne_repair_tim: crate::Reg<gnne_repair_tim::GNNE_REPAIR_TIM_SPEC>,
    #[doc = "0x326c - SRAM0 power domain repair timer register"]
    pub sram0_repair_tim: crate::Reg<sram0_repair_tim::SRAM0_REPAIR_TIM_SPEC>,
    #[doc = "0x3270 - SRAM1 power domain repair timer register"]
    pub sram1_repair_tim: crate::Reg<sram1_repair_tim::SRAM1_REPAIR_TIM_SPEC>,
    #[doc = "0x3274 - ISP_SYS power domain repair timer register"]
    pub isp_sys_repair_tim: crate::Reg<isp_sys_repair_tim::ISP_SYS_REPAIR_TIM_SPEC>,
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
#[doc = "NOC_CLK_CFG register accessor: an alias for `Reg<NOC_CLK_CFG_SPEC>`"]
pub type NOC_CLK_CFG = crate::Reg<noc_clk_cfg::NOC_CLK_CFG_SPEC>;
#[doc = "NOC bus clock Division and configure register"]
pub mod noc_clk_cfg;
#[doc = "PERI_SYS_BUS_CLK_CFG register accessor: an alias for `Reg<PERI_SYS_BUS_CLK_CFG_SPEC>`"]
pub type PERI_SYS_BUS_CLK_CFG = crate::Reg<peri_sys_bus_clk_cfg::PERI_SYS_BUS_CLK_CFG_SPEC>;
#[doc = "Peripheral subsystem modules bus IF clock configure register"]
pub mod peri_sys_bus_clk_cfg;
#[doc = "PERI_SYS_BUS_CLK_EN register accessor: an alias for `Reg<PERI_SYS_BUS_CLK_EN_SPEC>`"]
pub type PERI_SYS_BUS_CLK_EN = crate::Reg<peri_sys_bus_clk_en::PERI_SYS_BUS_CLK_EN_SPEC>;
#[doc = "Peripheral subsystem modules bus IF clock enable register"]
pub mod peri_sys_bus_clk_en;
#[doc = "UART_SCLK_CFG register accessor: an alias for `Reg<UART_SCLK_CFG_SPEC>`"]
pub type UART_SCLK_CFG = crate::Reg<uart_sclk_cfg::UART_SCLK_CFG_SPEC>;
#[doc = "UART\\[i\\]
host module serial interface clock configure register"]
pub mod uart_sclk_cfg;
#[doc = "I2S2_SCLK_CFG register accessor: an alias for `Reg<I2S2_SCLK_CFG_SPEC>`"]
pub type I2S2_SCLK_CFG = crate::Reg<i2s2_sclk_cfg::I2S2_SCLK_CFG_SPEC>;
#[doc = "I2S 2 slave module serial interface clock configure register"]
pub mod i2s2_sclk_cfg;
#[doc = "SPI_SCLK_CFG register accessor: an alias for `Reg<SPI_SCLK_CFG_SPEC>`"]
pub type SPI_SCLK_CFG = crate::Reg<spi_sclk_cfg::SPI_SCLK_CFG_SPEC>;
#[doc = "SPI\\[i\\]
host module serial interface clock configure register"]
pub mod spi_sclk_cfg;
#[doc = "AUDIF_SCLK_CFG register accessor: an alias for `Reg<AUDIF_SCLK_CFG_SPEC>`"]
pub type AUDIF_SCLK_CFG = crate::Reg<audif_sclk_cfg::AUDIF_SCLK_CFG_SPEC>;
#[doc = "Audio interface module serial clock configure register"]
pub mod audif_sclk_cfg;
#[doc = "AUDIF_DEVCLK_CFG register accessor: an alias for `Reg<AUDIF_DEVCLK_CFG_SPEC>`"]
pub type AUDIF_DEVCLK_CFG = crate::Reg<audif_devclk_cfg::AUDIF_DEVCLK_CFG_SPEC>;
#[doc = "Audio device clock configure register"]
pub mod audif_devclk_cfg;
#[doc = "SEC_SYS_BUS_CLK_CFG register accessor: an alias for `Reg<SEC_SYS_BUS_CLK_CFG_SPEC>`"]
pub type SEC_SYS_BUS_CLK_CFG = crate::Reg<sec_sys_bus_clk_cfg::SEC_SYS_BUS_CLK_CFG_SPEC>;
#[doc = "Security Subsystem bus interface clock configure register"]
pub mod sec_sys_bus_clk_cfg;
#[doc = "SEC_SYS_BUS_CLK_EN register accessor: an alias for `Reg<SEC_SYS_BUS_CLK_EN_SPEC>`"]
pub type SEC_SYS_BUS_CLK_EN = crate::Reg<sec_sys_bus_clk_en::SEC_SYS_BUS_CLK_EN_SPEC>;
#[doc = "Security Subsystem modules bus interface clock control register"]
pub mod sec_sys_bus_clk_en;
#[doc = "OTP_CLK_EN register accessor: an alias for `Reg<OTP_CLK_EN_SPEC>`"]
pub type OTP_CLK_EN = crate::Reg<otp_clk_en::OTP_CLK_EN_SPEC>;
#[doc = "OTP modules clock control register"]
pub mod otp_clk_en;
#[doc = "SRAM_BUS_CLK_EN register accessor: an alias for `Reg<SRAM_BUS_CLK_EN_SPEC>`"]
pub type SRAM_BUS_CLK_EN = crate::Reg<sram_bus_clk_en::SRAM_BUS_CLK_EN_SPEC>;
#[doc = "SRAM block 0/1 bus clock enable control register"]
pub mod sram_bus_clk_en;
#[doc = "SOC_CTL_PCLK_EN register accessor: an alias for `Reg<SOC_CTL_PCLK_EN_SPEC>`"]
pub type SOC_CTL_PCLK_EN = crate::Reg<soc_ctl_pclk_en::SOC_CTL_PCLK_EN_SPEC>;
#[doc = "SOC Control Units slave APB clock enable control register"]
pub mod soc_ctl_pclk_en;
#[doc = "SOC_CTL_PCLK_EN1 register accessor: an alias for `Reg<SOC_CTL_PCLK_EN1_SPEC>`"]
pub type SOC_CTL_PCLK_EN1 = crate::Reg<soc_ctl_pclk_en1::SOC_CTL_PCLK_EN1_SPEC>;
#[doc = "SOC Control Units slave APB clock enable control register"]
pub mod soc_ctl_pclk_en1;
#[doc = "I2C_ICCLK_CFG register accessor: an alias for `Reg<I2C_ICCLK_CFG_SPEC>`"]
pub type I2C_ICCLK_CFG = crate::Reg<i2c_icclk_cfg::I2C_ICCLK_CFG_SPEC>;
#[doc = "I2C\\[i\\]
host module serial clock configure register"]
pub mod i2c_icclk_cfg;
#[doc = "WDT_TCLK_CFG register accessor: an alias for `Reg<WDT_TCLK_CFG_SPEC>`"]
pub type WDT_TCLK_CFG = crate::Reg<wdt_tclk_cfg::WDT_TCLK_CFG_SPEC>;
#[doc = "WDT\\[i\\]
module tick clock configure register"]
pub mod wdt_tclk_cfg;
#[doc = "TIMER_TCLK_SRC register accessor: an alias for `Reg<TIMER_TCLK_SRC_SPEC>`"]
pub type TIMER_TCLK_SRC = crate::Reg<timer_tclk_src::TIMER_TCLK_SRC_SPEC>;
#[doc = "System Timer module tick clocks source configure register"]
pub mod timer_tclk_src;
#[doc = "TIMER_TCLK_CFG register accessor: an alias for `Reg<TIMER_TCLK_CFG_SPEC>`"]
pub type TIMER_TCLK_CFG = crate::Reg<timer_tclk_cfg::TIMER_TCLK_CFG_SPEC>;
#[doc = "System Timer module tick clocks division configure register"]
pub mod timer_tclk_cfg;
#[doc = "TIMER_TCLK_CFG1 register accessor: an alias for `Reg<TIMER_TCLK_CFG1_SPEC>`"]
pub type TIMER_TCLK_CFG1 = crate::Reg<timer_tclk_cfg1::TIMER_TCLK_CFG1_SPEC>;
#[doc = "System Timer module tick clocks division configure register"]
pub mod timer_tclk_cfg1;
#[doc = "VAD_SCLK_CFG register accessor: an alias for `Reg<VAD_SCLK_CFG_SPEC>`"]
pub type VAD_SCLK_CFG = crate::Reg<vad_sclk_cfg::VAD_SCLK_CFG_SPEC>;
#[doc = "VAD module audio data serial clock configure register"]
pub mod vad_sclk_cfg;
#[doc = "STOR_SYS_BUS_CLK_EN register accessor: an alias for `Reg<STOR_SYS_BUS_CLK_EN_SPEC>`"]
pub type STOR_SYS_BUS_CLK_EN = crate::Reg<stor_sys_bus_clk_en::STOR_SYS_BUS_CLK_EN_SPEC>;
#[doc = "Storage subsystem modules bus IF clock enable register"]
pub mod stor_sys_bus_clk_en;
#[doc = "EMAC_TRX_CLK_CFG register accessor: an alias for `Reg<EMAC_TRX_CLK_CFG_SPEC>`"]
pub type EMAC_TRX_CLK_CFG = crate::Reg<emac_trx_clk_cfg::EMAC_TRX_CLK_CFG_SPEC>;
#[doc = "EMAC-PHY interface transceiver clock configure register"]
pub mod emac_trx_clk_cfg;
#[doc = "SD_CARD_CLK_CFG register accessor: an alias for `Reg<SD_CARD_CLK_CFG_SPEC>`"]
pub type SD_CARD_CLK_CFG = crate::Reg<sd_card_clk_cfg::SD_CARD_CLK_CFG_SPEC>;
#[doc = "EMAC-PHY interface transceiver clock configure register"]
pub mod sd_card_clk_cfg;
#[doc = "SENSOR_CLK_EN register accessor: an alias for `Reg<SENSOR_CLK_EN_SPEC>`"]
pub type SENSOR_CLK_EN = crate::Reg<sensor_clk_en::SENSOR_CLK_EN_SPEC>;
#[doc = "SENSOR clock enable register"]
pub mod sensor_clk_en;
#[doc = "ISP_SYS_PCLK_EN register accessor: an alias for `Reg<ISP_SYS_PCLK_EN_SPEC>`"]
pub type ISP_SYS_PCLK_EN = crate::Reg<isp_sys_pclk_en::ISP_SYS_PCLK_EN_SPEC>;
#[doc = "ISP system APB clock enable control register"]
pub mod isp_sys_pclk_en;
#[doc = "ISP_SYS_ACLK_EN register accessor: an alias for `Reg<ISP_SYS_ACLK_EN_SPEC>`"]
pub type ISP_SYS_ACLK_EN = crate::Reg<isp_sys_aclk_en::ISP_SYS_ACLK_EN_SPEC>;
#[doc = "ISP system APB clock enable control register"]
pub mod isp_sys_aclk_en;
#[doc = "DISP_SYS_PCLK_EN register accessor: an alias for `Reg<DISP_SYS_PCLK_EN_SPEC>`"]
pub type DISP_SYS_PCLK_EN = crate::Reg<disp_sys_pclk_en::DISP_SYS_PCLK_EN_SPEC>;
#[doc = "DISP system APB clock enable control register"]
pub mod disp_sys_pclk_en;
#[doc = "DISP_SYS_ACLK_EN register accessor: an alias for `Reg<DISP_SYS_ACLK_EN_SPEC>`"]
pub type DISP_SYS_ACLK_EN = crate::Reg<disp_sys_aclk_en::DISP_SYS_ACLK_EN_SPEC>;
#[doc = "DISP system APB clock enable control register"]
pub mod disp_sys_aclk_en;
#[doc = "TPG_PIXEL_CLK_CFG register accessor: an alias for `Reg<TPG_PIXEL_CLK_CFG_SPEC>`"]
pub type TPG_PIXEL_CLK_CFG = crate::Reg<tpg_pixel_clk_cfg::TPG_PIXEL_CLK_CFG_SPEC>;
#[doc = "VI module tpg pixel clock configure register"]
pub mod tpg_pixel_clk_cfg;
#[doc = "CSI_PIXEL_CLK_CFG register accessor: an alias for `Reg<CSI_PIXEL_CLK_CFG_SPEC>`"]
pub type CSI_PIXEL_CLK_CFG = crate::Reg<csi_pixel_clk_cfg::CSI_PIXEL_CLK_CFG_SPEC>;
#[doc = "CSI\\[i\\]
module pixel clock configure register"]
pub mod csi_pixel_clk_cfg;
#[doc = "DISP_PIXEL_CLK_CFG register accessor: an alias for `Reg<DISP_PIXEL_CLK_CFG_SPEC>`"]
pub type DISP_PIXEL_CLK_CFG = crate::Reg<disp_pixel_clk_cfg::DISP_PIXEL_CLK_CFG_SPEC>;
#[doc = "DISP module pixel clock configure register"]
pub mod disp_pixel_clk_cfg;
#[doc = "CSI_SYS_CLK_CFG register accessor: an alias for `Reg<CSI_SYS_CLK_CFG_SPEC>`"]
pub type CSI_SYS_CLK_CFG = crate::Reg<csi_sys_clk_cfg::CSI_SYS_CLK_CFG_SPEC>;
#[doc = "CSI\\[i\\]
module system clock configure register"]
pub mod csi_sys_clk_cfg;
#[doc = "DSI_SYS_CLK_CFG register accessor: an alias for `Reg<DSI_SYS_CLK_CFG_SPEC>`"]
pub type DSI_SYS_CLK_CFG = crate::Reg<dsi_sys_clk_cfg::DSI_SYS_CLK_CFG_SPEC>;
#[doc = "DSI module system clock configure register"]
pub mod dsi_sys_clk_cfg;
#[doc = "H264_ACLK_EN register accessor: an alias for `Reg<H264_ACLK_EN_SPEC>`"]
pub type H264_ACLK_EN = crate::Reg<h264_aclk_en::H264_ACLK_EN_SPEC>;
#[doc = "H264 module AXI clock enable control register"]
pub mod h264_aclk_en;
#[doc = "USB_CLK_EN register accessor: an alias for `Reg<USB_CLK_EN_SPEC>`"]
pub type USB_CLK_EN = crate::Reg<usb_clk_en::USB_CLK_EN_SPEC>;
#[doc = "USB host module clock enable control register"]
pub mod usb_clk_en;
#[doc = "TXDPHY_CLK_EN register accessor: an alias for `Reg<TXDPHY_CLK_EN_SPEC>`"]
pub type TXDPHY_CLK_EN = crate::Reg<txdphy_clk_en::TXDPHY_CLK_EN_SPEC>;
#[doc = "MIPI TXDPHY clock enable control register"]
pub mod txdphy_clk_en;
#[doc = "RXDPHY_CLK_EN register accessor: an alias for `Reg<RXDPHY_CLK_EN_SPEC>`"]
pub type RXDPHY_CLK_EN = crate::Reg<rxdphy_clk_en::RXDPHY_CLK_EN_SPEC>;
#[doc = "MIPI RXDPHY clock enable control register"]
pub mod rxdphy_clk_en;
#[doc = "MEM_CTL_CMD_FIFO register accessor: an alias for `Reg<MEM_CTL_CMD_FIFO_SPEC>`"]
pub type MEM_CTL_CMD_FIFO = crate::Reg<mem_ctl_cmd_fifo::MEM_CTL_CMD_FIFO_SPEC>;
#[doc = "Memory Controller DFS Command FIFO"]
pub mod mem_ctl_cmd_fifo;
#[doc = "MEM_CTL_CMD_FIFO_STS register accessor: an alias for `Reg<MEM_CTL_CMD_FIFO_STS_SPEC>`"]
pub type MEM_CTL_CMD_FIFO_STS = crate::Reg<mem_ctl_cmd_fifo_sts::MEM_CTL_CMD_FIFO_STS_SPEC>;
#[doc = "Memory Controller DFS Command FIFO STATUS"]
pub mod mem_ctl_cmd_fifo_sts;
#[doc = "MEM_CTL_CLK_CFG register accessor: an alias for `Reg<MEM_CTL_CLK_CFG_SPEC>`"]
pub type MEM_CTL_CLK_CFG = crate::Reg<mem_ctl_clk_cfg::MEM_CTL_CLK_CFG_SPEC>;
#[doc = "Memory Controller clock configuration register"]
pub mod mem_ctl_clk_cfg;
#[doc = "MEM_CTL_DFS_CFG register accessor: an alias for `Reg<MEM_CTL_DFS_CFG_SPEC>`"]
pub type MEM_CTL_DFS_CFG = crate::Reg<mem_ctl_dfs_cfg::MEM_CTL_DFS_CFG_SPEC>;
#[doc = "Memory Controller clock DFS configuration register"]
pub mod mem_ctl_dfs_cfg;
#[doc = "MEM_CTL_CMD_FIFO_FLUSH register accessor: an alias for `Reg<MEM_CTL_CMD_FIFO_FLUSH_SPEC>`"]
pub type MEM_CTL_CMD_FIFO_FLUSH = crate::Reg<mem_ctl_cmd_fifo_flush::MEM_CTL_CMD_FIFO_FLUSH_SPEC>;
#[doc = "Memory Controller DFS Command FIFO FLUSH"]
pub mod mem_ctl_cmd_fifo_flush;
#[doc = "AX25M_RST_TIM register accessor: an alias for `Reg<AX25M_RST_TIM_SPEC>`"]
pub type AX25M_RST_TIM = crate::Reg<ax25m_rst_tim::AX25M_RST_TIM_SPEC>;
#[doc = "AX25M dual-core RISCV reset timing control register"]
pub mod ax25m_rst_tim;
#[doc = "AX25P_RST_TIM register accessor: an alias for `Reg<AX25P_RST_TIM_SPEC>`"]
pub type AX25P_RST_TIM = crate::Reg<ax25p_rst_tim::AX25P_RST_TIM_SPEC>;
#[doc = "AX25P single-core RISCV reset timing control register"]
pub mod ax25p_rst_tim;
#[doc = "AX25P_RST_CTL register accessor: an alias for `Reg<AX25P_RST_CTL_SPEC>`"]
pub type AX25P_RST_CTL = crate::Reg<ax25p_rst_ctl::AX25P_RST_CTL_SPEC>;
#[doc = "AX25P single-core RISCV reset control register"]
pub mod ax25p_rst_ctl;
#[doc = "GNNE_RST_TIM register accessor: an alias for `Reg<GNNE_RST_TIM_SPEC>`"]
pub type GNNE_RST_TIM = crate::Reg<gnne_rst_tim::GNNE_RST_TIM_SPEC>;
#[doc = "GNNE reset timing control register"]
pub mod gnne_rst_tim;
#[doc = "GNNE_RST_CTL register accessor: an alias for `Reg<GNNE_RST_CTL_SPEC>`"]
pub type GNNE_RST_CTL = crate::Reg<gnne_rst_ctl::GNNE_RST_CTL_SPEC>;
#[doc = "GNNE reset control register"]
pub mod gnne_rst_ctl;
#[doc = "SOC_CTL_RST_CTL register accessor: an alias for `Reg<SOC_CTL_RST_CTL_SPEC>`"]
pub type SOC_CTL_RST_CTL = crate::Reg<soc_ctl_rst_ctl::SOC_CTL_RST_CTL_SPEC>;
#[doc = "SOC Control subsystem reseting control register"]
pub mod soc_ctl_rst_ctl;
#[doc = "SOC_CTL_RST_CTL1 register accessor: an alias for `Reg<SOC_CTL_RST_CTL1_SPEC>`"]
pub type SOC_CTL_RST_CTL1 = crate::Reg<soc_ctl_rst_ctl1::SOC_CTL_RST_CTL1_SPEC>;
#[doc = "SOC Control subsystem reseting control register"]
pub mod soc_ctl_rst_ctl1;
#[doc = "PERI_SYS_RST_TIM register accessor: an alias for `Reg<PERI_SYS_RST_TIM_SPEC>`"]
pub type PERI_SYS_RST_TIM = crate::Reg<peri_sys_rst_tim::PERI_SYS_RST_TIM_SPEC>;
#[doc = "Peripheral subsystem reset timing control register"]
pub mod peri_sys_rst_tim;
#[doc = "UART_RST_TIM register accessor: an alias for `Reg<UART_RST_TIM_SPEC>`"]
pub type UART_RST_TIM = crate::Reg<uart_rst_tim::UART_RST_TIM_SPEC>;
#[doc = "UART host module reset timing control register"]
pub mod uart_rst_tim;
#[doc = "UART_RST_CTL register accessor: an alias for `Reg<UART_RST_CTL_SPEC>`"]
pub type UART_RST_CTL = crate::Reg<uart_rst_ctl::UART_RST_CTL_SPEC>;
#[doc = "UART\\[i\\]
host module reset control register"]
pub mod uart_rst_ctl;
#[doc = "I2S_RST_TIM register accessor: an alias for `Reg<I2S_RST_TIM_SPEC>`"]
pub type I2S_RST_TIM = crate::Reg<i2s_rst_tim::I2S_RST_TIM_SPEC>;
#[doc = "I2S slave module reset timing control register"]
pub mod i2s_rst_tim;
#[doc = "I2S2_RST_CTL register accessor: an alias for `Reg<I2S2_RST_CTL_SPEC>`"]
pub type I2S2_RST_CTL = crate::Reg<i2s2_rst_ctl::I2S2_RST_CTL_SPEC>;
#[doc = "I2S 2 slave module reset control register"]
pub mod i2s2_rst_ctl;
#[doc = "SPI_RST_TIM register accessor: an alias for `Reg<SPI_RST_TIM_SPEC>`"]
pub type SPI_RST_TIM = crate::Reg<spi_rst_tim::SPI_RST_TIM_SPEC>;
#[doc = "SPI module reset timing control register"]
pub mod spi_rst_tim;
#[doc = "SPI0_RST_CTL register accessor: an alias for `Reg<SPI0_RST_CTL_SPEC>`"]
pub type SPI0_RST_CTL = crate::Reg<spi0_rst_ctl::SPI0_RST_CTL_SPEC>;
#[doc = "SPI 0 host module reset control register"]
pub mod spi0_rst_ctl;
#[doc = "SPI1_RST_CTL register accessor: an alias for `Reg<SPI1_RST_CTL_SPEC>`"]
pub type SPI1_RST_CTL = crate::Reg<spi1_rst_ctl::SPI1_RST_CTL_SPEC>;
#[doc = "SPI 1 host module reset control register"]
pub mod spi1_rst_ctl;
#[doc = "SPI2_RST_CTL register accessor: an alias for `Reg<SPI2_RST_CTL_SPEC>`"]
pub type SPI2_RST_CTL = crate::Reg<spi2_rst_ctl::SPI2_RST_CTL_SPEC>;
#[doc = "SPI 2 host module reset control register"]
pub mod spi2_rst_ctl;
#[doc = "SPI3_RST_CTL register accessor: an alias for `Reg<SPI3_RST_CTL_SPEC>`"]
pub type SPI3_RST_CTL = crate::Reg<spi3_rst_ctl::SPI3_RST_CTL_SPEC>;
#[doc = "SPI 3 slave module reset control register"]
pub mod spi3_rst_ctl;
#[doc = "AUDIF_RST_TIM register accessor: an alias for `Reg<AUDIF_RST_TIM_SPEC>`"]
pub type AUDIF_RST_TIM = crate::Reg<audif_rst_tim::AUDIF_RST_TIM_SPEC>;
#[doc = "Audio Interface module reset timing control register"]
pub mod audif_rst_tim;
#[doc = "AUDIF_RST_CTL register accessor: an alias for `Reg<AUDIF_RST_CTL_SPEC>`"]
pub type AUDIF_RST_CTL = crate::Reg<audif_rst_ctl::AUDIF_RST_CTL_SPEC>;
#[doc = "Audio Interface module reset control register"]
pub mod audif_rst_ctl;
#[doc = "SEC_SYS_RST_TIM register accessor: an alias for `Reg<SEC_SYS_RST_TIM_SPEC>`"]
pub type SEC_SYS_RST_TIM = crate::Reg<sec_sys_rst_tim::SEC_SYS_RST_TIM_SPEC>;
#[doc = "Security subsystem reset timing control register"]
pub mod sec_sys_rst_tim;
#[doc = "SHA_RST_TIM register accessor: an alias for `Reg<SHA_RST_TIM_SPEC>`"]
pub type SHA_RST_TIM = crate::Reg<sha_rst_tim::SHA_RST_TIM_SPEC>;
#[doc = "SHA module reset timing control register"]
pub mod sha_rst_tim;
#[doc = "SHA_RST_CTL register accessor: an alias for `Reg<SHA_RST_CTL_SPEC>`"]
pub type SHA_RST_CTL = crate::Reg<sha_rst_ctl::SHA_RST_CTL_SPEC>;
#[doc = "SHA module reset control register"]
pub mod sha_rst_ctl;
#[doc = "AES_RST_TIM register accessor: an alias for `Reg<AES_RST_TIM_SPEC>`"]
pub type AES_RST_TIM = crate::Reg<aes_rst_tim::AES_RST_TIM_SPEC>;
#[doc = "AES module reset timing control register"]
pub mod aes_rst_tim;
#[doc = "AES_RST_CTL register accessor: an alias for `Reg<AES_RST_CTL_SPEC>`"]
pub type AES_RST_CTL = crate::Reg<aes_rst_ctl::AES_RST_CTL_SPEC>;
#[doc = "AES module reset control register"]
pub mod aes_rst_ctl;
#[doc = "RSA_RST_TIM register accessor: an alias for `Reg<RSA_RST_TIM_SPEC>`"]
pub type RSA_RST_TIM = crate::Reg<rsa_rst_tim::RSA_RST_TIM_SPEC>;
#[doc = "RSA module reset timing control register"]
pub mod rsa_rst_tim;
#[doc = "RSA_RST_CTL register accessor: an alias for `Reg<RSA_RST_CTL_SPEC>`"]
pub type RSA_RST_CTL = crate::Reg<rsa_rst_ctl::RSA_RST_CTL_SPEC>;
#[doc = "RSA module reset control register"]
pub mod rsa_rst_ctl;
#[doc = "ROM_RST_TIM register accessor: an alias for `Reg<ROM_RST_TIM_SPEC>`"]
pub type ROM_RST_TIM = crate::Reg<rom_rst_tim::ROM_RST_TIM_SPEC>;
#[doc = "ROM module reset timing control register"]
pub mod rom_rst_tim;
#[doc = "ROM_RST_CTL register accessor: an alias for `Reg<ROM_RST_CTL_SPEC>`"]
pub type ROM_RST_CTL = crate::Reg<rom_rst_ctl::ROM_RST_CTL_SPEC>;
#[doc = "ROM module reset control register"]
pub mod rom_rst_ctl;
#[doc = "OTP_RST_TIM register accessor: an alias for `Reg<OTP_RST_TIM_SPEC>`"]
pub type OTP_RST_TIM = crate::Reg<otp_rst_tim::OTP_RST_TIM_SPEC>;
#[doc = "OTP module reset timing control register"]
pub mod otp_rst_tim;
#[doc = "OTP_RST_CTL register accessor: an alias for `Reg<OTP_RST_CTL_SPEC>`"]
pub type OTP_RST_CTL = crate::Reg<otp_rst_ctl::OTP_RST_CTL_SPEC>;
#[doc = "OTP module reset control register"]
pub mod otp_rst_ctl;
#[doc = "STOR_SYS_RST_TIM register accessor: an alias for `Reg<STOR_SYS_RST_TIM_SPEC>`"]
pub type STOR_SYS_RST_TIM = crate::Reg<stor_sys_rst_tim::STOR_SYS_RST_TIM_SPEC>;
#[doc = "Storage subsystem reset timing control register"]
pub mod stor_sys_rst_tim;
#[doc = "SDCTL_RST_TIM register accessor: an alias for `Reg<SDCTL_RST_TIM_SPEC>`"]
pub type SDCTL_RST_TIM = crate::Reg<sdctl_rst_tim::SDCTL_RST_TIM_SPEC>;
#[doc = "SD host controllers reset timing control register"]
pub mod sdctl_rst_tim;
#[doc = "SDC0_RST_CTL register accessor: an alias for `Reg<SDC0_RST_CTL_SPEC>`"]
pub type SDC0_RST_CTL = crate::Reg<sdc0_rst_ctl::SDC0_RST_CTL_SPEC>;
#[doc = "SD host controller 0 reset control register"]
pub mod sdc0_rst_ctl;
#[doc = "SDC1_RST_CTL register accessor: an alias for `Reg<SDC1_RST_CTL_SPEC>`"]
pub type SDC1_RST_CTL = crate::Reg<sdc1_rst_ctl::SDC1_RST_CTL_SPEC>;
#[doc = "SD host controller 1 reset control register"]
pub mod sdc1_rst_ctl;
#[doc = "SDC2_RST_CTL register accessor: an alias for `Reg<SDC2_RST_CTL_SPEC>`"]
pub type SDC2_RST_CTL = crate::Reg<sdc2_rst_ctl::SDC2_RST_CTL_SPEC>;
#[doc = "SD host controller 2 reset control register"]
pub mod sdc2_rst_ctl;
#[doc = "DMAC_RST_TIM register accessor: an alias for `Reg<DMAC_RST_TIM_SPEC>`"]
pub type DMAC_RST_TIM = crate::Reg<dmac_rst_tim::DMAC_RST_TIM_SPEC>;
#[doc = "DMA controllers reset timing control register"]
pub mod dmac_rst_tim;
#[doc = "PERI_DMA_RST_CTL register accessor: an alias for `Reg<PERI_DMA_RST_CTL_SPEC>`"]
pub type PERI_DMA_RST_CTL = crate::Reg<peri_dma_rst_ctl::PERI_DMA_RST_CTL_SPEC>;
#[doc = "Peripheral DMAC reset control register"]
pub mod peri_dma_rst_ctl;
#[doc = "SYS_DMA_RST_CTL register accessor: an alias for `Reg<SYS_DMA_RST_CTL_SPEC>`"]
pub type SYS_DMA_RST_CTL = crate::Reg<sys_dma_rst_ctl::SYS_DMA_RST_CTL_SPEC>;
#[doc = "Memory DMAC reset control register"]
pub mod sys_dma_rst_ctl;
#[doc = "EMAC_RST_TIM register accessor: an alias for `Reg<EMAC_RST_TIM_SPEC>`"]
pub type EMAC_RST_TIM = crate::Reg<emac_rst_tim::EMAC_RST_TIM_SPEC>;
#[doc = "EMAC host controllers reset timing control register"]
pub mod emac_rst_tim;
#[doc = "EMAC_RST_CTL register accessor: an alias for `Reg<EMAC_RST_CTL_SPEC>`"]
pub type EMAC_RST_CTL = crate::Reg<emac_rst_ctl::EMAC_RST_CTL_SPEC>;
#[doc = "EMAC host controllers reset control register"]
pub mod emac_rst_ctl;
#[doc = "SDIO_RST_TIM register accessor: an alias for `Reg<SDIO_RST_TIM_SPEC>`"]
pub type SDIO_RST_TIM = crate::Reg<sdio_rst_tim::SDIO_RST_TIM_SPEC>;
#[doc = "SDIO slave controller reset timing control register"]
pub mod sdio_rst_tim;
#[doc = "SDIO_RST_CTL register accessor: an alias for `Reg<SDIO_RST_CTL_SPEC>`"]
pub type SDIO_RST_CTL = crate::Reg<sdio_rst_ctl::SDIO_RST_CTL_SPEC>;
#[doc = "SD slave controllers reset control register"]
pub mod sdio_rst_ctl;
#[doc = "MCTL_RST_TIM register accessor: an alias for `Reg<MCTL_RST_TIM_SPEC>`"]
pub type MCTL_RST_TIM = crate::Reg<mctl_rst_tim::MCTL_RST_TIM_SPEC>;
#[doc = "Memory controller reset timing control register"]
pub mod mctl_rst_tim;
#[doc = "MCTL_RST_CTL register accessor: an alias for `Reg<MCTL_RST_CTL_SPEC>`"]
pub type MCTL_RST_CTL = crate::Reg<mctl_rst_ctl::MCTL_RST_CTL_SPEC>;
#[doc = "Memory controller module reset control register"]
pub mod mctl_rst_ctl;
#[doc = "SRAM0_RST_TIM register accessor: an alias for `Reg<SRAM0_RST_TIM_SPEC>`"]
pub type SRAM0_RST_TIM = crate::Reg<sram0_rst_tim::SRAM0_RST_TIM_SPEC>;
#[doc = "SRAM block 0 reset timing control register"]
pub mod sram0_rst_tim;
#[doc = "SRAM0_RST_CTL register accessor: an alias for `Reg<SRAM0_RST_CTL_SPEC>`"]
pub type SRAM0_RST_CTL = crate::Reg<sram0_rst_ctl::SRAM0_RST_CTL_SPEC>;
#[doc = "SRAM block 0 module reset control register"]
pub mod sram0_rst_ctl;
#[doc = "SRAM1_RST_TIM register accessor: an alias for `Reg<SRAM1_RST_TIM_SPEC>`"]
pub type SRAM1_RST_TIM = crate::Reg<sram1_rst_tim::SRAM1_RST_TIM_SPEC>;
#[doc = "SRAM block 1 reset timing control register"]
pub mod sram1_rst_tim;
#[doc = "SRAM1_RST_CTL register accessor: an alias for `Reg<SRAM1_RST_CTL_SPEC>`"]
pub type SRAM1_RST_CTL = crate::Reg<sram1_rst_ctl::SRAM1_RST_CTL_SPEC>;
#[doc = "SRAM block 1 module reset control register"]
pub mod sram1_rst_ctl;
#[doc = "ISP_SYS_RST_TIM register accessor: an alias for `Reg<ISP_SYS_RST_TIM_SPEC>`"]
pub type ISP_SYS_RST_TIM = crate::Reg<isp_sys_rst_tim::ISP_SYS_RST_TIM_SPEC>;
#[doc = "Video subsystem reset timing control register"]
pub mod isp_sys_rst_tim;
#[doc = "ISP_RST_TIM register accessor: an alias for `Reg<ISP_RST_TIM_SPEC>`"]
pub type ISP_RST_TIM = crate::Reg<isp_rst_tim::ISP_RST_TIM_SPEC>;
#[doc = "ISP module reset timing control register"]
pub mod isp_rst_tim;
#[doc = "ISP_F2K_RST_CTL register accessor: an alias for `Reg<ISP_F2K_RST_CTL_SPEC>`"]
pub type ISP_F2K_RST_CTL = crate::Reg<isp_f2k_rst_ctl::ISP_F2K_RST_CTL_SPEC>;
#[doc = "ISP_F2K module reset control register"]
pub mod isp_f2k_rst_ctl;
#[doc = "ISP_R2K_RST_CTL register accessor: an alias for `Reg<ISP_R2K_RST_CTL_SPEC>`"]
pub type ISP_R2K_RST_CTL = crate::Reg<isp_r2k_rst_ctl::ISP_R2K_RST_CTL_SPEC>;
#[doc = "ISP_R2K module reset control register"]
pub mod isp_r2k_rst_ctl;
#[doc = "ISP_TOF_RST_CTL register accessor: an alias for `Reg<ISP_TOF_RST_CTL_SPEC>`"]
pub type ISP_TOF_RST_CTL = crate::Reg<isp_tof_rst_ctl::ISP_TOF_RST_CTL_SPEC>;
#[doc = "ISP_TOF module reset control register"]
pub mod isp_tof_rst_ctl;
#[doc = "CSI_RST_TIM register accessor: an alias for `Reg<CSI_RST_TIM_SPEC>`"]
pub type CSI_RST_TIM = crate::Reg<csi_rst_tim::CSI_RST_TIM_SPEC>;
#[doc = "CSI module reset timing control register"]
pub mod csi_rst_tim;
#[doc = "CSI0_RST_CTL register accessor: an alias for `Reg<CSI0_RST_CTL_SPEC>`"]
pub type CSI0_RST_CTL = crate::Reg<csi0_rst_ctl::CSI0_RST_CTL_SPEC>;
#[doc = "CSI0 module reset control register"]
pub mod csi0_rst_ctl;
#[doc = "CSI1_RST_CTL register accessor: an alias for `Reg<CSI1_RST_CTL_SPEC>`"]
pub type CSI1_RST_CTL = crate::Reg<csi1_rst_ctl::CSI1_RST_CTL_SPEC>;
#[doc = "CSI1 module reset control register"]
pub mod csi1_rst_ctl;
#[doc = "SENSOR_RST_TIM register accessor: an alias for `Reg<SENSOR_RST_TIM_SPEC>`"]
pub type SENSOR_RST_TIM = crate::Reg<sensor_rst_tim::SENSOR_RST_TIM_SPEC>;
#[doc = "SENSOR reset timing control register"]
pub mod sensor_rst_tim;
#[doc = "SENSOR_RST_CTL register accessor: an alias for `Reg<SENSOR_RST_CTL_SPEC>`"]
pub type SENSOR_RST_CTL = crate::Reg<sensor_rst_ctl::SENSOR_RST_CTL_SPEC>;
#[doc = "SENSOR reset control register"]
pub mod sensor_rst_ctl;
#[doc = "VI_RST_TIM register accessor: an alias for `Reg<VI_RST_TIM_SPEC>`"]
pub type VI_RST_TIM = crate::Reg<vi_rst_tim::VI_RST_TIM_SPEC>;
#[doc = "VI module reset timing control register"]
pub mod vi_rst_tim;
#[doc = "VI_RST_CTL register accessor: an alias for `Reg<VI_RST_CTL_SPEC>`"]
pub type VI_RST_CTL = crate::Reg<vi_rst_ctl::VI_RST_CTL_SPEC>;
#[doc = "VI module reset control register"]
pub mod vi_rst_ctl;
#[doc = "MFBC_RST_TIM register accessor: an alias for `Reg<MFBC_RST_TIM_SPEC>`"]
pub type MFBC_RST_TIM = crate::Reg<mfbc_rst_tim::MFBC_RST_TIM_SPEC>;
#[doc = "MFBC module reset timing control register"]
pub mod mfbc_rst_tim;
#[doc = "MFBC_RST_CTL register accessor: an alias for `Reg<MFBC_RST_CTL_SPEC>`"]
pub type MFBC_RST_CTL = crate::Reg<mfbc_rst_ctl::MFBC_RST_CTL_SPEC>;
#[doc = "MFBC module reset control register"]
pub mod mfbc_rst_ctl;
#[doc = "DISP_SYS_RST_TIM register accessor: an alias for `Reg<DISP_SYS_RST_TIM_SPEC>`"]
pub type DISP_SYS_RST_TIM = crate::Reg<disp_sys_rst_tim::DISP_SYS_RST_TIM_SPEC>;
#[doc = "Display subsystem reset timing control register"]
pub mod disp_sys_rst_tim;
#[doc = "DSI_RST_TIM register accessor: an alias for `Reg<DSI_RST_TIM_SPEC>`"]
pub type DSI_RST_TIM = crate::Reg<dsi_rst_tim::DSI_RST_TIM_SPEC>;
#[doc = "DSI module reset timing control register"]
pub mod dsi_rst_tim;
#[doc = "DSI_RST_CTL register accessor: an alias for `Reg<DSI_RST_CTL_SPEC>`"]
pub type DSI_RST_CTL = crate::Reg<dsi_rst_ctl::DSI_RST_CTL_SPEC>;
#[doc = "DSI module reset control register"]
pub mod dsi_rst_ctl;
#[doc = "BT1120_RST_TIM register accessor: an alias for `Reg<BT1120_RST_TIM_SPEC>`"]
pub type BT1120_RST_TIM = crate::Reg<bt1120_rst_tim::BT1120_RST_TIM_SPEC>;
#[doc = "BT1120 module reset timing control register"]
pub mod bt1120_rst_tim;
#[doc = "BT1120_RST_CTL register accessor: an alias for `Reg<BT1120_RST_CTL_SPEC>`"]
pub type BT1120_RST_CTL = crate::Reg<bt1120_rst_ctl::BT1120_RST_CTL_SPEC>;
#[doc = "BT1120 module reset control register"]
pub mod bt1120_rst_ctl;
#[doc = "TWOD_RST_TIM register accessor: an alias for `Reg<TWOD_RST_TIM_SPEC>`"]
pub type TWOD_RST_TIM = crate::Reg<twod_rst_tim::TWOD_RST_TIM_SPEC>;
#[doc = "TWOD module reset timing control register"]
pub mod twod_rst_tim;
#[doc = "TWOD_RST_CTL register accessor: an alias for `Reg<TWOD_RST_CTL_SPEC>`"]
pub type TWOD_RST_CTL = crate::Reg<twod_rst_ctl::TWOD_RST_CTL_SPEC>;
#[doc = "TWOD module reset control register"]
pub mod twod_rst_ctl;
#[doc = "VO_RST_TIM register accessor: an alias for `Reg<VO_RST_TIM_SPEC>`"]
pub type VO_RST_TIM = crate::Reg<vo_rst_tim::VO_RST_TIM_SPEC>;
#[doc = "VO module reset timing control register"]
pub mod vo_rst_tim;
#[doc = "VO_RST_CTL register accessor: an alias for `Reg<VO_RST_CTL_SPEC>`"]
pub type VO_RST_CTL = crate::Reg<vo_rst_ctl::VO_RST_CTL_SPEC>;
#[doc = "VO module reset control register"]
pub mod vo_rst_ctl;
#[doc = "H264_RST_TIM register accessor: an alias for `Reg<H264_RST_TIM_SPEC>`"]
pub type H264_RST_TIM = crate::Reg<h264_rst_tim::H264_RST_TIM_SPEC>;
#[doc = "H264 module reset timing control register"]
pub mod h264_rst_tim;
#[doc = "H264_RST_CTL register accessor: an alias for `Reg<H264_RST_CTL_SPEC>`"]
pub type H264_RST_CTL = crate::Reg<h264_rst_ctl::H264_RST_CTL_SPEC>;
#[doc = "H264 module reset control register"]
pub mod h264_rst_ctl;
#[doc = "USB_RST_TIM register accessor: an alias for `Reg<USB_RST_TIM_SPEC>`"]
pub type USB_RST_TIM = crate::Reg<usb_rst_tim::USB_RST_TIM_SPEC>;
#[doc = "USB module reset timing control register"]
pub mod usb_rst_tim;
#[doc = "USB_RST_CTL register accessor: an alias for `Reg<USB_RST_CTL_SPEC>`"]
pub type USB_RST_CTL = crate::Reg<usb_rst_ctl::USB_RST_CTL_SPEC>;
#[doc = "USB module reset control register"]
pub mod usb_rst_ctl;
#[doc = "MIPI_CORNER_RST_TIM register accessor: an alias for `Reg<MIPI_CORNER_RST_TIM_SPEC>`"]
pub type MIPI_CORNER_RST_TIM = crate::Reg<mipi_corner_rst_tim::MIPI_CORNER_RST_TIM_SPEC>;
#[doc = "MIPI CORNER module reset timing control register"]
pub mod mipi_corner_rst_tim;
#[doc = "MIPI_CORNER_RST_CTL register accessor: an alias for `Reg<MIPI_CORNER_RST_CTL_SPEC>`"]
pub type MIPI_CORNER_RST_CTL = crate::Reg<mipi_corner_rst_ctl::MIPI_CORNER_RST_CTL_SPEC>;
#[doc = "MIPI CORNER module reset control register"]
pub mod mipi_corner_rst_ctl;
#[doc = "DDR_PHY_RST_CTL register accessor: an alias for `Reg<DDR_PHY_RST_CTL_SPEC>`"]
pub type DDR_PHY_RST_CTL = crate::Reg<ddr_phy_rst_ctl::DDR_PHY_RST_CTL_SPEC>;
#[doc = "DDR PHY reset control register"]
pub mod ddr_phy_rst_ctl;
#[doc = "AX25M_PWR_TIM register accessor: an alias for `Reg<AX25M_PWR_TIM_SPEC>`"]
pub type AX25M_PWR_TIM = crate::Reg<ax25m_pwr_tim::AX25M_PWR_TIM_SPEC>;
#[doc = "AX25M dual-core processor power controller timing register"]
pub mod ax25m_pwr_tim;
#[doc = "AX25M_LPI_TIM register accessor: an alias for `Reg<AX25M_LPI_TIM_SPEC>`"]
pub type AX25M_LPI_TIM = crate::Reg<ax25m_lpi_tim::AX25M_LPI_TIM_SPEC>;
#[doc = "AX25M domain NOC power controller LPI timing register"]
pub mod ax25m_lpi_tim;
#[doc = "AX25M_LPI_CTL register accessor: an alias for `Reg<AX25M_LPI_CTL_SPEC>`"]
pub type AX25M_LPI_CTL = crate::Reg<ax25m_lpi_ctl::AX25M_LPI_CTL_SPEC>;
#[doc = "AX25M NOC power controller low-power interface control register"]
pub mod ax25m_lpi_ctl;
#[doc = "AX25M_PWR_CTL register accessor: an alias for `Reg<AX25M_PWR_CTL_SPEC>`"]
pub type AX25M_PWR_CTL = crate::Reg<ax25m_pwr_ctl::AX25M_PWR_CTL_SPEC>;
#[doc = "AX25M power domain power status control register"]
pub mod ax25m_pwr_ctl;
#[doc = "AX25M_LPI_STAT register accessor: an alias for `Reg<AX25M_LPI_STAT_SPEC>`"]
pub type AX25M_LPI_STAT = crate::Reg<ax25m_lpi_stat::AX25M_LPI_STAT_SPEC>;
#[doc = "AX25M NOC power controller low-power interface status register"]
pub mod ax25m_lpi_stat;
#[doc = "AX25M_PWR_STAT register accessor: an alias for `Reg<AX25M_PWR_STAT_SPEC>`"]
pub type AX25M_PWR_STAT = crate::Reg<ax25m_pwr_stat::AX25M_PWR_STAT_SPEC>;
#[doc = "AX25M power domain current power status register"]
pub mod ax25m_pwr_stat;
#[doc = "AX25M_WFI_TIM register accessor: an alias for `Reg<AX25M_WFI_TIM_SPEC>`"]
pub type AX25M_WFI_TIM = crate::Reg<ax25m_wfi_tim::AX25M_WFI_TIM_SPEC>;
#[doc = "AX25M dual-core processor WFI signal waiting timeout configure register"]
pub mod ax25m_wfi_tim;
#[doc = "AX25P_PWR_TIM register accessor: an alias for `Reg<AX25P_PWR_TIM_SPEC>`"]
pub type AX25P_PWR_TIM = crate::Reg<ax25p_pwr_tim::AX25P_PWR_TIM_SPEC>;
#[doc = "AX25P single-core processor power controller timing register"]
pub mod ax25p_pwr_tim;
#[doc = "AX25P_LPI_TIM register accessor: an alias for `Reg<AX25P_LPI_TIM_SPEC>`"]
pub type AX25P_LPI_TIM = crate::Reg<ax25p_lpi_tim::AX25P_LPI_TIM_SPEC>;
#[doc = "AX25P domain NOC power controller LPI timing register"]
pub mod ax25p_lpi_tim;
#[doc = "AX25P_LPI_CTL register accessor: an alias for `Reg<AX25P_LPI_CTL_SPEC>`"]
pub type AX25P_LPI_CTL = crate::Reg<ax25p_lpi_ctl::AX25P_LPI_CTL_SPEC>;
#[doc = "AX25P NOC power controller low-power interface control register"]
pub mod ax25p_lpi_ctl;
#[doc = "AX25P_PWR_CTL register accessor: an alias for `Reg<AX25P_PWR_CTL_SPEC>`"]
pub type AX25P_PWR_CTL = crate::Reg<ax25p_pwr_ctl::AX25P_PWR_CTL_SPEC>;
#[doc = "AX25P power domain power state control register"]
pub mod ax25p_pwr_ctl;
#[doc = "AX25P_LPI_STAT register accessor: an alias for `Reg<AX25P_LPI_STAT_SPEC>`"]
pub type AX25P_LPI_STAT = crate::Reg<ax25p_lpi_stat::AX25P_LPI_STAT_SPEC>;
#[doc = "AX25P NOC power controller low-power interface status register"]
pub mod ax25p_lpi_stat;
#[doc = "AX25P_PWR_STAT register accessor: an alias for `Reg<AX25P_PWR_STAT_SPEC>`"]
pub type AX25P_PWR_STAT = crate::Reg<ax25p_pwr_stat::AX25P_PWR_STAT_SPEC>;
#[doc = "AX25P power domain current power status register"]
pub mod ax25p_pwr_stat;
#[doc = "AX25P_WFI_TIM register accessor: an alias for `Reg<AX25P_WFI_TIM_SPEC>`"]
pub type AX25P_WFI_TIM = crate::Reg<ax25p_wfi_tim::AX25P_WFI_TIM_SPEC>;
#[doc = "AX25P processor WFI signal waiting timeout configure register"]
pub mod ax25p_wfi_tim;
#[doc = "GNNE_PWR_TIM register accessor: an alias for `Reg<GNNE_PWR_TIM_SPEC>`"]
pub type GNNE_PWR_TIM = crate::Reg<gnne_pwr_tim::GNNE_PWR_TIM_SPEC>;
#[doc = "GNNE power controller timing register"]
pub mod gnne_pwr_tim;
#[doc = "GNNE_LPI_TIM register accessor: an alias for `Reg<GNNE_LPI_TIM_SPEC>`"]
pub type GNNE_LPI_TIM = crate::Reg<gnne_lpi_tim::GNNE_LPI_TIM_SPEC>;
#[doc = "GNNE power domain NOC power controller LPI timing register"]
pub mod gnne_lpi_tim;
#[doc = "GNNE_LPI_CTL register accessor: an alias for `Reg<GNNE_LPI_CTL_SPEC>`"]
pub type GNNE_LPI_CTL = crate::Reg<gnne_lpi_ctl::GNNE_LPI_CTL_SPEC>;
#[doc = "GNNE power domain NOC power controller low- power interface control register"]
pub mod gnne_lpi_ctl;
#[doc = "GNNE_PWR_CTL register accessor: an alias for `Reg<GNNE_PWR_CTL_SPEC>`"]
pub type GNNE_PWR_CTL = crate::Reg<gnne_pwr_ctl::GNNE_PWR_CTL_SPEC>;
#[doc = "GNNE power domain power state control register"]
pub mod gnne_pwr_ctl;
#[doc = "GNNE_LPI_STAT register accessor: an alias for `Reg<GNNE_LPI_STAT_SPEC>`"]
pub type GNNE_LPI_STAT = crate::Reg<gnne_lpi_stat::GNNE_LPI_STAT_SPEC>;
#[doc = "GNNE NOC power controller low-power interface status register"]
pub mod gnne_lpi_stat;
#[doc = "GNNE_PWR_STAT register accessor: an alias for `Reg<GNNE_PWR_STAT_SPEC>`"]
pub type GNNE_PWR_STAT = crate::Reg<gnne_pwr_stat::GNNE_PWR_STAT_SPEC>;
#[doc = "GNNE power domain current power status register"]
pub mod gnne_pwr_stat;
#[doc = "SEC_PWR_TIM register accessor: an alias for `Reg<SEC_PWR_TIM_SPEC>`"]
pub type SEC_PWR_TIM = crate::Reg<sec_pwr_tim::SEC_PWR_TIM_SPEC>;
#[doc = "Security subsystem domain power controller timing register"]
pub mod sec_pwr_tim;
#[doc = "SEC_LPI_TIM register accessor: an alias for `Reg<SEC_LPI_TIM_SPEC>`"]
pub type SEC_LPI_TIM = crate::Reg<sec_lpi_tim::SEC_LPI_TIM_SPEC>;
#[doc = "Security subsystem domain NOC power controller LPI timing register"]
pub mod sec_lpi_tim;
#[doc = "SEC_LPI_CTL register accessor: an alias for `Reg<SEC_LPI_CTL_SPEC>`"]
pub type SEC_LPI_CTL = crate::Reg<sec_lpi_ctl::SEC_LPI_CTL_SPEC>;
#[doc = "Security subsystem NOC power controller low- power interface control register"]
pub mod sec_lpi_ctl;
#[doc = "SEC_PWR_CTL register accessor: an alias for `Reg<SEC_PWR_CTL_SPEC>`"]
pub type SEC_PWR_CTL = crate::Reg<sec_pwr_ctl::SEC_PWR_CTL_SPEC>;
#[doc = "Security subsys power domain power state control register"]
pub mod sec_pwr_ctl;
#[doc = "SEC_LPI_STAT register accessor: an alias for `Reg<SEC_LPI_STAT_SPEC>`"]
pub type SEC_LPI_STAT = crate::Reg<sec_lpi_stat::SEC_LPI_STAT_SPEC>;
#[doc = "Security subsystem NOC power controller low- power interface status register"]
pub mod sec_lpi_stat;
#[doc = "SEC_PWR_STAT register accessor: an alias for `Reg<SEC_PWR_STAT_SPEC>`"]
pub type SEC_PWR_STAT = crate::Reg<sec_pwr_stat::SEC_PWR_STAT_SPEC>;
#[doc = "Security subsystem power domain current power status register"]
pub mod sec_pwr_stat;
#[doc = "STOR_PWR_TIM register accessor: an alias for `Reg<STOR_PWR_TIM_SPEC>`"]
pub type STOR_PWR_TIM = crate::Reg<stor_pwr_tim::STOR_PWR_TIM_SPEC>;
#[doc = "Storage subsystem domain power controller timing register"]
pub mod stor_pwr_tim;
#[doc = "STOR_LPI_TIM register accessor: an alias for `Reg<STOR_LPI_TIM_SPEC>`"]
pub type STOR_LPI_TIM = crate::Reg<stor_lpi_tim::STOR_LPI_TIM_SPEC>;
#[doc = "Storage subsystem domain NOC power controller LPI timing register"]
pub mod stor_lpi_tim;
#[doc = "STOR_LPI_CTL register accessor: an alias for `Reg<STOR_LPI_CTL_SPEC>`"]
pub type STOR_LPI_CTL = crate::Reg<stor_lpi_ctl::STOR_LPI_CTL_SPEC>;
#[doc = "Storage subsystem NOC power controller low- power interface control register"]
pub mod stor_lpi_ctl;
#[doc = "STOR_PWR_CTL register accessor: an alias for `Reg<STOR_PWR_CTL_SPEC>`"]
pub type STOR_PWR_CTL = crate::Reg<stor_pwr_ctl::STOR_PWR_CTL_SPEC>;
#[doc = "Storage subsystem power domain power state control register"]
pub mod stor_pwr_ctl;
#[doc = "STOR_LPI_STAT register accessor: an alias for `Reg<STOR_LPI_STAT_SPEC>`"]
pub type STOR_LPI_STAT = crate::Reg<stor_lpi_stat::STOR_LPI_STAT_SPEC>;
#[doc = "Storage subsystem NOC power controller low- power interface status register"]
pub mod stor_lpi_stat;
#[doc = "STOR_PWR_STAT register accessor: an alias for `Reg<STOR_PWR_STAT_SPEC>`"]
pub type STOR_PWR_STAT = crate::Reg<stor_pwr_stat::STOR_PWR_STAT_SPEC>;
#[doc = "Storage subsystem power domain current power status register"]
pub mod stor_pwr_stat;
#[doc = "PERI_PWR_TIM register accessor: an alias for `Reg<PERI_PWR_TIM_SPEC>`"]
pub type PERI_PWR_TIM = crate::Reg<peri_pwr_tim::PERI_PWR_TIM_SPEC>;
#[doc = "Peripheral subsystem domain power controller timing register"]
pub mod peri_pwr_tim;
#[doc = "PERI_LPI_TIM register accessor: an alias for `Reg<PERI_LPI_TIM_SPEC>`"]
pub type PERI_LPI_TIM = crate::Reg<peri_lpi_tim::PERI_LPI_TIM_SPEC>;
#[doc = "Peripheral subsystem domain NOC power controller LPI timing register"]
pub mod peri_lpi_tim;
#[doc = "PERI_LPI_CTL register accessor: an alias for `Reg<PERI_LPI_CTL_SPEC>`"]
pub type PERI_LPI_CTL = crate::Reg<peri_lpi_ctl::PERI_LPI_CTL_SPEC>;
#[doc = "Peripheral subsystem NOC power controller low- power interface control register"]
pub mod peri_lpi_ctl;
#[doc = "PERI_PWR_CTL register accessor: an alias for `Reg<PERI_PWR_CTL_SPEC>`"]
pub type PERI_PWR_CTL = crate::Reg<peri_pwr_ctl::PERI_PWR_CTL_SPEC>;
#[doc = "Peripheral subsystem power domain power state control register"]
pub mod peri_pwr_ctl;
#[doc = "PERI_LPI_STAT register accessor: an alias for `Reg<PERI_LPI_STAT_SPEC>`"]
pub type PERI_LPI_STAT = crate::Reg<peri_lpi_stat::PERI_LPI_STAT_SPEC>;
#[doc = "Peripheral subsystem NOC power controller low- power interface status register"]
pub mod peri_lpi_stat;
#[doc = "PERI_PWR_STAT register accessor: an alias for `Reg<PERI_PWR_STAT_SPEC>`"]
pub type PERI_PWR_STAT = crate::Reg<peri_pwr_stat::PERI_PWR_STAT_SPEC>;
#[doc = "Peripheral subsystem power domain current power status register"]
pub mod peri_pwr_stat;
#[doc = "MCTL_PWR_TIM0 register accessor: an alias for `Reg<MCTL_PWR_TIM0_SPEC>`"]
pub type MCTL_PWR_TIM0 = crate::Reg<mctl_pwr_tim0::MCTL_PWR_TIM0_SPEC>;
#[doc = "System Memory contorller power controller timing register"]
pub mod mctl_pwr_tim0;
#[doc = "MCTL_PWR_TIM1 register accessor: an alias for `Reg<MCTL_PWR_TIM1_SPEC>`"]
pub type MCTL_PWR_TIM1 = crate::Reg<mctl_pwr_tim1::MCTL_PWR_TIM1_SPEC>;
#[doc = "System Memory contorller power controller timing register"]
pub mod mctl_pwr_tim1;
#[doc = "MCTL_LPI_TIM register accessor: an alias for `Reg<MCTL_LPI_TIM_SPEC>`"]
pub type MCTL_LPI_TIM = crate::Reg<mctl_lpi_tim::MCTL_LPI_TIM_SPEC>;
#[doc = "Memory Controller domain NOC power controller LPI timing register"]
pub mod mctl_lpi_tim;
#[doc = "MCTL_LPI_CTL register accessor: an alias for `Reg<MCTL_LPI_CTL_SPEC>`"]
pub type MCTL_LPI_CTL = crate::Reg<mctl_lpi_ctl::MCTL_LPI_CTL_SPEC>;
#[doc = "Memory Controller domain NOC power controller low-power interface control register"]
pub mod mctl_lpi_ctl;
#[doc = "MCTL_PWR_CTL register accessor: an alias for `Reg<MCTL_PWR_CTL_SPEC>`"]
pub type MCTL_PWR_CTL = crate::Reg<mctl_pwr_ctl::MCTL_PWR_CTL_SPEC>;
#[doc = "Memory Controller power domain power status control register"]
pub mod mctl_pwr_ctl;
#[doc = "MCTL_LPI_STAT register accessor: an alias for `Reg<MCTL_LPI_STAT_SPEC>`"]
pub type MCTL_LPI_STAT = crate::Reg<mctl_lpi_stat::MCTL_LPI_STAT_SPEC>;
#[doc = "Memory Controller NOC power controller low- power interface status register"]
pub mod mctl_lpi_stat;
#[doc = "MCTL_PWR_STAT register accessor: an alias for `Reg<MCTL_PWR_STAT_SPEC>`"]
pub type MCTL_PWR_STAT = crate::Reg<mctl_pwr_stat::MCTL_PWR_STAT_SPEC>;
#[doc = "Memory Controller power domain current power status register"]
pub mod mctl_pwr_stat;
#[doc = "SRAM0_PWR_TIM register accessor: an alias for `Reg<SRAM0_PWR_TIM_SPEC>`"]
pub type SRAM0_PWR_TIM = crate::Reg<sram0_pwr_tim::SRAM0_PWR_TIM_SPEC>;
#[doc = "System SRAM block 0 power controller timing register"]
pub mod sram0_pwr_tim;
#[doc = "SRAM0_LPI_TIM register accessor: an alias for `Reg<SRAM0_LPI_TIM_SPEC>`"]
pub type SRAM0_LPI_TIM = crate::Reg<sram0_lpi_tim::SRAM0_LPI_TIM_SPEC>;
#[doc = "System SRAM block 0 domain NOC power controller LPI timing register"]
pub mod sram0_lpi_tim;
#[doc = "SRAM0_LPI_CTL register accessor: an alias for `Reg<SRAM0_LPI_CTL_SPEC>`"]
pub type SRAM0_LPI_CTL = crate::Reg<sram0_lpi_ctl::SRAM0_LPI_CTL_SPEC>;
#[doc = "System SRAM block 0 domain NOC power controller low-power interface control register"]
pub mod sram0_lpi_ctl;
#[doc = "SRAM0_PWR_CTL register accessor: an alias for `Reg<SRAM0_PWR_CTL_SPEC>`"]
pub type SRAM0_PWR_CTL = crate::Reg<sram0_pwr_ctl::SRAM0_PWR_CTL_SPEC>;
#[doc = "System SRAM block 0 power domain power status control register"]
pub mod sram0_pwr_ctl;
#[doc = "SRAM0_LPI_STAT register accessor: an alias for `Reg<SRAM0_LPI_STAT_SPEC>`"]
pub type SRAM0_LPI_STAT = crate::Reg<sram0_lpi_stat::SRAM0_LPI_STAT_SPEC>;
#[doc = "System SRAM block 0 NOC power controller low- power interface status register"]
pub mod sram0_lpi_stat;
#[doc = "SRAM0_PWR_STAT register accessor: an alias for `Reg<SRAM0_PWR_STAT_SPEC>`"]
pub type SRAM0_PWR_STAT = crate::Reg<sram0_pwr_stat::SRAM0_PWR_STAT_SPEC>;
#[doc = "System SRAM block 0 power domain current power status register"]
pub mod sram0_pwr_stat;
#[doc = "SRAM1_PWR_TIM register accessor: an alias for `Reg<SRAM1_PWR_TIM_SPEC>`"]
pub type SRAM1_PWR_TIM = crate::Reg<sram1_pwr_tim::SRAM1_PWR_TIM_SPEC>;
#[doc = "System SRAM block 1 power controller timing register"]
pub mod sram1_pwr_tim;
#[doc = "SRAM1_LPI_TIM register accessor: an alias for `Reg<SRAM1_LPI_TIM_SPEC>`"]
pub type SRAM1_LPI_TIM = crate::Reg<sram1_lpi_tim::SRAM1_LPI_TIM_SPEC>;
#[doc = "System SRAM block 1 domain NOC power controller LPI timing register"]
pub mod sram1_lpi_tim;
#[doc = "SRAM1_LPI_CTL register accessor: an alias for `Reg<SRAM1_LPI_CTL_SPEC>`"]
pub type SRAM1_LPI_CTL = crate::Reg<sram1_lpi_ctl::SRAM1_LPI_CTL_SPEC>;
#[doc = "System SRAM block 1 domain NOC power controller low-power interface control register"]
pub mod sram1_lpi_ctl;
#[doc = "SRAM1_PWR_CTL register accessor: an alias for `Reg<SRAM1_PWR_CTL_SPEC>`"]
pub type SRAM1_PWR_CTL = crate::Reg<sram1_pwr_ctl::SRAM1_PWR_CTL_SPEC>;
#[doc = "System SRAM block 1 power domain power status control register"]
pub mod sram1_pwr_ctl;
#[doc = "SRAM1_LPI_STAT register accessor: an alias for `Reg<SRAM1_LPI_STAT_SPEC>`"]
pub type SRAM1_LPI_STAT = crate::Reg<sram1_lpi_stat::SRAM1_LPI_STAT_SPEC>;
#[doc = "System SRAM block 1 NOC power controller low- power interface status register"]
pub mod sram1_lpi_stat;
#[doc = "SRAM1_PWR_STAT register accessor: an alias for `Reg<SRAM1_PWR_STAT_SPEC>`"]
pub type SRAM1_PWR_STAT = crate::Reg<sram1_pwr_stat::SRAM1_PWR_STAT_SPEC>;
#[doc = "System SRAM block 1 power domain current power status register"]
pub mod sram1_pwr_stat;
#[doc = "DISP_PWR_TIM register accessor: an alias for `Reg<DISP_PWR_TIM_SPEC>`"]
pub type DISP_PWR_TIM = crate::Reg<disp_pwr_tim::DISP_PWR_TIM_SPEC>;
#[doc = "Display System power controller timing register"]
pub mod disp_pwr_tim;
#[doc = "DISP_LPI_TIM register accessor: an alias for `Reg<DISP_LPI_TIM_SPEC>`"]
pub type DISP_LPI_TIM = crate::Reg<disp_lpi_tim::DISP_LPI_TIM_SPEC>;
#[doc = "System DISP domain NOC power controller LPI timing register"]
pub mod disp_lpi_tim;
#[doc = "DISP_LPI_CTL register accessor: an alias for `Reg<DISP_LPI_CTL_SPEC>`"]
pub type DISP_LPI_CTL = crate::Reg<disp_lpi_ctl::DISP_LPI_CTL_SPEC>;
#[doc = "System DISP domain NOC power controller low- power interface control register"]
pub mod disp_lpi_ctl;
#[doc = "DISP_PWR_CTL register accessor: an alias for `Reg<DISP_PWR_CTL_SPEC>`"]
pub type DISP_PWR_CTL = crate::Reg<disp_pwr_ctl::DISP_PWR_CTL_SPEC>;
#[doc = "System DISP power domain power status control register"]
pub mod disp_pwr_ctl;
#[doc = "DISP_LPI_STAT register accessor: an alias for `Reg<DISP_LPI_STAT_SPEC>`"]
pub type DISP_LPI_STAT = crate::Reg<disp_lpi_stat::DISP_LPI_STAT_SPEC>;
#[doc = "System DISP NOC power controller low-power interface status register"]
pub mod disp_lpi_stat;
#[doc = "DISP_PWR_STAT register accessor: an alias for `Reg<DISP_PWR_STAT_SPEC>`"]
pub type DISP_PWR_STAT = crate::Reg<disp_pwr_stat::DISP_PWR_STAT_SPEC>;
#[doc = "System DISP power domain current power status register"]
pub mod disp_pwr_stat;
#[doc = "H264_PWR_TIM register accessor: an alias for `Reg<H264_PWR_TIM_SPEC>`"]
pub type H264_PWR_TIM = crate::Reg<h264_pwr_tim::H264_PWR_TIM_SPEC>;
#[doc = "System H264 power controller timing register"]
pub mod h264_pwr_tim;
#[doc = "H264_LPI_TIM register accessor: an alias for `Reg<H264_LPI_TIM_SPEC>`"]
pub type H264_LPI_TIM = crate::Reg<h264_lpi_tim::H264_LPI_TIM_SPEC>;
#[doc = "System H264 domain NOC power controller LPI timing register"]
pub mod h264_lpi_tim;
#[doc = "H264_LPI_CTL register accessor: an alias for `Reg<H264_LPI_CTL_SPEC>`"]
pub type H264_LPI_CTL = crate::Reg<h264_lpi_ctl::H264_LPI_CTL_SPEC>;
#[doc = "System H264 domain NOC power controller low- power interface control register"]
pub mod h264_lpi_ctl;
#[doc = "H264_PWR_CTL register accessor: an alias for `Reg<H264_PWR_CTL_SPEC>`"]
pub type H264_PWR_CTL = crate::Reg<h264_pwr_ctl::H264_PWR_CTL_SPEC>;
#[doc = "System H264 power domain power status control register"]
pub mod h264_pwr_ctl;
#[doc = "H264_LPI_STAT register accessor: an alias for `Reg<H264_LPI_STAT_SPEC>`"]
pub type H264_LPI_STAT = crate::Reg<h264_lpi_stat::H264_LPI_STAT_SPEC>;
#[doc = "System H264 NOC power controller low-power interface status register"]
pub mod h264_lpi_stat;
#[doc = "H264_PWR_STAT register accessor: an alias for `Reg<H264_PWR_STAT_SPEC>`"]
pub type H264_PWR_STAT = crate::Reg<h264_pwr_stat::H264_PWR_STAT_SPEC>;
#[doc = "System H264 power domain current power status register"]
pub mod h264_pwr_stat;
#[doc = "USB_PWR_TIM register accessor: an alias for `Reg<USB_PWR_TIM_SPEC>`"]
pub type USB_PWR_TIM = crate::Reg<usb_pwr_tim::USB_PWR_TIM_SPEC>;
#[doc = "System USB power controller timing register"]
pub mod usb_pwr_tim;
#[doc = "USB_LPI_TIM register accessor: an alias for `Reg<USB_LPI_TIM_SPEC>`"]
pub type USB_LPI_TIM = crate::Reg<usb_lpi_tim::USB_LPI_TIM_SPEC>;
#[doc = "System USB domain NOC power controller LPI timing register"]
pub mod usb_lpi_tim;
#[doc = "USB_LPI_CTL register accessor: an alias for `Reg<USB_LPI_CTL_SPEC>`"]
pub type USB_LPI_CTL = crate::Reg<usb_lpi_ctl::USB_LPI_CTL_SPEC>;
#[doc = "System USB domain NOC power controller low- power interface control register"]
pub mod usb_lpi_ctl;
#[doc = "USB_PWR_CTL register accessor: an alias for `Reg<USB_PWR_CTL_SPEC>`"]
pub type USB_PWR_CTL = crate::Reg<usb_pwr_ctl::USB_PWR_CTL_SPEC>;
#[doc = "System USB power domain power status control register"]
pub mod usb_pwr_ctl;
#[doc = "USB_LPI_STAT register accessor: an alias for `Reg<USB_LPI_STAT_SPEC>`"]
pub type USB_LPI_STAT = crate::Reg<usb_lpi_stat::USB_LPI_STAT_SPEC>;
#[doc = "System USB NOC power controller low-power interface status register"]
pub mod usb_lpi_stat;
#[doc = "USB_PWR_STAT register accessor: an alias for `Reg<USB_PWR_STAT_SPEC>`"]
pub type USB_PWR_STAT = crate::Reg<usb_pwr_stat::USB_PWR_STAT_SPEC>;
#[doc = "System USB power domain current power status register"]
pub mod usb_pwr_stat;
#[doc = "ISP_PWR_TIM register accessor: an alias for `Reg<ISP_PWR_TIM_SPEC>`"]
pub type ISP_PWR_TIM = crate::Reg<isp_pwr_tim::ISP_PWR_TIM_SPEC>;
#[doc = "System ISP power controller timing register"]
pub mod isp_pwr_tim;
#[doc = "ISP_LPI_TIM register accessor: an alias for `Reg<ISP_LPI_TIM_SPEC>`"]
pub type ISP_LPI_TIM = crate::Reg<isp_lpi_tim::ISP_LPI_TIM_SPEC>;
#[doc = "System ISP domain NOC power controller LPI timing register"]
pub mod isp_lpi_tim;
#[doc = "ISP_LPI_CTL register accessor: an alias for `Reg<ISP_LPI_CTL_SPEC>`"]
pub type ISP_LPI_CTL = crate::Reg<isp_lpi_ctl::ISP_LPI_CTL_SPEC>;
#[doc = "System ISP domain NOC power controller low- power interface control register"]
pub mod isp_lpi_ctl;
#[doc = "ISP_PWR_CTL register accessor: an alias for `Reg<ISP_PWR_CTL_SPEC>`"]
pub type ISP_PWR_CTL = crate::Reg<isp_pwr_ctl::ISP_PWR_CTL_SPEC>;
#[doc = "System ISP power domain power status control register"]
pub mod isp_pwr_ctl;
#[doc = "ISP_LPI_STAT register accessor: an alias for `Reg<ISP_LPI_STAT_SPEC>`"]
pub type ISP_LPI_STAT = crate::Reg<isp_lpi_stat::ISP_LPI_STAT_SPEC>;
#[doc = "ISP NOC power controller low-power interface status register"]
pub mod isp_lpi_stat;
#[doc = "ISP_PWR_STAT register accessor: an alias for `Reg<ISP_PWR_STAT_SPEC>`"]
pub type ISP_PWR_STAT = crate::Reg<isp_pwr_stat::ISP_PWR_STAT_SPEC>;
#[doc = "ISP power domain current power status register"]
pub mod isp_pwr_stat;
#[doc = "PWR_DOM_GRP_EN register accessor: an alias for `Reg<PWR_DOM_GRP_EN_SPEC>`"]
pub type PWR_DOM_GRP_EN = crate::Reg<pwr_dom_grp_en::PWR_DOM_GRP_EN_SPEC>;
#[doc = "power domain group repair en"]
pub mod pwr_dom_grp_en;
#[doc = "REPAIR_STATUS register accessor: an alias for `Reg<REPAIR_STATUS_SPEC>`"]
pub type REPAIR_STATUS = crate::Reg<repair_status::REPAIR_STATUS_SPEC>;
#[doc = "power domain group repair status"]
pub mod repair_status;
#[doc = "AX25M_REPAIR_TIM register accessor: an alias for `Reg<AX25M_REPAIR_TIM_SPEC>`"]
pub type AX25M_REPAIR_TIM = crate::Reg<ax25m_repair_tim::AX25M_REPAIR_TIM_SPEC>;
#[doc = "AX25M power domain repair timer register"]
pub mod ax25m_repair_tim;
#[doc = "AX25P_REPAIR_TIM register accessor: an alias for `Reg<AX25P_REPAIR_TIM_SPEC>`"]
pub type AX25P_REPAIR_TIM = crate::Reg<ax25p_repair_tim::AX25P_REPAIR_TIM_SPEC>;
#[doc = "AX25P power domain repair timer register"]
pub mod ax25p_repair_tim;
#[doc = "GNNE_REPAIR_TIM register accessor: an alias for `Reg<GNNE_REPAIR_TIM_SPEC>`"]
pub type GNNE_REPAIR_TIM = crate::Reg<gnne_repair_tim::GNNE_REPAIR_TIM_SPEC>;
#[doc = "GNNE power domain repair timer register"]
pub mod gnne_repair_tim;
#[doc = "SRAM0_REPAIR_TIM register accessor: an alias for `Reg<SRAM0_REPAIR_TIM_SPEC>`"]
pub type SRAM0_REPAIR_TIM = crate::Reg<sram0_repair_tim::SRAM0_REPAIR_TIM_SPEC>;
#[doc = "SRAM0 power domain repair timer register"]
pub mod sram0_repair_tim;
#[doc = "SRAM1_REPAIR_TIM register accessor: an alias for `Reg<SRAM1_REPAIR_TIM_SPEC>`"]
pub type SRAM1_REPAIR_TIM = crate::Reg<sram1_repair_tim::SRAM1_REPAIR_TIM_SPEC>;
#[doc = "SRAM1 power domain repair timer register"]
pub mod sram1_repair_tim;
#[doc = "ISP_SYS_REPAIR_TIM register accessor: an alias for `Reg<ISP_SYS_REPAIR_TIM_SPEC>`"]
pub type ISP_SYS_REPAIR_TIM = crate::Reg<isp_sys_repair_tim::ISP_SYS_REPAIR_TIM_SPEC>;
#[doc = "ISP_SYS power domain repair timer register"]
pub mod isp_sys_repair_tim;
