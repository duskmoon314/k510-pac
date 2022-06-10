#[doc = "Register `GPIO_INTMASK` reader"]
pub struct R(crate::R<GPIO_INTMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INTMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INTMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INTMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INTMASK` writer"]
pub struct W(crate::W<GPIO_INTMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INTMASK_SPEC>;
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
impl From<crate::W<GPIO_INTMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INTMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Controls whether an interrupt on Port A can create an interrupt for the interrupt controller by not masking it. By default, all interrupts bits are unmasked. Whenever a 1 is written to a bit in this register, it masks the interrupt generation capability for this signal; otherwise interrupts are allowed through. The unmasked status can be read as well as the resultant status after masking.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INTMASK_A {
    #[doc = "0: Interrupt bits are unmasked"]
    DISABLE = 0,
    #[doc = "1: Mask interrupt"]
    ENABLE = 1,
}
impl From<GPIO_INTMASK_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INTMASK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_INTMASK` reader - Controls whether an interrupt on Port A can create an interrupt for the interrupt controller by not masking it. By default, all interrupts bits are unmasked. Whenever a 1 is written to a bit in this register, it masks the interrupt generation capability for this signal; otherwise interrupts are allowed through. The unmasked status can be read as well as the resultant status after masking."]
pub type GPIO_INTMASK_R = crate::FieldReader<u32, GPIO_INTMASK_A>;
impl GPIO_INTMASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_INTMASK_A> {
        match self.bits {
            0 => Some(GPIO_INTMASK_A::DISABLE),
            1 => Some(GPIO_INTMASK_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INTMASK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INTMASK_A::ENABLE
    }
}
#[doc = "Field `GPIO_INTMASK` writer - Controls whether an interrupt on Port A can create an interrupt for the interrupt controller by not masking it. By default, all interrupts bits are unmasked. Whenever a 1 is written to a bit in this register, it masks the interrupt generation capability for this signal; otherwise interrupts are allowed through. The unmasked status can be read as well as the resultant status after masking."]
pub type GPIO_INTMASK_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_INTMASK_SPEC, u32, GPIO_INTMASK_A, 32, 0>;
impl<'a> GPIO_INTMASK_W<'a> {
    #[doc = "Interrupt bits are unmasked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INTMASK_A::DISABLE)
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INTMASK_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:31 - Controls whether an interrupt on Port A can create an interrupt for the interrupt controller by not masking it. By default, all interrupts bits are unmasked. Whenever a 1 is written to a bit in this register, it masks the interrupt generation capability for this signal; otherwise interrupts are allowed through. The unmasked status can be read as well as the resultant status after masking."]
    #[inline(always)]
    pub fn gpio_intmask(&self) -> GPIO_INTMASK_R {
        GPIO_INTMASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls whether an interrupt on Port A can create an interrupt for the interrupt controller by not masking it. By default, all interrupts bits are unmasked. Whenever a 1 is written to a bit in this register, it masks the interrupt generation capability for this signal; otherwise interrupts are allowed through. The unmasked status can be read as well as the resultant status after masking."]
    #[inline(always)]
    pub fn gpio_intmask(&mut self) -> GPIO_INTMASK_W {
        GPIO_INTMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_intmask](index.html) module"]
pub struct GPIO_INTMASK_SPEC;
impl crate::RegisterSpec for GPIO_INTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_intmask::R](R) reader structure"]
impl crate::Readable for GPIO_INTMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_intmask::W](W) writer structure"]
impl crate::Writable for GPIO_INTMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INTMASK to value 0"]
impl crate::Resettable for GPIO_INTMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
