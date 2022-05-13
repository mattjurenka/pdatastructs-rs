#![no_main]
use libfuzzer_sys::fuzz_target;

use pdatastructs::topk::lossycounter::LossyCounter;

fuzz_target!(|data: &[u8]| {
    let epsilon = 0.1;  // error threshold
    let mut lc = LossyCounter::with_epsilon(epsilon);
    for i in data {
        lc.add(*i);
    }
    let _: Vec<u8> = lc.query(0.2).collect();
});
