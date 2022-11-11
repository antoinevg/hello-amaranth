#[doc = "Register `CLK_DIVIDER` reader"]
pub struct R(crate::R<CLK_DIVIDER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_DIVIDER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_DIVIDER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_DIVIDER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_DIVIDER` writer"]
pub struct W(crate::W<CLK_DIVIDER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_DIVIDER_SPEC>;
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
impl From<crate::W<CLK_DIVIDER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_DIVIDER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clk_divider` reader - "]
pub type CLK_DIVIDER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `clk_divider` writer - "]
pub type CLK_DIVIDER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_DIVIDER_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn clk_divider(&self) -> CLK_DIVIDER_R {
        CLK_DIVIDER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divider(&mut self) -> CLK_DIVIDER_W<0> {
        CLK_DIVIDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Clk Divider.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_divider](index.html) module"]
pub struct CLK_DIVIDER_SPEC;
impl crate::RegisterSpec for CLK_DIVIDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_divider::R](R) reader structure"]
impl crate::Readable for CLK_DIVIDER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_divider::W](W) writer structure"]
impl crate::Writable for CLK_DIVIDER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_DIVIDER to value 0x07"]
impl crate::Resettable for CLK_DIVIDER_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
