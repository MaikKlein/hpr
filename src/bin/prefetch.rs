#[macro_use]
extern crate criterion;

#[macro_use]
extern crate lazy_static;
use criterion::Criterion;

lazy_static! {
    static ref DIMENSIONS: Vec<usize> = {
        (1..=50).collect()
    };
}
const ROW_LENGTH: usize = 10_000;

type Mat = Vec<Vec<f32>>;

fn dot(mat: &Mat) -> f32 {
    let mut res = 0.0;
    for row in 0..ROW_LENGTH {
        let mut mul = 1.0;
        for col in 0..mat.len() {
            mul *= mat[col][row];
        }
        res += mul;
    }
    res
}

#[inline(never)]
fn gen_mat(dim: usize) -> Mat {
    (0..dim)
        .map(|_| {
            (0..ROW_LENGTH)
                .map(|_| f32::sqrt(ROW_LENGTH as f32))
                .collect()
        }).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(&format!("Prefetch"), |b, &&dim| {
         b.iter_with_setup(move || gen_mat(dim), |mat| dot(&mat));
    }, &*DIMENSIONS);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
