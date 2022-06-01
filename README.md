# human-repr
### Generate beautiful human representations of bytes, durations and even throughputs!

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/human_repr.svg)](https://crates.io/crates/human-repr)
[![Docs](https://docs.rs/human-repr/badge.svg)](https://docs.rs/human-repr)

## What it does

Easily generate human-readable descriptions directly on primitive numbers, of several kinds:
- counts, supporting SI prefixes `k`, `M`, `G`, `T`, `P`, `E`, `Z`, and `Y` (optional IEC standard);
- durations, supporting nanos (`ns`), millis (`ms`), micros (`µs`), seconds (`s`), and even hour-minute-seconds (`HH:MM:SS`);
- throughputs, supporting per-day (`/d`), per-hour (`/h`), per-month (`/m`), and per-sec (`/s`).

It is also blazingly fast, taking only ~80 ns to generate a representation, and well-tested. Does not use any dependencies.

They work on all Rust primitive number types: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `f32`,
`f64`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.

The `what` parameter some methods refer to means the entity you're dealing with, like bytes, actions, iterations, errors, whatever! Just send that text, and you're good to go!
<br>Bytes have dedicated methods for convenience.

## Examples 

```rust
use human_repr::HumanRepr;

// counts (bytes or anything)
assert_eq!("43.2 MB", 43214321_u32.human_count_bytes());
assert_eq!("123.5 kPackets", 123456_u64.human_count("Packets"));

// durations
assert_eq!("15.6 µs", 0.0000156.human_duration());
assert_eq!("10 ms", 0.01.human_duration());
assert_eq!("1:14:48", 4488.395.human_duration());

// throughputs (bytes or anything)
assert_eq!("1.2 MB/s", (1234567. / 1.).human_throughput_bytes());
assert_eq!("6.1 tests/m", (10. / 99.).human_throughput("tests"));
assert_eq!("9 errors/d", (125. / 1200000.).human_throughput("errors"));
```

## How to use it

Add this dependency to your Cargo.toml file:

```toml
human-repr = "0"
```

Use the trait:

```rust, no_run
use human_repr::HumanRepr;
```

That's it! You can now call on any number:

```rust, no_run
# use human_repr::HumanRepr;
# let num = 123;
num.human_count("unit");
num.human_count_bytes();

num.human_duration();

num.human_throughput("unit");
num.human_throughput_bytes();
```

## Rust features:

According to the SI standard, there are 1000 bytes in a `kilobyte`.
<br>There is another standard called IEC that has 1024 bytes in a `kibibyte`, but this is only useful when measuring things that are naturally a power of two, e.g. a stick of RAM.

Be careful to not render IEC quantities with SI units, which would be incorrect. But I still support it, if you really want to ;)

By default, `human-repr` will use SI with `1000` quantities, with the prefixes: `k`, `M`, `G`, `T`, `P`, `E`, `Z`, and `Y`.
<br>You can modify this by:
- `iec` => enable to use IEC prefixes: `Ki`, `Mi`, `Gi`, `Ti`, `Pi`, `Ei`, `Zi`, `Yi` (implies `1024`)
- `1024` => enable to use `1024` quantities only, with prefixes: `K`, `M`, `G`, `T`, `P`, `E`, `Z`, and `Y` (note the upper 'K')
- `nospace` => enable to remove the space between values and units everywhere: `15.6µs` instead of `15.6 µs`

## Changelog highlights
- 0.3.x Jun 01, 2022: support for a new group of prefixes for `1024` only (without `iec`)
- 0.2.x Jun 01, 2022: more flexible API (`impl AsRef<str>`), greatly improved documentation
- 0.1.x Jun 01, 2022: first release, include readme, method and module docs, describe features already implemented


## License
This software is licensed under the MIT License. See the LICENSE file in the top distribution directory for the full license text.


---
Maintaining an open source project is hard and time-consuming, and I've put much ❤️ and effort into this.

If you've appreciated my work, you can back me up with a donation! Thank you 😊

[<img align="right" src="https://cdn.buymeacoffee.com/buttons/default-orange.png" width="217px" height="51x">](https://www.buymeacoffee.com/rsalmei)
[<img align="right" alt="Donate with PayPal button" src="https://www.paypalobjects.com/en_US/i/btn/btn_donate_LG.gif">](https://www.paypal.com/donate?business=6SWSHEB5ZNS5N&no_recurring=0&item_name=I%27m+the+author+of+alive-progress%2C+clearly+and+about-time.+Thank+you+for+appreciating+my+work%21&currency_code=USD)

---
