#![no_main]
use libfuzzer_sys::fuzz_target;

use pdatastructs::reservoirsampling::ReservoirSampling;
use pdatastructs::rand::SeedableRng;
use rand_chacha::ChaChaRng;

fuzz_target!(|data: &[u8]| {
    let rng = ChaChaRng::from_seed([0; 32]);
    let mut sampler = ReservoirSampling::new(10, rng);

    for i in data {
        sampler.add(*i);
    }
    let _: Vec<&u8> = sampler.reservoir().iter().collect();
});
