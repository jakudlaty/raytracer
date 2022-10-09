use rand::Rng;
pub struct CachedRandom {
    curr_index: usize,
    slice: Vec<f64>,
}

impl CachedRandom {
    pub(crate) fn new() -> Self {
        let mut rand = rand::thread_rng();
        let slice = vec![0; 10000].iter().map(|_| rand.gen::<f64>()).collect();

        Self {
            curr_index: 0,
            slice,
        }
    }

    pub(crate) fn get_next(&mut self) -> f64 {
        let val = self.slice[self.curr_index];
        self.curr_index += 1;
        if self.curr_index >= self.slice.len() {
            self.curr_index = 0;
        }
        val
    }
}
