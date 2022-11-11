#[doc = "Register `MOSI` reader"]
pub struct R(crate::R<MOSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOSI` writer"]
pub struct W(crate::W<MOSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOSI_SPEC>;
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
impl From<crate::W<MOSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mosi` reader - "]
pub type MOSI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mosi` writer - "]
pub type MOSI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MOSI_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mosi(&self) -> MOSI_R {
        MOSI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn mosi(&mut self) -> MOSI_W<0> {
        MOSI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI MOSI data (MSB-first serialization).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mosi](index.html) module"]
pub struct MOSI_SPEC;
impl crate::RegisterSpec for MOSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mosi::R](R) reader structure"]
impl crate::Readable for MOSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mosi::W](W) writer structure"]
impl crate::Writable for MOSI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOSI to value 0"]
impl crate::Resettable for MOSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
