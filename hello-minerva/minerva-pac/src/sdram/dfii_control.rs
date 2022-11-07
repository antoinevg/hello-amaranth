#[doc = "Register `DFII_CONTROL` reader"]
pub struct R(crate::R<DFII_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFII_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFII_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFII_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFII_CONTROL` writer"]
pub struct W(crate::W<DFII_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFII_CONTROL_SPEC>;
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
impl From<crate::W<DFII_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFII_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sel` reader - None"]
pub type SEL_R = crate::BitReader<bool>;
#[doc = "Field `sel` writer - None"]
pub type SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFII_CONTROL_SPEC, bool, O>;
#[doc = "Field `cke` reader - DFI clock enable bus"]
pub type CKE_R = crate::BitReader<bool>;
#[doc = "Field `cke` writer - DFI clock enable bus"]
pub type CKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFII_CONTROL_SPEC, bool, O>;
#[doc = "Field `odt` reader - DFI on-die termination bus"]
pub type ODT_R = crate::BitReader<bool>;
#[doc = "Field `odt` writer - DFI on-die termination bus"]
pub type ODT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFII_CONTROL_SPEC, bool, O>;
#[doc = "Field `reset_n` reader - DFI clock reset bus"]
pub type RESET_N_R = crate::BitReader<bool>;
#[doc = "Field `reset_n` writer - DFI clock reset bus"]
pub type RESET_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFII_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - None"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DFI clock enable bus"]
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DFI on-die termination bus"]
    #[inline(always)]
    pub fn odt(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DFI clock reset bus"]
    #[inline(always)]
    pub fn reset_n(&self) -> RESET_N_R {
        RESET_N_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - None"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Bit 1 - DFI clock enable bus"]
    #[inline(always)]
    #[must_use]
    pub fn cke(&mut self) -> CKE_W<1> {
        CKE_W::new(self)
    }
    #[doc = "Bit 2 - DFI on-die termination bus"]
    #[inline(always)]
    #[must_use]
    pub fn odt(&mut self) -> ODT_W<2> {
        ODT_W::new(self)
    }
    #[doc = "Bit 3 - DFI clock reset bus"]
    #[inline(always)]
    #[must_use]
    pub fn reset_n(&mut self) -> RESET_N_W<3> {
        RESET_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control DFI signals common to all phases\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfii_control](index.html) module"]
pub struct DFII_CONTROL_SPEC;
impl crate::RegisterSpec for DFII_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfii_control::R](R) reader structure"]
impl crate::Readable for DFII_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfii_control::W](W) writer structure"]
impl crate::Writable for DFII_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFII_CONTROL to value 0x01"]
impl crate::Resettable for DFII_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
