#[doc = "Register `GPIO_INTEN` reader"]
pub struct R(crate::R<GPIO_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INTEN` writer"]
pub struct W(crate::W<GPIO_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INTEN_SPEC>;
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
impl From<crate::W<GPIO_INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Allows each bit of Port A to be configured for interrupts. By default the generation of interrupts is disabled. Whenever a 1 is written to a bit of this register, it configures the corresponding bit on Port A to become an interrupt; otherwise, Port A operates as a normal GPIO signal. Interrupts are disabled on the corresponding bits of Port A if the corresponding data direction register is set to Output or if Port A mode is set to Hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INTEN_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLE = 0,
    #[doc = "1: Interrupt is enabled"]
    ENABLE = 1,
}
impl From<GPIO_INTEN_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INTEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_INTEN` reader - Allows each bit of Port A to be configured for interrupts. By default the generation of interrupts is disabled. Whenever a 1 is written to a bit of this register, it configures the corresponding bit on Port A to become an interrupt; otherwise, Port A operates as a normal GPIO signal. Interrupts are disabled on the corresponding bits of Port A if the corresponding data direction register is set to Output or if Port A mode is set to Hardware."]
pub type GPIO_INTEN_R = crate::FieldReader<u32, GPIO_INTEN_A>;
impl GPIO_INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_INTEN_A> {
        match self.bits {
            0 => Some(GPIO_INTEN_A::DISABLE),
            1 => Some(GPIO_INTEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INTEN_A::ENABLE
    }
}
#[doc = "Field `GPIO_INTEN` writer - Allows each bit of Port A to be configured for interrupts. By default the generation of interrupts is disabled. Whenever a 1 is written to a bit of this register, it configures the corresponding bit on Port A to become an interrupt; otherwise, Port A operates as a normal GPIO signal. Interrupts are disabled on the corresponding bits of Port A if the corresponding data direction register is set to Output or if Port A mode is set to Hardware."]
pub type GPIO_INTEN_W<'a> = crate::FieldWriter<'a, u32, GPIO_INTEN_SPEC, u32, GPIO_INTEN_A, 32, 0>;
impl<'a> GPIO_INTEN_W<'a> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INTEN_A::DISABLE)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INTEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:31 - Allows each bit of Port A to be configured for interrupts. By default the generation of interrupts is disabled. Whenever a 1 is written to a bit of this register, it configures the corresponding bit on Port A to become an interrupt; otherwise, Port A operates as a normal GPIO signal. Interrupts are disabled on the corresponding bits of Port A if the corresponding data direction register is set to Output or if Port A mode is set to Hardware."]
    #[inline(always)]
    pub fn gpio_inten(&self) -> GPIO_INTEN_R {
        GPIO_INTEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Allows each bit of Port A to be configured for interrupts. By default the generation of interrupts is disabled. Whenever a 1 is written to a bit of this register, it configures the corresponding bit on Port A to become an interrupt; otherwise, Port A operates as a normal GPIO signal. Interrupts are disabled on the corresponding bits of Port A if the corresponding data direction register is set to Output or if Port A mode is set to Hardware."]
    #[inline(always)]
    pub fn gpio_inten(&mut self) -> GPIO_INTEN_W {
        GPIO_INTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_inten](index.html) module"]
pub struct GPIO_INTEN_SPEC;
impl crate::RegisterSpec for GPIO_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_inten::R](R) reader structure"]
impl crate::Readable for GPIO_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_inten::W](W) writer structure"]
impl crate::Writable for GPIO_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INTEN to value 0"]
impl crate::Resettable for GPIO_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
