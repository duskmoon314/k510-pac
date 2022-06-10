#[doc = "Register `MCTL_LPI_TIM` reader"]
pub struct R(crate::R<MCTL_LPI_TIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTL_LPI_TIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTL_LPI_TIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTL_LPI_TIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTL_LPI_TIM` writer"]
pub struct W(crate::W<MCTL_LPI_TIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTL_LPI_TIM_SPEC>;
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
impl From<crate::W<MCTL_LPI_TIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTL_LPI_TIM_SPEC>) -> Self {
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
#[doc = "Memory Controller domain NOC power controller LPI timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctl_lpi_tim](index.html) module"]
pub struct MCTL_LPI_TIM_SPEC;
impl crate::RegisterSpec for MCTL_LPI_TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctl_lpi_tim::R](R) reader structure"]
impl crate::Readable for MCTL_LPI_TIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctl_lpi_tim::W](W) writer structure"]
impl crate::Writable for MCTL_LPI_TIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCTL_LPI_TIM to value 0"]
impl crate::Resettable for MCTL_LPI_TIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}