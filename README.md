# human-repr
### Generate beautiful human representations of bytes, durations and even throughputs!

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/human_repr.svg)](https://crates.io/crates/human-repr)
[![Docs](https://docs.rs/human-repr/badge.svg)](https://docs.rs/human-repr)

## What it does

Easily generate several kinds of human-readable descriptions, directly on primitive numbers or [`Durations`](`std::time::Duration`):

```rust
use human_repr::HumanRepr;

// counts (bytes or any other unit)
assert_eq!("43.2 MB", 43214321_u32.human_count_bytes());
assert_eq!("123.5 kPackets", 123456_u64.human_count("Packets"));

// primitive durations
assert_eq!("15.6 µs", 0.0000156.human_duration());
assert_eq!("10 ms", 0.01.human_duration());
assert_eq!("3.44 s", 3.435999.human_duration());
assert_eq!("19:20.4", 1160.36.human_duration());
assert_eq!("1:14:48", 4488.395.human_duration());

// throughputs (bytes or any other unit)
// the divisions below are just for the sake of clarity, 
// they show the very concept of a "throughput": the number of items per amount of time.
assert_eq!("1.2 MB/s", (1234567. / 1.).human_throughput_bytes());
assert_eq!("6.1 tests/m", (8. / 79.).human_throughput("tests").to_string());
assert_eq!("9 errors/d", (125. / 1200000.).human_throughput("errors"));
```

```rust
use human_repr::HumanReprDuration;
use std::time::Duration;

assert_eq!("15.6 µs", Duration::new(0, 15_600).human_duration());
assert_eq!("10 ms", Duration::from_secs_f64(0.01).human_duration());
assert_eq!("1:14:48", Duration::new(4488, 395_000_000).human_duration());
```

This lib implements a whole suite of:
- counts, supporting SI prefixes `k`, `M`, `G`, `T`, `P`, `E`, `Z`, and `Y` (optional IEC and "mixed" ones, see Rust features);
- durations, supporting nanos (`ns`), millis (`ms`), micros (`µs`), seconds (`s`), minutes (`M:SS`), and even hours (`H:MM:SS`);
- throughputs, supporting per day (`/d`), per hour (`/h`), per minute (`/m`), and per second (`/s`).

It does not use any dependencies, is well-tested, and is blazingly fast, taking only ~50 ns to generate a representation! (criterion benchmarks inside)
<br>Since version 0.4, it does not allocate any Strings anymore! I've returned structs that implement [`Display`](`std::fmt::Display`), so you can print them with no heap allocations at all! And if you do need the String, a simple `.to_string()` will do.

They work on all Rust primitive number types: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `f32`,
`f64`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.
<br>Since version 0.7, [`Duration`](`std::time::Duration`) is also supported! Yes yes, I know it does have a [`Debug`](`std::fmt::Debug`) impl that does almost this, but it is not very human: `Duration::new(0, 14184293)` comes out as `14.184293ms`, this crate would return `14.2 ms`. And of course, the minutes and hours views... `Duration::new(1000000, 1)` gives the horrendous `1000000.000000001s`, instead of `277:46:40` 👍

The `unit` parameter some methods refer to means the entity you're dealing with, like bytes, actions, iterations, errors, whatever! Just send that text, and you're good to go!
<br>Bytes have dedicated methods for convenience.

## How to use it

Add this dependency to your Cargo.toml file:

```toml
human-repr = "0"
```

Then just use the main trait and that's it! You can now call on any number:

```rust
use human_repr::HumanRepr;

3000_u16.human_count("bytes");
-5i8.human_count_bytes();

4244.32_f32.human_duration();
0.000000000004432_f64.human_duration();

8987_isize.human_throughput("transactions");
93321_usize.human_throughput_bytes();
```

For durations, use the specific trait:

```rust
use human_repr::HumanReprDuration;

std::time::Duration::from_secs_f64(0.00432).human_duration();
```

## Rust features:

According to the SI standard, there are 1000 bytes in a `kilobyte`.
<br>There is another standard called IEC that has 1024 bytes in a `kibibyte`, but this is only useful when measuring things that are naturally a power of two, e.g. a stick of RAM.

Be careful to not render IEC quantities with SI scaling, which would be incorrect. But I still support it, if you really want to ;)

By default, `human-repr` will use SI with `1000` divisor, and the prefixes: `k`, `M`, `G`, `T`, `P`, `E`, `Z`, and `Y`.
<br>You can modify this by enabling optional features:
- `iec` => use IEC instead of SI: `Ki`, `Mi`, `Gi`, `Ti`, `Pi`, `Ei`, `Zi`, `Yi` (implies `1024`)
- `1024` => use `1024` divisor, but if `iec` is not enabled, use prefixes: `K`, `M`, `G`, `T`, `P`, `E`, `Z`, and `Y` (note the upper 'K')
- `nospace` => remove the space between values and scales/units everywhere: `48GB` instead of `48 GB`, `15.6µs` instead of `15.6 µs`, and `12.4kB/s` instead of `12.4 kB/s`

## The human duration magic

I've used just one key concept in designing the human duration features: cleanliness.
> `3.44 s` is more meaningful than `3.43584783784 s`, and `14.1 µs` is much, much nicer than `.0000141233333 s`.

So what I do is: round values to at most two decimal places (larger scales have more decimals), and find the best scale to represent them, minimizing resulting values smaller than `1`. The search for the best scale considers even the rounding been applied!
> `0.000999999` does not end up as `999.9 µs` (truncate) nor `1000.0 µs` (bad scale), it is auto-upgraded to the next one `1.0 ms`!

The human duration scale changes seamlessly from nanoseconds to hours!
  - values smaller than 60 seconds are always rendered as `D.D[D] scale`, with one or two decimals;
  - `.0` and `.00` are efficiently not generated instead of removed from the output -> it is handled directly in the format arguments;
  - from 1 minute onward it changes to "M:SS";
  - from 1 hour onward it changes to "H:MM:SS".

## The human throughput magic

I've made the human throughput with a similar logic. It is funny how much trickier "throughput" is to the human brain!
> If something took `1165263` seconds to handle `123` items, how fast did it go? It's not obvious...

It doesn't help much even if we divide the duration by the number of items: `9473` seconds/item still does not seem that good. How fast was that? We can't say for sure.
<br>Humm, how many items did we do per time?
> Oh, we just need to invert it, so `0.000105555569858` items/second, there it is! 😂

To make some sense of it we now need to multiply that by 3600 (seconds in an hour) to get `0.38` per hour, which is much better, and again by 24 (hours in a day) to finally get `9.12` per day!! Now we know how fast that process was! \o/
<br>As you see, it's not easy at all for our brains to estimate that...

The human throughput scale changes seamlessly from per second to per day!
  - `.0` and `.00` are efficiently not generated too, much like the duration magic;
  - it also automatically inserts SI prefixes when in the fastest scale (per second), so we get `2.4 MB/s` or `6.42 Gitems/s` 👍

## The human count magic

Oh, this is the simplest of them all! I just continually divide by the divisor (1000 or 1024), until the value gets smaller than it. No funny business like logs or exponentials at all.

Rounding is also handled so there's no truncation or bad scale, the number of decimals also increase the larger the scale gets, and `.0` and `.00` are also never generated.

## Changelog highlights
- 0.7.x Jun 04, 2022: support for std::time::Duration via a new trait `HumanReprDuration`, include one decimal in the minutes representation
- 0.6.x Jun 04, 2022: improve signed support with new `ops::Neg` impl
- 0.5.x Jun 03, 2022: new minutes representation M:SS, between seconds and complete H:MM:SS
- 0.4.x Jun 03, 2022: even faster implementation, which does not do any String allocations
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
