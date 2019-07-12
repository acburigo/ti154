pub mod mac;
pub mod rpc;
pub mod sys;
pub mod util;

use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive, PartialEq)]
pub enum MTSubsystem {
    RPC = 0,
    SYS = 1,
    MAC = 2,
    UTIL = 7,
}
