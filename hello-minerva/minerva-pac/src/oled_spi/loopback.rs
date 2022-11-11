#[doc = "Register `LOOPBACK` reader"]
pub struct R(crate::R<LOOPBACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOOPBACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOOPBACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOOPBACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOOPBACK` writer"]
pub struct W(crate::W<LOOPBACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOOPBACK_SPEC>;
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
impl From<crate::W<LOOPBACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOOPBACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mode` reader - None"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `mode` writer - None"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOOPBACK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - None"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - None"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Loopback Mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loopback](index.html) module"]
pub struct LOOPBACK_SPEC;
impl crate::RegisterSpec for LOOPBACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [loopback::R](R) reader structure"]
impl crate::Readable for LOOPBACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [loopback::W](W) writer structure"]
impl crate::Writable for LOOPBACK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOOPBACK to value 0"]
impl crate::Resettable for LOOPBACK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
