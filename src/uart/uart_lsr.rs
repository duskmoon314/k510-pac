#[doc = "Register `UART_LSR` reader"]
pub struct R(crate::R<UART_LSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_LSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_LSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_LSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Address Received Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_RCVD_A {
    #[doc = "0: the character is data"]
    DATA = 0,
    #[doc = "1: the character is an address"]
    ADDR = 1,
}
impl From<ADDR_RCVD_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_RCVD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR_RCVD` reader - Address Received Bit"]
pub type ADDR_RCVD_R = crate::BitReader<ADDR_RCVD_A>;
impl ADDR_RCVD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_RCVD_A {
        match self.bits {
            false => ADDR_RCVD_A::DATA,
            true => ADDR_RCVD_A::ADDR,
        }
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == ADDR_RCVD_A::DATA
    }
    #[doc = "Checks if the value of the field is `ADDR`"]
    #[inline(always)]
    pub fn is_addr(&self) -> bool {
        *self == ADDR_RCVD_A::ADDR
    }
}
#[doc = "Receive FIFO Error Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFE_A {
    #[doc = "0: no error in RX FIFO"]
    NO_RX_FIFO_ERROR = 0,
    #[doc = "1: error in RX FIFO"]
    RX_FIFO_ERROR = 1,
}
impl From<RFE_A> for bool {
    #[inline(always)]
    fn from(variant: RFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFE` reader - Receive FIFO Error Bit"]
pub type RFE_R = crate::BitReader<RFE_A>;
impl RFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFE_A {
        match self.bits {
            false => RFE_A::NO_RX_FIFO_ERROR,
            true => RFE_A::RX_FIFO_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RX_FIFO_ERROR`"]
    #[inline(always)]
    pub fn is_no_rx_fifo_error(&self) -> bool {
        *self == RFE_A::NO_RX_FIFO_ERROR
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_ERROR`"]
    #[inline(always)]
    pub fn is_rx_fifo_error(&self) -> bool {
        *self == RFE_A::RX_FIFO_ERROR
    }
}
#[doc = "Transmitter Empty Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMT_A {
    #[doc = "0: transmitter is not empty"]
    NOT_EMPTY = 0,
    #[doc = "1: transmitter is empty"]
    EMPTY = 1,
}
impl From<TEMT_A> for bool {
    #[inline(always)]
    fn from(variant: TEMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMT` reader - Transmitter Empty Bit"]
pub type TEMT_R = crate::BitReader<TEMT_A>;
impl TEMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMT_A {
        match self.bits {
            false => TEMT_A::NOT_EMPTY,
            true => TEMT_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TEMT_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TEMT_A::EMPTY
    }
}
#[doc = "Transmit Holding Register Empty Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THRE_A {
    #[doc = "0: transmitter is not empty"]
    NOT_EMPTY = 0,
    #[doc = "1: transmitter is empty"]
    EMPTY = 1,
}
impl From<THRE_A> for bool {
    #[inline(always)]
    fn from(variant: THRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THRE` reader - Transmit Holding Register Empty Bit"]
pub type THRE_R = crate::BitReader<THRE_A>;
impl THRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THRE_A {
        match self.bits {
            false => THRE_A::NOT_EMPTY,
            true => THRE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == THRE_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == THRE_A::EMPTY
    }
}
#[doc = "Break Interrupt Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BI_A {
    #[doc = "0: no break detected"]
    NO_BREAK = 0,
    #[doc = "1: break detected"]
    BREAK = 1,
}
impl From<BI_A> for bool {
    #[inline(always)]
    fn from(variant: BI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BI` reader - Break Interrupt Bit"]
pub type BI_R = crate::BitReader<BI_A>;
impl BI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BI_A {
        match self.bits {
            false => BI_A::NO_BREAK,
            true => BI_A::BREAK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BREAK`"]
    #[inline(always)]
    pub fn is_no_break(&self) -> bool {
        *self == BI_A::NO_BREAK
    }
    #[doc = "Checks if the value of the field is `BREAK`"]
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == BI_A::BREAK
    }
}
#[doc = "Framing Error Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_A {
    #[doc = "0: no framing error"]
    NO_FRAMING_ERROR = 0,
    #[doc = "1: framing error"]
    FRAMING_ERROR = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Framing Error Bit"]
pub type FE_R = crate::BitReader<FE_A>;
impl FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::NO_FRAMING_ERROR,
            true => FE_A::FRAMING_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FRAMING_ERROR`"]
    #[inline(always)]
    pub fn is_no_framing_error(&self) -> bool {
        *self == FE_A::NO_FRAMING_ERROR
    }
    #[doc = "Checks if the value of the field is `FRAMING_ERROR`"]
    #[inline(always)]
    pub fn is_framing_error(&self) -> bool {
        *self == FE_A::FRAMING_ERROR
    }
}
#[doc = "Parity Error Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    #[doc = "0: no parity error"]
    NO_PARITY_ERROR = 0,
    #[doc = "1: parity error"]
    PARITY_ERROR = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Parity Error Bit"]
pub type PE_R = crate::BitReader<PE_A>;
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::NO_PARITY_ERROR,
            true => PE_A::PARITY_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PARITY_ERROR`"]
    #[inline(always)]
    pub fn is_no_parity_error(&self) -> bool {
        *self == PE_A::NO_PARITY_ERROR
    }
    #[doc = "Checks if the value of the field is `PARITY_ERROR`"]
    #[inline(always)]
    pub fn is_parity_error(&self) -> bool {
        *self == PE_A::PARITY_ERROR
    }
}
#[doc = "Overrun Error Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OE_A {
    #[doc = "0: no overrun error"]
    NO_OVERRUN_ERROR = 0,
    #[doc = "1: overrun error"]
    OVERRUN_ERROR = 1,
}
impl From<OE_A> for bool {
    #[inline(always)]
    fn from(variant: OE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OE` reader - Overrun Error Bit"]
pub type OE_R = crate::BitReader<OE_A>;
impl OE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OE_A {
        match self.bits {
            false => OE_A::NO_OVERRUN_ERROR,
            true => OE_A::OVERRUN_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERRUN_ERROR`"]
    #[inline(always)]
    pub fn is_no_overrun_error(&self) -> bool {
        *self == OE_A::NO_OVERRUN_ERROR
    }
    #[doc = "Checks if the value of the field is `OVERRUN_ERROR`"]
    #[inline(always)]
    pub fn is_overrun_error(&self) -> bool {
        *self == OE_A::OVERRUN_ERROR
    }
}
#[doc = "Data Ready Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DR_A {
    #[doc = "0: data not ready"]
    NOT_READY = 0,
    #[doc = "1: data ready"]
    READY = 1,
}
impl From<DR_A> for bool {
    #[inline(always)]
    fn from(variant: DR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DR` reader - Data Ready Bit"]
pub type DR_R = crate::BitReader<DR_A>;
impl DR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DR_A {
        match self.bits {
            false => DR_A::NOT_READY,
            true => DR_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == DR_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == DR_A::READY
    }
}
impl R {
    #[doc = "Bit 8 - Address Received Bit"]
    #[inline(always)]
    pub fn addr_rcvd(&self) -> ADDR_RCVD_R {
        ADDR_RCVD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO Error Bit"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty Bit"]
    #[inline(always)]
    pub fn temt(&self) -> TEMT_R {
        TEMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Holding Register Empty Bit"]
    #[inline(always)]
    pub fn thre(&self) -> THRE_R {
        THRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt Bit"]
    #[inline(always)]
    pub fn bi(&self) -> BI_R {
        BI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing Error Bit"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error Bit"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error Bit"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Data Ready Bit"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Line Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_lsr](index.html) module"]
pub struct UART_LSR_SPEC;
impl crate::RegisterSpec for UART_LSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_lsr::R](R) reader structure"]
impl crate::Readable for UART_LSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_LSR to value 0"]
impl crate::Resettable for UART_LSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
