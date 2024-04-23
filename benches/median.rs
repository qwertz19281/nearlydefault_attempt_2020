use criterion::{black_box, criterion_group, criterion_main, Criterion};
use medion::{alg::process, Opt, alg_alt::process_alt};
use std::path::PathBuf;

pub fn criterion_benchmark(c: &mut Criterion) {
    //*c=c.sample_size(10);

    {
        let mut g = c.benchmark_group("median");

        let small = {
            let data = include_bytes!("small.png");
            image::load_from_memory(data).unwrap().into_rgba()
        };
        let big = {
            let data = include_bytes!("big.png");
            image::load_from_memory(data).unwrap().into_rgba()
        };
        let osmall = Opt{
            scale: 4,
            factor: 2,
            wrap: true,
            circular_mod: 0.01,
            alternative: false,
            include: None,
            exclude: None,
            input: PathBuf::new(),
            output: PathBuf::new(),
            include_regex: None,
            exclude_regex: None,
        };
        let obig = Opt{
            scale: 16,
            factor: 8,
            wrap: true,
            circular_mod: 0.01,
            alternative: false,
            include: None,
            exclude: None,
            input: PathBuf::new(),
            output: PathBuf::new(),
            include_regex: None,
            exclude_regex: None,
        };
        let o4 = Opt{
            scale: 16,
            factor: 6,
            wrap: true,
            circular_mod: 0.01,
            alternative: false,
            include: None,
            exclude: None,
            input: PathBuf::new(),
            output: PathBuf::new(),
            include_regex: None,
            exclude_regex: None,
        };

        /*g.bench_function("small_old", |b| {
            b.iter(|| black_box( process(&small,&osmall) ) );
        });
        g.bench_function("small_new", |b| {
            b.iter(|| black_box( process_alt(&small,&osmall) ) );
        });
        g.bench_function("sig_old", |b| {
            b.iter(|| black_box( process(&small,&obig) ) );
        });
        g.bench_function("sig_new", |b| {
            b.iter(|| black_box( process_alt(&small,&obig) ) );
        });
        g.bench_function("bmall_old", |b| {
            b.iter(|| black_box( process(&big,&osmall) ) );
        });
        g.bench_function("bmall_new", |b| {
            b.iter(|| black_box( process_alt(&big,&osmall) ) );
        });
        g.bench_function("big_old", |b| {
            b.iter(|| black_box( process(&big,&obig) ) );
        });
        g.bench_function("big_new", |b| {
            b.iter(|| black_box( process_alt(&big,&obig) ) );
        });*/
        g.bench_function("med_new", |b| {
            b.iter(|| black_box( process_alt(&big,&o4) ) );
        });
        g.bench_function("med_old", |b| {
            b.iter(|| black_box( process(&big,&o4) ) );
        });
        
        
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);