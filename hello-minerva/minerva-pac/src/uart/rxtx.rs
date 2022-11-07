#[doc = "Register `RXTX` reader"]
pub struct R(crate::R<RXTX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXTX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXTX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXTX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXTX` writer"]
pub struct W(crate::W<RXTX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXTX_SPEC>;
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
impl From<crate::W<RXTX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXTX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxtx` reader - "]
pub type RXTX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rxtx` writer - "]
pub type RXTX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXTX_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rxtx(&self) -> RXTX_R {
        RXTX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx(&mut self) -> RXTX_W<0> {
        RXTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtx](index.html) module"]
pub struct RXTX_SPEC;
impl crate::RegisterSpec for RXTX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxtx::R](R) reader structure"]
impl crate::Readable for RXTX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxtx::W](W) writer structure"]
impl crate::Writable for RXTX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXTX to value 0"]
impl crate::Resettable for RXTX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
