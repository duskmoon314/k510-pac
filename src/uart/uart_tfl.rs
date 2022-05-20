#[doc = "Register `UART_TFL` reader"]
pub struct R(crate::R<UART_TFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_TFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_TFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_TFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TFL` reader - Transmit FIFO Level"]
pub type TFL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Transmit FIFO Level"]
    #[inline(always)]
    pub fn tfl(&self) -> TFL_R {
        TFL_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Transmit FIFO Level\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_tfl](index.html) module"]
pub struct UART_TFL_SPEC;
impl crate::RegisterSpec for UART_TFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_tfl::R](R) reader structure"]
impl crate::Readable for UART_TFL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_TFL to value 0"]
impl crate::Resettable for UART_TFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
