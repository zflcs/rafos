#[doc = "Register `a1_msb` reader"]
pub type R = crate::R<A1_MSB_SPEC>;
#[doc = "Register `a1_msb` writer"]
pub type W = crate::W<A1_MSB_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<A1_MSB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Argument A1 MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a1_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a1_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A1_MSB_SPEC;
impl crate::RegisterSpec for A1_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a1_msb::R`](R) reader structure"]
impl crate::Readable for A1_MSB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`a1_msb::W`](W) writer structure"]
impl crate::Writable for A1_MSB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets a1_msb to value 0"]
impl crate::Resettable for A1_MSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
