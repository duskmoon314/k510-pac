#[doc = "Register `UART_TAT` reader"]
pub struct R(crate::R<UART_TAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_TAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_TAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_TAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_TAT` writer"]
pub struct W(crate::W<UART_TAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_TAT_SPEC>;
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
impl From<crate::W<UART_TAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_TAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RE_TO_DE` reader - Receiver Enable to Driver Enable Turn Around Time"]
pub type RE_TO_DE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RE_TO_DE` writer - Receiver Enable to Driver Enable Turn Around Time"]
pub type RE_TO_DE_W<'a> = crate::FieldWriter<'a, u32, UART_TAT_SPEC, u16, u16, 16, 16>;
#[doc = "Field `DE_TO_RE` reader - Driver Enable to Receiver Enable Turn Around Time"]
pub type DE_TO_RE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DE_TO_RE` writer - Driver Enable to Receiver Enable Turn Around Time"]
pub type DE_TO_RE_W<'a> = crate::FieldWriter<'a, u32, UART_TAT_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 16:31 - Receiver Enable to Driver Enable Turn Around Time"]
    #[inline(always)]
    pub fn re_to_de(&self) -> RE_TO_DE_R {
        RE_TO_DE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Driver Enable to Receiver Enable Turn Around Time"]
    #[inline(always)]
    pub fn de_to_re(&self) -> DE_TO_RE_R {
        DE_TO_RE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Receiver Enable to Driver Enable Turn Around Time"]
    #[inline(always)]
    pub fn re_to_de(&mut self) -> RE_TO_DE_W {
        RE_TO_DE_W::new(self)
    }
    #[doc = "Bits 0:15 - Driver Enable to Receiver Enable Turn Around Time"]
    #[inline(always)]
    pub fn de_to_re(&mut self) -> DE_TO_RE_W {
        DE_TO_RE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Turn Around Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_tat](index.html) module"]
pub struct UART_TAT_SPEC;
impl crate::RegisterSpec for UART_TAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_tat::R](R) reader structure"]
impl crate::Readable for UART_TAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_tat::W](W) writer structure"]
impl crate::Writable for UART_TAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_TAT to value 0"]
impl crate::Resettable for UART_TAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
