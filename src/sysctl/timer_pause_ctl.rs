#[doc = "Register `TIMER_PAUSE_CTL` reader"]
pub struct R(crate::R<TIMER_PAUSE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_PAUSE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_PAUSE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_PAUSE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_PAUSE_CTL` writer"]
pub struct W(crate::W<TIMER_PAUSE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_PAUSE_CTL_SPEC>;
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
impl From<crate::W<TIMER_PAUSE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_PAUSE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fields `WE_timer_t(0-5)_pause` writer - Write enable for bit (timer_t\\[i\\]_pause)"]
pub type WE_TIMER_T_PAUSE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TIMER_PAUSE_CTL_SPEC, bool, O>;
#[doc = "SoC Timer module internal timer \\[i\\]
pause control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_T_PAUSE_A {
    #[doc = "0: Resume timer \\[i\\]
counting function"]
    RESUME = 0,
    #[doc = "1: Pause timer \\[i\\]
counting function"]
    ENABLE = 1,
}
impl From<TIMER_T_PAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER_T_PAUSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `timer_t(0-5)_pause` reader - SoC Timer module internal timer \\[i\\]
pause control bit"]
pub type TIMER_T_PAUSE_R = crate::BitReader<TIMER_T_PAUSE_A>;
impl TIMER_T_PAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_T_PAUSE_A {
        match self.bits {
            false => TIMER_T_PAUSE_A::RESUME,
            true => TIMER_T_PAUSE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == TIMER_T_PAUSE_A::RESUME
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIMER_T_PAUSE_A::ENABLE
    }
}
#[doc = "Fields `timer_t(0-5)_pause` writer - SoC Timer module internal timer \\[i\\]
pause control bit"]
pub type TIMER_T_PAUSE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TIMER_PAUSE_CTL_SPEC, TIMER_T_PAUSE_A, O>;
impl<'a, const O: u8> TIMER_T_PAUSE_W<'a, O> {
    #[doc = "Resume timer \\[i\\]
counting function"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(TIMER_T_PAUSE_A::RESUME)
    }
    #[doc = "Pause timer \\[i\\]
counting function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER_T_PAUSE_A::ENABLE)
    }
}
impl R {
    #[doc = "SoC Timer module internal timer \\[i\\]
pause control bit"]
    #[inline(always)]
    pub unsafe fn timer_t_pause(&self, n: u8) -> TIMER_T_PAUSE_R {
        TIMER_T_PAUSE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - SoC Timer module internal timer \\[i\\]
pause control bit"]
    #[inline(always)]
    pub fn timer_t0_pause(&self) -> TIMER_T_PAUSE_R {
        TIMER_T_PAUSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SoC Timer module internal timer \\[i\\]
pause control bit"]
    #[inline(always)]
    pub fn timer_t1_pause(&self) -> TIMER_T_PAUSE_R {
        TIMER_T_PAUSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SoC Timer module internal timer \\[i\\]
pause control bit"]
    #[inline(always)]
    pub fn timer_t2_pause(&self) -> TIMER_T_PAUSE_R {
        TIMER_T_PAUSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SoC Timer module internal timer \\[i\\]
pause control bit"]
    #[inline(always)]
    pub fn timer_t3_pause(&self) -> TIMER_T_PAUSE_R {
        TIMER_T_PAUSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SoC Timer module internal timer \\[i\\]
pause control bit"]
    #[inline(always)]
    pub fn timer_t4_pause(&self) -> TIMER_T_PAUSE_R {
        TIMER_T_PAUSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SoC Timer module internal timer \\[i\\]
pause control bit"]
    #[inline(always)]
    pub fn timer_t5_pause(&self) -> TIMER_T_PAUSE_R {
        TIMER_T_PAUSE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Write enable for bit (timer_t\\[i\\]_pause)"]
    #[inline(always)]
    pub unsafe fn we_timer_t_pause<const O: u8>(&mut self) -> WE_TIMER_T_PAUSE_W<O> {
        WE_TIMER_T_PAUSE_W::new(self)
    }
    #[doc = "Bit 16 - Write enable for bit (timer_t\\[i\\]_pause)"]
    #[inline(always)]
    pub fn we_timer_t0_pause(&mut self) -> WE_TIMER_T_PAUSE_W<16> {
        WE_TIMER_T_PAUSE_W::new(self)
    }
    #[doc = "Bit 17 - Write enable for bit (timer_t\\[i\\]_pause)"]
    #[inline(always)]
    pub fn we_timer_t1_pause(&mut self) -> WE_TIMER_T_PAUSE_W<17> {
        WE_TIMER_T_PAUSE_W::new(self)
    }
    #[doc = "Bit 18 - Write enable for bit (timer_t\\[i\\]_pause)"]
    #[inline(always)]
    pub fn we_timer_t2_pause(&mut self) -> WE_TIMER_T_PAUSE_W<18> {
        WE_TIMER_T_PAUSE_W::new(self)
    }
    #[doc = "Bit 19 - Write enable for bit (timer_t\\[i\\]_pause)"]
    #[inline(always)]
    pub fn we_timer_t3_pause(&mut self) -> WE_TIMER_T_PAUSE_W<19> {
        WE_TIMER_T_PAUSE_W::new(self)
    }
    #[doc = "Bit 20 - Write enable for bit (timer_t\\[i\\]_pause)"]
    #[inline(always)]
    pub fn we_timer_t4_pause(&mut self) -> WE_TIMER_T_PAUSE_W<20> {
        WE_TIMER_T_PAUSE_W::new(self)
    }
    #[doc = "Bit 21 - Write enable for bit (timer_t\\[i\\]_pause)"]
    #[inline(always)]
    pub fn we_timer_t5_pause(&mut self) -> WE_TIMER_T_PAUSE_W<21> {
        WE_TIMER_T_PAUSE_W::new(self)
    }
    #[doc = "SoC Timer module internal timer \\[i\\]
pause control bit"]
    #[inline(always)]
    pub unsafe fn timer_t_pause<const O: u8>(&mut self) -> TIMER_T_PAUSE_W<O> {
        TIMER_T_PAUSE_W::new(self)
    }
    #[doc = "Bit 0 - SoC Timer module internal timer \\[i\\]
pause control bit"]
    #[inline(always)]
    pub fn timer_t0_pause(&mut self) -> TIMER_T_PAUSE_W<0> {
        TIMER_T_PAUSE_W::new(self)
    }
    #[doc = "Bit 1 - SoC Timer module internal timer \\[i\\]
pause control bit"]
    #[inline(always)]
    pub fn timer_t1_pause(&mut self) -> TIMER_T_PAUSE_W<1> {
        TIMER_T_PAUSE_W::new(self)
    }
    #[doc = "Bit 2 - SoC Timer module internal timer \\[i\\]
pause control bit"]
    #[inline(always)]
    pub fn timer_t2_pause(&mut self) -> TIMER_T_PAUSE_W<2> {
        TIMER_T_PAUSE_W::new(self)
    }
    #[doc = "Bit 3 - SoC Timer module internal timer \\[i\\]
pause control bit"]
    #[inline(always)]
    pub fn timer_t3_pause(&mut self) -> TIMER_T_PAUSE_W<3> {
        TIMER_T_PAUSE_W::new(self)
    }
    #[doc = "Bit 4 - SoC Timer module internal timer \\[i\\]
pause control bit"]
    #[inline(always)]
    pub fn timer_t4_pause(&mut self) -> TIMER_T_PAUSE_W<4> {
        TIMER_T_PAUSE_W::new(self)
    }
    #[doc = "Bit 5 - SoC Timer module internal timer \\[i\\]
pause control bit"]
    #[inline(always)]
    pub fn timer_t5_pause(&mut self) -> TIMER_T_PAUSE_W<5> {
        TIMER_T_PAUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SoC internal Timer module's timer pause control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_pause_ctl](index.html) module"]
pub struct TIMER_PAUSE_CTL_SPEC;
impl crate::RegisterSpec for TIMER_PAUSE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_pause_ctl::R](R) reader structure"]
impl crate::Readable for TIMER_PAUSE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_pause_ctl::W](W) writer structure"]
impl crate::Writable for TIMER_PAUSE_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER_PAUSE_CTL to value 0"]
impl crate::Resettable for TIMER_PAUSE_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
