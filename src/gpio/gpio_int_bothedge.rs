#[doc = "Register `GPIO_INT_BOTHEDGE` reader"]
pub struct R(crate::R<GPIO_INT_BOTHEDGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_BOTHEDGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INT_BOTHEDGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INT_BOTHEDGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_BOTHEDGE` writer"]
pub struct W(crate::W<GPIO_INT_BOTHEDGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_BOTHEDGE_SPEC>;
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
impl From<crate::W<GPIO_INT_BOTHEDGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INT_BOTHEDGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Controls the edge type of interrupt that can occur on Port A. Whenever a particular bit is programmed to 1, it enables the generation of interrupts on both the rising edge and the falling edge of an external input signal corresponding to that bit on port A. - The values programmed in the registers gpio_intype_level and gpio_int_polarity for this particular bit are not considered when the corresponding bit of this register is set to 1. - Whenever a particular bit is programmed to 0, the interrupt type depends on the value of the corresponding bits in the gpio_inttype_level and gpio_int_polarity registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_INT_BOTHEDGE_A {
    #[doc = "0: Single Edge sensitive"]
    SINGLE_EDGE = 0,
    #[doc = "1: Both Edge sensitive"]
    BOTH_EDGE = 1,
}
impl From<GPIO_INT_BOTHEDGE_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INT_BOTHEDGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_INT_BOTHEDGE` reader - Controls the edge type of interrupt that can occur on Port A. Whenever a particular bit is programmed to 1, it enables the generation of interrupts on both the rising edge and the falling edge of an external input signal corresponding to that bit on port A. - The values programmed in the registers gpio_intype_level and gpio_int_polarity for this particular bit are not considered when the corresponding bit of this register is set to 1. - Whenever a particular bit is programmed to 0, the interrupt type depends on the value of the corresponding bits in the gpio_inttype_level and gpio_int_polarity registers."]
pub type GPIO_INT_BOTHEDGE_R = crate::FieldReader<u32, GPIO_INT_BOTHEDGE_A>;
impl GPIO_INT_BOTHEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_INT_BOTHEDGE_A> {
        match self.bits {
            0 => Some(GPIO_INT_BOTHEDGE_A::SINGLE_EDGE),
            1 => Some(GPIO_INT_BOTHEDGE_A::BOTH_EDGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE`"]
    #[inline(always)]
    pub fn is_single_edge(&self) -> bool {
        *self == GPIO_INT_BOTHEDGE_A::SINGLE_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGE`"]
    #[inline(always)]
    pub fn is_both_edge(&self) -> bool {
        *self == GPIO_INT_BOTHEDGE_A::BOTH_EDGE
    }
}
#[doc = "Field `GPIO_INT_BOTHEDGE` writer - Controls the edge type of interrupt that can occur on Port A. Whenever a particular bit is programmed to 1, it enables the generation of interrupts on both the rising edge and the falling edge of an external input signal corresponding to that bit on port A. - The values programmed in the registers gpio_intype_level and gpio_int_polarity for this particular bit are not considered when the corresponding bit of this register is set to 1. - Whenever a particular bit is programmed to 0, the interrupt type depends on the value of the corresponding bits in the gpio_inttype_level and gpio_int_polarity registers."]
pub type GPIO_INT_BOTHEDGE_W<'a> =
    crate::FieldWriter<'a, u32, GPIO_INT_BOTHEDGE_SPEC, u32, GPIO_INT_BOTHEDGE_A, 32, 0>;
impl<'a> GPIO_INT_BOTHEDGE_W<'a> {
    #[doc = "Single Edge sensitive"]
    #[inline(always)]
    pub fn single_edge(self) -> &'a mut W {
        self.variant(GPIO_INT_BOTHEDGE_A::SINGLE_EDGE)
    }
    #[doc = "Both Edge sensitive"]
    #[inline(always)]
    pub fn both_edge(self) -> &'a mut W {
        self.variant(GPIO_INT_BOTHEDGE_A::BOTH_EDGE)
    }
}
impl R {
    #[doc = "Bits 0:31 - Controls the edge type of interrupt that can occur on Port A. Whenever a particular bit is programmed to 1, it enables the generation of interrupts on both the rising edge and the falling edge of an external input signal corresponding to that bit on port A. - The values programmed in the registers gpio_intype_level and gpio_int_polarity for this particular bit are not considered when the corresponding bit of this register is set to 1. - Whenever a particular bit is programmed to 0, the interrupt type depends on the value of the corresponding bits in the gpio_inttype_level and gpio_int_polarity registers."]
    #[inline(always)]
    pub fn gpio_int_bothedge(&self) -> GPIO_INT_BOTHEDGE_R {
        GPIO_INT_BOTHEDGE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls the edge type of interrupt that can occur on Port A. Whenever a particular bit is programmed to 1, it enables the generation of interrupts on both the rising edge and the falling edge of an external input signal corresponding to that bit on port A. - The values programmed in the registers gpio_intype_level and gpio_int_polarity for this particular bit are not considered when the corresponding bit of this register is set to 1. - Whenever a particular bit is programmed to 0, the interrupt type depends on the value of the corresponding bits in the gpio_inttype_level and gpio_int_polarity registers."]
    #[inline(always)]
    pub fn gpio_int_bothedge(&mut self) -> GPIO_INT_BOTHEDGE_W {
        GPIO_INT_BOTHEDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt edge type\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_bothedge](index.html) module"]
pub struct GPIO_INT_BOTHEDGE_SPEC;
impl crate::RegisterSpec for GPIO_INT_BOTHEDGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_bothedge::R](R) reader structure"]
impl crate::Readable for GPIO_INT_BOTHEDGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_bothedge::W](W) writer structure"]
impl crate::Writable for GPIO_INT_BOTHEDGE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INT_BOTHEDGE to value 0"]
impl crate::Resettable for GPIO_INT_BOTHEDGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
