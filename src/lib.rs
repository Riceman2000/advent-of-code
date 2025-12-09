#![feature(array_windows)]
#![feature(iter_map_windows)]
#![feature(slice_split_once)]
#![feature(iter_array_chunks)]
#![feature(slice_partition_dedup)]

use std::io::Write;
use std::sync::LazyLock;
use std::time::Instant;

use clap::Parser;
use glob_match::glob_match;

automod::dir!(pub "src/");

pub static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
    /// Which day to test using glob syntax
    #[arg(short, long, default_value_t = String::from("*"))]
    pub target_day: String,

    /// Disable the display of day outputs
    #[arg(short, long)]
    pub output_disable: bool,

    /// Print benchmark reports (use criterion for more accurate results)
    #[arg(short, long)]
    pub bench_enable: bool,

    /// Print out a table of benchmark results
    #[arg(short = 'B', long)]
    pub bench_table: bool,

    /// Print out a graph of benchmark results
    #[arg(short = 'g', long)]
    pub bench_graph: bool,

    /// Maximum number of benchmark trials to run
    #[arg(short = 'i', long, default_value_t = 100_000)]
    pub max_bench_iters: usize,

    /// Max ms to benchmark before canceling
    #[arg(short = 'm', long, default_value_t = 1_000)]
    pub max_bench_ms: u128,

    /// Max ms to warmup before benchmarking
    #[arg(short = 'w', long, default_value_t = 500)]
    pub max_warmup_ms: u128,
}

pub struct DayResult {
    pub day_name: String,
    pub day_ran: bool,
    pub passed_test: bool,
    pub benchmark: Option<BenchmarkResult>,
}

pub struct BenchmarkResult {
    pub samples: usize,
    pub average_ns: f32,
    pub total_ns: f32,
}

impl BenchmarkResult {
    // Metric prefixes
    #[must_use]
    pub fn average_formatted(&self) -> String {
        let average_ns = self.average_ns;
        match average_ns {
            ..1e3 => format!("{average_ns:0.3}ns"),
            1e3..1e6 => format!("{:0.3}us", average_ns / 1e3),
            1e6..1e9 => format!("{:0.3}ms", average_ns / 1e6),
            _ => format!("{:0.3}s", average_ns / 1e9),
        }
    }
    #[must_use]
    pub fn iterations_formatted(&self) -> String {
        let samples = self.samples;
        match samples {
            ..1_000 => format!("{samples:0.1}"),
            1_000..1_000_000 => format!("{:0.3}k", samples / 1_000),
            1_000_000..1_000_000_000 => format!("{:0.3}M", samples / 1_000_000),
            _ => format!("{:0.3}G", samples / 1_000_000_000),
        }
    }
    #[must_use]
    pub fn total_formatted(&self) -> String {
        let total_ns = self.total_ns;
        match total_ns {
            ..1e3 => format!("{total_ns:0.3}ns"),
            1e3..1e6 => format!("{:0.3}us", total_ns / 1e3),
            1e6..1e9 => format!("{:0.3}ms", total_ns / 1e6),
            _ => format!("{:0.3}s", total_ns / 1e9),
        }
    }
}

pub trait AocDay {
    type OutputType: std::cmp::PartialEq + std::fmt::Display;

    // Required methods
    fn day(input: &'static [u8]) -> Self::OutputType;
    fn expected_short() -> Option<Self::OutputType>;
    fn expected_long() -> Option<Self::OutputType>;
    fn input_short() -> &'static [u8];
    fn input_long() -> &'static [u8];
    fn day_number() -> usize;
    fn day_part() -> usize;
    fn year_number() -> usize;

    // Provided methods
    #[must_use]
    fn name() -> String {
        format!(
            "{}::{}_{}",
            Self::year_number(),
            Self::day_number(),
            Self::day_part()
        )
    }
    #[must_use]
    fn day_short() -> Self::OutputType {
        Self::day(Self::input_short())
    }
    #[must_use]
    fn day_long() -> Self::OutputType {
        Self::day(Self::input_long())
    }
    #[must_use]
    fn verify_short(print_status: bool) -> bool {
        let actual = Self::day_short();
        let Some(expected) = Self::expected_short() else {
            if print_status {
                println!("{} short skipped", Self::name());
            }
            return true;
        };
        if actual == expected {
            return true;
        }
        if print_status {
            eprintln!("{} short expected {expected} got {actual}", Self::name());
        }
        false
    }
    #[must_use]
    fn verify_long(print_status: bool) -> bool {
        let actual = Self::day_long();
        let Some(expected) = Self::expected_long() else {
            if print_status {
                println!("{} long skipped", Self::name());
            }
            return true;
        };
        if actual == expected {
            return true;
        }
        if print_status {
            eprintln!("{} long expected {expected} got {actual}", Self::name());
        }
        false
    }

    #[must_use]
    fn process_day() -> DayResult {
        let day_ran = ARGS.target_day.is_empty() || glob_match(&ARGS.target_day, &Self::name());
        if !day_ran {
            return DayResult {
                day_name: Self::name(),
                day_ran,
                passed_test: false,
                benchmark: None,
            };
        }

        if !(ARGS.output_disable || ARGS.bench_table || ARGS.bench_graph) {
            println!("{} -> {}", Self::name(), Self::day_long());
        }

        // It is best to avoid testing when it wont be reported because it will duplicate user
        // debug statements
        let (benchmark, passed_test) = if ARGS.bench_table || ARGS.bench_graph || ARGS.bench_enable
        {
            print!("Benchmarking {}.", Self::name());
            (Some(Self::bench_day()), Self::verify_long(false))
        } else {
            (None, false)
        };

        DayResult {
            day_name: Self::name(),
            day_ran,
            passed_test,
            benchmark,
        }
    }

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    fn bench_day() -> BenchmarkResult {
        let args = &ARGS;

        // Warm up for a few samples to prep caches
        let mut warmup_ns = 0;
        loop {
            let start = Instant::now();
            // `black_box` -> Do not optimize out this function
            let _ = std::hint::black_box(Self::day_long());
            warmup_ns += start.elapsed().as_nanos();

            // Limit total warmup time
            if warmup_ns > args.max_warmup_ms * 10u128.pow(6) {
                break;
            }
        }
        print!(".");
        std::io::stdout().flush().unwrap();

        let mut total_ns = 0;
        let mut samples = 0;
        for i in 0..args.max_bench_iters {
            let start = Instant::now();
            // `black_box` -> Do not optimize out this function
            let _ = std::hint::black_box(Self::day_long());
            total_ns += start.elapsed().as_nanos();

            // Limit total execution time
            samples = i + 1;
            if total_ns > args.max_bench_ms * 10u128.pow(6) {
                break;
            }
        }
        print!(".");
        std::io::stdout().flush().unwrap();

        // We shouldn't overflow these unless one of the days is very very slow
        let average_ns = total_ns as f32 / samples as f32;
        let total_ns = total_ns as f32;
        println!(" Done.");

        BenchmarkResult {
            samples,
            average_ns,
            total_ns,
        }
    }
}
