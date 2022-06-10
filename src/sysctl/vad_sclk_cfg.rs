#[doc = "Register `VAD_SCLK_CFG` reader"]
pub struct R(crate::R<VAD_SCLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VAD_SCLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VAD_SCLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VAD_SCLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VAD_SCLK_CFG` writer"]
pub struct W(crate::W<VAD_SCLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VAD_SCLK_CFG_SPEC>;
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
impl From<crate::W<VAD_SCLK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VAD_SCLK_CFG_SPEC>) -> Self {
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
#[doc = "VAD module audio data serial clock configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vad_sclk_cfg](index.html) module"]
pub struct VAD_SCLK_CFG_SPEC;
impl crate::RegisterSpec for VAD_SCLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vad_sclk_cfg::R](R) reader structure"]
impl crate::Readable for VAD_SCLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vad_sclk_cfg::W](W) writer structure"]
impl crate::Writable for VAD_SCLK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VAD_SCLK_CFG to value 0"]
impl crate::Resettable for VAD_SCLK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
