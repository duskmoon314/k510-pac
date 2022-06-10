#[doc = "Register `UART%s_SCLK_CFG` reader"]
pub struct R(crate::R<UART_SCLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SCLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SCLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SCLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART%s_SCLK_CFG` writer"]
pub struct W(crate::W<UART_SCLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SCLK_CFG_SPEC>;
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
impl From<crate::W<UART_SCLK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SCLK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_uart_sclk_en` writer - Write enable for bit 12 (uart_sclk_en)"]
pub type WE_UART_SCLK_EN_W<'a> = crate::BitWriter<'a, u32, UART_SCLK_CFG_SPEC, bool, 26>;
#[doc = "Field `WE_uart_sclk_div` writer - Write enable for bit \\[8:4\\]
(uart_sclk_div)"]
pub type WE_UART_SCLK_DIV_W<'a> = crate::BitWriter<'a, u32, UART_SCLK_CFG_SPEC, bool, 25>;
#[doc = "Field `WE_uart_sclk_sel` writer - Write enable for bit 0 (uart_sclk_sel)"]
pub type WE_UART_SCLK_SEL_W<'a> = crate::BitWriter<'a, u32, UART_SCLK_CFG_SPEC, bool, 24>;
#[doc = "UART\\[i\\]
host module serial clock enable control bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_SCLK_EN_AW {
    #[doc = "0: disable uart_sclk_output"]
    DISABLE = 0,
    #[doc = "1: enable uart_sclk_output"]
    ENABLE = 1,
}
impl From<UART_SCLK_EN_AW> for bool {
    #[inline(always)]
    fn from(variant: UART_SCLK_EN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `uart_sclk_en` writer - UART\\[i\\]
host module serial clock enable control bit"]
pub type UART_SCLK_EN_W<'a> = crate::BitWriter<'a, u32, UART_SCLK_CFG_SPEC, UART_SCLK_EN_AW, 12>;
impl<'a> UART_SCLK_EN_W<'a> {
    #[doc = "disable uart_sclk_output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART_SCLK_EN_AW::DISABLE)
    }
    #[doc = "enable uart_sclk_output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART_SCLK_EN_AW::ENABLE)
    }
}
#[doc = "UART\\[i\\]
host module serial clock division ratio configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART_SCLK_DIV_A {
    #[doc = "0: no division"]
    D1 = 0,
    #[doc = "1: 1/2 division"]
    D2 = 1,
    #[doc = "2: 1/3 division"]
    D3 = 2,
    #[doc = "3: 1/4 division"]
    D4 = 3,
    #[doc = "4: 1/5 division"]
    D5 = 4,
    #[doc = "5: 1/6 division"]
    D6 = 5,
    #[doc = "6: 1/7 division"]
    D7 = 6,
    #[doc = "7: 1/8 division"]
    D8 = 7,
    #[doc = "8: 1/9 division"]
    D9 = 8,
    #[doc = "9: 1/10 division"]
    D10 = 9,
    #[doc = "10: 1/11 division"]
    D11 = 10,
    #[doc = "11: 1/12 division"]
    D12 = 11,
    #[doc = "12: 1/13 division"]
    D13 = 12,
    #[doc = "13: 1/14 division"]
    D14 = 13,
    #[doc = "14: 1/15 division"]
    D15 = 14,
    #[doc = "15: 1/16 division"]
    D16 = 15,
    #[doc = "16: 1/17 division"]
    D17 = 16,
    #[doc = "17: 1/18 division"]
    D18 = 17,
    #[doc = "18: 1/19 division"]
    D19 = 18,
    #[doc = "19: 1/20 division"]
    D20 = 19,
    #[doc = "20: 1/21 division"]
    D21 = 20,
    #[doc = "21: 1/22 division"]
    D22 = 21,
    #[doc = "22: 1/23 division"]
    D23 = 22,
    #[doc = "23: 1/24 division"]
    D24 = 23,
    #[doc = "24: 1/25 division"]
    D25 = 24,
    #[doc = "25: 1/26 division"]
    D26 = 25,
    #[doc = "26: 1/27 division"]
    D27 = 26,
    #[doc = "27: 1/28 division"]
    D28 = 27,
    #[doc = "28: 1/29 division"]
    D29 = 28,
    #[doc = "29: 1/30 division"]
    D30 = 29,
    #[doc = "30: 1/31 division"]
    D31 = 30,
    #[doc = "31: 1/32 division"]
    D32 = 31,
}
impl From<UART_SCLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: UART_SCLK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `uart_sclk_div` reader - UART\\[i\\]
host module serial clock division ratio configure"]
pub type UART_SCLK_DIV_R = crate::FieldReader<u8, UART_SCLK_DIV_A>;
impl UART_SCLK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART_SCLK_DIV_A {
        match self.bits {
            0 => UART_SCLK_DIV_A::D1,
            1 => UART_SCLK_DIV_A::D2,
            2 => UART_SCLK_DIV_A::D3,
            3 => UART_SCLK_DIV_A::D4,
            4 => UART_SCLK_DIV_A::D5,
            5 => UART_SCLK_DIV_A::D6,
            6 => UART_SCLK_DIV_A::D7,
            7 => UART_SCLK_DIV_A::D8,
            8 => UART_SCLK_DIV_A::D9,
            9 => UART_SCLK_DIV_A::D10,
            10 => UART_SCLK_DIV_A::D11,
            11 => UART_SCLK_DIV_A::D12,
            12 => UART_SCLK_DIV_A::D13,
            13 => UART_SCLK_DIV_A::D14,
            14 => UART_SCLK_DIV_A::D15,
            15 => UART_SCLK_DIV_A::D16,
            16 => UART_SCLK_DIV_A::D17,
            17 => UART_SCLK_DIV_A::D18,
            18 => UART_SCLK_DIV_A::D19,
            19 => UART_SCLK_DIV_A::D20,
            20 => UART_SCLK_DIV_A::D21,
            21 => UART_SCLK_DIV_A::D22,
            22 => UART_SCLK_DIV_A::D23,
            23 => UART_SCLK_DIV_A::D24,
            24 => UART_SCLK_DIV_A::D25,
            25 => UART_SCLK_DIV_A::D26,
            26 => UART_SCLK_DIV_A::D27,
            27 => UART_SCLK_DIV_A::D28,
            28 => UART_SCLK_DIV_A::D29,
            29 => UART_SCLK_DIV_A::D30,
            30 => UART_SCLK_DIV_A::D31,
            31 => UART_SCLK_DIV_A::D32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D1`"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == UART_SCLK_DIV_A::D1
    }
    #[doc = "Checks if the value of the field is `D2`"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == UART_SCLK_DIV_A::D2
    }
    #[doc = "Checks if the value of the field is `D3`"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == UART_SCLK_DIV_A::D3
    }
    #[doc = "Checks if the value of the field is `D4`"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == UART_SCLK_DIV_A::D4
    }
    #[doc = "Checks if the value of the field is `D5`"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == UART_SCLK_DIV_A::D5
    }
    #[doc = "Checks if the value of the field is `D6`"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == UART_SCLK_DIV_A::D6
    }
    #[doc = "Checks if the value of the field is `D7`"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == UART_SCLK_DIV_A::D7
    }
    #[doc = "Checks if the value of the field is `D8`"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == UART_SCLK_DIV_A::D8
    }
    #[doc = "Checks if the value of the field is `D9`"]
    #[inline(always)]
    pub fn is_d9(&self) -> bool {
        *self == UART_SCLK_DIV_A::D9
    }
    #[doc = "Checks if the value of the field is `D10`"]
    #[inline(always)]
    pub fn is_d10(&self) -> bool {
        *self == UART_SCLK_DIV_A::D10
    }
    #[doc = "Checks if the value of the field is `D11`"]
    #[inline(always)]
    pub fn is_d11(&self) -> bool {
        *self == UART_SCLK_DIV_A::D11
    }
    #[doc = "Checks if the value of the field is `D12`"]
    #[inline(always)]
    pub fn is_d12(&self) -> bool {
        *self == UART_SCLK_DIV_A::D12
    }
    #[doc = "Checks if the value of the field is `D13`"]
    #[inline(always)]
    pub fn is_d13(&self) -> bool {
        *self == UART_SCLK_DIV_A::D13
    }
    #[doc = "Checks if the value of the field is `D14`"]
    #[inline(always)]
    pub fn is_d14(&self) -> bool {
        *self == UART_SCLK_DIV_A::D14
    }
    #[doc = "Checks if the value of the field is `D15`"]
    #[inline(always)]
    pub fn is_d15(&self) -> bool {
        *self == UART_SCLK_DIV_A::D15
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == UART_SCLK_DIV_A::D16
    }
    #[doc = "Checks if the value of the field is `D17`"]
    #[inline(always)]
    pub fn is_d17(&self) -> bool {
        *self == UART_SCLK_DIV_A::D17
    }
    #[doc = "Checks if the value of the field is `D18`"]
    #[inline(always)]
    pub fn is_d18(&self) -> bool {
        *self == UART_SCLK_DIV_A::D18
    }
    #[doc = "Checks if the value of the field is `D19`"]
    #[inline(always)]
    pub fn is_d19(&self) -> bool {
        *self == UART_SCLK_DIV_A::D19
    }
    #[doc = "Checks if the value of the field is `D20`"]
    #[inline(always)]
    pub fn is_d20(&self) -> bool {
        *self == UART_SCLK_DIV_A::D20
    }
    #[doc = "Checks if the value of the field is `D21`"]
    #[inline(always)]
    pub fn is_d21(&self) -> bool {
        *self == UART_SCLK_DIV_A::D21
    }
    #[doc = "Checks if the value of the field is `D22`"]
    #[inline(always)]
    pub fn is_d22(&self) -> bool {
        *self == UART_SCLK_DIV_A::D22
    }
    #[doc = "Checks if the value of the field is `D23`"]
    #[inline(always)]
    pub fn is_d23(&self) -> bool {
        *self == UART_SCLK_DIV_A::D23
    }
    #[doc = "Checks if the value of the field is `D24`"]
    #[inline(always)]
    pub fn is_d24(&self) -> bool {
        *self == UART_SCLK_DIV_A::D24
    }
    #[doc = "Checks if the value of the field is `D25`"]
    #[inline(always)]
    pub fn is_d25(&self) -> bool {
        *self == UART_SCLK_DIV_A::D25
    }
    #[doc = "Checks if the value of the field is `D26`"]
    #[inline(always)]
    pub fn is_d26(&self) -> bool {
        *self == UART_SCLK_DIV_A::D26
    }
    #[doc = "Checks if the value of the field is `D27`"]
    #[inline(always)]
    pub fn is_d27(&self) -> bool {
        *self == UART_SCLK_DIV_A::D27
    }
    #[doc = "Checks if the value of the field is `D28`"]
    #[inline(always)]
    pub fn is_d28(&self) -> bool {
        *self == UART_SCLK_DIV_A::D28
    }
    #[doc = "Checks if the value of the field is `D29`"]
    #[inline(always)]
    pub fn is_d29(&self) -> bool {
        *self == UART_SCLK_DIV_A::D29
    }
    #[doc = "Checks if the value of the field is `D30`"]
    #[inline(always)]
    pub fn is_d30(&self) -> bool {
        *self == UART_SCLK_DIV_A::D30
    }
    #[doc = "Checks if the value of the field is `D31`"]
    #[inline(always)]
    pub fn is_d31(&self) -> bool {
        *self == UART_SCLK_DIV_A::D31
    }
    #[doc = "Checks if the value of the field is `D32`"]
    #[inline(always)]
    pub fn is_d32(&self) -> bool {
        *self == UART_SCLK_DIV_A::D32
    }
}
#[doc = "Field `uart_sclk_div` writer - UART\\[i\\]
host module serial clock division ratio configure"]
pub type UART_SCLK_DIV_W<'a> =
    crate::FieldWriterSafe<'a, u32, UART_SCLK_CFG_SPEC, u8, UART_SCLK_DIV_A, 5, 4>;
impl<'a> UART_SCLK_DIV_W<'a> {
    #[doc = "no division"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D1)
    }
    #[doc = "1/2 division"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D2)
    }
    #[doc = "1/3 division"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D3)
    }
    #[doc = "1/4 division"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D4)
    }
    #[doc = "1/5 division"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D5)
    }
    #[doc = "1/6 division"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D6)
    }
    #[doc = "1/7 division"]
    #[inline(always)]
    pub fn d7(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D7)
    }
    #[doc = "1/8 division"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D8)
    }
    #[doc = "1/9 division"]
    #[inline(always)]
    pub fn d9(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D9)
    }
    #[doc = "1/10 division"]
    #[inline(always)]
    pub fn d10(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D10)
    }
    #[doc = "1/11 division"]
    #[inline(always)]
    pub fn d11(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D11)
    }
    #[doc = "1/12 division"]
    #[inline(always)]
    pub fn d12(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D12)
    }
    #[doc = "1/13 division"]
    #[inline(always)]
    pub fn d13(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D13)
    }
    #[doc = "1/14 division"]
    #[inline(always)]
    pub fn d14(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D14)
    }
    #[doc = "1/15 division"]
    #[inline(always)]
    pub fn d15(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D15)
    }
    #[doc = "1/16 division"]
    #[inline(always)]
    pub fn d16(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D16)
    }
    #[doc = "1/17 division"]
    #[inline(always)]
    pub fn d17(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D17)
    }
    #[doc = "1/18 division"]
    #[inline(always)]
    pub fn d18(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D18)
    }
    #[doc = "1/19 division"]
    #[inline(always)]
    pub fn d19(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D19)
    }
    #[doc = "1/20 division"]
    #[inline(always)]
    pub fn d20(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D20)
    }
    #[doc = "1/21 division"]
    #[inline(always)]
    pub fn d21(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D21)
    }
    #[doc = "1/22 division"]
    #[inline(always)]
    pub fn d22(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D22)
    }
    #[doc = "1/23 division"]
    #[inline(always)]
    pub fn d23(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D23)
    }
    #[doc = "1/24 division"]
    #[inline(always)]
    pub fn d24(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D24)
    }
    #[doc = "1/25 division"]
    #[inline(always)]
    pub fn d25(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D25)
    }
    #[doc = "1/26 division"]
    #[inline(always)]
    pub fn d26(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D26)
    }
    #[doc = "1/27 division"]
    #[inline(always)]
    pub fn d27(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D27)
    }
    #[doc = "1/28 division"]
    #[inline(always)]
    pub fn d28(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D28)
    }
    #[doc = "1/29 division"]
    #[inline(always)]
    pub fn d29(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D29)
    }
    #[doc = "1/30 division"]
    #[inline(always)]
    pub fn d30(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D30)
    }
    #[doc = "1/31 division"]
    #[inline(always)]
    pub fn d31(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D31)
    }
    #[doc = "1/32 division"]
    #[inline(always)]
    pub fn d32(self) -> &'a mut W {
        self.variant(UART_SCLK_DIV_A::D32)
    }
}
#[doc = "UART\\[i\\]
host module serial clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_SCLK_SEL_A {
    #[doc = "0: 25MHz clock from oscillator"]
    OSCILLATOR = 0,
    #[doc = "1: 25MHz clock from PLL"]
    PLL = 1,
}
impl From<UART_SCLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: UART_SCLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `uart_sclk_sel` reader - UART\\[i\\]
host module serial clock source selection"]
pub type UART_SCLK_SEL_R = crate::BitReader<UART_SCLK_SEL_A>;
impl UART_SCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART_SCLK_SEL_A {
        match self.bits {
            false => UART_SCLK_SEL_A::OSCILLATOR,
            true => UART_SCLK_SEL_A::PLL,
        }
    }
    #[doc = "Checks if the value of the field is `OSCILLATOR`"]
    #[inline(always)]
    pub fn is_oscillator(&self) -> bool {
        *self == UART_SCLK_SEL_A::OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == UART_SCLK_SEL_A::PLL
    }
}
#[doc = "Field `uart_sclk_sel` writer - UART\\[i\\]
host module serial clock source selection"]
pub type UART_SCLK_SEL_W<'a> = crate::BitWriter<'a, u32, UART_SCLK_CFG_SPEC, UART_SCLK_SEL_A, 0>;
impl<'a> UART_SCLK_SEL_W<'a> {
    #[doc = "25MHz clock from oscillator"]
    #[inline(always)]
    pub fn oscillator(self) -> &'a mut W {
        self.variant(UART_SCLK_SEL_A::OSCILLATOR)
    }
    #[doc = "25MHz clock from PLL"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(UART_SCLK_SEL_A::PLL)
    }
}
impl R {
    #[doc = "Bits 4:8 - UART\\[i\\]
host module serial clock division ratio configure"]
    #[inline(always)]
    pub fn uart_sclk_div(&self) -> UART_SCLK_DIV_R {
        UART_SCLK_DIV_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - UART\\[i\\]
host module serial clock source selection"]
    #[inline(always)]
    pub fn uart_sclk_sel(&self) -> UART_SCLK_SEL_R {
        UART_SCLK_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - Write enable for bit 12 (uart_sclk_en)"]
    #[inline(always)]
    pub fn we_uart_sclk_en(&mut self) -> WE_UART_SCLK_EN_W {
        WE_UART_SCLK_EN_W::new(self)
    }
    #[doc = "Bit 25 - Write enable for bit \\[8:4\\]
(uart_sclk_div)"]
    #[inline(always)]
    pub fn we_uart_sclk_div(&mut self) -> WE_UART_SCLK_DIV_W {
        WE_UART_SCLK_DIV_W::new(self)
    }
    #[doc = "Bit 24 - Write enable for bit 0 (uart_sclk_sel)"]
    #[inline(always)]
    pub fn we_uart_sclk_sel(&mut self) -> WE_UART_SCLK_SEL_W {
        WE_UART_SCLK_SEL_W::new(self)
    }
    #[doc = "Bit 12 - UART\\[i\\]
host module serial clock enable control bit"]
    #[inline(always)]
    pub fn uart_sclk_en(&mut self) -> UART_SCLK_EN_W {
        UART_SCLK_EN_W::new(self)
    }
    #[doc = "Bits 4:8 - UART\\[i\\]
host module serial clock division ratio configure"]
    #[inline(always)]
    pub fn uart_sclk_div(&mut self) -> UART_SCLK_DIV_W {
        UART_SCLK_DIV_W::new(self)
    }
    #[doc = "Bit 0 - UART\\[i\\]
host module serial clock source selection"]
    #[inline(always)]
    pub fn uart_sclk_sel(&mut self) -> UART_SCLK_SEL_W {
        UART_SCLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART\\[i\\]
host module serial interface clock configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_sclk_cfg](index.html) module"]
pub struct UART_SCLK_CFG_SPEC;
impl crate::RegisterSpec for UART_SCLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_sclk_cfg::R](R) reader structure"]
impl crate::Readable for UART_SCLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_sclk_cfg::W](W) writer structure"]
impl crate::Writable for UART_SCLK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART%s_SCLK_CFG to value 0x1000"]
impl crate::Resettable for UART_SCLK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
