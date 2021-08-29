mod brainfuck;
mod shell;

use indicatif::{ProgressBar, ProgressStyle};
use std::time::{Instant};

use clap::{Arg, App};
use crate::brainfuck::{
    execute_directly_to_vec,
};
use crate::shell::run_shell;

fn main() {
    let matches = App::new("bfrs")
        .version("3.0")
        .author("Ian Kim. <ian@ianmkim.com>")
        .about("A lightning fast brainfuck interpreter written in Rust")
        .arg(Arg::new("file")
             .value_name("FILENAME")
             .about("Brainfuck source file")
             .index(1))
        .arg(Arg::new("benchmark")
             .long("benchmark")
             .short('b')
             .value_name("NUM_REPEAT")
             .about("Benchmark the Brainfuck interpreter using input file"))
        .get_matches();

    if let Some(benchmark) = matches.value_of("benchmark"){
        let benchmark_num = benchmark.parse::<u32>().unwrap();
        let bar = ProgressBar::new(benchmark_num as u64);
        bar.set_style(ProgressStyle::default_bar()
                      .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}")
                      .progress_chars("##-"));

        let mut time = 0f64; 
        let filename = matches.value_of("file").unwrap();
        for _ in 0..benchmark_num{
            bar.inc(1);
            let start = Instant::now();
            execute_directly_to_vec(filename.clone(), false);
            let duration = start.elapsed().subsec_nanos() as f64 / 1000000 as f64;
            time += duration;
        }
        bar.finish();

        println!("Executed {} {} times", filename, benchmark_num);
        println!("\tTotal execution time: {}ms", time);
        println!("\tAverage execution time: {}ms", time/benchmark_num as f64);

    } else if let Some(filename) = matches.value_of("file"){
        execute_directly_to_vec(filename, true);
    } else {
        run_shell();
    }
}

