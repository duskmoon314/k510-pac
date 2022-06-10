#[doc = "Register `REPAIR_STATUS` reader"]
pub struct R(crate::R<REPAIR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REPAIR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REPAIR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REPAIR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REPAIR_STATUS` writer"]
pub struct W(crate::W<REPAIR_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REPAIR_STATUS_SPEC>;
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
impl From<crate::W<REPAIR_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REPAIR_STATUS_SPEC>) -> Self {
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
#[doc = "power domain group repair status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [repair_status](index.html) module"]
pub struct REPAIR_STATUS_SPEC;
impl crate::RegisterSpec for REPAIR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [repair_status::R](R) reader structure"]
impl crate::Readable for REPAIR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [repair_status::W](W) writer structure"]
impl crate::Writable for REPAIR_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REPAIR_STATUS to value 0"]
impl crate::Resettable for REPAIR_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
