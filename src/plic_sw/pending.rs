#[doc = "Register `PENDING%s` reader"]
pub struct R(crate::R<PENDING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PENDING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PENDING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PENDING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PENDING%s` writer"]
pub struct W(crate::W<PENDING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PENDING_SPEC>;
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
impl From<crate::W<PENDING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PENDING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Source\\[i\\]
interrupt pending status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOURCE_A {
    #[doc = "0: Source\\[i\\]
interrupt is not pending"]
    NOT_PENDING = 0,
    #[doc = "1: Source\\[i\\]
interrupt is pending"]
    PENDING = 1,
}
impl From<SOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: SOURCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `SOURCE(0-31)` reader - Source\\[i\\]
interrupt pending status"]
pub type SOURCE_R = crate::BitReader<SOURCE_A>;
impl SOURCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOURCE_A {
        match self.bits {
            false => SOURCE_A::NOT_PENDING,
            true => SOURCE_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == SOURCE_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SOURCE_A::PENDING
    }
}
#[doc = "Fields `SOURCE(0-31)` writer - Source\\[i\\]
interrupt pending status"]
pub type SOURCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PENDING_SPEC, SOURCE_A, O>;
impl<'a, const O: u8> SOURCE_W<'a, O> {
    #[doc = "Source\\[i\\]
interrupt is not pending"]
    #[inline(always)]
    pub fn not_pending(self) -> &'a mut W {
        self.variant(SOURCE_A::NOT_PENDING)
    }
    #[doc = "Source\\[i\\]
interrupt is pending"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(SOURCE_A::PENDING)
    }
}
impl R {
    #[doc = "Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub unsafe fn source(&self, n: u8) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source0(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source1(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source2(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source3(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source4(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source5(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source6(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source7(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source8(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source9(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source10(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source11(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source12(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source13(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source14(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source15(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source16(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source17(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source18(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source19(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source20(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source21(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source22(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source23(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source24(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source25(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source26(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source27(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source28(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source29(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source30(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source31(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub unsafe fn source<const O: u8>(&mut self) -> SOURCE_W<O> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 0 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source0(&mut self) -> SOURCE_W<0> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 1 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source1(&mut self) -> SOURCE_W<1> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 2 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source2(&mut self) -> SOURCE_W<2> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 3 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source3(&mut self) -> SOURCE_W<3> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 4 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source4(&mut self) -> SOURCE_W<4> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 5 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source5(&mut self) -> SOURCE_W<5> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 6 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source6(&mut self) -> SOURCE_W<6> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 7 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source7(&mut self) -> SOURCE_W<7> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 8 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source8(&mut self) -> SOURCE_W<8> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 9 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source9(&mut self) -> SOURCE_W<9> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 10 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source10(&mut self) -> SOURCE_W<10> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 11 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source11(&mut self) -> SOURCE_W<11> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 12 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source12(&mut self) -> SOURCE_W<12> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 13 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source13(&mut self) -> SOURCE_W<13> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 14 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source14(&mut self) -> SOURCE_W<14> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 15 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source15(&mut self) -> SOURCE_W<15> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 16 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source16(&mut self) -> SOURCE_W<16> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 17 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source17(&mut self) -> SOURCE_W<17> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 18 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source18(&mut self) -> SOURCE_W<18> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 19 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source19(&mut self) -> SOURCE_W<19> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 20 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source20(&mut self) -> SOURCE_W<20> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 21 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source21(&mut self) -> SOURCE_W<21> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 22 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source22(&mut self) -> SOURCE_W<22> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 23 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source23(&mut self) -> SOURCE_W<23> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 24 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source24(&mut self) -> SOURCE_W<24> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 25 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source25(&mut self) -> SOURCE_W<25> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 26 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source26(&mut self) -> SOURCE_W<26> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 27 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source27(&mut self) -> SOURCE_W<27> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 28 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source28(&mut self) -> SOURCE_W<28> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 29 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source29(&mut self) -> SOURCE_W<29> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 30 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source30(&mut self) -> SOURCE_W<30> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 31 - Source\\[i\\]
interrupt pending status"]
    #[inline(always)]
    pub fn source31(&mut self) -> SOURCE_W<31> {
        SOURCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Souce\\[i:i+31\\]
interrupt pending\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pending](index.html) module"]
pub struct PENDING_SPEC;
impl crate::RegisterSpec for PENDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pending::R](R) reader structure"]
impl crate::Readable for PENDING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pending::W](W) writer structure"]
impl crate::Writable for PENDING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PENDING%s to value 0"]
impl crate::Resettable for PENDING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
