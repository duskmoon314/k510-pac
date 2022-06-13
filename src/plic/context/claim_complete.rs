#[doc = "Register `CLAIM_COMPLETE` reader"]
pub struct R(crate::R<CLAIM_COMPLETE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLAIM_COMPLETE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLAIM_COMPLETE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLAIM_COMPLETE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLAIM_COMPLETE` writer"]
pub struct W(crate::W<CLAIM_COMPLETE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLAIM_COMPLETE_SPEC>;
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
impl From<crate::W<CLAIM_COMPLETE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLAIM_COMPLETE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLAIM_COMPLETE` reader - Target\\[M\\]'s claim and complete"]
pub type CLAIM_COMPLETE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLAIM_COMPLETE` writer - Target\\[M\\]'s claim and complete"]
pub type CLAIM_COMPLETE_W<'a> = crate::FieldWriter<'a, u32, CLAIM_COMPLETE_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Target\\[M\\]'s claim and complete"]
    #[inline(always)]
    pub fn claim_complete(&self) -> CLAIM_COMPLETE_R {
        CLAIM_COMPLETE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Target\\[M\\]'s claim and complete"]
    #[inline(always)]
    pub fn claim_complete(&mut self) -> CLAIM_COMPLETE_W {
        CLAIM_COMPLETE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Target\\[M\\]'s claim and complete register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claim_complete](index.html) module"]
pub struct CLAIM_COMPLETE_SPEC;
impl crate::RegisterSpec for CLAIM_COMPLETE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [claim_complete::R](R) reader structure"]
impl crate::Readable for CLAIM_COMPLETE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [claim_complete::W](W) writer structure"]
impl crate::Writable for CLAIM_COMPLETE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLAIM_COMPLETE to value 0"]
impl crate::Resettable for CLAIM_COMPLETE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
