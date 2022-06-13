#[doc = "Register `PREEMPTED_PRIORITY` reader"]
pub struct R(crate::R<PREEMPTED_PRIORITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREEMPTED_PRIORITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREEMPTED_PRIORITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREEMPTED_PRIORITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREEMPTED_PRIORITY` writer"]
pub struct W(crate::W<PREEMPTED_PRIORITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREEMPTED_PRIORITY_SPEC>;
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
impl From<crate::W<PREEMPTED_PRIORITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREEMPTED_PRIORITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Target\\[M\\]'s preempted priority\\[P\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREEMPTED_PRIORITY_A {
    #[doc = "0: This priority no preempted by a higher priority interrupt"]
    NO_PREEMPTED = 0,
    #[doc = "1: This priority has been preempted by a higher priority interrupt"]
    PREEMPTED = 1,
}
impl From<PREEMPTED_PRIORITY_A> for bool {
    #[inline(always)]
    fn from(variant: PREEMPTED_PRIORITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `PREEMPTED_PRIORITY(0-15)` reader - Target\\[M\\]'s preempted priority\\[P\\]"]
pub type PREEMPTED_PRIORITY_R = crate::BitReader<PREEMPTED_PRIORITY_A>;
impl PREEMPTED_PRIORITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREEMPTED_PRIORITY_A {
        match self.bits {
            false => PREEMPTED_PRIORITY_A::NO_PREEMPTED,
            true => PREEMPTED_PRIORITY_A::PREEMPTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PREEMPTED`"]
    #[inline(always)]
    pub fn is_no_preempted(&self) -> bool {
        *self == PREEMPTED_PRIORITY_A::NO_PREEMPTED
    }
    #[doc = "Checks if the value of the field is `PREEMPTED`"]
    #[inline(always)]
    pub fn is_preempted(&self) -> bool {
        *self == PREEMPTED_PRIORITY_A::PREEMPTED
    }
}
#[doc = "Fields `PREEMPTED_PRIORITY(0-15)` writer - Target\\[M\\]'s preempted priority\\[P\\]"]
pub type PREEMPTED_PRIORITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PREEMPTED_PRIORITY_SPEC, PREEMPTED_PRIORITY_A, O>;
impl<'a, const O: u8> PREEMPTED_PRIORITY_W<'a, O> {
    #[doc = "This priority no preempted by a higher priority interrupt"]
    #[inline(always)]
    pub fn no_preempted(self) -> &'a mut W {
        self.variant(PREEMPTED_PRIORITY_A::NO_PREEMPTED)
    }
    #[doc = "This priority has been preempted by a higher priority interrupt"]
    #[inline(always)]
    pub fn preempted(self) -> &'a mut W {
        self.variant(PREEMPTED_PRIORITY_A::PREEMPTED)
    }
}
impl R {
    #[doc = "Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub unsafe fn preempted_priority(&self, n: u8) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority0(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority1(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority2(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority3(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority4(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority5(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority6(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority7(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority8(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority9(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority10(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority11(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority12(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority13(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority14(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority15(&self) -> PREEMPTED_PRIORITY_R {
        PREEMPTED_PRIORITY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub unsafe fn preempted_priority<const O: u8>(&mut self) -> PREEMPTED_PRIORITY_W<O> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 0 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority0(&mut self) -> PREEMPTED_PRIORITY_W<0> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 1 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority1(&mut self) -> PREEMPTED_PRIORITY_W<1> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 2 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority2(&mut self) -> PREEMPTED_PRIORITY_W<2> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 3 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority3(&mut self) -> PREEMPTED_PRIORITY_W<3> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 4 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority4(&mut self) -> PREEMPTED_PRIORITY_W<4> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 5 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority5(&mut self) -> PREEMPTED_PRIORITY_W<5> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 6 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority6(&mut self) -> PREEMPTED_PRIORITY_W<6> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 7 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority7(&mut self) -> PREEMPTED_PRIORITY_W<7> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 8 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority8(&mut self) -> PREEMPTED_PRIORITY_W<8> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 9 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority9(&mut self) -> PREEMPTED_PRIORITY_W<9> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 10 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority10(&mut self) -> PREEMPTED_PRIORITY_W<10> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 11 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority11(&mut self) -> PREEMPTED_PRIORITY_W<11> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 12 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority12(&mut self) -> PREEMPTED_PRIORITY_W<12> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 13 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority13(&mut self) -> PREEMPTED_PRIORITY_W<13> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 14 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority14(&mut self) -> PREEMPTED_PRIORITY_W<14> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Bit 15 - Target\\[M\\]'s preempted priority\\[P\\]"]
    #[inline(always)]
    pub fn preempted_priority15(&mut self) -> PREEMPTED_PRIORITY_W<15> {
        PREEMPTED_PRIORITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Target\\[M\\]'s preempted priority register\n\nprovide the preempted priority stack information for each target\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preempted_priority](index.html) module"]
pub struct PREEMPTED_PRIORITY_SPEC;
impl crate::RegisterSpec for PREEMPTED_PRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [preempted_priority::R](R) reader structure"]
impl crate::Readable for PREEMPTED_PRIORITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [preempted_priority::W](W) writer structure"]
impl crate::Writable for PREEMPTED_PRIORITY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PREEMPTED_PRIORITY to value 0"]
impl crate::Resettable for PREEMPTED_PRIORITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
