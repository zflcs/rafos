#[doc = "Register `eptr` reader"]
pub type R = crate::R<EPTR_SPEC>;
#[doc = "Register `eptr` writer"]
pub type W = crate::W<EPTR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<EPTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Executor Base Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPTR_SPEC;
impl crate::RegisterSpec for EPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eptr::R`](R) reader structure"]
impl crate::Readable for EPTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eptr::W`](W) writer structure"]
impl crate::Writable for EPTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets eptr to value 0"]
impl crate::Resettable for EPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
