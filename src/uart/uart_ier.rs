#[doc = "Register `UART_IER` reader"]
pub struct R(crate::R<UART_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_IER` writer"]
pub struct W(crate::W<UART_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_IER_SPEC>;
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
impl From<crate::W<UART_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Programmable THRE Interrupt Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTIME_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<PTIME_A> for bool {
    #[inline(always)]
    fn from(variant: PTIME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTIME` reader - Programmable THRE Interrupt Mode Enable"]
pub type PTIME_R = crate::BitReader<PTIME_A>;
impl PTIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTIME_A {
        match self.bits {
            false => PTIME_A::DISABLE,
            true => PTIME_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PTIME_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PTIME_A::ENABLE
    }
}
#[doc = "Field `PTIME` writer - Programmable THRE Interrupt Mode Enable"]
pub type PTIME_W<'a> = crate::BitWriter<'a, u32, UART_IER_SPEC, PTIME_A, 7>;
impl<'a> PTIME_W<'a> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PTIME_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PTIME_A::ENABLE)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELCOLR_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<ELCOLR_A> for bool {
    #[inline(always)]
    fn from(variant: ELCOLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELCOLR` reader - "]
pub type ELCOLR_R = crate::BitReader<ELCOLR_A>;
impl ELCOLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELCOLR_A {
        match self.bits {
            false => ELCOLR_A::DISABLE,
            true => ELCOLR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ELCOLR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ELCOLR_A::ENABLE
    }
}
#[doc = "Enable Modem Status Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDSSI_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<EDSSI_A> for bool {
    #[inline(always)]
    fn from(variant: EDSSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDSSI` reader - Enable Modem Status Interrupt"]
pub type EDSSI_R = crate::BitReader<EDSSI_A>;
impl EDSSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDSSI_A {
        match self.bits {
            false => EDSSI_A::DISABLE,
            true => EDSSI_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDSSI_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDSSI_A::ENABLE
    }
}
#[doc = "Field `EDSSI` writer - Enable Modem Status Interrupt"]
pub type EDSSI_W<'a> = crate::BitWriter<'a, u32, UART_IER_SPEC, EDSSI_A, 3>;
impl<'a> EDSSI_W<'a> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDSSI_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDSSI_A::ENABLE)
    }
}
#[doc = "Enable Receiver Line Status Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELSI_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<ELSI_A> for bool {
    #[inline(always)]
    fn from(variant: ELSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELSI` reader - Enable Receiver Line Status Interrupt"]
pub type ELSI_R = crate::BitReader<ELSI_A>;
impl ELSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSI_A {
        match self.bits {
            false => ELSI_A::DISABLE,
            true => ELSI_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ELSI_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ELSI_A::ENABLE
    }
}
#[doc = "Field `ELSI` writer - Enable Receiver Line Status Interrupt"]
pub type ELSI_W<'a> = crate::BitWriter<'a, u32, UART_IER_SPEC, ELSI_A, 2>;
impl<'a> ELSI_W<'a> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ELSI_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ELSI_A::ENABLE)
    }
}
#[doc = "Enable Transmit Holding Register Empty Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETBEI_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<ETBEI_A> for bool {
    #[inline(always)]
    fn from(variant: ETBEI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETBEI` reader - Enable Transmit Holding Register Empty Interrupt"]
pub type ETBEI_R = crate::BitReader<ETBEI_A>;
impl ETBEI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETBEI_A {
        match self.bits {
            false => ETBEI_A::DISABLE,
            true => ETBEI_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ETBEI_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ETBEI_A::ENABLE
    }
}
#[doc = "Field `ETBEI` writer - Enable Transmit Holding Register Empty Interrupt"]
pub type ETBEI_W<'a> = crate::BitWriter<'a, u32, UART_IER_SPEC, ETBEI_A, 1>;
impl<'a> ETBEI_W<'a> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ETBEI_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ETBEI_A::ENABLE)
    }
}
#[doc = "Enable Receive Data Available Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERBFI_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<ERBFI_A> for bool {
    #[inline(always)]
    fn from(variant: ERBFI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERBFI` reader - Enable Receive Data Available Interrupt"]
pub type ERBFI_R = crate::BitReader<ERBFI_A>;
impl ERBFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERBFI_A {
        match self.bits {
            false => ERBFI_A::DISABLE,
            true => ERBFI_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERBFI_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERBFI_A::ENABLE
    }
}
#[doc = "Field `ERBFI` writer - Enable Receive Data Available Interrupt"]
pub type ERBFI_W<'a> = crate::BitWriter<'a, u32, UART_IER_SPEC, ERBFI_A, 0>;
impl<'a> ERBFI_W<'a> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERBFI_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERBFI_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 7 - Programmable THRE Interrupt Mode Enable"]
    #[inline(always)]
    pub fn ptime(&self) -> PTIME_R {
        PTIME_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn elcolr(&self) -> ELCOLR_R {
        ELCOLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Modem Status Interrupt"]
    #[inline(always)]
    pub fn edssi(&self) -> EDSSI_R {
        EDSSI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn elsi(&self) -> ELSI_R {
        ELSI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Transmit Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn etbei(&self) -> ETBEI_R {
        ETBEI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable Receive Data Available Interrupt"]
    #[inline(always)]
    pub fn erbfi(&self) -> ERBFI_R {
        ERBFI_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Programmable THRE Interrupt Mode Enable"]
    #[inline(always)]
    pub fn ptime(&mut self) -> PTIME_W {
        PTIME_W::new(self)
    }
    #[doc = "Bit 3 - Enable Modem Status Interrupt"]
    #[inline(always)]
    pub fn edssi(&mut self) -> EDSSI_W {
        EDSSI_W::new(self)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn elsi(&mut self) -> ELSI_W {
        ELSI_W::new(self)
    }
    #[doc = "Bit 1 - Enable Transmit Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn etbei(&mut self) -> ETBEI_W {
        ETBEI_W::new(self)
    }
    #[doc = "Bit 0 - Enable Receive Data Available Interrupt"]
    #[inline(always)]
    pub fn erbfi(&mut self) -> ERBFI_W {
        ERBFI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ier](index.html) module"]
pub struct UART_IER_SPEC;
impl crate::RegisterSpec for UART_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_ier::R](R) reader structure"]
impl crate::Readable for UART_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_ier::W](W) writer structure"]
impl crate::Writable for UART_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_IER to value 0"]
impl crate::Resettable for UART_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
