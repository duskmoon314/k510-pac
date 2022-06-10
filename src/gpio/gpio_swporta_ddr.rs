#[doc = "Register `GPIO_SWPORTA_DDR` reader"]
pub struct R(crate::R<GPIO_SWPORTA_DDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_SWPORTA_DDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_SWPORTA_DDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_SWPORTA_DDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_SWPORTA_DDR` writer"]
pub struct W(crate::W<GPIO_SWPORTA_DDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_SWPORTA_DDR_SPEC>;
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
impl From<crate::W<GPIO_SWPORTA_DDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_SWPORTA_DDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Values written to this register independently control the direction of the corresponding data bit in Port A. The default direction can be configured as input or output after system reset through the GPIO_DFLT_DIR_A parameter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_SWPORTA_DDR_A {
    #[doc = "0: Input Direction"]
    INPUT = 0,
    #[doc = "1: Output Direction"]
    OUTPUT = 1,
}
impl From<GPIO_SWPORTA_DDR_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_SWPORTA_DDR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_SWPORTA_DDR` reader - Values written to this register independently control the direction of the corresponding data bit in Port A. The default direction can be configured as input or output after system reset through the GPIO_DFLT_DIR_A parameter."]
pub type GPIO_SWPORTA_DDR_R = crate::FieldReader<u32, GPIO_SWPORTA_DDR_A>;
impl GPIO_SWPORTA_DDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_SWPORTA_DDR_A> {
        match self.bits {
            0 => Some(GPIO_SWPORTA_DDR_A::INPUT),
            1 => Some(GPIO_SWPORTA_DDR_A::OUTPUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == GPIO_SWPORTA_DDR_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == GPIO_SWPORTA_DDR_A::OUTPUT
    }
}
#[doc = "Field `GPIO_SWPORTA_DDR` writer - Values written to this register independently control the direction of the corresponding data bit in Port A. The default direction can be configured as input or output after system reset through the GPIO_DFLT_DIR_A parameter."]
pub type GPIO_SWPORTA_DDR_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_SWPORTA_DDR_SPEC, u32, GPIO_SWPORTA_DDR_A, 32, 0>;
impl<'a> GPIO_SWPORTA_DDR_W<'a> {
    #[doc = "Input Direction"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(GPIO_SWPORTA_DDR_A::INPUT)
    }
    #[doc = "Output Direction"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(GPIO_SWPORTA_DDR_A::OUTPUT)
    }
}
impl R {
    #[doc = "Bits 0:31 - Values written to this register independently control the direction of the corresponding data bit in Port A. The default direction can be configured as input or output after system reset through the GPIO_DFLT_DIR_A parameter."]
    #[inline(always)]
    pub fn gpio_swporta_ddr(&self) -> GPIO_SWPORTA_DDR_R {
        GPIO_SWPORTA_DDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Values written to this register independently control the direction of the corresponding data bit in Port A. The default direction can be configured as input or output after system reset through the GPIO_DFLT_DIR_A parameter."]
    #[inline(always)]
    pub fn gpio_swporta_ddr(&mut self) -> GPIO_SWPORTA_DDR_W {
        GPIO_SWPORTA_DDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "data direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_swporta_ddr](index.html) module"]
pub struct GPIO_SWPORTA_DDR_SPEC;
impl crate::RegisterSpec for GPIO_SWPORTA_DDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_swporta_ddr::R](R) reader structure"]
impl crate::Readable for GPIO_SWPORTA_DDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_swporta_ddr::W](W) writer structure"]
impl crate::Writable for GPIO_SWPORTA_DDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_SWPORTA_DDR to value 0"]
impl crate::Resettable for GPIO_SWPORTA_DDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
