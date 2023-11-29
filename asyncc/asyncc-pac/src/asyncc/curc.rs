#[doc = "Register `curc` reader"]
pub type R = crate::R<CURC_SPEC>;
#[doc = "Register `curc` writer"]
pub type W = crate::W<CURC_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CURC_SPEC> {
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
#[doc = "Current coroutine register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`curc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CURC_SPEC;
impl crate::RegisterSpec for CURC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curc::R`](R) reader structure"]
impl crate::Readable for CURC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`curc::W`](W) writer structure"]
impl crate::Writable for CURC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets curc to value 0"]
impl crate::Resettable for CURC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
