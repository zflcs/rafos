#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Executor Base Address register"]
    pub eptr: EPTR,
    #[doc = "0x04 - Execution Flow status register"]
    pub status: STATUS,
    #[doc = "0x08 - Message Buffer Base Address register"]
    pub msgbuf: MSGBUF,
    #[doc = "0x0c - Current coroutine register"]
    pub curc: CURC,
}
#[doc = "eptr (rw) register accessor: Executor Base Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`eptr`]
module"]
pub type EPTR = crate::Reg<eptr::EPTR_SPEC>;
#[doc = "Executor Base Address register"]
pub mod eptr;
#[doc = "status (rw) register accessor: Execution Flow status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Execution Flow status register"]
pub mod status;
#[doc = "msgbuf (rw) register accessor: Message Buffer Base Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msgbuf`]
module"]
pub type MSGBUF = crate::Reg<msgbuf::MSGBUF_SPEC>;
#[doc = "Message Buffer Base Address register"]
pub mod msgbuf;
#[doc = "curc (rw) register accessor: Current coroutine register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`curc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`curc`]
module"]
pub type CURC = crate::Reg<curc::CURC_SPEC>;
#[doc = "Current coroutine register"]
pub mod curc;
