# human-repr
### Generate beautiful human representations of bytes, durations and even throughputs!

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/human_repr.svg)](https://crates.io/crates/human-repr)
[![Docs](https://docs.rs/human-repr/badge.svg)](https://docs.rs/human-repr)

## What it does

Easily generate human-readable descriptions directly on primitive numbers, of several kinds:
- counts, supporting SI prefixes `k`, `M`, `G`, `T`, `P`, `E`, `Z`, and `Y` (optional IEC and "mixed" ones, see Rust features);
- durations, supporting nanos (`ns`), millis (`ms`), micros (`µs`), seconds (`s`), minutes (`[M]M:SS`), and even hours (`[H]H:MM:SS`);
- throughputs, supporting per-day (`/d`), per-hour (`/h`), per-minute (`/m`), and per-second (`/s`).

It does not use any dependencies, is well-tested, and is blazingly fast, taking only ~50 ns to generate a representation! (criterion benchmarks inside)
<br>In the new version 0.4, it even does not allocate Strings anymore! I've returned structs that implement [`std::fmt::Display`], so you can now print them with no heap allocations at all! And if you do need the String, a simple `.to_string()` will do 😜

They work on all Rust primitive number types: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `f32`,
`f64`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.

The `unit` parameter some methods refer to means the entity you're dealing with, like bytes, actions, iterations, errors, whatever! Just send that text, and you're good to go!
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

Be careful to not render IEC quantities with SI scaling, which would be incorrect. But I still support it, if you really want to ;)

By default, `human-repr` will use SI with `1000` divisor, and the prefixes: `k`, `M`, `G`, `T`, `P`, `E`, `Z`, and `Y`.
<br>You can modify this by enabling optional features:
- `iec` => use IEC instead of SI: `Ki`, `Mi`, `Gi`, `Ti`, `Pi`, `Ei`, `Zi`, `Yi` (implies `1024`)
- `1024` => use `1024` divisor, but if `iec` is not enabled, use prefixes: `K`, `M`, `G`, `T`, `P`, `E`, `Z`, and `Y` (note the upper 'K')
- `nospace` => remove the space between values and scales/units everywhere: `48GB` instead of `48 GB`, `15.6µs` instead of `15.6 µs`, and `12.4 B/m` instead of `12.4 B/m`

## The human duration magic

I've used just one key concept in designing the human duration features: cleanliness.
> `3.44 s` is more meaningful than `3.43584783784 s`, and `14.1 µs` is much, much nicer than `.0000141233333 s`.

So what I do is: round values to at most two decimal places (larger scales have more decimals), and find the best scale to represent them, minimizing resulting values smaller than `1`. The search for the best scale considers even the rounding been applied!
> `0.000999999` does not end up as `999.9 µs` (truncate) nor `1000.0 µs` (bad scale), it is auto-upgraded to the next one `1.0 ms`!

The human duration scale changes seamlessly from nanoseconds to hours!
  - values smaller than 60 seconds are always rendered as `D.D[D] scale`, with one or two decimals;
  - `.0` and `.00` are efficiently not generated (no `trim` followed by `to_owned` for example), it is handled directly in the format arguments;
  - from 1 minute onward it changes to "H:MM:SS".

## The human throughput magic

I've made the human throughput with a similar logic. It is funny how much trickier "throughput" is to the human brain!
> If something took `1165263` seconds to handle `123` items, how fast did it go? It's not obvious...

It doesn't help much even if we divide the duration by the number of items: `9473` seconds/item still does not seem that good. How fast was that? We can't say for sure.
<br>Humm, how many items did we do per time?
> Oh, we just need to invert it, so `0.000105555569858` items/second, there it is! 😂

To make some sense of it we now need to multiply that by 3600 (seconds in an hour) to get `0.38` per hour, which is much better, and again by 24 (hours in a day) to finally get `9.12` per day!! Now we know how fast that process was! \o/
<br>As you see, it's not easy at all for our brains to estimate that...

The human throughput scale changes seamlessly from per-second to per-day!
  - `.0` and `.00` are efficiently not generated too, much like the duration magic;
  - it also automatically inserts SI prefixes when in the fastest scale /second, so we get `2.4 MB/s` or `6.42 Gitems/s` 👍

## The human count magic

Oh, this is the simplest of them all! I just continually divide by the divisor (1000 or 1024), until the value gets smaller than it. No funny business like logs or exponentials at all.

Rounding is also handled so there's no truncation or bad scale, the number of decimals also increase the larger the scale gets, and `.0` and `.00` are also never generated.

## Changelog highlights
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
