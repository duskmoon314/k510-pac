#[doc = "Register `MTIMECMP%s` reader"]
pub struct R(crate::R<MTIMECMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTIMECMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTIMECMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTIMECMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTIMECMP%s` writer"]
pub struct W(crate::W<MTIMECMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTIMECMP_SPEC>;
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
impl From<crate::W<MTIMECMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTIMECMP_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "stores a 64-bit value for comparing with mtime for generate core\\[i\\]
timer interrupt\n\nstores a 64-bit value for comparing with mtime for generate core\\[i\\]
timer interrupt when mtime >= mtimecmp\\[i\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtimecmp](index.html) module"]
pub struct MTIMECMP_SPEC;
impl crate::RegisterSpec for MTIMECMP_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [mtimecmp::R](R) reader structure"]
impl crate::Readable for MTIMECMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtimecmp::W](W) writer structure"]
impl crate::Writable for MTIMECMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTIMECMP%s to value 0xffff_ffff_ffff_ffff"]
impl crate::Resettable for MTIMECMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff_ffff_ffff
    }
}
