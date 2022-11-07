#[doc = "Register `IDENTIFIER_MEM` reader"]
pub struct R(crate::R<IDENTIFIER_MEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDENTIFIER_MEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDENTIFIER_MEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDENTIFIER_MEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDENTIFIER_MEM` writer"]
pub struct W(crate::W<IDENTIFIER_MEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDENTIFIER_MEM_SPEC>;
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
impl From<crate::W<IDENTIFIER_MEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDENTIFIER_MEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `identifier_mem` reader - "]
pub type IDENTIFIER_MEM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `identifier_mem` writer - "]
pub type IDENTIFIER_MEM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IDENTIFIER_MEM_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn identifier_mem(&self) -> IDENTIFIER_MEM_R {
        IDENTIFIER_MEM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn identifier_mem(&mut self) -> IDENTIFIER_MEM_W<0> {
        IDENTIFIER_MEM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "8 x 39-bit memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [identifier_mem](index.html) module"]
pub struct IDENTIFIER_MEM_SPEC;
impl crate::RegisterSpec for IDENTIFIER_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [identifier_mem::R](R) reader structure"]
impl crate::Readable for IDENTIFIER_MEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [identifier_mem::W](W) writer structure"]
impl crate::Writable for IDENTIFIER_MEM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDENTIFIER_MEM to value 0"]
impl crate::Resettable for IDENTIFIER_MEM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
