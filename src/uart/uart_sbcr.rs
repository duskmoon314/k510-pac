#[doc = "Register `UART_SBCR` reader"]
pub struct R(crate::R<UART_SBCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SBCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SBCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_SBCR` writer"]
pub struct W(crate::W<UART_SBCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SBCR_SPEC>;
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
impl From<crate::W<UART_SBCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SBCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shadow Break Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBCR_A {
    #[doc = "0: No spacing on serial output"]
    NO_BREAK = 0,
    #[doc = "1: Serial output forced to the spacing"]
    BREAK = 1,
}
impl From<SBCR_A> for bool {
    #[inline(always)]
    fn from(variant: SBCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBCR` reader - Shadow Break Control"]
pub type SBCR_R = crate::BitReader<SBCR_A>;
impl SBCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBCR_A {
        match self.bits {
            false => SBCR_A::NO_BREAK,
            true => SBCR_A::BREAK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BREAK`"]
    #[inline(always)]
    pub fn is_no_break(&self) -> bool {
        *self == SBCR_A::NO_BREAK
    }
    #[doc = "Checks if the value of the field is `BREAK`"]
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == SBCR_A::BREAK
    }
}
#[doc = "Field `SBCR` writer - Shadow Break Control"]
pub type SBCR_W<'a> = crate::BitWriter<'a, u32, UART_SBCR_SPEC, SBCR_A, 0>;
impl<'a> SBCR_W<'a> {
    #[doc = "No spacing on serial output"]
    #[inline(always)]
    pub fn no_break(self) -> &'a mut W {
        self.variant(SBCR_A::NO_BREAK)
    }
    #[doc = "Serial output forced to the spacing"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut W {
        self.variant(SBCR_A::BREAK)
    }
}
impl R {
    #[doc = "Bit 0 - Shadow Break Control"]
    #[inline(always)]
    pub fn sbcr(&self) -> SBCR_R {
        SBCR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow Break Control"]
    #[inline(always)]
    pub fn sbcr(&mut self) -> SBCR_W {
        SBCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_sbcr](index.html) module"]
pub struct UART_SBCR_SPEC;
impl crate::RegisterSpec for UART_SBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_sbcr::R](R) reader structure"]
impl crate::Readable for UART_SBCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_sbcr::W](W) writer structure"]
impl crate::Writable for UART_SBCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_SBCR to value 0"]
impl crate::Resettable for UART_SBCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
