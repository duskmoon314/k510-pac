#[doc = "Register `UART_SDMAM` reader"]
pub struct R(crate::R<UART_SDMAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SDMAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SDMAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SDMAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_SDMAM` writer"]
pub struct W(crate::W<UART_SDMAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SDMAM_SPEC>;
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
impl From<crate::W<UART_SDMAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SDMAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shadow DMA Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMAM_A {
    #[doc = "0: Mode 0"]
    MODE0 = 0,
    #[doc = "1: Mode 1"]
    MODE1 = 1,
}
impl From<SDMAM_A> for bool {
    #[inline(always)]
    fn from(variant: SDMAM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMAM` reader - Shadow DMA Mode"]
pub type SDMAM_R = crate::BitReader<SDMAM_A>;
impl SDMAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMAM_A {
        match self.bits {
            false => SDMAM_A::MODE0,
            true => SDMAM_A::MODE1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == SDMAM_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == SDMAM_A::MODE1
    }
}
#[doc = "Field `SDMAM` writer - Shadow DMA Mode"]
pub type SDMAM_W<'a> = crate::BitWriter<'a, u32, UART_SDMAM_SPEC, SDMAM_A, 0>;
impl<'a> SDMAM_W<'a> {
    #[doc = "Mode 0"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(SDMAM_A::MODE0)
    }
    #[doc = "Mode 1"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(SDMAM_A::MODE1)
    }
}
impl R {
    #[doc = "Bit 0 - Shadow DMA Mode"]
    #[inline(always)]
    pub fn sdmam(&self) -> SDMAM_R {
        SDMAM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow DMA Mode"]
    #[inline(always)]
    pub fn sdmam(&mut self) -> SDMAM_W {
        SDMAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow DMA Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_sdmam](index.html) module"]
pub struct UART_SDMAM_SPEC;
impl crate::RegisterSpec for UART_SDMAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_sdmam::R](R) reader structure"]
impl crate::Readable for UART_SDMAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_sdmam::W](W) writer structure"]
impl crate::Writable for UART_SDMAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_SDMAM to value 0"]
impl crate::Resettable for UART_SDMAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
