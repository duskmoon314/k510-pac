#[doc = "Register `GPIO_INTTYPE_LEVEL` reader"]
pub struct R(crate::R<GPIO_INTTYPE_LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INTTYPE_LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INTTYPE_LEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INTTYPE_LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INTTYPE_LEVEL` writer"]
pub struct W(crate::W<GPIO_INTTYPE_LEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INTTYPE_LEVEL_SPEC>;
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
impl From<crate::W<GPIO_INTTYPE_LEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INTTYPE_LEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Controls the type of interrupt that can occur on Port A. Whenever a 0 is written to a bit of this register, it configures the interrupt type to be level- sensitive; otherwise, it is edgesensitive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INTTYPE_LEVEL_A {
    #[doc = "0: Interrupt is level sensitive"]
    LEVEL_SENSITIVE = 0,
    #[doc = "1: Interrupt is edge sensitive"]
    EDGE_SENSITIVE = 1,
}
impl From<GPIO_INTTYPE_LEVEL_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INTTYPE_LEVEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_INTTYPE_LEVEL` reader - Controls the type of interrupt that can occur on Port A. Whenever a 0 is written to a bit of this register, it configures the interrupt type to be level- sensitive; otherwise, it is edgesensitive."]
pub type GPIO_INTTYPE_LEVEL_R = crate::FieldReader<u32, GPIO_INTTYPE_LEVEL_A>;
impl GPIO_INTTYPE_LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_INTTYPE_LEVEL_A> {
        match self.bits {
            0 => Some(GPIO_INTTYPE_LEVEL_A::LEVEL_SENSITIVE),
            1 => Some(GPIO_INTTYPE_LEVEL_A::EDGE_SENSITIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE`"]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == GPIO_INTTYPE_LEVEL_A::LEVEL_SENSITIVE
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE`"]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == GPIO_INTTYPE_LEVEL_A::EDGE_SENSITIVE
    }
}
#[doc = "Field `GPIO_INTTYPE_LEVEL` writer - Controls the type of interrupt that can occur on Port A. Whenever a 0 is written to a bit of this register, it configures the interrupt type to be level- sensitive; otherwise, it is edgesensitive."]
pub type GPIO_INTTYPE_LEVEL_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_INTTYPE_LEVEL_SPEC, u32, GPIO_INTTYPE_LEVEL_A, 32, 0>;
impl<'a> GPIO_INTTYPE_LEVEL_W<'a> {
    #[doc = "Interrupt is level sensitive"]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut W {
        self.variant(GPIO_INTTYPE_LEVEL_A::LEVEL_SENSITIVE)
    }
    #[doc = "Interrupt is edge sensitive"]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut W {
        self.variant(GPIO_INTTYPE_LEVEL_A::EDGE_SENSITIVE)
    }
}
impl R {
    #[doc = "Bits 0:31 - Controls the type of interrupt that can occur on Port A. Whenever a 0 is written to a bit of this register, it configures the interrupt type to be level- sensitive; otherwise, it is edgesensitive."]
    #[inline(always)]
    pub fn gpio_inttype_level(&self) -> GPIO_INTTYPE_LEVEL_R {
        GPIO_INTTYPE_LEVEL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls the type of interrupt that can occur on Port A. Whenever a 0 is written to a bit of this register, it configures the interrupt type to be level- sensitive; otherwise, it is edgesensitive."]
    #[inline(always)]
    pub fn gpio_inttype_level(&mut self) -> GPIO_INTTYPE_LEVEL_W {
        GPIO_INTTYPE_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt level\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_inttype_level](index.html) module"]
pub struct GPIO_INTTYPE_LEVEL_SPEC;
impl crate::RegisterSpec for GPIO_INTTYPE_LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_inttype_level::R](R) reader structure"]
impl crate::Readable for GPIO_INTTYPE_LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_inttype_level::W](W) writer structure"]
impl crate::Writable for GPIO_INTTYPE_LEVEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INTTYPE_LEVEL to value 0"]
impl crate::Resettable for GPIO_INTTYPE_LEVEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
