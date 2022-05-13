#![no_main]
use libfuzzer_sys::fuzz_target;

use pdatastructs::countminsketch::CountMinSketch;
use pdatastructs::topk::cmsheap::CMSHeap;

fuzz_target!(|data: &[u8]| {
    let cms = CountMinSketch::with_point_query_properties(0.1, 0.2);
    let mut tk = CMSHeap::new(2, cms);
    for i in data {
        tk.add(*i);
    }

    let _: Vec<u8> = tk.iter().collect();
});
