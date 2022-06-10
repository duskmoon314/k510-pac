#[doc = "Register `PLL%s_CFG1` reader"]
pub struct R(crate::R<PLL_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL%s_CFG1` writer"]
pub struct W(crate::W<PLL_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_CFG1_SPEC>;
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
impl From<crate::W<PLL_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_pll_bypass` writer - Write enable for bit 19 (pll_bypass)"]
pub type WE_PLL_BYPASS_W<'a> = crate::BitWriter<'a, u32, PLL_CFG1_SPEC, bool, 28>;
#[doc = "Field `WE_pll_test` writer - Write enable for bit 18 (pll_test)"]
pub type WE_PLL_TEST_W<'a> = crate::BitWriter<'a, u32, PLL_CFG1_SPEC, bool, 27>;
#[doc = "Field `WE_pll_ensat` writer - Write enable for bit 17 (pll_ensat)"]
pub type WE_PLL_ENSAT_W<'a> = crate::BitWriter<'a, u32, PLL_CFG1_SPEC, bool, 26>;
#[doc = "Field `WE_pll_fasten` writer - Write enable for bit 16 (pll_fasten)"]
pub type WE_PLL_FASTEN_W<'a> = crate::BitWriter<'a, u32, PLL_CFG1_SPEC, bool, 25>;
#[doc = "Field `WE_pll_bwadj` writer - Write enable for bits \\[11:0\\]
(pll_bwadj)"]
pub type WE_PLL_BWADJ_W<'a> = crate::BitWriter<'a, u32, PLL_CFG1_SPEC, bool, 24>;
#[doc = "PLL\\[i\\]
bypass mode enable control bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_BYPASS_A {
    #[doc = "0: Disable PLL bypass mode"]
    DISABLE = 0,
    #[doc = "1: Enable PLL bypass mode"]
    ENABLE = 1,
}
impl From<PLL_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll_bypass` reader - PLL\\[i\\]
bypass mode enable control bit."]
pub type PLL_BYPASS_R = crate::BitReader<PLL_BYPASS_A>;
impl PLL_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_BYPASS_A {
        match self.bits {
            false => PLL_BYPASS_A::DISABLE,
            true => PLL_BYPASS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_BYPASS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_BYPASS_A::ENABLE
    }
}
#[doc = "Field `pll_bypass` writer - PLL\\[i\\]
bypass mode enable control bit."]
pub type PLL_BYPASS_W<'a> = crate::BitWriter<'a, u32, PLL_CFG1_SPEC, PLL_BYPASS_A, 19>;
impl<'a> PLL_BYPASS_W<'a> {
    #[doc = "Disable PLL bypass mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_BYPASS_A::DISABLE)
    }
    #[doc = "Enable PLL bypass mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_BYPASS_A::ENABLE)
    }
}
#[doc = "PLL\\[i\\]
test mode enable control bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_TEST_A {
    #[doc = "0: Disable PLL test mode"]
    DISABLE = 0,
    #[doc = "1: Enable PLL test mode"]
    ENABLE = 1,
}
impl From<PLL_TEST_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_TEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll_test` reader - PLL\\[i\\]
test mode enable control bit."]
pub type PLL_TEST_R = crate::BitReader<PLL_TEST_A>;
impl PLL_TEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_TEST_A {
        match self.bits {
            false => PLL_TEST_A::DISABLE,
            true => PLL_TEST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_TEST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_TEST_A::ENABLE
    }
}
#[doc = "Field `pll_test` writer - PLL\\[i\\]
test mode enable control bit."]
pub type PLL_TEST_W<'a> = crate::BitWriter<'a, u32, PLL_CFG1_SPEC, PLL_TEST_A, 18>;
impl<'a> PLL_TEST_W<'a> {
    #[doc = "Disable PLL test mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_TEST_A::DISABLE)
    }
    #[doc = "Enable PLL test mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_TEST_A::ENABLE)
    }
}
#[doc = "PLL\\[i\\]
saturation behavior enable control bit.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_ENSAT_A {
    #[doc = "0: Disable PLL saturation behavior"]
    DISABLE = 0,
    #[doc = "1: Enable PLL saturation behavior"]
    ENABLE = 1,
}
impl From<PLL_ENSAT_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_ENSAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll_ensat` reader - PLL\\[i\\]
saturation behavior enable control bit."]
pub type PLL_ENSAT_R = crate::BitReader<PLL_ENSAT_A>;
impl PLL_ENSAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_ENSAT_A {
        match self.bits {
            false => PLL_ENSAT_A::DISABLE,
            true => PLL_ENSAT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_ENSAT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_ENSAT_A::ENABLE
    }
}
#[doc = "Field `pll_ensat` writer - PLL\\[i\\]
saturation behavior enable control bit."]
pub type PLL_ENSAT_W<'a> = crate::BitWriter<'a, u32, PLL_CFG1_SPEC, PLL_ENSAT_A, 17>;
impl<'a> PLL_ENSAT_W<'a> {
    #[doc = "Disable PLL saturation behavior"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_ENSAT_A::DISABLE)
    }
    #[doc = "Enable PLL saturation behavior"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_ENSAT_A::ENABLE)
    }
}
#[doc = "PLL\\[i\\]
fast locking circuit enable control bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_FASTEN_A {
    #[doc = "0: Disable PLL fast locking circuit"]
    DISABLE = 0,
    #[doc = "1: Enable PLL fast locking circuit"]
    ENABLE = 1,
}
impl From<PLL_FASTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_FASTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pll_fasten` reader - PLL\\[i\\]
fast locking circuit enable control bit."]
pub type PLL_FASTEN_R = crate::BitReader<PLL_FASTEN_A>;
impl PLL_FASTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_FASTEN_A {
        match self.bits {
            false => PLL_FASTEN_A::DISABLE,
            true => PLL_FASTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_FASTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_FASTEN_A::ENABLE
    }
}
#[doc = "Field `pll_fasten` writer - PLL\\[i\\]
fast locking circuit enable control bit."]
pub type PLL_FASTEN_W<'a> = crate::BitWriter<'a, u32, PLL_CFG1_SPEC, PLL_FASTEN_A, 16>;
impl<'a> PLL_FASTEN_W<'a> {
    #[doc = "Disable PLL fast locking circuit"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_FASTEN_A::DISABLE)
    }
    #[doc = "Enable PLL fast locking circuit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_FASTEN_A::ENABLE)
    }
}
#[doc = "Field `pll_bwadj` reader - PLL\\[i\\]
bandwidth adjustment parameter."]
pub type PLL_BWADJ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pll_bwadj` writer - PLL\\[i\\]
bandwidth adjustment parameter."]
pub type PLL_BWADJ_W<'a> = crate::FieldWriter<'a, u32, PLL_CFG1_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bit 19 - PLL\\[i\\]
bypass mode enable control bit."]
    #[inline(always)]
    pub fn pll_bypass(&self) -> PLL_BYPASS_R {
        PLL_BYPASS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - PLL\\[i\\]
test mode enable control bit."]
    #[inline(always)]
    pub fn pll_test(&self) -> PLL_TEST_R {
        PLL_TEST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - PLL\\[i\\]
saturation behavior enable control bit."]
    #[inline(always)]
    pub fn pll_ensat(&self) -> PLL_ENSAT_R {
        PLL_ENSAT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - PLL\\[i\\]
fast locking circuit enable control bit."]
    #[inline(always)]
    pub fn pll_fasten(&self) -> PLL_FASTEN_R {
        PLL_FASTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 0:11 - PLL\\[i\\]
bandwidth adjustment parameter."]
    #[inline(always)]
    pub fn pll_bwadj(&self) -> PLL_BWADJ_R {
        PLL_BWADJ_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 28 - Write enable for bit 19 (pll_bypass)"]
    #[inline(always)]
    pub fn we_pll_bypass(&mut self) -> WE_PLL_BYPASS_W {
        WE_PLL_BYPASS_W::new(self)
    }
    #[doc = "Bit 27 - Write enable for bit 18 (pll_test)"]
    #[inline(always)]
    pub fn we_pll_test(&mut self) -> WE_PLL_TEST_W {
        WE_PLL_TEST_W::new(self)
    }
    #[doc = "Bit 26 - Write enable for bit 17 (pll_ensat)"]
    #[inline(always)]
    pub fn we_pll_ensat(&mut self) -> WE_PLL_ENSAT_W {
        WE_PLL_ENSAT_W::new(self)
    }
    #[doc = "Bit 25 - Write enable for bit 16 (pll_fasten)"]
    #[inline(always)]
    pub fn we_pll_fasten(&mut self) -> WE_PLL_FASTEN_W {
        WE_PLL_FASTEN_W::new(self)
    }
    #[doc = "Bit 24 - Write enable for bits \\[11:0\\]
(pll_bwadj)"]
    #[inline(always)]
    pub fn we_pll_bwadj(&mut self) -> WE_PLL_BWADJ_W {
        WE_PLL_BWADJ_W::new(self)
    }
    #[doc = "Bit 19 - PLL\\[i\\]
bypass mode enable control bit."]
    #[inline(always)]
    pub fn pll_bypass(&mut self) -> PLL_BYPASS_W {
        PLL_BYPASS_W::new(self)
    }
    #[doc = "Bit 18 - PLL\\[i\\]
test mode enable control bit."]
    #[inline(always)]
    pub fn pll_test(&mut self) -> PLL_TEST_W {
        PLL_TEST_W::new(self)
    }
    #[doc = "Bit 17 - PLL\\[i\\]
saturation behavior enable control bit."]
    #[inline(always)]
    pub fn pll_ensat(&mut self) -> PLL_ENSAT_W {
        PLL_ENSAT_W::new(self)
    }
    #[doc = "Bit 16 - PLL\\[i\\]
fast locking circuit enable control bit."]
    #[inline(always)]
    pub fn pll_fasten(&mut self) -> PLL_FASTEN_W {
        PLL_FASTEN_W::new(self)
    }
    #[doc = "Bits 0:11 - PLL\\[i\\]
bandwidth adjustment parameter."]
    #[inline(always)]
    pub fn pll_bwadj(&mut self) -> PLL_BWADJ_W {
        PLL_BWADJ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL\\[i\\]
configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_cfg1](index.html) module"]
pub struct PLL_CFG1_SPEC;
impl crate::RegisterSpec for PLL_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_cfg1::R](R) reader structure"]
impl crate::Readable for PLL_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_cfg1::W](W) writer structure"]
impl crate::Writable for PLL_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL%s_CFG1 to value 0x0002_0027"]
impl crate::Resettable for PLL_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0027
    }
}
