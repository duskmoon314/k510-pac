#[doc = "Register `UART_RFW` writer"]
pub struct W(crate::W<UART_RFW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_RFW_SPEC>;
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
impl From<crate::W<UART_RFW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_RFW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive FIFO Framing Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFFE_AW {
    #[doc = "0: Receive FIFO framing error disabled"]
    DISABLED = 0,
    #[doc = "1: Receive FIFO framing error enabled"]
    ENABLED = 1,
}
impl From<RFFE_AW> for bool {
    #[inline(always)]
    fn from(variant: RFFE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFFE` writer - Receive FIFO Framing Error"]
pub type RFFE_W<'a> = crate::BitWriter<'a, u32, UART_RFW_SPEC, RFFE_AW, 9>;
impl<'a> RFFE_W<'a> {
    #[doc = "Receive FIFO framing error disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RFFE_AW::DISABLED)
    }
    #[doc = "Receive FIFO framing error enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RFFE_AW::ENABLED)
    }
}
#[doc = "Receive FIFO Parity Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFPE_AW {
    #[doc = "0: Receive FIFO parity error disabled"]
    DISABLED = 0,
    #[doc = "1: Receive FIFO parity error enabled"]
    ENABLED = 1,
}
impl From<RFPE_AW> for bool {
    #[inline(always)]
    fn from(variant: RFPE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFPE` writer - Receive FIFO Parity Error"]
pub type RFPE_W<'a> = crate::BitWriter<'a, u32, UART_RFW_SPEC, RFPE_AW, 8>;
impl<'a> RFPE_W<'a> {
    #[doc = "Receive FIFO parity error disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RFPE_AW::DISABLED)
    }
    #[doc = "Receive FIFO parity error enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RFPE_AW::ENABLED)
    }
}
#[doc = "Field `RFWD` writer - Receive FIFO Write Data"]
pub type RFWD_W<'a> = crate::FieldWriter<'a, u32, UART_RFW_SPEC, u8, u8, 8, 0>;
impl W {
    #[doc = "Bit 9 - Receive FIFO Framing Error"]
    #[inline(always)]
    pub fn rffe(&mut self) -> RFFE_W {
        RFFE_W::new(self)
    }
    #[doc = "Bit 8 - Receive FIFO Parity Error"]
    #[inline(always)]
    pub fn rfpe(&mut self) -> RFPE_W {
        RFPE_W::new(self)
    }
    #[doc = "Bits 0:7 - Receive FIFO Write Data"]
    #[inline(always)]
    pub fn rfwd(&mut self) -> RFWD_W {
        RFWD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO control\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rfw](index.html) module"]
pub struct UART_RFW_SPEC;
impl crate::RegisterSpec for UART_RFW_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uart_rfw::W](W) writer structure"]
impl crate::Writable for UART_RFW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_RFW to value 0"]
impl crate::Resettable for UART_RFW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
