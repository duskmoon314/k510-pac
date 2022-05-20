#[doc = "Register `UART_FCR` writer"]
pub struct W(crate::W<UART_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_FCR_SPEC>;
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
impl From<crate::W<UART_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RCVR Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT_AW {
    #[doc = "0: 1 character in FIFO"]
    FIFO_CHAR_1 = 0,
    #[doc = "1: FIFO 1/4 full"]
    FIFO_QUARTER_FULL = 1,
    #[doc = "2: FIFO 1/2 full"]
    FIFO_HALF_FULL = 2,
    #[doc = "3: FIFO 2 less than full"]
    FIFO_FULL_2 = 3,
}
impl From<RT_AW> for u8 {
    #[inline(always)]
    fn from(variant: RT_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `RT` writer - RCVR Trigger"]
pub type RT_W<'a> = crate::FieldWriterSafe<'a, u32, UART_FCR_SPEC, u8, RT_AW, 2, 6>;
impl<'a> RT_W<'a> {
    #[doc = "1 character in FIFO"]
    #[inline(always)]
    pub fn fifo_char_1(self) -> &'a mut W {
        self.variant(RT_AW::FIFO_CHAR_1)
    }
    #[doc = "FIFO 1/4 full"]
    #[inline(always)]
    pub fn fifo_quarter_full(self) -> &'a mut W {
        self.variant(RT_AW::FIFO_QUARTER_FULL)
    }
    #[doc = "FIFO 1/2 full"]
    #[inline(always)]
    pub fn fifo_half_full(self) -> &'a mut W {
        self.variant(RT_AW::FIFO_HALF_FULL)
    }
    #[doc = "FIFO 2 less than full"]
    #[inline(always)]
    pub fn fifo_full_2(self) -> &'a mut W {
        self.variant(RT_AW::FIFO_FULL_2)
    }
}
#[doc = "TX Empty Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TET_AW {
    #[doc = "0: FIFO empty"]
    FIFO_EMPTY = 0,
    #[doc = "1: 2 characters in FIFO"]
    FIFO_CHAR_2 = 1,
    #[doc = "2: FIFO 1/4 full"]
    FIFO_QUARTER_FULL = 2,
    #[doc = "3: FIFO 1/2 full"]
    FIFO_HALF_FULL = 3,
}
impl From<TET_AW> for u8 {
    #[inline(always)]
    fn from(variant: TET_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `TET` writer - TX Empty Trigger"]
pub type TET_W<'a> = crate::FieldWriterSafe<'a, u32, UART_FCR_SPEC, u8, TET_AW, 2, 4>;
impl<'a> TET_W<'a> {
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn fifo_empty(self) -> &'a mut W {
        self.variant(TET_AW::FIFO_EMPTY)
    }
    #[doc = "2 characters in FIFO"]
    #[inline(always)]
    pub fn fifo_char_2(self) -> &'a mut W {
        self.variant(TET_AW::FIFO_CHAR_2)
    }
    #[doc = "FIFO 1/4 full"]
    #[inline(always)]
    pub fn fifo_quarter_full(self) -> &'a mut W {
        self.variant(TET_AW::FIFO_QUARTER_FULL)
    }
    #[doc = "FIFO 1/2 full"]
    #[inline(always)]
    pub fn fifo_half_full(self) -> &'a mut W {
        self.variant(TET_AW::FIFO_HALF_FULL)
    }
}
#[doc = "DMA Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAM_AW {
    #[doc = "0: Mode 0"]
    MODE0 = 0,
    #[doc = "1: Mode 1"]
    MODE1 = 1,
}
impl From<DMAM_AW> for bool {
    #[inline(always)]
    fn from(variant: DMAM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAM` writer - DMA Mode"]
pub type DMAM_W<'a> = crate::BitWriter<'a, u32, UART_FCR_SPEC, DMAM_AW, 3>;
impl<'a> DMAM_W<'a> {
    #[doc = "Mode 0"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(DMAM_AW::MODE0)
    }
    #[doc = "Mode 1"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(DMAM_AW::MODE1)
    }
}
#[doc = "XMIT FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XFIFOR_AW {
    #[doc = "1: reset"]
    RESET = 1,
}
impl From<XFIFOR_AW> for bool {
    #[inline(always)]
    fn from(variant: XFIFOR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XFIFOR` writer - XMIT FIFO Reset"]
pub type XFIFOR_W<'a> = crate::BitWriter0C<'a, u32, UART_FCR_SPEC, XFIFOR_AW, 2>;
impl<'a> XFIFOR_W<'a> {
    #[doc = "reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(XFIFOR_AW::RESET)
    }
}
#[doc = "RCVR FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFIFOR_AW {
    #[doc = "1: reset"]
    RESET = 1,
}
impl From<RFIFOR_AW> for bool {
    #[inline(always)]
    fn from(variant: RFIFOR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFIFOR` writer - RCVR FIFO Reset"]
pub type RFIFOR_W<'a> = crate::BitWriter0C<'a, u32, UART_FCR_SPEC, RFIFOR_AW, 1>;
impl<'a> RFIFOR_W<'a> {
    #[doc = "reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RFIFOR_AW::RESET)
    }
}
#[doc = "FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOE_AW {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<FIFOE_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOE` writer - FIFO Enable"]
pub type FIFOE_W<'a> = crate::BitWriter<'a, u32, UART_FCR_SPEC, FIFOE_AW, 0>;
impl<'a> FIFOE_W<'a> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FIFOE_AW::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FIFOE_AW::ENABLE)
    }
}
impl W {
    #[doc = "Bits 6:7 - RCVR Trigger"]
    #[inline(always)]
    pub fn rt(&mut self) -> RT_W {
        RT_W::new(self)
    }
    #[doc = "Bits 4:5 - TX Empty Trigger"]
    #[inline(always)]
    pub fn tet(&mut self) -> TET_W {
        TET_W::new(self)
    }
    #[doc = "Bit 3 - DMA Mode"]
    #[inline(always)]
    pub fn dmam(&mut self) -> DMAM_W {
        DMAM_W::new(self)
    }
    #[doc = "Bit 2 - XMIT FIFO Reset"]
    #[inline(always)]
    pub fn xfifor(&mut self) -> XFIFOR_W {
        XFIFOR_W::new(self)
    }
    #[doc = "Bit 1 - RCVR FIFO Reset"]
    #[inline(always)]
    pub fn rfifor(&mut self) -> RFIFOR_W {
        RFIFOR_W::new(self)
    }
    #[doc = "Bit 0 - FIFO Enable"]
    #[inline(always)]
    pub fn fifoe(&mut self) -> FIFOE_W {
        FIFOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fcr](index.html) module"]
pub struct UART_FCR_SPEC;
impl crate::RegisterSpec for UART_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uart_fcr::W](W) writer structure"]
impl crate::Writable for UART_FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_FCR to value 0"]
impl crate::Resettable for UART_FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
