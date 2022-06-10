#[doc = "Register `AUDIF_RST_TIM` reader"]
pub struct R(crate::R<AUDIF_RST_TIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIF_RST_TIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIF_RST_TIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIF_RST_TIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUDIF_RST_TIM` writer"]
pub struct W(crate::W<AUDIF_RST_TIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDIF_RST_TIM_SPEC>;
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
impl From<crate::W<AUDIF_RST_TIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDIF_RST_TIM_SPEC>) -> Self {
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
#[doc = "Audio Interface module reset timing control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audif_rst_tim](index.html) module"]
pub struct AUDIF_RST_TIM_SPEC;
impl crate::RegisterSpec for AUDIF_RST_TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audif_rst_tim::R](R) reader structure"]
impl crate::Readable for AUDIF_RST_TIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audif_rst_tim::W](W) writer structure"]
impl crate::Writable for AUDIF_RST_TIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUDIF_RST_TIM to value 0"]
impl crate::Resettable for AUDIF_RST_TIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
