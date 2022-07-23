# human-repr
### Generate beautiful human representations of bytes, durations and even throughputs!

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/human_repr.svg)](https://crates.io/crates/human-repr)
[![Docs](https://docs.rs/human-repr/badge.svg)](https://docs.rs/human-repr)
[![dependency status](https://deps.rs/repo/github/rsalmei/human-repr/status.svg)](https://deps.rs/repo/github/rsalmei/human-repr)

Easily generate several kinds of human-readable descriptions, directly on primitive numbers and [`Duration`](`std::time::Duration`)s!

```rust
use human_repr::HumanRepr;

// counts (bare, bytes, or any custom unit)
assert_eq!("43.21GB", 43214321123_u64.human_count_bytes());
assert_eq!("123.5kPackets", 123456_u32.human_count("Packets"));
assert_eq!("74.9M", 74893200.human_count_bare());
assert_eq!("48.1°C", 48.132323432.human_count("°C"));

// primitive durations
assert_eq!("15.6µs", 0.0000156.human_duration());
assert_eq!("10ms", 0.01.human_duration());
assert_eq!("3.44s", 3.435999.human_duration());
assert_eq!("19:20.4", 1160.36.human_duration());
assert_eq!("1:14:48", 4488.395.human_duration());

// throughputs (bare, bytes, or any custom unit)
assert_eq!("1.2MB/s", (1234567. / 1.).human_throughput_bytes());
assert_eq!("6.1tests/m", (8. / 79.).human_throughput("tests"));
assert_eq!("9/d", (125. / 1200000.).human_throughput_bare());
assert_eq!("54°C/h", (24. / 1600.).human_throughput("°C"));
// the divisions above are just for the sake of clarity: they show 
// the very concept of a "throughput": number of items per time.

use human_repr::HumanReprDuration;
# use std::time::Duration;

assert_eq!("15.6µs", Duration::new(0, 15_600).human_duration());
assert_eq!("10ms", Duration::from_secs_f64(0.01).human_duration());
assert_eq!("1:14:48", Duration::new(4488, 395_000_000).human_duration());
```

This crate implements a whole suite of:
- counts, supporting SI prefixes `k`, `M`, `G`, `T`, `P`, `E`, `Z`, and `Y` (including optional IEC prefixes and "mixed" ones, see Rust features);
- durations, supporting SI prefixes nanos (`ns`), micros (`µs`), millis (`ms`), and seconds (`s`), in addition to custom minutes:seconds (`M:SS.m`) and even hours:minutes:seconds (`H:MM:SS`);
- throughputs, supporting per day (`/d`), per hour (`/h`), per minute (`/m`), and per second (`/s`) -> per second even gets SI prefixes too.

This crate doesn't have any dependencies, is well-tested, and is blazing fast, taking only ~50 ns to generate a representation! Checked with criterion benchmarks.

> Since version 0.11, the [`PartialEq`](`std::cmp::PartialEq`) impls for `&str` do not allocate any Strings too!
> <br>I've developed a particularly interesting [`Write`](`std::fmt::Write`) impl, which compares partial sequences with what the [`Display`](`std::fmt::Display`) impl would be generating!

> Since version 0.10, the [`Debug`](`std::fmt::Debug`) impl will show both the raw value and the final representation! Very, very cool:
> ```rust
> # use human_repr::HumanRepr;
> assert_eq!("HumanDuration { val: 1.56e-5 } -> 15.6µs", format!("{:?}", 0.0000156.human_duration()));
> assert_eq!(r#"HumanThroughput { val: 0.015, unit: "°C" } -> 54°C/h"#, format!("{:?}", 0.015.human_throughput("°C")));
> ```

> Since version 0.4, I do not allocate any Strings to generate the output! I've returned structs that implement [`Display`](`std::fmt::Display`), so you can print them with no heap allocations at all! And if you do need the String, a simple `.to_string()` will do.

They work on all Rust primitive number types: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `f32`,
`f64`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, as well as [`Duration`](`std::time::Duration`) types.

> Yes, I know `Duration`s do have a [`Debug`](`std::fmt::Debug`) impl that does something like this, but it is not very _human_:
> ```rust
> # use human_repr::HumanReprDuration;
> # use std::time::Duration;
> let default = format!("{:?}", Duration::new(0, 14184293));
> assert_eq!("14.184293ms", default); // 😫👎
> assert_eq!("14.2ms", Duration::new(0, 14184293).human_duration()); // 😃👍
> ```
> 
> And of course, I have the minutes and hours views...
> ```rust
> # use human_repr::HumanReprDuration;
> # use std::time::Duration;
> let default = format!("{:?}", Duration::new(10000, 1));
> assert_eq!("10000.000000001s", default); // 😫👎
> assert_eq!("2:46:40", Duration::new(10000, 1).human_duration()); // 😃👍
> ```

The `unit` parameter some methods make available means the entity you're dealing with, like "bytes", "actions", "iterations", "errors", whatever! You can send either the whole unit name, or a shortened one like "B", "it", etc, or even an empty str!
<br>Bytes and bare units have dedicated methods for convenience.

## How to use it

Add this dependency to your Cargo.toml file:

```toml
human-repr = "0"
```

Then just `use` the main trait!
<br>You can now call on any number:

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
# use std::time::Duration;
Duration::from_secs_f64(0.00432).human_duration();
```

## Rust features:

According to the SI standard, there are 1000 bytes in a `kilobyte`.
<br>There is another standard called IEC that has 1024 bytes in a `kibibyte`, but this is only useful when measuring things that are naturally a power of two, e.g. a stick of RAM.

Be careful to not render IEC quantities with SI scaling, which would be incorrect. But I still support it, if you really want to ;)

By default, `human-repr` will have no features enabled, i.e. it will use SI prefixes, `1000` divisor, and no space between values and scales/units.

This crate supports these optional features:
- `space` => include a space between values and scales/units everywhere: `48 B` instead of `48B`, `15.6 µs` instead of `15.6µs`, and `12.4 kB/s` instead of `12.4kB/s`;
- `1024` => use `1024` divisor — if `iec` is not enabled, use prefixes: `K`, `M`, `G`, `T`, `P`, `E`, `Z`, and `Y` (note the upper `'K'`);
- `iec` => use IEC instead of SI: `Ki`, `Mi`, `Gi`, `Ti`, `Pi`, `Ei`, `Zi`, `Yi` (implies `1024`).

## The human duration magic

I've used just one key concept in designing the human duration features: cleanliness.
> `3.44s` is more meaningful than `3.43584783784s`, and `14.1µs` is much, much nicer than `.0000141233333s`.

So what I do is: round values to at most two decimal places (larger scales have more decimals), and find the best scale to represent them, minimizing resulting values smaller than `1`. The search for the best scale considers even the rounding been applied!
> `0.000999999` does not end up as `999.9µs` (truncate) nor `1000µs` (bad scale), it is auto-upgraded to the next one `1ms`!

The human duration scale changes seamlessly from nanoseconds to hours!
  - values smaller than 60 seconds are rendered as `D[.D]scale`, with up to two decimals;
  - `.0` and `.00` are efficiently not generated instead of removed from the output -> it is handled directly in the format arguments;
  - from 1 minute onward it changes to `M:SS[.m]`;
  - from 1 hour onward it changes to `H:MM:SS`.

## The human throughput magic

I've made the human throughput with a similar logic. It is funny how much trickier "throughput" is to the human brain!
> If something took `1165263` seconds to handle `123` items, how fast did it go? It's not obvious...

It doesn't help much even if we divide the duration by the number of items: 9473 seconds/item still does not seem that good. How fast was that? We can't say for sure.
<br>Humm, how many items did we do per time?
> Oh, we just need to invert it, so 0.000105555569858 items/second, there it is! 😂

To make some sense of it we now need to multiply that by 3600 (seconds in an hour) to get 0.38 per hour, which is much better, and again by 24 (hours in a day) to finally get 9.12 per day!! Now we know how fast that process was! \o/
<br>As you see, it's not easy at all for our brains to estimate it...

The human throughput scale changes seamlessly from per second to per day!
  - `.0` and `.00` are efficiently not generated too, much like the duration magic;
  - it also automatically inserts SI prefixes when in the fastest scale (per second), so we get `2.4MB/s` or `6.42Gitems/s` 👍

## The human count magic

This is the simplest of them all, I just continually divide by the current divisor (1000 or 1024), until the value gets smaller than it. No funny business like logs or exponentials at all.

Rounding is also handled so there's no truncation or bad scale, the number of decimals also increase the larger the scale gets, and `.0` and `.00` are also never generated.

## Changelog highlights
- 0.11.x Jul 22, 2022: new PartialEq impls for `&str`, which is even faster and does not allocate any Strings
- 0.10.x Jul 17, 2022: new Debug impl with raw and rendered values, new "bare unit" method variations, remove `space` from default features
- 0.9.x Jun 22, 2022: do not use captured identifiers in format strings, to support much broader Rust versions instead of only >= 1.58
- 0.8.x Jun 12, 2022: change `nospace` feature to `space`, to avoid the negative logic (it is now default, to maintain behavior)
- 0.7.x Jun 04, 2022: support for std::time::Duration via a new trait `HumanReprDuration`, include one decimal in the minutes representation
- 0.6.x Jun 04, 2022: improve signed support with new `ops::Neg` impl
- 0.5.x Jun 03, 2022: new minutes representation M:SS, between seconds and complete H:MM:SS
- 0.4.x Jun 03, 2022: new render engine via Display, which is even faster and does not allocate any Strings
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
