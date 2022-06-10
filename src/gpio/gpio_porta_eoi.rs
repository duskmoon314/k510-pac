#[doc = "Register `GPIO_PORTA_EOI` writer"]
pub struct W(crate::W<GPIO_PORTA_EOI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_PORTA_EOI_SPEC>;
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
impl From<crate::W<GPIO_PORTA_EOI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_PORTA_EOI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Controls the clearing of edge type interrupts from Port A. When a 1 is written into a corresponding bit of this register, the interrupt is cleared. All interrupts are cleared when Port A is not configured for interrupts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_PORTA_EOI_AW {
    #[doc = "1: Clear interrupt"]
    CLEAR = 1,
}
impl From<GPIO_PORTA_EOI_AW> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_PORTA_EOI_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_PORTA_EOI` writer - Controls the clearing of edge type interrupts from Port A. When a 1 is written into a corresponding bit of this register, the interrupt is cleared. All interrupts are cleared when Port A is not configured for interrupts."]
pub type GPIO_PORTA_EOI_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_PORTA_EOI_SPEC, u32, GPIO_PORTA_EOI_AW, 32, 0>;
impl<'a> GPIO_PORTA_EOI_W<'a> {
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO_PORTA_EOI_AW::CLEAR)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls the clearing of edge type interrupts from Port A. When a 1 is written into a corresponding bit of this register, the interrupt is cleared. All interrupts are cleared when Port A is not configured for interrupts."]
    #[inline(always)]
    pub fn gpio_porta_eoi(&mut self) -> GPIO_PORTA_EOI_W {
        GPIO_PORTA_EOI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "end of interrupt\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_porta_eoi](index.html) module"]
pub struct GPIO_PORTA_EOI_SPEC;
impl crate::RegisterSpec for GPIO_PORTA_EOI_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpio_porta_eoi::W](W) writer structure"]
impl crate::Writable for GPIO_PORTA_EOI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_PORTA_EOI to value 0"]
impl crate::Resettable for GPIO_PORTA_EOI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
