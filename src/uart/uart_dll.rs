#[doc = "Register `UART_DLL` reader"]
pub struct R(crate::R<UART_DLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_DLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_DLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_DLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_DLL` writer"]
pub struct W(crate::W<UART_DLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_DLL_SPEC>;
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
impl From<crate::W<UART_DLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_DLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLL` reader - Divisor Latch Low"]
pub type DLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLL` writer - Divisor Latch Low"]
pub type DLL_W<'a> = crate::FieldWriter<'a, u32, UART_DLL_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Divisor Latch Low"]
    #[inline(always)]
    pub fn dll(&self) -> DLL_R {
        DLL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Divisor Latch Low"]
    #[inline(always)]
    pub fn dll(&mut self) -> DLL_W {
        DLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divisor Latch (Low) Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_dll](index.html) module"]
pub struct UART_DLL_SPEC;
impl crate::RegisterSpec for UART_DLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_dll::R](R) reader structure"]
impl crate::Readable for UART_DLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_dll::W](W) writer structure"]
impl crate::Writable for UART_DLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_DLL to value 0"]
impl crate::Resettable for UART_DLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
