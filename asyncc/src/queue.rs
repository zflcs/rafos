/// this mod define the message buffer
/// 

use heapless::mpmc::MpMcQueue;

use crate::TaskRef;
/// 
pub type MsgQueue = MpMcQueue<TaskRef, 128>;