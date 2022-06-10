#[doc = "Register `PLL%s_CFG0` reader"]
pub struct R(crate::R<PLL_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL%s_CFG0` writer"]
pub struct W(crate::W<PLL_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_CFG0_SPEC>;
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
impl From<crate::W<PLL_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_pll_out_div` writer - Write enable for bits \\[27:24\\]
(pll_out_div)"]
pub type WE_PLL_OUT_DIV_W<'a> = crate::BitWriter<'a, u32, PLL_CFG0_SPEC, bool, 30>;
#[doc = "Field `WE_pll_ref_div` writer - Write enable for bits \\[21:16\\]
(pll_ref_div)"]
pub type WE_PLL_REF_DIV_W<'a> = crate::BitWriter<'a, u32, PLL_CFG0_SPEC, bool, 29>;
#[doc = "Field `WE_pll_fb_div` writer - Write enable for bits \\[12:0\\]
(pll_fb_div)"]
pub type WE_PLL_FB_DIV_W<'a> = crate::BitWriter<'a, u32, PLL_CFG0_SPEC, bool, 28>;
#[doc = "Field `pll_out_div` reader - PLL\\[i\\]
VCO output clock post division ratio. It is strongly suggested not to modify this value."]
pub type PLL_OUT_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pll_out_div` writer - PLL\\[i\\]
VCO output clock post division ratio. It is strongly suggested not to modify this value."]
pub type PLL_OUT_DIV_W<'a> = crate::FieldWriter<'a, u32, PLL_CFG0_SPEC, u8, u8, 4, 24>;
#[doc = "Field `pll_ref_div` reader - PLL\\[i\\]
reference clock path division ratio."]
pub type PLL_REF_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pll_ref_div` writer - PLL\\[i\\]
reference clock path division ratio."]
pub type PLL_REF_DIV_W<'a> = crate::FieldWriter<'a, u32, PLL_CFG0_SPEC, u8, u8, 6, 16>;
#[doc = "Field `pll_fb_div` reader - PLL\\[i\\]
feedback path clock division ratio."]
pub type PLL_FB_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pll_fb_div` writer - PLL\\[i\\]
feedback path clock division ratio."]
pub type PLL_FB_DIV_W<'a> = crate::FieldWriter<'a, u32, PLL_CFG0_SPEC, u16, u16, 13, 0>;
impl R {
    #[doc = "Bits 24:27 - PLL\\[i\\]
VCO output clock post division ratio. It is strongly suggested not to modify this value."]
    #[inline(always)]
    pub fn pll_out_div(&self) -> PLL_OUT_DIV_R {
        PLL_OUT_DIV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - PLL\\[i\\]
reference clock path division ratio."]
    #[inline(always)]
    pub fn pll_ref_div(&self) -> PLL_REF_DIV_R {
        PLL_REF_DIV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 0:12 - PLL\\[i\\]
feedback path clock division ratio."]
    #[inline(always)]
    pub fn pll_fb_div(&self) -> PLL_FB_DIV_R {
        PLL_FB_DIV_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 30 - Write enable for bits \\[27:24\\]
(pll_out_div)"]
    #[inline(always)]
    pub fn we_pll_out_div(&mut self) -> WE_PLL_OUT_DIV_W {
        WE_PLL_OUT_DIV_W::new(self)
    }
    #[doc = "Bit 29 - Write enable for bits \\[21:16\\]
(pll_ref_div)"]
    #[inline(always)]
    pub fn we_pll_ref_div(&mut self) -> WE_PLL_REF_DIV_W {
        WE_PLL_REF_DIV_W::new(self)
    }
    #[doc = "Bit 28 - Write enable for bits \\[12:0\\]
(pll_fb_div)"]
    #[inline(always)]
    pub fn we_pll_fb_div(&mut self) -> WE_PLL_FB_DIV_W {
        WE_PLL_FB_DIV_W::new(self)
    }
    #[doc = "Bits 24:27 - PLL\\[i\\]
VCO output clock post division ratio. It is strongly suggested not to modify this value."]
    #[inline(always)]
    pub fn pll_out_div(&mut self) -> PLL_OUT_DIV_W {
        PLL_OUT_DIV_W::new(self)
    }
    #[doc = "Bits 16:21 - PLL\\[i\\]
reference clock path division ratio."]
    #[inline(always)]
    pub fn pll_ref_div(&mut self) -> PLL_REF_DIV_W {
        PLL_REF_DIV_W::new(self)
    }
    #[doc = "Bits 0:12 - PLL\\[i\\]
feedback path clock division ratio."]
    #[inline(always)]
    pub fn pll_fb_div(&mut self) -> PLL_FB_DIV_W {
        PLL_FB_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL\\[i\\]
configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_cfg0](index.html) module"]
pub struct PLL_CFG0_SPEC;
impl crate::RegisterSpec for PLL_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_cfg0::R](R) reader structure"]
impl crate::Readable for PLL_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_cfg0::W](W) writer structure"]
impl crate::Writable for PLL_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL%s_CFG0 to value 0x0100_004f"]
impl crate::Resettable for PLL_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_004f
    }
}
