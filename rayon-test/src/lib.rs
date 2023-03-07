use rayon::prelude::*;
use rand::prelude::*;


pub struct Node {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Node {

    fn new(x: f64, y: f64, z: f64) -> Self{
        Node {x : x,
              y : y,
              z : z,
        }
    }

    fn norm(&self) -> f64{
       f64::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z))
    }
    
    fn dist_between_nodes(&self, ref_node: Node) -> f64  {
       f64::sqrt((self.x - ref_node.x).powf(2.0) + (self.y - ref_node.y).powf(2.0) + (self.z - ref_node.z).powf(2.0))
    }
}

pub fn create_random_nodes(n: usize) -> Vec<Node>{
    let mut nodes = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);
        let z = rng.gen_range(-1.0..1.0);
        nodes.push(Node::new(x, y, z));
    }
    nodes
}

pub fn number_of_nodes_with_norm_gt(nodes: &Vec<Node>) -> usize{
    nodes.iter().filter(|&n| n.norm() > 1.0).count()
}

pub fn par_number_of_nodes_with_norm_gt(nodes: &Vec<Node>) -> usize{
    nodes.par_iter().filter(|&n| n.norm() > 1.0).count()
}
