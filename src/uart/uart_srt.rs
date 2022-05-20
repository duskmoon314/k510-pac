#[doc = "Register `UART_SRT` reader"]
pub struct R(crate::R<UART_SRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_SRT` writer"]
pub struct W(crate::W<UART_SRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SRT_SPEC>;
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
impl From<crate::W<UART_SRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shadow RCVR Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRT_A {
    #[doc = "0: 1 character in FIFO"]
    FIFO_CHAR_1 = 0,
    #[doc = "1: FIFO 1/4 full"]
    FIFO_QUARTER_FULL = 1,
    #[doc = "2: FIFO 1/2 full"]
    FIFO_HALF_FULL = 2,
    #[doc = "3: FIFO 2 less than full"]
    FIFO_FULL_2 = 3,
}
impl From<SRT_A> for u8 {
    #[inline(always)]
    fn from(variant: SRT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRT` reader - Shadow RCVR Trigger"]
pub type SRT_R = crate::FieldReader<u8, SRT_A>;
impl SRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRT_A {
        match self.bits {
            0 => SRT_A::FIFO_CHAR_1,
            1 => SRT_A::FIFO_QUARTER_FULL,
            2 => SRT_A::FIFO_HALF_FULL,
            3 => SRT_A::FIFO_FULL_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_CHAR_1`"]
    #[inline(always)]
    pub fn is_fifo_char_1(&self) -> bool {
        *self == SRT_A::FIFO_CHAR_1
    }
    #[doc = "Checks if the value of the field is `FIFO_QUARTER_FULL`"]
    #[inline(always)]
    pub fn is_fifo_quarter_full(&self) -> bool {
        *self == SRT_A::FIFO_QUARTER_FULL
    }
    #[doc = "Checks if the value of the field is `FIFO_HALF_FULL`"]
    #[inline(always)]
    pub fn is_fifo_half_full(&self) -> bool {
        *self == SRT_A::FIFO_HALF_FULL
    }
    #[doc = "Checks if the value of the field is `FIFO_FULL_2`"]
    #[inline(always)]
    pub fn is_fifo_full_2(&self) -> bool {
        *self == SRT_A::FIFO_FULL_2
    }
}
#[doc = "Field `SRT` writer - Shadow RCVR Trigger"]
pub type SRT_W<'a> = crate::FieldWriterSafe<'a, u32, UART_SRT_SPEC, u8, SRT_A, 2, 0>;
impl<'a> SRT_W<'a> {
    #[doc = "1 character in FIFO"]
    #[inline(always)]
    pub fn fifo_char_1(self) -> &'a mut W {
        self.variant(SRT_A::FIFO_CHAR_1)
    }
    #[doc = "FIFO 1/4 full"]
    #[inline(always)]
    pub fn fifo_quarter_full(self) -> &'a mut W {
        self.variant(SRT_A::FIFO_QUARTER_FULL)
    }
    #[doc = "FIFO 1/2 full"]
    #[inline(always)]
    pub fn fifo_half_full(self) -> &'a mut W {
        self.variant(SRT_A::FIFO_HALF_FULL)
    }
    #[doc = "FIFO 2 less than full"]
    #[inline(always)]
    pub fn fifo_full_2(self) -> &'a mut W {
        self.variant(SRT_A::FIFO_FULL_2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Shadow RCVR Trigger"]
    #[inline(always)]
    pub fn srt(&self) -> SRT_R {
        SRT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Shadow RCVR Trigger"]
    #[inline(always)]
    pub fn srt(&mut self) -> SRT_W {
        SRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow Receive Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_srt](index.html) module"]
pub struct UART_SRT_SPEC;
impl crate::RegisterSpec for UART_SRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_srt::R](R) reader structure"]
impl crate::Readable for UART_SRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_srt::W](W) writer structure"]
impl crate::Writable for UART_SRT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_SRT to value 0"]
impl crate::Resettable for UART_SRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
