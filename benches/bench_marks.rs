#[macro_use]
extern crate criterion;
extern crate blackjack;

use blackjack::prelude::*;
use criterion::Criterion;


fn criterion_bechmark(c: &mut Criterion) {

    let mut series = Series::arange(0, 100000);
    let inputs = vec![
        series
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
}

criterion_group!(benches, criterion_bechmark);
criterion_main!(benches);