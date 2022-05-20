#[doc = "Register `UART_LCR` reader"]
pub struct R(crate::R<UART_LCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_LCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_LCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_LCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_LCR` writer"]
pub struct W(crate::W<UART_LCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_LCR_SPEC>;
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
impl From<crate::W<UART_LCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_LCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Divisor Latch Access Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLAB_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<DLAB_A> for bool {
    #[inline(always)]
    fn from(variant: DLAB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLAB` reader - Divisor Latch Access Bit"]
pub type DLAB_R = crate::BitReader<DLAB_A>;
impl DLAB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLAB_A {
        match self.bits {
            false => DLAB_A::DISABLE,
            true => DLAB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DLAB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DLAB_A::ENABLE
    }
}
#[doc = "Field `DLAB` writer - Divisor Latch Access Bit"]
pub type DLAB_W<'a> = crate::BitWriter<'a, u32, UART_LCR_SPEC, DLAB_A, 7>;
impl<'a> DLAB_W<'a> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DLAB_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DLAB_A::ENABLE)
    }
}
#[doc = "Break Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BC_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<BC_A> for bool {
    #[inline(always)]
    fn from(variant: BC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BC` reader - Break Control Bit"]
pub type BC_R = crate::BitReader<BC_A>;
impl BC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BC_A {
        match self.bits {
            false => BC_A::DISABLE,
            true => BC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BC_A::ENABLE
    }
}
#[doc = "Field `BC` writer - Break Control Bit"]
pub type BC_W<'a> = crate::BitWriter<'a, u32, UART_LCR_SPEC, BC_A, 6>;
impl<'a> BC_W<'a> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BC_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BC_A::ENABLE)
    }
}
#[doc = "Stick Parity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<SP_A> for bool {
    #[inline(always)]
    fn from(variant: SP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP` reader - Stick Parity"]
pub type SP_R = crate::BitReader<SP_A>;
impl SP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP_A {
        match self.bits {
            false => SP_A::DISABLE,
            true => SP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SP_A::ENABLE
    }
}
#[doc = "Field `SP` writer - Stick Parity"]
pub type SP_W<'a> = crate::BitWriter<'a, u32, UART_LCR_SPEC, SP_A, 5>;
impl<'a> SP_W<'a> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SP_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SP_A::ENABLE)
    }
}
#[doc = "Even Parity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPS_A {
    #[doc = "0: an odd parity is transmitted or checked"]
    ODD_PARITY = 0,
    #[doc = "1: an even parity is transmitted or checked"]
    EVEN_PARITY = 1,
}
impl From<EPS_A> for bool {
    #[inline(always)]
    fn from(variant: EPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPS` reader - Even Parity Select"]
pub type EPS_R = crate::BitReader<EPS_A>;
impl EPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPS_A {
        match self.bits {
            false => EPS_A::ODD_PARITY,
            true => EPS_A::EVEN_PARITY,
        }
    }
    #[doc = "Checks if the value of the field is `ODD_PARITY`"]
    #[inline(always)]
    pub fn is_odd_parity(&self) -> bool {
        *self == EPS_A::ODD_PARITY
    }
    #[doc = "Checks if the value of the field is `EVEN_PARITY`"]
    #[inline(always)]
    pub fn is_even_parity(&self) -> bool {
        *self == EPS_A::EVEN_PARITY
    }
}
#[doc = "Field `EPS` writer - Even Parity Select"]
pub type EPS_W<'a> = crate::BitWriter<'a, u32, UART_LCR_SPEC, EPS_A, 4>;
impl<'a> EPS_W<'a> {
    #[doc = "an odd parity is transmitted or checked"]
    #[inline(always)]
    pub fn odd_parity(self) -> &'a mut W {
        self.variant(EPS_A::ODD_PARITY)
    }
    #[doc = "an even parity is transmitted or checked"]
    #[inline(always)]
    pub fn even_parity(self) -> &'a mut W {
        self.variant(EPS_A::EVEN_PARITY)
    }
}
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<PEN_A> for bool {
    #[inline(always)]
    fn from(variant: PEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEN` reader - Parity Enable"]
pub type PEN_R = crate::BitReader<PEN_A>;
impl PEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN_A {
        match self.bits {
            false => PEN_A::DISABLE,
            true => PEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PEN_A::ENABLE
    }
}
#[doc = "Field `PEN` writer - Parity Enable"]
pub type PEN_W<'a> = crate::BitWriter<'a, u32, UART_LCR_SPEC, PEN_A, 3>;
impl<'a> PEN_W<'a> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PEN_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PEN_A::ENABLE)
    }
}
#[doc = "Number of stop bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_A {
    #[doc = "0: 1 stop bit"]
    STOP_1BIT = 0,
    #[doc = "1: 1.5 stop bits or 2 stop bits"]
    STOP_1_5BIT_OR_2BIT = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Number of stop bits"]
pub type STOP_R = crate::BitReader<STOP_A>;
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::STOP_1BIT,
            true => STOP_A::STOP_1_5BIT_OR_2BIT,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_1BIT`"]
    #[inline(always)]
    pub fn is_stop_1bit(&self) -> bool {
        *self == STOP_A::STOP_1BIT
    }
    #[doc = "Checks if the value of the field is `STOP_1_5BIT_OR_2BIT`"]
    #[inline(always)]
    pub fn is_stop_1_5bit_or_2bit(&self) -> bool {
        *self == STOP_A::STOP_1_5BIT_OR_2BIT
    }
}
#[doc = "Field `STOP` writer - Number of stop bits"]
pub type STOP_W<'a> = crate::BitWriter<'a, u32, UART_LCR_SPEC, STOP_A, 2>;
impl<'a> STOP_W<'a> {
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn stop_1bit(self) -> &'a mut W {
        self.variant(STOP_A::STOP_1BIT)
    }
    #[doc = "1.5 stop bits or 2 stop bits"]
    #[inline(always)]
    pub fn stop_1_5bit_or_2bit(self) -> &'a mut W {
        self.variant(STOP_A::STOP_1_5BIT_OR_2BIT)
    }
}
#[doc = "Data Length Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DLS_A {
    #[doc = "0: 5 data bits per character"]
    CHAR_5BITS = 0,
    #[doc = "1: 6 data bits per character"]
    CHAR_6BITS = 1,
    #[doc = "2: 7 data bits per character"]
    CHAR_7BITS = 2,
    #[doc = "3: 8 data bits per character"]
    CHAR_8BITS = 3,
}
impl From<DLS_A> for u8 {
    #[inline(always)]
    fn from(variant: DLS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DLS` reader - Data Length Select"]
pub type DLS_R = crate::FieldReader<u8, DLS_A>;
impl DLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLS_A {
        match self.bits {
            0 => DLS_A::CHAR_5BITS,
            1 => DLS_A::CHAR_6BITS,
            2 => DLS_A::CHAR_7BITS,
            3 => DLS_A::CHAR_8BITS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CHAR_5BITS`"]
    #[inline(always)]
    pub fn is_char_5bits(&self) -> bool {
        *self == DLS_A::CHAR_5BITS
    }
    #[doc = "Checks if the value of the field is `CHAR_6BITS`"]
    #[inline(always)]
    pub fn is_char_6bits(&self) -> bool {
        *self == DLS_A::CHAR_6BITS
    }
    #[doc = "Checks if the value of the field is `CHAR_7BITS`"]
    #[inline(always)]
    pub fn is_char_7bits(&self) -> bool {
        *self == DLS_A::CHAR_7BITS
    }
    #[doc = "Checks if the value of the field is `CHAR_8BITS`"]
    #[inline(always)]
    pub fn is_char_8bits(&self) -> bool {
        *self == DLS_A::CHAR_8BITS
    }
}
#[doc = "Field `DLS` writer - Data Length Select"]
pub type DLS_W<'a> = crate::FieldWriterSafe<'a, u32, UART_LCR_SPEC, u8, DLS_A, 2, 0>;
impl<'a> DLS_W<'a> {
    #[doc = "5 data bits per character"]
    #[inline(always)]
    pub fn char_5bits(self) -> &'a mut W {
        self.variant(DLS_A::CHAR_5BITS)
    }
    #[doc = "6 data bits per character"]
    #[inline(always)]
    pub fn char_6bits(self) -> &'a mut W {
        self.variant(DLS_A::CHAR_6BITS)
    }
    #[doc = "7 data bits per character"]
    #[inline(always)]
    pub fn char_7bits(self) -> &'a mut W {
        self.variant(DLS_A::CHAR_7BITS)
    }
    #[doc = "8 data bits per character"]
    #[inline(always)]
    pub fn char_8bits(self) -> &'a mut W {
        self.variant(DLS_A::CHAR_8BITS)
    }
}
impl R {
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&self) -> DLAB_R {
        DLAB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Break Control Bit"]
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Stick Parity"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Even Parity Select"]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Number of stop bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Data Length Select"]
    #[inline(always)]
    pub fn dls(&self) -> DLS_R {
        DLS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&mut self) -> DLAB_W {
        DLAB_W::new(self)
    }
    #[doc = "Bit 6 - Break Control Bit"]
    #[inline(always)]
    pub fn bc(&mut self) -> BC_W {
        BC_W::new(self)
    }
    #[doc = "Bit 5 - Stick Parity"]
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W {
        SP_W::new(self)
    }
    #[doc = "Bit 4 - Even Parity Select"]
    #[inline(always)]
    pub fn eps(&mut self) -> EPS_W {
        EPS_W::new(self)
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W::new(self)
    }
    #[doc = "Bit 2 - Number of stop bits"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W::new(self)
    }
    #[doc = "Bits 0:1 - Data Length Select"]
    #[inline(always)]
    pub fn dls(&mut self) -> DLS_W {
        DLS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_lcr](index.html) module"]
pub struct UART_LCR_SPEC;
impl crate::RegisterSpec for UART_LCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_lcr::R](R) reader structure"]
impl crate::Readable for UART_LCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_lcr::W](W) writer structure"]
impl crate::Writable for UART_LCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_LCR to value 0"]
impl crate::Resettable for UART_LCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
