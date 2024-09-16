use burn::nn;
use burn::module::Module;
use burn::tensor::backend::Backend;

pub struct GPTConfig {
    vocab_size: i32,
    context_length: i32,
    emb_dim: i32,     
    n_heads: i32, 
    n_layers: i32, 
    drop_rate: f64,
    qkv_bias: boolean
}

#[derive(Module, Debug)]
pub struct GPTModel<B: Backend> {
    

}