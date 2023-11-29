#[doc = "Register `status` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `status` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `code` reader - The code of cause"]
pub type CODE_R = crate::FieldReader<u32>;
#[doc = "Field `code` writer - The code of cause"]
pub type CODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
#[doc = "Field `mode` reader - The mode of execution flow change"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "The mode of execution flow change\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "3: `11`"]
    INTERRUPT = 3,
    #[doc = "2: `10`"]
    EXCEPTION = 2,
    #[doc = "1: `1`"]
    AWAIT = 1,
    #[doc = "0: `0`"]
    FINISH = 0,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            3 => MODE_A::INTERRUPT,
            2 => MODE_A::EXCEPTION,
            1 => MODE_A::AWAIT,
            0 => MODE_A::FINISH,
            _ => unreachable!(),
        }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == MODE_A::INTERRUPT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_exception(&self) -> bool {
        *self == MODE_A::EXCEPTION
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_await(&self) -> bool {
        *self == MODE_A::AWAIT
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_finish(&self) -> bool {
        *self == MODE_A::FINISH
    }
}
#[doc = "Field `mode` writer - The mode of execution flow change"]
pub type MODE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, MODE_A>;
impl<'a, REG, const O: u8> MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`11`"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::INTERRUPT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn exception(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::EXCEPTION)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn await_(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::AWAIT)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn finish(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::FINISH)
    }
}
impl R {
    #[doc = "Bits 0:29 - The code of cause"]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bits 30:31 - The mode of execution flow change"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:29 - The code of cause"]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CODE_W<STATUS_SPEC, 0> {
        CODE_W::new(self)
    }
    #[doc = "Bits 30:31 - The mode of execution flow change"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<STATUS_SPEC, 30> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Execution Flow status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
