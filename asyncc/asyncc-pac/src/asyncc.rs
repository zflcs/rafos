#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    eptr: EPTR,
    status: STATUS,
    msgbuf: MSGBUF,
    curc: CURC,
    a0_lsb: A0_LSB,
    a0_msb: A0_MSB,
    a1_lsb: A1_LSB,
    a1_msb: A1_MSB,
    a2_lsb: A2_LSB,
    a2_msb: A2_MSB,
    a3_lsb: A3_LSB,
    a3_msb: A3_MSB,
    a4_lsb: A4_LSB,
    a4_msb: A4_MSB,
    a5_lsb: A5_LSB,
    a5_msb: A5_MSB,
    a6_lsb: A6_LSB,
    a6_msb: A6_MSB,
    a7_lsb: A7_LSB,
    a7_msb: A7_MSB,
}
impl RegisterBlock {
    #[doc = "0x00 - Executor Base Address register"]
    #[inline(always)]
    pub const fn eptr(&self) -> &EPTR {
        &self.eptr
    }
    #[doc = "0x04 - Execution Flow status register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - Message Buffer Base Address register"]
    #[inline(always)]
    pub const fn msgbuf(&self) -> &MSGBUF {
        &self.msgbuf
    }
    #[doc = "0x0c - Current coroutine register"]
    #[inline(always)]
    pub const fn curc(&self) -> &CURC {
        &self.curc
    }
    #[doc = "0x10 - Argument A0 LSB"]
    #[inline(always)]
    pub const fn a0_lsb(&self) -> &A0_LSB {
        &self.a0_lsb
    }
    #[doc = "0x14 - Argument A0 MSB"]
    #[inline(always)]
    pub const fn a0_msb(&self) -> &A0_MSB {
        &self.a0_msb
    }
    #[doc = "0x18 - Argument A1 LSB"]
    #[inline(always)]
    pub const fn a1_lsb(&self) -> &A1_LSB {
        &self.a1_lsb
    }
    #[doc = "0x1c - Argument A1 MSB"]
    #[inline(always)]
    pub const fn a1_msb(&self) -> &A1_MSB {
        &self.a1_msb
    }
    #[doc = "0x20 - Argument A2 LSB"]
    #[inline(always)]
    pub const fn a2_lsb(&self) -> &A2_LSB {
        &self.a2_lsb
    }
    #[doc = "0x24 - Argument A2 MSB"]
    #[inline(always)]
    pub const fn a2_msb(&self) -> &A2_MSB {
        &self.a2_msb
    }
    #[doc = "0x28 - Argument A3 LSB"]
    #[inline(always)]
    pub const fn a3_lsb(&self) -> &A3_LSB {
        &self.a3_lsb
    }
    #[doc = "0x2c - Argument A3 MSB"]
    #[inline(always)]
    pub const fn a3_msb(&self) -> &A3_MSB {
        &self.a3_msb
    }
    #[doc = "0x30 - Argument A4 LSB"]
    #[inline(always)]
    pub const fn a4_lsb(&self) -> &A4_LSB {
        &self.a4_lsb
    }
    #[doc = "0x34 - Argument A4 MSB"]
    #[inline(always)]
    pub const fn a4_msb(&self) -> &A4_MSB {
        &self.a4_msb
    }
    #[doc = "0x38 - Argument A5 LSB"]
    #[inline(always)]
    pub const fn a5_lsb(&self) -> &A5_LSB {
        &self.a5_lsb
    }
    #[doc = "0x3c - Argument A5 MSB"]
    #[inline(always)]
    pub const fn a5_msb(&self) -> &A5_MSB {
        &self.a5_msb
    }
    #[doc = "0x40 - Argument A6 LSB"]
    #[inline(always)]
    pub const fn a6_lsb(&self) -> &A6_LSB {
        &self.a6_lsb
    }
    #[doc = "0x44 - Argument A6 MSB"]
    #[inline(always)]
    pub const fn a6_msb(&self) -> &A6_MSB {
        &self.a6_msb
    }
    #[doc = "0x48 - Argument A7 LSB"]
    #[inline(always)]
    pub const fn a7_lsb(&self) -> &A7_LSB {
        &self.a7_lsb
    }
    #[doc = "0x4c - Argument A7 MSB"]
    #[inline(always)]
    pub const fn a7_msb(&self) -> &A7_MSB {
        &self.a7_msb
    }
}
#[doc = "eptr (rw) register accessor: Executor Base Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptr`]
module"]
pub type EPTR = crate::Reg<eptr::EPTR_SPEC>;
#[doc = "Executor Base Address register"]
pub mod eptr;
#[doc = "status (rw) register accessor: Execution Flow status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Execution Flow status register"]
pub mod status;
#[doc = "msgbuf (rw) register accessor: Message Buffer Base Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgbuf`]
module"]
pub type MSGBUF = crate::Reg<msgbuf::MSGBUF_SPEC>;
#[doc = "Message Buffer Base Address register"]
pub mod msgbuf;
#[doc = "curc (rw) register accessor: Current coroutine register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`curc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@curc`]
module"]
pub type CURC = crate::Reg<curc::CURC_SPEC>;
#[doc = "Current coroutine register"]
pub mod curc;
#[doc = "a0_lsb (rw) register accessor: Argument A0 LSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a0_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a0_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a0_lsb`]
module"]
pub type A0_LSB = crate::Reg<a0_lsb::A0_LSB_SPEC>;
#[doc = "Argument A0 LSB"]
pub mod a0_lsb;
#[doc = "a0_msb (rw) register accessor: Argument A0 MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a0_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a0_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a0_msb`]
module"]
pub type A0_MSB = crate::Reg<a0_msb::A0_MSB_SPEC>;
#[doc = "Argument A0 MSB"]
pub mod a0_msb;
#[doc = "a1_lsb (rw) register accessor: Argument A1 LSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a1_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a1_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a1_lsb`]
module"]
pub type A1_LSB = crate::Reg<a1_lsb::A1_LSB_SPEC>;
#[doc = "Argument A1 LSB"]
pub mod a1_lsb;
#[doc = "a1_msb (rw) register accessor: Argument A1 MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a1_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a1_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a1_msb`]
module"]
pub type A1_MSB = crate::Reg<a1_msb::A1_MSB_SPEC>;
#[doc = "Argument A1 MSB"]
pub mod a1_msb;
#[doc = "a2_lsb (rw) register accessor: Argument A2 LSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a2_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a2_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2_lsb`]
module"]
pub type A2_LSB = crate::Reg<a2_lsb::A2_LSB_SPEC>;
#[doc = "Argument A2 LSB"]
pub mod a2_lsb;
#[doc = "a2_msb (rw) register accessor: Argument A2 MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a2_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a2_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2_msb`]
module"]
pub type A2_MSB = crate::Reg<a2_msb::A2_MSB_SPEC>;
#[doc = "Argument A2 MSB"]
pub mod a2_msb;
#[doc = "a3_lsb (rw) register accessor: Argument A3 LSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a3_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a3_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a3_lsb`]
module"]
pub type A3_LSB = crate::Reg<a3_lsb::A3_LSB_SPEC>;
#[doc = "Argument A3 LSB"]
pub mod a3_lsb;
#[doc = "a3_msb (rw) register accessor: Argument A3 MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a3_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a3_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a3_msb`]
module"]
pub type A3_MSB = crate::Reg<a3_msb::A3_MSB_SPEC>;
#[doc = "Argument A3 MSB"]
pub mod a3_msb;
#[doc = "a4_lsb (rw) register accessor: Argument A4 LSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a4_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a4_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a4_lsb`]
module"]
pub type A4_LSB = crate::Reg<a4_lsb::A4_LSB_SPEC>;
#[doc = "Argument A4 LSB"]
pub mod a4_lsb;
#[doc = "a4_msb (rw) register accessor: Argument A4 MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a4_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a4_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a4_msb`]
module"]
pub type A4_MSB = crate::Reg<a4_msb::A4_MSB_SPEC>;
#[doc = "Argument A4 MSB"]
pub mod a4_msb;
#[doc = "a5_lsb (rw) register accessor: Argument A5 LSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a5_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a5_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a5_lsb`]
module"]
pub type A5_LSB = crate::Reg<a5_lsb::A5_LSB_SPEC>;
#[doc = "Argument A5 LSB"]
pub mod a5_lsb;
#[doc = "a5_msb (rw) register accessor: Argument A5 MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a5_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a5_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a5_msb`]
module"]
pub type A5_MSB = crate::Reg<a5_msb::A5_MSB_SPEC>;
#[doc = "Argument A5 MSB"]
pub mod a5_msb;
#[doc = "a6_lsb (rw) register accessor: Argument A6 LSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a6_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a6_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a6_lsb`]
module"]
pub type A6_LSB = crate::Reg<a6_lsb::A6_LSB_SPEC>;
#[doc = "Argument A6 LSB"]
pub mod a6_lsb;
#[doc = "a6_msb (rw) register accessor: Argument A6 MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a6_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a6_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a6_msb`]
module"]
pub type A6_MSB = crate::Reg<a6_msb::A6_MSB_SPEC>;
#[doc = "Argument A6 MSB"]
pub mod a6_msb;
#[doc = "a7_lsb (rw) register accessor: Argument A7 LSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a7_lsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a7_lsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a7_lsb`]
module"]
pub type A7_LSB = crate::Reg<a7_lsb::A7_LSB_SPEC>;
#[doc = "Argument A7 LSB"]
pub mod a7_lsb;
#[doc = "a7_msb (rw) register accessor: Argument A7 MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a7_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a7_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a7_msb`]
module"]
pub type A7_MSB = crate::Reg<a7_msb::A7_MSB_SPEC>;
#[doc = "Argument A7 MSB"]
pub mod a7_msb;
