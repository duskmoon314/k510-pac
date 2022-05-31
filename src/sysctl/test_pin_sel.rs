#[doc = "Register `TEST_PIN_SEL` reader"]
pub struct R(crate::R<TEST_PIN_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_PIN_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_PIN_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_PIN_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST_PIN_SEL` writer"]
pub struct W(crate::W<TEST_PIN_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_PIN_SEL_SPEC>;
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
impl From<crate::W<TEST_PIN_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_PIN_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "test pin group sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TEST_PIN_GROUP_SEL_A {
    #[doc = "0: group 0"]
    G0 = 0,
    #[doc = "1: group 1"]
    G1 = 1,
    #[doc = "2: group 2"]
    G2 = 2,
    #[doc = "3: group 3"]
    G3 = 3,
    #[doc = "4: group 4"]
    G4 = 4,
    #[doc = "5: group 5"]
    G5 = 5,
    #[doc = "6: group 6"]
    G6 = 6,
}
impl From<TEST_PIN_GROUP_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TEST_PIN_GROUP_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TEST_PIN_GROUP_SEL` reader - test pin group sel"]
pub type TEST_PIN_GROUP_SEL_R = crate::FieldReader<u8, TEST_PIN_GROUP_SEL_A>;
impl TEST_PIN_GROUP_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TEST_PIN_GROUP_SEL_A> {
        match self.bits {
            0 => Some(TEST_PIN_GROUP_SEL_A::G0),
            1 => Some(TEST_PIN_GROUP_SEL_A::G1),
            2 => Some(TEST_PIN_GROUP_SEL_A::G2),
            3 => Some(TEST_PIN_GROUP_SEL_A::G3),
            4 => Some(TEST_PIN_GROUP_SEL_A::G4),
            5 => Some(TEST_PIN_GROUP_SEL_A::G5),
            6 => Some(TEST_PIN_GROUP_SEL_A::G6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `G0`"]
    #[inline(always)]
    pub fn is_g0(&self) -> bool {
        *self == TEST_PIN_GROUP_SEL_A::G0
    }
    #[doc = "Checks if the value of the field is `G1`"]
    #[inline(always)]
    pub fn is_g1(&self) -> bool {
        *self == TEST_PIN_GROUP_SEL_A::G1
    }
    #[doc = "Checks if the value of the field is `G2`"]
    #[inline(always)]
    pub fn is_g2(&self) -> bool {
        *self == TEST_PIN_GROUP_SEL_A::G2
    }
    #[doc = "Checks if the value of the field is `G3`"]
    #[inline(always)]
    pub fn is_g3(&self) -> bool {
        *self == TEST_PIN_GROUP_SEL_A::G3
    }
    #[doc = "Checks if the value of the field is `G4`"]
    #[inline(always)]
    pub fn is_g4(&self) -> bool {
        *self == TEST_PIN_GROUP_SEL_A::G4
    }
    #[doc = "Checks if the value of the field is `G5`"]
    #[inline(always)]
    pub fn is_g5(&self) -> bool {
        *self == TEST_PIN_GROUP_SEL_A::G5
    }
    #[doc = "Checks if the value of the field is `G6`"]
    #[inline(always)]
    pub fn is_g6(&self) -> bool {
        *self == TEST_PIN_GROUP_SEL_A::G6
    }
}
#[doc = "Field `TEST_PIN_GROUP_SEL` writer - test pin group sel"]
pub type TEST_PIN_GROUP_SEL_W<'a> =
    crate::FieldWriter<'a, u32, TEST_PIN_SEL_SPEC, u8, TEST_PIN_GROUP_SEL_A, 3, 0>;
impl<'a> TEST_PIN_GROUP_SEL_W<'a> {
    #[doc = "group 0"]
    #[inline(always)]
    pub fn g0(self) -> &'a mut W {
        self.variant(TEST_PIN_GROUP_SEL_A::G0)
    }
    #[doc = "group 1"]
    #[inline(always)]
    pub fn g1(self) -> &'a mut W {
        self.variant(TEST_PIN_GROUP_SEL_A::G1)
    }
    #[doc = "group 2"]
    #[inline(always)]
    pub fn g2(self) -> &'a mut W {
        self.variant(TEST_PIN_GROUP_SEL_A::G2)
    }
    #[doc = "group 3"]
    #[inline(always)]
    pub fn g3(self) -> &'a mut W {
        self.variant(TEST_PIN_GROUP_SEL_A::G3)
    }
    #[doc = "group 4"]
    #[inline(always)]
    pub fn g4(self) -> &'a mut W {
        self.variant(TEST_PIN_GROUP_SEL_A::G4)
    }
    #[doc = "group 5"]
    #[inline(always)]
    pub fn g5(self) -> &'a mut W {
        self.variant(TEST_PIN_GROUP_SEL_A::G5)
    }
    #[doc = "group 6"]
    #[inline(always)]
    pub fn g6(self) -> &'a mut W {
        self.variant(TEST_PIN_GROUP_SEL_A::G6)
    }
}
impl R {
    #[doc = "Bits 0:2 - test pin group sel"]
    #[inline(always)]
    pub fn test_pin_group_sel(&self) -> TEST_PIN_GROUP_SEL_R {
        TEST_PIN_GROUP_SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - test pin group sel"]
    #[inline(always)]
    pub fn test_pin_group_sel(&mut self) -> TEST_PIN_GROUP_SEL_W {
        TEST_PIN_GROUP_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test pin group select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_pin_sel](index.html) module"]
pub struct TEST_PIN_SEL_SPEC;
impl crate::RegisterSpec for TEST_PIN_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test_pin_sel::R](R) reader structure"]
impl crate::Readable for TEST_PIN_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test_pin_sel::W](W) writer structure"]
impl crate::Writable for TEST_PIN_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEST_PIN_SEL to value 0"]
impl crate::Resettable for TEST_PIN_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
