#[doc = "Register `GPIO_CONFIG_REG2` reader"]
pub struct R(crate::R<GPIO_CONFIG_REG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CONFIG_REG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CONFIG_REG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CONFIG_REG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENCODED_ID_PWIDTH_A` reader - The value of this register is derived from the GPIO_PWIDTH_A configuration parameter"]
pub type ENCODED_ID_PWIDTH_A_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - The value of this register is derived from the GPIO_PWIDTH_A configuration parameter"]
    #[inline(always)]
    pub fn encoded_id_pwidth_a(&self) -> ENCODED_ID_PWIDTH_A_R {
        ENCODED_ID_PWIDTH_A_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "parameters info\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_config_reg2](index.html) module"]
pub struct GPIO_CONFIG_REG2_SPEC;
impl crate::RegisterSpec for GPIO_CONFIG_REG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_config_reg2::R](R) reader structure"]
impl crate::Readable for GPIO_CONFIG_REG2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_CONFIG_REG2 to value 0"]
impl crate::Resettable for GPIO_CONFIG_REG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
