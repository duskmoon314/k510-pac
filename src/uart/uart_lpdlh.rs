#[doc = "Register `UART_LPDLH` reader"]
pub struct R(crate::R<UART_LPDLH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_LPDLH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_LPDLH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_LPDLH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_LPDLH` writer"]
pub struct W(crate::W<UART_LPDLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_LPDLH_SPEC>;
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
impl From<crate::W<UART_LPDLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_LPDLH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPDLH` reader - Low Power Divisor Latch High Register"]
pub type LPDLH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPDLH` writer - Low Power Divisor Latch High Register"]
pub type LPDLH_W<'a> = crate::FieldWriter<'a, u32, UART_LPDLH_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Low Power Divisor Latch High Register"]
    #[inline(always)]
    pub fn lpdlh(&self) -> LPDLH_R {
        LPDLH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low Power Divisor Latch High Register"]
    #[inline(always)]
    pub fn lpdlh(&mut self) -> LPDLH_W {
        LPDLH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power Divisor Latch High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_lpdlh](index.html) module"]
pub struct UART_LPDLH_SPEC;
impl crate::RegisterSpec for UART_LPDLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_lpdlh::R](R) reader structure"]
impl crate::Readable for UART_LPDLH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_lpdlh::W](W) writer structure"]
impl crate::Writable for UART_LPDLH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_LPDLH to value 0"]
impl crate::Resettable for UART_LPDLH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
