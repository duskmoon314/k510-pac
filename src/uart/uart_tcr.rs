#[doc = "Register `UART_TCR` reader"]
pub struct R(crate::R<UART_TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_TCR` writer"]
pub struct W(crate::W<UART_TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_TCR_SPEC>;
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
impl From<crate::W<UART_TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transfer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XFER_MODE_A {
    #[doc = "0: Mode 0"]
    MODE0 = 0,
    #[doc = "1: Mode 1"]
    MODE1 = 1,
    #[doc = "2: Mode 2"]
    MODE2 = 2,
}
impl From<XFER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: XFER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `XFER_MODE` reader - Transfer Mode"]
pub type XFER_MODE_R = crate::FieldReader<u8, XFER_MODE_A>;
impl XFER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<XFER_MODE_A> {
        match self.bits {
            0 => Some(XFER_MODE_A::MODE0),
            1 => Some(XFER_MODE_A::MODE1),
            2 => Some(XFER_MODE_A::MODE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == XFER_MODE_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == XFER_MODE_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == XFER_MODE_A::MODE2
    }
}
#[doc = "Field `XFER_MODE` writer - Transfer Mode"]
pub type XFER_MODE_W<'a> = crate::FieldWriter<'a, u32, UART_TCR_SPEC, u8, XFER_MODE_A, 2, 3>;
impl<'a> XFER_MODE_W<'a> {
    #[doc = "Mode 0"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(XFER_MODE_A::MODE0)
    }
    #[doc = "Mode 1"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(XFER_MODE_A::MODE1)
    }
    #[doc = "Mode 2"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(XFER_MODE_A::MODE2)
    }
}
#[doc = "Driver Enable Polarity\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DE_POL_A {
    #[doc = "0: Driver Enable active low"]
    ACTIVE_LOW = 0,
    #[doc = "1: Driver Enable active high"]
    ACTIVE_HIGH = 1,
}
impl From<DE_POL_A> for bool {
    #[inline(always)]
    fn from(variant: DE_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DE_POL` reader - Driver Enable Polarity"]
pub type DE_POL_R = crate::BitReader<DE_POL_A>;
impl DE_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DE_POL_A {
        match self.bits {
            false => DE_POL_A::ACTIVE_LOW,
            true => DE_POL_A::ACTIVE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == DE_POL_A::ACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == DE_POL_A::ACTIVE_HIGH
    }
}
#[doc = "Field `DE_POL` writer - Driver Enable Polarity"]
pub type DE_POL_W<'a> = crate::BitWriter<'a, u32, UART_TCR_SPEC, DE_POL_A, 2>;
impl<'a> DE_POL_W<'a> {
    #[doc = "Driver Enable active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(DE_POL_A::ACTIVE_LOW)
    }
    #[doc = "Driver Enable active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(DE_POL_A::ACTIVE_HIGH)
    }
}
#[doc = "Receiver Enable Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_POL_A {
    #[doc = "0: Receiver Enable active low"]
    ACTIVE_LOW = 0,
    #[doc = "1: Receiver Enable active high"]
    ACTIVE_HIGH = 1,
}
impl From<RE_POL_A> for bool {
    #[inline(always)]
    fn from(variant: RE_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE_POL` reader - Receiver Enable Polarity"]
pub type RE_POL_R = crate::BitReader<RE_POL_A>;
impl RE_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_POL_A {
        match self.bits {
            false => RE_POL_A::ACTIVE_LOW,
            true => RE_POL_A::ACTIVE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == RE_POL_A::ACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == RE_POL_A::ACTIVE_HIGH
    }
}
#[doc = "Field `RE_POL` writer - Receiver Enable Polarity"]
pub type RE_POL_W<'a> = crate::BitWriter<'a, u32, UART_TCR_SPEC, RE_POL_A, 1>;
impl<'a> RE_POL_W<'a> {
    #[doc = "Receiver Enable active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(RE_POL_A::ACTIVE_LOW)
    }
    #[doc = "Receiver Enable active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(RE_POL_A::ACTIVE_HIGH)
    }
}
#[doc = "RS485 Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RS485_EN_A {
    #[doc = "0: RS485 transfer disable"]
    DISABLE = 0,
    #[doc = "1: RS485 transfer enable"]
    ENABLE = 1,
}
impl From<RS485_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RS485_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RS485_EN` reader - RS485 Transfer Enable"]
pub type RS485_EN_R = crate::BitReader<RS485_EN_A>;
impl RS485_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RS485_EN_A {
        match self.bits {
            false => RS485_EN_A::DISABLE,
            true => RS485_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RS485_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RS485_EN_A::ENABLE
    }
}
#[doc = "Field `RS485_EN` writer - RS485 Transfer Enable"]
pub type RS485_EN_W<'a> = crate::BitWriter<'a, u32, UART_TCR_SPEC, RS485_EN_A, 0>;
impl<'a> RS485_EN_W<'a> {
    #[doc = "RS485 transfer disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RS485_EN_A::DISABLE)
    }
    #[doc = "RS485 transfer enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RS485_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 3:4 - Transfer Mode"]
    #[inline(always)]
    pub fn xfer_mode(&self) -> XFER_MODE_R {
        XFER_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 2 - Driver Enable Polarity"]
    #[inline(always)]
    pub fn de_pol(&self) -> DE_POL_R {
        DE_POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver Enable Polarity"]
    #[inline(always)]
    pub fn re_pol(&self) -> RE_POL_R {
        RE_POL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - RS485 Transfer Enable"]
    #[inline(always)]
    pub fn rs485_en(&self) -> RS485_EN_R {
        RS485_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 3:4 - Transfer Mode"]
    #[inline(always)]
    pub fn xfer_mode(&mut self) -> XFER_MODE_W {
        XFER_MODE_W::new(self)
    }
    #[doc = "Bit 2 - Driver Enable Polarity"]
    #[inline(always)]
    pub fn de_pol(&mut self) -> DE_POL_W {
        DE_POL_W::new(self)
    }
    #[doc = "Bit 1 - Receiver Enable Polarity"]
    #[inline(always)]
    pub fn re_pol(&mut self) -> RE_POL_W {
        RE_POL_W::new(self)
    }
    #[doc = "Bit 0 - RS485 Transfer Enable"]
    #[inline(always)]
    pub fn rs485_en(&mut self) -> RS485_EN_W {
        RS485_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "enable or disable RS485 mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_tcr](index.html) module"]
pub struct UART_TCR_SPEC;
impl crate::RegisterSpec for UART_TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_tcr::R](R) reader structure"]
impl crate::Readable for UART_TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_tcr::W](W) writer structure"]
impl crate::Writable for UART_TCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_TCR to value 0x04"]
impl crate::Resettable for UART_TCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
