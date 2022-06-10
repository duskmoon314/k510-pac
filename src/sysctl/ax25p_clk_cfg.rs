#[doc = "Register `AX25P_CLK_CFG` reader"]
pub struct R(crate::R<AX25P_CLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AX25P_CLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AX25P_CLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AX25P_CLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AX25P_CLK_CFG` writer"]
pub struct W(crate::W<AX25P_CLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AX25P_CLK_CFG_SPEC>;
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
impl From<crate::W<AX25P_CLK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AX25P_CLK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_ax25p_lmclk_en` writer - Write enable for bit 9 (ax25p_lmclk_en)"]
pub type WE_AX25P_LMCLK_EN_W<'a> = crate::BitWriter<'a, u32, AX25P_CLK_CFG_SPEC, bool, 27>;
#[doc = "Field `WE_ax25p_cclk_en` writer - Write enable for bit 8 (ax25p_cclk_en)"]
pub type WE_AX25P_CCLK_EN_W<'a> = crate::BitWriter<'a, u32, AX25P_CLK_CFG_SPEC, bool, 26>;
#[doc = "Field `WE_ax25p_cclk_div` writer - Write enable for bit \\[6:4\\]
(ax25p_cclk_div)"]
pub type WE_AX25P_CCLK_DIV_W<'a> = crate::BitWriter<'a, u32, AX25P_CLK_CFG_SPEC, bool, 25>;
#[doc = "Field `WE_ax25p_cclk_sel` writer - Write enable for bit 0 (ax25p_cclk_sel)"]
pub type WE_AX25P_CCLK_SEL_W<'a> = crate::BitWriter<'a, u32, AX25P_CLK_CFG_SPEC, bool, 24>;
#[doc = "AX25P CPU local memory clock enable control bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AX25P_LMCLK_EN_A {
    #[doc = "0: Disable CPU core clock output"]
    DISABLE = 0,
    #[doc = "1: Enable CPU core clock output"]
    ENABLE = 1,
}
impl From<AX25P_LMCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AX25P_LMCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ax25p_lmclk_en` reader - AX25P CPU local memory clock enable control bit"]
pub type AX25P_LMCLK_EN_R = crate::BitReader<AX25P_LMCLK_EN_A>;
impl AX25P_LMCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AX25P_LMCLK_EN_A {
        match self.bits {
            false => AX25P_LMCLK_EN_A::DISABLE,
            true => AX25P_LMCLK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AX25P_LMCLK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AX25P_LMCLK_EN_A::ENABLE
    }
}
#[doc = "Field `ax25p_lmclk_en` writer - AX25P CPU local memory clock enable control bit"]
pub type AX25P_LMCLK_EN_W<'a> = crate::BitWriter<'a, u32, AX25P_CLK_CFG_SPEC, AX25P_LMCLK_EN_A, 9>;
impl<'a> AX25P_LMCLK_EN_W<'a> {
    #[doc = "Disable CPU core clock output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AX25P_LMCLK_EN_A::DISABLE)
    }
    #[doc = "Enable CPU core clock output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AX25P_LMCLK_EN_A::ENABLE)
    }
}
#[doc = "AX25P CPU core clock enable control bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AX25P_CCLK_EN_A {
    #[doc = "0: Disable CPU core clock output"]
    DISABLE = 0,
    #[doc = "1: Enable CPU core clock output"]
    ENABLE = 1,
}
impl From<AX25P_CCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AX25P_CCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ax25p_cclk_en` reader - AX25P CPU core clock enable control bit"]
pub type AX25P_CCLK_EN_R = crate::BitReader<AX25P_CCLK_EN_A>;
impl AX25P_CCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AX25P_CCLK_EN_A {
        match self.bits {
            false => AX25P_CCLK_EN_A::DISABLE,
            true => AX25P_CCLK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AX25P_CCLK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AX25P_CCLK_EN_A::ENABLE
    }
}
#[doc = "Field `ax25p_cclk_en` writer - AX25P CPU core clock enable control bit"]
pub type AX25P_CCLK_EN_W<'a> = crate::BitWriter<'a, u32, AX25P_CLK_CFG_SPEC, AX25P_CCLK_EN_A, 8>;
impl<'a> AX25P_CCLK_EN_W<'a> {
    #[doc = "Disable CPU core clock output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AX25P_CCLK_EN_A::DISABLE)
    }
    #[doc = "Enable CPU core clock output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AX25P_CCLK_EN_A::ENABLE)
    }
}
#[doc = "AX25P CPU core clock division ratio configure\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AX25P_CCLK_DIV_A {
    #[doc = "0: 6/6 division"]
    D6 = 0,
    #[doc = "1: 5/6 division"]
    D5 = 1,
    #[doc = "2: 4/6 division"]
    D4 = 2,
    #[doc = "3: 3/6 division"]
    D3 = 3,
    #[doc = "4: 2/6 division"]
    D2 = 4,
    #[doc = "5: 1/6 division"]
    D1 = 5,
}
impl From<AX25P_CCLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: AX25P_CCLK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ax25p_cclk_div` reader - AX25P CPU core clock division ratio configure"]
pub type AX25P_CCLK_DIV_R = crate::FieldReader<u8, AX25P_CCLK_DIV_A>;
impl AX25P_CCLK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AX25P_CCLK_DIV_A> {
        match self.bits {
            0 => Some(AX25P_CCLK_DIV_A::D6),
            1 => Some(AX25P_CCLK_DIV_A::D5),
            2 => Some(AX25P_CCLK_DIV_A::D4),
            3 => Some(AX25P_CCLK_DIV_A::D3),
            4 => Some(AX25P_CCLK_DIV_A::D2),
            5 => Some(AX25P_CCLK_DIV_A::D1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `D6`"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == AX25P_CCLK_DIV_A::D6
    }
    #[doc = "Checks if the value of the field is `D5`"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == AX25P_CCLK_DIV_A::D5
    }
    #[doc = "Checks if the value of the field is `D4`"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == AX25P_CCLK_DIV_A::D4
    }
    #[doc = "Checks if the value of the field is `D3`"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == AX25P_CCLK_DIV_A::D3
    }
    #[doc = "Checks if the value of the field is `D2`"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == AX25P_CCLK_DIV_A::D2
    }
    #[doc = "Checks if the value of the field is `D1`"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == AX25P_CCLK_DIV_A::D1
    }
}
#[doc = "Field `ax25p_cclk_div` writer - AX25P CPU core clock division ratio configure"]
pub type AX25P_CCLK_DIV_W<'a> =
    crate::FieldWriter<'a, u32, AX25P_CLK_CFG_SPEC, u8, AX25P_CCLK_DIV_A, 3, 4>;
impl<'a> AX25P_CCLK_DIV_W<'a> {
    #[doc = "6/6 division"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut W {
        self.variant(AX25P_CCLK_DIV_A::D6)
    }
    #[doc = "5/6 division"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut W {
        self.variant(AX25P_CCLK_DIV_A::D5)
    }
    #[doc = "4/6 division"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut W {
        self.variant(AX25P_CCLK_DIV_A::D4)
    }
    #[doc = "3/6 division"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut W {
        self.variant(AX25P_CCLK_DIV_A::D3)
    }
    #[doc = "2/6 division"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut W {
        self.variant(AX25P_CCLK_DIV_A::D2)
    }
    #[doc = "1/6 division"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut W {
        self.variant(AX25P_CCLK_DIV_A::D1)
    }
}
#[doc = "AX25P CPU core clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AX25P_CCLK_SEL_A {
    #[doc = "0: 792MHz source clock from PLL 2"]
    PLL2_792M = 0,
    #[doc = "1: 1GHz source clock from PLL 0"]
    PLL0_1G = 1,
}
impl From<AX25P_CCLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: AX25P_CCLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ax25p_cclk_sel` reader - AX25P CPU core clock source selection"]
pub type AX25P_CCLK_SEL_R = crate::BitReader<AX25P_CCLK_SEL_A>;
impl AX25P_CCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AX25P_CCLK_SEL_A {
        match self.bits {
            false => AX25P_CCLK_SEL_A::PLL2_792M,
            true => AX25P_CCLK_SEL_A::PLL0_1G,
        }
    }
    #[doc = "Checks if the value of the field is `PLL2_792M`"]
    #[inline(always)]
    pub fn is_pll2_792m(&self) -> bool {
        *self == AX25P_CCLK_SEL_A::PLL2_792M
    }
    #[doc = "Checks if the value of the field is `PLL0_1G`"]
    #[inline(always)]
    pub fn is_pll0_1g(&self) -> bool {
        *self == AX25P_CCLK_SEL_A::PLL0_1G
    }
}
#[doc = "Field `ax25p_cclk_sel` writer - AX25P CPU core clock source selection"]
pub type AX25P_CCLK_SEL_W<'a> = crate::BitWriter<'a, u32, AX25P_CLK_CFG_SPEC, AX25P_CCLK_SEL_A, 0>;
impl<'a> AX25P_CCLK_SEL_W<'a> {
    #[doc = "792MHz source clock from PLL 2"]
    #[inline(always)]
    pub fn pll2_792m(self) -> &'a mut W {
        self.variant(AX25P_CCLK_SEL_A::PLL2_792M)
    }
    #[doc = "1GHz source clock from PLL 0"]
    #[inline(always)]
    pub fn pll0_1g(self) -> &'a mut W {
        self.variant(AX25P_CCLK_SEL_A::PLL0_1G)
    }
}
impl R {
    #[doc = "Bit 9 - AX25P CPU local memory clock enable control bit"]
    #[inline(always)]
    pub fn ax25p_lmclk_en(&self) -> AX25P_LMCLK_EN_R {
        AX25P_LMCLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - AX25P CPU core clock enable control bit"]
    #[inline(always)]
    pub fn ax25p_cclk_en(&self) -> AX25P_CCLK_EN_R {
        AX25P_CCLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 4:6 - AX25P CPU core clock division ratio configure"]
    #[inline(always)]
    pub fn ax25p_cclk_div(&self) -> AX25P_CCLK_DIV_R {
        AX25P_CCLK_DIV_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 0 - AX25P CPU core clock source selection"]
    #[inline(always)]
    pub fn ax25p_cclk_sel(&self) -> AX25P_CCLK_SEL_R {
        AX25P_CCLK_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Write enable for bit 9 (ax25p_lmclk_en)"]
    #[inline(always)]
    pub fn we_ax25p_lmclk_en(&mut self) -> WE_AX25P_LMCLK_EN_W {
        WE_AX25P_LMCLK_EN_W::new(self)
    }
    #[doc = "Bit 26 - Write enable for bit 8 (ax25p_cclk_en)"]
    #[inline(always)]
    pub fn we_ax25p_cclk_en(&mut self) -> WE_AX25P_CCLK_EN_W {
        WE_AX25P_CCLK_EN_W::new(self)
    }
    #[doc = "Bit 25 - Write enable for bit \\[6:4\\]
(ax25p_cclk_div)"]
    #[inline(always)]
    pub fn we_ax25p_cclk_div(&mut self) -> WE_AX25P_CCLK_DIV_W {
        WE_AX25P_CCLK_DIV_W::new(self)
    }
    #[doc = "Bit 24 - Write enable for bit 0 (ax25p_cclk_sel)"]
    #[inline(always)]
    pub fn we_ax25p_cclk_sel(&mut self) -> WE_AX25P_CCLK_SEL_W {
        WE_AX25P_CCLK_SEL_W::new(self)
    }
    #[doc = "Bit 9 - AX25P CPU local memory clock enable control bit"]
    #[inline(always)]
    pub fn ax25p_lmclk_en(&mut self) -> AX25P_LMCLK_EN_W {
        AX25P_LMCLK_EN_W::new(self)
    }
    #[doc = "Bit 8 - AX25P CPU core clock enable control bit"]
    #[inline(always)]
    pub fn ax25p_cclk_en(&mut self) -> AX25P_CCLK_EN_W {
        AX25P_CCLK_EN_W::new(self)
    }
    #[doc = "Bits 4:6 - AX25P CPU core clock division ratio configure"]
    #[inline(always)]
    pub fn ax25p_cclk_div(&mut self) -> AX25P_CCLK_DIV_W {
        AX25P_CCLK_DIV_W::new(self)
    }
    #[doc = "Bit 0 - AX25P CPU core clock source selection"]
    #[inline(always)]
    pub fn ax25p_cclk_sel(&mut self) -> AX25P_CCLK_SEL_W {
        AX25P_CCLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AX25P single-core RISCV core clock Division configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ax25p_clk_cfg](index.html) module"]
pub struct AX25P_CLK_CFG_SPEC;
impl crate::RegisterSpec for AX25P_CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ax25p_clk_cfg::R](R) reader structure"]
impl crate::Readable for AX25P_CLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ax25p_clk_cfg::W](W) writer structure"]
impl crate::Writable for AX25P_CLK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AX25P_CLK_CFG to value 0x0330"]
impl crate::Resettable for AX25P_CLK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0330
    }
}
