#[doc = "Register `MISO` reader"]
pub struct R(crate::R<MISO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISO` writer"]
pub struct W(crate::W<MISO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISO_SPEC>;
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
impl From<crate::W<MISO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `miso` reader - "]
pub type MISO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `miso` writer - "]
pub type MISO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MISO_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn miso(&self) -> MISO_R {
        MISO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn miso(&mut self) -> MISO_W<0> {
        MISO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI MISO data (MSB-first de-serialization).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miso](index.html) module"]
pub struct MISO_SPEC;
impl crate::RegisterSpec for MISO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miso::R](R) reader structure"]
impl crate::Readable for MISO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miso::W](W) writer structure"]
impl crate::Writable for MISO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISO to value 0"]
impl crate::Resettable for MISO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
