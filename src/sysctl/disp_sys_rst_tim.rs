#[doc = "Register `DISP_SYS_RST_TIM` reader"]
pub struct R(crate::R<DISP_SYS_RST_TIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISP_SYS_RST_TIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISP_SYS_RST_TIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISP_SYS_RST_TIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DISP_SYS_RST_TIM` writer"]
pub struct W(crate::W<DISP_SYS_RST_TIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISP_SYS_RST_TIM_SPEC>;
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
impl From<crate::W<DISP_SYS_RST_TIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISP_SYS_RST_TIM_SPEC>) -> Self {
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
#[doc = "Display subsystem reset timing control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [disp_sys_rst_tim](index.html) module"]
pub struct DISP_SYS_RST_TIM_SPEC;
impl crate::RegisterSpec for DISP_SYS_RST_TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [disp_sys_rst_tim::R](R) reader structure"]
impl crate::Readable for DISP_SYS_RST_TIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [disp_sys_rst_tim::W](W) writer structure"]
impl crate::Writable for DISP_SYS_RST_TIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DISP_SYS_RST_TIM to value 0"]
impl crate::Resettable for DISP_SYS_RST_TIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
