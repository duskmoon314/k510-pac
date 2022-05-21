#[doc = "Register `PLL%s_STAT` reader"]
pub struct R(crate::R<PLL_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "PLL\\[i\\]
controlling FSM current status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL_FSM_STAT_A {
    #[doc = "0: PLL\\[i\\]
is in POWER_DOWN mode"]
    POWER_DOWN = 0,
    #[doc = "1: PLL\\[i\\]
is in RESET_PLL mode"]
    RESET_PLL = 1,
    #[doc = "2: PLL\\[i\\]
is in WAIT_LOCK mode"]
    WAIT_LOCK = 2,
    #[doc = "3: PLL\\[i\\]
is in PLL_READY mode"]
    PLL_READY = 3,
}
impl From<PLL_FSM_STAT_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_FSM_STAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pll_fsm_stat` reader - PLL\\[i\\]
controlling FSM current status"]
pub type PLL_FSM_STAT_R = crate::FieldReader<u8, PLL_FSM_STAT_A>;
impl PLL_FSM_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_FSM_STAT_A {
        match self.bits {
            0 => PLL_FSM_STAT_A::POWER_DOWN,
            1 => PLL_FSM_STAT_A::RESET_PLL,
            2 => PLL_FSM_STAT_A::WAIT_LOCK,
            3 => PLL_FSM_STAT_A::PLL_READY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == PLL_FSM_STAT_A::POWER_DOWN
    }
    #[doc = "Checks if the value of the field is `RESET_PLL`"]
    #[inline(always)]
    pub fn is_reset_pll(&self) -> bool {
        *self == PLL_FSM_STAT_A::RESET_PLL
    }
    #[doc = "Checks if the value of the field is `WAIT_LOCK`"]
    #[inline(always)]
    pub fn is_wait_lock(&self) -> bool {
        *self == PLL_FSM_STAT_A::WAIT_LOCK
    }
    #[doc = "Checks if the value of the field is `PLL_READY`"]
    #[inline(always)]
    pub fn is_pll_ready(&self) -> bool {
        *self == PLL_FSM_STAT_A::PLL_READY
    }
}
#[doc = "This is the feedback clock path frequency/phase slip status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_FB_SLIP_A {
    #[doc = "0: PLL working OK"]
    OK = 0,
    #[doc = "1: There are frequency/phase slip on feedback clock"]
    SLIP = 1,
}
impl From<PLL_FB_SLIP_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_FB_SLIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll_fb_slip` reader - This is the feedback clock path frequency/phase slip status"]
pub type PLL_FB_SLIP_R = crate::BitReader<PLL_FB_SLIP_A>;
impl PLL_FB_SLIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_FB_SLIP_A {
        match self.bits {
            false => PLL_FB_SLIP_A::OK,
            true => PLL_FB_SLIP_A::SLIP,
        }
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == PLL_FB_SLIP_A::OK
    }
    #[doc = "Checks if the value of the field is `SLIP`"]
    #[inline(always)]
    pub fn is_slip(&self) -> bool {
        *self == PLL_FB_SLIP_A::SLIP
    }
}
#[doc = "This is the reference clock path frequency/phase slip status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_REF_SLIP_A {
    #[doc = "0: PLL working OK"]
    OK = 0,
    #[doc = "1: There are frequency/phase slip on reference clock"]
    SLIP = 1,
}
impl From<PLL_REF_SLIP_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_REF_SLIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll_ref_slip` reader - This is the reference clock path frequency/phase slip status"]
pub type PLL_REF_SLIP_R = crate::BitReader<PLL_REF_SLIP_A>;
impl PLL_REF_SLIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_REF_SLIP_A {
        match self.bits {
            false => PLL_REF_SLIP_A::OK,
            true => PLL_REF_SLIP_A::SLIP,
        }
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == PLL_REF_SLIP_A::OK
    }
    #[doc = "Checks if the value of the field is `SLIP`"]
    #[inline(always)]
    pub fn is_slip(&self) -> bool {
        *self == PLL_REF_SLIP_A::SLIP
    }
}
#[doc = "PLL\\[i\\]
current lock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_LOCK_A {
    #[doc = "0: PLL\\[i\\]
not in lock state"]
    NOT_LOCKED = 0,
    #[doc = "1: PLL\\[i\\]
in lock state"]
    LOCKED = 1,
}
impl From<PLL_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll_lock` reader - PLL\\[i\\]
current lock status"]
pub type PLL_LOCK_R = crate::BitReader<PLL_LOCK_A>;
impl PLL_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_LOCK_A {
        match self.bits {
            false => PLL_LOCK_A::NOT_LOCKED,
            true => PLL_LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_LOCKED`"]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        *self == PLL_LOCK_A::NOT_LOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLL_LOCK_A::LOCKED
    }
}
impl R {
    #[doc = "Bits 4:5 - PLL\\[i\\]
controlling FSM current status"]
    #[inline(always)]
    pub fn pll_fsm_stat(&self) -> PLL_FSM_STAT_R {
        PLL_FSM_STAT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 2 - This is the feedback clock path frequency/phase slip status"]
    #[inline(always)]
    pub fn pll_fb_slip(&self) -> PLL_FB_SLIP_R {
        PLL_FB_SLIP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - This is the reference clock path frequency/phase slip status"]
    #[inline(always)]
    pub fn pll_ref_slip(&self) -> PLL_REF_SLIP_R {
        PLL_REF_SLIP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - PLL\\[i\\]
current lock status"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PLL_LOCK_R {
        PLL_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[doc = "PLL\\[i\\]
status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_stat](index.html) module"]
pub struct PLL_STAT_SPEC;
impl crate::RegisterSpec for PLL_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_stat::R](R) reader structure"]
impl crate::Readable for PLL_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PLL%s_STAT to value 0"]
impl crate::Resettable for PLL_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
