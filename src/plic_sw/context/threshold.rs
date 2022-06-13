#[doc = "Register `THRESHOLD` reader"]
pub struct R(crate::R<THRESHOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRESHOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRESHOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRESHOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THRESHOLD` writer"]
pub struct W(crate::W<THRESHOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRESHOLD_SPEC>;
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
impl From<crate::W<THRESHOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRESHOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRESHOLD` reader - Target\\[M\\]'s priority threshold"]
pub type THRESHOLD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `THRESHOLD` writer - Target\\[M\\]'s priority threshold"]
pub type THRESHOLD_W<'a> = crate::FieldWriter<'a, u32, THRESHOLD_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Target\\[M\\]'s priority threshold"]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Target\\[M\\]'s priority threshold"]
    #[inline(always)]
    pub fn threshold(&mut self) -> THRESHOLD_W {
        THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Target\\[M\\]'s priority threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [threshold](index.html) module"]
pub struct THRESHOLD_SPEC;
impl crate::RegisterSpec for THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [threshold::R](R) reader structure"]
impl crate::Readable for THRESHOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [threshold::W](W) writer structure"]
impl crate::Writable for THRESHOLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THRESHOLD to value 0"]
impl crate::Resettable for THRESHOLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
