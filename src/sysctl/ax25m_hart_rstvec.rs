#[doc = "Register `AX25M_HART%s_RSTVEC` reader"]
pub struct R(crate::R<AX25M_HART_RSTVEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AX25M_HART_RSTVEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AX25M_HART_RSTVEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AX25M_HART_RSTVEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESET_VECTOR` reader - AX25M dual-core CPU hart0 reset vector. After releasing reset to AX25M CPU, hart\\[i\\]
will start program excution from address set here"]
pub type RESET_VECTOR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - AX25M dual-core CPU hart0 reset vector. After releasing reset to AX25M CPU, hart\\[i\\]
will start program excution from address set here"]
    #[inline(always)]
    pub fn reset_vector(&self) -> RESET_VECTOR_R {
        RESET_VECTOR_R::new(self.bits)
    }
}
#[doc = "AX25M dual-core CPU hart\\[i\\]
reset vector register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ax25m_hart_rstvec](index.html) module"]
pub struct AX25M_HART_RSTVEC_SPEC;
impl crate::RegisterSpec for AX25M_HART_RSTVEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ax25m_hart_rstvec::R](R) reader structure"]
impl crate::Readable for AX25M_HART_RSTVEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AX25M_HART%s_RSTVEC to value 0"]
impl crate::Resettable for AX25M_HART_RSTVEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
