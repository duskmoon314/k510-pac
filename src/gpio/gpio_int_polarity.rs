#[doc = "Register `GPIO_INT_POLARITY` reader"]
pub struct R(crate::R<GPIO_INT_POLARITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_POLARITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INT_POLARITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INT_POLARITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_POLARITY` writer"]
pub struct W(crate::W<GPIO_INT_POLARITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_POLARITY_SPEC>;
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
impl From<crate::W<GPIO_INT_POLARITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INT_POLARITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Controls the polarity of edge or level sensitivity that can occur on input of Port A. Whenever a 0 is written to a bit of this register, it configures the interrupt type to falling-edge or active-low sensitive; otherwise, it is rising-edge or active-high sensitive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INT_POLARITY_A {
    #[doc = "0: Active Low polarity"]
    ACTIVE_LOW = 0,
    #[doc = "1: Active High polarity"]
    ACTIVE_HIGH = 1,
}
impl From<GPIO_INT_POLARITY_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INT_POLARITY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_INT_POLARITY` reader - Controls the polarity of edge or level sensitivity that can occur on input of Port A. Whenever a 0 is written to a bit of this register, it configures the interrupt type to falling-edge or active-low sensitive; otherwise, it is rising-edge or active-high sensitive."]
pub type GPIO_INT_POLARITY_R = crate::FieldReader<u32, GPIO_INT_POLARITY_A>;
impl GPIO_INT_POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_INT_POLARITY_A> {
        match self.bits {
            0 => Some(GPIO_INT_POLARITY_A::ACTIVE_LOW),
            1 => Some(GPIO_INT_POLARITY_A::ACTIVE_HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == GPIO_INT_POLARITY_A::ACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == GPIO_INT_POLARITY_A::ACTIVE_HIGH
    }
}
#[doc = "Field `GPIO_INT_POLARITY` writer - Controls the polarity of edge or level sensitivity that can occur on input of Port A. Whenever a 0 is written to a bit of this register, it configures the interrupt type to falling-edge or active-low sensitive; otherwise, it is rising-edge or active-high sensitive."]
pub type GPIO_INT_POLARITY_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_INT_POLARITY_SPEC, u32, GPIO_INT_POLARITY_A, 32, 0>;
impl<'a> GPIO_INT_POLARITY_W<'a> {
    #[doc = "Active Low polarity"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(GPIO_INT_POLARITY_A::ACTIVE_LOW)
    }
    #[doc = "Active High polarity"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(GPIO_INT_POLARITY_A::ACTIVE_HIGH)
    }
}
impl R {
    #[doc = "Bits 0:31 - Controls the polarity of edge or level sensitivity that can occur on input of Port A. Whenever a 0 is written to a bit of this register, it configures the interrupt type to falling-edge or active-low sensitive; otherwise, it is rising-edge or active-high sensitive."]
    #[inline(always)]
    pub fn gpio_int_polarity(&self) -> GPIO_INT_POLARITY_R {
        GPIO_INT_POLARITY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls the polarity of edge or level sensitivity that can occur on input of Port A. Whenever a 0 is written to a bit of this register, it configures the interrupt type to falling-edge or active-low sensitive; otherwise, it is rising-edge or active-high sensitive."]
    #[inline(always)]
    pub fn gpio_int_polarity(&mut self) -> GPIO_INT_POLARITY_W {
        GPIO_INT_POLARITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_polarity](index.html) module"]
pub struct GPIO_INT_POLARITY_SPEC;
impl crate::RegisterSpec for GPIO_INT_POLARITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_polarity::R](R) reader structure"]
impl crate::Readable for GPIO_INT_POLARITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_polarity::W](W) writer structure"]
impl crate::Writable for GPIO_INT_POLARITY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INT_POLARITY to value 0"]
impl crate::Resettable for GPIO_INT_POLARITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
