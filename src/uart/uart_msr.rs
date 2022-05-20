#[doc = "Register `UART_MSR` reader"]
pub struct R(crate::R<UART_MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Data Carrier Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCD_A {
    #[doc = "0: dcd_n input is de-asserted"]
    DEASSERTED = 0,
    #[doc = "1: dcd_n input is asserted"]
    ASSERTED = 1,
}
impl From<DCD_A> for bool {
    #[inline(always)]
    fn from(variant: DCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCD` reader - Data Carrier Detect"]
pub type DCD_R = crate::BitReader<DCD_A>;
impl DCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCD_A {
        match self.bits {
            false => DCD_A::DEASSERTED,
            true => DCD_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == DCD_A::DEASSERTED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == DCD_A::ASSERTED
    }
}
#[doc = "Ring Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RI_A {
    #[doc = "0: ri_n input is de-asserted"]
    DEASSERTED = 0,
    #[doc = "1: ri_n input is asserted"]
    ASSERTED = 1,
}
impl From<RI_A> for bool {
    #[inline(always)]
    fn from(variant: RI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RI` reader - Ring Indicator"]
pub type RI_R = crate::BitReader<RI_A>;
impl RI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RI_A {
        match self.bits {
            false => RI_A::DEASSERTED,
            true => RI_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == RI_A::DEASSERTED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == RI_A::ASSERTED
    }
}
#[doc = "Data Set Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSR_A {
    #[doc = "0: dsr_n input is de-asserted"]
    DEASSERTED = 0,
    #[doc = "1: dsr_n input is asserted"]
    ASSERTED = 1,
}
impl From<DSR_A> for bool {
    #[inline(always)]
    fn from(variant: DSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSR` reader - Data Set Ready"]
pub type DSR_R = crate::BitReader<DSR_A>;
impl DSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSR_A {
        match self.bits {
            false => DSR_A::DEASSERTED,
            true => DSR_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == DSR_A::DEASSERTED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == DSR_A::ASSERTED
    }
}
#[doc = "Clear To Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_A {
    #[doc = "0: cts_n input is de-asserted"]
    DEASSERTED = 0,
    #[doc = "1: cts_n input is asserted"]
    ASSERTED = 1,
}
impl From<CTS_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` reader - Clear To Send"]
pub type CTS_R = crate::BitReader<CTS_A>;
impl CTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTS_A {
        match self.bits {
            false => CTS_A::DEASSERTED,
            true => CTS_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        *self == CTS_A::DEASSERTED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == CTS_A::ASSERTED
    }
}
#[doc = "Delta Data Carrier Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDCD_A {
    #[doc = "0: no change on dcd_n since last read of MSR"]
    NO_CHANGE = 0,
    #[doc = "1: change on dcd_n since last read of MSR"]
    CHANGE = 1,
}
impl From<DDCD_A> for bool {
    #[inline(always)]
    fn from(variant: DDCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDCD` reader - Delta Data Carrier Detect"]
pub type DDCD_R = crate::BitReader<DDCD_A>;
impl DDCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDCD_A {
        match self.bits {
            false => DDCD_A::NO_CHANGE,
            true => DDCD_A::CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == DDCD_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == DDCD_A::CHANGE
    }
}
#[doc = "Trailing Edge of Ring Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TERI_A {
    #[doc = "0: no change on ri_n since last read of MSR"]
    NO_CHANGE = 0,
    #[doc = "1: change on ri_n since last read of MSR"]
    CHANGE = 1,
}
impl From<TERI_A> for bool {
    #[inline(always)]
    fn from(variant: TERI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TERI` reader - Trailing Edge of Ring Indicator"]
pub type TERI_R = crate::BitReader<TERI_A>;
impl TERI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TERI_A {
        match self.bits {
            false => TERI_A::NO_CHANGE,
            true => TERI_A::CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == TERI_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == TERI_A::CHANGE
    }
}
#[doc = "Delta Data Set Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDST_A {
    #[doc = "0: no change on dsr_n since last read of MSR"]
    NO_CHANGE = 0,
    #[doc = "1: change on dsr_n since last read of MSR"]
    CHANGE = 1,
}
impl From<DDST_A> for bool {
    #[inline(always)]
    fn from(variant: DDST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDST` reader - Delta Data Set Ready"]
pub type DDST_R = crate::BitReader<DDST_A>;
impl DDST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDST_A {
        match self.bits {
            false => DDST_A::NO_CHANGE,
            true => DDST_A::CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == DDST_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == DDST_A::CHANGE
    }
}
#[doc = "Delta Clear To Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCTS_A {
    #[doc = "0: no change on cts_n since last read of MSR"]
    NO_CHANGE = 0,
    #[doc = "1: change on cts_n since last read of MSR"]
    CHANGE = 1,
}
impl From<DCTS_A> for bool {
    #[inline(always)]
    fn from(variant: DCTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCTS` reader - Delta Clear To Send"]
pub type DCTS_R = crate::BitReader<DCTS_A>;
impl DCTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCTS_A {
        match self.bits {
            false => DCTS_A::NO_CHANGE,
            true => DCTS_A::CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == DCTS_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == DCTS_A::CHANGE
    }
}
impl R {
    #[doc = "Bit 7 - Data Carrier Detect"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Ring Indicator"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Set Ready"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear To Send"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Delta Data Carrier Detect"]
    #[inline(always)]
    pub fn ddcd(&self) -> DDCD_R {
        DDCD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Trailing Edge of Ring Indicator"]
    #[inline(always)]
    pub fn teri(&self) -> TERI_R {
        TERI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Delta Data Set Ready"]
    #[inline(always)]
    pub fn ddst(&self) -> DDST_R {
        DDST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Delta Clear To Send"]
    #[inline(always)]
    pub fn dcts(&self) -> DCTS_R {
        DCTS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Modem Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_msr](index.html) module"]
pub struct UART_MSR_SPEC;
impl crate::RegisterSpec for UART_MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_msr::R](R) reader structure"]
impl crate::Readable for UART_MSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_MSR to value 0"]
impl crate::Resettable for UART_MSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
