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
        |b, series| b.iter(|| series.min()),
        inputs.clone()
    );

    c.bench_function_over_inputs(
        "series max",  
        |b, series| b.iter(|| series.max()),
        inputs.clone()
    );

    c.bench_function_over_inputs(
        "series mean",  
        |b, series| b.iter(|| series.mean()),
        inputs.clone()
    );
    
    c.bench_function_over_inputs(
        "series sum",  
        |b, series| b.iter(|| series.sum()),
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

    /*
    c.bench_function(
        "dataframe read_csv BASIC",
        |b| b.iter(|| {
            let path = format!("{}/tests/data/medium_csv.csv", env!("CARGO_MANIFEST_DIR"));
            let _df = DataFrame::read_csv(path, b',');
        })
    );
    */
    
    c.bench_function(
        "series scalar ops - (Mul)",
        |b| b.iter_with_setup(|| {
                Series::arange(0, 10000)
            }, | series | {
                let _series = series * 2;
            })
    );

    c.bench_function(
        "series scalar ops - (MulAssign)",
        |b| b.iter_with_setup(|| {
                let s = Series::arange(0, 10000);
                s.astype::<i64>().unwrap();
                s
            }, | mut series | {
                series *= 2_i64;
            })
    );

    c.bench_function(
        "series agg ops (MODE)",
        |b| b.iter_with_setup(|| {
                let mut s = Series::arange(0, 10000);
                s.append(0);
                s.append(0);
                s.append(1);
                s.append(1);
                s
            }, | series | {
                let _mode = series.mode().unwrap();
            })
    );

    c.bench_function(
        "series agg ops (VAR)",
        |b| b.iter_with_setup(|| {
                Series::arange(0, 10000)
            }, | series | {
                let _var = series.var(1_f64).unwrap();
            })
    );

    c.bench_function(
        "series agg ops (STD)",
        |b| b.iter_with_setup(|| {
                Series::arange(0, 10000)
            }, | series | {
                let _std = series.std(1.).unwrap();
            })
    );

    c.bench_function(
        "series agg ops (MEDIAN)",
        |b| b.iter_with_setup(|| {
                Series::arange(0, 10000)
            }, | series | {
                let _median = series.median().unwrap();
            })
    );

    c.bench_function(
        "series agg ops (QUANTILE)",
        |b| b.iter_with_setup(|| {
                Series::arange(0, 10000)
            }, | series | {
                let _qtl = series.quantile(0.5).unwrap();
            })
    );

    c.bench_function(
        "series groupby",
        |b| b.iter_with_setup(|| {
            let series = Series::arange(0, 10000);
            let keys   = Series::arange(0, 10000);
            (keys, series)
        }, |(keys, series)| {
            let _res = series.groupby(&keys).sum();
        })
    );

    c.bench_function(
        "series rolling (MEAN)",
        |b| b.iter_with_setup(|| {
            let series = Series::arange(0, 10000);
            series
        }, |series| {
            let _res = series.rolling(5).mean();
        })
    );


}

criterion_group!(benches, criterion_bechmark);
criterion_main!(benches);