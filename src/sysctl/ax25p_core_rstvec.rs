#[doc = "Register `AX25P_CORE_RSTVEC` reader"]
pub struct R(crate::R<AX25P_CORE_RSTVEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AX25P_CORE_RSTVEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AX25P_CORE_RSTVEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AX25P_CORE_RSTVEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESET_VECTOR` reader - AX25P single-core CPU reset vector. After releasing reset to AX25P CPU, the core will start program excution from address set here"]
pub type RESET_VECTOR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - AX25P single-core CPU reset vector. After releasing reset to AX25P CPU, the core will start program excution from address set here"]
    #[inline(always)]
    pub fn reset_vector(&self) -> RESET_VECTOR_R {
        RESET_VECTOR_R::new(self.bits)
    }
}
#[doc = "AX25P processor CPU core reset vector register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ax25p_core_rstvec](index.html) module"]
pub struct AX25P_CORE_RSTVEC_SPEC;
impl crate::RegisterSpec for AX25P_CORE_RSTVEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ax25p_core_rstvec::R](R) reader structure"]
impl crate::Readable for AX25P_CORE_RSTVEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AX25P_CORE_RSTVEC to value 0"]
impl crate::Resettable for AX25P_CORE_RSTVEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
