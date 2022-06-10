#[doc = "Register `RESET_STATUS` reader"]
pub struct R(crate::R<RESET_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_STATUS` writer"]
pub struct W(crate::W<RESET_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_STATUS_SPEC>;
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
impl From<crate::W<RESET_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_clear_reset_status` writer - Write enable for bit 0 (clear_reset_status)"]
pub type WE_CLEAR_RESET_STATUS_W<'a> = crate::BitWriter<'a, u32, RESET_STATUS_SPEC, bool, 16>;
#[doc = "offchip reset status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFCHIP_RESET_STATUS_A {
    #[doc = "0: offchip reset doen't occur"]
    NOT_RESET = 0,
    #[doc = "1: offchip reset occurs"]
    RESET = 1,
}
impl From<OFFCHIP_RESET_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: OFFCHIP_RESET_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `offchip_reset_status` reader - offchip reset status"]
pub type OFFCHIP_RESET_STATUS_R = crate::BitReader<OFFCHIP_RESET_STATUS_A>;
impl OFFCHIP_RESET_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFCHIP_RESET_STATUS_A {
        match self.bits {
            false => OFFCHIP_RESET_STATUS_A::NOT_RESET,
            true => OFFCHIP_RESET_STATUS_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RESET`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == OFFCHIP_RESET_STATUS_A::NOT_RESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OFFCHIP_RESET_STATUS_A::RESET
    }
}
#[doc = "WDT1 reset status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT1_RESET_STATUS_A {
    #[doc = "0: WDT1 reset doen't occur"]
    NOT_RESET = 0,
    #[doc = "1: WDT1 reset occurs"]
    RESET = 1,
}
impl From<WDT1_RESET_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: WDT1_RESET_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `wdt1_reset_status` reader - WDT1 reset status"]
pub type WDT1_RESET_STATUS_R = crate::BitReader<WDT1_RESET_STATUS_A>;
impl WDT1_RESET_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT1_RESET_STATUS_A {
        match self.bits {
            false => WDT1_RESET_STATUS_A::NOT_RESET,
            true => WDT1_RESET_STATUS_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RESET`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == WDT1_RESET_STATUS_A::NOT_RESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WDT1_RESET_STATUS_A::RESET
    }
}
#[doc = "WDT0 reset status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT0_RESET_STATUS_A {
    #[doc = "0: WDT0 reset doen't occur"]
    NOT_RESET = 0,
    #[doc = "1: WDT0 reset occurs"]
    RESET = 1,
}
impl From<WDT0_RESET_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: WDT0_RESET_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `wdt0_reset_status` reader - WDT0 reset status"]
pub type WDT0_RESET_STATUS_R = crate::BitReader<WDT0_RESET_STATUS_A>;
impl WDT0_RESET_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT0_RESET_STATUS_A {
        match self.bits {
            false => WDT0_RESET_STATUS_A::NOT_RESET,
            true => WDT0_RESET_STATUS_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RESET`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == WDT0_RESET_STATUS_A::NOT_RESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WDT0_RESET_STATUS_A::RESET
    }
}
#[doc = "Soft reset status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFT_RESET_STATUS_A {
    #[doc = "0: Soft reset doen't occur"]
    NOT_RESET = 0,
    #[doc = "1: Soft reset occurs"]
    RESET = 1,
}
impl From<SOFT_RESET_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: SOFT_RESET_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soft_reset_status` reader - Soft reset status"]
pub type SOFT_RESET_STATUS_R = crate::BitReader<SOFT_RESET_STATUS_A>;
impl SOFT_RESET_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFT_RESET_STATUS_A {
        match self.bits {
            false => SOFT_RESET_STATUS_A::NOT_RESET,
            true => SOFT_RESET_STATUS_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RESET`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == SOFT_RESET_STATUS_A::NOT_RESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SOFT_RESET_STATUS_A::RESET
    }
}
#[doc = "Clear reset status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEAR_RESET_STATUS_AW {
    #[doc = "0: No operation"]
    NO_OPERATION = 0,
    #[doc = "1: Clear all reset status"]
    CLEAR = 1,
}
impl From<CLEAR_RESET_STATUS_AW> for bool {
    #[inline(always)]
    fn from(variant: CLEAR_RESET_STATUS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `clear_reset_status` writer - Clear reset status"]
pub type CLEAR_RESET_STATUS_W<'a> =
    crate::BitWriter<'a, u32, RESET_STATUS_SPEC, CLEAR_RESET_STATUS_AW, 0>;
impl<'a> CLEAR_RESET_STATUS_W<'a> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(CLEAR_RESET_STATUS_AW::NO_OPERATION)
    }
    #[doc = "Clear all reset status"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLEAR_RESET_STATUS_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 5 - offchip reset status"]
    #[inline(always)]
    pub fn offchip_reset_status(&self) -> OFFCHIP_RESET_STATUS_R {
        OFFCHIP_RESET_STATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 3 - WDT1 reset status"]
    #[inline(always)]
    pub fn wdt1_reset_status(&self) -> WDT1_RESET_STATUS_R {
        WDT1_RESET_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - WDT0 reset status"]
    #[inline(always)]
    pub fn wdt0_reset_status(&self) -> WDT0_RESET_STATUS_R {
        WDT0_RESET_STATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Soft reset status"]
    #[inline(always)]
    pub fn soft_reset_status(&self) -> SOFT_RESET_STATUS_R {
        SOFT_RESET_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Write enable for bit 0 (clear_reset_status)"]
    #[inline(always)]
    pub fn we_clear_reset_status(&mut self) -> WE_CLEAR_RESET_STATUS_W {
        WE_CLEAR_RESET_STATUS_W::new(self)
    }
    #[doc = "Bit 0 - Clear reset status"]
    #[inline(always)]
    pub fn clear_reset_status(&mut self) -> CLEAR_RESET_STATUS_W {
        CLEAR_RESET_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_status](index.html) module"]
pub struct RESET_STATUS_SPEC;
impl crate::RegisterSpec for RESET_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_status::R](R) reader structure"]
impl crate::Readable for RESET_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_status::W](W) writer structure"]
impl crate::Writable for RESET_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESET_STATUS to value 0x20"]
impl crate::Resettable for RESET_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
