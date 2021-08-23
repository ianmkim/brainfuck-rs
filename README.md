# Brainfuck-rs
A tiny brainfuck interpreter written in Rust

# Benchmark
### Brainfuck-rs (Rust)
https://github.com/parvusvox/brainfuck-rs
```
cargo run examples/helloworld.bf --benchmark 10000
```
Executing examples/helloworld.bf 1000 times:
 - Total execution time: 624.83 ms
 - Average execution time: 0.62 ms

### Python-Brainfuck (Python)
https://github.com/pocmo/Python-Brainfuck
```
chmod +x benchmark_py
./benchmark_py
```
Executing examples/helloworld.bf 1000 times
 - Total execution time: 11494.40 ms 
 - Average execution time: 11.49 ms

### Brainfuck (C)
https://github.com/fabianishere/brainfuck 
```
chmod +x benchmark_c
./benchmark_c
```
Executing examples/helloworld.bf 1000 times
 - Total execution time: 557.00 ms
 - Average execution time: 0.56 ms

### Brainfuck-Interpreter (C++)
https://github.com/texus/Brainfuck-interpreter
```
chmod +x benchmark_cpp
./benchmark_cpp
```
Executing examples/helloworld.bf 1000 times
 - Total execution time: 1958.00 ms 
 - Average execution time: 1.97 ms


