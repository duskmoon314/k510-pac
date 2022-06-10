#[doc = "Register `SENSOR_RST_CTL` reader"]
pub struct R(crate::R<SENSOR_RST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SENSOR_RST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SENSOR_RST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SENSOR_RST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SENSOR_RST_CTL` writer"]
pub struct W(crate::W<SENSOR_RST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SENSOR_RST_CTL_SPEC>;
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
impl From<crate::W<SENSOR_RST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SENSOR_RST_CTL_SPEC>) -> Self {
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
#[doc = "SENSOR reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensor_rst_ctl](index.html) module"]
pub struct SENSOR_RST_CTL_SPEC;
impl crate::RegisterSpec for SENSOR_RST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sensor_rst_ctl::R](R) reader structure"]
impl crate::Readable for SENSOR_RST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sensor_rst_ctl::W](W) writer structure"]
impl crate::Writable for SENSOR_RST_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SENSOR_RST_CTL to value 0"]
impl crate::Resettable for SENSOR_RST_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}