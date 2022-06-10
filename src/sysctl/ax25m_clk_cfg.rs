#[doc = "Register `AX25M_CLK_CFG` reader"]
pub struct R(crate::R<AX25M_CLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AX25M_CLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AX25M_CLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AX25M_CLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AX25M_CLK_CFG` writer"]
pub struct W(crate::W<AX25M_CLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AX25M_CLK_CFG_SPEC>;
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
impl From<crate::W<AX25M_CLK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AX25M_CLK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fields `WE_ax25m_dcclk(0-1)_en` writer - Write enable for bit 10/11 (ax25m_dcclk\\[01\\]_en)"]
pub type WE_AX25M_DCCLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AX25M_CLK_CFG_SPEC, bool, O>;
#[doc = "Fields `WE_ax25m_cclk(0-1)_en` writer - Write enable for bit 8/9 (ax25m_cclk\\[01\\]_en)"]
pub type WE_AX25M_CCLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AX25M_CLK_CFG_SPEC, bool, O>;
#[doc = "Field `WE_ax25m_cclk_div` writer - Write enable for bit \\[7:4\\]
(ax25m_cclk_div)"]
pub type WE_AX25M_CCLK_DIV_W<'a> = crate::BitWriter<'a, u32, AX25M_CLK_CFG_SPEC, bool, 25>;
#[doc = "Field `WE_ax25m_cclk_sel` writer - Write enable for bit \\[1:0\\]
(ax25m_cclk_sel)"]
pub type WE_AX25M_CCLK_SEL_W<'a> = crate::BitWriter<'a, u32, AX25M_CLK_CFG_SPEC, bool, 24>;
#[doc = "AX25M CPU dc clock enable control bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AX25M_DCCLK_EN_A {
    #[doc = "0: Disable CPU core clock output"]
    DISABLE = 0,
    #[doc = "1: Enable CPU core clock output"]
    ENABLE = 1,
}
impl From<AX25M_DCCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AX25M_DCCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `ax25m_dcclk(0-1)_en` reader - AX25M CPU dc clock enable control bit"]
pub type AX25M_DCCLK_EN_R = crate::BitReader<AX25M_DCCLK_EN_A>;
impl AX25M_DCCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AX25M_DCCLK_EN_A {
        match self.bits {
            false => AX25M_DCCLK_EN_A::DISABLE,
            true => AX25M_DCCLK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AX25M_DCCLK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AX25M_DCCLK_EN_A::ENABLE
    }
}
#[doc = "Fields `ax25m_dcclk(0-1)_en` writer - AX25M CPU dc clock enable control bit"]
pub type AX25M_DCCLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AX25M_CLK_CFG_SPEC, AX25M_DCCLK_EN_A, O>;
impl<'a, const O: u8> AX25M_DCCLK_EN_W<'a, O> {
    #[doc = "Disable CPU core clock output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AX25M_DCCLK_EN_A::DISABLE)
    }
    #[doc = "Enable CPU core clock output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AX25M_DCCLK_EN_A::ENABLE)
    }
}
#[doc = "AX25M CPU core clock enable control bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AX25M_CCLK_EN_A {
    #[doc = "0: Disable CPU core clock output"]
    DISABLE = 0,
    #[doc = "1: Enable CPU core clock output"]
    ENABLE = 1,
}
impl From<AX25M_CCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AX25M_CCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `ax25m_cclk(0-1)_en` reader - AX25M CPU core clock enable control bit"]
pub type AX25M_CCLK_EN_R = crate::BitReader<AX25M_CCLK_EN_A>;
impl AX25M_CCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AX25M_CCLK_EN_A {
        match self.bits {
            false => AX25M_CCLK_EN_A::DISABLE,
            true => AX25M_CCLK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AX25M_CCLK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AX25M_CCLK_EN_A::ENABLE
    }
}
#[doc = "Fields `ax25m_cclk(0-1)_en` writer - AX25M CPU core clock enable control bit"]
pub type AX25M_CCLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AX25M_CLK_CFG_SPEC, AX25M_CCLK_EN_A, O>;
impl<'a, const O: u8> AX25M_CCLK_EN_W<'a, O> {
    #[doc = "Disable CPU core clock output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AX25M_CCLK_EN_A::DISABLE)
    }
    #[doc = "Enable CPU core clock output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AX25M_CCLK_EN_A::ENABLE)
    }
}
#[doc = "AX25M CPU core clock division ratio configure\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AX25M_CCLK_DIV_A {
    #[doc = "0: 12/12 division"]
    D12 = 0,
    #[doc = "1: 11/12 division"]
    D11 = 1,
    #[doc = "2: 10/12 division"]
    D10 = 2,
    #[doc = "3: 9/12 division"]
    D9 = 3,
    #[doc = "4: 8/12 division"]
    D8 = 4,
    #[doc = "5: 7/12 division"]
    D7 = 5,
    #[doc = "6: 6/12 division"]
    D6 = 6,
    #[doc = "7: 5/12 division"]
    D5 = 7,
    #[doc = "8: 4/12 division"]
    D4 = 8,
    #[doc = "9: 3/12 division"]
    D3 = 9,
    #[doc = "10: 2/12 division"]
    D2 = 10,
    #[doc = "11: 1/12 division"]
    D1 = 11,
}
impl From<AX25M_CCLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: AX25M_CCLK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ax25m_cclk_div` reader - AX25M CPU core clock division ratio configure"]
pub type AX25M_CCLK_DIV_R = crate::FieldReader<u8, AX25M_CCLK_DIV_A>;
impl AX25M_CCLK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AX25M_CCLK_DIV_A> {
        match self.bits {
            0 => Some(AX25M_CCLK_DIV_A::D12),
            1 => Some(AX25M_CCLK_DIV_A::D11),
            2 => Some(AX25M_CCLK_DIV_A::D10),
            3 => Some(AX25M_CCLK_DIV_A::D9),
            4 => Some(AX25M_CCLK_DIV_A::D8),
            5 => Some(AX25M_CCLK_DIV_A::D7),
            6 => Some(AX25M_CCLK_DIV_A::D6),
            7 => Some(AX25M_CCLK_DIV_A::D5),
            8 => Some(AX25M_CCLK_DIV_A::D4),
            9 => Some(AX25M_CCLK_DIV_A::D3),
            10 => Some(AX25M_CCLK_DIV_A::D2),
            11 => Some(AX25M_CCLK_DIV_A::D1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `D12`"]
    #[inline(always)]
    pub fn is_d12(&self) -> bool {
        *self == AX25M_CCLK_DIV_A::D12
    }
    #[doc = "Checks if the value of the field is `D11`"]
    #[inline(always)]
    pub fn is_d11(&self) -> bool {
        *self == AX25M_CCLK_DIV_A::D11
    }
    #[doc = "Checks if the value of the field is `D10`"]
    #[inline(always)]
    pub fn is_d10(&self) -> bool {
        *self == AX25M_CCLK_DIV_A::D10
    }
    #[doc = "Checks if the value of the field is `D9`"]
    #[inline(always)]
    pub fn is_d9(&self) -> bool {
        *self == AX25M_CCLK_DIV_A::D9
    }
    #[doc = "Checks if the value of the field is `D8`"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == AX25M_CCLK_DIV_A::D8
    }
    #[doc = "Checks if the value of the field is `D7`"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == AX25M_CCLK_DIV_A::D7
    }
    #[doc = "Checks if the value of the field is `D6`"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == AX25M_CCLK_DIV_A::D6
    }
    #[doc = "Checks if the value of the field is `D5`"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == AX25M_CCLK_DIV_A::D5
    }
    #[doc = "Checks if the value of the field is `D4`"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == AX25M_CCLK_DIV_A::D4
    }
    #[doc = "Checks if the value of the field is `D3`"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == AX25M_CCLK_DIV_A::D3
    }
    #[doc = "Checks if the value of the field is `D2`"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == AX25M_CCLK_DIV_A::D2
    }
    #[doc = "Checks if the value of the field is `D1`"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == AX25M_CCLK_DIV_A::D1
    }
}
#[doc = "Field `ax25m_cclk_div` writer - AX25M CPU core clock division ratio configure"]
pub type AX25M_CCLK_DIV_W<'a> =
    crate::FieldWriter<'a, u32, AX25M_CLK_CFG_SPEC, u8, AX25M_CCLK_DIV_A, 4, 4>;
impl<'a> AX25M_CCLK_DIV_W<'a> {
    #[doc = "12/12 division"]
    #[inline(always)]
    pub fn d12(self) -> &'a mut W {
        self.variant(AX25M_CCLK_DIV_A::D12)
    }
    #[doc = "11/12 division"]
    #[inline(always)]
    pub fn d11(self) -> &'a mut W {
        self.variant(AX25M_CCLK_DIV_A::D11)
    }
    #[doc = "10/12 division"]
    #[inline(always)]
    pub fn d10(self) -> &'a mut W {
        self.variant(AX25M_CCLK_DIV_A::D10)
    }
    #[doc = "9/12 division"]
    #[inline(always)]
    pub fn d9(self) -> &'a mut W {
        self.variant(AX25M_CCLK_DIV_A::D9)
    }
    #[doc = "8/12 division"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut W {
        self.variant(AX25M_CCLK_DIV_A::D8)
    }
    #[doc = "7/12 division"]
    #[inline(always)]
    pub fn d7(self) -> &'a mut W {
        self.variant(AX25M_CCLK_DIV_A::D7)
    }
    #[doc = "6/12 division"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut W {
        self.variant(AX25M_CCLK_DIV_A::D6)
    }
    #[doc = "5/12 division"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut W {
        self.variant(AX25M_CCLK_DIV_A::D5)
    }
    #[doc = "4/12 division"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut W {
        self.variant(AX25M_CCLK_DIV_A::D4)
    }
    #[doc = "3/12 division"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut W {
        self.variant(AX25M_CCLK_DIV_A::D3)
    }
    #[doc = "2/12 division"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut W {
        self.variant(AX25M_CCLK_DIV_A::D2)
    }
    #[doc = "1/12 division"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut W {
        self.variant(AX25M_CCLK_DIV_A::D1)
    }
}
#[doc = "AX25M CPU core clock source selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AX25M_CCLK_SEL_A {
    #[doc = "0: 800MHz source clock from PLL 2"]
    PLL2_800M = 0,
    #[doc = "1: 1GHz source clock from PLL 0"]
    PLL0_1G = 1,
    #[doc = "2: 1.2GHz source clock from PLL 2"]
    PLL2_1_2G = 2,
}
impl From<AX25M_CCLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: AX25M_CCLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ax25m_cclk_sel` reader - AX25M CPU core clock source selection"]
pub type AX25M_CCLK_SEL_R = crate::FieldReader<u8, AX25M_CCLK_SEL_A>;
impl AX25M_CCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AX25M_CCLK_SEL_A> {
        match self.bits {
            0 => Some(AX25M_CCLK_SEL_A::PLL2_800M),
            1 => Some(AX25M_CCLK_SEL_A::PLL0_1G),
            2 => Some(AX25M_CCLK_SEL_A::PLL2_1_2G),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLL2_800M`"]
    #[inline(always)]
    pub fn is_pll2_800m(&self) -> bool {
        *self == AX25M_CCLK_SEL_A::PLL2_800M
    }
    #[doc = "Checks if the value of the field is `PLL0_1G`"]
    #[inline(always)]
    pub fn is_pll0_1g(&self) -> bool {
        *self == AX25M_CCLK_SEL_A::PLL0_1G
    }
    #[doc = "Checks if the value of the field is `PLL2_1_2G`"]
    #[inline(always)]
    pub fn is_pll2_1_2g(&self) -> bool {
        *self == AX25M_CCLK_SEL_A::PLL2_1_2G
    }
}
#[doc = "Field `ax25m_cclk_sel` writer - AX25M CPU core clock source selection"]
pub type AX25M_CCLK_SEL_W<'a> =
    crate::FieldWriter<'a, u32, AX25M_CLK_CFG_SPEC, u8, AX25M_CCLK_SEL_A, 2, 0>;
impl<'a> AX25M_CCLK_SEL_W<'a> {
    #[doc = "800MHz source clock from PLL 2"]
    #[inline(always)]
    pub fn pll2_800m(self) -> &'a mut W {
        self.variant(AX25M_CCLK_SEL_A::PLL2_800M)
    }
    #[doc = "1GHz source clock from PLL 0"]
    #[inline(always)]
    pub fn pll0_1g(self) -> &'a mut W {
        self.variant(AX25M_CCLK_SEL_A::PLL0_1G)
    }
    #[doc = "1.2GHz source clock from PLL 2"]
    #[inline(always)]
    pub fn pll2_1_2g(self) -> &'a mut W {
        self.variant(AX25M_CCLK_SEL_A::PLL2_1_2G)
    }
}
impl R {
    #[doc = "AX25M CPU dc clock enable control bit"]
    #[inline(always)]
    pub unsafe fn ax25m_dcclk_en(&self, n: u8) -> AX25M_DCCLK_EN_R {
        AX25M_DCCLK_EN_R::new(((self.bits >> (n + 10)) & 1) != 0)
    }
    #[doc = "Bit 10 - AX25M CPU dc clock enable control bit"]
    #[inline(always)]
    pub fn ax25m_dcclk0_en(&self) -> AX25M_DCCLK_EN_R {
        AX25M_DCCLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AX25M CPU dc clock enable control bit"]
    #[inline(always)]
    pub fn ax25m_dcclk1_en(&self) -> AX25M_DCCLK_EN_R {
        AX25M_DCCLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "AX25M CPU core clock enable control bit"]
    #[inline(always)]
    pub unsafe fn ax25m_cclk_en(&self, n: u8) -> AX25M_CCLK_EN_R {
        AX25M_CCLK_EN_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Bit 8 - AX25M CPU core clock enable control bit"]
    #[inline(always)]
    pub fn ax25m_cclk0_en(&self) -> AX25M_CCLK_EN_R {
        AX25M_CCLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AX25M CPU core clock enable control bit"]
    #[inline(always)]
    pub fn ax25m_cclk1_en(&self) -> AX25M_CCLK_EN_R {
        AX25M_CCLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 4:7 - AX25M CPU core clock division ratio configure"]
    #[inline(always)]
    pub fn ax25m_cclk_div(&self) -> AX25M_CCLK_DIV_R {
        AX25M_CCLK_DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - AX25M CPU core clock source selection"]
    #[inline(always)]
    pub fn ax25m_cclk_sel(&self) -> AX25M_CCLK_SEL_R {
        AX25M_CCLK_SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Write enable for bit 10/11 (ax25m_dcclk\\[01\\]_en)"]
    #[inline(always)]
    pub unsafe fn we_ax25m_dcclk_en<const O: u8>(&mut self) -> WE_AX25M_DCCLK_EN_W<O> {
        WE_AX25M_DCCLK_EN_W::new(self)
    }
    #[doc = "Bit 28 - Write enable for bit 10/11 (ax25m_dcclk\\[01\\]_en)"]
    #[inline(always)]
    pub fn we_ax25m_dcclk0_en(&mut self) -> WE_AX25M_DCCLK_EN_W<28> {
        WE_AX25M_DCCLK_EN_W::new(self)
    }
    #[doc = "Bit 29 - Write enable for bit 10/11 (ax25m_dcclk\\[01\\]_en)"]
    #[inline(always)]
    pub fn we_ax25m_dcclk1_en(&mut self) -> WE_AX25M_DCCLK_EN_W<29> {
        WE_AX25M_DCCLK_EN_W::new(self)
    }
    #[doc = "Write enable for bit 8/9 (ax25m_cclk\\[01\\]_en)"]
    #[inline(always)]
    pub unsafe fn we_ax25m_cclk_en<const O: u8>(&mut self) -> WE_AX25M_CCLK_EN_W<O> {
        WE_AX25M_CCLK_EN_W::new(self)
    }
    #[doc = "Bit 26 - Write enable for bit 8/9 (ax25m_cclk\\[01\\]_en)"]
    #[inline(always)]
    pub fn we_ax25m_cclk0_en(&mut self) -> WE_AX25M_CCLK_EN_W<26> {
        WE_AX25M_CCLK_EN_W::new(self)
    }
    #[doc = "Bit 27 - Write enable for bit 8/9 (ax25m_cclk\\[01\\]_en)"]
    #[inline(always)]
    pub fn we_ax25m_cclk1_en(&mut self) -> WE_AX25M_CCLK_EN_W<27> {
        WE_AX25M_CCLK_EN_W::new(self)
    }
    #[doc = "Bit 25 - Write enable for bit \\[7:4\\]
(ax25m_cclk_div)"]
    #[inline(always)]
    pub fn we_ax25m_cclk_div(&mut self) -> WE_AX25M_CCLK_DIV_W {
        WE_AX25M_CCLK_DIV_W::new(self)
    }
    #[doc = "Bit 24 - Write enable for bit \\[1:0\\]
(ax25m_cclk_sel)"]
    #[inline(always)]
    pub fn we_ax25m_cclk_sel(&mut self) -> WE_AX25M_CCLK_SEL_W {
        WE_AX25M_CCLK_SEL_W::new(self)
    }
    #[doc = "AX25M CPU dc clock enable control bit"]
    #[inline(always)]
    pub unsafe fn ax25m_dcclk_en<const O: u8>(&mut self) -> AX25M_DCCLK_EN_W<O> {
        AX25M_DCCLK_EN_W::new(self)
    }
    #[doc = "Bit 10 - AX25M CPU dc clock enable control bit"]
    #[inline(always)]
    pub fn ax25m_dcclk0_en(&mut self) -> AX25M_DCCLK_EN_W<10> {
        AX25M_DCCLK_EN_W::new(self)
    }
    #[doc = "Bit 11 - AX25M CPU dc clock enable control bit"]
    #[inline(always)]
    pub fn ax25m_dcclk1_en(&mut self) -> AX25M_DCCLK_EN_W<11> {
        AX25M_DCCLK_EN_W::new(self)
    }
    #[doc = "AX25M CPU core clock enable control bit"]
    #[inline(always)]
    pub unsafe fn ax25m_cclk_en<const O: u8>(&mut self) -> AX25M_CCLK_EN_W<O> {
        AX25M_CCLK_EN_W::new(self)
    }
    #[doc = "Bit 8 - AX25M CPU core clock enable control bit"]
    #[inline(always)]
    pub fn ax25m_cclk0_en(&mut self) -> AX25M_CCLK_EN_W<8> {
        AX25M_CCLK_EN_W::new(self)
    }
    #[doc = "Bit 9 - AX25M CPU core clock enable control bit"]
    #[inline(always)]
    pub fn ax25m_cclk1_en(&mut self) -> AX25M_CCLK_EN_W<9> {
        AX25M_CCLK_EN_W::new(self)
    }
    #[doc = "Bits 4:7 - AX25M CPU core clock division ratio configure"]
    #[inline(always)]
    pub fn ax25m_cclk_div(&mut self) -> AX25M_CCLK_DIV_W {
        AX25M_CCLK_DIV_W::new(self)
    }
    #[doc = "Bits 0:1 - AX25M CPU core clock source selection"]
    #[inline(always)]
    pub fn ax25m_cclk_sel(&mut self) -> AX25M_CCLK_SEL_W {
        AX25M_CCLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AX25M dual-core RISCV core clock Division configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ax25m_clk_cfg](index.html) module"]
pub struct AX25M_CLK_CFG_SPEC;
impl crate::RegisterSpec for AX25M_CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ax25m_clk_cfg::R](R) reader structure"]
impl crate::Readable for AX25M_CLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ax25m_clk_cfg::W](W) writer structure"]
impl crate::Writable for AX25M_CLK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AX25M_CLK_CFG to value 0x0f61"]
impl crate::Resettable for AX25M_CLK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f61
    }
}
