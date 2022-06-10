#[doc = "Register `GPIO_RAW_INTSTATUS` reader"]
pub struct R(crate::R<GPIO_RAW_INTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_RAW_INTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_RAW_INTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_RAW_INTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Raw interrupt status of Port A (premasking bits).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_RAW_INTSTATUS_A {
    #[doc = "0: Inactive"]
    INACTIVE = 0,
    #[doc = "1: Active"]
    ACTIVE = 1,
}
impl From<GPIO_RAW_INTSTATUS_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_RAW_INTSTATUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_RAW_INTSTATUS` reader - Raw interrupt status of Port A (premasking bits)."]
pub type GPIO_RAW_INTSTATUS_R = crate::FieldReader<u32, GPIO_RAW_INTSTATUS_A>;
impl GPIO_RAW_INTSTATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_RAW_INTSTATUS_A> {
        match self.bits {
            0 => Some(GPIO_RAW_INTSTATUS_A::INACTIVE),
            1 => Some(GPIO_RAW_INTSTATUS_A::ACTIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == GPIO_RAW_INTSTATUS_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == GPIO_RAW_INTSTATUS_A::ACTIVE
    }
}
impl R {
    #[doc = "Bits 0:31 - Raw interrupt status of Port A (premasking bits)."]
    #[inline(always)]
    pub fn gpio_raw_intstatus(&self) -> GPIO_RAW_INTSTATUS_R {
        GPIO_RAW_INTSTATUS_R::new(self.bits)
    }
}
#[doc = "raw interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_raw_intstatus](index.html) module"]
pub struct GPIO_RAW_INTSTATUS_SPEC;
impl crate::RegisterSpec for GPIO_RAW_INTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_raw_intstatus::R](R) reader structure"]
impl crate::Readable for GPIO_RAW_INTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_RAW_INTSTATUS to value 0"]
impl crate::Resettable for GPIO_RAW_INTSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
