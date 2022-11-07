#[doc = "Register `EV_ENABLE` reader"]
pub struct R(crate::R<EV_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EV_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EV_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EV_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EV_ENABLE` writer"]
pub struct W(crate::W<EV_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EV_ENABLE_SPEC>;
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
impl From<crate::W<EV_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EV_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `zero` reader - Write a ``1`` to enable the ``zero`` Event"]
pub type ZERO_R = crate::BitReader<bool>;
#[doc = "Field `zero` writer - Write a ``1`` to enable the ``zero`` Event"]
pub type ZERO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EV_ENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``zero`` Event"]
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``zero`` Event"]
    #[inline(always)]
    #[must_use]
    pub fn zero(&mut self) -> ZERO_W<0> {
        ZERO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register enables the corresponding zero events. Write a ``0`` to this register to disable individual events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ev_enable](index.html) module"]
pub struct EV_ENABLE_SPEC;
impl crate::RegisterSpec for EV_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ev_enable::R](R) reader structure"]
impl crate::Readable for EV_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ev_enable::W](W) writer structure"]
impl crate::Writable for EV_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EV_ENABLE to value 0"]
impl crate::Resettable for EV_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
