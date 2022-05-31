#[doc = "Register `SYS_CTL_INT2_EN` reader"]
pub struct R(crate::R<SYS_CTL_INT2_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CTL_INT2_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CTL_INT2_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CTL_INT2_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_CTL_INT2_EN` writer"]
pub struct W(crate::W<SYS_CTL_INT2_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_CTL_INT2_EN_SPEC>;
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
impl From<crate::W<SYS_CTL_INT2_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_CTL_INT2_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ddrc dfs fail interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRC_DFS_FAIL_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<DDRC_DFS_FAIL_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: DDRC_DFS_FAIL_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ddrc_dfs_fail_ien` reader - ddrc dfs fail interrupt enable bit"]
pub type DDRC_DFS_FAIL_IEN_R = crate::BitReader<DDRC_DFS_FAIL_IEN_A>;
impl DDRC_DFS_FAIL_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRC_DFS_FAIL_IEN_A {
        match self.bits {
            false => DDRC_DFS_FAIL_IEN_A::DISABLE,
            true => DDRC_DFS_FAIL_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DDRC_DFS_FAIL_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DDRC_DFS_FAIL_IEN_A::ENABLE
    }
}
#[doc = "Field `ddrc_dfs_fail_ien` writer - ddrc dfs fail interrupt enable bit"]
pub type DDRC_DFS_FAIL_IEN_W<'a> =
    crate::BitWriter<'a, u32, SYS_CTL_INT2_EN_SPEC, DDRC_DFS_FAIL_IEN_A, 7>;
impl<'a> DDRC_DFS_FAIL_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DDRC_DFS_FAIL_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DDRC_DFS_FAIL_IEN_A::ENABLE)
    }
}
#[doc = "ddrc dfs success interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRC_DFS_SUCCESS_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<DDRC_DFS_SUCCESS_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: DDRC_DFS_SUCCESS_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ddrc_dfs_success_ien` reader - ddrc dfs success interrupt enable bit"]
pub type DDRC_DFS_SUCCESS_IEN_R = crate::BitReader<DDRC_DFS_SUCCESS_IEN_A>;
impl DDRC_DFS_SUCCESS_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRC_DFS_SUCCESS_IEN_A {
        match self.bits {
            false => DDRC_DFS_SUCCESS_IEN_A::DISABLE,
            true => DDRC_DFS_SUCCESS_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DDRC_DFS_SUCCESS_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DDRC_DFS_SUCCESS_IEN_A::ENABLE
    }
}
#[doc = "Field `ddrc_dfs_success_ien` writer - ddrc dfs success interrupt enable bit"]
pub type DDRC_DFS_SUCCESS_IEN_W<'a> =
    crate::BitWriter<'a, u32, SYS_CTL_INT2_EN_SPEC, DDRC_DFS_SUCCESS_IEN_A, 6>;
impl<'a> DDRC_DFS_SUCCESS_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DDRC_DFS_SUCCESS_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DDRC_DFS_SUCCESS_IEN_A::ENABLE)
    }
}
#[doc = "SoC core wake up from DEEPSLEEP mode interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_DSLP_WAKUP_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<SOC_DSLP_WAKUP_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_DSLP_WAKUP_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_dslp_wakup_ien` reader - SoC core wake up from DEEPSLEEP mode interrupt enable bit"]
pub type SOC_DSLP_WAKUP_IEN_R = crate::BitReader<SOC_DSLP_WAKUP_IEN_A>;
impl SOC_DSLP_WAKUP_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_DSLP_WAKUP_IEN_A {
        match self.bits {
            false => SOC_DSLP_WAKUP_IEN_A::DISABLE,
            true => SOC_DSLP_WAKUP_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SOC_DSLP_WAKUP_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SOC_DSLP_WAKUP_IEN_A::ENABLE
    }
}
#[doc = "Field `soc_dslp_wakup_ien` writer - SoC core wake up from DEEPSLEEP mode interrupt enable bit"]
pub type SOC_DSLP_WAKUP_IEN_W<'a> =
    crate::BitWriter<'a, u32, SYS_CTL_INT2_EN_SPEC, SOC_DSLP_WAKUP_IEN_A, 5>;
impl<'a> SOC_DSLP_WAKUP_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SOC_DSLP_WAKUP_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SOC_DSLP_WAKUP_IEN_A::ENABLE)
    }
}
#[doc = "SoC core wake up from SLEEP1 mode interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_SLP1_WAKUP_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<SOC_SLP1_WAKUP_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_SLP1_WAKUP_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_slp1_wakup_ien` reader - SoC core wake up from SLEEP1 mode interrupt enable bit"]
pub type SOC_SLP1_WAKUP_IEN_R = crate::BitReader<SOC_SLP1_WAKUP_IEN_A>;
impl SOC_SLP1_WAKUP_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_SLP1_WAKUP_IEN_A {
        match self.bits {
            false => SOC_SLP1_WAKUP_IEN_A::DISABLE,
            true => SOC_SLP1_WAKUP_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SOC_SLP1_WAKUP_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SOC_SLP1_WAKUP_IEN_A::ENABLE
    }
}
#[doc = "Field `soc_slp1_wakup_ien` writer - SoC core wake up from SLEEP1 mode interrupt enable bit"]
pub type SOC_SLP1_WAKUP_IEN_W<'a> =
    crate::BitWriter<'a, u32, SYS_CTL_INT2_EN_SPEC, SOC_SLP1_WAKUP_IEN_A, 4>;
impl<'a> SOC_SLP1_WAKUP_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SOC_SLP1_WAKUP_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SOC_SLP1_WAKUP_IEN_A::ENABLE)
    }
}
#[doc = "SoC core wake up from SLEEP0 mode interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_SLP0_WAKUP_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<SOC_SLP0_WAKUP_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_SLP0_WAKUP_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_slp0_wakup_ien` reader - SoC core wake up from SLEEP0 mode interrupt enable bit"]
pub type SOC_SLP0_WAKUP_IEN_R = crate::BitReader<SOC_SLP0_WAKUP_IEN_A>;
impl SOC_SLP0_WAKUP_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_SLP0_WAKUP_IEN_A {
        match self.bits {
            false => SOC_SLP0_WAKUP_IEN_A::DISABLE,
            true => SOC_SLP0_WAKUP_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SOC_SLP0_WAKUP_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SOC_SLP0_WAKUP_IEN_A::ENABLE
    }
}
#[doc = "Field `soc_slp0_wakup_ien` writer - SoC core wake up from SLEEP0 mode interrupt enable bit"]
pub type SOC_SLP0_WAKUP_IEN_W<'a> =
    crate::BitWriter<'a, u32, SYS_CTL_INT2_EN_SPEC, SOC_SLP0_WAKUP_IEN_A, 3>;
impl<'a> SOC_SLP0_WAKUP_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SOC_SLP0_WAKUP_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SOC_SLP0_WAKUP_IEN_A::ENABLE)
    }
}
#[doc = "SoC core entering DEEPSLEEP mode interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_GO_DSLP_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<SOC_GO_DSLP_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_GO_DSLP_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_go_dslp_ien` reader - SoC core entering DEEPSLEEP mode interrupt enable bit"]
pub type SOC_GO_DSLP_IEN_R = crate::BitReader<SOC_GO_DSLP_IEN_A>;
impl SOC_GO_DSLP_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_GO_DSLP_IEN_A {
        match self.bits {
            false => SOC_GO_DSLP_IEN_A::DISABLE,
            true => SOC_GO_DSLP_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SOC_GO_DSLP_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SOC_GO_DSLP_IEN_A::ENABLE
    }
}
#[doc = "Field `soc_go_dslp_ien` writer - SoC core entering DEEPSLEEP mode interrupt enable bit"]
pub type SOC_GO_DSLP_IEN_W<'a> =
    crate::BitWriter<'a, u32, SYS_CTL_INT2_EN_SPEC, SOC_GO_DSLP_IEN_A, 2>;
impl<'a> SOC_GO_DSLP_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SOC_GO_DSLP_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SOC_GO_DSLP_IEN_A::ENABLE)
    }
}
#[doc = "SoC core entering SLEEP1 mode interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_GO_SLP1_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<SOC_GO_SLP1_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_GO_SLP1_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_go_slp1_ien` reader - SoC core entering SLEEP1 mode interrupt enable bit"]
pub type SOC_GO_SLP1_IEN_R = crate::BitReader<SOC_GO_SLP1_IEN_A>;
impl SOC_GO_SLP1_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_GO_SLP1_IEN_A {
        match self.bits {
            false => SOC_GO_SLP1_IEN_A::DISABLE,
            true => SOC_GO_SLP1_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SOC_GO_SLP1_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SOC_GO_SLP1_IEN_A::ENABLE
    }
}
#[doc = "Field `soc_go_slp1_ien` writer - SoC core entering SLEEP1 mode interrupt enable bit"]
pub type SOC_GO_SLP1_IEN_W<'a> =
    crate::BitWriter<'a, u32, SYS_CTL_INT2_EN_SPEC, SOC_GO_SLP1_IEN_A, 1>;
impl<'a> SOC_GO_SLP1_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SOC_GO_SLP1_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SOC_GO_SLP1_IEN_A::ENABLE)
    }
}
#[doc = "SoC core entering SLEEP0 mode interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_GO_SLP0_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<SOC_GO_SLP0_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_GO_SLP0_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_go_slp0_ien` reader - SoC core entering SLEEP0 mode interrupt enable bit"]
pub type SOC_GO_SLP0_IEN_R = crate::BitReader<SOC_GO_SLP0_IEN_A>;
impl SOC_GO_SLP0_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_GO_SLP0_IEN_A {
        match self.bits {
            false => SOC_GO_SLP0_IEN_A::DISABLE,
            true => SOC_GO_SLP0_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SOC_GO_SLP0_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SOC_GO_SLP0_IEN_A::ENABLE
    }
}
#[doc = "Field `soc_go_slp0_ien` writer - SoC core entering SLEEP0 mode interrupt enable bit"]
pub type SOC_GO_SLP0_IEN_W<'a> =
    crate::BitWriter<'a, u32, SYS_CTL_INT2_EN_SPEC, SOC_GO_SLP0_IEN_A, 0>;
impl<'a> SOC_GO_SLP0_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SOC_GO_SLP0_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SOC_GO_SLP0_IEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 7 - ddrc dfs fail interrupt enable bit"]
    #[inline(always)]
    pub fn ddrc_dfs_fail_ien(&self) -> DDRC_DFS_FAIL_IEN_R {
        DDRC_DFS_FAIL_IEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - ddrc dfs success interrupt enable bit"]
    #[inline(always)]
    pub fn ddrc_dfs_success_ien(&self) -> DDRC_DFS_SUCCESS_IEN_R {
        DDRC_DFS_SUCCESS_IEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - SoC core wake up from DEEPSLEEP mode interrupt enable bit"]
    #[inline(always)]
    pub fn soc_dslp_wakup_ien(&self) -> SOC_DSLP_WAKUP_IEN_R {
        SOC_DSLP_WAKUP_IEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - SoC core wake up from SLEEP1 mode interrupt enable bit"]
    #[inline(always)]
    pub fn soc_slp1_wakup_ien(&self) -> SOC_SLP1_WAKUP_IEN_R {
        SOC_SLP1_WAKUP_IEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - SoC core wake up from SLEEP0 mode interrupt enable bit"]
    #[inline(always)]
    pub fn soc_slp0_wakup_ien(&self) -> SOC_SLP0_WAKUP_IEN_R {
        SOC_SLP0_WAKUP_IEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - SoC core entering DEEPSLEEP mode interrupt enable bit"]
    #[inline(always)]
    pub fn soc_go_dslp_ien(&self) -> SOC_GO_DSLP_IEN_R {
        SOC_GO_DSLP_IEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - SoC core entering SLEEP1 mode interrupt enable bit"]
    #[inline(always)]
    pub fn soc_go_slp1_ien(&self) -> SOC_GO_SLP1_IEN_R {
        SOC_GO_SLP1_IEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - SoC core entering SLEEP0 mode interrupt enable bit"]
    #[inline(always)]
    pub fn soc_go_slp0_ien(&self) -> SOC_GO_SLP0_IEN_R {
        SOC_GO_SLP0_IEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - ddrc dfs fail interrupt enable bit"]
    #[inline(always)]
    pub fn ddrc_dfs_fail_ien(&mut self) -> DDRC_DFS_FAIL_IEN_W {
        DDRC_DFS_FAIL_IEN_W::new(self)
    }
    #[doc = "Bit 6 - ddrc dfs success interrupt enable bit"]
    #[inline(always)]
    pub fn ddrc_dfs_success_ien(&mut self) -> DDRC_DFS_SUCCESS_IEN_W {
        DDRC_DFS_SUCCESS_IEN_W::new(self)
    }
    #[doc = "Bit 5 - SoC core wake up from DEEPSLEEP mode interrupt enable bit"]
    #[inline(always)]
    pub fn soc_dslp_wakup_ien(&mut self) -> SOC_DSLP_WAKUP_IEN_W {
        SOC_DSLP_WAKUP_IEN_W::new(self)
    }
    #[doc = "Bit 4 - SoC core wake up from SLEEP1 mode interrupt enable bit"]
    #[inline(always)]
    pub fn soc_slp1_wakup_ien(&mut self) -> SOC_SLP1_WAKUP_IEN_W {
        SOC_SLP1_WAKUP_IEN_W::new(self)
    }
    #[doc = "Bit 3 - SoC core wake up from SLEEP0 mode interrupt enable bit"]
    #[inline(always)]
    pub fn soc_slp0_wakup_ien(&mut self) -> SOC_SLP0_WAKUP_IEN_W {
        SOC_SLP0_WAKUP_IEN_W::new(self)
    }
    #[doc = "Bit 2 - SoC core entering DEEPSLEEP mode interrupt enable bit"]
    #[inline(always)]
    pub fn soc_go_dslp_ien(&mut self) -> SOC_GO_DSLP_IEN_W {
        SOC_GO_DSLP_IEN_W::new(self)
    }
    #[doc = "Bit 1 - SoC core entering SLEEP1 mode interrupt enable bit"]
    #[inline(always)]
    pub fn soc_go_slp1_ien(&mut self) -> SOC_GO_SLP1_IEN_W {
        SOC_GO_SLP1_IEN_W::new(self)
    }
    #[doc = "Bit 0 - SoC core entering SLEEP0 mode interrupt enable bit"]
    #[inline(always)]
    pub fn soc_go_slp0_ien(&mut self) -> SOC_GO_SLP0_IEN_W {
        SOC_GO_SLP0_IEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sysctl module interrupt 2 interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_ctl_int2_en](index.html) module"]
pub struct SYS_CTL_INT2_EN_SPEC;
impl crate::RegisterSpec for SYS_CTL_INT2_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_ctl_int2_en::R](R) reader structure"]
impl crate::Readable for SYS_CTL_INT2_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_ctl_int2_en::W](W) writer structure"]
impl crate::Writable for SYS_CTL_INT2_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_CTL_INT2_EN to value 0"]
impl crate::Resettable for SYS_CTL_INT2_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
