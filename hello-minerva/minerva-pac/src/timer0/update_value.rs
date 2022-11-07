#[doc = "Register `UPDATE_VALUE` reader"]
pub struct R(crate::R<UPDATE_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPDATE_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPDATE_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPDATE_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPDATE_VALUE` writer"]
pub struct W(crate::W<UPDATE_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPDATE_VALUE_SPEC>;
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
impl From<crate::W<UPDATE_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPDATE_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `update_value` reader - "]
pub type UPDATE_VALUE_R = crate::BitReader<bool>;
#[doc = "Field `update_value` writer - "]
pub type UPDATE_VALUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPDATE_VALUE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn update_value(&self) -> UPDATE_VALUE_R {
        UPDATE_VALUE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn update_value(&mut self) -> UPDATE_VALUE_W<0> {
        UPDATE_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Update trigger for the current countdown value. A write to this register latches the current countdown value to ``value`` register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [update_value](index.html) module"]
pub struct UPDATE_VALUE_SPEC;
impl crate::RegisterSpec for UPDATE_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [update_value::R](R) reader structure"]
impl crate::Readable for UPDATE_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [update_value::W](W) writer structure"]
impl crate::Writable for UPDATE_VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPDATE_VALUE to value 0"]
impl crate::Resettable for UPDATE_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
