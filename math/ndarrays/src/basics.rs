use nalgebra::{Vector2, Vector3};
use nshare::ToNalgebra;

fn main() {
    let vec2d = Vector2::new(1.0, 2.0);
    println!("Fixed2d Vector: {:?}", vec2d);

    let vector_1 = Vector3::new(1.0, 2.0, 3.5);
    println!("Fixed3d Vector: {:?}", vector_1);

    let vector_2 = Vector3::from_column_slice(&[4.0, 5.9, 5.2]);
    println!("Fixed 3D vector from slice {:?}", vector_2);

    let vector_3 = Vector3::from_fn(|i, _| (i + 2) as f32);
    println!("Fixed 3D vector from fn {:?}", vector_3);

    let vector_4 = Vector3::from_iterator((1..=3).map(|x| x as f32));
    println!("Fixed 3D vector from fn {:?}", vector_4);

    // easy way: however this array belong to ndarray and not nalgebra, to convert crate nshare is needed!
    let a1 = ndarray::array![1.9, 2.3, 4.5];
    let a2 = a1.view().into_nalgebra();
    println!("ndarray to nalgebra: {:?}", a2);
    
}
