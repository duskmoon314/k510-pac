#[doc = "Register `UART_DET` reader"]
pub struct R(crate::R<UART_DET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_DET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_DET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_DET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_DET` writer"]
pub struct W(crate::W<UART_DET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_DET_SPEC>;
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
impl From<crate::W<UART_DET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_DET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DE_DEASSERTION_TIME` reader - DE Deassertion Time"]
pub type DE_DEASSERTION_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DE_DEASSERTION_TIME` writer - DE Deassertion Time"]
pub type DE_DEASSERTION_TIME_W<'a> = crate::FieldWriter<'a, u32, UART_DET_SPEC, u8, u8, 8, 16>;
#[doc = "Field `DE_ASSERTION_TIME` reader - DE Assertion Time"]
pub type DE_ASSERTION_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DE_ASSERTION_TIME` writer - DE Assertion Time"]
pub type DE_ASSERTION_TIME_W<'a> = crate::FieldWriter<'a, u32, UART_DET_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 16:23 - DE Deassertion Time"]
    #[inline(always)]
    pub fn de_deassertion_time(&self) -> DE_DEASSERTION_TIME_R {
        DE_DEASSERTION_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - DE Assertion Time"]
    #[inline(always)]
    pub fn de_assertion_time(&self) -> DE_ASSERTION_TIME_R {
        DE_ASSERTION_TIME_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - DE Deassertion Time"]
    #[inline(always)]
    pub fn de_deassertion_time(&mut self) -> DE_DEASSERTION_TIME_W {
        DE_DEASSERTION_TIME_W::new(self)
    }
    #[doc = "Bits 0:7 - DE Assertion Time"]
    #[inline(always)]
    pub fn de_assertion_time(&mut self) -> DE_ASSERTION_TIME_W {
        DE_ASSERTION_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Driver Output Enable Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_det](index.html) module"]
pub struct UART_DET_SPEC;
impl crate::RegisterSpec for UART_DET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_det::R](R) reader structure"]
impl crate::Readable for UART_DET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_det::W](W) writer structure"]
impl crate::Writable for UART_DET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_DET to value 0"]
impl crate::Resettable for UART_DET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
