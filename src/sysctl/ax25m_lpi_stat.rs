#[doc = "Register `AX25M_LPI_STAT` reader"]
pub struct R(crate::R<AX25M_LPI_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AX25M_LPI_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AX25M_LPI_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AX25M_LPI_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AX25M_LPI_STAT` writer"]
pub struct W(crate::W<AX25M_LPI_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AX25M_LPI_STAT_SPEC>;
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
impl From<crate::W<AX25M_LPI_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AX25M_LPI_STAT_SPEC>) -> Self {
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
#[doc = "AX25M NOC power controller low-power interface status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ax25m_lpi_stat](index.html) module"]
pub struct AX25M_LPI_STAT_SPEC;
impl crate::RegisterSpec for AX25M_LPI_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ax25m_lpi_stat::R](R) reader structure"]
impl crate::Readable for AX25M_LPI_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ax25m_lpi_stat::W](W) writer structure"]
impl crate::Writable for AX25M_LPI_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AX25M_LPI_STAT to value 0"]
impl crate::Resettable for AX25M_LPI_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
