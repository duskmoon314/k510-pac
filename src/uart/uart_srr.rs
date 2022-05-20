#[doc = "Register `UART_SRR` writer"]
pub struct W(crate::W<UART_SRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SRR_SPEC>;
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
impl From<crate::W<UART_SRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFR` writer - XMIT FIFO Reset"]
pub type XFR_W<'a> = crate::BitWriter<'a, u32, UART_SRR_SPEC, bool, 2>;
#[doc = "Field `RFR` writer - RCVR FIFO Reset"]
pub type RFR_W<'a> = crate::BitWriter<'a, u32, UART_SRR_SPEC, bool, 1>;
#[doc = "UART Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UR_AW {
    #[doc = "1: Reset UART"]
    RESET = 1,
}
impl From<UR_AW> for bool {
    #[inline(always)]
    fn from(variant: UR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UR` writer - UART Reset"]
pub type UR_W<'a> = crate::BitWriter<'a, u32, UART_SRR_SPEC, UR_AW, 0>;
impl<'a> UR_W<'a> {
    #[doc = "Reset UART"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(UR_AW::RESET)
    }
}
impl W {
    #[doc = "Bit 2 - XMIT FIFO Reset"]
    #[inline(always)]
    pub fn xfr(&mut self) -> XFR_W {
        XFR_W::new(self)
    }
    #[doc = "Bit 1 - RCVR FIFO Reset"]
    #[inline(always)]
    pub fn rfr(&mut self) -> RFR_W {
        RFR_W::new(self)
    }
    #[doc = "Bit 0 - UART Reset"]
    #[inline(always)]
    pub fn ur(&mut self) -> UR_W {
        UR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_srr](index.html) module"]
pub struct UART_SRR_SPEC;
impl crate::RegisterSpec for UART_SRR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uart_srr::W](W) writer structure"]
impl crate::Writable for UART_SRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_SRR to value 0"]
impl crate::Resettable for UART_SRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
