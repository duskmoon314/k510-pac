#[doc = "Register `FEATURE_ENABLE` reader"]
pub struct R(crate::R<FEATURE_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEATURE_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEATURE_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEATURE_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEATURE_ENABLE` writer"]
pub struct W(crate::W<FEATURE_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEATURE_ENABLE_SPEC>;
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
impl From<crate::W<FEATURE_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEATURE_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Preemptive Priority Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_A {
    #[doc = "0: Preemptive priority interrupt feature is Disabled"]
    DISABLED = 0,
    #[doc = "1: Preemptive priority interrupt feature is Enabled"]
    ENABLED = 1,
}
impl From<PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE` reader - Preemptive Priority Interrupt Enable"]
pub type PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_R =
    crate::BitReader<PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_A>;
impl PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_A {
        match self.bits {
            false => PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_A::DISABLED,
            true => PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_A::ENABLED
    }
}
#[doc = "Field `PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE` writer - Preemptive Priority Interrupt Enable"]
pub type PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_W<'a> =
    crate::BitWriter<'a, u32, FEATURE_ENABLE_SPEC, PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_A, 1>;
impl<'a> PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_W<'a> {
    #[doc = "Preemptive priority interrupt feature is Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_A::DISABLED)
    }
    #[doc = "Preemptive priority interrupt feature is Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_A::ENABLED)
    }
}
#[doc = "Enable Vectored Mode for PLIC\n\nAX25MP not support vector mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTORED_MODE_ENABLE_A {
    #[doc = "0: Vectored mode is Disabled"]
    DISABLED = 0,
    #[doc = "1: Vectored mode is Enabled"]
    ENABLED = 1,
}
impl From<VECTORED_MODE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: VECTORED_MODE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VECTORED_MODE_ENABLE` reader - Enable Vectored Mode for PLIC\n\nAX25MP not support vector mode"]
pub type VECTORED_MODE_ENABLE_R = crate::BitReader<VECTORED_MODE_ENABLE_A>;
impl VECTORED_MODE_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VECTORED_MODE_ENABLE_A {
        match self.bits {
            false => VECTORED_MODE_ENABLE_A::DISABLED,
            true => VECTORED_MODE_ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VECTORED_MODE_ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VECTORED_MODE_ENABLE_A::ENABLED
    }
}
#[doc = "Field `VECTORED_MODE_ENABLE` writer - Enable Vectored Mode for PLIC\n\nAX25MP not support vector mode"]
pub type VECTORED_MODE_ENABLE_W<'a> =
    crate::BitWriter<'a, u32, FEATURE_ENABLE_SPEC, VECTORED_MODE_ENABLE_A, 0>;
impl<'a> VECTORED_MODE_ENABLE_W<'a> {
    #[doc = "Vectored mode is Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VECTORED_MODE_ENABLE_A::DISABLED)
    }
    #[doc = "Vectored mode is Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VECTORED_MODE_ENABLE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 1 - Preemptive Priority Interrupt Enable"]
    #[inline(always)]
    pub fn preemptive_priority_interrupt_enable(&self) -> PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_R {
        PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable Vectored Mode for PLIC\n\nAX25MP not support vector mode"]
    #[inline(always)]
    pub fn vectored_mode_enable(&self) -> VECTORED_MODE_ENABLE_R {
        VECTORED_MODE_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Preemptive Priority Interrupt Enable"]
    #[inline(always)]
    pub fn preemptive_priority_interrupt_enable(
        &mut self,
    ) -> PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_W {
        PREEMPTIVE_PRIORITY_INTERRUPT_ENABLE_W::new(self)
    }
    #[doc = "Bit 0 - Enable Vectored Mode for PLIC\n\nAX25MP not support vector mode"]
    #[inline(always)]
    pub fn vectored_mode_enable(&mut self) -> VECTORED_MODE_ENABLE_W {
        VECTORED_MODE_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Feature Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feature_enable](index.html) module"]
pub struct FEATURE_ENABLE_SPEC;
impl crate::RegisterSpec for FEATURE_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [feature_enable::R](R) reader structure"]
impl crate::Readable for FEATURE_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [feature_enable::W](W) writer structure"]
impl crate::Writable for FEATURE_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FEATURE_ENABLE to value 0"]
impl crate::Resettable for FEATURE_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
