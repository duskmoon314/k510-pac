#[doc = "Register `SOC_SLEEP_TIM` reader"]
pub struct R(crate::R<SOC_SLEEP_TIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOC_SLEEP_TIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOC_SLEEP_TIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOC_SLEEP_TIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOC_SLEEP_TIM` writer"]
pub struct W(crate::W<SOC_SLEEP_TIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOC_SLEEP_TIM_SPEC>;
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
impl From<crate::W<SOC_SLEEP_TIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOC_SLEEP_TIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `enter_sleep1_time` reader - The Max allowed time SoC can stay in NAP mode. If no wakeup event detected, SoC core will then enter SLEEP1 mode. The time unit here is a cycle of 32KHz clock."]
pub type ENTER_SLEEP1_TIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `enter_sleep1_time` writer - The Max allowed time SoC can stay in NAP mode. If no wakeup event detected, SoC core will then enter SLEEP1 mode. The time unit here is a cycle of 32KHz clock."]
pub type ENTER_SLEEP1_TIME_W<'a> = crate::FieldWriter<'a, u32, SOC_SLEEP_TIM_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - The Max allowed time SoC can stay in NAP mode. If no wakeup event detected, SoC core will then enter SLEEP1 mode. The time unit here is a cycle of 32KHz clock."]
    #[inline(always)]
    pub fn enter_sleep1_time(&self) -> ENTER_SLEEP1_TIME_R {
        ENTER_SLEEP1_TIME_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The Max allowed time SoC can stay in NAP mode. If no wakeup event detected, SoC core will then enter SLEEP1 mode. The time unit here is a cycle of 32KHz clock."]
    #[inline(always)]
    pub fn enter_sleep1_time(&mut self) -> ENTER_SLEEP1_TIME_W {
        ENTER_SLEEP1_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SoC sleep mode timing configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_sleep_tim](index.html) module"]
pub struct SOC_SLEEP_TIM_SPEC;
impl crate::RegisterSpec for SOC_SLEEP_TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soc_sleep_tim::R](R) reader structure"]
impl crate::Readable for SOC_SLEEP_TIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soc_sleep_tim::W](W) writer structure"]
impl crate::Writable for SOC_SLEEP_TIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOC_SLEEP_TIM to value 0x08"]
impl crate::Resettable for SOC_SLEEP_TIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
