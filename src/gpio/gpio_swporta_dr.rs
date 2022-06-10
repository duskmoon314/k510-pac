#[doc = "Register `GPIO_SWPORTA_DR` reader"]
pub struct R(crate::R<GPIO_SWPORTA_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_SWPORTA_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_SWPORTA_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_SWPORTA_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_SWPORTA_DR` writer"]
pub struct W(crate::W<GPIO_SWPORTA_DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_SWPORTA_DR_SPEC>;
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
impl From<crate::W<GPIO_SWPORTA_DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_SWPORTA_DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_SWPORTA_DR` reader - Values written to this register are output on the I/O signals for Port A if the corresponding data direction bits for Port A are set to Output mode and the corresponding control bit for Port A is set to Software mode. The value read back is equal to the last value written to this register."]
pub type GPIO_SWPORTA_DR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPIO_SWPORTA_DR` writer - Values written to this register are output on the I/O signals for Port A if the corresponding data direction bits for Port A are set to Output mode and the corresponding control bit for Port A is set to Software mode. The value read back is equal to the last value written to this register."]
pub type GPIO_SWPORTA_DR_W<'a> = crate::FieldWriter<'a, u32, GPIO_SWPORTA_DR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Values written to this register are output on the I/O signals for Port A if the corresponding data direction bits for Port A are set to Output mode and the corresponding control bit for Port A is set to Software mode. The value read back is equal to the last value written to this register."]
    #[inline(always)]
    pub fn gpio_swporta_dr(&self) -> GPIO_SWPORTA_DR_R {
        GPIO_SWPORTA_DR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Values written to this register are output on the I/O signals for Port A if the corresponding data direction bits for Port A are set to Output mode and the corresponding control bit for Port A is set to Software mode. The value read back is equal to the last value written to this register."]
    #[inline(always)]
    pub fn gpio_swporta_dr(&mut self) -> GPIO_SWPORTA_DR_W {
        GPIO_SWPORTA_DR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_swporta_dr](index.html) module"]
pub struct GPIO_SWPORTA_DR_SPEC;
impl crate::RegisterSpec for GPIO_SWPORTA_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_swporta_dr::R](R) reader structure"]
impl crate::Readable for GPIO_SWPORTA_DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_swporta_dr::W](W) writer structure"]
impl crate::Writable for GPIO_SWPORTA_DR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_SWPORTA_DR to value 0"]
impl crate::Resettable for GPIO_SWPORTA_DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
