#[doc = "Register `UART_SRBR%s` reader"]
pub struct R(crate::R<UART_SRBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SRBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SRBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SRBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRBR` reader - Shadow Receive Buffer Register"]
pub type SRBR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:8 - Shadow Receive Buffer Register"]
    #[inline(always)]
    pub fn srbr(&self) -> SRBR_R {
        SRBR_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Shadow Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_srbr](index.html) module"]
pub struct UART_SRBR_SPEC;
impl crate::RegisterSpec for UART_SRBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_srbr::R](R) reader structure"]
impl crate::Readable for UART_SRBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_SRBR%s to value 0"]
impl crate::Resettable for UART_SRBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
