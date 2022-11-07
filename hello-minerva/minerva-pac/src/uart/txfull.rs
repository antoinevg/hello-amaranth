#[doc = "Register `TXFULL` reader"]
pub struct R(crate::R<TXFULL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFULL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFULL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFULL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXFULL` writer"]
pub struct W(crate::W<TXFULL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFULL_SPEC>;
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
impl From<crate::W<TXFULL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFULL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `txfull` reader - "]
pub type TXFULL_R = crate::BitReader<bool>;
#[doc = "Field `txfull` writer - "]
pub type TXFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXFULL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn txfull(&mut self) -> TXFULL_W<0> {
        TXFULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX FIFO Full.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfull](index.html) module"]
pub struct TXFULL_SPEC;
impl crate::RegisterSpec for TXFULL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfull::R](R) reader structure"]
impl crate::Readable for TXFULL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txfull::W](W) writer structure"]
impl crate::Writable for TXFULL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXFULL to value 0"]
impl crate::Resettable for TXFULL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
