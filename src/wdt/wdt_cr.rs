#[doc = "Register `WDT_CR` reader"]
pub struct R(crate::R<WDT_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_CR` writer"]
pub struct W(crate::W<WDT_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_CR_SPEC>;
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
impl From<crate::W<WDT_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_CR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_cr](index.html) module"]
pub struct WDT_CR_SPEC;
impl crate::RegisterSpec for WDT_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_cr::R](R) reader structure"]
impl crate::Readable for WDT_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_cr::W](W) writer structure"]
impl crate::Writable for WDT_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDT_CR to value 0"]
impl crate::Resettable for WDT_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
