pub struct Checkpoint {
    pub token_index: usize,
    pub concept: String,         // eg. "trust", "entanglement", "contradiction"
    pub alignment_score: f64,    // how much the model agrees with the concept
}
