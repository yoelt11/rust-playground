use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rayon_test::{create_random_nodes, number_of_nodes_with_norm_gt, par_number_of_nodes_with_norm_gt}; 


fn number_of_nodes_with_norm_gt_benchmkark(c: &mut Criterion) {
    let mut nodes = create_random_nodes(1000000); 
    c.bench_function("unparallelized", |b| b.iter(|| number_of_nodes_with_norm_gt(black_box(&mut nodes))));
}

fn par_number_of_nodes_with_norm_gt_benchmkark(c: &mut Criterion) {
    let mut nodes = create_random_nodes(1000000); 
    c.bench_function("parallelized", |b| b.iter(|| par_number_of_nodes_with_norm_gt(black_box(&mut nodes))));
}

criterion_group!(benches, number_of_nodes_with_norm_gt_benchmkark, par_number_of_nodes_with_norm_gt_benchmkark);
criterion_main!(benches);
