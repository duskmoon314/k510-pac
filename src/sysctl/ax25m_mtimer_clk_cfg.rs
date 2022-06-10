#[doc = "Register `AX25M_MTIMER_CLK_CFG` reader"]
pub struct R(crate::R<AX25M_MTIMER_CLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AX25M_MTIMER_CLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AX25M_MTIMER_CLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AX25M_MTIMER_CLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AX25M_MTIMER_CLK_CFG` writer"]
pub struct W(crate::W<AX25M_MTIMER_CLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AX25M_MTIMER_CLK_CFG_SPEC>;
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
impl From<crate::W<AX25M_MTIMER_CLK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AX25M_MTIMER_CLK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_ax25m_mtimer_clk_en` reader - Write enable for bit 8 (ax25m_mtimer_clk_en)"]
pub type WE_AX25M_MTIMER_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `WE_ax25m_mtimer_clk_en` writer - Write enable for bit 8 (ax25m_mtimer_clk_en)"]
pub type WE_AX25M_MTIMER_CLK_EN_W<'a> =
    crate::BitWriter<'a, u32, AX25M_MTIMER_CLK_CFG_SPEC, bool, 26>;
#[doc = "Field `WE_ax25m_mtimerclk_div` reader - Write enable for bit \\[6:2\\]
(ax25m_mtimerclk_div)"]
pub type WE_AX25M_MTIMERCLK_DIV_R = crate::BitReader<bool>;
#[doc = "Field `WE_ax25m_mtimerclk_div` writer - Write enable for bit \\[6:2\\]
(ax25m_mtimerclk_div)"]
pub type WE_AX25M_MTIMERCLK_DIV_W<'a> =
    crate::BitWriter<'a, u32, AX25M_MTIMER_CLK_CFG_SPEC, bool, 25>;
#[doc = "Field `WE_ax25m_mtimerclk_sel` reader - Write enable for bit 0 (ax25m_mtimerclk_sel)"]
pub type WE_AX25M_MTIMERCLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `WE_ax25m_mtimerclk_sel` writer - Write enable for bit 0 (ax25m_mtimerclk_sel)"]
pub type WE_AX25M_MTIMERCLK_SEL_W<'a> =
    crate::BitWriter<'a, u32, AX25M_MTIMER_CLK_CFG_SPEC, bool, 24>;
#[doc = "AX25M Machine Timer clock enable control bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AX25M_MTIMER_CLK_EN_A {
    #[doc = "0: Disable ax25m Machine Timer clock"]
    DISABLE = 0,
    #[doc = "1: Enable ax25m Machine Timer clock"]
    ENABLE = 1,
}
impl From<AX25M_MTIMER_CLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AX25M_MTIMER_CLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ax25m_mtimer_clk_en` reader - AX25M Machine Timer clock enable control bit"]
pub type AX25M_MTIMER_CLK_EN_R = crate::BitReader<AX25M_MTIMER_CLK_EN_A>;
impl AX25M_MTIMER_CLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AX25M_MTIMER_CLK_EN_A {
        match self.bits {
            false => AX25M_MTIMER_CLK_EN_A::DISABLE,
            true => AX25M_MTIMER_CLK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AX25M_MTIMER_CLK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AX25M_MTIMER_CLK_EN_A::ENABLE
    }
}
#[doc = "Field `ax25m_mtimer_clk_en` writer - AX25M Machine Timer clock enable control bit"]
pub type AX25M_MTIMER_CLK_EN_W<'a> =
    crate::BitWriter<'a, u32, AX25M_MTIMER_CLK_CFG_SPEC, AX25M_MTIMER_CLK_EN_A, 8>;
impl<'a> AX25M_MTIMER_CLK_EN_W<'a> {
    #[doc = "Disable ax25m Machine Timer clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AX25M_MTIMER_CLK_EN_A::DISABLE)
    }
    #[doc = "Enable ax25m Machine Timer clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AX25M_MTIMER_CLK_EN_A::ENABLE)
    }
}
#[doc = "AX25M Machine Timer clock division ratio configure\n\nValue on reset: 19"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AX25M_MTIMERCLK_DIV_A {
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
impl From<AX25M_MTIMERCLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: AX25M_MTIMERCLK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ax25m_mtimerclk_div` reader - AX25M Machine Timer clock division ratio configure"]
pub type AX25M_MTIMERCLK_DIV_R = crate::FieldReader<u8, AX25M_MTIMERCLK_DIV_A>;
impl AX25M_MTIMERCLK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AX25M_MTIMERCLK_DIV_A {
        match self.bits {
            0 => AX25M_MTIMERCLK_DIV_A::D1,
            1 => AX25M_MTIMERCLK_DIV_A::D2,
            2 => AX25M_MTIMERCLK_DIV_A::D3,
            3 => AX25M_MTIMERCLK_DIV_A::D4,
            4 => AX25M_MTIMERCLK_DIV_A::D5,
            5 => AX25M_MTIMERCLK_DIV_A::D6,
            6 => AX25M_MTIMERCLK_DIV_A::D7,
            7 => AX25M_MTIMERCLK_DIV_A::D8,
            8 => AX25M_MTIMERCLK_DIV_A::D9,
            9 => AX25M_MTIMERCLK_DIV_A::D10,
            10 => AX25M_MTIMERCLK_DIV_A::D11,
            11 => AX25M_MTIMERCLK_DIV_A::D12,
            12 => AX25M_MTIMERCLK_DIV_A::D13,
            13 => AX25M_MTIMERCLK_DIV_A::D14,
            14 => AX25M_MTIMERCLK_DIV_A::D15,
            15 => AX25M_MTIMERCLK_DIV_A::D16,
            16 => AX25M_MTIMERCLK_DIV_A::D17,
            17 => AX25M_MTIMERCLK_DIV_A::D18,
            18 => AX25M_MTIMERCLK_DIV_A::D19,
            19 => AX25M_MTIMERCLK_DIV_A::D20,
            20 => AX25M_MTIMERCLK_DIV_A::D21,
            21 => AX25M_MTIMERCLK_DIV_A::D22,
            22 => AX25M_MTIMERCLK_DIV_A::D23,
            23 => AX25M_MTIMERCLK_DIV_A::D24,
            24 => AX25M_MTIMERCLK_DIV_A::D25,
            25 => AX25M_MTIMERCLK_DIV_A::D26,
            26 => AX25M_MTIMERCLK_DIV_A::D27,
            27 => AX25M_MTIMERCLK_DIV_A::D28,
            28 => AX25M_MTIMERCLK_DIV_A::D29,
            29 => AX25M_MTIMERCLK_DIV_A::D30,
            30 => AX25M_MTIMERCLK_DIV_A::D31,
            31 => AX25M_MTIMERCLK_DIV_A::D32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D1`"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D1
    }
    #[doc = "Checks if the value of the field is `D2`"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D2
    }
    #[doc = "Checks if the value of the field is `D3`"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D3
    }
    #[doc = "Checks if the value of the field is `D4`"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D4
    }
    #[doc = "Checks if the value of the field is `D5`"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D5
    }
    #[doc = "Checks if the value of the field is `D6`"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D6
    }
    #[doc = "Checks if the value of the field is `D7`"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D7
    }
    #[doc = "Checks if the value of the field is `D8`"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D8
    }
    #[doc = "Checks if the value of the field is `D9`"]
    #[inline(always)]
    pub fn is_d9(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D9
    }
    #[doc = "Checks if the value of the field is `D10`"]
    #[inline(always)]
    pub fn is_d10(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D10
    }
    #[doc = "Checks if the value of the field is `D11`"]
    #[inline(always)]
    pub fn is_d11(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D11
    }
    #[doc = "Checks if the value of the field is `D12`"]
    #[inline(always)]
    pub fn is_d12(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D12
    }
    #[doc = "Checks if the value of the field is `D13`"]
    #[inline(always)]
    pub fn is_d13(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D13
    }
    #[doc = "Checks if the value of the field is `D14`"]
    #[inline(always)]
    pub fn is_d14(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D14
    }
    #[doc = "Checks if the value of the field is `D15`"]
    #[inline(always)]
    pub fn is_d15(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D15
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D16
    }
    #[doc = "Checks if the value of the field is `D17`"]
    #[inline(always)]
    pub fn is_d17(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D17
    }
    #[doc = "Checks if the value of the field is `D18`"]
    #[inline(always)]
    pub fn is_d18(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D18
    }
    #[doc = "Checks if the value of the field is `D19`"]
    #[inline(always)]
    pub fn is_d19(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D19
    }
    #[doc = "Checks if the value of the field is `D20`"]
    #[inline(always)]
    pub fn is_d20(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D20
    }
    #[doc = "Checks if the value of the field is `D21`"]
    #[inline(always)]
    pub fn is_d21(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D21
    }
    #[doc = "Checks if the value of the field is `D22`"]
    #[inline(always)]
    pub fn is_d22(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D22
    }
    #[doc = "Checks if the value of the field is `D23`"]
    #[inline(always)]
    pub fn is_d23(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D23
    }
    #[doc = "Checks if the value of the field is `D24`"]
    #[inline(always)]
    pub fn is_d24(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D24
    }
    #[doc = "Checks if the value of the field is `D25`"]
    #[inline(always)]
    pub fn is_d25(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D25
    }
    #[doc = "Checks if the value of the field is `D26`"]
    #[inline(always)]
    pub fn is_d26(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D26
    }
    #[doc = "Checks if the value of the field is `D27`"]
    #[inline(always)]
    pub fn is_d27(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D27
    }
    #[doc = "Checks if the value of the field is `D28`"]
    #[inline(always)]
    pub fn is_d28(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D28
    }
    #[doc = "Checks if the value of the field is `D29`"]
    #[inline(always)]
    pub fn is_d29(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D29
    }
    #[doc = "Checks if the value of the field is `D30`"]
    #[inline(always)]
    pub fn is_d30(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D30
    }
    #[doc = "Checks if the value of the field is `D31`"]
    #[inline(always)]
    pub fn is_d31(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D31
    }
    #[doc = "Checks if the value of the field is `D32`"]
    #[inline(always)]
    pub fn is_d32(&self) -> bool {
        *self == AX25M_MTIMERCLK_DIV_A::D32
    }
}
#[doc = "Field `ax25m_mtimerclk_div` writer - AX25M Machine Timer clock division ratio configure"]
pub type AX25M_MTIMERCLK_DIV_W<'a> =
    crate::FieldWriterSafe<'a, u32, AX25M_MTIMER_CLK_CFG_SPEC, u8, AX25M_MTIMERCLK_DIV_A, 5, 2>;
impl<'a> AX25M_MTIMERCLK_DIV_W<'a> {
    #[doc = "no division"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D1)
    }
    #[doc = "1/2 division"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D2)
    }
    #[doc = "1/3 division"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D3)
    }
    #[doc = "1/4 division"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D4)
    }
    #[doc = "1/5 division"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D5)
    }
    #[doc = "1/6 division"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D6)
    }
    #[doc = "1/7 division"]
    #[inline(always)]
    pub fn d7(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D7)
    }
    #[doc = "1/8 division"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D8)
    }
    #[doc = "1/9 division"]
    #[inline(always)]
    pub fn d9(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D9)
    }
    #[doc = "1/10 division"]
    #[inline(always)]
    pub fn d10(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D10)
    }
    #[doc = "1/11 division"]
    #[inline(always)]
    pub fn d11(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D11)
    }
    #[doc = "1/12 division"]
    #[inline(always)]
    pub fn d12(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D12)
    }
    #[doc = "1/13 division"]
    #[inline(always)]
    pub fn d13(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D13)
    }
    #[doc = "1/14 division"]
    #[inline(always)]
    pub fn d14(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D14)
    }
    #[doc = "1/15 division"]
    #[inline(always)]
    pub fn d15(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D15)
    }
    #[doc = "1/16 division"]
    #[inline(always)]
    pub fn d16(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D16)
    }
    #[doc = "1/17 division"]
    #[inline(always)]
    pub fn d17(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D17)
    }
    #[doc = "1/18 division"]
    #[inline(always)]
    pub fn d18(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D18)
    }
    #[doc = "1/19 division"]
    #[inline(always)]
    pub fn d19(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D19)
    }
    #[doc = "1/20 division"]
    #[inline(always)]
    pub fn d20(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D20)
    }
    #[doc = "1/21 division"]
    #[inline(always)]
    pub fn d21(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D21)
    }
    #[doc = "1/22 division"]
    #[inline(always)]
    pub fn d22(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D22)
    }
    #[doc = "1/23 division"]
    #[inline(always)]
    pub fn d23(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D23)
    }
    #[doc = "1/24 division"]
    #[inline(always)]
    pub fn d24(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D24)
    }
    #[doc = "1/25 division"]
    #[inline(always)]
    pub fn d25(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D25)
    }
    #[doc = "1/26 division"]
    #[inline(always)]
    pub fn d26(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D26)
    }
    #[doc = "1/27 division"]
    #[inline(always)]
    pub fn d27(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D27)
    }
    #[doc = "1/28 division"]
    #[inline(always)]
    pub fn d28(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D28)
    }
    #[doc = "1/29 division"]
    #[inline(always)]
    pub fn d29(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D29)
    }
    #[doc = "1/30 division"]
    #[inline(always)]
    pub fn d30(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D30)
    }
    #[doc = "1/31 division"]
    #[inline(always)]
    pub fn d31(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D31)
    }
    #[doc = "1/32 division"]
    #[inline(always)]
    pub fn d32(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_DIV_A::D32)
    }
}
#[doc = "AX25M Machine Timer clock source selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AX25M_MTIMERCLK_SEL_A {
    #[doc = "0: 32KHz OSC"]
    OSC_32KHZ = 0,
    #[doc = "1: 25MHz OSC"]
    OSC_25MHZ = 1,
}
impl From<AX25M_MTIMERCLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: AX25M_MTIMERCLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ax25m_mtimerclk_sel` reader - AX25M Machine Timer clock source selection"]
pub type AX25M_MTIMERCLK_SEL_R = crate::BitReader<AX25M_MTIMERCLK_SEL_A>;
impl AX25M_MTIMERCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AX25M_MTIMERCLK_SEL_A {
        match self.bits {
            false => AX25M_MTIMERCLK_SEL_A::OSC_32KHZ,
            true => AX25M_MTIMERCLK_SEL_A::OSC_25MHZ,
        }
    }
    #[doc = "Checks if the value of the field is `OSC_32KHZ`"]
    #[inline(always)]
    pub fn is_osc_32khz(&self) -> bool {
        *self == AX25M_MTIMERCLK_SEL_A::OSC_32KHZ
    }
    #[doc = "Checks if the value of the field is `OSC_25MHZ`"]
    #[inline(always)]
    pub fn is_osc_25mhz(&self) -> bool {
        *self == AX25M_MTIMERCLK_SEL_A::OSC_25MHZ
    }
}
#[doc = "Field `ax25m_mtimerclk_sel` writer - AX25M Machine Timer clock source selection"]
pub type AX25M_MTIMERCLK_SEL_W<'a> =
    crate::BitWriter<'a, u32, AX25M_MTIMER_CLK_CFG_SPEC, AX25M_MTIMERCLK_SEL_A, 0>;
impl<'a> AX25M_MTIMERCLK_SEL_W<'a> {
    #[doc = "32KHz OSC"]
    #[inline(always)]
    pub fn osc_32khz(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_SEL_A::OSC_32KHZ)
    }
    #[doc = "25MHz OSC"]
    #[inline(always)]
    pub fn osc_25mhz(self) -> &'a mut W {
        self.variant(AX25M_MTIMERCLK_SEL_A::OSC_25MHZ)
    }
}
impl R {
    #[doc = "Bit 26 - Write enable for bit 8 (ax25m_mtimer_clk_en)"]
    #[inline(always)]
    pub fn we_ax25m_mtimer_clk_en(&self) -> WE_AX25M_MTIMER_CLK_EN_R {
        WE_AX25M_MTIMER_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - Write enable for bit \\[6:2\\]
(ax25m_mtimerclk_div)"]
    #[inline(always)]
    pub fn we_ax25m_mtimerclk_div(&self) -> WE_AX25M_MTIMERCLK_DIV_R {
        WE_AX25M_MTIMERCLK_DIV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - Write enable for bit 0 (ax25m_mtimerclk_sel)"]
    #[inline(always)]
    pub fn we_ax25m_mtimerclk_sel(&self) -> WE_AX25M_MTIMERCLK_SEL_R {
        WE_AX25M_MTIMERCLK_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 8 - AX25M Machine Timer clock enable control bit"]
    #[inline(always)]
    pub fn ax25m_mtimer_clk_en(&self) -> AX25M_MTIMER_CLK_EN_R {
        AX25M_MTIMER_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 2:6 - AX25M Machine Timer clock division ratio configure"]
    #[inline(always)]
    pub fn ax25m_mtimerclk_div(&self) -> AX25M_MTIMERCLK_DIV_R {
        AX25M_MTIMERCLK_DIV_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - AX25M Machine Timer clock source selection"]
    #[inline(always)]
    pub fn ax25m_mtimerclk_sel(&self) -> AX25M_MTIMERCLK_SEL_R {
        AX25M_MTIMERCLK_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - Write enable for bit 8 (ax25m_mtimer_clk_en)"]
    #[inline(always)]
    pub fn we_ax25m_mtimer_clk_en(&mut self) -> WE_AX25M_MTIMER_CLK_EN_W {
        WE_AX25M_MTIMER_CLK_EN_W::new(self)
    }
    #[doc = "Bit 25 - Write enable for bit \\[6:2\\]
(ax25m_mtimerclk_div)"]
    #[inline(always)]
    pub fn we_ax25m_mtimerclk_div(&mut self) -> WE_AX25M_MTIMERCLK_DIV_W {
        WE_AX25M_MTIMERCLK_DIV_W::new(self)
    }
    #[doc = "Bit 24 - Write enable for bit 0 (ax25m_mtimerclk_sel)"]
    #[inline(always)]
    pub fn we_ax25m_mtimerclk_sel(&mut self) -> WE_AX25M_MTIMERCLK_SEL_W {
        WE_AX25M_MTIMERCLK_SEL_W::new(self)
    }
    #[doc = "Bit 8 - AX25M Machine Timer clock enable control bit"]
    #[inline(always)]
    pub fn ax25m_mtimer_clk_en(&mut self) -> AX25M_MTIMER_CLK_EN_W {
        AX25M_MTIMER_CLK_EN_W::new(self)
    }
    #[doc = "Bits 2:6 - AX25M Machine Timer clock division ratio configure"]
    #[inline(always)]
    pub fn ax25m_mtimerclk_div(&mut self) -> AX25M_MTIMERCLK_DIV_W {
        AX25M_MTIMERCLK_DIV_W::new(self)
    }
    #[doc = "Bit 0 - AX25M Machine Timer clock source selection"]
    #[inline(always)]
    pub fn ax25m_mtimerclk_sel(&mut self) -> AX25M_MTIMERCLK_SEL_W {
        AX25M_MTIMERCLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AX25M dual-core RISCV core Machine Timer clock Division configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ax25m_mtimer_clk_cfg](index.html) module"]
pub struct AX25M_MTIMER_CLK_CFG_SPEC;
impl crate::RegisterSpec for AX25M_MTIMER_CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ax25m_mtimer_clk_cfg::R](R) reader structure"]
impl crate::Readable for AX25M_MTIMER_CLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ax25m_mtimer_clk_cfg::W](W) writer structure"]
impl crate::Writable for AX25M_MTIMER_CLK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AX25M_MTIMER_CLK_CFG to value 0x014d"]
impl crate::Resettable for AX25M_MTIMER_CLK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x014d
    }
}
