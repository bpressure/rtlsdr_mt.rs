# rtlsdr\_mt – High-level, multithreading interface to RTL-SDR

[Documentation](https://kchmck.github.io/doc/rtlsdr_mt/rtlsdr_mt/)

This crate provides a high-level interface to the RTL-SDR that separates controlling
the device and reading samples, for integration into multithreaded applications.

## Example

This example reads incoming samples, printing the first I/Q pair, in the main thread
while incrementing the receive frequency by 1kHz every second in a subthread.

```rust
let (mut ctl, mut reader) = rtlsdr_mt::open(0).unwrap();

ctl.enable_agc().unwrap();
ctl.set_ppm(-2).unwrap();
ctl.set_center_freq(774_781_250).unwrap();

std::thread::spawn(move || {
    loop {
        let next = ctl.center_freq() + 1000;
        ctl.set_center_freq(next).unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
});

reader.read_async(4, 32768, |bytes| {
    println!("i[0] = {}", bytes[0]);
    println!("q[0] = {}", bytes[1]);
}).unwrap();
```

## Usage

This crate can be used through cargo by adding it as a dependency in `Cargo.toml`:

```toml
[dependencies]
rtlsdr_mt = "2.0.0"
```
and importing it in the crate root:

```rust
extern crate rtlsdr_mt;
```
