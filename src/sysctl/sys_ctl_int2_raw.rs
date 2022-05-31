#[doc = "Register `SYS_CTL_INT2_RAW` reader"]
pub struct R(crate::R<SYS_CTL_INT2_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CTL_INT2_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CTL_INT2_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CTL_INT2_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_CTL_INT2_RAW` writer"]
pub struct W(crate::W<SYS_CTL_INT2_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_CTL_INT2_RAW_SPEC>;
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
impl From<crate::W<SYS_CTL_INT2_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_CTL_INT2_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ddrc dfs fail interrupt raw status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRC_DFS_FAIL_RAW_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: ddrc dfs fail"]
    PENDING = 0,
}
impl From<DDRC_DFS_FAIL_RAW_A> for bool {
    #[inline(always)]
    fn from(variant: DDRC_DFS_FAIL_RAW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ddrc_dfs_fail_raw` reader - ddrc dfs fail interrupt raw status bit"]
pub type DDRC_DFS_FAIL_RAW_R = crate::BitReader<DDRC_DFS_FAIL_RAW_A>;
impl DDRC_DFS_FAIL_RAW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRC_DFS_FAIL_RAW_A {
        match self.bits {
            true => DDRC_DFS_FAIL_RAW_A::NO_INTERRUPT,
            false => DDRC_DFS_FAIL_RAW_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == DDRC_DFS_FAIL_RAW_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DDRC_DFS_FAIL_RAW_A::PENDING
    }
}
#[doc = "Field `ddrc_dfs_fail_raw` writer - ddrc dfs fail interrupt raw status bit"]
pub type DDRC_DFS_FAIL_RAW_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT2_RAW_SPEC, DDRC_DFS_FAIL_RAW_A, 7>;
impl<'a> DDRC_DFS_FAIL_RAW_W<'a> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(DDRC_DFS_FAIL_RAW_A::NO_INTERRUPT)
    }
    #[doc = "ddrc dfs fail"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(DDRC_DFS_FAIL_RAW_A::PENDING)
    }
}
#[doc = "ddrc dfs success interrupt raw status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRC_DFS_SUCCESS_RAW_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: ddrc dfs success"]
    PENDING = 0,
}
impl From<DDRC_DFS_SUCCESS_RAW_A> for bool {
    #[inline(always)]
    fn from(variant: DDRC_DFS_SUCCESS_RAW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ddrc_dfs_success_raw` reader - ddrc dfs success interrupt raw status bit"]
pub type DDRC_DFS_SUCCESS_RAW_R = crate::BitReader<DDRC_DFS_SUCCESS_RAW_A>;
impl DDRC_DFS_SUCCESS_RAW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRC_DFS_SUCCESS_RAW_A {
        match self.bits {
            true => DDRC_DFS_SUCCESS_RAW_A::NO_INTERRUPT,
            false => DDRC_DFS_SUCCESS_RAW_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == DDRC_DFS_SUCCESS_RAW_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DDRC_DFS_SUCCESS_RAW_A::PENDING
    }
}
#[doc = "Field `ddrc_dfs_success_raw` writer - ddrc dfs success interrupt raw status bit"]
pub type DDRC_DFS_SUCCESS_RAW_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT2_RAW_SPEC, DDRC_DFS_SUCCESS_RAW_A, 6>;
impl<'a> DDRC_DFS_SUCCESS_RAW_W<'a> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(DDRC_DFS_SUCCESS_RAW_A::NO_INTERRUPT)
    }
    #[doc = "ddrc dfs success"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(DDRC_DFS_SUCCESS_RAW_A::PENDING)
    }
}
#[doc = "SoC core wake up from DEEPSLEEP mode interrupt raw status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_DSLP_WAKUP_RAW_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: SoC core wake up from DEEPSLEEP mode"]
    PENDING = 0,
}
impl From<SOC_DSLP_WAKUP_RAW_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_DSLP_WAKUP_RAW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_dslp_wakup_raw` reader - SoC core wake up from DEEPSLEEP mode interrupt raw status bit"]
pub type SOC_DSLP_WAKUP_RAW_R = crate::BitReader<SOC_DSLP_WAKUP_RAW_A>;
impl SOC_DSLP_WAKUP_RAW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_DSLP_WAKUP_RAW_A {
        match self.bits {
            true => SOC_DSLP_WAKUP_RAW_A::NO_INTERRUPT,
            false => SOC_DSLP_WAKUP_RAW_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == SOC_DSLP_WAKUP_RAW_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SOC_DSLP_WAKUP_RAW_A::PENDING
    }
}
#[doc = "Field `soc_dslp_wakup_raw` writer - SoC core wake up from DEEPSLEEP mode interrupt raw status bit"]
pub type SOC_DSLP_WAKUP_RAW_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT2_RAW_SPEC, SOC_DSLP_WAKUP_RAW_A, 5>;
impl<'a> SOC_DSLP_WAKUP_RAW_W<'a> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(SOC_DSLP_WAKUP_RAW_A::NO_INTERRUPT)
    }
    #[doc = "SoC core wake up from DEEPSLEEP mode"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(SOC_DSLP_WAKUP_RAW_A::PENDING)
    }
}
#[doc = "SoC core wake up from SLEEP1 mode interrupt raw status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_SLP1_WAKUP_RAW_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: SoC core wake up from SLEEP1 mode"]
    PENDING = 0,
}
impl From<SOC_SLP1_WAKUP_RAW_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_SLP1_WAKUP_RAW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_slp1_wakup_raw` reader - SoC core wake up from SLEEP1 mode interrupt raw status bit"]
pub type SOC_SLP1_WAKUP_RAW_R = crate::BitReader<SOC_SLP1_WAKUP_RAW_A>;
impl SOC_SLP1_WAKUP_RAW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_SLP1_WAKUP_RAW_A {
        match self.bits {
            true => SOC_SLP1_WAKUP_RAW_A::NO_INTERRUPT,
            false => SOC_SLP1_WAKUP_RAW_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == SOC_SLP1_WAKUP_RAW_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SOC_SLP1_WAKUP_RAW_A::PENDING
    }
}
#[doc = "Field `soc_slp1_wakup_raw` writer - SoC core wake up from SLEEP1 mode interrupt raw status bit"]
pub type SOC_SLP1_WAKUP_RAW_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT2_RAW_SPEC, SOC_SLP1_WAKUP_RAW_A, 4>;
impl<'a> SOC_SLP1_WAKUP_RAW_W<'a> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(SOC_SLP1_WAKUP_RAW_A::NO_INTERRUPT)
    }
    #[doc = "SoC core wake up from SLEEP1 mode"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(SOC_SLP1_WAKUP_RAW_A::PENDING)
    }
}
#[doc = "SoC core wake up from SLEEP0 mode interrupt raw status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_SLP0_WAKUP_RAW_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: SoC core wake up from SLEEP0 mode"]
    PENDING = 0,
}
impl From<SOC_SLP0_WAKUP_RAW_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_SLP0_WAKUP_RAW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_slp0_wakup_raw` reader - SoC core wake up from SLEEP0 mode interrupt raw status bit"]
pub type SOC_SLP0_WAKUP_RAW_R = crate::BitReader<SOC_SLP0_WAKUP_RAW_A>;
impl SOC_SLP0_WAKUP_RAW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_SLP0_WAKUP_RAW_A {
        match self.bits {
            true => SOC_SLP0_WAKUP_RAW_A::NO_INTERRUPT,
            false => SOC_SLP0_WAKUP_RAW_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == SOC_SLP0_WAKUP_RAW_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SOC_SLP0_WAKUP_RAW_A::PENDING
    }
}
#[doc = "Field `soc_slp0_wakup_raw` writer - SoC core wake up from SLEEP0 mode interrupt raw status bit"]
pub type SOC_SLP0_WAKUP_RAW_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT2_RAW_SPEC, SOC_SLP0_WAKUP_RAW_A, 3>;
impl<'a> SOC_SLP0_WAKUP_RAW_W<'a> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(SOC_SLP0_WAKUP_RAW_A::NO_INTERRUPT)
    }
    #[doc = "SoC core wake up from SLEEP0 mode"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(SOC_SLP0_WAKUP_RAW_A::PENDING)
    }
}
#[doc = "SoC core entering DEEPSLEEP mode interrupt raw status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_GO_DSLP_RAW_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: SoC core entering DEEPSLEEP mode"]
    PENDING = 0,
}
impl From<SOC_GO_DSLP_RAW_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_GO_DSLP_RAW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_go_dslp_raw` reader - SoC core entering DEEPSLEEP mode interrupt raw status bit"]
pub type SOC_GO_DSLP_RAW_R = crate::BitReader<SOC_GO_DSLP_RAW_A>;
impl SOC_GO_DSLP_RAW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_GO_DSLP_RAW_A {
        match self.bits {
            true => SOC_GO_DSLP_RAW_A::NO_INTERRUPT,
            false => SOC_GO_DSLP_RAW_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == SOC_GO_DSLP_RAW_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SOC_GO_DSLP_RAW_A::PENDING
    }
}
#[doc = "Field `soc_go_dslp_raw` writer - SoC core entering DEEPSLEEP mode interrupt raw status bit"]
pub type SOC_GO_DSLP_RAW_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT2_RAW_SPEC, SOC_GO_DSLP_RAW_A, 2>;
impl<'a> SOC_GO_DSLP_RAW_W<'a> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(SOC_GO_DSLP_RAW_A::NO_INTERRUPT)
    }
    #[doc = "SoC core entering DEEPSLEEP mode"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(SOC_GO_DSLP_RAW_A::PENDING)
    }
}
#[doc = "SoC core entering SLEEP1 mode interrupt raw status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_GO_SLP1_RAW_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: SoC core entering SLEEP1 mode"]
    PENDING = 0,
}
impl From<SOC_GO_SLP1_RAW_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_GO_SLP1_RAW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_go_slp1_raw` reader - SoC core entering SLEEP1 mode interrupt raw status bit"]
pub type SOC_GO_SLP1_RAW_R = crate::BitReader<SOC_GO_SLP1_RAW_A>;
impl SOC_GO_SLP1_RAW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_GO_SLP1_RAW_A {
        match self.bits {
            true => SOC_GO_SLP1_RAW_A::NO_INTERRUPT,
            false => SOC_GO_SLP1_RAW_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == SOC_GO_SLP1_RAW_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SOC_GO_SLP1_RAW_A::PENDING
    }
}
#[doc = "Field `soc_go_slp1_raw` writer - SoC core entering SLEEP1 mode interrupt raw status bit"]
pub type SOC_GO_SLP1_RAW_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT2_RAW_SPEC, SOC_GO_SLP1_RAW_A, 1>;
impl<'a> SOC_GO_SLP1_RAW_W<'a> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(SOC_GO_SLP1_RAW_A::NO_INTERRUPT)
    }
    #[doc = "SoC core entering SLEEP1 mode"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(SOC_GO_SLP1_RAW_A::PENDING)
    }
}
#[doc = "SoC core entering SLEEP0 mode interrupt raw status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_GO_SLP0_RAW_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: SoC core entering SLEEP0 mode"]
    PENDING = 0,
}
impl From<SOC_GO_SLP0_RAW_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_GO_SLP0_RAW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_go_slp0_raw` reader - SoC core entering SLEEP0 mode interrupt raw status bit"]
pub type SOC_GO_SLP0_RAW_R = crate::BitReader<SOC_GO_SLP0_RAW_A>;
impl SOC_GO_SLP0_RAW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_GO_SLP0_RAW_A {
        match self.bits {
            true => SOC_GO_SLP0_RAW_A::NO_INTERRUPT,
            false => SOC_GO_SLP0_RAW_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == SOC_GO_SLP0_RAW_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SOC_GO_SLP0_RAW_A::PENDING
    }
}
#[doc = "Field `soc_go_slp0_raw` writer - SoC core entering SLEEP0 mode interrupt raw status bit"]
pub type SOC_GO_SLP0_RAW_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT2_RAW_SPEC, SOC_GO_SLP0_RAW_A, 0>;
impl<'a> SOC_GO_SLP0_RAW_W<'a> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(SOC_GO_SLP0_RAW_A::NO_INTERRUPT)
    }
    #[doc = "SoC core entering SLEEP0 mode"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(SOC_GO_SLP0_RAW_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 7 - ddrc dfs fail interrupt raw status bit"]
    #[inline(always)]
    pub fn ddrc_dfs_fail_raw(&self) -> DDRC_DFS_FAIL_RAW_R {
        DDRC_DFS_FAIL_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - ddrc dfs success interrupt raw status bit"]
    #[inline(always)]
    pub fn ddrc_dfs_success_raw(&self) -> DDRC_DFS_SUCCESS_RAW_R {
        DDRC_DFS_SUCCESS_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - SoC core wake up from DEEPSLEEP mode interrupt raw status bit"]
    #[inline(always)]
    pub fn soc_dslp_wakup_raw(&self) -> SOC_DSLP_WAKUP_RAW_R {
        SOC_DSLP_WAKUP_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - SoC core wake up from SLEEP1 mode interrupt raw status bit"]
    #[inline(always)]
    pub fn soc_slp1_wakup_raw(&self) -> SOC_SLP1_WAKUP_RAW_R {
        SOC_SLP1_WAKUP_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - SoC core wake up from SLEEP0 mode interrupt raw status bit"]
    #[inline(always)]
    pub fn soc_slp0_wakup_raw(&self) -> SOC_SLP0_WAKUP_RAW_R {
        SOC_SLP0_WAKUP_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - SoC core entering DEEPSLEEP mode interrupt raw status bit"]
    #[inline(always)]
    pub fn soc_go_dslp_raw(&self) -> SOC_GO_DSLP_RAW_R {
        SOC_GO_DSLP_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - SoC core entering SLEEP1 mode interrupt raw status bit"]
    #[inline(always)]
    pub fn soc_go_slp1_raw(&self) -> SOC_GO_SLP1_RAW_R {
        SOC_GO_SLP1_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - SoC core entering SLEEP0 mode interrupt raw status bit"]
    #[inline(always)]
    pub fn soc_go_slp0_raw(&self) -> SOC_GO_SLP0_RAW_R {
        SOC_GO_SLP0_RAW_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - ddrc dfs fail interrupt raw status bit"]
    #[inline(always)]
    pub fn ddrc_dfs_fail_raw(&mut self) -> DDRC_DFS_FAIL_RAW_W {
        DDRC_DFS_FAIL_RAW_W::new(self)
    }
    #[doc = "Bit 6 - ddrc dfs success interrupt raw status bit"]
    #[inline(always)]
    pub fn ddrc_dfs_success_raw(&mut self) -> DDRC_DFS_SUCCESS_RAW_W {
        DDRC_DFS_SUCCESS_RAW_W::new(self)
    }
    #[doc = "Bit 5 - SoC core wake up from DEEPSLEEP mode interrupt raw status bit"]
    #[inline(always)]
    pub fn soc_dslp_wakup_raw(&mut self) -> SOC_DSLP_WAKUP_RAW_W {
        SOC_DSLP_WAKUP_RAW_W::new(self)
    }
    #[doc = "Bit 4 - SoC core wake up from SLEEP1 mode interrupt raw status bit"]
    #[inline(always)]
    pub fn soc_slp1_wakup_raw(&mut self) -> SOC_SLP1_WAKUP_RAW_W {
        SOC_SLP1_WAKUP_RAW_W::new(self)
    }
    #[doc = "Bit 3 - SoC core wake up from SLEEP0 mode interrupt raw status bit"]
    #[inline(always)]
    pub fn soc_slp0_wakup_raw(&mut self) -> SOC_SLP0_WAKUP_RAW_W {
        SOC_SLP0_WAKUP_RAW_W::new(self)
    }
    #[doc = "Bit 2 - SoC core entering DEEPSLEEP mode interrupt raw status bit"]
    #[inline(always)]
    pub fn soc_go_dslp_raw(&mut self) -> SOC_GO_DSLP_RAW_W {
        SOC_GO_DSLP_RAW_W::new(self)
    }
    #[doc = "Bit 1 - SoC core entering SLEEP1 mode interrupt raw status bit"]
    #[inline(always)]
    pub fn soc_go_slp1_raw(&mut self) -> SOC_GO_SLP1_RAW_W {
        SOC_GO_SLP1_RAW_W::new(self)
    }
    #[doc = "Bit 0 - SoC core entering SLEEP0 mode interrupt raw status bit"]
    #[inline(always)]
    pub fn soc_go_slp0_raw(&mut self) -> SOC_GO_SLP0_RAW_W {
        SOC_GO_SLP0_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sysctl module interrupt 2 raw status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_ctl_int2_raw](index.html) module"]
pub struct SYS_CTL_INT2_RAW_SPEC;
impl crate::RegisterSpec for SYS_CTL_INT2_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_ctl_int2_raw::R](R) reader structure"]
impl crate::Readable for SYS_CTL_INT2_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_ctl_int2_raw::W](W) writer structure"]
impl crate::Writable for SYS_CTL_INT2_RAW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_CTL_INT2_RAW to value 0"]
impl crate::Resettable for SYS_CTL_INT2_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
