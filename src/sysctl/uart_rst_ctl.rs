#[doc = "Register `UART%s_RST_CTL` reader"]
pub struct R(crate::R<UART_RST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_RST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_RST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_RST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART%s_RST_CTL` writer"]
pub struct W(crate::W<UART_RST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_RST_CTL_SPEC>;
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
impl From<crate::W<UART_RST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_RST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "UART\\[i\\]
host module reseting done status bit.\n\nWriting '1' to clear this status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_RST_DONE_A {
    #[doc = "0: no event"]
    NO_EVENT = 0,
    #[doc = "1: module reseting done"]
    DONE = 1,
}
impl From<UART_RST_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: UART_RST_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `uart_rst_done` reader - UART\\[i\\]
host module reseting done status bit.\n\nWriting '1' to clear this status bit."]
pub type UART_RST_DONE_R = crate::BitReader<UART_RST_DONE_A>;
impl UART_RST_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART_RST_DONE_A {
        match self.bits {
            false => UART_RST_DONE_A::NO_EVENT,
            true => UART_RST_DONE_A::DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == UART_RST_DONE_A::NO_EVENT
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == UART_RST_DONE_A::DONE
    }
}
#[doc = "Field `uart_rst_done` writer - UART\\[i\\]
host module reseting done status bit.\n\nWriting '1' to clear this status bit."]
pub type UART_RST_DONE_W<'a> = crate::BitWriter1C<'a, u32, UART_RST_CTL_SPEC, UART_RST_DONE_A, 31>;
impl<'a> UART_RST_DONE_W<'a> {
    #[doc = "no event"]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(UART_RST_DONE_A::NO_EVENT)
    }
    #[doc = "module reseting done"]
    #[inline(always)]
    pub fn done(self) -> &'a mut W {
        self.variant(UART_RST_DONE_A::DONE)
    }
}
impl R {
    #[doc = "Bit 31 - UART\\[i\\]
host module reseting done status bit.\n\nWriting '1' to clear this status bit."]
    #[inline(always)]
    pub fn uart_rst_done(&self) -> UART_RST_DONE_R {
        UART_RST_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - UART\\[i\\]
host module reseting done status bit.\n\nWriting '1' to clear this status bit."]
    #[inline(always)]
    pub fn uart_rst_done(&mut self) -> UART_RST_DONE_W {
        UART_RST_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART\\[i\\]
host module reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rst_ctl](index.html) module"]
pub struct UART_RST_CTL_SPEC;
impl crate::RegisterSpec for UART_RST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_rst_ctl::R](R) reader structure"]
impl crate::Readable for UART_RST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_rst_ctl::W](W) writer structure"]
impl crate::Writable for UART_RST_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART%s_RST_CTL to value 0"]
impl crate::Resettable for UART_RST_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
