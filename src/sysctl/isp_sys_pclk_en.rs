#[doc = "Register `ISP_SYS_PCLK_EN` reader"]
pub struct R(crate::R<ISP_SYS_PCLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISP_SYS_PCLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISP_SYS_PCLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISP_SYS_PCLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISP_SYS_PCLK_EN` writer"]
pub struct W(crate::W<ISP_SYS_PCLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISP_SYS_PCLK_EN_SPEC>;
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
impl From<crate::W<ISP_SYS_PCLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISP_SYS_PCLK_EN_SPEC>) -> Self {
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
#[doc = "ISP system APB clock enable control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isp_sys_pclk_en](index.html) module"]
pub struct ISP_SYS_PCLK_EN_SPEC;
impl crate::RegisterSpec for ISP_SYS_PCLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isp_sys_pclk_en::R](R) reader structure"]
impl crate::Readable for ISP_SYS_PCLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isp_sys_pclk_en::W](W) writer structure"]
impl crate::Writable for ISP_SYS_PCLK_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISP_SYS_PCLK_EN to value 0"]
impl crate::Resettable for ISP_SYS_PCLK_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
