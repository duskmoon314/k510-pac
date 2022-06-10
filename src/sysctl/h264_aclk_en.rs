#[doc = "Register `H264_ACLK_EN` reader"]
pub struct R(crate::R<H264_ACLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<H264_ACLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<H264_ACLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<H264_ACLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `H264_ACLK_EN` writer"]
pub struct W(crate::W<H264_ACLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<H264_ACLK_EN_SPEC>;
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
impl From<crate::W<H264_ACLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<H264_ACLK_EN_SPEC>) -> Self {
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
#[doc = "H264 module AXI clock enable control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [h264_aclk_en](index.html) module"]
pub struct H264_ACLK_EN_SPEC;
impl crate::RegisterSpec for H264_ACLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [h264_aclk_en::R](R) reader structure"]
impl crate::Readable for H264_ACLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [h264_aclk_en::W](W) writer structure"]
impl crate::Writable for H264_ACLK_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets H264_ACLK_EN to value 0"]
impl crate::Resettable for H264_ACLK_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
