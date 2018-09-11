#[macro_use]
extern crate criterion;
extern crate blackjack;

use blackjack::prelude::*;
use criterion::Criterion;


fn criterion_bechmark(c: &mut Criterion) {

    let inputs = vec![
        Series::arange(0, 10000)
    ];

    c.bench_function_over_inputs(
        "series min",  
        |b, series| b.iter(|| series.min::<i64>()),
        inputs.clone()
    );

    c.bench_function_over_inputs(
        "series max",  
        |b, series| b.iter(|| series.max::<i64>()),
        inputs.clone()
    );

    c.bench_function_over_inputs(
        "series mean",  
        |b, series| b.iter(|| series.mean()),
        inputs.clone()
    );
    
    c.bench_function_over_inputs(
        "series sum",  
        |b, series| b.iter(|| series.sum::<i64>()),
        inputs.clone()
    );

    c.bench_function(
        "series index", 
        |b| b.iter_with_setup(|| {
                Series::arange(0, 10000)
            }, |mut series| {
                series[0] = 1.into();
            })
    );

    c.bench_function(
        "series append", 
        |b| b.iter_with_setup(|| {
                Series::arange(0, 10000)
            }, |mut series| {
                series.append(1);
            })
    );

    c.bench_function(
        "dataframe read_csv BASIC",
        |b| b.iter(|| {
            let path = format!("{}/tests/data/basic_csv.csv", env!("CARGO_MANIFEST_DIR"));
            let _df = DataFrame::read_csv(path);
        })
    );
}

criterion_group!(benches, criterion_bechmark);
criterion_main!(benches);