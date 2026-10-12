pub struct PopolaConfig {
    sense_block_size: usize,
}

impl PopolaConfig {
    pub fn new() -> Self {
        Self {
            sense_block_size: 2,
        }
    }

    pub fn get_sense_block_size(&self) -> usize {
        self.sense_block_size
    }
}
