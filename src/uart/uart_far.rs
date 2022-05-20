#[doc = "Register `UART_FAR` reader"]
pub struct R(crate::R<UART_FAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_FAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_FAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_FAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "FIFO Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAR_A {
    #[doc = "0: FIFO access mode disabled"]
    DISABLED = 0,
    #[doc = "1: FIFO access mode enabled"]
    ENABLED = 1,
}
impl From<FAR_A> for bool {
    #[inline(always)]
    fn from(variant: FAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAR` reader - FIFO Access"]
pub type FAR_R = crate::BitReader<FAR_A>;
impl FAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAR_A {
        match self.bits {
            false => FAR_A::DISABLED,
            true => FAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FAR_A::ENABLED
    }
}
impl R {
    #[doc = "Bit 0 - FIFO Access"]
    #[inline(always)]
    pub fn far(&self) -> FAR_R {
        FAR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "FIFO Access Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_far](index.html) module"]
pub struct UART_FAR_SPEC;
impl crate::RegisterSpec for UART_FAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_far::R](R) reader structure"]
impl crate::Readable for UART_FAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_FAR to value 0"]
impl crate::Resettable for UART_FAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
