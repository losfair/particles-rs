use std::cell::RefCell;
use rand::{StdRng, SeedableRng, Open01, Rng};
use imports;

thread_local! {
    static GENERATOR: RefCell<StdRng> = {
        let seed: Vec<usize> = (0..512)
            .map(|_| (imports::rand01() * 1000000000.0) as usize)
            .collect();
        RefCell::new(SeedableRng::from_seed(
            seed.as_slice()
        ))
    };
}

pub fn open01() -> f64 {
    GENERATOR.with(|v| {
        v.borrow_mut().gen::<Open01<f64>>().0
    })
}
