#[doc = "Register `CPU_WAKUP_CFG` reader"]
pub struct R(crate::R<CPU_WAKUP_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_WAKUP_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_WAKUP_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_WAKUP_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_WAKUP_CFG` writer"]
pub struct W(crate::W<CPU_WAKUP_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_WAKUP_CFG_SPEC>;
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
impl From<crate::W<CPU_WAKUP_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_WAKUP_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fields `WE_soc_wakup_ax25(0-1)_en` writer - Write enable for bit (soc_wakup_ax25\\[mp\\]_en)"]
pub type WE_SOC_WAKUP_AX25_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_WAKUP_CFG_SPEC, bool, O>;
#[doc = "AX25M/AX25P auto wake-up enable when SoC wakes up.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_WAKUP_AX25_EN_A {
    #[doc = "0: Don't wakeup when SoC waks up from SLEEP mode"]
    DISABLE = 0,
    #[doc = "1: Auto Waking-up when SoC wakes up from SLEEP mode"]
    ENABLE = 1,
}
impl From<SOC_WAKUP_AX25_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_WAKUP_AX25_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `soc_wakup_ax25(0-1)_en` reader - AX25M/AX25P auto wake-up enable when SoC wakes up."]
pub type SOC_WAKUP_AX25_EN_R = crate::BitReader<SOC_WAKUP_AX25_EN_A>;
impl SOC_WAKUP_AX25_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_WAKUP_AX25_EN_A {
        match self.bits {
            false => SOC_WAKUP_AX25_EN_A::DISABLE,
            true => SOC_WAKUP_AX25_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SOC_WAKUP_AX25_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SOC_WAKUP_AX25_EN_A::ENABLE
    }
}
#[doc = "Fields `soc_wakup_ax25(0-1)_en` writer - AX25M/AX25P auto wake-up enable when SoC wakes up."]
pub type SOC_WAKUP_AX25_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_WAKUP_CFG_SPEC, SOC_WAKUP_AX25_EN_A, O>;
impl<'a, const O: u8> SOC_WAKUP_AX25_EN_W<'a, O> {
    #[doc = "Don't wakeup when SoC waks up from SLEEP mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SOC_WAKUP_AX25_EN_A::DISABLE)
    }
    #[doc = "Auto Waking-up when SoC wakes up from SLEEP mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SOC_WAKUP_AX25_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "AX25M/AX25P auto wake-up enable when SoC wakes up."]
    #[inline(always)]
    pub unsafe fn soc_wakup_ax25_en(&self, n: u8) -> SOC_WAKUP_AX25_EN_R {
        SOC_WAKUP_AX25_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - AX25M/AX25P auto wake-up enable when SoC wakes up."]
    #[inline(always)]
    pub fn soc_wakup_ax25m_en(&self) -> SOC_WAKUP_AX25_EN_R {
        SOC_WAKUP_AX25_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AX25M/AX25P auto wake-up enable when SoC wakes up."]
    #[inline(always)]
    pub fn soc_wakup_ax25p_en(&self) -> SOC_WAKUP_AX25_EN_R {
        SOC_WAKUP_AX25_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Write enable for bit (soc_wakup_ax25\\[mp\\]_en)"]
    #[inline(always)]
    pub unsafe fn we_soc_wakup_ax25_en<const O: u8>(&mut self) -> WE_SOC_WAKUP_AX25_EN_W<O> {
        WE_SOC_WAKUP_AX25_EN_W::new(self)
    }
    #[doc = "Bit 24 - Write enable for bit (soc_wakup_ax25\\[mp\\]_en)"]
    #[inline(always)]
    pub fn we_soc_wakup_ax25m_en(&mut self) -> WE_SOC_WAKUP_AX25_EN_W<24> {
        WE_SOC_WAKUP_AX25_EN_W::new(self)
    }
    #[doc = "Bit 25 - Write enable for bit (soc_wakup_ax25\\[mp\\]_en)"]
    #[inline(always)]
    pub fn we_soc_wakup_ax25p_en(&mut self) -> WE_SOC_WAKUP_AX25_EN_W<25> {
        WE_SOC_WAKUP_AX25_EN_W::new(self)
    }
    #[doc = "AX25M/AX25P auto wake-up enable when SoC wakes up."]
    #[inline(always)]
    pub unsafe fn soc_wakup_ax25_en<const O: u8>(&mut self) -> SOC_WAKUP_AX25_EN_W<O> {
        SOC_WAKUP_AX25_EN_W::new(self)
    }
    #[doc = "Bit 0 - AX25M/AX25P auto wake-up enable when SoC wakes up."]
    #[inline(always)]
    pub fn soc_wakup_ax25m_en(&mut self) -> SOC_WAKUP_AX25_EN_W<0> {
        SOC_WAKUP_AX25_EN_W::new(self)
    }
    #[doc = "Bit 1 - AX25M/AX25P auto wake-up enable when SoC wakes up."]
    #[inline(always)]
    pub fn soc_wakup_ax25p_en(&mut self) -> SOC_WAKUP_AX25_EN_W<1> {
        SOC_WAKUP_AX25_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU wake-up (when SoC core is woken up) configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_wakup_cfg](index.html) module"]
pub struct CPU_WAKUP_CFG_SPEC;
impl crate::RegisterSpec for CPU_WAKUP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_wakup_cfg::R](R) reader structure"]
impl crate::Readable for CPU_WAKUP_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_wakup_cfg::W](W) writer structure"]
impl crate::Writable for CPU_WAKUP_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_WAKUP_CFG to value 0x03"]
impl crate::Resettable for CPU_WAKUP_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
