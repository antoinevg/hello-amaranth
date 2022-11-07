#[doc = "Register `RXFULL` reader"]
pub struct R(crate::R<RXFULL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFULL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFULL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFULL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXFULL` writer"]
pub struct W(crate::W<RXFULL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXFULL_SPEC>;
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
impl From<crate::W<RXFULL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXFULL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxfull` reader - "]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `rxfull` writer - "]
pub type RXFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXFULL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<0> {
        RXFULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX FIFO Full.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfull](index.html) module"]
pub struct RXFULL_SPEC;
impl crate::RegisterSpec for RXFULL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfull::R](R) reader structure"]
impl crate::Readable for RXFULL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxfull::W](W) writer structure"]
impl crate::Writable for RXFULL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXFULL to value 0"]
impl crate::Resettable for RXFULL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
