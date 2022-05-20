#[doc = "Register `UART_SCR` reader"]
pub struct R(crate::R<UART_SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_SCR` writer"]
pub struct W(crate::W<UART_SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SCR_SPEC>;
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
impl From<crate::W<UART_SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCR` reader - Scratch Pad Register"]
pub type SCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCR` writer - Scratch Pad Register"]
pub type SCR_W<'a> = crate::FieldWriter<'a, u32, UART_SCR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Scratch Pad Register"]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Scratch Pad Register"]
    #[inline(always)]
    pub fn scr(&mut self) -> SCR_W {
        SCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scratch Pad Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_scr](index.html) module"]
pub struct UART_SCR_SPEC;
impl crate::RegisterSpec for UART_SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_scr::R](R) reader structure"]
impl crate::Readable for UART_SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_scr::W](W) writer structure"]
impl crate::Writable for UART_SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_SCR to value 0"]
impl crate::Resettable for UART_SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
