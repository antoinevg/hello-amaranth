#[doc = "Register `TXEMPTY` reader"]
pub struct R(crate::R<TXEMPTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXEMPTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXEMPTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXEMPTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXEMPTY` writer"]
pub struct W(crate::W<TXEMPTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXEMPTY_SPEC>;
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
impl From<crate::W<TXEMPTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXEMPTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `txempty` reader - "]
pub type TXEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `txempty` writer - "]
pub type TXEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXEMPTY_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<0> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX FIFO Empty.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txempty](index.html) module"]
pub struct TXEMPTY_SPEC;
impl crate::RegisterSpec for TXEMPTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txempty::R](R) reader structure"]
impl crate::Readable for TXEMPTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txempty::W](W) writer structure"]
impl crate::Writable for TXEMPTY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXEMPTY to value 0"]
impl crate::Resettable for TXEMPTY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
