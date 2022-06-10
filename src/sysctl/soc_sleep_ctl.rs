#[doc = "Register `SOC_SLEEP_CTL` reader"]
pub struct R(crate::R<SOC_SLEEP_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOC_SLEEP_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOC_SLEEP_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOC_SLEEP_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOC_SLEEP_CTL` writer"]
pub struct W(crate::W<SOC_SLEEP_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOC_SLEEP_CTL_SPEC>;
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
impl From<crate::W<SOC_SLEEP_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOC_SLEEP_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_pll3_init_en` reader - Write enable for bit 6 (pll3_init_en)"]
pub type WE_PLL3_INIT_EN_R = crate::BitReader<bool>;
#[doc = "Field `WE_pll3_init_en` writer - Write enable for bit 6 (pll3_init_en)"]
pub type WE_PLL3_INIT_EN_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_CTL_SPEC, bool, 27>;
#[doc = "Field `WE_pll2_init_en` reader - Write enable for bit 5 (pll2_init_en)"]
pub type WE_PLL2_INIT_EN_R = crate::BitReader<bool>;
#[doc = "Field `WE_pll2_init_en` writer - Write enable for bit 5 (pll2_init_en)"]
pub type WE_PLL2_INIT_EN_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_CTL_SPEC, bool, 26>;
#[doc = "Field `WE_auto_slp_en` reader - Write enable for bit 4 (auto_slp_en)"]
pub type WE_AUTO_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `WE_auto_slp_en` writer - Write enable for bit 4 (auto_slp_en)"]
pub type WE_AUTO_SLP_EN_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_CTL_SPEC, bool, 25>;
#[doc = "Field `WE_core_sleep_req` reader - Write enable for bit 0 (core_sleep_req)"]
pub type WE_CORE_SLEEP_REQ_R = crate::BitReader<bool>;
#[doc = "Field `WE_core_sleep_req` writer - Write enable for bit 0 (core_sleep_req)"]
pub type WE_CORE_SLEEP_REQ_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_CTL_SPEC, bool, 24>;
#[doc = "PLL3 auto initialization enable bit.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL3_INIT_EN_A {
    #[doc = "0: Disable PLL auto initialization"]
    DISABLE = 0,
    #[doc = "1: Enable PLL auto initialization"]
    ENABLE = 1,
}
impl From<PLL3_INIT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL3_INIT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll3_init_en` reader - PLL3 auto initialization enable bit."]
pub type PLL3_INIT_EN_R = crate::BitReader<PLL3_INIT_EN_A>;
impl PLL3_INIT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL3_INIT_EN_A {
        match self.bits {
            false => PLL3_INIT_EN_A::DISABLE,
            true => PLL3_INIT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL3_INIT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL3_INIT_EN_A::ENABLE
    }
}
#[doc = "Field `pll3_init_en` writer - PLL3 auto initialization enable bit."]
pub type PLL3_INIT_EN_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_CTL_SPEC, PLL3_INIT_EN_A, 6>;
impl<'a> PLL3_INIT_EN_W<'a> {
    #[doc = "Disable PLL auto initialization"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL3_INIT_EN_A::DISABLE)
    }
    #[doc = "Enable PLL auto initialization"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL3_INIT_EN_A::ENABLE)
    }
}
#[doc = "PLL2 auto initialization enable bit.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL2_INIT_EN_A {
    #[doc = "0: Disable PLL auto initialization"]
    DISABLE = 0,
    #[doc = "1: Enable PLL auto initialization"]
    ENABLE = 1,
}
impl From<PLL2_INIT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2_INIT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll2_init_en` reader - PLL2 auto initialization enable bit."]
pub type PLL2_INIT_EN_R = crate::BitReader<PLL2_INIT_EN_A>;
impl PLL2_INIT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL2_INIT_EN_A {
        match self.bits {
            false => PLL2_INIT_EN_A::DISABLE,
            true => PLL2_INIT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL2_INIT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL2_INIT_EN_A::ENABLE
    }
}
#[doc = "Field `pll2_init_en` writer - PLL2 auto initialization enable bit."]
pub type PLL2_INIT_EN_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_CTL_SPEC, PLL2_INIT_EN_A, 5>;
impl<'a> PLL2_INIT_EN_W<'a> {
    #[doc = "Disable PLL auto initialization"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL2_INIT_EN_A::DISABLE)
    }
    #[doc = "Enable PLL auto initialization"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL2_INIT_EN_A::ENABLE)
    }
}
#[doc = "SoC core auto sleep enable control bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_SLP_EN_A {
    #[doc = "0: Disable SoC core auto sleep entering feature"]
    DISABLE = 0,
    #[doc = "1: Enable SoC core auto sleep entering feature"]
    ENABLE = 1,
}
impl From<AUTO_SLP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AUTO_SLP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `auto_slp_en` reader - SoC core auto sleep enable control bit."]
pub type AUTO_SLP_EN_R = crate::BitReader<AUTO_SLP_EN_A>;
impl AUTO_SLP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTO_SLP_EN_A {
        match self.bits {
            false => AUTO_SLP_EN_A::DISABLE,
            true => AUTO_SLP_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUTO_SLP_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AUTO_SLP_EN_A::ENABLE
    }
}
#[doc = "Field `auto_slp_en` writer - SoC core auto sleep enable control bit."]
pub type AUTO_SLP_EN_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_CTL_SPEC, AUTO_SLP_EN_A, 4>;
impl<'a> AUTO_SLP_EN_W<'a> {
    #[doc = "Disable SoC core auto sleep entering feature"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTO_SLP_EN_A::DISABLE)
    }
    #[doc = "Enable SoC core auto sleep entering feature"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUTO_SLP_EN_A::ENABLE)
    }
}
#[doc = "SoC core go sleep request bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORE_SLEEP_REQ_AW {
    #[doc = "0: No operation"]
    NO_OPERATION = 0,
    #[doc = "1: Request SoC core to enter SLEEP mode"]
    SLEEP = 1,
}
impl From<CORE_SLEEP_REQ_AW> for bool {
    #[inline(always)]
    fn from(variant: CORE_SLEEP_REQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `core_sleep_req` writer - SoC core go sleep request bit"]
pub type CORE_SLEEP_REQ_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_CTL_SPEC, CORE_SLEEP_REQ_AW, 0>;
impl<'a> CORE_SLEEP_REQ_W<'a> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(CORE_SLEEP_REQ_AW::NO_OPERATION)
    }
    #[doc = "Request SoC core to enter SLEEP mode"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut W {
        self.variant(CORE_SLEEP_REQ_AW::SLEEP)
    }
}
impl R {
    #[doc = "Bit 27 - Write enable for bit 6 (pll3_init_en)"]
    #[inline(always)]
    pub fn we_pll3_init_en(&self) -> WE_PLL3_INIT_EN_R {
        WE_PLL3_INIT_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - Write enable for bit 5 (pll2_init_en)"]
    #[inline(always)]
    pub fn we_pll2_init_en(&self) -> WE_PLL2_INIT_EN_R {
        WE_PLL2_INIT_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - Write enable for bit 4 (auto_slp_en)"]
    #[inline(always)]
    pub fn we_auto_slp_en(&self) -> WE_AUTO_SLP_EN_R {
        WE_AUTO_SLP_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - Write enable for bit 0 (core_sleep_req)"]
    #[inline(always)]
    pub fn we_core_sleep_req(&self) -> WE_CORE_SLEEP_REQ_R {
        WE_CORE_SLEEP_REQ_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL3 auto initialization enable bit."]
    #[inline(always)]
    pub fn pll3_init_en(&self) -> PLL3_INIT_EN_R {
        PLL3_INIT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL2 auto initialization enable bit."]
    #[inline(always)]
    pub fn pll2_init_en(&self) -> PLL2_INIT_EN_R {
        PLL2_INIT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - SoC core auto sleep enable control bit."]
    #[inline(always)]
    pub fn auto_slp_en(&self) -> AUTO_SLP_EN_R {
        AUTO_SLP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Write enable for bit 6 (pll3_init_en)"]
    #[inline(always)]
    pub fn we_pll3_init_en(&mut self) -> WE_PLL3_INIT_EN_W {
        WE_PLL3_INIT_EN_W::new(self)
    }
    #[doc = "Bit 26 - Write enable for bit 5 (pll2_init_en)"]
    #[inline(always)]
    pub fn we_pll2_init_en(&mut self) -> WE_PLL2_INIT_EN_W {
        WE_PLL2_INIT_EN_W::new(self)
    }
    #[doc = "Bit 25 - Write enable for bit 4 (auto_slp_en)"]
    #[inline(always)]
    pub fn we_auto_slp_en(&mut self) -> WE_AUTO_SLP_EN_W {
        WE_AUTO_SLP_EN_W::new(self)
    }
    #[doc = "Bit 24 - Write enable for bit 0 (core_sleep_req)"]
    #[inline(always)]
    pub fn we_core_sleep_req(&mut self) -> WE_CORE_SLEEP_REQ_W {
        WE_CORE_SLEEP_REQ_W::new(self)
    }
    #[doc = "Bit 6 - PLL3 auto initialization enable bit."]
    #[inline(always)]
    pub fn pll3_init_en(&mut self) -> PLL3_INIT_EN_W {
        PLL3_INIT_EN_W::new(self)
    }
    #[doc = "Bit 5 - PLL2 auto initialization enable bit."]
    #[inline(always)]
    pub fn pll2_init_en(&mut self) -> PLL2_INIT_EN_W {
        PLL2_INIT_EN_W::new(self)
    }
    #[doc = "Bit 4 - SoC core auto sleep enable control bit."]
    #[inline(always)]
    pub fn auto_slp_en(&mut self) -> AUTO_SLP_EN_W {
        AUTO_SLP_EN_W::new(self)
    }
    #[doc = "Bit 0 - SoC core go sleep request bit"]
    #[inline(always)]
    pub fn core_sleep_req(&mut self) -> CORE_SLEEP_REQ_W {
        CORE_SLEEP_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SoC sleep mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_sleep_ctl](index.html) module"]
pub struct SOC_SLEEP_CTL_SPEC;
impl crate::RegisterSpec for SOC_SLEEP_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soc_sleep_ctl::R](R) reader structure"]
impl crate::Readable for SOC_SLEEP_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soc_sleep_ctl::W](W) writer structure"]
impl crate::Writable for SOC_SLEEP_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOC_SLEEP_CTL to value 0x60"]
impl crate::Resettable for SOC_SLEEP_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x60
    }
}
