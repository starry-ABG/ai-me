use candle_core::{Device, MetalDevice, Tensor};


fn init() {

    let device = &Device::Cpu;
    let d_f32 = candle_core::DType::F32;
    let sha = (2,3);

    // zeros
    let tensor = Tensor::zeros(sha, d_f32, device).unwrap();
    tensor.ones_like().unwrap();

    // ones
    let one = Tensor::ones(sha, d_f32, device).unwrap();
    tensor.zeros_like().unwrap();

    // rand
    let tensor_r1 = Tensor::rand(-1., 1., sha, device).unwrap();
    tensor_r1.rand_like(-2., 3.).unwrap();
    let tensor_r2 = Tensor::randn(1.2, 0.33, sha, device).unwrap();
    tensor_r2.randn_like(3.1, 0.3).unwrap();

    // arange
    Tensor::arange(2i64, 2934i64, device).unwrap();
    Tensor::arange_step(3i64, 8923i64, 23, device).unwrap();

    // linspace


}


#[cfg(test)]
mod tests {
    use super::*;



}