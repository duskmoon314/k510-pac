#[doc = "Register `CLK_STABLE_TIM` reader"]
pub struct R(crate::R<CLK_STABLE_TIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_STABLE_TIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_STABLE_TIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_STABLE_TIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_STABLE_TIM` writer"]
pub struct W(crate::W<CLK_STABLE_TIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_STABLE_TIM_SPEC>;
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
impl From<crate::W<CLK_STABLE_TIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_STABLE_TIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pll_stable_time` reader - The Max allowed time SoC for PLL oscillator stable. The time unit here is a cycle of 32KHz clock."]
pub type PLL_STABLE_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pll_stable_time` writer - The Max allowed time SoC for PLL oscillator stable. The time unit here is a cycle of 32KHz clock."]
pub type PLL_STABLE_TIME_W<'a> = crate::FieldWriter<'a, u32, CLK_STABLE_TIM_SPEC, u16, u16, 16, 16>;
#[doc = "Field `clk_25m_stable_time` reader - The Max allowed time SoC for 25MHz oscillator stable. The time unit here is a cycle of 32KHz clock."]
pub type CLK_25M_STABLE_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `clk_25m_stable_time` writer - The Max allowed time SoC for 25MHz oscillator stable. The time unit here is a cycle of 32KHz clock."]
pub type CLK_25M_STABLE_TIME_W<'a> =
    crate::FieldWriter<'a, u32, CLK_STABLE_TIM_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 16:31 - The Max allowed time SoC for PLL oscillator stable. The time unit here is a cycle of 32KHz clock."]
    #[inline(always)]
    pub fn pll_stable_time(&self) -> PLL_STABLE_TIME_R {
        PLL_STABLE_TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - The Max allowed time SoC for 25MHz oscillator stable. The time unit here is a cycle of 32KHz clock."]
    #[inline(always)]
    pub fn clk_25m_stable_time(&self) -> CLK_25M_STABLE_TIME_R {
        CLK_25M_STABLE_TIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - The Max allowed time SoC for PLL oscillator stable. The time unit here is a cycle of 32KHz clock."]
    #[inline(always)]
    pub fn pll_stable_time(&mut self) -> PLL_STABLE_TIME_W {
        PLL_STABLE_TIME_W::new(self)
    }
    #[doc = "Bits 0:15 - The Max allowed time SoC for 25MHz oscillator stable. The time unit here is a cycle of 32KHz clock."]
    #[inline(always)]
    pub fn clk_25m_stable_time(&mut self) -> CLK_25M_STABLE_TIME_W {
        CLK_25M_STABLE_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock stable timing configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_stable_tim](index.html) module"]
pub struct CLK_STABLE_TIM_SPEC;
impl crate::RegisterSpec for CLK_STABLE_TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_stable_tim::R](R) reader structure"]
impl crate::Readable for CLK_STABLE_TIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_stable_tim::W](W) writer structure"]
impl crate::Writable for CLK_STABLE_TIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_STABLE_TIM to value 0x000a_0001"]
impl crate::Resettable for CLK_STABLE_TIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000a_0001
    }
}
