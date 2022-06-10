#[doc = "Register `GPIO_ID_CODE` reader"]
pub struct R(crate::R<GPIO_ID_CODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_ID_CODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_ID_CODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_ID_CODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO_ID_CODE` reader - This is a user-specified code that a system can read. It can be used for chip identification, and so on."]
pub type GPIO_ID_CODE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This is a user-specified code that a system can read. It can be used for chip identification, and so on."]
    #[inline(always)]
    pub fn gpio_id_code(&self) -> GPIO_ID_CODE_R {
        GPIO_ID_CODE_R::new(self.bits)
    }
}
#[doc = "ID info\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_id_code](index.html) module"]
pub struct GPIO_ID_CODE_SPEC;
impl crate::RegisterSpec for GPIO_ID_CODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_id_code::R](R) reader structure"]
impl crate::Readable for GPIO_ID_CODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_ID_CODE to value 0"]
impl crate::Resettable for GPIO_ID_CODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
