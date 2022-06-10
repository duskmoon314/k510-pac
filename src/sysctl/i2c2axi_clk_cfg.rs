#[doc = "Register `I2C2AXI_CLK_CFG` reader"]
pub struct R(crate::R<I2C2AXI_CLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C2AXI_CLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C2AXI_CLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C2AXI_CLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C2AXI_CLK_CFG` writer"]
pub struct W(crate::W<I2C2AXI_CLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C2AXI_CLK_CFG_SPEC>;
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
impl From<crate::W<I2C2AXI_CLK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C2AXI_CLK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_para_i2c2axi_clk_en` writer - Write enable for bit 0 (para_i2c2axi_clk_en)"]
pub type WE_PARA_I2C2AXI_CLK_EN_W<'a> = crate::BitWriter<'a, u32, I2C2AXI_CLK_CFG_SPEC, bool, 24>;
#[doc = "I2C2AXI clock enable control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARA_I2C2AXI_CLK_EN_A {
    #[doc = "0: Disable I2C2AXI clock"]
    DISABLE = 0,
    #[doc = "1: Enable I2C2AXI clock"]
    ENABLE = 1,
}
impl From<PARA_I2C2AXI_CLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PARA_I2C2AXI_CLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `para_i2c2axi_clk_en` reader - I2C2AXI clock enable control"]
pub type PARA_I2C2AXI_CLK_EN_R = crate::BitReader<PARA_I2C2AXI_CLK_EN_A>;
impl PARA_I2C2AXI_CLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARA_I2C2AXI_CLK_EN_A {
        match self.bits {
            false => PARA_I2C2AXI_CLK_EN_A::DISABLE,
            true => PARA_I2C2AXI_CLK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PARA_I2C2AXI_CLK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PARA_I2C2AXI_CLK_EN_A::ENABLE
    }
}
#[doc = "Field `para_i2c2axi_clk_en` writer - I2C2AXI clock enable control"]
pub type PARA_I2C2AXI_CLK_EN_W<'a> =
    crate::BitWriter<'a, u32, I2C2AXI_CLK_CFG_SPEC, PARA_I2C2AXI_CLK_EN_A, 0>;
impl<'a> PARA_I2C2AXI_CLK_EN_W<'a> {
    #[doc = "Disable I2C2AXI clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PARA_I2C2AXI_CLK_EN_A::DISABLE)
    }
    #[doc = "Enable I2C2AXI clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PARA_I2C2AXI_CLK_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - I2C2AXI clock enable control"]
    #[inline(always)]
    pub fn para_i2c2axi_clk_en(&self) -> PARA_I2C2AXI_CLK_EN_R {
        PARA_I2C2AXI_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Write enable for bit 0 (para_i2c2axi_clk_en)"]
    #[inline(always)]
    pub fn we_para_i2c2axi_clk_en(&mut self) -> WE_PARA_I2C2AXI_CLK_EN_W {
        WE_PARA_I2C2AXI_CLK_EN_W::new(self)
    }
    #[doc = "Bit 0 - I2C2AXI clock enable control"]
    #[inline(always)]
    pub fn para_i2c2axi_clk_en(&mut self) -> PARA_I2C2AXI_CLK_EN_W {
        PARA_I2C2AXI_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C2AXI clock configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c2axi_clk_cfg](index.html) module"]
pub struct I2C2AXI_CLK_CFG_SPEC;
impl crate::RegisterSpec for I2C2AXI_CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c2axi_clk_cfg::R](R) reader structure"]
impl crate::Readable for I2C2AXI_CLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c2axi_clk_cfg::W](W) writer structure"]
impl crate::Writable for I2C2AXI_CLK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C2AXI_CLK_CFG to value 0x01"]
impl crate::Resettable for I2C2AXI_CLK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
