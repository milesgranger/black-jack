#[macro_use]
extern crate criterion;
extern crate blackjack;

use blackjack::prelude::*;
use criterion::Criterion;


fn bench_series_sum() {
    let series = Series::arange(0, 10000);
    series.sum::<i64>();
}

fn bench_series_min() {
    let series = Series::arange(0, 10000);
    series.min::<i64>().unwrap();
}

fn bench_series_max() {
    let series = Series::arange(0, 10000);
    series.max::<i64>().unwrap();
}

fn bench_series_mean() {
    let series = Series::arange(0, 10000);
    series.mean().unwrap();
}

fn criterion_bechmark(c: &mut Criterion) {
    c.bench_function("series min",  |b| b.iter(|| bench_series_min()));
    c.bench_function("series max",  |b| b.iter(|| bench_series_max()));
    c.bench_function("series mean", |b| b.iter(|| bench_series_mean()));
    c.bench_function("series sum",  |b| b.iter(|| bench_series_sum()));
}

criterion_group!(benches, criterion_bechmark);
criterion_main!(benches);