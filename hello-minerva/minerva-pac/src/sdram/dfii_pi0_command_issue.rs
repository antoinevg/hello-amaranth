#[doc = "Register `DFII_PI0_COMMAND_ISSUE` reader"]
pub struct R(crate::R<DFII_PI0_COMMAND_ISSUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFII_PI0_COMMAND_ISSUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFII_PI0_COMMAND_ISSUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFII_PI0_COMMAND_ISSUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFII_PI0_COMMAND_ISSUE` writer"]
pub struct W(crate::W<DFII_PI0_COMMAND_ISSUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFII_PI0_COMMAND_ISSUE_SPEC>;
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
impl From<crate::W<DFII_PI0_COMMAND_ISSUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFII_PI0_COMMAND_ISSUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dfii_pi0_command_issue` reader - "]
pub type DFII_PI0_COMMAND_ISSUE_R = crate::BitReader<bool>;
#[doc = "Field `dfii_pi0_command_issue` writer - "]
pub type DFII_PI0_COMMAND_ISSUE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DFII_PI0_COMMAND_ISSUE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dfii_pi0_command_issue(&self) -> DFII_PI0_COMMAND_ISSUE_R {
        DFII_PI0_COMMAND_ISSUE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dfii_pi0_command_issue(&mut self) -> DFII_PI0_COMMAND_ISSUE_W<0> {
        DFII_PI0_COMMAND_ISSUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfii_pi0_command_issue](index.html) module"]
pub struct DFII_PI0_COMMAND_ISSUE_SPEC;
impl crate::RegisterSpec for DFII_PI0_COMMAND_ISSUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfii_pi0_command_issue::R](R) reader structure"]
impl crate::Readable for DFII_PI0_COMMAND_ISSUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfii_pi0_command_issue::W](W) writer structure"]
impl crate::Writable for DFII_PI0_COMMAND_ISSUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFII_PI0_COMMAND_ISSUE to value 0"]
impl crate::Resettable for DFII_PI0_COMMAND_ISSUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
