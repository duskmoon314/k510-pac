#[doc = "Register `SOURCE%s_PRIORITY` reader"]
pub struct R(crate::R<SOURCE_PRIORITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCE_PRIORITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCE_PRIORITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCE_PRIORITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOURCE%s_PRIORITY` writer"]
pub struct W(crate::W<SOURCE_PRIORITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOURCE_PRIORITY_SPEC>;
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
impl From<crate::W<SOURCE_PRIORITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOURCE_PRIORITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIORITY` reader - Interrupt Source Priority"]
pub type PRIORITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIORITY` writer - Interrupt Source Priority"]
pub type PRIORITY_W<'a> = crate::FieldWriter<'a, u32, SOURCE_PRIORITY_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Interrupt Source Priority"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Source Priority"]
    #[inline(always)]
    pub fn priority(&mut self) -> PRIORITY_W {
        PRIORITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Priority Register for Source\\[i\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [source_priority](index.html) module"]
pub struct SOURCE_PRIORITY_SPEC;
impl crate::RegisterSpec for SOURCE_PRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [source_priority::R](R) reader structure"]
impl crate::Readable for SOURCE_PRIORITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [source_priority::W](W) writer structure"]
impl crate::Writable for SOURCE_PRIORITY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOURCE%s_PRIORITY to value 0x01"]
impl crate::Resettable for SOURCE_PRIORITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
