#[doc = "Register `UART_UCV` reader"]
pub struct R(crate::R<UART_UCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_UCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_UCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_UCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UART_COMPONENT_VERSION` reader - "]
pub type UART_COMPONENT_VERSION_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn uart_component_version(&self) -> UART_COMPONENT_VERSION_R {
        UART_COMPONENT_VERSION_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ucv](index.html) module"]
pub struct UART_UCV_SPEC;
impl crate::RegisterSpec for UART_UCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_ucv::R](R) reader structure"]
impl crate::Readable for UART_UCV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_UCV to value 0"]
impl crate::Resettable for UART_UCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
