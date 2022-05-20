#[doc = "Register `UART_SFE` reader"]
pub struct R(crate::R<UART_SFE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SFE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SFE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SFE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_SFE` writer"]
pub struct W(crate::W<UART_SFE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SFE_SPEC>;
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
impl From<crate::W<UART_SFE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SFE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shadow FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<SFE_A> for bool {
    #[inline(always)]
    fn from(variant: SFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFE` reader - Shadow FIFO Enable"]
pub type SFE_R = crate::BitReader<SFE_A>;
impl SFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFE_A {
        match self.bits {
            false => SFE_A::DISABLE,
            true => SFE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SFE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SFE_A::ENABLE
    }
}
#[doc = "Field `SFE` writer - Shadow FIFO Enable"]
pub type SFE_W<'a> = crate::BitWriter<'a, u32, UART_SFE_SPEC, SFE_A, 0>;
impl<'a> SFE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SFE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SFE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Shadow FIFO Enable"]
    #[inline(always)]
    pub fn sfe(&self) -> SFE_R {
        SFE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow FIFO Enable"]
    #[inline(always)]
    pub fn sfe(&mut self) -> SFE_W {
        SFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow FIFO Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_sfe](index.html) module"]
pub struct UART_SFE_SPEC;
impl crate::RegisterSpec for UART_SFE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_sfe::R](R) reader structure"]
impl crate::Readable for UART_SFE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_sfe::W](W) writer structure"]
impl crate::Writable for UART_SFE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_SFE to value 0"]
impl crate::Resettable for UART_SFE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
