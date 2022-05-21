#[doc = "Register `PLL%s_CTL` reader"]
pub struct R(crate::R<PLL_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL%s_CTL` writer"]
pub struct W(crate::W<PLL_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_CTL_SPEC>;
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
impl From<crate::W<PLL_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_19_12` writer - Write enable for bits \\[19:12\\]"]
pub type WE_19_12_W<'a> = crate::BitWriter<'a, u32, PLL_CTL_SPEC, bool, 27>;
#[doc = "Field `WE_8` writer - Write enable for bit 8"]
pub type WE_8_W<'a> = crate::BitWriter<'a, u32, PLL_CTL_SPEC, bool, 26>;
#[doc = "Field `WE_4` writer - Write enable for bit 4"]
pub type WE_4_W<'a> = crate::BitWriter<'a, u32, PLL_CTL_SPEC, bool, 25>;
#[doc = "Field `WE_0` writer - Write enable for bit 0"]
pub type WE_0_W<'a> = crate::BitWriter<'a, u32, PLL_CTL_SPEC, bool, 24>;
#[doc = "Field `pll_lock_tim` reader - The time required for PLL\\[i\\]
to reach lock state."]
pub type PLL_LOCK_TIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pll_lock_tim` writer - The time required for PLL\\[i\\]
to reach lock state."]
pub type PLL_LOCK_TIM_W<'a> = crate::FieldWriter<'a, u32, PLL_CTL_SPEC, u8, u8, 8, 12>;
#[doc = "PLL\\[i\\]
VSO output clock enable control bit.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CLK_OE_A {
    #[doc = "0: Disable PLL VSO output clock"]
    DISABLE = 0,
    #[doc = "1: Enable PLL VSO output clock"]
    ENABLE = 1,
}
impl From<PLL_CLK_OE_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CLK_OE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll_clk_oe` reader - PLL\\[i\\]
VSO output clock enable control bit."]
pub type PLL_CLK_OE_R = crate::BitReader<PLL_CLK_OE_A>;
impl PLL_CLK_OE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_CLK_OE_A {
        match self.bits {
            false => PLL_CLK_OE_A::DISABLE,
            true => PLL_CLK_OE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_CLK_OE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_CLK_OE_A::ENABLE
    }
}
#[doc = "Field `pll_clk_oe` writer - PLL\\[i\\]
VSO output clock enable control bit."]
pub type PLL_CLK_OE_W<'a> = crate::BitWriter<'a, u32, PLL_CTL_SPEC, PLL_CLK_OE_A, 8>;
impl<'a> PLL_CLK_OE_W<'a> {
    #[doc = "Disable PLL VSO output clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_CLK_OE_A::DISABLE)
    }
    #[doc = "Enable PLL VSO output clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_CLK_OE_A::ENABLE)
    }
}
#[doc = "PLL\\[i\\]
initialization trigger bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_INIT_AW {
    #[doc = "0: No operation"]
    NO_OPERATION = 0,
    #[doc = "1: Start PLL initialization sequence"]
    START = 1,
}
impl From<PLL_INIT_AW> for bool {
    #[inline(always)]
    fn from(variant: PLL_INIT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll_init` writer - PLL\\[i\\]
initialization trigger bit."]
pub type PLL_INIT_W<'a> = crate::BitWriter<'a, u32, PLL_CTL_SPEC, PLL_INIT_AW, 4>;
impl<'a> PLL_INIT_W<'a> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(PLL_INIT_AW::NO_OPERATION)
    }
    #[doc = "Start PLL initialization sequence"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(PLL_INIT_AW::START)
    }
}
#[doc = "PLL\\[i\\]
go power-down mode trigger bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_PWRDWN_AW {
    #[doc = "0: No operation"]
    NO_OPERATION = 0,
    #[doc = "1: Put PLL\\[i\\]
into power-down mode"]
    POWER_DOWN = 1,
}
impl From<PLL_PWRDWN_AW> for bool {
    #[inline(always)]
    fn from(variant: PLL_PWRDWN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll_pwrdwn` writer - PLL\\[i\\]
go power-down mode trigger bit."]
pub type PLL_PWRDWN_W<'a> = crate::BitWriter<'a, u32, PLL_CTL_SPEC, PLL_PWRDWN_AW, 0>;
impl<'a> PLL_PWRDWN_W<'a> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(PLL_PWRDWN_AW::NO_OPERATION)
    }
    #[doc = "Put PLL\\[i\\]
into power-down mode"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(PLL_PWRDWN_AW::POWER_DOWN)
    }
}
impl R {
    #[doc = "Bits 12:19 - The time required for PLL\\[i\\]
to reach lock state."]
    #[inline(always)]
    pub fn pll_lock_tim(&self) -> PLL_LOCK_TIM_R {
        PLL_LOCK_TIM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 8 - PLL\\[i\\]
VSO output clock enable control bit."]
    #[inline(always)]
    pub fn pll_clk_oe(&self) -> PLL_CLK_OE_R {
        PLL_CLK_OE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Write enable for bits \\[19:12\\]"]
    #[inline(always)]
    pub fn we_19_12(&mut self) -> WE_19_12_W {
        WE_19_12_W::new(self)
    }
    #[doc = "Bit 26 - Write enable for bit 8"]
    #[inline(always)]
    pub fn we_8(&mut self) -> WE_8_W {
        WE_8_W::new(self)
    }
    #[doc = "Bit 25 - Write enable for bit 4"]
    #[inline(always)]
    pub fn we_4(&mut self) -> WE_4_W {
        WE_4_W::new(self)
    }
    #[doc = "Bit 24 - Write enable for bit 0"]
    #[inline(always)]
    pub fn we_0(&mut self) -> WE_0_W {
        WE_0_W::new(self)
    }
    #[doc = "Bits 12:19 - The time required for PLL\\[i\\]
to reach lock state."]
    #[inline(always)]
    pub fn pll_lock_tim(&mut self) -> PLL_LOCK_TIM_W {
        PLL_LOCK_TIM_W::new(self)
    }
    #[doc = "Bit 8 - PLL\\[i\\]
VSO output clock enable control bit."]
    #[inline(always)]
    pub fn pll_clk_oe(&mut self) -> PLL_CLK_OE_W {
        PLL_CLK_OE_W::new(self)
    }
    #[doc = "Bit 4 - PLL\\[i\\]
initialization trigger bit."]
    #[inline(always)]
    pub fn pll_init(&mut self) -> PLL_INIT_W {
        PLL_INIT_W::new(self)
    }
    #[doc = "Bit 0 - PLL\\[i\\]
go power-down mode trigger bit."]
    #[inline(always)]
    pub fn pll_pwrdwn(&mut self) -> PLL_PWRDWN_W {
        PLL_PWRDWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL\\[i\\]
control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_ctl](index.html) module"]
pub struct PLL_CTL_SPEC;
impl crate::RegisterSpec for PLL_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_ctl::R](R) reader structure"]
impl crate::Readable for PLL_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_ctl::W](W) writer structure"]
impl crate::Writable for PLL_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL%s_CTL to value 0x3100"]
impl crate::Resettable for PLL_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3100
    }
}
