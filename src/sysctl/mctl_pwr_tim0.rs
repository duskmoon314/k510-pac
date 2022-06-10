#[doc = "Register `MCTL_PWR_TIM0` reader"]
pub struct R(crate::R<MCTL_PWR_TIM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTL_PWR_TIM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTL_PWR_TIM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTL_PWR_TIM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTL_PWR_TIM0` writer"]
pub struct W(crate::W<MCTL_PWR_TIM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTL_PWR_TIM0_SPEC>;
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
impl From<crate::W<MCTL_PWR_TIM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTL_PWR_TIM0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Memory contorller power controller timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctl_pwr_tim0](index.html) module"]
pub struct MCTL_PWR_TIM0_SPEC;
impl crate::RegisterSpec for MCTL_PWR_TIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctl_pwr_tim0::R](R) reader structure"]
impl crate::Readable for MCTL_PWR_TIM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctl_pwr_tim0::W](W) writer structure"]
impl crate::Writable for MCTL_PWR_TIM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCTL_PWR_TIM0 to value 0"]
impl crate::Resettable for MCTL_PWR_TIM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
