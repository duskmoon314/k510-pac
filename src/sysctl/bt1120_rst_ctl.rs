#[doc = "Register `BT1120_RST_CTL` reader"]
pub struct R(crate::R<BT1120_RST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BT1120_RST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BT1120_RST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BT1120_RST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BT1120_RST_CTL` writer"]
pub struct W(crate::W<BT1120_RST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BT1120_RST_CTL_SPEC>;
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
impl From<crate::W<BT1120_RST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BT1120_RST_CTL_SPEC>) -> Self {
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
#[doc = "BT1120 module reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt1120_rst_ctl](index.html) module"]
pub struct BT1120_RST_CTL_SPEC;
impl crate::RegisterSpec for BT1120_RST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bt1120_rst_ctl::R](R) reader structure"]
impl crate::Readable for BT1120_RST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bt1120_rst_ctl::W](W) writer structure"]
impl crate::Writable for BT1120_RST_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BT1120_RST_CTL to value 0"]
impl crate::Resettable for BT1120_RST_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
