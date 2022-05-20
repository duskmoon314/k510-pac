#[doc = "Register `UART_DLH` reader"]
pub struct R(crate::R<UART_DLH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_DLH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_DLH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_DLH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_DLH` writer"]
pub struct W(crate::W<UART_DLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_DLH_SPEC>;
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
impl From<crate::W<UART_DLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_DLH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLH` reader - Divisor Latch High"]
pub type DLH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLH` writer - Divisor Latch High"]
pub type DLH_W<'a> = crate::FieldWriter<'a, u32, UART_DLH_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Divisor Latch High"]
    #[inline(always)]
    pub fn dlh(&self) -> DLH_R {
        DLH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Divisor Latch High"]
    #[inline(always)]
    pub fn dlh(&mut self) -> DLH_W {
        DLH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divisor Latch (High) Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_dlh](index.html) module"]
pub struct UART_DLH_SPEC;
impl crate::RegisterSpec for UART_DLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_dlh::R](R) reader structure"]
impl crate::Readable for UART_DLH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_dlh::W](W) writer structure"]
impl crate::Writable for UART_DLH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_DLH to value 0"]
impl crate::Resettable for UART_DLH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
