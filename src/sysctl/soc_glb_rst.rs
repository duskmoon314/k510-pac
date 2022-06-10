#[doc = "Register `SOC_GLB_RST` writer"]
pub struct W(crate::W<SOC_GLB_RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOC_GLB_RST_SPEC>;
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
impl From<crate::W<SOC_GLB_RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOC_GLB_RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_SOC_CORE_GLOBAL_RESET` writer - Write enable for bit 0 (SOC_CORE_GLOBAL_RESET)"]
pub type WE_SOC_CORE_GLOBAL_RESET_W<'a> = crate::BitWriter<'a, u32, SOC_GLB_RST_SPEC, bool, 16>;
#[doc = "SoC core global reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_CORE_GLOBAL_RESET_AW {
    #[doc = "0: No operation"]
    NO_OPERATION = 0,
    #[doc = "1: Reset SoC core"]
    RESET = 1,
}
impl From<SOC_CORE_GLOBAL_RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: SOC_CORE_GLOBAL_RESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOC_CORE_GLOBAL_RESET` writer - SoC core global reset"]
pub type SOC_CORE_GLOBAL_RESET_W<'a> =
    crate::BitWriter<'a, u32, SOC_GLB_RST_SPEC, SOC_CORE_GLOBAL_RESET_AW, 0>;
impl<'a> SOC_CORE_GLOBAL_RESET_W<'a> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(SOC_CORE_GLOBAL_RESET_AW::NO_OPERATION)
    }
    #[doc = "Reset SoC core"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SOC_CORE_GLOBAL_RESET_AW::RESET)
    }
}
impl W {
    #[doc = "Bit 16 - Write enable for bit 0 (SOC_CORE_GLOBAL_RESET)"]
    #[inline(always)]
    pub fn we_soc_core_global_reset(&mut self) -> WE_SOC_CORE_GLOBAL_RESET_W {
        WE_SOC_CORE_GLOBAL_RESET_W::new(self)
    }
    #[doc = "Bit 0 - SoC core global reset"]
    #[inline(always)]
    pub fn soc_core_global_reset(&mut self) -> SOC_CORE_GLOBAL_RESET_W {
        SOC_CORE_GLOBAL_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SoC global reset control register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_glb_rst](index.html) module"]
pub struct SOC_GLB_RST_SPEC;
impl crate::RegisterSpec for SOC_GLB_RST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [soc_glb_rst::W](W) writer structure"]
impl crate::Writable for SOC_GLB_RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOC_GLB_RST to value 0"]
impl crate::Resettable for SOC_GLB_RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
