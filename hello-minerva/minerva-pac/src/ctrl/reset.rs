#[doc = "Register `RESET` reader"]
pub struct R(crate::R<RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET` writer"]
pub struct W(crate::W<RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_SPEC>;
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
impl From<crate::W<RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `soc_rst` reader - Write `1` to this register to reset the full SoC (Pulse Reset)"]
pub type SOC_RST_R = crate::BitReader<bool>;
#[doc = "Field `soc_rst` writer - Write `1` to this register to reset the full SoC (Pulse Reset)"]
pub type SOC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `cpu_rst` reader - Write `1` to this register to reset the CPU(s) of the SoC (Hold Reset)"]
pub type CPU_RST_R = crate::BitReader<bool>;
#[doc = "Field `cpu_rst` writer - Write `1` to this register to reset the CPU(s) of the SoC (Hold Reset)"]
pub type CPU_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write `1` to this register to reset the full SoC (Pulse Reset)"]
    #[inline(always)]
    pub fn soc_rst(&self) -> SOC_RST_R {
        SOC_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write `1` to this register to reset the CPU(s) of the SoC (Hold Reset)"]
    #[inline(always)]
    pub fn cpu_rst(&self) -> CPU_RST_R {
        CPU_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write `1` to this register to reset the full SoC (Pulse Reset)"]
    #[inline(always)]
    #[must_use]
    pub fn soc_rst(&mut self) -> SOC_RST_W<0> {
        SOC_RST_W::new(self)
    }
    #[doc = "Bit 1 - Write `1` to this register to reset the CPU(s) of the SoC (Hold Reset)"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_rst(&mut self) -> CPU_RST_W<1> {
        CPU_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset](index.html) module"]
pub struct RESET_SPEC;
impl crate::RegisterSpec for RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset::R](R) reader structure"]
impl crate::Readable for RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset::W](W) writer structure"]
impl crate::Writable for RESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESET to value 0"]
impl crate::Resettable for RESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
