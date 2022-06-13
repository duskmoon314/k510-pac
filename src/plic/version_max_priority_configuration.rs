#[doc = "Register `VERSION_MAX_PRIORITY_CONFIGURATION` reader"]
pub struct R(crate::R<VERSION_MAX_PRIORITY_CONFIGURATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VERSION_MAX_PRIORITY_CONFIGURATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VERSION_MAX_PRIORITY_CONFIGURATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VERSION_MAX_PRIORITY_CONFIGURATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAX_PRIORITY` reader - The maximum priority supported"]
pub type MAX_PRIORITY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VERSION` reader - The version of the AndeStar V5 PLIC design"]
pub type VERSION_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - The maximum priority supported"]
    #[inline(always)]
    pub fn max_priority(&self) -> MAX_PRIORITY_R {
        MAX_PRIORITY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - The version of the AndeStar V5 PLIC design"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Version and Maximum priority configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version_max_priority_configuration](index.html) module"]
pub struct VERSION_MAX_PRIORITY_CONFIGURATION_SPEC;
impl crate::RegisterSpec for VERSION_MAX_PRIORITY_CONFIGURATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [version_max_priority_configuration::R](R) reader structure"]
impl crate::Readable for VERSION_MAX_PRIORITY_CONFIGURATION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VERSION_MAX_PRIORITY_CONFIGURATION to value 0"]
impl crate::Resettable for VERSION_MAX_PRIORITY_CONFIGURATION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
