#[doc = "Register `SYS_CTL_INT0_EN` reader"]
pub struct R(crate::R<SYS_CTL_INT0_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CTL_INT0_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CTL_INT0_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CTL_INT0_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_CTL_INT0_EN` writer"]
pub struct W(crate::W<SYS_CTL_INT0_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_CTL_INT0_EN_SPEC>;
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
impl From<crate::W<SYS_CTL_INT0_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_CTL_INT0_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB power domain go power-off interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_USB_PD_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<PD_USB_PD_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: PD_USB_PD_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_usb_pd_ien` reader - USB power domain go power-off interrupt enable bit"]
pub type PD_USB_PD_IEN_R = crate::BitReader<PD_USB_PD_IEN_A>;
impl PD_USB_PD_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_USB_PD_IEN_A {
        match self.bits {
            false => PD_USB_PD_IEN_A::DISABLE,
            true => PD_USB_PD_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PD_USB_PD_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PD_USB_PD_IEN_A::ENABLE
    }
}
#[doc = "Field `pd_usb_pd_ien` writer - USB power domain go power-off interrupt enable bit"]
pub type PD_USB_PD_IEN_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT0_EN_SPEC, PD_USB_PD_IEN_A, 17>;
impl<'a> PD_USB_PD_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PD_USB_PD_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PD_USB_PD_IEN_A::ENABLE)
    }
}
#[doc = "H264 power domain go power-off interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_H264_PD_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<PD_H264_PD_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: PD_H264_PD_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_h264_pd_ien` reader - H264 power domain go power-off interrupt enable bit"]
pub type PD_H264_PD_IEN_R = crate::BitReader<PD_H264_PD_IEN_A>;
impl PD_H264_PD_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_H264_PD_IEN_A {
        match self.bits {
            false => PD_H264_PD_IEN_A::DISABLE,
            true => PD_H264_PD_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PD_H264_PD_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PD_H264_PD_IEN_A::ENABLE
    }
}
#[doc = "Field `pd_h264_pd_ien` writer - H264 power domain go power-off interrupt enable bit"]
pub type PD_H264_PD_IEN_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT0_EN_SPEC, PD_H264_PD_IEN_A, 16>;
impl<'a> PD_H264_PD_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PD_H264_PD_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PD_H264_PD_IEN_A::ENABLE)
    }
}
#[doc = "DISP power domain go power-off interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_DISP_PD_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<PD_DISP_PD_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: PD_DISP_PD_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_disp_pd_ien` reader - DISP power domain go power-off interrupt enable bit"]
pub type PD_DISP_PD_IEN_R = crate::BitReader<PD_DISP_PD_IEN_A>;
impl PD_DISP_PD_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_DISP_PD_IEN_A {
        match self.bits {
            false => PD_DISP_PD_IEN_A::DISABLE,
            true => PD_DISP_PD_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PD_DISP_PD_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PD_DISP_PD_IEN_A::ENABLE
    }
}
#[doc = "Field `pd_disp_pd_ien` writer - DISP power domain go power-off interrupt enable bit"]
pub type PD_DISP_PD_IEN_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT0_EN_SPEC, PD_DISP_PD_IEN_A, 15>;
impl<'a> PD_DISP_PD_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PD_DISP_PD_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PD_DISP_PD_IEN_A::ENABLE)
    }
}
#[doc = "ISP power domain go power-off interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_ISP_PD_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<PD_ISP_PD_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: PD_ISP_PD_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_isp_pd_ien` reader - ISP power domain go power-off interrupt enable bit"]
pub type PD_ISP_PD_IEN_R = crate::BitReader<PD_ISP_PD_IEN_A>;
impl PD_ISP_PD_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_ISP_PD_IEN_A {
        match self.bits {
            false => PD_ISP_PD_IEN_A::DISABLE,
            true => PD_ISP_PD_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PD_ISP_PD_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PD_ISP_PD_IEN_A::ENABLE
    }
}
#[doc = "Field `pd_isp_pd_ien` writer - ISP power domain go power-off interrupt enable bit"]
pub type PD_ISP_PD_IEN_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT0_EN_SPEC, PD_ISP_PD_IEN_A, 10>;
impl<'a> PD_ISP_PD_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PD_ISP_PD_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PD_ISP_PD_IEN_A::ENABLE)
    }
}
#[doc = "Memory Controller power domain go power-off interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_MCTL_PD_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<PD_MCTL_PD_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: PD_MCTL_PD_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_mctl_pd_ien` reader - Memory Controller power domain go power-off interrupt enable bit"]
pub type PD_MCTL_PD_IEN_R = crate::BitReader<PD_MCTL_PD_IEN_A>;
impl PD_MCTL_PD_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_MCTL_PD_IEN_A {
        match self.bits {
            false => PD_MCTL_PD_IEN_A::DISABLE,
            true => PD_MCTL_PD_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PD_MCTL_PD_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PD_MCTL_PD_IEN_A::ENABLE
    }
}
#[doc = "Field `pd_mctl_pd_ien` writer - Memory Controller power domain go power-off interrupt enable bit"]
pub type PD_MCTL_PD_IEN_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT0_EN_SPEC, PD_MCTL_PD_IEN_A, 9>;
impl<'a> PD_MCTL_PD_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PD_MCTL_PD_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PD_MCTL_PD_IEN_A::ENABLE)
    }
}
#[doc = "SRAM1 power domain go power-off interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_RAM1_PD_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<PD_RAM1_PD_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: PD_RAM1_PD_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_ram1_pd_ien` reader - SRAM1 power domain go power-off interrupt enable bit"]
pub type PD_RAM1_PD_IEN_R = crate::BitReader<PD_RAM1_PD_IEN_A>;
impl PD_RAM1_PD_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_RAM1_PD_IEN_A {
        match self.bits {
            false => PD_RAM1_PD_IEN_A::DISABLE,
            true => PD_RAM1_PD_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PD_RAM1_PD_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PD_RAM1_PD_IEN_A::ENABLE
    }
}
#[doc = "Field `pd_ram1_pd_ien` writer - SRAM1 power domain go power-off interrupt enable bit"]
pub type PD_RAM1_PD_IEN_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT0_EN_SPEC, PD_RAM1_PD_IEN_A, 8>;
impl<'a> PD_RAM1_PD_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PD_RAM1_PD_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PD_RAM1_PD_IEN_A::ENABLE)
    }
}
#[doc = "SRAM0 power domain go power-off interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_RAM0_PD_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<PD_RAM0_PD_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: PD_RAM0_PD_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_ram0_pd_ien` reader - SRAM0 power domain go power-off interrupt enable bit"]
pub type PD_RAM0_PD_IEN_R = crate::BitReader<PD_RAM0_PD_IEN_A>;
impl PD_RAM0_PD_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_RAM0_PD_IEN_A {
        match self.bits {
            false => PD_RAM0_PD_IEN_A::DISABLE,
            true => PD_RAM0_PD_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PD_RAM0_PD_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PD_RAM0_PD_IEN_A::ENABLE
    }
}
#[doc = "Field `pd_ram0_pd_ien` writer - SRAM0 power domain go power-off interrupt enable bit"]
pub type PD_RAM0_PD_IEN_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT0_EN_SPEC, PD_RAM0_PD_IEN_A, 7>;
impl<'a> PD_RAM0_PD_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PD_RAM0_PD_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PD_RAM0_PD_IEN_A::ENABLE)
    }
}
#[doc = "Storage power domain go power-off interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_STOR_PD_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<PD_STOR_PD_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: PD_STOR_PD_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_stor_pd_ien` reader - Storage power domain go power-off interrupt enable bit"]
pub type PD_STOR_PD_IEN_R = crate::BitReader<PD_STOR_PD_IEN_A>;
impl PD_STOR_PD_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_STOR_PD_IEN_A {
        match self.bits {
            false => PD_STOR_PD_IEN_A::DISABLE,
            true => PD_STOR_PD_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PD_STOR_PD_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PD_STOR_PD_IEN_A::ENABLE
    }
}
#[doc = "Field `pd_stor_pd_ien` writer - Storage power domain go power-off interrupt enable bit"]
pub type PD_STOR_PD_IEN_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT0_EN_SPEC, PD_STOR_PD_IEN_A, 6>;
impl<'a> PD_STOR_PD_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PD_STOR_PD_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PD_STOR_PD_IEN_A::ENABLE)
    }
}
#[doc = "Peripheral power domain go power-off interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_PERI_PD_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<PD_PERI_PD_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: PD_PERI_PD_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_peri_pd_ien` reader - Peripheral power domain go power-off interrupt enable bit"]
pub type PD_PERI_PD_IEN_R = crate::BitReader<PD_PERI_PD_IEN_A>;
impl PD_PERI_PD_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_PERI_PD_IEN_A {
        match self.bits {
            false => PD_PERI_PD_IEN_A::DISABLE,
            true => PD_PERI_PD_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PD_PERI_PD_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PD_PERI_PD_IEN_A::ENABLE
    }
}
#[doc = "Field `pd_peri_pd_ien` writer - Peripheral power domain go power-off interrupt enable bit"]
pub type PD_PERI_PD_IEN_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT0_EN_SPEC, PD_PERI_PD_IEN_A, 5>;
impl<'a> PD_PERI_PD_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PD_PERI_PD_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PD_PERI_PD_IEN_A::ENABLE)
    }
}
#[doc = "Security power domain go power-off interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_SEC_PD_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<PD_SEC_PD_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: PD_SEC_PD_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_sec_pd_ien` reader - Security power domain go power-off interrupt enable bit"]
pub type PD_SEC_PD_IEN_R = crate::BitReader<PD_SEC_PD_IEN_A>;
impl PD_SEC_PD_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_SEC_PD_IEN_A {
        match self.bits {
            false => PD_SEC_PD_IEN_A::DISABLE,
            true => PD_SEC_PD_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PD_SEC_PD_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PD_SEC_PD_IEN_A::ENABLE
    }
}
#[doc = "Field `pd_sec_pd_ien` writer - Security power domain go power-off interrupt enable bit"]
pub type PD_SEC_PD_IEN_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT0_EN_SPEC, PD_SEC_PD_IEN_A, 4>;
impl<'a> PD_SEC_PD_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PD_SEC_PD_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PD_SEC_PD_IEN_A::ENABLE)
    }
}
#[doc = "GNNE power domain go power-off interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_GNNE_PD_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<PD_GNNE_PD_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: PD_GNNE_PD_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_gnne_pd_ien` reader - GNNE power domain go power-off interrupt enable bit"]
pub type PD_GNNE_PD_IEN_R = crate::BitReader<PD_GNNE_PD_IEN_A>;
impl PD_GNNE_PD_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_GNNE_PD_IEN_A {
        match self.bits {
            false => PD_GNNE_PD_IEN_A::DISABLE,
            true => PD_GNNE_PD_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PD_GNNE_PD_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PD_GNNE_PD_IEN_A::ENABLE
    }
}
#[doc = "Field `pd_gnne_pd_ien` writer - GNNE power domain go power-off interrupt enable bit"]
pub type PD_GNNE_PD_IEN_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT0_EN_SPEC, PD_GNNE_PD_IEN_A, 3>;
impl<'a> PD_GNNE_PD_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PD_GNNE_PD_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PD_GNNE_PD_IEN_A::ENABLE)
    }
}
#[doc = "CPUp (AX25P) power domain go power-off interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_CPUP_PD_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<PD_CPUP_PD_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: PD_CPUP_PD_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_cpup_pd_ien` reader - CPUp (AX25P) power domain go power-off interrupt enable bit"]
pub type PD_CPUP_PD_IEN_R = crate::BitReader<PD_CPUP_PD_IEN_A>;
impl PD_CPUP_PD_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_CPUP_PD_IEN_A {
        match self.bits {
            false => PD_CPUP_PD_IEN_A::DISABLE,
            true => PD_CPUP_PD_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PD_CPUP_PD_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PD_CPUP_PD_IEN_A::ENABLE
    }
}
#[doc = "Field `pd_cpup_pd_ien` writer - CPUp (AX25P) power domain go power-off interrupt enable bit"]
pub type PD_CPUP_PD_IEN_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT0_EN_SPEC, PD_CPUP_PD_IEN_A, 1>;
impl<'a> PD_CPUP_PD_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PD_CPUP_PD_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PD_CPUP_PD_IEN_A::ENABLE)
    }
}
#[doc = "CPUm (AX25M) power domain go power-off interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_CPUM_PD_IEN_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<PD_CPUM_PD_IEN_A> for bool {
    #[inline(always)]
    fn from(variant: PD_CPUM_PD_IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pd_cpum_pd_ien` reader - CPUm (AX25M) power domain go power-off interrupt enable bit"]
pub type PD_CPUM_PD_IEN_R = crate::BitReader<PD_CPUM_PD_IEN_A>;
impl PD_CPUM_PD_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_CPUM_PD_IEN_A {
        match self.bits {
            false => PD_CPUM_PD_IEN_A::DISABLE,
            true => PD_CPUM_PD_IEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PD_CPUM_PD_IEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PD_CPUM_PD_IEN_A::ENABLE
    }
}
#[doc = "Field `pd_cpum_pd_ien` writer - CPUm (AX25M) power domain go power-off interrupt enable bit"]
pub type PD_CPUM_PD_IEN_W<'a> =
    crate::BitWriter1C<'a, u32, SYS_CTL_INT0_EN_SPEC, PD_CPUM_PD_IEN_A, 0>;
impl<'a> PD_CPUM_PD_IEN_W<'a> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PD_CPUM_PD_IEN_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PD_CPUM_PD_IEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 17 - USB power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_usb_pd_ien(&self) -> PD_USB_PD_IEN_R {
        PD_USB_PD_IEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - H264 power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_h264_pd_ien(&self) -> PD_H264_PD_IEN_R {
        PD_H264_PD_IEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - DISP power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_disp_pd_ien(&self) -> PD_DISP_PD_IEN_R {
        PD_DISP_PD_IEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 10 - ISP power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_isp_pd_ien(&self) -> PD_ISP_PD_IEN_R {
        PD_ISP_PD_IEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Memory Controller power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_mctl_pd_ien(&self) -> PD_MCTL_PD_IEN_R {
        PD_MCTL_PD_IEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM1 power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_ram1_pd_ien(&self) -> PD_RAM1_PD_IEN_R {
        PD_RAM1_PD_IEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - SRAM0 power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_ram0_pd_ien(&self) -> PD_RAM0_PD_IEN_R {
        PD_RAM0_PD_IEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Storage power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_stor_pd_ien(&self) -> PD_STOR_PD_IEN_R {
        PD_STOR_PD_IEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_peri_pd_ien(&self) -> PD_PERI_PD_IEN_R {
        PD_PERI_PD_IEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Security power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_sec_pd_ien(&self) -> PD_SEC_PD_IEN_R {
        PD_SEC_PD_IEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - GNNE power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_gnne_pd_ien(&self) -> PD_GNNE_PD_IEN_R {
        PD_GNNE_PD_IEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 1 - CPUp (AX25P) power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_cpup_pd_ien(&self) -> PD_CPUP_PD_IEN_R {
        PD_CPUP_PD_IEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - CPUm (AX25M) power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_cpum_pd_ien(&self) -> PD_CPUM_PD_IEN_R {
        PD_CPUM_PD_IEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - USB power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_usb_pd_ien(&mut self) -> PD_USB_PD_IEN_W {
        PD_USB_PD_IEN_W::new(self)
    }
    #[doc = "Bit 16 - H264 power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_h264_pd_ien(&mut self) -> PD_H264_PD_IEN_W {
        PD_H264_PD_IEN_W::new(self)
    }
    #[doc = "Bit 15 - DISP power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_disp_pd_ien(&mut self) -> PD_DISP_PD_IEN_W {
        PD_DISP_PD_IEN_W::new(self)
    }
    #[doc = "Bit 10 - ISP power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_isp_pd_ien(&mut self) -> PD_ISP_PD_IEN_W {
        PD_ISP_PD_IEN_W::new(self)
    }
    #[doc = "Bit 9 - Memory Controller power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_mctl_pd_ien(&mut self) -> PD_MCTL_PD_IEN_W {
        PD_MCTL_PD_IEN_W::new(self)
    }
    #[doc = "Bit 8 - SRAM1 power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_ram1_pd_ien(&mut self) -> PD_RAM1_PD_IEN_W {
        PD_RAM1_PD_IEN_W::new(self)
    }
    #[doc = "Bit 7 - SRAM0 power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_ram0_pd_ien(&mut self) -> PD_RAM0_PD_IEN_W {
        PD_RAM0_PD_IEN_W::new(self)
    }
    #[doc = "Bit 6 - Storage power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_stor_pd_ien(&mut self) -> PD_STOR_PD_IEN_W {
        PD_STOR_PD_IEN_W::new(self)
    }
    #[doc = "Bit 5 - Peripheral power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_peri_pd_ien(&mut self) -> PD_PERI_PD_IEN_W {
        PD_PERI_PD_IEN_W::new(self)
    }
    #[doc = "Bit 4 - Security power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_sec_pd_ien(&mut self) -> PD_SEC_PD_IEN_W {
        PD_SEC_PD_IEN_W::new(self)
    }
    #[doc = "Bit 3 - GNNE power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_gnne_pd_ien(&mut self) -> PD_GNNE_PD_IEN_W {
        PD_GNNE_PD_IEN_W::new(self)
    }
    #[doc = "Bit 1 - CPUp (AX25P) power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_cpup_pd_ien(&mut self) -> PD_CPUP_PD_IEN_W {
        PD_CPUP_PD_IEN_W::new(self)
    }
    #[doc = "Bit 0 - CPUm (AX25M) power domain go power-off interrupt enable bit"]
    #[inline(always)]
    pub fn pd_cpum_pd_ien(&mut self) -> PD_CPUM_PD_IEN_W {
        PD_CPUM_PD_IEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sysctl module interrupt 0 interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_ctl_int0_en](index.html) module"]
pub struct SYS_CTL_INT0_EN_SPEC;
impl crate::RegisterSpec for SYS_CTL_INT0_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_ctl_int0_en::R](R) reader structure"]
impl crate::Readable for SYS_CTL_INT0_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_ctl_int0_en::W](W) writer structure"]
impl crate::Writable for SYS_CTL_INT0_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_CTL_INT0_EN to value 0"]
impl crate::Resettable for SYS_CTL_INT0_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
