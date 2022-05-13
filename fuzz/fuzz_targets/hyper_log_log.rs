#![no_main]
use libfuzzer_sys::fuzz_target;

use pdatastructs::hyperloglog::HyperLogLog;

fuzz_target!(|data: &[u8]| {
    let address_bits = 4;
    let mut hll = HyperLogLog::new(address_bits);

    for i in data {
        hll.add(i);
    }
    let _ = hll.count();
});
