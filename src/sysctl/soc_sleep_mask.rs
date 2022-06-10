#[doc = "Register `SOC_SLEEP_MASK` reader"]
pub struct R(crate::R<SOC_SLEEP_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOC_SLEEP_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOC_SLEEP_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOC_SLEEP_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOC_SLEEP_MASK` writer"]
pub struct W(crate::W<SOC_SLEEP_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOC_SLEEP_MASK_SPEC>;
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
impl From<crate::W<SOC_SLEEP_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOC_SLEEP_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_usb_slp_mask` writer - Write enable for bit 13 (ub_slp_mask)"]
pub type WE_USB_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, bool, 29>;
#[doc = "Field `WE_h264_slp_mask` writer - Write enable for bit 12 (h264_slp_mask)"]
pub type WE_H264_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, bool, 28>;
#[doc = "Field `WE_disp_slp_mask` writer - Write enable for bit 11 (disp_slp_mask)"]
pub type WE_DISP_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, bool, 27>;
#[doc = "Field `WE_isp_slp_mask` writer - Write enable for bit 10 (isp_slp_mask)"]
pub type WE_ISP_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, bool, 26>;
#[doc = "Field `WE_sram1_slp_mask` writer - Write enable for bit 9 (sram1_slp_mask)"]
pub type WE_SRAM1_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, bool, 25>;
#[doc = "Field `WE_sram0_slp_mask` writer - Write enable for bit 8 (sram0_slp_mask)"]
pub type WE_SRAM0_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, bool, 24>;
#[doc = "Field `WE_mctl_slp_mask` writer - Write enable for bit 7 (mctl_slp_mask)"]
pub type WE_MCTL_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, bool, 23>;
#[doc = "Field `WE_peri_slp_mask` writer - Write enable for bit 6 (peri_slp_mask)"]
pub type WE_PERI_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, bool, 22>;
#[doc = "Field `WE_stor_slp_mask` writer - Write enable for bit 5 (stor_slp_mask)"]
pub type WE_STOR_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, bool, 21>;
#[doc = "Field `WE_gnne_slp_mask` writer - Write enable for bit 3 (gnne_slp_mask)"]
pub type WE_GNNE_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, bool, 19>;
#[doc = "PD_USB sleep mask bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_SLP_MASK_A {
    #[doc = "0: Unmask PD_USB sleep"]
    UNMASK = 0,
    #[doc = "1: Mask PD_USB sleep"]
    MASK = 1,
}
impl From<USB_SLP_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: USB_SLP_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `usb_slp_mask` reader - PD_USB sleep mask bit"]
pub type USB_SLP_MASK_R = crate::BitReader<USB_SLP_MASK_A>;
impl USB_SLP_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_SLP_MASK_A {
        match self.bits {
            false => USB_SLP_MASK_A::UNMASK,
            true => USB_SLP_MASK_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == USB_SLP_MASK_A::UNMASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == USB_SLP_MASK_A::MASK
    }
}
#[doc = "Field `usb_slp_mask` writer - PD_USB sleep mask bit"]
pub type USB_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, USB_SLP_MASK_A, 13>;
impl<'a> USB_SLP_MASK_W<'a> {
    #[doc = "Unmask PD_USB sleep"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(USB_SLP_MASK_A::UNMASK)
    }
    #[doc = "Mask PD_USB sleep"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(USB_SLP_MASK_A::MASK)
    }
}
#[doc = "PD_H264 sleep mask bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum H264_SLP_MASK_A {
    #[doc = "0: Unmask PD_H264 sleep"]
    UNMASK = 0,
    #[doc = "1: Mask PD_H264 sleep"]
    MASK = 1,
}
impl From<H264_SLP_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: H264_SLP_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `h264_slp_mask` reader - PD_H264 sleep mask bit"]
pub type H264_SLP_MASK_R = crate::BitReader<H264_SLP_MASK_A>;
impl H264_SLP_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> H264_SLP_MASK_A {
        match self.bits {
            false => H264_SLP_MASK_A::UNMASK,
            true => H264_SLP_MASK_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == H264_SLP_MASK_A::UNMASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == H264_SLP_MASK_A::MASK
    }
}
#[doc = "Field `h264_slp_mask` writer - PD_H264 sleep mask bit"]
pub type H264_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, H264_SLP_MASK_A, 12>;
impl<'a> H264_SLP_MASK_W<'a> {
    #[doc = "Unmask PD_H264 sleep"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(H264_SLP_MASK_A::UNMASK)
    }
    #[doc = "Mask PD_H264 sleep"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(H264_SLP_MASK_A::MASK)
    }
}
#[doc = "PD_DISP sleep mask bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISP_SLP_MASK_A {
    #[doc = "0: Unmask PD_DISP sleep"]
    UNMASK = 0,
    #[doc = "1: Mask PD_DISP sleep"]
    MASK = 1,
}
impl From<DISP_SLP_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: DISP_SLP_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `disp_slp_mask` reader - PD_DISP sleep mask bit"]
pub type DISP_SLP_MASK_R = crate::BitReader<DISP_SLP_MASK_A>;
impl DISP_SLP_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISP_SLP_MASK_A {
        match self.bits {
            false => DISP_SLP_MASK_A::UNMASK,
            true => DISP_SLP_MASK_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == DISP_SLP_MASK_A::UNMASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == DISP_SLP_MASK_A::MASK
    }
}
#[doc = "Field `disp_slp_mask` writer - PD_DISP sleep mask bit"]
pub type DISP_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, DISP_SLP_MASK_A, 11>;
impl<'a> DISP_SLP_MASK_W<'a> {
    #[doc = "Unmask PD_DISP sleep"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(DISP_SLP_MASK_A::UNMASK)
    }
    #[doc = "Mask PD_DISP sleep"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(DISP_SLP_MASK_A::MASK)
    }
}
#[doc = "PD_ISP sleep mask bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISP_SLP_MASK_A {
    #[doc = "0: Unmask PD_ISP sleep"]
    UNMASK = 0,
    #[doc = "1: Mask PD_ISP sleep"]
    MASK = 1,
}
impl From<ISP_SLP_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: ISP_SLP_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `isp_slp_mask` reader - PD_ISP sleep mask bit"]
pub type ISP_SLP_MASK_R = crate::BitReader<ISP_SLP_MASK_A>;
impl ISP_SLP_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISP_SLP_MASK_A {
        match self.bits {
            false => ISP_SLP_MASK_A::UNMASK,
            true => ISP_SLP_MASK_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == ISP_SLP_MASK_A::UNMASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == ISP_SLP_MASK_A::MASK
    }
}
#[doc = "Field `isp_slp_mask` writer - PD_ISP sleep mask bit"]
pub type ISP_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, ISP_SLP_MASK_A, 10>;
impl<'a> ISP_SLP_MASK_W<'a> {
    #[doc = "Unmask PD_ISP sleep"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(ISP_SLP_MASK_A::UNMASK)
    }
    #[doc = "Mask PD_ISP sleep"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(ISP_SLP_MASK_A::MASK)
    }
}
#[doc = "PD_SRAM1 sleep mask bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM1_SLP_MASK_A {
    #[doc = "0: Unmask PD_SRAM1 sleep"]
    UNMASK = 0,
    #[doc = "1: Mask PD_SRAM1 sleep"]
    MASK = 1,
}
impl From<SRAM1_SLP_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1_SLP_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sram1_slp_mask` reader - PD_SRAM1 sleep mask bit"]
pub type SRAM1_SLP_MASK_R = crate::BitReader<SRAM1_SLP_MASK_A>;
impl SRAM1_SLP_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM1_SLP_MASK_A {
        match self.bits {
            false => SRAM1_SLP_MASK_A::UNMASK,
            true => SRAM1_SLP_MASK_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == SRAM1_SLP_MASK_A::UNMASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == SRAM1_SLP_MASK_A::MASK
    }
}
#[doc = "Field `sram1_slp_mask` writer - PD_SRAM1 sleep mask bit"]
pub type SRAM1_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, SRAM1_SLP_MASK_A, 9>;
impl<'a> SRAM1_SLP_MASK_W<'a> {
    #[doc = "Unmask PD_SRAM1 sleep"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SRAM1_SLP_MASK_A::UNMASK)
    }
    #[doc = "Mask PD_SRAM1 sleep"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SRAM1_SLP_MASK_A::MASK)
    }
}
#[doc = "PD_SRAM0 sleep mask bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM0_SLP_MASK_A {
    #[doc = "0: Unmask PD_SRAM0 sleep"]
    UNMASK = 0,
    #[doc = "1: Mask PD_SRAM0 sleep"]
    MASK = 1,
}
impl From<SRAM0_SLP_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM0_SLP_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sram0_slp_mask` reader - PD_SRAM0 sleep mask bit"]
pub type SRAM0_SLP_MASK_R = crate::BitReader<SRAM0_SLP_MASK_A>;
impl SRAM0_SLP_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM0_SLP_MASK_A {
        match self.bits {
            false => SRAM0_SLP_MASK_A::UNMASK,
            true => SRAM0_SLP_MASK_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == SRAM0_SLP_MASK_A::UNMASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == SRAM0_SLP_MASK_A::MASK
    }
}
#[doc = "Field `sram0_slp_mask` writer - PD_SRAM0 sleep mask bit"]
pub type SRAM0_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, SRAM0_SLP_MASK_A, 8>;
impl<'a> SRAM0_SLP_MASK_W<'a> {
    #[doc = "Unmask PD_SRAM0 sleep"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SRAM0_SLP_MASK_A::UNMASK)
    }
    #[doc = "Mask PD_SRAM0 sleep"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SRAM0_SLP_MASK_A::MASK)
    }
}
#[doc = "PD_MCTL sleep mask bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCTL_SLP_MASK_A {
    #[doc = "0: Unmask PD_MCTL sleep"]
    UNMASK = 0,
    #[doc = "1: Mask PD_MCTL sleep"]
    MASK = 1,
}
impl From<MCTL_SLP_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: MCTL_SLP_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `mctl_slp_mask` reader - PD_MCTL sleep mask bit"]
pub type MCTL_SLP_MASK_R = crate::BitReader<MCTL_SLP_MASK_A>;
impl MCTL_SLP_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCTL_SLP_MASK_A {
        match self.bits {
            false => MCTL_SLP_MASK_A::UNMASK,
            true => MCTL_SLP_MASK_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == MCTL_SLP_MASK_A::UNMASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == MCTL_SLP_MASK_A::MASK
    }
}
#[doc = "Field `mctl_slp_mask` writer - PD_MCTL sleep mask bit"]
pub type MCTL_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, MCTL_SLP_MASK_A, 7>;
impl<'a> MCTL_SLP_MASK_W<'a> {
    #[doc = "Unmask PD_MCTL sleep"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(MCTL_SLP_MASK_A::UNMASK)
    }
    #[doc = "Mask PD_MCTL sleep"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(MCTL_SLP_MASK_A::MASK)
    }
}
#[doc = "PD_PERI sleep mask bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERI_SLP_MASK_A {
    #[doc = "0: Unmask PD_PERI sleep"]
    UNMASK = 0,
    #[doc = "1: Mask PD_PERI sleep"]
    MASK = 1,
}
impl From<PERI_SLP_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PERI_SLP_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `peri_slp_mask` reader - PD_PERI sleep mask bit"]
pub type PERI_SLP_MASK_R = crate::BitReader<PERI_SLP_MASK_A>;
impl PERI_SLP_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERI_SLP_MASK_A {
        match self.bits {
            false => PERI_SLP_MASK_A::UNMASK,
            true => PERI_SLP_MASK_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == PERI_SLP_MASK_A::UNMASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == PERI_SLP_MASK_A::MASK
    }
}
#[doc = "Field `peri_slp_mask` writer - PD_PERI sleep mask bit"]
pub type PERI_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, PERI_SLP_MASK_A, 6>;
impl<'a> PERI_SLP_MASK_W<'a> {
    #[doc = "Unmask PD_PERI sleep"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(PERI_SLP_MASK_A::UNMASK)
    }
    #[doc = "Mask PD_PERI sleep"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(PERI_SLP_MASK_A::MASK)
    }
}
#[doc = "PD_STOR sleep mask bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOR_SLP_MASK_A {
    #[doc = "0: Unmask PD_STOR sleep"]
    UNMASK = 0,
    #[doc = "1: Mask PD_STOR sleep"]
    MASK = 1,
}
impl From<STOR_SLP_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: STOR_SLP_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `stor_slp_mask` reader - PD_STOR sleep mask bit"]
pub type STOR_SLP_MASK_R = crate::BitReader<STOR_SLP_MASK_A>;
impl STOR_SLP_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOR_SLP_MASK_A {
        match self.bits {
            false => STOR_SLP_MASK_A::UNMASK,
            true => STOR_SLP_MASK_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == STOR_SLP_MASK_A::UNMASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == STOR_SLP_MASK_A::MASK
    }
}
#[doc = "Field `stor_slp_mask` writer - PD_STOR sleep mask bit"]
pub type STOR_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, STOR_SLP_MASK_A, 5>;
impl<'a> STOR_SLP_MASK_W<'a> {
    #[doc = "Unmask PD_STOR sleep"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(STOR_SLP_MASK_A::UNMASK)
    }
    #[doc = "Mask PD_STOR sleep"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(STOR_SLP_MASK_A::MASK)
    }
}
#[doc = "PD_GNNE sleep mask bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GNNE_SLP_MASK_A {
    #[doc = "0: Unmask PD_GNNE sleep"]
    UNMASK = 0,
    #[doc = "1: Mask PD_GNNE sleep"]
    MASK = 1,
}
impl From<GNNE_SLP_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: GNNE_SLP_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `gnne_slp_mask` reader - PD_GNNE sleep mask bit"]
pub type GNNE_SLP_MASK_R = crate::BitReader<GNNE_SLP_MASK_A>;
impl GNNE_SLP_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GNNE_SLP_MASK_A {
        match self.bits {
            false => GNNE_SLP_MASK_A::UNMASK,
            true => GNNE_SLP_MASK_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == GNNE_SLP_MASK_A::UNMASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == GNNE_SLP_MASK_A::MASK
    }
}
#[doc = "Field `gnne_slp_mask` writer - PD_GNNE sleep mask bit"]
pub type GNNE_SLP_MASK_W<'a> = crate::BitWriter<'a, u32, SOC_SLEEP_MASK_SPEC, GNNE_SLP_MASK_A, 3>;
impl<'a> GNNE_SLP_MASK_W<'a> {
    #[doc = "Unmask PD_GNNE sleep"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(GNNE_SLP_MASK_A::UNMASK)
    }
    #[doc = "Mask PD_GNNE sleep"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(GNNE_SLP_MASK_A::MASK)
    }
}
impl R {
    #[doc = "Bit 13 - PD_USB sleep mask bit"]
    #[inline(always)]
    pub fn usb_slp_mask(&self) -> USB_SLP_MASK_R {
        USB_SLP_MASK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - PD_H264 sleep mask bit"]
    #[inline(always)]
    pub fn h264_slp_mask(&self) -> H264_SLP_MASK_R {
        H264_SLP_MASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - PD_DISP sleep mask bit"]
    #[inline(always)]
    pub fn disp_slp_mask(&self) -> DISP_SLP_MASK_R {
        DISP_SLP_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - PD_ISP sleep mask bit"]
    #[inline(always)]
    pub fn isp_slp_mask(&self) -> ISP_SLP_MASK_R {
        ISP_SLP_MASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - PD_SRAM1 sleep mask bit"]
    #[inline(always)]
    pub fn sram1_slp_mask(&self) -> SRAM1_SLP_MASK_R {
        SRAM1_SLP_MASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - PD_SRAM0 sleep mask bit"]
    #[inline(always)]
    pub fn sram0_slp_mask(&self) -> SRAM0_SLP_MASK_R {
        SRAM0_SLP_MASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - PD_MCTL sleep mask bit"]
    #[inline(always)]
    pub fn mctl_slp_mask(&self) -> MCTL_SLP_MASK_R {
        MCTL_SLP_MASK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - PD_PERI sleep mask bit"]
    #[inline(always)]
    pub fn peri_slp_mask(&self) -> PERI_SLP_MASK_R {
        PERI_SLP_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - PD_STOR sleep mask bit"]
    #[inline(always)]
    pub fn stor_slp_mask(&self) -> STOR_SLP_MASK_R {
        STOR_SLP_MASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 3 - PD_GNNE sleep mask bit"]
    #[inline(always)]
    pub fn gnne_slp_mask(&self) -> GNNE_SLP_MASK_R {
        GNNE_SLP_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - Write enable for bit 13 (ub_slp_mask)"]
    #[inline(always)]
    pub fn we_usb_slp_mask(&mut self) -> WE_USB_SLP_MASK_W {
        WE_USB_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 28 - Write enable for bit 12 (h264_slp_mask)"]
    #[inline(always)]
    pub fn we_h264_slp_mask(&mut self) -> WE_H264_SLP_MASK_W {
        WE_H264_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 27 - Write enable for bit 11 (disp_slp_mask)"]
    #[inline(always)]
    pub fn we_disp_slp_mask(&mut self) -> WE_DISP_SLP_MASK_W {
        WE_DISP_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 26 - Write enable for bit 10 (isp_slp_mask)"]
    #[inline(always)]
    pub fn we_isp_slp_mask(&mut self) -> WE_ISP_SLP_MASK_W {
        WE_ISP_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 25 - Write enable for bit 9 (sram1_slp_mask)"]
    #[inline(always)]
    pub fn we_sram1_slp_mask(&mut self) -> WE_SRAM1_SLP_MASK_W {
        WE_SRAM1_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 24 - Write enable for bit 8 (sram0_slp_mask)"]
    #[inline(always)]
    pub fn we_sram0_slp_mask(&mut self) -> WE_SRAM0_SLP_MASK_W {
        WE_SRAM0_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 23 - Write enable for bit 7 (mctl_slp_mask)"]
    #[inline(always)]
    pub fn we_mctl_slp_mask(&mut self) -> WE_MCTL_SLP_MASK_W {
        WE_MCTL_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 22 - Write enable for bit 6 (peri_slp_mask)"]
    #[inline(always)]
    pub fn we_peri_slp_mask(&mut self) -> WE_PERI_SLP_MASK_W {
        WE_PERI_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 21 - Write enable for bit 5 (stor_slp_mask)"]
    #[inline(always)]
    pub fn we_stor_slp_mask(&mut self) -> WE_STOR_SLP_MASK_W {
        WE_STOR_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 19 - Write enable for bit 3 (gnne_slp_mask)"]
    #[inline(always)]
    pub fn we_gnne_slp_mask(&mut self) -> WE_GNNE_SLP_MASK_W {
        WE_GNNE_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 13 - PD_USB sleep mask bit"]
    #[inline(always)]
    pub fn usb_slp_mask(&mut self) -> USB_SLP_MASK_W {
        USB_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 12 - PD_H264 sleep mask bit"]
    #[inline(always)]
    pub fn h264_slp_mask(&mut self) -> H264_SLP_MASK_W {
        H264_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 11 - PD_DISP sleep mask bit"]
    #[inline(always)]
    pub fn disp_slp_mask(&mut self) -> DISP_SLP_MASK_W {
        DISP_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 10 - PD_ISP sleep mask bit"]
    #[inline(always)]
    pub fn isp_slp_mask(&mut self) -> ISP_SLP_MASK_W {
        ISP_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 9 - PD_SRAM1 sleep mask bit"]
    #[inline(always)]
    pub fn sram1_slp_mask(&mut self) -> SRAM1_SLP_MASK_W {
        SRAM1_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 8 - PD_SRAM0 sleep mask bit"]
    #[inline(always)]
    pub fn sram0_slp_mask(&mut self) -> SRAM0_SLP_MASK_W {
        SRAM0_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 7 - PD_MCTL sleep mask bit"]
    #[inline(always)]
    pub fn mctl_slp_mask(&mut self) -> MCTL_SLP_MASK_W {
        MCTL_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 6 - PD_PERI sleep mask bit"]
    #[inline(always)]
    pub fn peri_slp_mask(&mut self) -> PERI_SLP_MASK_W {
        PERI_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 5 - PD_STOR sleep mask bit"]
    #[inline(always)]
    pub fn stor_slp_mask(&mut self) -> STOR_SLP_MASK_W {
        STOR_SLP_MASK_W::new(self)
    }
    #[doc = "Bit 3 - PD_GNNE sleep mask bit"]
    #[inline(always)]
    pub fn gnne_slp_mask(&mut self) -> GNNE_SLP_MASK_W {
        GNNE_SLP_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SoC sleep mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_sleep_mask](index.html) module"]
pub struct SOC_SLEEP_MASK_SPEC;
impl crate::RegisterSpec for SOC_SLEEP_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soc_sleep_mask::R](R) reader structure"]
impl crate::Readable for SOC_SLEEP_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soc_sleep_mask::W](W) writer structure"]
impl crate::Writable for SOC_SLEEP_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOC_SLEEP_MASK to value 0"]
impl crate::Resettable for SOC_SLEEP_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
