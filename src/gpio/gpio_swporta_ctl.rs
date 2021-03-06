#[doc = "Register `GPIO_SWPORTA_CTL` reader"]
pub struct R(crate::R<GPIO_SWPORTA_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_SWPORTA_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_SWPORTA_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_SWPORTA_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_SWPORTA_CTL` writer"]
pub struct W(crate::W<GPIO_SWPORTA_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_SWPORTA_CTL_SPEC>;
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
impl From<crate::W<GPIO_SWPORTA_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_SWPORTA_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_SWPORTA_CTL` reader - The data and control source for a signal can come from either software or hardware; this bit selects between them."]
pub type GPIO_SWPORTA_CTL_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_SWPORTA_CTL` writer - The data and control source for a signal can come from either software or hardware; this bit selects between them."]
pub type GPIO_SWPORTA_CTL_W<'a> = crate::BitWriter<'a, u32, GPIO_SWPORTA_CTL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - The data and control source for a signal can come from either software or hardware; this bit selects between them."]
    #[inline(always)]
    pub fn gpio_swporta_ctl(&self) -> GPIO_SWPORTA_CTL_R {
        GPIO_SWPORTA_CTL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The data and control source for a signal can come from either software or hardware; this bit selects between them."]
    #[inline(always)]
    pub fn gpio_swporta_ctl(&mut self) -> GPIO_SWPORTA_CTL_W {
        GPIO_SWPORTA_CTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "data source control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_swporta_ctl](index.html) module"]
pub struct GPIO_SWPORTA_CTL_SPEC;
impl crate::RegisterSpec for GPIO_SWPORTA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_swporta_ctl::R](R) reader structure"]
impl crate::Readable for GPIO_SWPORTA_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_swporta_ctl::W](W) writer structure"]
impl crate::Writable for GPIO_SWPORTA_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_SWPORTA_CTL to value 0"]
impl crate::Resettable for GPIO_SWPORTA_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
