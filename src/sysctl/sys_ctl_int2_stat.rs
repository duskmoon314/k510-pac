#[doc = "Register `SYS_CTL_INT2_STAT` reader"]
pub struct R(crate::R<SYS_CTL_INT2_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CTL_INT2_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CTL_INT2_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CTL_INT2_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ddrc dfs fail interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRC_DFS_FAIL_STAT_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt happens"]
    PENDING = 1,
}
impl From<DDRC_DFS_FAIL_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: DDRC_DFS_FAIL_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ddrc_dfs_fail_stat` reader - ddrc dfs fail interrupt status bit"]
pub type DDRC_DFS_FAIL_STAT_R = crate::BitReader<DDRC_DFS_FAIL_STAT_A>;
impl DDRC_DFS_FAIL_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRC_DFS_FAIL_STAT_A {
        match self.bits {
            false => DDRC_DFS_FAIL_STAT_A::NO_INTERRUPT,
            true => DDRC_DFS_FAIL_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == DDRC_DFS_FAIL_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DDRC_DFS_FAIL_STAT_A::PENDING
    }
}
#[doc = "ddrc dfs success interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRC_DFS_SUCCESS_STAT_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt happens"]
    PENDING = 1,
}
impl From<DDRC_DFS_SUCCESS_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: DDRC_DFS_SUCCESS_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ddrc_dfs_success_stat` reader - ddrc dfs success interrupt status bit"]
pub type DDRC_DFS_SUCCESS_STAT_R = crate::BitReader<DDRC_DFS_SUCCESS_STAT_A>;
impl DDRC_DFS_SUCCESS_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRC_DFS_SUCCESS_STAT_A {
        match self.bits {
            false => DDRC_DFS_SUCCESS_STAT_A::NO_INTERRUPT,
            true => DDRC_DFS_SUCCESS_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == DDRC_DFS_SUCCESS_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DDRC_DFS_SUCCESS_STAT_A::PENDING
    }
}
#[doc = "SoC core wake up from DEEPSLEEP mode interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_DSLP_WAKUP_STAT_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt happens"]
    PENDING = 1,
}
impl From<SOC_DSLP_WAKUP_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_DSLP_WAKUP_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_dslp_wakup_stat` reader - SoC core wake up from DEEPSLEEP mode interrupt status bit"]
pub type SOC_DSLP_WAKUP_STAT_R = crate::BitReader<SOC_DSLP_WAKUP_STAT_A>;
impl SOC_DSLP_WAKUP_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_DSLP_WAKUP_STAT_A {
        match self.bits {
            false => SOC_DSLP_WAKUP_STAT_A::NO_INTERRUPT,
            true => SOC_DSLP_WAKUP_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == SOC_DSLP_WAKUP_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SOC_DSLP_WAKUP_STAT_A::PENDING
    }
}
#[doc = "SoC core wake up from SLEEP1 mode interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_SLP1_WAKUP_STAT_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt happens"]
    PENDING = 1,
}
impl From<SOC_SLP1_WAKUP_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_SLP1_WAKUP_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_slp1_wakup_stat` reader - SoC core wake up from SLEEP1 mode interrupt status bit"]
pub type SOC_SLP1_WAKUP_STAT_R = crate::BitReader<SOC_SLP1_WAKUP_STAT_A>;
impl SOC_SLP1_WAKUP_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_SLP1_WAKUP_STAT_A {
        match self.bits {
            false => SOC_SLP1_WAKUP_STAT_A::NO_INTERRUPT,
            true => SOC_SLP1_WAKUP_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == SOC_SLP1_WAKUP_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SOC_SLP1_WAKUP_STAT_A::PENDING
    }
}
#[doc = "SoC core wake up from SLEEP0 mode interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_SLP0_WAKUP_STAT_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt happens"]
    PENDING = 1,
}
impl From<SOC_SLP0_WAKUP_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_SLP0_WAKUP_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_slp0_wakup_stat` reader - SoC core wake up from SLEEP0 mode interrupt status bit"]
pub type SOC_SLP0_WAKUP_STAT_R = crate::BitReader<SOC_SLP0_WAKUP_STAT_A>;
impl SOC_SLP0_WAKUP_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_SLP0_WAKUP_STAT_A {
        match self.bits {
            false => SOC_SLP0_WAKUP_STAT_A::NO_INTERRUPT,
            true => SOC_SLP0_WAKUP_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == SOC_SLP0_WAKUP_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SOC_SLP0_WAKUP_STAT_A::PENDING
    }
}
#[doc = "SoC core entering DEEPSLEEP mode interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_GO_DSLP_STAT_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt happens"]
    PENDING = 1,
}
impl From<SOC_GO_DSLP_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_GO_DSLP_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_go_dslp_stat` reader - SoC core entering DEEPSLEEP mode interrupt status bit"]
pub type SOC_GO_DSLP_STAT_R = crate::BitReader<SOC_GO_DSLP_STAT_A>;
impl SOC_GO_DSLP_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_GO_DSLP_STAT_A {
        match self.bits {
            false => SOC_GO_DSLP_STAT_A::NO_INTERRUPT,
            true => SOC_GO_DSLP_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == SOC_GO_DSLP_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SOC_GO_DSLP_STAT_A::PENDING
    }
}
#[doc = "SoC core entering SLEEP1 mode interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_GO_SLP1_STAT_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt happens"]
    PENDING = 1,
}
impl From<SOC_GO_SLP1_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_GO_SLP1_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_go_slp1_stat` reader - SoC core entering SLEEP1 mode interrupt status bit"]
pub type SOC_GO_SLP1_STAT_R = crate::BitReader<SOC_GO_SLP1_STAT_A>;
impl SOC_GO_SLP1_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_GO_SLP1_STAT_A {
        match self.bits {
            false => SOC_GO_SLP1_STAT_A::NO_INTERRUPT,
            true => SOC_GO_SLP1_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == SOC_GO_SLP1_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SOC_GO_SLP1_STAT_A::PENDING
    }
}
#[doc = "SoC core entering SLEEP0 mode interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_GO_SLP0_STAT_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt happens"]
    PENDING = 1,
}
impl From<SOC_GO_SLP0_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: SOC_GO_SLP0_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `soc_go_slp0_stat` reader - SoC core entering SLEEP0 mode interrupt status bit"]
pub type SOC_GO_SLP0_STAT_R = crate::BitReader<SOC_GO_SLP0_STAT_A>;
impl SOC_GO_SLP0_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_GO_SLP0_STAT_A {
        match self.bits {
            false => SOC_GO_SLP0_STAT_A::NO_INTERRUPT,
            true => SOC_GO_SLP0_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == SOC_GO_SLP0_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SOC_GO_SLP0_STAT_A::PENDING
    }
}
impl R {
    #[doc = "Bit 7 - ddrc dfs fail interrupt status bit"]
    #[inline(always)]
    pub fn ddrc_dfs_fail_stat(&self) -> DDRC_DFS_FAIL_STAT_R {
        DDRC_DFS_FAIL_STAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - ddrc dfs success interrupt status bit"]
    #[inline(always)]
    pub fn ddrc_dfs_success_stat(&self) -> DDRC_DFS_SUCCESS_STAT_R {
        DDRC_DFS_SUCCESS_STAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - SoC core wake up from DEEPSLEEP mode interrupt status bit"]
    #[inline(always)]
    pub fn soc_dslp_wakup_stat(&self) -> SOC_DSLP_WAKUP_STAT_R {
        SOC_DSLP_WAKUP_STAT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - SoC core wake up from SLEEP1 mode interrupt status bit"]
    #[inline(always)]
    pub fn soc_slp1_wakup_stat(&self) -> SOC_SLP1_WAKUP_STAT_R {
        SOC_SLP1_WAKUP_STAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - SoC core wake up from SLEEP0 mode interrupt status bit"]
    #[inline(always)]
    pub fn soc_slp0_wakup_stat(&self) -> SOC_SLP0_WAKUP_STAT_R {
        SOC_SLP0_WAKUP_STAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - SoC core entering DEEPSLEEP mode interrupt status bit"]
    #[inline(always)]
    pub fn soc_go_dslp_stat(&self) -> SOC_GO_DSLP_STAT_R {
        SOC_GO_DSLP_STAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - SoC core entering SLEEP1 mode interrupt status bit"]
    #[inline(always)]
    pub fn soc_go_slp1_stat(&self) -> SOC_GO_SLP1_STAT_R {
        SOC_GO_SLP1_STAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - SoC core entering SLEEP0 mode interrupt status bit"]
    #[inline(always)]
    pub fn soc_go_slp0_stat(&self) -> SOC_GO_SLP0_STAT_R {
        SOC_GO_SLP0_STAT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Sysctl module interrupt 2 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_ctl_int2_stat](index.html) module"]
pub struct SYS_CTL_INT2_STAT_SPEC;
impl crate::RegisterSpec for SYS_CTL_INT2_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_ctl_int2_stat::R](R) reader structure"]
impl crate::Readable for SYS_CTL_INT2_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYS_CTL_INT2_STAT to value 0"]
impl crate::Resettable for SYS_CTL_INT2_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
