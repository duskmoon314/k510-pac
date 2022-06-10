#[doc = "Register `OSC_25M_OFF` reader"]
pub struct R(crate::R<OSC_25M_OFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_25M_OFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_25M_OFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_25M_OFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_25M_OFF` writer"]
pub struct W(crate::W<OSC_25M_OFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_25M_OFF_SPEC>;
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
impl From<crate::W<OSC_25M_OFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_25M_OFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_osc_25m_clock_off` writer - Write enable for bit 0 (osc_25m_clock_off)"]
pub type WE_OSC_25M_CLOCK_OFF_W<'a> = crate::BitWriter<'a, u32, OSC_25M_OFF_SPEC, bool, 16>;
#[doc = "osc 25m clock off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_25M_CLOCK_OFF_A {
    #[doc = "0: 25m_osc enable"]
    ON = 0,
    #[doc = "1: 25m_osc off"]
    OFF = 1,
}
impl From<OSC_25M_CLOCK_OFF_A> for bool {
    #[inline(always)]
    fn from(variant: OSC_25M_CLOCK_OFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `osc_25m_clock_off` reader - osc 25m clock off"]
pub type OSC_25M_CLOCK_OFF_R = crate::BitReader<OSC_25M_CLOCK_OFF_A>;
impl OSC_25M_CLOCK_OFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC_25M_CLOCK_OFF_A {
        match self.bits {
            false => OSC_25M_CLOCK_OFF_A::ON,
            true => OSC_25M_CLOCK_OFF_A::OFF,
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == OSC_25M_CLOCK_OFF_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == OSC_25M_CLOCK_OFF_A::OFF
    }
}
#[doc = "Field `osc_25m_clock_off` writer - osc 25m clock off"]
pub type OSC_25M_CLOCK_OFF_W<'a> =
    crate::BitWriter<'a, u32, OSC_25M_OFF_SPEC, OSC_25M_CLOCK_OFF_A, 0>;
impl<'a> OSC_25M_CLOCK_OFF_W<'a> {
    #[doc = "25m_osc enable"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(OSC_25M_CLOCK_OFF_A::ON)
    }
    #[doc = "25m_osc off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(OSC_25M_CLOCK_OFF_A::OFF)
    }
}
impl R {
    #[doc = "Bit 0 - osc 25m clock off"]
    #[inline(always)]
    pub fn osc_25m_clock_off(&self) -> OSC_25M_CLOCK_OFF_R {
        OSC_25M_CLOCK_OFF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Write enable for bit 0 (osc_25m_clock_off)"]
    #[inline(always)]
    pub fn we_osc_25m_clock_off(&mut self) -> WE_OSC_25M_CLOCK_OFF_W {
        WE_OSC_25M_CLOCK_OFF_W::new(self)
    }
    #[doc = "Bit 0 - osc 25m clock off"]
    #[inline(always)]
    pub fn osc_25m_clock_off(&mut self) -> OSC_25M_CLOCK_OFF_W {
        OSC_25M_CLOCK_OFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC 25M clock off register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_25m_off](index.html) module"]
pub struct OSC_25M_OFF_SPEC;
impl crate::RegisterSpec for OSC_25M_OFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_25m_off::R](R) reader structure"]
impl crate::Readable for OSC_25M_OFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_25m_off::W](W) writer structure"]
impl crate::Writable for OSC_25M_OFF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC_25M_OFF to value 0"]
impl crate::Resettable for OSC_25M_OFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
