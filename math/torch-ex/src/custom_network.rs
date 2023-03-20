use anyhow::Result; // provices easy error handling
use tch::{nn, nn::ModuleT, nn::OptimizerConfig, Device, Tensor, kind};

// Step 1: Define a structure which will contain the layers

#[derive(Debug)]
struct Network {
    conv1: nn::Conv2D,
    conv2: nn::Conv2D,
    fc1 : nn::Linear,
    fc2 : nn::Linear,
}

// Step 2: Create implementation of the network or initializer -> this is similar to the __init__ in python
impl Network {
    // vs:: represents where the variables are stored in device (weights)
    fn new(vs: &nn::Path) -> Network{
        let conv1 = nn::conv2d(vs, 1, 32, 5, Default::default()); // Default uses the same datatype as the input
        let conv2 = nn::conv2d(vs, 32, 64, 5, Default::default());
        let fc1 = nn::linear(vs, 1024, 1024, Default::default());
        let fc2 = nn::linear(vs , 1024, 10, Default::default());
        Network {conv1, conv2, fc1, fc2}
    }
}

impl nn::ModuleT for Network {
    fn forward_t(&self, xs: &Tensor, train: bool) -> Tensor{
        xs.view([-1, 1, 28,28])
        .apply(&self.conv1) // This is how we apply the components of our network to the input tensor
        .max_pool2d_default(2)
        .apply(&self.conv2)
        .max_pool2d_default(2)
        .view([-1, 1024])
        .apply(&self.fc1)
        .relu()
        .dropout(0.5, train)
        .apply(&self.fc2)
    }
}

pub fn run() -> Result<()> {
    let vs = nn::VarStore::new(Device::cuda_if_available());
    let network = Network::new(&vs.root());

    let t = Tensor::randn(&[30,1,28,28], kind::FLOAT_CPU);
    
    let results = network.forward_t(&t, false);

    println!("{:?}", results);

    Ok(())
}

pub fn main() {
   _ = run();
}