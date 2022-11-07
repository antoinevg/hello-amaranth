#[doc = "Register `DFII_PI0_COMMAND` reader"]
pub struct R(crate::R<DFII_PI0_COMMAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFII_PI0_COMMAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFII_PI0_COMMAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFII_PI0_COMMAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFII_PI0_COMMAND` writer"]
pub struct W(crate::W<DFII_PI0_COMMAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFII_PI0_COMMAND_SPEC>;
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
impl From<crate::W<DFII_PI0_COMMAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFII_PI0_COMMAND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cs` reader - DFI chip select bus"]
pub type CS_R = crate::BitReader<bool>;
#[doc = "Field `cs` writer - DFI chip select bus"]
pub type CS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFII_PI0_COMMAND_SPEC, bool, O>;
#[doc = "Field `we` reader - DFI write enable bus"]
pub type WE_R = crate::BitReader<bool>;
#[doc = "Field `we` writer - DFI write enable bus"]
pub type WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFII_PI0_COMMAND_SPEC, bool, O>;
#[doc = "Field `cas` reader - DFI column address strobe bus"]
pub type CAS_R = crate::BitReader<bool>;
#[doc = "Field `cas` writer - DFI column address strobe bus"]
pub type CAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFII_PI0_COMMAND_SPEC, bool, O>;
#[doc = "Field `ras` reader - DFI row address strobe bus"]
pub type RAS_R = crate::BitReader<bool>;
#[doc = "Field `ras` writer - DFI row address strobe bus"]
pub type RAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFII_PI0_COMMAND_SPEC, bool, O>;
#[doc = "Field `wren` reader - DFI write data enable bus"]
pub type WREN_R = crate::BitReader<bool>;
#[doc = "Field `wren` writer - DFI write data enable bus"]
pub type WREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFII_PI0_COMMAND_SPEC, bool, O>;
#[doc = "Field `rden` reader - DFI read data enable bus"]
pub type RDEN_R = crate::BitReader<bool>;
#[doc = "Field `rden` writer - DFI read data enable bus"]
pub type RDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFII_PI0_COMMAND_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DFI chip select bus"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DFI write enable bus"]
    #[inline(always)]
    pub fn we(&self) -> WE_R {
        WE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DFI column address strobe bus"]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DFI row address strobe bus"]
    #[inline(always)]
    pub fn ras(&self) -> RAS_R {
        RAS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DFI write data enable bus"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DFI read data enable bus"]
    #[inline(always)]
    pub fn rden(&self) -> RDEN_R {
        RDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFI chip select bus"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<0> {
        CS_W::new(self)
    }
    #[doc = "Bit 1 - DFI write enable bus"]
    #[inline(always)]
    #[must_use]
    pub fn we(&mut self) -> WE_W<1> {
        WE_W::new(self)
    }
    #[doc = "Bit 2 - DFI column address strobe bus"]
    #[inline(always)]
    #[must_use]
    pub fn cas(&mut self) -> CAS_W<2> {
        CAS_W::new(self)
    }
    #[doc = "Bit 3 - DFI row address strobe bus"]
    #[inline(always)]
    #[must_use]
    pub fn ras(&mut self) -> RAS_W<3> {
        RAS_W::new(self)
    }
    #[doc = "Bit 4 - DFI write data enable bus"]
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WREN_W<4> {
        WREN_W::new(self)
    }
    #[doc = "Bit 5 - DFI read data enable bus"]
    #[inline(always)]
    #[must_use]
    pub fn rden(&mut self) -> RDEN_W<5> {
        RDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control DFI signals on a single phase\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfii_pi0_command](index.html) module"]
pub struct DFII_PI0_COMMAND_SPEC;
impl crate::RegisterSpec for DFII_PI0_COMMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfii_pi0_command::R](R) reader structure"]
impl crate::Readable for DFII_PI0_COMMAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfii_pi0_command::W](W) writer structure"]
impl crate::Writable for DFII_PI0_COMMAND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFII_PI0_COMMAND to value 0"]
impl crate::Resettable for DFII_PI0_COMMAND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
