pub struct PathEmbedding {
    // Unique identifier of reasoning session
    pub session_id: String,

    // list of tokens recognised by the model as "self-acknowledged"
    pub anchor_tokens: Vec<String>,

    // Markers of reasoning focus, chosen by Reasoner
    pub semantic_checkpoints: Vec<Checkpoint>,

    // Time-series of divergence spikes and their resolutions
    pub empathic_gradient_trace: Vec<GradientStep>,

    // Context embeddings before, during and after reasoning. Note the precision
    pub context_vectors: Vec<Vec<f64>>,

    // Metadata for a reasoning (date, author, signature)
    pub metadata: MetaInfo,
}
