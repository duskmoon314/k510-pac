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
#[doc = "Field `WE_12` writer - Write enable for bit 12"]
pub type WE_12_W<'a> = crate::BitWriter<'a, u32, UART_SCLK_CFG_SPEC, bool, 26>;
#[doc = "Field `WE_8_4` writer - Write enable for bit \\[8:4\\]"]
pub type WE_8_4_W<'a> = crate::BitWriter<'a, u32, UART_SCLK_CFG_SPEC, bool, 25>;
#[doc = "Field `WE_0` writer - Write enable for bit 0"]
pub type WE_0_W<'a> = crate::BitWriter<'a, u32, UART_SCLK_CFG_SPEC, bool, 24>;
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
#[doc = "Field `uart_sclk_div` reader - UART\\[i\\]
host module serial clock division ratio configure"]
pub type UART_SCLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_sclk_div` writer - UART\\[i\\]
host module serial clock division ratio configure"]
pub type UART_SCLK_DIV_W<'a> = crate::FieldWriter<'a, u32, UART_SCLK_CFG_SPEC, u8, u8, 5, 4>;
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
    #[doc = "Bit 26 - Write enable for bit 12"]
    #[inline(always)]
    pub fn we_12(&mut self) -> WE_12_W {
        WE_12_W::new(self)
    }
    #[doc = "Bit 25 - Write enable for bit \\[8:4\\]"]
    #[inline(always)]
    pub fn we_8_4(&mut self) -> WE_8_4_W {
        WE_8_4_W::new(self)
    }
    #[doc = "Bit 24 - Write enable for bit 0"]
    #[inline(always)]
    pub fn we_0(&mut self) -> WE_0_W {
        WE_0_W::new(self)
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
