# Brainfuck-rs
A tiny & fast as fuck brainfuck interpreter written in Rust

# Benchmark
### Brainfuck-rs 3.0 (Rust)
https://github.com/parvusvox/brainfuck-rs
```
cargo run examples/helloworld.bf --benchmark 1000
```
 - Total execution time: 399.15 ms
 - Average execution time: 0.40 ms


### Brainfuck-rs 2.0 (Rust)
https://github.com/parvusvox/brainfuck-rs @ commit 47bc043e0354f2c46f845e976e5edb80927dc2b6
```
cargo run examples/helloworld.bf --benchmark 1000
```
 - Total execution time: 576.61 ms
 - Average execution time: 0.58 ms


### Brainfuck-rs 1.0 (Rust)
https://github.com/parvusvox/brainfuck-rs @ commit e7a53718e1f1869e984c02824d9b65f9b5c31ed2
```
cargo run examples/helloworld.bf --benchmark 1000
```
 - Total execution time: 624.83 ms
 - Average execution time: 0.62 ms


### Python-Brainfuck (Python)
https://github.com/pocmo/Python-Brainfuck
```
chmod +x benchmark_py
./benchmark_py
```
 - Total execution time: 11494.40 ms 
 - Average execution time: 11.49 ms


### Brainfuck (C)
https://github.com/fabianishere/brainfuck 
```
chmod +x benchmark_c
./benchmark_c
```
 - Total execution time: 557.00 ms
 - Average execution time: 0.56 ms


### Brainfuck-Interpreter (C++)
https://github.com/texus/Brainfuck-interpreter
```
chmod +x benchmark_cpp
./benchmark_cpp
```
 - Total execution time: 1958.00 ms 
 - Average execution time: 1.97 ms


