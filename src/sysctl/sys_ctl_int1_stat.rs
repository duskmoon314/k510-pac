#[doc = "Register `SYS_CTL_INT1_STAT` reader"]
pub struct R(crate::R<SYS_CTL_INT1_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CTL_INT1_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CTL_INT1_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CTL_INT1_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "USB power domain go power-up interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_USB_PU_STAT_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: Interrupt happens"]
    PENDING = 0,
}
impl From<PD_USB_PU_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: PD_USB_PU_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_usb_pu_stat` reader - USB power domain go power-up interrupt status bit"]
pub type PD_USB_PU_STAT_R = crate::BitReader<PD_USB_PU_STAT_A>;
impl PD_USB_PU_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_USB_PU_STAT_A {
        match self.bits {
            true => PD_USB_PU_STAT_A::NO_INTERRUPT,
            false => PD_USB_PU_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PD_USB_PU_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD_USB_PU_STAT_A::PENDING
    }
}
#[doc = "H264 power domain go power-up interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_H264_PU_STAT_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: Interrupt happens"]
    PENDING = 0,
}
impl From<PD_H264_PU_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: PD_H264_PU_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_h264_pu_stat` reader - H264 power domain go power-up interrupt status bit"]
pub type PD_H264_PU_STAT_R = crate::BitReader<PD_H264_PU_STAT_A>;
impl PD_H264_PU_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_H264_PU_STAT_A {
        match self.bits {
            true => PD_H264_PU_STAT_A::NO_INTERRUPT,
            false => PD_H264_PU_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PD_H264_PU_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD_H264_PU_STAT_A::PENDING
    }
}
#[doc = "DISP power domain go power-up interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_DISP_PU_STAT_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: Interrupt happens"]
    PENDING = 0,
}
impl From<PD_DISP_PU_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: PD_DISP_PU_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_disp_pu_stat` reader - DISP power domain go power-up interrupt status bit"]
pub type PD_DISP_PU_STAT_R = crate::BitReader<PD_DISP_PU_STAT_A>;
impl PD_DISP_PU_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_DISP_PU_STAT_A {
        match self.bits {
            true => PD_DISP_PU_STAT_A::NO_INTERRUPT,
            false => PD_DISP_PU_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PD_DISP_PU_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD_DISP_PU_STAT_A::PENDING
    }
}
#[doc = "ISP power domain go power-up interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_ISP_PU_STAT_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: Interrupt happens"]
    PENDING = 0,
}
impl From<PD_ISP_PU_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: PD_ISP_PU_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_isp_pu_stat` reader - ISP power domain go power-up interrupt status bit"]
pub type PD_ISP_PU_STAT_R = crate::BitReader<PD_ISP_PU_STAT_A>;
impl PD_ISP_PU_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_ISP_PU_STAT_A {
        match self.bits {
            true => PD_ISP_PU_STAT_A::NO_INTERRUPT,
            false => PD_ISP_PU_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PD_ISP_PU_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD_ISP_PU_STAT_A::PENDING
    }
}
#[doc = "MCTL power domain go power-up interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_MCTL_PU_STAT_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: Interrupt happens"]
    PENDING = 0,
}
impl From<PD_MCTL_PU_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: PD_MCTL_PU_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_mctl_pu_stat` reader - MCTL power domain go power-up interrupt status bit"]
pub type PD_MCTL_PU_STAT_R = crate::BitReader<PD_MCTL_PU_STAT_A>;
impl PD_MCTL_PU_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_MCTL_PU_STAT_A {
        match self.bits {
            true => PD_MCTL_PU_STAT_A::NO_INTERRUPT,
            false => PD_MCTL_PU_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PD_MCTL_PU_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD_MCTL_PU_STAT_A::PENDING
    }
}
#[doc = "SRAM1 power domain go power-up interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_RAM1_PU_STAT_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: Interrupt happens"]
    PENDING = 0,
}
impl From<PD_RAM1_PU_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: PD_RAM1_PU_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_ram1_pu_stat` reader - SRAM1 power domain go power-up interrupt status bit"]
pub type PD_RAM1_PU_STAT_R = crate::BitReader<PD_RAM1_PU_STAT_A>;
impl PD_RAM1_PU_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_RAM1_PU_STAT_A {
        match self.bits {
            true => PD_RAM1_PU_STAT_A::NO_INTERRUPT,
            false => PD_RAM1_PU_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PD_RAM1_PU_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD_RAM1_PU_STAT_A::PENDING
    }
}
#[doc = "SRAM0 power domain go power-up interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_RAM0_PU_STAT_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: Interrupt happens"]
    PENDING = 0,
}
impl From<PD_RAM0_PU_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: PD_RAM0_PU_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_ram0_pu_stat` reader - SRAM0 power domain go power-up interrupt status bit"]
pub type PD_RAM0_PU_STAT_R = crate::BitReader<PD_RAM0_PU_STAT_A>;
impl PD_RAM0_PU_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_RAM0_PU_STAT_A {
        match self.bits {
            true => PD_RAM0_PU_STAT_A::NO_INTERRUPT,
            false => PD_RAM0_PU_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PD_RAM0_PU_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD_RAM0_PU_STAT_A::PENDING
    }
}
#[doc = "STOR power domain go power-up interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_STOR_PU_STAT_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: Interrupt happens"]
    PENDING = 0,
}
impl From<PD_STOR_PU_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: PD_STOR_PU_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_stor_pu_stat` reader - STOR power domain go power-up interrupt status bit"]
pub type PD_STOR_PU_STAT_R = crate::BitReader<PD_STOR_PU_STAT_A>;
impl PD_STOR_PU_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_STOR_PU_STAT_A {
        match self.bits {
            true => PD_STOR_PU_STAT_A::NO_INTERRUPT,
            false => PD_STOR_PU_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PD_STOR_PU_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD_STOR_PU_STAT_A::PENDING
    }
}
#[doc = "Peripheral power domain go power-up interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_PERI_PU_STAT_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: Interrupt happens"]
    PENDING = 0,
}
impl From<PD_PERI_PU_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: PD_PERI_PU_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_peri_pu_stat` reader - Peripheral power domain go power-up interrupt status bit"]
pub type PD_PERI_PU_STAT_R = crate::BitReader<PD_PERI_PU_STAT_A>;
impl PD_PERI_PU_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_PERI_PU_STAT_A {
        match self.bits {
            true => PD_PERI_PU_STAT_A::NO_INTERRUPT,
            false => PD_PERI_PU_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PD_PERI_PU_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD_PERI_PU_STAT_A::PENDING
    }
}
#[doc = "Security power domain go power-up interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_SEC_PU_STAT_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: Interrupt happens"]
    PENDING = 0,
}
impl From<PD_SEC_PU_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: PD_SEC_PU_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_sec_pu_stat` reader - Security power domain go power-up interrupt status bit"]
pub type PD_SEC_PU_STAT_R = crate::BitReader<PD_SEC_PU_STAT_A>;
impl PD_SEC_PU_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_SEC_PU_STAT_A {
        match self.bits {
            true => PD_SEC_PU_STAT_A::NO_INTERRUPT,
            false => PD_SEC_PU_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PD_SEC_PU_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD_SEC_PU_STAT_A::PENDING
    }
}
#[doc = "GNNE power domain go power-up interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_GNNE_PU_STAT_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: Interrupt happens"]
    PENDING = 0,
}
impl From<PD_GNNE_PU_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: PD_GNNE_PU_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_gnne_pu_stat` reader - GNNE power domain go power-up interrupt status bit"]
pub type PD_GNNE_PU_STAT_R = crate::BitReader<PD_GNNE_PU_STAT_A>;
impl PD_GNNE_PU_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_GNNE_PU_STAT_A {
        match self.bits {
            true => PD_GNNE_PU_STAT_A::NO_INTERRUPT,
            false => PD_GNNE_PU_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PD_GNNE_PU_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD_GNNE_PU_STAT_A::PENDING
    }
}
#[doc = "CPUp (AX25P) power domain go power-up interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_CPUP_PU_STAT_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: Interrupt happens"]
    PENDING = 0,
}
impl From<PD_CPUP_PU_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: PD_CPUP_PU_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_cpup_pu_stat` reader - CPUp (AX25P) power domain go power-up interrupt status bit"]
pub type PD_CPUP_PU_STAT_R = crate::BitReader<PD_CPUP_PU_STAT_A>;
impl PD_CPUP_PU_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_CPUP_PU_STAT_A {
        match self.bits {
            true => PD_CPUP_PU_STAT_A::NO_INTERRUPT,
            false => PD_CPUP_PU_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PD_CPUP_PU_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD_CPUP_PU_STAT_A::PENDING
    }
}
#[doc = "CPUm (AX25M) power domain go power-up interrupt status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_CPUM_PU_STAT_A {
    #[doc = "1: No interrupt"]
    NO_INTERRUPT = 1,
    #[doc = "0: Interrupt happens"]
    PENDING = 0,
}
impl From<PD_CPUM_PU_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: PD_CPUM_PU_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_cpum_pu_stat` reader - CPUm (AX25M) power domain go power-up interrupt status bit"]
pub type PD_CPUM_PU_STAT_R = crate::BitReader<PD_CPUM_PU_STAT_A>;
impl PD_CPUM_PU_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_CPUM_PU_STAT_A {
        match self.bits {
            true => PD_CPUM_PU_STAT_A::NO_INTERRUPT,
            false => PD_CPUM_PU_STAT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PD_CPUM_PU_STAT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PD_CPUM_PU_STAT_A::PENDING
    }
}
impl R {
    #[doc = "Bit 17 - USB power domain go power-up interrupt status bit"]
    #[inline(always)]
    pub fn pd_usb_pu_stat(&self) -> PD_USB_PU_STAT_R {
        PD_USB_PU_STAT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - H264 power domain go power-up interrupt status bit"]
    #[inline(always)]
    pub fn pd_h264_pu_stat(&self) -> PD_H264_PU_STAT_R {
        PD_H264_PU_STAT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - DISP power domain go power-up interrupt status bit"]
    #[inline(always)]
    pub fn pd_disp_pu_stat(&self) -> PD_DISP_PU_STAT_R {
        PD_DISP_PU_STAT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 10 - ISP power domain go power-up interrupt status bit"]
    #[inline(always)]
    pub fn pd_isp_pu_stat(&self) -> PD_ISP_PU_STAT_R {
        PD_ISP_PU_STAT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - MCTL power domain go power-up interrupt status bit"]
    #[inline(always)]
    pub fn pd_mctl_pu_stat(&self) -> PD_MCTL_PU_STAT_R {
        PD_MCTL_PU_STAT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM1 power domain go power-up interrupt status bit"]
    #[inline(always)]
    pub fn pd_ram1_pu_stat(&self) -> PD_RAM1_PU_STAT_R {
        PD_RAM1_PU_STAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - SRAM0 power domain go power-up interrupt status bit"]
    #[inline(always)]
    pub fn pd_ram0_pu_stat(&self) -> PD_RAM0_PU_STAT_R {
        PD_RAM0_PU_STAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - STOR power domain go power-up interrupt status bit"]
    #[inline(always)]
    pub fn pd_stor_pu_stat(&self) -> PD_STOR_PU_STAT_R {
        PD_STOR_PU_STAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral power domain go power-up interrupt status bit"]
    #[inline(always)]
    pub fn pd_peri_pu_stat(&self) -> PD_PERI_PU_STAT_R {
        PD_PERI_PU_STAT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Security power domain go power-up interrupt status bit"]
    #[inline(always)]
    pub fn pd_sec_pu_stat(&self) -> PD_SEC_PU_STAT_R {
        PD_SEC_PU_STAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - GNNE power domain go power-up interrupt status bit"]
    #[inline(always)]
    pub fn pd_gnne_pu_stat(&self) -> PD_GNNE_PU_STAT_R {
        PD_GNNE_PU_STAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 1 - CPUp (AX25P) power domain go power-up interrupt status bit"]
    #[inline(always)]
    pub fn pd_cpup_pu_stat(&self) -> PD_CPUP_PU_STAT_R {
        PD_CPUP_PU_STAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - CPUm (AX25M) power domain go power-up interrupt status bit"]
    #[inline(always)]
    pub fn pd_cpum_pu_stat(&self) -> PD_CPUM_PU_STAT_R {
        PD_CPUM_PU_STAT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Sysctl module interrupt 1 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_ctl_int1_stat](index.html) module"]
pub struct SYS_CTL_INT1_STAT_SPEC;
impl crate::RegisterSpec for SYS_CTL_INT1_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_ctl_int1_stat::R](R) reader structure"]
impl crate::Readable for SYS_CTL_INT1_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYS_CTL_INT1_STAT to value 0"]
impl crate::Resettable for SYS_CTL_INT1_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
