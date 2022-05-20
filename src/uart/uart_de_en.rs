#[doc = "Register `UART_DE_EN` reader"]
pub struct R(crate::R<UART_DE_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_DE_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_DE_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_DE_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_DE_EN` writer"]
pub struct W(crate::W<UART_DE_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_DE_EN_SPEC>;
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
impl From<crate::W<UART_DE_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_DE_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DE Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DE_EN_A {
    #[doc = "0: De-assert 'de' signal"]
    DISABLE = 0,
    #[doc = "1: Assert 'de' signal"]
    ENABLE = 1,
}
impl From<DE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DE_EN` reader - DE Enable"]
pub type DE_EN_R = crate::BitReader<DE_EN_A>;
impl DE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DE_EN_A {
        match self.bits {
            false => DE_EN_A::DISABLE,
            true => DE_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DE_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DE_EN_A::ENABLE
    }
}
#[doc = "Field `DE_EN` writer - DE Enable"]
pub type DE_EN_W<'a> = crate::BitWriter<'a, u32, UART_DE_EN_SPEC, DE_EN_A, 0>;
impl<'a> DE_EN_W<'a> {
    #[doc = "De-assert 'de' signal"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DE_EN_A::DISABLE)
    }
    #[doc = "Assert 'de' signal"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DE_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - DE Enable"]
    #[inline(always)]
    pub fn de_en(&self) -> DE_EN_R {
        DE_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DE Enable"]
    #[inline(always)]
    pub fn de_en(&mut self) -> DE_EN_W {
        DE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Driver Output Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_de_en](index.html) module"]
pub struct UART_DE_EN_SPEC;
impl crate::RegisterSpec for UART_DE_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_de_en::R](R) reader structure"]
impl crate::Readable for UART_DE_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_de_en::W](W) writer structure"]
impl crate::Writable for UART_DE_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_DE_EN to value 0"]
impl crate::Resettable for UART_DE_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
