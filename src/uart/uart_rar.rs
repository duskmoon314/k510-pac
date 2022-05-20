#[doc = "Register `UART_RAR` reader"]
pub struct R(crate::R<UART_RAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_RAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_RAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_RAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_RAR` writer"]
pub struct W(crate::W<UART_RAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_RAR_SPEC>;
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
impl From<crate::W<UART_RAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_RAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAR` reader - Receive Address"]
pub type RAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RAR` writer - Receive Address"]
pub type RAR_W<'a> = crate::FieldWriter<'a, u32, UART_RAR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Receive Address"]
    #[inline(always)]
    pub fn rar(&self) -> RAR_R {
        RAR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Address"]
    #[inline(always)]
    pub fn rar(&mut self) -> RAR_W {
        RAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rar](index.html) module"]
pub struct UART_RAR_SPEC;
impl crate::RegisterSpec for UART_RAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_rar::R](R) reader structure"]
impl crate::Readable for UART_RAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_rar::W](W) writer structure"]
impl crate::Writable for UART_RAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_RAR to value 0"]
impl crate::Resettable for UART_RAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
