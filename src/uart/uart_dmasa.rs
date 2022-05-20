#[doc = "Register `UART_DMASA` writer"]
pub struct W(crate::W<UART_DMASA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_DMASA_SPEC>;
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
impl From<crate::W<UART_DMASA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_DMASA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA Software Acknowledge Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMASA_AW {
    #[doc = "1: DMA software acknowledge"]
    SOFT_ACK = 1,
}
impl From<DMASA_AW> for bool {
    #[inline(always)]
    fn from(variant: DMASA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASA` writer - DMA Software Acknowledge Register"]
pub type DMASA_W<'a> = crate::BitWriter<'a, u32, UART_DMASA_SPEC, DMASA_AW, 0>;
impl<'a> DMASA_W<'a> {
    #[doc = "DMA software acknowledge"]
    #[inline(always)]
    pub fn soft_ack(self) -> &'a mut W {
        self.variant(DMASA_AW::SOFT_ACK)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Software Acknowledge Register"]
    #[inline(always)]
    pub fn dmasa(&mut self) -> DMASA_W {
        DMASA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Software Acknowledge Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_dmasa](index.html) module"]
pub struct UART_DMASA_SPEC;
impl crate::RegisterSpec for UART_DMASA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uart_dmasa::W](W) writer structure"]
impl crate::Writable for UART_DMASA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_DMASA to value 0"]
impl crate::Resettable for UART_DMASA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
