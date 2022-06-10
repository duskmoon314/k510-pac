#[doc = "Register `NOC_CLK_CFG` reader"]
pub struct R(crate::R<NOC_CLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NOC_CLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NOC_CLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NOC_CLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NOC_CLK_CFG` writer"]
pub struct W(crate::W<NOC_CLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NOC_CLK_CFG_SPEC>;
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
impl From<crate::W<NOC_CLK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NOC_CLK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WE_noc_clk1_div` writer - Write enable for bits \\[10:8\\]
(noc_clk1_div)"]
pub type WE_NOC_CLK1_DIV_W<'a> = crate::BitWriter<'a, u32, NOC_CLK_CFG_SPEC, bool, 26>;
#[doc = "Field `WE_noc_clk0_div` writer - Write enable for bits \\[6:4\\]
(noc_clk0_div)"]
pub type WE_NOC_CLK0_DIV_W<'a> = crate::BitWriter<'a, u32, NOC_CLK_CFG_SPEC, bool, 25>;
#[doc = "NOC domain\\[i\\]
bus clock division ratio configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NOC_CLK_DIV_A {
    #[doc = "0: 6/6 division"]
    D6 = 0,
    #[doc = "1: 5/6 division"]
    D5 = 1,
    #[doc = "2: 4/6 division"]
    D4 = 2,
    #[doc = "3: 3/6 division"]
    D3 = 3,
    #[doc = "4: 2/6 division"]
    D2 = 4,
    #[doc = "5: 1/6 division"]
    D1 = 5,
}
impl From<NOC_CLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: NOC_CLK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Fields `noc_clk(0-1)_div` reader - NOC domain\\[i\\]
bus clock division ratio configure"]
pub type NOC_CLK_DIV_R = crate::FieldReader<u8, NOC_CLK_DIV_A>;
impl NOC_CLK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NOC_CLK_DIV_A> {
        match self.bits {
            0 => Some(NOC_CLK_DIV_A::D6),
            1 => Some(NOC_CLK_DIV_A::D5),
            2 => Some(NOC_CLK_DIV_A::D4),
            3 => Some(NOC_CLK_DIV_A::D3),
            4 => Some(NOC_CLK_DIV_A::D2),
            5 => Some(NOC_CLK_DIV_A::D1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `D6`"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == NOC_CLK_DIV_A::D6
    }
    #[doc = "Checks if the value of the field is `D5`"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == NOC_CLK_DIV_A::D5
    }
    #[doc = "Checks if the value of the field is `D4`"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == NOC_CLK_DIV_A::D4
    }
    #[doc = "Checks if the value of the field is `D3`"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == NOC_CLK_DIV_A::D3
    }
    #[doc = "Checks if the value of the field is `D2`"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == NOC_CLK_DIV_A::D2
    }
    #[doc = "Checks if the value of the field is `D1`"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == NOC_CLK_DIV_A::D1
    }
}
#[doc = "Fields `noc_clk(0-1)_div` writer - NOC domain\\[i\\]
bus clock division ratio configure"]
pub type NOC_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NOC_CLK_CFG_SPEC, u8, NOC_CLK_DIV_A, 3, O>;
impl<'a, const O: u8> NOC_CLK_DIV_W<'a, O> {
    #[doc = "6/6 division"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut W {
        self.variant(NOC_CLK_DIV_A::D6)
    }
    #[doc = "5/6 division"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut W {
        self.variant(NOC_CLK_DIV_A::D5)
    }
    #[doc = "4/6 division"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut W {
        self.variant(NOC_CLK_DIV_A::D4)
    }
    #[doc = "3/6 division"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut W {
        self.variant(NOC_CLK_DIV_A::D3)
    }
    #[doc = "2/6 division"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut W {
        self.variant(NOC_CLK_DIV_A::D2)
    }
    #[doc = "1/6 division"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut W {
        self.variant(NOC_CLK_DIV_A::D1)
    }
}
impl R {
    #[doc = "NOC domain\\[i\\]
bus clock division ratio configure"]
    #[inline(always)]
    pub unsafe fn noc_clk_div(&self, n: u8) -> NOC_CLK_DIV_R {
        NOC_CLK_DIV_R::new(((self.bits >> (n * 4 + 4)) & 7) as u8)
    }
    #[doc = "Bits 4:6 - NOC domain\\[i\\]
bus clock division ratio configure"]
    #[inline(always)]
    pub fn noc_clk0_div(&self) -> NOC_CLK_DIV_R {
        NOC_CLK_DIV_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - NOC domain\\[i\\]
bus clock division ratio configure"]
    #[inline(always)]
    pub fn noc_clk1_div(&self) -> NOC_CLK_DIV_R {
        NOC_CLK_DIV_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 26 - Write enable for bits \\[10:8\\]
(noc_clk1_div)"]
    #[inline(always)]
    pub fn we_noc_clk1_div(&mut self) -> WE_NOC_CLK1_DIV_W {
        WE_NOC_CLK1_DIV_W::new(self)
    }
    #[doc = "Bit 25 - Write enable for bits \\[6:4\\]
(noc_clk0_div)"]
    #[inline(always)]
    pub fn we_noc_clk0_div(&mut self) -> WE_NOC_CLK0_DIV_W {
        WE_NOC_CLK0_DIV_W::new(self)
    }
    #[doc = "NOC domain\\[i\\]
bus clock division ratio configure"]
    #[inline(always)]
    pub unsafe fn noc_clk_div<const O: u8>(&mut self) -> NOC_CLK_DIV_W<O> {
        NOC_CLK_DIV_W::new(self)
    }
    #[doc = "Bits 4:6 - NOC domain\\[i\\]
bus clock division ratio configure"]
    #[inline(always)]
    pub fn noc_clk0_div(&mut self) -> NOC_CLK_DIV_W<4> {
        NOC_CLK_DIV_W::new(self)
    }
    #[doc = "Bits 8:10 - NOC domain\\[i\\]
bus clock division ratio configure"]
    #[inline(always)]
    pub fn noc_clk1_div(&mut self) -> NOC_CLK_DIV_W<8> {
        NOC_CLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NOC bus clock Division and configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [noc_clk_cfg](index.html) module"]
pub struct NOC_CLK_CFG_SPEC;
impl crate::RegisterSpec for NOC_CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [noc_clk_cfg::R](R) reader structure"]
impl crate::Readable for NOC_CLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [noc_clk_cfg::W](W) writer structure"]
impl crate::Writable for NOC_CLK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NOC_CLK_CFG to value 0"]
impl crate::Resettable for NOC_CLK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
