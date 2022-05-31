#[doc = "Register `SOC_RESET_TIM` reader"]
pub struct R(crate::R<SOC_RESET_TIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOC_RESET_TIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOC_RESET_TIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOC_RESET_TIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOC_RESET_TIM` writer"]
pub struct W(crate::W<SOC_RESET_TIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOC_RESET_TIM_SPEC>;
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
impl From<crate::W<SOC_RESET_TIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOC_RESET_TIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reset1_time` reader - The Max allowed time SoC after soc reset assert, then the system will enter Mission state."]
pub type RESET1_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reset1_time` writer - The Max allowed time SoC after soc reset assert, then the system will enter Mission state."]
pub type RESET1_TIME_W<'a> = crate::FieldWriter<'a, u32, SOC_RESET_TIM_SPEC, u16, u16, 16, 16>;
#[doc = "Field `reset0_time` reader - The Max allowed time SoC between offchip reset and global reset assert. The time unit here is a cycle of 32KHz clock."]
pub type RESET0_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reset0_time` writer - The Max allowed time SoC between offchip reset and global reset assert. The time unit here is a cycle of 32KHz clock."]
pub type RESET0_TIME_W<'a> = crate::FieldWriter<'a, u32, SOC_RESET_TIM_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 16:31 - The Max allowed time SoC after soc reset assert, then the system will enter Mission state."]
    #[inline(always)]
    pub fn reset1_time(&self) -> RESET1_TIME_R {
        RESET1_TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - The Max allowed time SoC between offchip reset and global reset assert. The time unit here is a cycle of 32KHz clock."]
    #[inline(always)]
    pub fn reset0_time(&self) -> RESET0_TIME_R {
        RESET0_TIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - The Max allowed time SoC after soc reset assert, then the system will enter Mission state."]
    #[inline(always)]
    pub fn reset1_time(&mut self) -> RESET1_TIME_W {
        RESET1_TIME_W::new(self)
    }
    #[doc = "Bits 0:15 - The Max allowed time SoC between offchip reset and global reset assert. The time unit here is a cycle of 32KHz clock."]
    #[inline(always)]
    pub fn reset0_time(&mut self) -> RESET0_TIME_W {
        RESET0_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SoC reset timing configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_reset_tim](index.html) module"]
pub struct SOC_RESET_TIM_SPEC;
impl crate::RegisterSpec for SOC_RESET_TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soc_reset_tim::R](R) reader structure"]
impl crate::Readable for SOC_RESET_TIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soc_reset_tim::W](W) writer structure"]
impl crate::Writable for SOC_RESET_TIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOC_RESET_TIM to value 0x0006_0010"]
impl crate::Resettable for SOC_RESET_TIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0006_0010
    }
}
