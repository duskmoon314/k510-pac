# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- register description of MTIMER, PLIC, PLIC_SW (CPU)
- register address of WDT

## [0.0.2] - 2022-06-11

### Added

- register description of GPIO
- register name, address and description of sysctl
- register detailed description of sysctl
  - pll_cfg0
  - pll_cfg1
  - pll_ctl
  - pll_stat
  - soc_boot_ctrl
  - reset_status
  - osc_25m_off
  - soc_glb_rst
  - soc_reset_tim
  - soc_sleep_tim
  - soc_sleep_ctl
  - clk_stable_tim
  - cpu_wakup_tim
  - soc_wakup_src
  - cpu_wakup_cfg
  - timer_pause_ctl
  - uart_sclk_cfg
  - uart_rst_ctl
  - sys_ctl_int012_raw
  - sys_ctl_int012_en
  - sys_ctl_int012_stat
  - AX25M_HART_RSTVEC
  - AX25P_CORE_RSTVEC
  - SOC_SLEEP_MASK
  - TEST_PIN_SEL
  - AX25M_CLK_CFG
  - AX25M_MTIMER_CLK_CFG
  - AX25P_CLK_CFG
  - AX25P_MTIMER_CLK_CFG
  - GNNE_ACLK_CFG
  - GNNE_SYSCLK_CFG
  - I2C2AXI_CLK_CFG
  - NOC_CLK_CFG
  - PERI_SYS_BUS_CLK_CFG
  - PERI_SYS_BUS_CLK_EN

## [0.0.1] - 2022-05-20

### Added

- register description of uart
- generate code via svd2rust 0.24.0

[0.0.2]: https://github.com/duskmoon314/k510-pac/releases/tag/v0.0.2
[0.0.1]: https://github.com/duskmoon314/k510-pac/releases/tag/v0.0.1
