#[doc = "Register `UART_CTR` reader"]
pub struct R(crate::R<UART_CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERIPHERAL_ID` reader - "]
pub type PERIPHERAL_ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn peripheral_id(&self) -> PERIPHERAL_ID_R {
        PERIPHERAL_ID_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ctr](index.html) module"]
pub struct UART_CTR_SPEC;
impl crate::RegisterSpec for UART_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_ctr::R](R) reader structure"]
impl crate::Readable for UART_CTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_CTR to value 0"]
impl crate::Resettable for UART_CTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
