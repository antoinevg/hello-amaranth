#[doc = "Register `BUS_ERRORS` reader"]
pub struct R(crate::R<BUS_ERRORS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_ERRORS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_ERRORS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_ERRORS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUS_ERRORS` writer"]
pub struct W(crate::W<BUS_ERRORS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUS_ERRORS_SPEC>;
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
impl From<crate::W<BUS_ERRORS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUS_ERRORS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bus_errors` reader - "]
pub type BUS_ERRORS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `bus_errors` writer - "]
pub type BUS_ERRORS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BUS_ERRORS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bus_errors(&self) -> BUS_ERRORS_R {
        BUS_ERRORS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn bus_errors(&mut self) -> BUS_ERRORS_W<0> {
        BUS_ERRORS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Total number of Wishbone bus errors (timeouts) since start.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_errors](index.html) module"]
pub struct BUS_ERRORS_SPEC;
impl crate::RegisterSpec for BUS_ERRORS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_errors::R](R) reader structure"]
impl crate::Readable for BUS_ERRORS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bus_errors::W](W) writer structure"]
impl crate::Writable for BUS_ERRORS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUS_ERRORS to value 0"]
impl crate::Resettable for BUS_ERRORS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
