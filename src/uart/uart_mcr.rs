#[doc = "Register `UART_MCR` reader"]
pub struct R(crate::R<UART_MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_MCR` writer"]
pub struct W(crate::W<UART_MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_MCR_SPEC>;
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
impl From<crate::W<UART_MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SIR Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRE_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<SIRE_A> for bool {
    #[inline(always)]
    fn from(variant: SIRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIRE` reader - SIR Mode Enable"]
pub type SIRE_R = crate::BitReader<SIRE_A>;
impl SIRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIRE_A {
        match self.bits {
            false => SIRE_A::DISABLE,
            true => SIRE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SIRE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SIRE_A::ENABLE
    }
}
#[doc = "Field `SIRE` writer - SIR Mode Enable"]
pub type SIRE_W<'a> = crate::BitWriter<'a, u32, UART_MCR_SPEC, SIRE_A, 6>;
impl<'a> SIRE_W<'a> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SIRE_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SIRE_A::ENABLE)
    }
}
#[doc = "Automatic Flow Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFCE_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<AFCE_A> for bool {
    #[inline(always)]
    fn from(variant: AFCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFCE` reader - Automatic Flow Control Enable"]
pub type AFCE_R = crate::BitReader<AFCE_A>;
impl AFCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFCE_A {
        match self.bits {
            false => AFCE_A::DISABLE,
            true => AFCE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AFCE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AFCE_A::ENABLE
    }
}
#[doc = "Field `AFCE` writer - Automatic Flow Control Enable"]
pub type AFCE_W<'a> = crate::BitWriter<'a, u32, UART_MCR_SPEC, AFCE_A, 5>;
impl<'a> AFCE_W<'a> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AFCE_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AFCE_A::ENABLE)
    }
}
#[doc = "Loopback Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPBACK_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<LOOPBACK_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPBACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPBACK` reader - Loopback Bit"]
pub type LOOPBACK_R = crate::BitReader<LOOPBACK_A>;
impl LOOPBACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPBACK_A {
        match self.bits {
            false => LOOPBACK_A::DISABLE,
            true => LOOPBACK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOOPBACK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LOOPBACK_A::ENABLE
    }
}
#[doc = "Field `LOOPBACK` writer - Loopback Bit"]
pub type LOOPBACK_W<'a> = crate::BitWriter<'a, u32, UART_MCR_SPEC, LOOPBACK_A, 4>;
impl<'a> LOOPBACK_W<'a> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOOPBACK_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOOPBACK_A::ENABLE)
    }
}
#[doc = "Out2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT2_A {
    #[doc = "0: out2_n de-asserted"]
    OUT2_0 = 0,
    #[doc = "1: out2_n asserted"]
    OUT2_1 = 1,
}
impl From<OUT2_A> for bool {
    #[inline(always)]
    fn from(variant: OUT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT2` reader - Out2"]
pub type OUT2_R = crate::BitReader<OUT2_A>;
impl OUT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT2_A {
        match self.bits {
            false => OUT2_A::OUT2_0,
            true => OUT2_A::OUT2_1,
        }
    }
    #[doc = "Checks if the value of the field is `OUT2_0`"]
    #[inline(always)]
    pub fn is_out2_0(&self) -> bool {
        *self == OUT2_A::OUT2_0
    }
    #[doc = "Checks if the value of the field is `OUT2_1`"]
    #[inline(always)]
    pub fn is_out2_1(&self) -> bool {
        *self == OUT2_A::OUT2_1
    }
}
#[doc = "Field `OUT2` writer - Out2"]
pub type OUT2_W<'a> = crate::BitWriter<'a, u32, UART_MCR_SPEC, OUT2_A, 3>;
impl<'a> OUT2_W<'a> {
    #[doc = "out2_n de-asserted"]
    #[inline(always)]
    pub fn out2_0(self) -> &'a mut W {
        self.variant(OUT2_A::OUT2_0)
    }
    #[doc = "out2_n asserted"]
    #[inline(always)]
    pub fn out2_1(self) -> &'a mut W {
        self.variant(OUT2_A::OUT2_1)
    }
}
#[doc = "Out1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT1_A {
    #[doc = "0: out1_n de-asserted"]
    OUT1_0 = 0,
    #[doc = "1: out1_n asserted"]
    OUT1_1 = 1,
}
impl From<OUT1_A> for bool {
    #[inline(always)]
    fn from(variant: OUT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT1` reader - Out1"]
pub type OUT1_R = crate::BitReader<OUT1_A>;
impl OUT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT1_A {
        match self.bits {
            false => OUT1_A::OUT1_0,
            true => OUT1_A::OUT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `OUT1_0`"]
    #[inline(always)]
    pub fn is_out1_0(&self) -> bool {
        *self == OUT1_A::OUT1_0
    }
    #[doc = "Checks if the value of the field is `OUT1_1`"]
    #[inline(always)]
    pub fn is_out1_1(&self) -> bool {
        *self == OUT1_A::OUT1_1
    }
}
#[doc = "Field `OUT1` writer - Out1"]
pub type OUT1_W<'a> = crate::BitWriter<'a, u32, UART_MCR_SPEC, OUT1_A, 2>;
impl<'a> OUT1_W<'a> {
    #[doc = "out1_n de-asserted"]
    #[inline(always)]
    pub fn out1_0(self) -> &'a mut W {
        self.variant(OUT1_A::OUT1_0)
    }
    #[doc = "out1_n asserted"]
    #[inline(always)]
    pub fn out1_1(self) -> &'a mut W {
        self.variant(OUT1_A::OUT1_1)
    }
}
#[doc = "Request to Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTS_A {
    #[doc = "0: Request to Send rts_n de-asserted"]
    INACTIVE = 0,
    #[doc = "1: Request to Send rts_n asserted"]
    ACTIVE = 1,
}
impl From<RTS_A> for bool {
    #[inline(always)]
    fn from(variant: RTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTS` reader - Request to Send"]
pub type RTS_R = crate::BitReader<RTS_A>;
impl RTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTS_A {
        match self.bits {
            false => RTS_A::INACTIVE,
            true => RTS_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == RTS_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == RTS_A::ACTIVE
    }
}
#[doc = "Field `RTS` writer - Request to Send"]
pub type RTS_W<'a> = crate::BitWriter<'a, u32, UART_MCR_SPEC, RTS_A, 1>;
impl<'a> RTS_W<'a> {
    #[doc = "Request to Send rts_n de-asserted"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(RTS_A::INACTIVE)
    }
    #[doc = "Request to Send rts_n asserted"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(RTS_A::ACTIVE)
    }
}
#[doc = "Data Terminal Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTR_A {
    #[doc = "0: Data Terminal Ready dtr_n de-asserted"]
    INACTIVE = 0,
    #[doc = "1: Data Terminal Ready dtr_n asserted"]
    ACTIVE = 1,
}
impl From<DTR_A> for bool {
    #[inline(always)]
    fn from(variant: DTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTR` reader - Data Terminal Ready"]
pub type DTR_R = crate::BitReader<DTR_A>;
impl DTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTR_A {
        match self.bits {
            false => DTR_A::INACTIVE,
            true => DTR_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DTR_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == DTR_A::ACTIVE
    }
}
#[doc = "Field `DTR` writer - Data Terminal Ready"]
pub type DTR_W<'a> = crate::BitWriter<'a, u32, UART_MCR_SPEC, DTR_A, 0>;
impl<'a> DTR_W<'a> {
    #[doc = "Data Terminal Ready dtr_n de-asserted"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DTR_A::INACTIVE)
    }
    #[doc = "Data Terminal Ready dtr_n asserted"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(DTR_A::ACTIVE)
    }
}
impl R {
    #[doc = "Bit 6 - SIR Mode Enable"]
    #[inline(always)]
    pub fn sire(&self) -> SIRE_R {
        SIRE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Flow Control Enable"]
    #[inline(always)]
    pub fn afce(&self) -> AFCE_R {
        AFCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback Bit"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Out2"]
    #[inline(always)]
    pub fn out2(&self) -> OUT2_R {
        OUT2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Out1"]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Request to Send"]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Data Terminal Ready"]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - SIR Mode Enable"]
    #[inline(always)]
    pub fn sire(&mut self) -> SIRE_W {
        SIRE_W::new(self)
    }
    #[doc = "Bit 5 - Automatic Flow Control Enable"]
    #[inline(always)]
    pub fn afce(&mut self) -> AFCE_W {
        AFCE_W::new(self)
    }
    #[doc = "Bit 4 - Loopback Bit"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W::new(self)
    }
    #[doc = "Bit 3 - Out2"]
    #[inline(always)]
    pub fn out2(&mut self) -> OUT2_W {
        OUT2_W::new(self)
    }
    #[doc = "Bit 2 - Out1"]
    #[inline(always)]
    pub fn out1(&mut self) -> OUT1_W {
        OUT1_W::new(self)
    }
    #[doc = "Bit 1 - Request to Send"]
    #[inline(always)]
    pub fn rts(&mut self) -> RTS_W {
        RTS_W::new(self)
    }
    #[doc = "Bit 0 - Data Terminal Ready"]
    #[inline(always)]
    pub fn dtr(&mut self) -> DTR_W {
        DTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Modem Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_mcr](index.html) module"]
pub struct UART_MCR_SPEC;
impl crate::RegisterSpec for UART_MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_mcr::R](R) reader structure"]
impl crate::Readable for UART_MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_mcr::W](W) writer structure"]
impl crate::Writable for UART_MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_MCR to value 0"]
impl crate::Resettable for UART_MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
