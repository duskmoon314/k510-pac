#[doc = "Register `UART_HTX` reader"]
pub struct R(crate::R<UART_HTX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_HTX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_HTX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_HTX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_HTX` writer"]
pub struct W(crate::W<UART_HTX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_HTX_SPEC>;
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
impl From<crate::W<UART_HTX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_HTX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Halt Tx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTX_A {
    #[doc = "0: Halt Transmission disable"]
    DISABLE = 0,
    #[doc = "1: Halt Transmission enable"]
    ENABLE = 1,
}
impl From<HTX_A> for bool {
    #[inline(always)]
    fn from(variant: HTX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTX` reader - Halt Tx"]
pub type HTX_R = crate::BitReader<HTX_A>;
impl HTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTX_A {
        match self.bits {
            false => HTX_A::DISABLE,
            true => HTX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HTX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HTX_A::ENABLE
    }
}
#[doc = "Field `HTX` writer - Halt Tx"]
pub type HTX_W<'a> = crate::BitWriter<'a, u32, UART_HTX_SPEC, HTX_A, 0>;
impl<'a> HTX_W<'a> {
    #[doc = "Halt Transmission disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HTX_A::DISABLE)
    }
    #[doc = "Halt Transmission enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HTX_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Halt Tx"]
    #[inline(always)]
    pub fn htx(&self) -> HTX_R {
        HTX_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt Tx"]
    #[inline(always)]
    pub fn htx(&mut self) -> HTX_W {
        HTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Halt Tx\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_htx](index.html) module"]
pub struct UART_HTX_SPEC;
impl crate::RegisterSpec for UART_HTX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_htx::R](R) reader structure"]
impl crate::Readable for UART_HTX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_htx::W](W) writer structure"]
impl crate::Writable for UART_HTX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_HTX to value 0"]
impl crate::Resettable for UART_HTX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
