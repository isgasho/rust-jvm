use std::collections::HashMap;
use std::sync::Mutex;

/**
 * 0: no info
 * 1: emit instruction
 * 2: emit instruction + operandstack
 */
#[derive(Debug)]
pub struct RjOption {
    pub debug_mode: usize,
}

impl RjOption {
    pub fn new() -> RjOption {
        RjOption { debug_mode: 0 }
    }
}

lazy_static! {
    pub static ref RJ_OPTION: Mutex<RjOption> = Mutex::new(RjOption::new());
    pub static ref OBJECT_ID: Mutex<usize> = Mutex::new(0);
    pub static ref STRING_POOL: Mutex<HashMap<usize, String>> = Mutex::new(HashMap::new());
}
