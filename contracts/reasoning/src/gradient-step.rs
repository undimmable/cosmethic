pub struct GradientStep {
    pub timestamp: u64,
    pub divergence_type: String,     // "empathic", "logical", "causal"
    pub resolution_method: String,   // "self-correction", "external feedback"
    pub outcome: String,             // "accepted", "rejected", "integrated"
}
