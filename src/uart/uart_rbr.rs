#[doc = "Register `UART_RBR` reader"]
pub struct R(crate::R<UART_RBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_RBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_RBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_RBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RBR` reader - Receive Buffer Register"]
pub type RBR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:8 - Receive Buffer Register"]
    #[inline(always)]
    pub fn rbr(&self) -> RBR_R {
        RBR_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rbr](index.html) module"]
pub struct UART_RBR_SPEC;
impl crate::RegisterSpec for UART_RBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_rbr::R](R) reader structure"]
impl crate::Readable for UART_RBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_RBR to value 0"]
impl crate::Resettable for UART_RBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
