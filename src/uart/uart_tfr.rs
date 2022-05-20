#[doc = "Register `UART_TFR` reader"]
pub struct R(crate::R<UART_TFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_TFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_TFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_TFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TFR` reader - Transmit FIFO Read"]
pub type TFR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO Read"]
    #[inline(always)]
    pub fn tfr(&self) -> TFR_R {
        TFR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Transmit FIFO Read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_tfr](index.html) module"]
pub struct UART_TFR_SPEC;
impl crate::RegisterSpec for UART_TFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_tfr::R](R) reader structure"]
impl crate::Readable for UART_TFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_TFR to value 0"]
impl crate::Resettable for UART_TFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
