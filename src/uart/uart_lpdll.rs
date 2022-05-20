#[doc = "Register `UART_LPDLL` reader"]
pub struct R(crate::R<UART_LPDLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_LPDLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_LPDLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_LPDLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_LPDLL` writer"]
pub struct W(crate::W<UART_LPDLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_LPDLL_SPEC>;
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
impl From<crate::W<UART_LPDLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_LPDLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPDLL` reader - Low Power Divisor Latch Low Register"]
pub type LPDLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPDLL` writer - Low Power Divisor Latch Low Register"]
pub type LPDLL_W<'a> = crate::FieldWriter<'a, u32, UART_LPDLL_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Low Power Divisor Latch Low Register"]
    #[inline(always)]
    pub fn lpdll(&self) -> LPDLL_R {
        LPDLL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low Power Divisor Latch Low Register"]
    #[inline(always)]
    pub fn lpdll(&mut self) -> LPDLL_W {
        LPDLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power Divisor Latch Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_lpdll](index.html) module"]
pub struct UART_LPDLL_SPEC;
impl crate::RegisterSpec for UART_LPDLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_lpdll::R](R) reader structure"]
impl crate::Readable for UART_LPDLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_lpdll::W](W) writer structure"]
impl crate::Writable for UART_LPDLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_LPDLL to value 0"]
impl crate::Resettable for UART_LPDLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
