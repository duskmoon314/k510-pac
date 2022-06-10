#[doc = "Register `GPIO_EXT_PORTA` reader"]
pub struct R(crate::R<GPIO_EXT_PORTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_EXT_PORTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_EXT_PORTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_EXT_PORTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO_EXT_PORTA` reader - This register always reflects the signals value on the External Port A."]
pub type GPIO_EXT_PORTA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register always reflects the signals value on the External Port A."]
    #[inline(always)]
    pub fn gpio_ext_porta(&self) -> GPIO_EXT_PORTA_R {
        GPIO_EXT_PORTA_R::new(self.bits)
    }
}
#[doc = "external port\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_ext_porta](index.html) module"]
pub struct GPIO_EXT_PORTA_SPEC;
impl crate::RegisterSpec for GPIO_EXT_PORTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_ext_porta::R](R) reader structure"]
impl crate::Readable for GPIO_EXT_PORTA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_EXT_PORTA to value 0"]
impl crate::Resettable for GPIO_EXT_PORTA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
