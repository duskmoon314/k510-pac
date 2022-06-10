#[doc = "Register `GPIO_DEBOUNCE` reader"]
pub struct R(crate::R<GPIO_DEBOUNCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_DEBOUNCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_DEBOUNCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_DEBOUNCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_DEBOUNCE` writer"]
pub struct W(crate::W<GPIO_DEBOUNCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_DEBOUNCE_SPEC>;
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
impl From<crate::W<GPIO_DEBOUNCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_DEBOUNCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Controls whether an external signal that is the source of an interrupt needs to be debounced to remove any spurious glitches. Writing a 1 to a bit in this register enables the debouncing circuitry. A signal must be valid for two periods of an external clock before it is internally processed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_DEBOUNCE_A {
    #[doc = "0: Disable debounce"]
    DISABLE = 0,
    #[doc = "1: Enable debounce"]
    ENABLE = 1,
}
impl From<GPIO_DEBOUNCE_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_DEBOUNCE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_DEBOUNCE` reader - Controls whether an external signal that is the source of an interrupt needs to be debounced to remove any spurious glitches. Writing a 1 to a bit in this register enables the debouncing circuitry. A signal must be valid for two periods of an external clock before it is internally processed."]
pub type GPIO_DEBOUNCE_R = crate::FieldReader<u32, GPIO_DEBOUNCE_A>;
impl GPIO_DEBOUNCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_DEBOUNCE_A> {
        match self.bits {
            0 => Some(GPIO_DEBOUNCE_A::DISABLE),
            1 => Some(GPIO_DEBOUNCE_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_DEBOUNCE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_DEBOUNCE_A::ENABLE
    }
}
#[doc = "Field `GPIO_DEBOUNCE` writer - Controls whether an external signal that is the source of an interrupt needs to be debounced to remove any spurious glitches. Writing a 1 to a bit in this register enables the debouncing circuitry. A signal must be valid for two periods of an external clock before it is internally processed."]
pub type GPIO_DEBOUNCE_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_DEBOUNCE_SPEC, u32, GPIO_DEBOUNCE_A, 32, 0>;
impl<'a> GPIO_DEBOUNCE_W<'a> {
    #[doc = "Disable debounce"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_DEBOUNCE_A::DISABLE)
    }
    #[doc = "Enable debounce"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_DEBOUNCE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:31 - Controls whether an external signal that is the source of an interrupt needs to be debounced to remove any spurious glitches. Writing a 1 to a bit in this register enables the debouncing circuitry. A signal must be valid for two periods of an external clock before it is internally processed."]
    #[inline(always)]
    pub fn gpio_debounce(&self) -> GPIO_DEBOUNCE_R {
        GPIO_DEBOUNCE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls whether an external signal that is the source of an interrupt needs to be debounced to remove any spurious glitches. Writing a 1 to a bit in this register enables the debouncing circuitry. A signal must be valid for two periods of an external clock before it is internally processed."]
    #[inline(always)]
    pub fn gpio_debounce(&mut self) -> GPIO_DEBOUNCE_W {
        GPIO_DEBOUNCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "debounce enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_debounce](index.html) module"]
pub struct GPIO_DEBOUNCE_SPEC;
impl crate::RegisterSpec for GPIO_DEBOUNCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_debounce::R](R) reader structure"]
impl crate::Readable for GPIO_DEBOUNCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_debounce::W](W) writer structure"]
impl crate::Writable for GPIO_DEBOUNCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_DEBOUNCE to value 0"]
impl crate::Resettable for GPIO_DEBOUNCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
