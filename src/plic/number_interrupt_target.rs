#[doc = "Register `NUMBER_INTERRUPT_TARGET` reader"]
pub struct R(crate::R<NUMBER_INTERRUPT_TARGET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NUMBER_INTERRUPT_TARGET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NUMBER_INTERRUPT_TARGET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NUMBER_INTERRUPT_TARGET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TARGET` reader - Number of supported target"]
pub type TARGET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INTERRUPT` reader - Number of supported interrupt"]
pub type INTERRUPT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - Number of supported target"]
    #[inline(always)]
    pub fn target(&self) -> TARGET_R {
        TARGET_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Number of supported interrupt"]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Number of interrupt and target configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [number_interrupt_target](index.html) module"]
pub struct NUMBER_INTERRUPT_TARGET_SPEC;
impl crate::RegisterSpec for NUMBER_INTERRUPT_TARGET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [number_interrupt_target::R](R) reader structure"]
impl crate::Readable for NUMBER_INTERRUPT_TARGET_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NUMBER_INTERRUPT_TARGET to value 0"]
impl crate::Resettable for NUMBER_INTERRUPT_TARGET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
