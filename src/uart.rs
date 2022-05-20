#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_uart: [u8; 0x04],
    _reserved_1_uart: [u8; 0x04],
    _reserved_2_uart: [u8; 0x04],
    #[doc = "0x0c - Line Control Register"]
    pub uart_lcr: crate::Reg<uart_lcr::UART_LCR_SPEC>,
    #[doc = "0x10 - Modem Control Register"]
    pub uart_mcr: crate::Reg<uart_mcr::UART_MCR_SPEC>,
    #[doc = "0x14 - Line Status Register"]
    pub uart_lsr: crate::Reg<uart_lsr::UART_LSR_SPEC>,
    #[doc = "0x18 - Modem Status Register"]
    pub uart_msr: crate::Reg<uart_msr::UART_MSR_SPEC>,
    #[doc = "0x1c - Scratch Pad Register"]
    pub uart_scr: crate::Reg<uart_scr::UART_SCR_SPEC>,
    #[doc = "0x20 - Low Power Divisor Latch Low Register"]
    pub uart_lpdll: crate::Reg<uart_lpdll::UART_LPDLL_SPEC>,
    #[doc = "0x24 - Low Power Divisor Latch High Register"]
    pub uart_lpdlh: crate::Reg<uart_lpdlh::UART_LPDLH_SPEC>,
    _reserved10: [u8; 0x08],
    _reserved_10_uart: [u8; 0x40],
    #[doc = "0x70 - FIFO Access Register"]
    pub uart_far: crate::Reg<uart_far::UART_FAR_SPEC>,
    #[doc = "0x74 - Transmit FIFO Read"]
    pub uart_tfr: crate::Reg<uart_tfr::UART_TFR_SPEC>,
    #[doc = "0x78 - Receive FIFO control"]
    pub uart_rfw: crate::Reg<uart_rfw::UART_RFW_SPEC>,
    #[doc = "0x7c - UART Status Register"]
    pub uart_usr: crate::Reg<uart_usr::UART_USR_SPEC>,
    #[doc = "0x80 - Transmit FIFO Level"]
    pub uart_tfl: crate::Reg<uart_tfl::UART_TFL_SPEC>,
    #[doc = "0x84 - Receive FIFO Level"]
    pub uart_rfl: crate::Reg<uart_rfl::UART_RFL_SPEC>,
    #[doc = "0x88 - "]
    pub uart_srr: crate::Reg<uart_srr::UART_SRR_SPEC>,
    #[doc = "0x8c - "]
    pub uart_srts: crate::Reg<uart_srts::UART_SRTS_SPEC>,
    #[doc = "0x90 - "]
    pub uart_sbcr: crate::Reg<uart_sbcr::UART_SBCR_SPEC>,
    #[doc = "0x94 - Shadow DMA Mode"]
    pub uart_sdmam: crate::Reg<uart_sdmam::UART_SDMAM_SPEC>,
    #[doc = "0x98 - Shadow FIFO Enable"]
    pub uart_sfe: crate::Reg<uart_sfe::UART_SFE_SPEC>,
    #[doc = "0x9c - Shadow Receive Trigger"]
    pub uart_srt: crate::Reg<uart_srt::UART_SRT_SPEC>,
    #[doc = "0xa0 - Shadow Transmit Empty Trigger"]
    pub uart_stet: crate::Reg<uart_stet::UART_STET_SPEC>,
    #[doc = "0xa4 - Halt Tx"]
    pub uart_htx: crate::Reg<uart_htx::UART_HTX_SPEC>,
    #[doc = "0xa8 - DMA Software Acknowledge Register"]
    pub uart_dmasa: crate::Reg<uart_dmasa::UART_DMASA_SPEC>,
    #[doc = "0xac - enable or disable RS485 mode"]
    pub uart_tcr: crate::Reg<uart_tcr::UART_TCR_SPEC>,
    #[doc = "0xb0 - The Driver Output Enable Register"]
    pub uart_de_en: crate::Reg<uart_de_en::UART_DE_EN_SPEC>,
    #[doc = "0xb4 - The Receiver Output Enable Register"]
    pub uart_re_en: crate::Reg<uart_re_en::UART_RE_EN_SPEC>,
    #[doc = "0xb8 - The Driver Output Enable Timing Register"]
    pub uart_det: crate::Reg<uart_det::UART_DET_SPEC>,
    #[doc = "0xbc - The Turn Around Timing Register"]
    pub uart_tat: crate::Reg<uart_tat::UART_TAT_SPEC>,
    #[doc = "0xc0 - Fractional Baud rate Divisor"]
    pub uart_dlf: crate::Reg<uart_dlf::UART_DLF_SPEC>,
    #[doc = "0xc4 - Receive Address Register"]
    pub uart_rar: crate::Reg<uart_rar::UART_RAR_SPEC>,
    #[doc = "0xc8 - Transmit Address Register"]
    pub uart_tar: crate::Reg<uart_tar::UART_TAR_SPEC>,
    #[doc = "0xcc - Line Control Register Extended"]
    pub uart_lcr_ext: crate::Reg<uart_lcr_ext::UART_LCR_EXT_SPEC>,
    _reserved35: [u8; 0x24],
    #[doc = "0xf4 - Component Parameter Register"]
    pub uart_cpr: crate::Reg<uart_cpr::UART_CPR_SPEC>,
    #[doc = "0xf8 - "]
    pub uart_ucv: crate::Reg<uart_ucv::UART_UCV_SPEC>,
    #[doc = "0xfc - "]
    pub uart_ctr: crate::Reg<uart_ctr::UART_CTR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - Transmit Holding Register"]
    #[inline(always)]
    pub fn uart_thr(&self) -> &crate::Reg<uart_thr::UART_THR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<uart_thr::UART_THR_SPEC>)
        }
    }
    #[doc = "0x00 - Divisor Latch (Low) Register"]
    #[inline(always)]
    pub fn uart_dll(&self) -> &crate::Reg<uart_dll::UART_DLL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<uart_dll::UART_DLL_SPEC>)
        }
    }
    #[doc = "0x00 - Receive Buffer Register"]
    #[inline(always)]
    pub fn uart_rbr(&self) -> &crate::Reg<uart_rbr::UART_RBR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<uart_rbr::UART_RBR_SPEC>)
        }
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn uart_ier(&self) -> &crate::Reg<uart_ier::UART_IER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<uart_ier::UART_IER_SPEC>)
        }
    }
    #[doc = "0x04 - Divisor Latch (High) Register"]
    #[inline(always)]
    pub fn uart_dlh(&self) -> &crate::Reg<uart_dlh::UART_DLH_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<uart_dlh::UART_DLH_SPEC>)
        }
    }
    #[doc = "0x08 - Interrupt Identification Register"]
    #[inline(always)]
    pub fn uart_iir(&self) -> &crate::Reg<uart_iir::UART_IIR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<uart_iir::UART_IIR_SPEC>)
        }
    }
    #[doc = "0x08 - FIFO Control Register"]
    #[inline(always)]
    pub fn uart_fcr(&self) -> &crate::Reg<uart_fcr::UART_FCR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<uart_fcr::UART_FCR_SPEC>)
        }
    }
    #[doc = "0x30..0x70 - Shadow Transmit Holding Register"]
    #[inline(always)]
    pub fn uart_sthr(&self) -> &[crate::Reg<uart_sthr::UART_STHR_SPEC>; 16] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const [crate::Reg<uart_sthr::UART_STHR_SPEC>; 16])
        }
    }
    #[doc = "0x30..0x70 - Shadow Receive Buffer Register"]
    #[inline(always)]
    pub fn uart_srbr(&self) -> &[crate::Reg<uart_srbr::UART_SRBR_SPEC>; 16] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const [crate::Reg<uart_srbr::UART_SRBR_SPEC>; 16])
        }
    }
}
#[doc = "UART_RBR register accessor: an alias for `Reg<UART_RBR_SPEC>`"]
pub type UART_RBR = crate::Reg<uart_rbr::UART_RBR_SPEC>;
#[doc = "Receive Buffer Register"]
pub mod uart_rbr;
#[doc = "UART_DLL register accessor: an alias for `Reg<UART_DLL_SPEC>`"]
pub type UART_DLL = crate::Reg<uart_dll::UART_DLL_SPEC>;
#[doc = "Divisor Latch (Low) Register"]
pub mod uart_dll;
#[doc = "UART_THR register accessor: an alias for `Reg<UART_THR_SPEC>`"]
pub type UART_THR = crate::Reg<uart_thr::UART_THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod uart_thr;
#[doc = "UART_DLH register accessor: an alias for `Reg<UART_DLH_SPEC>`"]
pub type UART_DLH = crate::Reg<uart_dlh::UART_DLH_SPEC>;
#[doc = "Divisor Latch (High) Register"]
pub mod uart_dlh;
#[doc = "UART_IER register accessor: an alias for `Reg<UART_IER_SPEC>`"]
pub type UART_IER = crate::Reg<uart_ier::UART_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod uart_ier;
#[doc = "UART_FCR register accessor: an alias for `Reg<UART_FCR_SPEC>`"]
pub type UART_FCR = crate::Reg<uart_fcr::UART_FCR_SPEC>;
#[doc = "FIFO Control Register"]
pub mod uart_fcr;
#[doc = "UART_IIR register accessor: an alias for `Reg<UART_IIR_SPEC>`"]
pub type UART_IIR = crate::Reg<uart_iir::UART_IIR_SPEC>;
#[doc = "Interrupt Identification Register"]
pub mod uart_iir;
#[doc = "UART_LCR register accessor: an alias for `Reg<UART_LCR_SPEC>`"]
pub type UART_LCR = crate::Reg<uart_lcr::UART_LCR_SPEC>;
#[doc = "Line Control Register"]
pub mod uart_lcr;
#[doc = "UART_MCR register accessor: an alias for `Reg<UART_MCR_SPEC>`"]
pub type UART_MCR = crate::Reg<uart_mcr::UART_MCR_SPEC>;
#[doc = "Modem Control Register"]
pub mod uart_mcr;
#[doc = "UART_LSR register accessor: an alias for `Reg<UART_LSR_SPEC>`"]
pub type UART_LSR = crate::Reg<uart_lsr::UART_LSR_SPEC>;
#[doc = "Line Status Register"]
pub mod uart_lsr;
#[doc = "UART_MSR register accessor: an alias for `Reg<UART_MSR_SPEC>`"]
pub type UART_MSR = crate::Reg<uart_msr::UART_MSR_SPEC>;
#[doc = "Modem Status Register"]
pub mod uart_msr;
#[doc = "UART_SCR register accessor: an alias for `Reg<UART_SCR_SPEC>`"]
pub type UART_SCR = crate::Reg<uart_scr::UART_SCR_SPEC>;
#[doc = "Scratch Pad Register"]
pub mod uart_scr;
#[doc = "UART_LPDLL register accessor: an alias for `Reg<UART_LPDLL_SPEC>`"]
pub type UART_LPDLL = crate::Reg<uart_lpdll::UART_LPDLL_SPEC>;
#[doc = "Low Power Divisor Latch Low Register"]
pub mod uart_lpdll;
#[doc = "UART_LPDLH register accessor: an alias for `Reg<UART_LPDLH_SPEC>`"]
pub type UART_LPDLH = crate::Reg<uart_lpdlh::UART_LPDLH_SPEC>;
#[doc = "Low Power Divisor Latch High Register"]
pub mod uart_lpdlh;
#[doc = "UART_SRBR register accessor: an alias for `Reg<UART_SRBR_SPEC>`"]
pub type UART_SRBR = crate::Reg<uart_srbr::UART_SRBR_SPEC>;
#[doc = "Shadow Receive Buffer Register"]
pub mod uart_srbr;
#[doc = "UART_STHR register accessor: an alias for `Reg<UART_STHR_SPEC>`"]
pub type UART_STHR = crate::Reg<uart_sthr::UART_STHR_SPEC>;
#[doc = "Shadow Transmit Holding Register"]
pub mod uart_sthr;
#[doc = "UART_FAR register accessor: an alias for `Reg<UART_FAR_SPEC>`"]
pub type UART_FAR = crate::Reg<uart_far::UART_FAR_SPEC>;
#[doc = "FIFO Access Register"]
pub mod uart_far;
#[doc = "UART_TFR register accessor: an alias for `Reg<UART_TFR_SPEC>`"]
pub type UART_TFR = crate::Reg<uart_tfr::UART_TFR_SPEC>;
#[doc = "Transmit FIFO Read"]
pub mod uart_tfr;
#[doc = "UART_RFW register accessor: an alias for `Reg<UART_RFW_SPEC>`"]
pub type UART_RFW = crate::Reg<uart_rfw::UART_RFW_SPEC>;
#[doc = "Receive FIFO control"]
pub mod uart_rfw;
#[doc = "UART_USR register accessor: an alias for `Reg<UART_USR_SPEC>`"]
pub type UART_USR = crate::Reg<uart_usr::UART_USR_SPEC>;
#[doc = "UART Status Register"]
pub mod uart_usr;
#[doc = "UART_TFL register accessor: an alias for `Reg<UART_TFL_SPEC>`"]
pub type UART_TFL = crate::Reg<uart_tfl::UART_TFL_SPEC>;
#[doc = "Transmit FIFO Level"]
pub mod uart_tfl;
#[doc = "UART_RFL register accessor: an alias for `Reg<UART_RFL_SPEC>`"]
pub type UART_RFL = crate::Reg<uart_rfl::UART_RFL_SPEC>;
#[doc = "Receive FIFO Level"]
pub mod uart_rfl;
#[doc = "UART_SRR register accessor: an alias for `Reg<UART_SRR_SPEC>`"]
pub type UART_SRR = crate::Reg<uart_srr::UART_SRR_SPEC>;
#[doc = ""]
pub mod uart_srr;
#[doc = "UART_SRTS register accessor: an alias for `Reg<UART_SRTS_SPEC>`"]
pub type UART_SRTS = crate::Reg<uart_srts::UART_SRTS_SPEC>;
#[doc = ""]
pub mod uart_srts;
#[doc = "UART_SBCR register accessor: an alias for `Reg<UART_SBCR_SPEC>`"]
pub type UART_SBCR = crate::Reg<uart_sbcr::UART_SBCR_SPEC>;
#[doc = ""]
pub mod uart_sbcr;
#[doc = "UART_SDMAM register accessor: an alias for `Reg<UART_SDMAM_SPEC>`"]
pub type UART_SDMAM = crate::Reg<uart_sdmam::UART_SDMAM_SPEC>;
#[doc = "Shadow DMA Mode"]
pub mod uart_sdmam;
#[doc = "UART_SFE register accessor: an alias for `Reg<UART_SFE_SPEC>`"]
pub type UART_SFE = crate::Reg<uart_sfe::UART_SFE_SPEC>;
#[doc = "Shadow FIFO Enable"]
pub mod uart_sfe;
#[doc = "UART_SRT register accessor: an alias for `Reg<UART_SRT_SPEC>`"]
pub type UART_SRT = crate::Reg<uart_srt::UART_SRT_SPEC>;
#[doc = "Shadow Receive Trigger"]
pub mod uart_srt;
#[doc = "UART_STET register accessor: an alias for `Reg<UART_STET_SPEC>`"]
pub type UART_STET = crate::Reg<uart_stet::UART_STET_SPEC>;
#[doc = "Shadow Transmit Empty Trigger"]
pub mod uart_stet;
#[doc = "UART_HTX register accessor: an alias for `Reg<UART_HTX_SPEC>`"]
pub type UART_HTX = crate::Reg<uart_htx::UART_HTX_SPEC>;
#[doc = "Halt Tx"]
pub mod uart_htx;
#[doc = "UART_DMASA register accessor: an alias for `Reg<UART_DMASA_SPEC>`"]
pub type UART_DMASA = crate::Reg<uart_dmasa::UART_DMASA_SPEC>;
#[doc = "DMA Software Acknowledge Register"]
pub mod uart_dmasa;
#[doc = "UART_TCR register accessor: an alias for `Reg<UART_TCR_SPEC>`"]
pub type UART_TCR = crate::Reg<uart_tcr::UART_TCR_SPEC>;
#[doc = "enable or disable RS485 mode"]
pub mod uart_tcr;
#[doc = "UART_DE_EN register accessor: an alias for `Reg<UART_DE_EN_SPEC>`"]
pub type UART_DE_EN = crate::Reg<uart_de_en::UART_DE_EN_SPEC>;
#[doc = "The Driver Output Enable Register"]
pub mod uart_de_en;
#[doc = "UART_RE_EN register accessor: an alias for `Reg<UART_RE_EN_SPEC>`"]
pub type UART_RE_EN = crate::Reg<uart_re_en::UART_RE_EN_SPEC>;
#[doc = "The Receiver Output Enable Register"]
pub mod uart_re_en;
#[doc = "UART_DET register accessor: an alias for `Reg<UART_DET_SPEC>`"]
pub type UART_DET = crate::Reg<uart_det::UART_DET_SPEC>;
#[doc = "The Driver Output Enable Timing Register"]
pub mod uart_det;
#[doc = "UART_TAT register accessor: an alias for `Reg<UART_TAT_SPEC>`"]
pub type UART_TAT = crate::Reg<uart_tat::UART_TAT_SPEC>;
#[doc = "The Turn Around Timing Register"]
pub mod uart_tat;
#[doc = "UART_DLF register accessor: an alias for `Reg<UART_DLF_SPEC>`"]
pub type UART_DLF = crate::Reg<uart_dlf::UART_DLF_SPEC>;
#[doc = "Fractional Baud rate Divisor"]
pub mod uart_dlf;
#[doc = "UART_RAR register accessor: an alias for `Reg<UART_RAR_SPEC>`"]
pub type UART_RAR = crate::Reg<uart_rar::UART_RAR_SPEC>;
#[doc = "Receive Address Register"]
pub mod uart_rar;
#[doc = "UART_TAR register accessor: an alias for `Reg<UART_TAR_SPEC>`"]
pub type UART_TAR = crate::Reg<uart_tar::UART_TAR_SPEC>;
#[doc = "Transmit Address Register"]
pub mod uart_tar;
#[doc = "UART_LCR_EXT register accessor: an alias for `Reg<UART_LCR_EXT_SPEC>`"]
pub type UART_LCR_EXT = crate::Reg<uart_lcr_ext::UART_LCR_EXT_SPEC>;
#[doc = "Line Control Register Extended"]
pub mod uart_lcr_ext;
#[doc = "UART_CPR register accessor: an alias for `Reg<UART_CPR_SPEC>`"]
pub type UART_CPR = crate::Reg<uart_cpr::UART_CPR_SPEC>;
#[doc = "Component Parameter Register"]
pub mod uart_cpr;
#[doc = "UART_UCV register accessor: an alias for `Reg<UART_UCV_SPEC>`"]
pub type UART_UCV = crate::Reg<uart_ucv::UART_UCV_SPEC>;
#[doc = ""]
pub mod uart_ucv;
#[doc = "UART_CTR register accessor: an alias for `Reg<UART_CTR_SPEC>`"]
pub type UART_CTR = crate::Reg<uart_ctr::UART_CTR_SPEC>;
#[doc = ""]
pub mod uart_ctr;
