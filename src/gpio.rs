#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - data register"]
    pub gpio_swporta_dr: crate::Reg<gpio_swporta_dr::GPIO_SWPORTA_DR_SPEC>,
    #[doc = "0x04 - data direction"]
    pub gpio_swporta_ddr: crate::Reg<gpio_swporta_ddr::GPIO_SWPORTA_DDR_SPEC>,
    #[doc = "0x08 - data source control"]
    pub gpio_swporta_ctl: crate::Reg<gpio_swporta_ctl::GPIO_SWPORTA_CTL_SPEC>,
    _reserved3: [u8; 0x24],
    #[doc = "0x30 - interrupt enable"]
    pub gpio_inten: crate::Reg<gpio_inten::GPIO_INTEN_SPEC>,
    #[doc = "0x34 - interrupt mask"]
    pub gpio_intmask: crate::Reg<gpio_intmask::GPIO_INTMASK_SPEC>,
    #[doc = "0x38 - interrupt level"]
    pub gpio_inttype_level: crate::Reg<gpio_inttype_level::GPIO_INTTYPE_LEVEL_SPEC>,
    #[doc = "0x3c - interrupt polarity"]
    pub gpio_int_polarity: crate::Reg<gpio_int_polarity::GPIO_INT_POLARITY_SPEC>,
    #[doc = "0x40 - interrupt status"]
    pub gpio_intstatus: crate::Reg<gpio_intstatus::GPIO_INTSTATUS_SPEC>,
    #[doc = "0x44 - raw interrupt status"]
    pub gpio_raw_intstatus: crate::Reg<gpio_raw_intstatus::GPIO_RAW_INTSTATUS_SPEC>,
    #[doc = "0x48 - debounce enable"]
    pub gpio_debounce: crate::Reg<gpio_debounce::GPIO_DEBOUNCE_SPEC>,
    #[doc = "0x4c - end of interrupt"]
    pub gpio_porta_eoi: crate::Reg<gpio_porta_eoi::GPIO_PORTA_EOI_SPEC>,
    #[doc = "0x50 - external port"]
    pub gpio_ext_porta: crate::Reg<gpio_ext_porta::GPIO_EXT_PORTA_SPEC>,
    _reserved12: [u8; 0x0c],
    #[doc = "0x60 - synchronization level"]
    pub gpio_ls_sync: crate::Reg<gpio_ls_sync::GPIO_LS_SYNC_SPEC>,
    #[doc = "0x64 - ID info"]
    pub gpio_id_code: crate::Reg<gpio_id_code::GPIO_ID_CODE_SPEC>,
    #[doc = "0x68 - interrupt edge type"]
    pub gpio_int_bothedge: crate::Reg<gpio_int_bothedge::GPIO_INT_BOTHEDGE_SPEC>,
    #[doc = "0x6c - version info"]
    pub gpio_ver_id_code: crate::Reg<gpio_ver_id_code::GPIO_VER_ID_CODE_SPEC>,
    #[doc = "0x70 - parameters info"]
    pub gpio_config_reg2: crate::Reg<gpio_config_reg2::GPIO_CONFIG_REG2_SPEC>,
    #[doc = "0x74 - parameters info"]
    pub gpio_config_reg1: crate::Reg<gpio_config_reg1::GPIO_CONFIG_REG1_SPEC>,
}
#[doc = "GPIO_SWPORTA_DR register accessor: an alias for `Reg<GPIO_SWPORTA_DR_SPEC>`"]
pub type GPIO_SWPORTA_DR = crate::Reg<gpio_swporta_dr::GPIO_SWPORTA_DR_SPEC>;
#[doc = "data register"]
pub mod gpio_swporta_dr;
#[doc = "GPIO_SWPORTA_DDR register accessor: an alias for `Reg<GPIO_SWPORTA_DDR_SPEC>`"]
pub type GPIO_SWPORTA_DDR = crate::Reg<gpio_swporta_ddr::GPIO_SWPORTA_DDR_SPEC>;
#[doc = "data direction"]
pub mod gpio_swporta_ddr;
#[doc = "GPIO_SWPORTA_CTL register accessor: an alias for `Reg<GPIO_SWPORTA_CTL_SPEC>`"]
pub type GPIO_SWPORTA_CTL = crate::Reg<gpio_swporta_ctl::GPIO_SWPORTA_CTL_SPEC>;
#[doc = "data source control"]
pub mod gpio_swporta_ctl;
#[doc = "GPIO_INTEN register accessor: an alias for `Reg<GPIO_INTEN_SPEC>`"]
pub type GPIO_INTEN = crate::Reg<gpio_inten::GPIO_INTEN_SPEC>;
#[doc = "interrupt enable"]
pub mod gpio_inten;
#[doc = "GPIO_INTMASK register accessor: an alias for `Reg<GPIO_INTMASK_SPEC>`"]
pub type GPIO_INTMASK = crate::Reg<gpio_intmask::GPIO_INTMASK_SPEC>;
#[doc = "interrupt mask"]
pub mod gpio_intmask;
#[doc = "GPIO_INTTYPE_LEVEL register accessor: an alias for `Reg<GPIO_INTTYPE_LEVEL_SPEC>`"]
pub type GPIO_INTTYPE_LEVEL = crate::Reg<gpio_inttype_level::GPIO_INTTYPE_LEVEL_SPEC>;
#[doc = "interrupt level"]
pub mod gpio_inttype_level;
#[doc = "GPIO_INT_POLARITY register accessor: an alias for `Reg<GPIO_INT_POLARITY_SPEC>`"]
pub type GPIO_INT_POLARITY = crate::Reg<gpio_int_polarity::GPIO_INT_POLARITY_SPEC>;
#[doc = "interrupt polarity"]
pub mod gpio_int_polarity;
#[doc = "GPIO_INTSTATUS register accessor: an alias for `Reg<GPIO_INTSTATUS_SPEC>`"]
pub type GPIO_INTSTATUS = crate::Reg<gpio_intstatus::GPIO_INTSTATUS_SPEC>;
#[doc = "interrupt status"]
pub mod gpio_intstatus;
#[doc = "GPIO_RAW_INTSTATUS register accessor: an alias for `Reg<GPIO_RAW_INTSTATUS_SPEC>`"]
pub type GPIO_RAW_INTSTATUS = crate::Reg<gpio_raw_intstatus::GPIO_RAW_INTSTATUS_SPEC>;
#[doc = "raw interrupt status"]
pub mod gpio_raw_intstatus;
#[doc = "GPIO_DEBOUNCE register accessor: an alias for `Reg<GPIO_DEBOUNCE_SPEC>`"]
pub type GPIO_DEBOUNCE = crate::Reg<gpio_debounce::GPIO_DEBOUNCE_SPEC>;
#[doc = "debounce enable"]
pub mod gpio_debounce;
#[doc = "GPIO_PORTA_EOI register accessor: an alias for `Reg<GPIO_PORTA_EOI_SPEC>`"]
pub type GPIO_PORTA_EOI = crate::Reg<gpio_porta_eoi::GPIO_PORTA_EOI_SPEC>;
#[doc = "end of interrupt"]
pub mod gpio_porta_eoi;
#[doc = "GPIO_EXT_PORTA register accessor: an alias for `Reg<GPIO_EXT_PORTA_SPEC>`"]
pub type GPIO_EXT_PORTA = crate::Reg<gpio_ext_porta::GPIO_EXT_PORTA_SPEC>;
#[doc = "external port"]
pub mod gpio_ext_porta;
#[doc = "GPIO_LS_SYNC register accessor: an alias for `Reg<GPIO_LS_SYNC_SPEC>`"]
pub type GPIO_LS_SYNC = crate::Reg<gpio_ls_sync::GPIO_LS_SYNC_SPEC>;
#[doc = "synchronization level"]
pub mod gpio_ls_sync;
#[doc = "GPIO_ID_CODE register accessor: an alias for `Reg<GPIO_ID_CODE_SPEC>`"]
pub type GPIO_ID_CODE = crate::Reg<gpio_id_code::GPIO_ID_CODE_SPEC>;
#[doc = "ID info"]
pub mod gpio_id_code;
#[doc = "GPIO_INT_BOTHEDGE register accessor: an alias for `Reg<GPIO_INT_BOTHEDGE_SPEC>`"]
pub type GPIO_INT_BOTHEDGE = crate::Reg<gpio_int_bothedge::GPIO_INT_BOTHEDGE_SPEC>;
#[doc = "interrupt edge type"]
pub mod gpio_int_bothedge;
#[doc = "GPIO_VER_ID_CODE register accessor: an alias for `Reg<GPIO_VER_ID_CODE_SPEC>`"]
pub type GPIO_VER_ID_CODE = crate::Reg<gpio_ver_id_code::GPIO_VER_ID_CODE_SPEC>;
#[doc = "version info"]
pub mod gpio_ver_id_code;
#[doc = "GPIO_CONFIG_REG2 register accessor: an alias for `Reg<GPIO_CONFIG_REG2_SPEC>`"]
pub type GPIO_CONFIG_REG2 = crate::Reg<gpio_config_reg2::GPIO_CONFIG_REG2_SPEC>;
#[doc = "parameters info"]
pub mod gpio_config_reg2;
#[doc = "GPIO_CONFIG_REG1 register accessor: an alias for `Reg<GPIO_CONFIG_REG1_SPEC>`"]
pub type GPIO_CONFIG_REG1 = crate::Reg<gpio_config_reg1::GPIO_CONFIG_REG1_SPEC>;
#[doc = "parameters info"]
pub mod gpio_config_reg1;
