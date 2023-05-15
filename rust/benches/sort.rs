use criterion::{criterion_main, criterion_group, Criterion, PlotConfiguration, BenchmarkId, BatchSize};
use criterion::AxisScale::Logarithmic;
use algorithms::sort::prelude::{insertion_sort, bubble_sort, shell_sort, merge_sort, quicksort};
use algorithms::sort::parallel::{quicksort::quicksort as parallel_quicksort, mergesort::merge_sort as parallel_merge_sort};

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use lazy_static::lazy_static;
use SIZES::{EXTREME, VERY_LARGE, VERY_SMALL};
use crate::SIZES::{LARGE, MID, SMALL};

lazy_static! {
    static ref NUMBERS_10: Vec<u32> = read_numbers_from_file("bench_data/random_numbers_10.txt");
    static ref NUMBERS_100: Vec<u32> = read_numbers_from_file("bench_data/random_numbers_100.txt");
    static ref NUMBERS_1000: Vec<u32> = read_numbers_from_file("bench_data/random_numbers_1000.txt");
    static ref NUMBERS_10000: Vec<u32> = read_numbers_from_file("bench_data/random_numbers_10000.txt");
    static ref NUMBERS_100000: Vec<u32> = read_numbers_from_file("bench_data/random_numbers_100000.txt");
    static ref NUMBERS_1000000: Vec<u32> = read_numbers_from_file("bench_data/random_numbers_1000000.txt");
}

fn read_numbers_from_file(filename: &str) -> Vec<u32> {
    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.split(',')
        .map(|s| u32::from_str(s).unwrap())
        .collect()
}

#[derive(Debug, Copy, Clone)]
enum SIZES {
    VERY_SMALL,
    SMALL,
    MID,
    LARGE,
    VERY_LARGE,
    EXTREME,
}

impl Display for SIZES {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn get_test_vec(size: SIZES) -> &'static Vec<u32> {
    match size {
        VERY_SMALL => &NUMBERS_10,
        SMALL => &NUMBERS_100,
        MID => &NUMBERS_1000,
        LARGE => &NUMBERS_10000,
        VERY_LARGE => &NUMBERS_100000,
        EXTREME => &NUMBERS_1000000,
    }
}

macro_rules! bench_sort {
    ($group:ident, $name:expr, $sort_func:expr, $size:ident) => {
        $group.bench_with_input(
            BenchmarkId::new($name, $size),
            &$size,
            |b, &$size| {
                b.iter_batched(
                    || get_test_vec($size),
                    |data| $sort_func(&mut data.clone()),
                    BatchSize::SmallInput,
                )
            },
        );
    };
}

fn sort_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting Algorithms");
    group.plot_config(PlotConfiguration::default().summary_scale(Logarithmic));

    for size in [SMALL, MID, LARGE] {
        bench_sort!(group, "Bubble Sort", bubble_sort, size);
        bench_sort!(group, "Insertion Sort", insertion_sort, size);
        bench_sort!(group, "Shell Sort", shell_sort, size);
        bench_sort!(group, "Merge Sort", merge_sort, size);
        bench_sort!(group, "Quick Sort", quicksort, size);
        bench_sort!(group, "Parallel Merge Sort", parallel_merge_sort, size);
        bench_sort!(group, "Parallel Quick Sort", parallel_quicksort, size);
    }

    group.finish()
}

criterion_group!(benches, sort_bench);
criterion_main!(benches);