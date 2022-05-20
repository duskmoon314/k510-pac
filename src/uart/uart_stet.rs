#[doc = "Register `UART_STET` reader"]
pub struct R(crate::R<UART_STET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_STET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_STET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_STET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_STET` writer"]
pub struct W(crate::W<UART_STET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_STET_SPEC>;
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
impl From<crate::W<UART_STET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_STET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shadow TX Empty Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STET_A {
    #[doc = "0: FIFO empty"]
    FIFO_EMPTY = 0,
    #[doc = "1: 2 characters in FIFO"]
    FIFO_CHAR_2 = 1,
    #[doc = "2: FIFO 1/4 full"]
    FIFO_QUARTER_FULL = 2,
    #[doc = "3: FIFO 1/2 full"]
    FIFO_HALF_FULL = 3,
}
impl From<STET_A> for u8 {
    #[inline(always)]
    fn from(variant: STET_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STET` reader - Shadow TX Empty Trigger"]
pub type STET_R = crate::FieldReader<u8, STET_A>;
impl STET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STET_A {
        match self.bits {
            0 => STET_A::FIFO_EMPTY,
            1 => STET_A::FIFO_CHAR_2,
            2 => STET_A::FIFO_QUARTER_FULL,
            3 => STET_A::FIFO_HALF_FULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_EMPTY`"]
    #[inline(always)]
    pub fn is_fifo_empty(&self) -> bool {
        *self == STET_A::FIFO_EMPTY
    }
    #[doc = "Checks if the value of the field is `FIFO_CHAR_2`"]
    #[inline(always)]
    pub fn is_fifo_char_2(&self) -> bool {
        *self == STET_A::FIFO_CHAR_2
    }
    #[doc = "Checks if the value of the field is `FIFO_QUARTER_FULL`"]
    #[inline(always)]
    pub fn is_fifo_quarter_full(&self) -> bool {
        *self == STET_A::FIFO_QUARTER_FULL
    }
    #[doc = "Checks if the value of the field is `FIFO_HALF_FULL`"]
    #[inline(always)]
    pub fn is_fifo_half_full(&self) -> bool {
        *self == STET_A::FIFO_HALF_FULL
    }
}
#[doc = "Field `STET` writer - Shadow TX Empty Trigger"]
pub type STET_W<'a> = crate::FieldWriterSafe<'a, u32, UART_STET_SPEC, u8, STET_A, 2, 0>;
impl<'a> STET_W<'a> {
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn fifo_empty(self) -> &'a mut W {
        self.variant(STET_A::FIFO_EMPTY)
    }
    #[doc = "2 characters in FIFO"]
    #[inline(always)]
    pub fn fifo_char_2(self) -> &'a mut W {
        self.variant(STET_A::FIFO_CHAR_2)
    }
    #[doc = "FIFO 1/4 full"]
    #[inline(always)]
    pub fn fifo_quarter_full(self) -> &'a mut W {
        self.variant(STET_A::FIFO_QUARTER_FULL)
    }
    #[doc = "FIFO 1/2 full"]
    #[inline(always)]
    pub fn fifo_half_full(self) -> &'a mut W {
        self.variant(STET_A::FIFO_HALF_FULL)
    }
}
impl R {
    #[doc = "Bits 0:1 - Shadow TX Empty Trigger"]
    #[inline(always)]
    pub fn stet(&self) -> STET_R {
        STET_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Shadow TX Empty Trigger"]
    #[inline(always)]
    pub fn stet(&mut self) -> STET_W {
        STET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow Transmit Empty Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_stet](index.html) module"]
pub struct UART_STET_SPEC;
impl crate::RegisterSpec for UART_STET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_stet::R](R) reader structure"]
impl crate::Readable for UART_STET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_stet::W](W) writer structure"]
impl crate::Writable for UART_STET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_STET to value 0"]
impl crate::Resettable for UART_STET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
