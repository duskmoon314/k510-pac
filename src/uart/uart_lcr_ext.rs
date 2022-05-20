#[doc = "Register `UART_LCR_EXT` reader"]
pub struct R(crate::R<UART_LCR_EXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_LCR_EXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_LCR_EXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_LCR_EXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_LCR_EXT` writer"]
pub struct W(crate::W<UART_LCR_EXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_LCR_EXT_SPEC>;
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
impl From<crate::W<UART_LCR_EXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_LCR_EXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRANSMIT_MODE_A {
    #[doc = "0: `0`"]
    MODE0 = 0,
    #[doc = "1: `1`"]
    MODE1 = 1,
}
impl From<TRANSMIT_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: TRANSMIT_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRANSMIT_MODE` reader - Transmit Mode Control"]
pub type TRANSMIT_MODE_R = crate::BitReader<TRANSMIT_MODE_A>;
impl TRANSMIT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRANSMIT_MODE_A {
        match self.bits {
            false => TRANSMIT_MODE_A::MODE0,
            true => TRANSMIT_MODE_A::MODE1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == TRANSMIT_MODE_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == TRANSMIT_MODE_A::MODE1
    }
}
#[doc = "Field `TRANSMIT_MODE` writer - Transmit Mode Control"]
pub type TRANSMIT_MODE_W<'a> = crate::BitWriter<'a, u32, UART_LCR_EXT_SPEC, TRANSMIT_MODE_A, 3>;
impl<'a> TRANSMIT_MODE_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(TRANSMIT_MODE_A::MODE0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(TRANSMIT_MODE_A::MODE1)
    }
}
#[doc = "Field `SEND_ADDR` reader - Send Address Control"]
pub type SEND_ADDR_R = crate::BitReader<bool>;
#[doc = "Field `SEND_ADDR` writer - Send Address Control"]
pub type SEND_ADDR_W<'a> = crate::BitWriter<'a, u32, UART_LCR_EXT_SPEC, bool, 2>;
#[doc = "Address Match Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_MATCH_A {
    #[doc = "0: `0`"]
    NORMAL_MODE = 0,
    #[doc = "1: `1`"]
    ADDRESS_MATCH_MODE = 1,
}
impl From<ADDR_MATCH_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_MATCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR_MATCH` reader - Address Match Mode"]
pub type ADDR_MATCH_R = crate::BitReader<ADDR_MATCH_A>;
impl ADDR_MATCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_MATCH_A {
        match self.bits {
            false => ADDR_MATCH_A::NORMAL_MODE,
            true => ADDR_MATCH_A::ADDRESS_MATCH_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == ADDR_MATCH_A::NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `ADDRESS_MATCH_MODE`"]
    #[inline(always)]
    pub fn is_address_match_mode(&self) -> bool {
        *self == ADDR_MATCH_A::ADDRESS_MATCH_MODE
    }
}
#[doc = "Field `ADDR_MATCH` writer - Address Match Mode"]
pub type ADDR_MATCH_W<'a> = crate::BitWriter<'a, u32, UART_LCR_EXT_SPEC, ADDR_MATCH_A, 1>;
impl<'a> ADDR_MATCH_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(ADDR_MATCH_A::NORMAL_MODE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn address_match_mode(self) -> &'a mut W {
        self.variant(ADDR_MATCH_A::ADDRESS_MATCH_MODE)
    }
}
#[doc = "Field `DLS_E` reader - Extension for DLS"]
pub type DLS_E_R = crate::BitReader<bool>;
#[doc = "Field `DLS_E` writer - Extension for DLS"]
pub type DLS_E_W<'a> = crate::BitWriter<'a, u32, UART_LCR_EXT_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 3 - Transmit Mode Control"]
    #[inline(always)]
    pub fn transmit_mode(&self) -> TRANSMIT_MODE_R {
        TRANSMIT_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Send Address Control"]
    #[inline(always)]
    pub fn send_addr(&self) -> SEND_ADDR_R {
        SEND_ADDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Address Match Mode"]
    #[inline(always)]
    pub fn addr_match(&self) -> ADDR_MATCH_R {
        ADDR_MATCH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Extension for DLS"]
    #[inline(always)]
    pub fn dls_e(&self) -> DLS_E_R {
        DLS_E_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Transmit Mode Control"]
    #[inline(always)]
    pub fn transmit_mode(&mut self) -> TRANSMIT_MODE_W {
        TRANSMIT_MODE_W::new(self)
    }
    #[doc = "Bit 2 - Send Address Control"]
    #[inline(always)]
    pub fn send_addr(&mut self) -> SEND_ADDR_W {
        SEND_ADDR_W::new(self)
    }
    #[doc = "Bit 1 - Address Match Mode"]
    #[inline(always)]
    pub fn addr_match(&mut self) -> ADDR_MATCH_W {
        ADDR_MATCH_W::new(self)
    }
    #[doc = "Bit 0 - Extension for DLS"]
    #[inline(always)]
    pub fn dls_e(&mut self) -> DLS_E_W {
        DLS_E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Control Register Extended\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_lcr_ext](index.html) module"]
pub struct UART_LCR_EXT_SPEC;
impl crate::RegisterSpec for UART_LCR_EXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_lcr_ext::R](R) reader structure"]
impl crate::Readable for UART_LCR_EXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_lcr_ext::W](W) writer structure"]
impl crate::Writable for UART_LCR_EXT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_LCR_EXT to value 0"]
impl crate::Resettable for UART_LCR_EXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
