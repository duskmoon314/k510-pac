#[doc = "Register `UART_TAR` reader"]
pub struct R(crate::R<UART_TAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_TAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_TAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_TAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_TAR` writer"]
pub struct W(crate::W<UART_TAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_TAR_SPEC>;
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
impl From<crate::W<UART_TAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_TAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAR` reader - Transmit Address"]
pub type TAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAR` writer - Transmit Address"]
pub type TAR_W<'a> = crate::FieldWriter<'a, u32, UART_TAR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Transmit Address"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Address"]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W {
        TAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_tar](index.html) module"]
pub struct UART_TAR_SPEC;
impl crate::RegisterSpec for UART_TAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_tar::R](R) reader structure"]
impl crate::Readable for UART_TAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_tar::W](W) writer structure"]
impl crate::Writable for UART_TAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_TAR to value 0"]
impl crate::Resettable for UART_TAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
