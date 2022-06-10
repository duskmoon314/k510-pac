#[doc = "Register `SRAM0_LPI_CTL` reader"]
pub struct R(crate::R<SRAM0_LPI_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM0_LPI_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM0_LPI_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM0_LPI_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM0_LPI_CTL` writer"]
pub struct W(crate::W<SRAM0_LPI_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM0_LPI_CTL_SPEC>;
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
impl From<crate::W<SRAM0_LPI_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM0_LPI_CTL_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System SRAM block 0 domain NOC power controller low-power interface control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram0_lpi_ctl](index.html) module"]
pub struct SRAM0_LPI_CTL_SPEC;
impl crate::RegisterSpec for SRAM0_LPI_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram0_lpi_ctl::R](R) reader structure"]
impl crate::Readable for SRAM0_LPI_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram0_lpi_ctl::W](W) writer structure"]
impl crate::Writable for SRAM0_LPI_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAM0_LPI_CTL to value 0"]
impl crate::Resettable for SRAM0_LPI_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
