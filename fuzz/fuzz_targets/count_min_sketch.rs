#![no_main]
use libfuzzer_sys::fuzz_target;

use pdatastructs::countminsketch::CountMinSketch;

fuzz_target!(|data: &[u8]| {
    let epsilon = 0.1;  // error threshold
    let delta = 0.2;  // epsilon is hit in (1 - 0.2) * 100% = 80%
    let mut cms = CountMinSketch::<&str, u32>::with_point_query_properties(
        epsilon, delta
    );
    for _ in 0..100 {
        match std::str::from_utf8(data) {
            Ok(s) => {
                cms.add(&s);
            },
            Err(_) => {},
        }
    }
});
