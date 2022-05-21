#[doc = "Register `SOC_BOOT_CTL` reader"]
pub struct R(crate::R<SOC_BOOT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOC_BOOT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOC_BOOT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOC_BOOT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "SoC core otp_bypass pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTP_BYPASS_A {
    #[doc = "0: Disable OTP bypass"]
    DISABLE = 0,
    #[doc = "1: Enable OTP bypass"]
    ENABLE = 1,
}
impl From<OTP_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: OTP_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `otp_bypass` reader - SoC core otp_bypass pin"]
pub type OTP_BYPASS_R = crate::BitReader<OTP_BYPASS_A>;
impl OTP_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTP_BYPASS_A {
        match self.bits {
            false => OTP_BYPASS_A::DISABLE,
            true => OTP_BYPASS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OTP_BYPASS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OTP_BYPASS_A::ENABLE
    }
}
#[doc = "SoC core boot_ctl pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SOC_BOOT_CTL_A {
    #[doc = "0: Boot mode 0: from UART"]
    MODE0 = 0,
    #[doc = "1: Boot mode 1: from SD"]
    MODE1 = 1,
    #[doc = "2: Boot mode 2: from SPI NAND"]
    MODE2 = 2,
    #[doc = "3: Boot mode 3: from eMMC"]
    MODE3 = 3,
}
impl From<SOC_BOOT_CTL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOC_BOOT_CTL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `soc_boot_ctl` reader - SoC core boot_ctl pin"]
pub type SOC_BOOT_CTL_R = crate::FieldReader<u8, SOC_BOOT_CTL_A>;
impl SOC_BOOT_CTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_BOOT_CTL_A {
        match self.bits {
            0 => SOC_BOOT_CTL_A::MODE0,
            1 => SOC_BOOT_CTL_A::MODE1,
            2 => SOC_BOOT_CTL_A::MODE2,
            3 => SOC_BOOT_CTL_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == SOC_BOOT_CTL_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == SOC_BOOT_CTL_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == SOC_BOOT_CTL_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == SOC_BOOT_CTL_A::MODE3
    }
}
impl R {
    #[doc = "Bit 2 - SoC core otp_bypass pin"]
    #[inline(always)]
    pub fn otp_bypass(&self) -> OTP_BYPASS_R {
        OTP_BYPASS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - SoC core boot_ctl pin"]
    #[inline(always)]
    pub fn soc_boot_ctl(&self) -> SOC_BOOT_CTL_R {
        SOC_BOOT_CTL_R::new((self.bits & 3) as u8)
    }
}
#[doc = "SoC boot control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_boot_ctl](index.html) module"]
pub struct SOC_BOOT_CTL_SPEC;
impl crate::RegisterSpec for SOC_BOOT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soc_boot_ctl::R](R) reader structure"]
impl crate::Readable for SOC_BOOT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SOC_BOOT_CTL to value 0"]
impl crate::Resettable for SOC_BOOT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
