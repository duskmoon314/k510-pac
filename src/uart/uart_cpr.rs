#[doc = "Register `UART_CPR` reader"]
pub struct R(crate::R<UART_CPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_CPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_CPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_CPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "FIFO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIFO_MODE_A {
    #[doc = "0: FIFO mode is 0"]
    FIFO_MODE_0 = 0,
    #[doc = "1: FIFO mode is 16"]
    FIFO_MODE_16 = 1,
    #[doc = "2: FIFO mode is 32"]
    FIFO_MODE_32 = 2,
    #[doc = "4: FIFO mode is 64"]
    FIFO_MODE_64 = 4,
    #[doc = "8: FIFO mode is 128"]
    FIFO_MODE_128 = 8,
    #[doc = "16: FIFO mode is 256"]
    FIFO_MODE_256 = 16,
    #[doc = "32: FIFO mode is 512"]
    FIFO_MODE_512 = 32,
    #[doc = "64: FIFO mode is 1024"]
    FIFO_MODE_1024 = 64,
    #[doc = "128: FIFO mode is 2048"]
    FIFO_MODE_2048 = 128,
}
impl From<FIFO_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FIFO_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FIFO_MODE` reader - FIFO Mode"]
pub type FIFO_MODE_R = crate::FieldReader<u8, FIFO_MODE_A>;
impl FIFO_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FIFO_MODE_A> {
        match self.bits {
            0 => Some(FIFO_MODE_A::FIFO_MODE_0),
            1 => Some(FIFO_MODE_A::FIFO_MODE_16),
            2 => Some(FIFO_MODE_A::FIFO_MODE_32),
            4 => Some(FIFO_MODE_A::FIFO_MODE_64),
            8 => Some(FIFO_MODE_A::FIFO_MODE_128),
            16 => Some(FIFO_MODE_A::FIFO_MODE_256),
            32 => Some(FIFO_MODE_A::FIFO_MODE_512),
            64 => Some(FIFO_MODE_A::FIFO_MODE_1024),
            128 => Some(FIFO_MODE_A::FIFO_MODE_2048),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_MODE_0`"]
    #[inline(always)]
    pub fn is_fifo_mode_0(&self) -> bool {
        *self == FIFO_MODE_A::FIFO_MODE_0
    }
    #[doc = "Checks if the value of the field is `FIFO_MODE_16`"]
    #[inline(always)]
    pub fn is_fifo_mode_16(&self) -> bool {
        *self == FIFO_MODE_A::FIFO_MODE_16
    }
    #[doc = "Checks if the value of the field is `FIFO_MODE_32`"]
    #[inline(always)]
    pub fn is_fifo_mode_32(&self) -> bool {
        *self == FIFO_MODE_A::FIFO_MODE_32
    }
    #[doc = "Checks if the value of the field is `FIFO_MODE_64`"]
    #[inline(always)]
    pub fn is_fifo_mode_64(&self) -> bool {
        *self == FIFO_MODE_A::FIFO_MODE_64
    }
    #[doc = "Checks if the value of the field is `FIFO_MODE_128`"]
    #[inline(always)]
    pub fn is_fifo_mode_128(&self) -> bool {
        *self == FIFO_MODE_A::FIFO_MODE_128
    }
    #[doc = "Checks if the value of the field is `FIFO_MODE_256`"]
    #[inline(always)]
    pub fn is_fifo_mode_256(&self) -> bool {
        *self == FIFO_MODE_A::FIFO_MODE_256
    }
    #[doc = "Checks if the value of the field is `FIFO_MODE_512`"]
    #[inline(always)]
    pub fn is_fifo_mode_512(&self) -> bool {
        *self == FIFO_MODE_A::FIFO_MODE_512
    }
    #[doc = "Checks if the value of the field is `FIFO_MODE_1024`"]
    #[inline(always)]
    pub fn is_fifo_mode_1024(&self) -> bool {
        *self == FIFO_MODE_A::FIFO_MODE_1024
    }
    #[doc = "Checks if the value of the field is `FIFO_MODE_2048`"]
    #[inline(always)]
    pub fn is_fifo_mode_2048(&self) -> bool {
        *self == FIFO_MODE_A::FIFO_MODE_2048
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_EXTRA_A {
    #[doc = "0: DMA_EXTRA disabled"]
    DISABLED = 0,
    #[doc = "1: DMA_EXTRA enabled"]
    ENABLED = 1,
}
impl From<DMA_EXTRA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_EXTRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_EXTRA` reader - "]
pub type DMA_EXTRA_R = crate::BitReader<DMA_EXTRA_A>;
impl DMA_EXTRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_EXTRA_A {
        match self.bits {
            false => DMA_EXTRA_A::DISABLED,
            true => DMA_EXTRA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_EXTRA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_EXTRA_A::ENABLED
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_ADD_ENCODED_PARAMS_A {
    #[doc = "0: UART_ADD_ENCODED_PARAMS disabled"]
    DISABLED = 0,
    #[doc = "1: UART_ADD_ENCODED_PARAMS enabled"]
    ENABLED = 1,
}
impl From<UART_ADD_ENCODED_PARAMS_A> for bool {
    #[inline(always)]
    fn from(variant: UART_ADD_ENCODED_PARAMS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART_ADD_ENCODED_PARAMS` reader - "]
pub type UART_ADD_ENCODED_PARAMS_R = crate::BitReader<UART_ADD_ENCODED_PARAMS_A>;
impl UART_ADD_ENCODED_PARAMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART_ADD_ENCODED_PARAMS_A {
        match self.bits {
            false => UART_ADD_ENCODED_PARAMS_A::DISABLED,
            true => UART_ADD_ENCODED_PARAMS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UART_ADD_ENCODED_PARAMS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UART_ADD_ENCODED_PARAMS_A::ENABLED
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHADOW_A {
    #[doc = "0: SHADOW disabled"]
    DISABLED = 0,
    #[doc = "1: SHADOW enabled"]
    ENABLED = 1,
}
impl From<SHADOW_A> for bool {
    #[inline(always)]
    fn from(variant: SHADOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHADOW` reader - "]
pub type SHADOW_R = crate::BitReader<SHADOW_A>;
impl SHADOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHADOW_A {
        match self.bits {
            false => SHADOW_A::DISABLED,
            true => SHADOW_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SHADOW_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SHADOW_A::ENABLED
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_STAT_A {
    #[doc = "0: FIFO_STAT disabled"]
    DISABLED = 0,
    #[doc = "1: FIFO_STAT enabled"]
    ENABLED = 1,
}
impl From<FIFO_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_STAT` reader - "]
pub type FIFO_STAT_R = crate::BitReader<FIFO_STAT_A>;
impl FIFO_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_STAT_A {
        match self.bits {
            false => FIFO_STAT_A::DISABLED,
            true => FIFO_STAT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FIFO_STAT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIFO_STAT_A::ENABLED
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_ACCESS_A {
    #[doc = "0: FIFO_ACCESS disabled"]
    DISABLED = 0,
    #[doc = "1: FIFO_ACCESS enabled"]
    ENABLED = 1,
}
impl From<FIFO_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_ACCESS` reader - "]
pub type FIFO_ACCESS_R = crate::BitReader<FIFO_ACCESS_A>;
impl FIFO_ACCESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_ACCESS_A {
        match self.bits {
            false => FIFO_ACCESS_A::DISABLED,
            true => FIFO_ACCESS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FIFO_ACCESS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIFO_ACCESS_A::ENABLED
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDITIONAL_FEATURES_A {
    #[doc = "0: ADDITIONAL_FEATURES disabled"]
    DISABLED = 0,
    #[doc = "1: ADDITIONAL_FEATURES enabled"]
    ENABLED = 1,
}
impl From<ADDITIONAL_FEATURES_A> for bool {
    #[inline(always)]
    fn from(variant: ADDITIONAL_FEATURES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDITIONAL_FEATURES` reader - "]
pub type ADDITIONAL_FEATURES_R = crate::BitReader<ADDITIONAL_FEATURES_A>;
impl ADDITIONAL_FEATURES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDITIONAL_FEATURES_A {
        match self.bits {
            false => ADDITIONAL_FEATURES_A::DISABLED,
            true => ADDITIONAL_FEATURES_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDITIONAL_FEATURES_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDITIONAL_FEATURES_A::ENABLED
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIR_LP_MODE_A {
    #[doc = "0: SIR_LP_MODE disabled"]
    DISABLED = 0,
    #[doc = "1: SIR_LP_MODE enabled"]
    ENABLED = 1,
}
impl From<SIR_LP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: SIR_LP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIR_LP_MODE` reader - "]
pub type SIR_LP_MODE_R = crate::BitReader<SIR_LP_MODE_A>;
impl SIR_LP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIR_LP_MODE_A {
        match self.bits {
            false => SIR_LP_MODE_A::DISABLED,
            true => SIR_LP_MODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SIR_LP_MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SIR_LP_MODE_A::ENABLED
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIR_MODE_A {
    #[doc = "0: SIR_MODE disabled"]
    DISABLED = 0,
    #[doc = "1: SIR_MODE enabled"]
    ENABLED = 1,
}
impl From<SIR_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: SIR_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIR_MODE` reader - "]
pub type SIR_MODE_R = crate::BitReader<SIR_MODE_A>;
impl SIR_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIR_MODE_A {
        match self.bits {
            false => SIR_MODE_A::DISABLED,
            true => SIR_MODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SIR_MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SIR_MODE_A::ENABLED
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THRE_MODE_A {
    #[doc = "0: THRE_MODE disabled"]
    DISABLED = 0,
    #[doc = "1: THRE_MODE enabled"]
    ENABLED = 1,
}
impl From<THRE_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: THRE_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THRE_MODE` reader - "]
pub type THRE_MODE_R = crate::BitReader<THRE_MODE_A>;
impl THRE_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THRE_MODE_A {
        match self.bits {
            false => THRE_MODE_A::DISABLED,
            true => THRE_MODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == THRE_MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == THRE_MODE_A::ENABLED
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFCE_MODE_A {
    #[doc = "0: AFCE_MODE disabled"]
    DISABLED = 0,
    #[doc = "1: AFCE_MODE enabled"]
    ENABLED = 1,
}
impl From<AFCE_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: AFCE_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFCE_MODE` reader - "]
pub type AFCE_MODE_R = crate::BitReader<AFCE_MODE_A>;
impl AFCE_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFCE_MODE_A {
        match self.bits {
            false => AFCE_MODE_A::DISABLED,
            true => AFCE_MODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AFCE_MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AFCE_MODE_A::ENABLED
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum APB_DATA_WIDTH_A {
    #[doc = "0: APB data width is 8 bits"]
    APB_8BITS = 0,
    #[doc = "1: APB data width is 16 bits"]
    APB_16BITS = 1,
    #[doc = "2: APB data width is 32 bits"]
    APB_32BITS = 2,
}
impl From<APB_DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: APB_DATA_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `APB_DATA_WIDTH` reader - "]
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
    #[doc = "Bits 16:23 - FIFO Mode"]
    #[inline(always)]
    pub fn fifo_mode(&self) -> FIFO_MODE_R {
        FIFO_MODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dma_extra(&self) -> DMA_EXTRA_R {
        DMA_EXTRA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uart_add_encoded_params(&self) -> UART_ADD_ENCODED_PARAMS_R {
        UART_ADD_ENCODED_PARAMS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn shadow(&self) -> SHADOW_R {
        SHADOW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fifo_stat(&self) -> FIFO_STAT_R {
        FIFO_STAT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fifo_access(&self) -> FIFO_ACCESS_R {
        FIFO_ACCESS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn additional_features(&self) -> ADDITIONAL_FEATURES_R {
        ADDITIONAL_FEATURES_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sir_lp_mode(&self) -> SIR_LP_MODE_R {
        SIR_LP_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sir_mode(&self) -> SIR_MODE_R {
        SIR_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn thre_mode(&self) -> THRE_MODE_R {
        THRE_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn afce_mode(&self) -> AFCE_MODE_R {
        AFCE_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn apb_data_width(&self) -> APB_DATA_WIDTH_R {
        APB_DATA_WIDTH_R::new((self.bits & 3) as u8)
    }
}
#[doc = "Component Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_cpr](index.html) module"]
pub struct UART_CPR_SPEC;
impl crate::RegisterSpec for UART_CPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_cpr::R](R) reader structure"]
impl crate::Readable for UART_CPR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_CPR to value 0"]
impl crate::Resettable for UART_CPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
