#[doc = "Register `PERI_SYS_BUS_CLK_CFG` reader"]
pub struct R(crate::R<PERI_SYS_BUS_CLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERI_SYS_BUS_CLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERI_SYS_BUS_CLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERI_SYS_BUS_CLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERI_SYS_BUS_CLK_CFG` writer"]
pub struct W(crate::W<PERI_SYS_BUS_CLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERI_SYS_BUS_CLK_CFG_SPEC>;
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
impl From<crate::W<PERI_SYS_BUS_CLK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERI_SYS_BUS_CLK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_peri_sys_hclk_div` writer - Write enable for bits \\[6:4\\]
(peri_sys_hclk_div)"]
pub type WE_PERI_SYS_HCLK_DIV_W<'a> =
    crate::BitWriter<'a, u32, PERI_SYS_BUS_CLK_CFG_SPEC, bool, 25>;
#[doc = "Field `WE_peri_sys_pclk_div` writer - Write enable for bits \\[3:0\\]
(peri_sys_pclk_div)"]
pub type WE_PERI_SYS_PCLK_DIV_W<'a> =
    crate::BitWriter<'a, u32, PERI_SYS_BUS_CLK_CFG_SPEC, bool, 24>;
#[doc = "Peripheral system AHB hclk division ratio configure\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERI_SYS_HCLK_DIV_A {
    #[doc = "1: 1/2 division"]
    D2 = 1,
    #[doc = "2: 1/3 division"]
    D3 = 2,
    #[doc = "3: 1/4 division"]
    D4 = 3,
    #[doc = "4: 1/5 division"]
    D5 = 4,
    #[doc = "5: 1/6 division"]
    D6 = 5,
    #[doc = "6: 1/7 division"]
    D7 = 6,
    #[doc = "7: 1/8 division"]
    D8 = 7,
}
impl From<PERI_SYS_HCLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PERI_SYS_HCLK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `peri_sys_hclk_div` reader - Peripheral system AHB hclk division ratio configure"]
pub type PERI_SYS_HCLK_DIV_R = crate::FieldReader<u8, PERI_SYS_HCLK_DIV_A>;
impl PERI_SYS_HCLK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PERI_SYS_HCLK_DIV_A> {
        match self.bits {
            1 => Some(PERI_SYS_HCLK_DIV_A::D2),
            2 => Some(PERI_SYS_HCLK_DIV_A::D3),
            3 => Some(PERI_SYS_HCLK_DIV_A::D4),
            4 => Some(PERI_SYS_HCLK_DIV_A::D5),
            5 => Some(PERI_SYS_HCLK_DIV_A::D6),
            6 => Some(PERI_SYS_HCLK_DIV_A::D7),
            7 => Some(PERI_SYS_HCLK_DIV_A::D8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `D2`"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == PERI_SYS_HCLK_DIV_A::D2
    }
    #[doc = "Checks if the value of the field is `D3`"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == PERI_SYS_HCLK_DIV_A::D3
    }
    #[doc = "Checks if the value of the field is `D4`"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == PERI_SYS_HCLK_DIV_A::D4
    }
    #[doc = "Checks if the value of the field is `D5`"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == PERI_SYS_HCLK_DIV_A::D5
    }
    #[doc = "Checks if the value of the field is `D6`"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == PERI_SYS_HCLK_DIV_A::D6
    }
    #[doc = "Checks if the value of the field is `D7`"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == PERI_SYS_HCLK_DIV_A::D7
    }
    #[doc = "Checks if the value of the field is `D8`"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == PERI_SYS_HCLK_DIV_A::D8
    }
}
#[doc = "Field `peri_sys_hclk_div` writer - Peripheral system AHB hclk division ratio configure"]
pub type PERI_SYS_HCLK_DIV_W<'a> =
    crate::FieldWriter<'a, u32, PERI_SYS_BUS_CLK_CFG_SPEC, u8, PERI_SYS_HCLK_DIV_A, 3, 4>;
impl<'a> PERI_SYS_HCLK_DIV_W<'a> {
    #[doc = "1/2 division"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut W {
        self.variant(PERI_SYS_HCLK_DIV_A::D2)
    }
    #[doc = "1/3 division"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut W {
        self.variant(PERI_SYS_HCLK_DIV_A::D3)
    }
    #[doc = "1/4 division"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut W {
        self.variant(PERI_SYS_HCLK_DIV_A::D4)
    }
    #[doc = "1/5 division"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut W {
        self.variant(PERI_SYS_HCLK_DIV_A::D5)
    }
    #[doc = "1/6 division"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut W {
        self.variant(PERI_SYS_HCLK_DIV_A::D6)
    }
    #[doc = "1/7 division"]
    #[inline(always)]
    pub fn d7(self) -> &'a mut W {
        self.variant(PERI_SYS_HCLK_DIV_A::D7)
    }
    #[doc = "1/8 division"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut W {
        self.variant(PERI_SYS_HCLK_DIV_A::D8)
    }
}
#[doc = "Peripheral system APB pclk division ratio configure\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERI_SYS_PCLK_DIV_A {
    #[doc = "3: 1/4 division"]
    D4 = 3,
    #[doc = "4: 1/5 division"]
    D5 = 4,
    #[doc = "5: 1/6 division"]
    D6 = 5,
    #[doc = "6: 1/7 division"]
    D7 = 6,
    #[doc = "7: 1/8 division"]
    D8 = 7,
    #[doc = "8: 1/9 division"]
    D9 = 8,
    #[doc = "9: 1/10 division"]
    D10 = 9,
    #[doc = "10: 1/11 division"]
    D11 = 10,
    #[doc = "11: 1/12 division"]
    D12 = 11,
    #[doc = "12: 1/13 division"]
    D13 = 12,
    #[doc = "13: 1/14 division"]
    D14 = 13,
    #[doc = "14: 1/15 division"]
    D15 = 14,
    #[doc = "15: 1/16 division"]
    D16 = 15,
}
impl From<PERI_SYS_PCLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PERI_SYS_PCLK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `peri_sys_pclk_div` reader - Peripheral system APB pclk division ratio configure"]
pub type PERI_SYS_PCLK_DIV_R = crate::FieldReader<u8, PERI_SYS_PCLK_DIV_A>;
impl PERI_SYS_PCLK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PERI_SYS_PCLK_DIV_A> {
        match self.bits {
            3 => Some(PERI_SYS_PCLK_DIV_A::D4),
            4 => Some(PERI_SYS_PCLK_DIV_A::D5),
            5 => Some(PERI_SYS_PCLK_DIV_A::D6),
            6 => Some(PERI_SYS_PCLK_DIV_A::D7),
            7 => Some(PERI_SYS_PCLK_DIV_A::D8),
            8 => Some(PERI_SYS_PCLK_DIV_A::D9),
            9 => Some(PERI_SYS_PCLK_DIV_A::D10),
            10 => Some(PERI_SYS_PCLK_DIV_A::D11),
            11 => Some(PERI_SYS_PCLK_DIV_A::D12),
            12 => Some(PERI_SYS_PCLK_DIV_A::D13),
            13 => Some(PERI_SYS_PCLK_DIV_A::D14),
            14 => Some(PERI_SYS_PCLK_DIV_A::D15),
            15 => Some(PERI_SYS_PCLK_DIV_A::D16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `D4`"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == PERI_SYS_PCLK_DIV_A::D4
    }
    #[doc = "Checks if the value of the field is `D5`"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == PERI_SYS_PCLK_DIV_A::D5
    }
    #[doc = "Checks if the value of the field is `D6`"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == PERI_SYS_PCLK_DIV_A::D6
    }
    #[doc = "Checks if the value of the field is `D7`"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == PERI_SYS_PCLK_DIV_A::D7
    }
    #[doc = "Checks if the value of the field is `D8`"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == PERI_SYS_PCLK_DIV_A::D8
    }
    #[doc = "Checks if the value of the field is `D9`"]
    #[inline(always)]
    pub fn is_d9(&self) -> bool {
        *self == PERI_SYS_PCLK_DIV_A::D9
    }
    #[doc = "Checks if the value of the field is `D10`"]
    #[inline(always)]
    pub fn is_d10(&self) -> bool {
        *self == PERI_SYS_PCLK_DIV_A::D10
    }
    #[doc = "Checks if the value of the field is `D11`"]
    #[inline(always)]
    pub fn is_d11(&self) -> bool {
        *self == PERI_SYS_PCLK_DIV_A::D11
    }
    #[doc = "Checks if the value of the field is `D12`"]
    #[inline(always)]
    pub fn is_d12(&self) -> bool {
        *self == PERI_SYS_PCLK_DIV_A::D12
    }
    #[doc = "Checks if the value of the field is `D13`"]
    #[inline(always)]
    pub fn is_d13(&self) -> bool {
        *self == PERI_SYS_PCLK_DIV_A::D13
    }
    #[doc = "Checks if the value of the field is `D14`"]
    #[inline(always)]
    pub fn is_d14(&self) -> bool {
        *self == PERI_SYS_PCLK_DIV_A::D14
    }
    #[doc = "Checks if the value of the field is `D15`"]
    #[inline(always)]
    pub fn is_d15(&self) -> bool {
        *self == PERI_SYS_PCLK_DIV_A::D15
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == PERI_SYS_PCLK_DIV_A::D16
    }
}
#[doc = "Field `peri_sys_pclk_div` writer - Peripheral system APB pclk division ratio configure"]
pub type PERI_SYS_PCLK_DIV_W<'a> =
    crate::FieldWriter<'a, u32, PERI_SYS_BUS_CLK_CFG_SPEC, u8, PERI_SYS_PCLK_DIV_A, 4, 0>;
impl<'a> PERI_SYS_PCLK_DIV_W<'a> {
    #[doc = "1/4 division"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut W {
        self.variant(PERI_SYS_PCLK_DIV_A::D4)
    }
    #[doc = "1/5 division"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut W {
        self.variant(PERI_SYS_PCLK_DIV_A::D5)
    }
    #[doc = "1/6 division"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut W {
        self.variant(PERI_SYS_PCLK_DIV_A::D6)
    }
    #[doc = "1/7 division"]
    #[inline(always)]
    pub fn d7(self) -> &'a mut W {
        self.variant(PERI_SYS_PCLK_DIV_A::D7)
    }
    #[doc = "1/8 division"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut W {
        self.variant(PERI_SYS_PCLK_DIV_A::D8)
    }
    #[doc = "1/9 division"]
    #[inline(always)]
    pub fn d9(self) -> &'a mut W {
        self.variant(PERI_SYS_PCLK_DIV_A::D9)
    }
    #[doc = "1/10 division"]
    #[inline(always)]
    pub fn d10(self) -> &'a mut W {
        self.variant(PERI_SYS_PCLK_DIV_A::D10)
    }
    #[doc = "1/11 division"]
    #[inline(always)]
    pub fn d11(self) -> &'a mut W {
        self.variant(PERI_SYS_PCLK_DIV_A::D11)
    }
    #[doc = "1/12 division"]
    #[inline(always)]
    pub fn d12(self) -> &'a mut W {
        self.variant(PERI_SYS_PCLK_DIV_A::D12)
    }
    #[doc = "1/13 division"]
    #[inline(always)]
    pub fn d13(self) -> &'a mut W {
        self.variant(PERI_SYS_PCLK_DIV_A::D13)
    }
    #[doc = "1/14 division"]
    #[inline(always)]
    pub fn d14(self) -> &'a mut W {
        self.variant(PERI_SYS_PCLK_DIV_A::D14)
    }
    #[doc = "1/15 division"]
    #[inline(always)]
    pub fn d15(self) -> &'a mut W {
        self.variant(PERI_SYS_PCLK_DIV_A::D15)
    }
    #[doc = "1/16 division"]
    #[inline(always)]
    pub fn d16(self) -> &'a mut W {
        self.variant(PERI_SYS_PCLK_DIV_A::D16)
    }
}
impl R {
    #[doc = "Bits 4:6 - Peripheral system AHB hclk division ratio configure"]
    #[inline(always)]
    pub fn peri_sys_hclk_div(&self) -> PERI_SYS_HCLK_DIV_R {
        PERI_SYS_HCLK_DIV_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 0:3 - Peripheral system APB pclk division ratio configure"]
    #[inline(always)]
    pub fn peri_sys_pclk_div(&self) -> PERI_SYS_PCLK_DIV_R {
        PERI_SYS_PCLK_DIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 25 - Write enable for bits \\[6:4\\]
(peri_sys_hclk_div)"]
    #[inline(always)]
    pub fn we_peri_sys_hclk_div(&mut self) -> WE_PERI_SYS_HCLK_DIV_W {
        WE_PERI_SYS_HCLK_DIV_W::new(self)
    }
    #[doc = "Bit 24 - Write enable for bits \\[3:0\\]
(peri_sys_pclk_div)"]
    #[inline(always)]
    pub fn we_peri_sys_pclk_div(&mut self) -> WE_PERI_SYS_PCLK_DIV_W {
        WE_PERI_SYS_PCLK_DIV_W::new(self)
    }
    #[doc = "Bits 4:6 - Peripheral system AHB hclk division ratio configure"]
    #[inline(always)]
    pub fn peri_sys_hclk_div(&mut self) -> PERI_SYS_HCLK_DIV_W {
        PERI_SYS_HCLK_DIV_W::new(self)
    }
    #[doc = "Bits 0:3 - Peripheral system APB pclk division ratio configure"]
    #[inline(always)]
    pub fn peri_sys_pclk_div(&mut self) -> PERI_SYS_PCLK_DIV_W {
        PERI_SYS_PCLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral subsystem modules bus IF clock configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_sys_bus_clk_cfg](index.html) module"]
pub struct PERI_SYS_BUS_CLK_CFG_SPEC;
impl crate::RegisterSpec for PERI_SYS_BUS_CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peri_sys_bus_clk_cfg::R](R) reader structure"]
impl crate::Readable for PERI_SYS_BUS_CLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peri_sys_bus_clk_cfg::W](W) writer structure"]
impl crate::Writable for PERI_SYS_BUS_CLK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERI_SYS_BUS_CLK_CFG to value 0x37"]
impl crate::Resettable for PERI_SYS_BUS_CLK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x37
    }
}
