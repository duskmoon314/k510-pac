#[doc = "Register `CPU_WAKUP_TIM` reader"]
pub struct R(crate::R<CPU_WAKUP_TIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_WAKUP_TIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_WAKUP_TIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_WAKUP_TIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_WAKUP_TIM` writer"]
pub struct W(crate::W<CPU_WAKUP_TIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_WAKUP_TIM_SPEC>;
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
impl From<crate::W<CPU_WAKUP_TIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_WAKUP_TIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpu_wakeup_time` reader - The Max allowed time SoC can stay in WAKECPU mode. Then SoC core will enter Mission mode. The time unit here is a cycle of 32KHz clock."]
pub type CPU_WAKEUP_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cpu_wakeup_time` writer - The Max allowed time SoC can stay in WAKECPU mode. Then SoC core will enter Mission mode. The time unit here is a cycle of 32KHz clock."]
pub type CPU_WAKEUP_TIME_W<'a> = crate::FieldWriter<'a, u32, CPU_WAKUP_TIM_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - The Max allowed time SoC can stay in WAKECPU mode. Then SoC core will enter Mission mode. The time unit here is a cycle of 32KHz clock."]
    #[inline(always)]
    pub fn cpu_wakeup_time(&self) -> CPU_WAKEUP_TIME_R {
        CPU_WAKEUP_TIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The Max allowed time SoC can stay in WAKECPU mode. Then SoC core will enter Mission mode. The time unit here is a cycle of 32KHz clock."]
    #[inline(always)]
    pub fn cpu_wakeup_time(&mut self) -> CPU_WAKEUP_TIME_W {
        CPU_WAKEUP_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU wake-up timing configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_wakup_tim](index.html) module"]
pub struct CPU_WAKUP_TIM_SPEC;
impl crate::RegisterSpec for CPU_WAKUP_TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_wakup_tim::R](R) reader structure"]
impl crate::Readable for CPU_WAKUP_TIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_wakup_tim::W](W) writer structure"]
impl crate::Writable for CPU_WAKUP_TIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_WAKUP_TIM to value 0x02"]
impl crate::Resettable for CPU_WAKUP_TIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
