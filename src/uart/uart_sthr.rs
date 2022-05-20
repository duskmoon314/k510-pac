#[doc = "Register `UART_STHR%s` writer"]
pub struct W(crate::W<UART_STHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_STHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UART_STHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_STHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STHR` writer - Shadow Transmit Holding Register"]
pub type STHR_W<'a> = crate::FieldWriter<'a, u32, UART_STHR_SPEC, u16, u16, 9, 0>;
impl W {
    #[doc = "Bits 0:8 - Shadow Transmit Holding Register"]
    #[inline(always)]
    pub fn sthr(&mut self) -> STHR_W {
        STHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow Transmit Holding Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_sthr](index.html) module"]
pub struct UART_STHR_SPEC;
impl crate::RegisterSpec for UART_STHR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uart_sthr::W](W) writer structure"]
impl crate::Writable for UART_STHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_STHR%s to value 0"]
impl crate::Resettable for UART_STHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
