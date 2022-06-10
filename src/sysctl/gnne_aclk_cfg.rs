#[doc = "Register `GNNE_ACLK_CFG` reader"]
pub struct R(crate::R<GNNE_ACLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GNNE_ACLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GNNE_ACLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GNNE_ACLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GNNE_ACLK_CFG` writer"]
pub struct W(crate::W<GNNE_ACLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GNNE_ACLK_CFG_SPEC>;
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
impl From<crate::W<GNNE_ACLK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GNNE_ACLK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_gnne_aclk_en` writer - Write enable for bit 9 (gnne_aclk_en)"]
pub type WE_GNNE_ACLK_EN_W<'a> = crate::BitWriter<'a, u32, GNNE_ACLK_CFG_SPEC, bool, 27>;
#[doc = "Field `WE_gnne_aclk_div` writer - Write enable for bits \\[6:4\\]
(gnne_aclk_div)"]
pub type WE_GNNE_ACLK_DIV_W<'a> = crate::BitWriter<'a, u32, GNNE_ACLK_CFG_SPEC, bool, 25>;
#[doc = "Field `WE_gnne_aclk_sel` writer - Write enable for bit 0 (gnne_aclk_sel)"]
pub type WE_GNNE_ACLK_SEL_W<'a> = crate::BitWriter<'a, u32, GNNE_ACLK_CFG_SPEC, bool, 24>;
#[doc = "GNNE system axi master clock enable control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GNNE_ACLK_EN_A {
    #[doc = "0: Disable GNNE system axi master clock output"]
    DISABLE = 0,
    #[doc = "1: Enable GNNE system axi master clock output"]
    ENABLE = 1,
}
impl From<GNNE_ACLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: GNNE_ACLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `gnne_aclk_en` reader - GNNE system axi master clock enable control bit"]
pub type GNNE_ACLK_EN_R = crate::BitReader<GNNE_ACLK_EN_A>;
impl GNNE_ACLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GNNE_ACLK_EN_A {
        match self.bits {
            false => GNNE_ACLK_EN_A::DISABLE,
            true => GNNE_ACLK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GNNE_ACLK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GNNE_ACLK_EN_A::ENABLE
    }
}
#[doc = "Field `gnne_aclk_en` writer - GNNE system axi master clock enable control bit"]
pub type GNNE_ACLK_EN_W<'a> = crate::BitWriter<'a, u32, GNNE_ACLK_CFG_SPEC, GNNE_ACLK_EN_A, 9>;
impl<'a> GNNE_ACLK_EN_W<'a> {
    #[doc = "Disable GNNE system axi master clock output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GNNE_ACLK_EN_A::DISABLE)
    }
    #[doc = "Enable GNNE system axi master clock output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GNNE_ACLK_EN_A::ENABLE)
    }
}
#[doc = "GNNE axi port clock division ratio configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GNNE_ACLK_DIV_A {
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
impl From<GNNE_ACLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: GNNE_ACLK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `gnne_aclk_div` reader - GNNE axi port clock division ratio configure"]
pub type GNNE_ACLK_DIV_R = crate::FieldReader<u8, GNNE_ACLK_DIV_A>;
impl GNNE_ACLK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GNNE_ACLK_DIV_A> {
        match self.bits {
            0 => Some(GNNE_ACLK_DIV_A::D6),
            1 => Some(GNNE_ACLK_DIV_A::D5),
            2 => Some(GNNE_ACLK_DIV_A::D4),
            3 => Some(GNNE_ACLK_DIV_A::D3),
            4 => Some(GNNE_ACLK_DIV_A::D2),
            5 => Some(GNNE_ACLK_DIV_A::D1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `D6`"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == GNNE_ACLK_DIV_A::D6
    }
    #[doc = "Checks if the value of the field is `D5`"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == GNNE_ACLK_DIV_A::D5
    }
    #[doc = "Checks if the value of the field is `D4`"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == GNNE_ACLK_DIV_A::D4
    }
    #[doc = "Checks if the value of the field is `D3`"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == GNNE_ACLK_DIV_A::D3
    }
    #[doc = "Checks if the value of the field is `D2`"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == GNNE_ACLK_DIV_A::D2
    }
    #[doc = "Checks if the value of the field is `D1`"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == GNNE_ACLK_DIV_A::D1
    }
}
#[doc = "Field `gnne_aclk_div` writer - GNNE axi port clock division ratio configure"]
pub type GNNE_ACLK_DIV_W<'a> =
    crate::FieldWriter<'a, u32, GNNE_ACLK_CFG_SPEC, u8, GNNE_ACLK_DIV_A, 3, 4>;
impl<'a> GNNE_ACLK_DIV_W<'a> {
    #[doc = "6/6 division"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut W {
        self.variant(GNNE_ACLK_DIV_A::D6)
    }
    #[doc = "5/6 division"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut W {
        self.variant(GNNE_ACLK_DIV_A::D5)
    }
    #[doc = "4/6 division"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut W {
        self.variant(GNNE_ACLK_DIV_A::D4)
    }
    #[doc = "3/6 division"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut W {
        self.variant(GNNE_ACLK_DIV_A::D3)
    }
    #[doc = "2/6 division"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut W {
        self.variant(GNNE_ACLK_DIV_A::D2)
    }
    #[doc = "1/6 division"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut W {
        self.variant(GNNE_ACLK_DIV_A::D1)
    }
}
#[doc = "GNNE axi master clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GNNE_ACLK_SEL_A {
    #[doc = "0: 500MHz source clock from PLL 0 div2"]
    PLL0_500M = 0,
    #[doc = "1: 667MHz source clock from PLL 1 div2"]
    PLL1_667M = 1,
}
impl From<GNNE_ACLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: GNNE_ACLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `gnne_aclk_sel` reader - GNNE axi master clock source selection"]
pub type GNNE_ACLK_SEL_R = crate::BitReader<GNNE_ACLK_SEL_A>;
impl GNNE_ACLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GNNE_ACLK_SEL_A {
        match self.bits {
            false => GNNE_ACLK_SEL_A::PLL0_500M,
            true => GNNE_ACLK_SEL_A::PLL1_667M,
        }
    }
    #[doc = "Checks if the value of the field is `PLL0_500M`"]
    #[inline(always)]
    pub fn is_pll0_500m(&self) -> bool {
        *self == GNNE_ACLK_SEL_A::PLL0_500M
    }
    #[doc = "Checks if the value of the field is `PLL1_667M`"]
    #[inline(always)]
    pub fn is_pll1_667m(&self) -> bool {
        *self == GNNE_ACLK_SEL_A::PLL1_667M
    }
}
#[doc = "Field `gnne_aclk_sel` writer - GNNE axi master clock source selection"]
pub type GNNE_ACLK_SEL_W<'a> = crate::BitWriter<'a, u32, GNNE_ACLK_CFG_SPEC, GNNE_ACLK_SEL_A, 0>;
impl<'a> GNNE_ACLK_SEL_W<'a> {
    #[doc = "500MHz source clock from PLL 0 div2"]
    #[inline(always)]
    pub fn pll0_500m(self) -> &'a mut W {
        self.variant(GNNE_ACLK_SEL_A::PLL0_500M)
    }
    #[doc = "667MHz source clock from PLL 1 div2"]
    #[inline(always)]
    pub fn pll1_667m(self) -> &'a mut W {
        self.variant(GNNE_ACLK_SEL_A::PLL1_667M)
    }
}
impl R {
    #[doc = "Bit 9 - GNNE system axi master clock enable control bit"]
    #[inline(always)]
    pub fn gnne_aclk_en(&self) -> GNNE_ACLK_EN_R {
        GNNE_ACLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 4:6 - GNNE axi port clock division ratio configure"]
    #[inline(always)]
    pub fn gnne_aclk_div(&self) -> GNNE_ACLK_DIV_R {
        GNNE_ACLK_DIV_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 0 - GNNE axi master clock source selection"]
    #[inline(always)]
    pub fn gnne_aclk_sel(&self) -> GNNE_ACLK_SEL_R {
        GNNE_ACLK_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Write enable for bit 9 (gnne_aclk_en)"]
    #[inline(always)]
    pub fn we_gnne_aclk_en(&mut self) -> WE_GNNE_ACLK_EN_W {
        WE_GNNE_ACLK_EN_W::new(self)
    }
    #[doc = "Bit 25 - Write enable for bits \\[6:4\\]
(gnne_aclk_div)"]
    #[inline(always)]
    pub fn we_gnne_aclk_div(&mut self) -> WE_GNNE_ACLK_DIV_W {
        WE_GNNE_ACLK_DIV_W::new(self)
    }
    #[doc = "Bit 24 - Write enable for bit 0 (gnne_aclk_sel)"]
    #[inline(always)]
    pub fn we_gnne_aclk_sel(&mut self) -> WE_GNNE_ACLK_SEL_W {
        WE_GNNE_ACLK_SEL_W::new(self)
    }
    #[doc = "Bit 9 - GNNE system axi master clock enable control bit"]
    #[inline(always)]
    pub fn gnne_aclk_en(&mut self) -> GNNE_ACLK_EN_W {
        GNNE_ACLK_EN_W::new(self)
    }
    #[doc = "Bits 4:6 - GNNE axi port clock division ratio configure"]
    #[inline(always)]
    pub fn gnne_aclk_div(&mut self) -> GNNE_ACLK_DIV_W {
        GNNE_ACLK_DIV_W::new(self)
    }
    #[doc = "Bit 0 - GNNE axi master clock source selection"]
    #[inline(always)]
    pub fn gnne_aclk_sel(&mut self) -> GNNE_ACLK_SEL_W {
        GNNE_ACLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GNNE axi clock configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gnne_aclk_cfg](index.html) module"]
pub struct GNNE_ACLK_CFG_SPEC;
impl crate::RegisterSpec for GNNE_ACLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gnne_aclk_cfg::R](R) reader structure"]
impl crate::Readable for GNNE_ACLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gnne_aclk_cfg::W](W) writer structure"]
impl crate::Writable for GNNE_ACLK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GNNE_ACLK_CFG to value 0"]
impl crate::Resettable for GNNE_ACLK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
