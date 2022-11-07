#[doc = "Register `RXEMPTY` reader"]
pub struct R(crate::R<RXEMPTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXEMPTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXEMPTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXEMPTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXEMPTY` writer"]
pub struct W(crate::W<RXEMPTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXEMPTY_SPEC>;
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
impl From<crate::W<RXEMPTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXEMPTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxempty` reader - "]
pub type RXEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `rxempty` writer - "]
pub type RXEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXEMPTY_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rxempty(&mut self) -> RXEMPTY_W<0> {
        RXEMPTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX FIFO Empty.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxempty](index.html) module"]
pub struct RXEMPTY_SPEC;
impl crate::RegisterSpec for RXEMPTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxempty::R](R) reader structure"]
impl crate::Readable for RXEMPTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxempty::W](W) writer structure"]
impl crate::Writable for RXEMPTY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXEMPTY to value 0"]
impl crate::Resettable for RXEMPTY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
