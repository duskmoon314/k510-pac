#[doc = "Register `SOC_WAKUP_SRC` reader"]
pub struct R(crate::R<SOC_WAKUP_SRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOC_WAKUP_SRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOC_WAKUP_SRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOC_WAKUP_SRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "The cause of SoC waking up from DEEPSLEEP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSLP_WAKUP_CAUSE_A {
    #[doc = "1: SoC is woken up by RTC generate event"]
    RTC = 1,
    #[doc = "4: SoC is woken up by TIMER generate event"]
    TIMER = 4,
    #[doc = "8: SoC is woken up by GPIO key pressing event"]
    GPIO = 8,
}
impl From<DSLP_WAKUP_CAUSE_A> for u8 {
    #[inline(always)]
    fn from(variant: DSLP_WAKUP_CAUSE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `dslp_wakup_cause` reader - The cause of SoC waking up from DEEPSLEEP mode."]
pub type DSLP_WAKUP_CAUSE_R = crate::FieldReader<u8, DSLP_WAKUP_CAUSE_A>;
impl DSLP_WAKUP_CAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSLP_WAKUP_CAUSE_A> {
        match self.bits {
            1 => Some(DSLP_WAKUP_CAUSE_A::RTC),
            4 => Some(DSLP_WAKUP_CAUSE_A::TIMER),
            8 => Some(DSLP_WAKUP_CAUSE_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == DSLP_WAKUP_CAUSE_A::RTC
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == DSLP_WAKUP_CAUSE_A::TIMER
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == DSLP_WAKUP_CAUSE_A::GPIO
    }
}
#[doc = "The cause of SoC waking up from SLEEP1 mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLP1_WAKUP_CAUSE_A {
    #[doc = "1: SoC is woken up by RTC generate event"]
    RTC = 1,
    #[doc = "2: SoC is woken up by VAD generate event"]
    VAD = 2,
    #[doc = "4: SoC is woken up by TIMER generate event"]
    TIMER = 4,
    #[doc = "8: SoC is woken up by GPIO key pressing event"]
    GPIO = 8,
}
impl From<SLP1_WAKUP_CAUSE_A> for u8 {
    #[inline(always)]
    fn from(variant: SLP1_WAKUP_CAUSE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `slp1_wakup_cause` reader - The cause of SoC waking up from SLEEP1 mode."]
pub type SLP1_WAKUP_CAUSE_R = crate::FieldReader<u8, SLP1_WAKUP_CAUSE_A>;
impl SLP1_WAKUP_CAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLP1_WAKUP_CAUSE_A> {
        match self.bits {
            1 => Some(SLP1_WAKUP_CAUSE_A::RTC),
            2 => Some(SLP1_WAKUP_CAUSE_A::VAD),
            4 => Some(SLP1_WAKUP_CAUSE_A::TIMER),
            8 => Some(SLP1_WAKUP_CAUSE_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == SLP1_WAKUP_CAUSE_A::RTC
    }
    #[doc = "Checks if the value of the field is `VAD`"]
    #[inline(always)]
    pub fn is_vad(&self) -> bool {
        *self == SLP1_WAKUP_CAUSE_A::VAD
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == SLP1_WAKUP_CAUSE_A::TIMER
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == SLP1_WAKUP_CAUSE_A::GPIO
    }
}
#[doc = "The cause of SoC waking up from SLEEP0 mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLP0_WAKUP_CAUSE_A {
    #[doc = "1: SoC is woken up by RTC generate event"]
    RTC = 1,
    #[doc = "2: SoC is woken up by VAD generate event"]
    VAD = 2,
    #[doc = "4: SoC is woken up by TIMER generate event"]
    TIMER = 4,
    #[doc = "8: SoC is woken up by GPIO key pressing event"]
    GPIO = 8,
}
impl From<SLP0_WAKUP_CAUSE_A> for u8 {
    #[inline(always)]
    fn from(variant: SLP0_WAKUP_CAUSE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `slp0_wakup_cause` reader - The cause of SoC waking up from SLEEP0 mode."]
pub type SLP0_WAKUP_CAUSE_R = crate::FieldReader<u8, SLP0_WAKUP_CAUSE_A>;
impl SLP0_WAKUP_CAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLP0_WAKUP_CAUSE_A> {
        match self.bits {
            1 => Some(SLP0_WAKUP_CAUSE_A::RTC),
            2 => Some(SLP0_WAKUP_CAUSE_A::VAD),
            4 => Some(SLP0_WAKUP_CAUSE_A::TIMER),
            8 => Some(SLP0_WAKUP_CAUSE_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == SLP0_WAKUP_CAUSE_A::RTC
    }
    #[doc = "Checks if the value of the field is `VAD`"]
    #[inline(always)]
    pub fn is_vad(&self) -> bool {
        *self == SLP0_WAKUP_CAUSE_A::VAD
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == SLP0_WAKUP_CAUSE_A::TIMER
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == SLP0_WAKUP_CAUSE_A::GPIO
    }
}
impl R {
    #[doc = "Bits 8:11 - The cause of SoC waking up from DEEPSLEEP mode."]
    #[inline(always)]
    pub fn dslp_wakup_cause(&self) -> DSLP_WAKUP_CAUSE_R {
        DSLP_WAKUP_CAUSE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The cause of SoC waking up from SLEEP1 mode."]
    #[inline(always)]
    pub fn slp1_wakup_cause(&self) -> SLP1_WAKUP_CAUSE_R {
        SLP1_WAKUP_CAUSE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - The cause of SoC waking up from SLEEP0 mode."]
    #[inline(always)]
    pub fn slp0_wakup_cause(&self) -> SLP0_WAKUP_CAUSE_R {
        SLP0_WAKUP_CAUSE_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "SoC wake-up cause status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_wakup_src](index.html) module"]
pub struct SOC_WAKUP_SRC_SPEC;
impl crate::RegisterSpec for SOC_WAKUP_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soc_wakup_src::R](R) reader structure"]
impl crate::Readable for SOC_WAKUP_SRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SOC_WAKUP_SRC to value 0"]
impl crate::Resettable for SOC_WAKUP_SRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
