#[doc = "Register `UART_SRTS` reader"]
pub struct R(crate::R<UART_SRTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SRTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SRTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SRTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_SRTS` writer"]
pub struct W(crate::W<UART_SRTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SRTS_SPEC>;
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
impl From<crate::W<UART_SRTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SRTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shadow Request to Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRTS_A {
    #[doc = "0: Deasserted"]
    DEASSERTED = 0,
    #[doc = "1: Asserted"]
    ASSERTED = 1,
}
impl From<SRTS_A> for bool {
    #[inline(always)]
    fn from(variant: SRTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRTS` reader - Shadow Request to Send"]
pub type SRTS_R = crate::BitReader<SRTS_A>;
impl SRTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRTS_A {
        match self.bits {
            false => SRTS_A::DEASSERTED,
            true => SRTS_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == SRTS_A::DEASSERTED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SRTS_A::ASSERTED
    }
}
#[doc = "Field `SRTS` writer - Shadow Request to Send"]
pub type SRTS_W<'a> = crate::BitWriter<'a, u32, UART_SRTS_SPEC, SRTS_A, 0>;
impl<'a> SRTS_W<'a> {
    #[doc = "Deasserted"]
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut W {
        self.variant(SRTS_A::DEASSERTED)
    }
    #[doc = "Asserted"]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SRTS_A::ASSERTED)
    }
}
impl R {
    #[doc = "Bit 0 - Shadow Request to Send"]
    #[inline(always)]
    pub fn srts(&self) -> SRTS_R {
        SRTS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow Request to Send"]
    #[inline(always)]
    pub fn srts(&mut self) -> SRTS_W {
        SRTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_srts](index.html) module"]
pub struct UART_SRTS_SPEC;
impl crate::RegisterSpec for UART_SRTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_srts::R](R) reader structure"]
impl crate::Readable for UART_SRTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_srts::W](W) writer structure"]
impl crate::Writable for UART_SRTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_SRTS to value 0"]
impl crate::Resettable for UART_SRTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
