use tch::{kind, Tensor};

pub fn main(){
    // load device
    println!("Cuda available: {}", tch::Cuda::is_available());
    let device = tch::Device::cuda_if_available();
    
    // create tensor
    let t = Tensor::randn(&[3,3,3], kind::FLOAT_CPU).to(device);
    t.print();
    let v = Tensor::zeros(&[3,3,3], kind::FLOAT_CPU).to(device);
    v.print();
    
    // utility to load npy tensors is also available
    //let tensor = tch::Tetensor::read_npy(filename)?;
}