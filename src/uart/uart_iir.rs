#[doc = "Register `UART_IIR` reader"]
pub struct R(crate::R<UART_IIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_IIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_IIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_IIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "FIFOs Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIFOSE_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "3: enable"]
    ENABLE = 3,
}
impl From<FIFOSE_A> for u8 {
    #[inline(always)]
    fn from(variant: FIFOSE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FIFOSE` reader - FIFOs Enabled"]
pub type FIFOSE_R = crate::FieldReader<u8, FIFOSE_A>;
impl FIFOSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FIFOSE_A> {
        match self.bits {
            0 => Some(FIFOSE_A::DISABLE),
            3 => Some(FIFOSE_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FIFOSE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FIFOSE_A::ENABLE
    }
}
#[doc = "Interrupt ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ID_A {
    #[doc = "0: modem status"]
    MODEM_STATUS = 0,
    #[doc = "1: no interrupt pending"]
    NO_INTERRUPT_PENDING = 1,
    #[doc = "2: THR empty"]
    THR_EMPTY = 2,
    #[doc = "4: received data available"]
    RECEIVED_DATA_AVAILABLE = 4,
    #[doc = "6: receiver line status"]
    RECEIVER_LINE_STATUS = 6,
    #[doc = "7: busy detect"]
    BUSY_DETECT = 7,
    #[doc = "12: character timeout"]
    CHARACTER_TIMEOUT = 12,
}
impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ID` reader - Interrupt ID"]
pub type ID_R = crate::FieldReader<u8, ID_A>;
impl ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ID_A> {
        match self.bits {
            0 => Some(ID_A::MODEM_STATUS),
            1 => Some(ID_A::NO_INTERRUPT_PENDING),
            2 => Some(ID_A::THR_EMPTY),
            4 => Some(ID_A::RECEIVED_DATA_AVAILABLE),
            6 => Some(ID_A::RECEIVER_LINE_STATUS),
            7 => Some(ID_A::BUSY_DETECT),
            12 => Some(ID_A::CHARACTER_TIMEOUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MODEM_STATUS`"]
    #[inline(always)]
    pub fn is_modem_status(&self) -> bool {
        *self == ID_A::MODEM_STATUS
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_PENDING`"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == ID_A::NO_INTERRUPT_PENDING
    }
    #[doc = "Checks if the value of the field is `THR_EMPTY`"]
    #[inline(always)]
    pub fn is_thr_empty(&self) -> bool {
        *self == ID_A::THR_EMPTY
    }
    #[doc = "Checks if the value of the field is `RECEIVED_DATA_AVAILABLE`"]
    #[inline(always)]
    pub fn is_received_data_available(&self) -> bool {
        *self == ID_A::RECEIVED_DATA_AVAILABLE
    }
    #[doc = "Checks if the value of the field is `RECEIVER_LINE_STATUS`"]
    #[inline(always)]
    pub fn is_receiver_line_status(&self) -> bool {
        *self == ID_A::RECEIVER_LINE_STATUS
    }
    #[doc = "Checks if the value of the field is `BUSY_DETECT`"]
    #[inline(always)]
    pub fn is_busy_detect(&self) -> bool {
        *self == ID_A::BUSY_DETECT
    }
    #[doc = "Checks if the value of the field is `CHARACTER_TIMEOUT`"]
    #[inline(always)]
    pub fn is_character_timeout(&self) -> bool {
        *self == ID_A::CHARACTER_TIMEOUT
    }
}
impl R {
    #[doc = "Bits 6:7 - FIFOs Enabled"]
    #[inline(always)]
    pub fn fifose(&self) -> FIFOSE_R {
        FIFOSE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 0:3 - Interrupt ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Interrupt Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_iir](index.html) module"]
pub struct UART_IIR_SPEC;
impl crate::RegisterSpec for UART_IIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_iir::R](R) reader structure"]
impl crate::Readable for UART_IIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_IIR to value 0"]
impl crate::Resettable for UART_IIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
