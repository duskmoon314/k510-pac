#[doc = "Register `UART_RFL` reader"]
pub struct R(crate::R<UART_RFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_RFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_RFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_RFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFL` reader - Receive FIFO Level"]
pub type RFL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Receive FIFO Level"]
    #[inline(always)]
    pub fn rfl(&self) -> RFL_R {
        RFL_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Receive FIFO Level\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rfl](index.html) module"]
pub struct UART_RFL_SPEC;
impl crate::RegisterSpec for UART_RFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_rfl::R](R) reader structure"]
impl crate::Readable for UART_RFL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_RFL to value 0"]
impl crate::Resettable for UART_RFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
