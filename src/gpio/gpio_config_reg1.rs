#[doc = "Register `GPIO_CONFIG_REG1` reader"]
pub struct R(crate::R<GPIO_CONFIG_REG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CONFIG_REG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CONFIG_REG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CONFIG_REG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "The value of this register is derived from the GPIO_INT_BOTHEDGE configuration parameter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERRUPT_BOTH_EDGE_TYPE_A {
    #[doc = "0: Interrupt generation on rising or falling edge"]
    SINGLE_EDGE = 0,
    #[doc = "1: Interrupt generation on both rising and falling edge"]
    BOTH_EDGE = 1,
}
impl From<INTERRUPT_BOTH_EDGE_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_BOTH_EDGE_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTERRUPT_BOTH_EDGE_TYPE` reader - The value of this register is derived from the GPIO_INT_BOTHEDGE configuration parameter"]
pub type INTERRUPT_BOTH_EDGE_TYPE_R = crate::BitReader<INTERRUPT_BOTH_EDGE_TYPE_A>;
impl INTERRUPT_BOTH_EDGE_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERRUPT_BOTH_EDGE_TYPE_A {
        match self.bits {
            false => INTERRUPT_BOTH_EDGE_TYPE_A::SINGLE_EDGE,
            true => INTERRUPT_BOTH_EDGE_TYPE_A::BOTH_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE`"]
    #[inline(always)]
    pub fn is_single_edge(&self) -> bool {
        *self == INTERRUPT_BOTH_EDGE_TYPE_A::SINGLE_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGE`"]
    #[inline(always)]
    pub fn is_both_edge(&self) -> bool {
        *self == INTERRUPT_BOTH_EDGE_TYPE_A::BOTH_EDGE
    }
}
#[doc = "Field `ENCODED_ID_WIDTH` reader - The value of this register is derived from the GPIO_ID_WIDTH configuration parameter."]
pub type ENCODED_ID_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "The value of this register is derived from the GPIO_ID configuration parameter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_ID_A {
    #[doc = "0: GPIO_ID not included"]
    NOT_INCLUDED = 0,
    #[doc = "1: GPIO_ID included"]
    INCLUDED = 1,
}
impl From<GPIO_ID_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_ID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_ID` reader - The value of this register is derived from the GPIO_ID configuration parameter."]
pub type GPIO_ID_R = crate::BitReader<GPIO_ID_A>;
impl GPIO_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_ID_A {
        match self.bits {
            false => GPIO_ID_A::NOT_INCLUDED,
            true => GPIO_ID_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INCLUDED`"]
    #[inline(always)]
    pub fn is_not_included(&self) -> bool {
        *self == GPIO_ID_A::NOT_INCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == GPIO_ID_A::INCLUDED
    }
}
#[doc = "The value of this register is derived from the GPIO_ADD_ENCODED_PARAMS configuration parameter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADD_ENCODED_PARAMS_A {
    #[doc = "0: Encoded parameters not added"]
    NOT_ADDED = 0,
    #[doc = "1: Encoded parameters added"]
    ADDED = 1,
}
impl From<ADD_ENCODED_PARAMS_A> for bool {
    #[inline(always)]
    fn from(variant: ADD_ENCODED_PARAMS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD_ENCODED_PARAMS` reader - The value of this register is derived from the GPIO_ADD_ENCODED_PARAMS configuration parameter."]
pub type ADD_ENCODED_PARAMS_R = crate::BitReader<ADD_ENCODED_PARAMS_A>;
impl ADD_ENCODED_PARAMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADD_ENCODED_PARAMS_A {
        match self.bits {
            false => ADD_ENCODED_PARAMS_A::NOT_ADDED,
            true => ADD_ENCODED_PARAMS_A::ADDED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ADDED`"]
    #[inline(always)]
    pub fn is_not_added(&self) -> bool {
        *self == ADD_ENCODED_PARAMS_A::NOT_ADDED
    }
    #[doc = "Checks if the value of the field is `ADDED`"]
    #[inline(always)]
    pub fn is_added(&self) -> bool {
        *self == ADD_ENCODED_PARAMS_A::ADDED
    }
}
#[doc = "The value of this register is derived from the GPIO_DEBOUNCE configuration parameter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBOUNCE_A {
    #[doc = "0: Exclude debounce capability"]
    EXCLUDE = 0,
    #[doc = "1: Include debounce capability"]
    INCLUDE = 1,
}
impl From<DEBOUNCE_A> for bool {
    #[inline(always)]
    fn from(variant: DEBOUNCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBOUNCE` reader - The value of this register is derived from the GPIO_DEBOUNCE configuration parameter."]
pub type DEBOUNCE_R = crate::BitReader<DEBOUNCE_A>;
impl DEBOUNCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBOUNCE_A {
        match self.bits {
            false => DEBOUNCE_A::EXCLUDE,
            true => DEBOUNCE_A::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == DEBOUNCE_A::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == DEBOUNCE_A::INCLUDE
    }
}
#[doc = "The value of this register is derived from the GPIO_PORTA_INTR configuration parameter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTA_INTR_A {
    #[doc = "0: PORT A is not used as an interrupt source"]
    DISABLED = 0,
    #[doc = "1: PORT A is used as an interrupt source"]
    ENABLED = 1,
}
impl From<PORTA_INTR_A> for bool {
    #[inline(always)]
    fn from(variant: PORTA_INTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORTA_INTR` reader - The value of this register is derived from the GPIO_PORTA_INTR configuration parameter."]
pub type PORTA_INTR_R = crate::BitReader<PORTA_INTR_A>;
impl PORTA_INTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTA_INTR_A {
        match self.bits {
            false => PORTA_INTR_A::DISABLED,
            true => PORTA_INTR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PORTA_INTR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PORTA_INTR_A::ENABLED
    }
}
#[doc = "The value of this register is derived from the GPIO_HW_PORT\\[ABCD\\]
configuration parameter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW_PORT_A {
    #[doc = "0: PORT has external, auxiliary hardware signals excluded"]
    EXCLUDED = 0,
    #[doc = "1: PORT has external, auxiliary hardware signals included"]
    INCLUDED = 1,
}
impl From<HW_PORT_A> for bool {
    #[inline(always)]
    fn from(variant: HW_PORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `HW_PORT(0-3)` reader - The value of this register is derived from the GPIO_HW_PORT\\[ABCD\\]
configuration parameter."]
pub type HW_PORT_R = crate::BitReader<HW_PORT_A>;
impl HW_PORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HW_PORT_A {
        match self.bits {
            false => HW_PORT_A::EXCLUDED,
            true => HW_PORT_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == HW_PORT_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == HW_PORT_A::INCLUDED
    }
}
#[doc = "The value of this register is derived from the GPIO_PORT\\[ABCD\\]_SINGLE_CTL configuration parameter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORT_SINGLE_CTL_A {
    #[doc = "0: PORT is not controlled from a single source"]
    NOT_SINGLE = 0,
    #[doc = "1: PORT is controlled from a single source"]
    SINGLE = 1,
}
impl From<PORT_SINGLE_CTL_A> for bool {
    #[inline(always)]
    fn from(variant: PORT_SINGLE_CTL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `PORT(0-3)_SINGLE_CTL` reader - The value of this register is derived from the GPIO_PORT\\[ABCD\\]_SINGLE_CTL configuration parameter."]
pub type PORT_SINGLE_CTL_R = crate::BitReader<PORT_SINGLE_CTL_A>;
impl PORT_SINGLE_CTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT_SINGLE_CTL_A {
        match self.bits {
            false => PORT_SINGLE_CTL_A::NOT_SINGLE,
            true => PORT_SINGLE_CTL_A::SINGLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SINGLE`"]
    #[inline(always)]
    pub fn is_not_single(&self) -> bool {
        *self == PORT_SINGLE_CTL_A::NOT_SINGLE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == PORT_SINGLE_CTL_A::SINGLE
    }
}
#[doc = "The value of this register is derived from the GPIO_NUM_PORT configuration parameter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NUM_PORTS_A {
    #[doc = "0: One port"]
    NUM_PORTS_1 = 0,
    #[doc = "1: Two ports"]
    NUM_PORTS_2 = 1,
    #[doc = "2: Three ports"]
    NUM_PORTS_3 = 2,
    #[doc = "3: Four ports"]
    NUM_PORTS_4 = 3,
}
impl From<NUM_PORTS_A> for u8 {
    #[inline(always)]
    fn from(variant: NUM_PORTS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NUM_PORTS` reader - The value of this register is derived from the GPIO_NUM_PORT configuration parameter."]
pub type NUM_PORTS_R = crate::FieldReader<u8, NUM_PORTS_A>;
impl NUM_PORTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUM_PORTS_A {
        match self.bits {
            0 => NUM_PORTS_A::NUM_PORTS_1,
            1 => NUM_PORTS_A::NUM_PORTS_2,
            2 => NUM_PORTS_A::NUM_PORTS_3,
            3 => NUM_PORTS_A::NUM_PORTS_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NUM_PORTS_1`"]
    #[inline(always)]
    pub fn is_num_ports_1(&self) -> bool {
        *self == NUM_PORTS_A::NUM_PORTS_1
    }
    #[doc = "Checks if the value of the field is `NUM_PORTS_2`"]
    #[inline(always)]
    pub fn is_num_ports_2(&self) -> bool {
        *self == NUM_PORTS_A::NUM_PORTS_2
    }
    #[doc = "Checks if the value of the field is `NUM_PORTS_3`"]
    #[inline(always)]
    pub fn is_num_ports_3(&self) -> bool {
        *self == NUM_PORTS_A::NUM_PORTS_3
    }
    #[doc = "Checks if the value of the field is `NUM_PORTS_4`"]
    #[inline(always)]
    pub fn is_num_ports_4(&self) -> bool {
        *self == NUM_PORTS_A::NUM_PORTS_4
    }
}
#[doc = "The value of this register is derived from the GPIO_APB_DATA_WIDTH configuration parameter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum APB_DATA_WIDTH_A {
    #[doc = "0: APB DATA WIDTH is 8 bits"]
    APB_8BITS = 0,
    #[doc = "1: APB DATA WIDTH is 16 bits"]
    APB_16BITS = 1,
    #[doc = "2: APB DATA WIDTH is 32 bits"]
    APB_32BITS = 2,
}
impl From<APB_DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: APB_DATA_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `APB_DATA_WIDTH` reader - The value of this register is derived from the GPIO_APB_DATA_WIDTH configuration parameter."]
pub type APB_DATA_WIDTH_R = crate::FieldReader<u8, APB_DATA_WIDTH_A>;
impl APB_DATA_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<APB_DATA_WIDTH_A> {
        match self.bits {
            0 => Some(APB_DATA_WIDTH_A::APB_8BITS),
            1 => Some(APB_DATA_WIDTH_A::APB_16BITS),
            2 => Some(APB_DATA_WIDTH_A::APB_32BITS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APB_8BITS`"]
    #[inline(always)]
    pub fn is_apb_8bits(&self) -> bool {
        *self == APB_DATA_WIDTH_A::APB_8BITS
    }
    #[doc = "Checks if the value of the field is `APB_16BITS`"]
    #[inline(always)]
    pub fn is_apb_16bits(&self) -> bool {
        *self == APB_DATA_WIDTH_A::APB_16BITS
    }
    #[doc = "Checks if the value of the field is `APB_32BITS`"]
    #[inline(always)]
    pub fn is_apb_32bits(&self) -> bool {
        *self == APB_DATA_WIDTH_A::APB_32BITS
    }
}
impl R {
    #[doc = "Bit 21 - The value of this register is derived from the GPIO_INT_BOTHEDGE configuration parameter"]
    #[inline(always)]
    pub fn interrupt_both_edge_type(&self) -> INTERRUPT_BOTH_EDGE_TYPE_R {
        INTERRUPT_BOTH_EDGE_TYPE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 16:20 - The value of this register is derived from the GPIO_ID_WIDTH configuration parameter."]
    #[inline(always)]
    pub fn encoded_id_width(&self) -> ENCODED_ID_WIDTH_R {
        ENCODED_ID_WIDTH_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - The value of this register is derived from the GPIO_ID configuration parameter."]
    #[inline(always)]
    pub fn gpio_id(&self) -> GPIO_ID_R {
        GPIO_ID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - The value of this register is derived from the GPIO_ADD_ENCODED_PARAMS configuration parameter."]
    #[inline(always)]
    pub fn add_encoded_params(&self) -> ADD_ENCODED_PARAMS_R {
        ADD_ENCODED_PARAMS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - The value of this register is derived from the GPIO_DEBOUNCE configuration parameter."]
    #[inline(always)]
    pub fn debounce(&self) -> DEBOUNCE_R {
        DEBOUNCE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - The value of this register is derived from the GPIO_PORTA_INTR configuration parameter."]
    #[inline(always)]
    pub fn porta_intr(&self) -> PORTA_INTR_R {
        PORTA_INTR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "The value of this register is derived from the GPIO_HW_PORT\\[ABCD\\]
configuration parameter."]
    #[inline(always)]
    pub unsafe fn hw_port(&self, n: u8) -> HW_PORT_R {
        HW_PORT_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Bit 8 - The value of this register is derived from the GPIO_HW_PORT\\[ABCD\\]
configuration parameter."]
    #[inline(always)]
    pub fn hw_porta(&self) -> HW_PORT_R {
        HW_PORT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The value of this register is derived from the GPIO_HW_PORT\\[ABCD\\]
configuration parameter."]
    #[inline(always)]
    pub fn hw_portb(&self) -> HW_PORT_R {
        HW_PORT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The value of this register is derived from the GPIO_HW_PORT\\[ABCD\\]
configuration parameter."]
    #[inline(always)]
    pub fn hw_portc(&self) -> HW_PORT_R {
        HW_PORT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The value of this register is derived from the GPIO_HW_PORT\\[ABCD\\]
configuration parameter."]
    #[inline(always)]
    pub fn hw_portd(&self) -> HW_PORT_R {
        HW_PORT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "The value of this register is derived from the GPIO_PORT\\[ABCD\\]_SINGLE_CTL configuration parameter."]
    #[inline(always)]
    pub unsafe fn port_single_ctl(&self, n: u8) -> PORT_SINGLE_CTL_R {
        PORT_SINGLE_CTL_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Bit 4 - The value of this register is derived from the GPIO_PORT\\[ABCD\\]_SINGLE_CTL configuration parameter."]
    #[inline(always)]
    pub fn porta_single_ctl(&self) -> PORT_SINGLE_CTL_R {
        PORT_SINGLE_CTL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The value of this register is derived from the GPIO_PORT\\[ABCD\\]_SINGLE_CTL configuration parameter."]
    #[inline(always)]
    pub fn portb_single_ctl(&self) -> PORT_SINGLE_CTL_R {
        PORT_SINGLE_CTL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The value of this register is derived from the GPIO_PORT\\[ABCD\\]_SINGLE_CTL configuration parameter."]
    #[inline(always)]
    pub fn portc_single_ctl(&self) -> PORT_SINGLE_CTL_R {
        PORT_SINGLE_CTL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The value of this register is derived from the GPIO_PORT\\[ABCD\\]_SINGLE_CTL configuration parameter."]
    #[inline(always)]
    pub fn portd_single_ctl(&self) -> PORT_SINGLE_CTL_R {
        PORT_SINGLE_CTL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 2:3 - The value of this register is derived from the GPIO_NUM_PORT configuration parameter."]
    #[inline(always)]
    pub fn num_ports(&self) -> NUM_PORTS_R {
        NUM_PORTS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - The value of this register is derived from the GPIO_APB_DATA_WIDTH configuration parameter."]
    #[inline(always)]
    pub fn apb_data_width(&self) -> APB_DATA_WIDTH_R {
        APB_DATA_WIDTH_R::new((self.bits & 3) as u8)
    }
}
#[doc = "parameters info\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_config_reg1](index.html) module"]
pub struct GPIO_CONFIG_REG1_SPEC;
impl crate::RegisterSpec for GPIO_CONFIG_REG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_config_reg1::R](R) reader structure"]
impl crate::Readable for GPIO_CONFIG_REG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_CONFIG_REG1 to value 0"]
impl crate::Resettable for GPIO_CONFIG_REG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
