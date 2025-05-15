[crates.io](https://crates.io/crates/rsspice) |
[docs.rs](https://docs.rs/rsspice/) |
[GitHub](https://github.com/zaynar/rsspice)

# `rsspice`

Pure Rust port of the SPICE Toolkit for space geometry.

This implementation is fully memory-safe and thread-safe,
and does not depend on any external C/FORTRAN libraries.
It provides nearly the entire SPICELIB API.
The code has been mechanically [translated](https://github.com/zaynar/f2rust)
from the FORTRAN version of the SPICE Toolkit into Rust.

It is completely unofficial, unsupported, and not heavily tested
(though it does pass the Toolkit's regression tests so it's probably
not too bad). Use at your own risk.

Users of this library should not expect any support from NAIF.
Use the [GitHub project](https://github.com/zaynar/rsspice)
for any issues or queries.

# Usage example

```rust
use rsspice::*;

const TIMFMT: &str = "YYYY MON DD HR:MN:SC.###### (TDB)::TDB";
const MAXWIN: usize = 2 * 100;

// Find solar eclipses as seen from the center of the Earth.
fn main() -> Result<()> {
    let mut spice = SpiceContext::new();

    spice.furnsh("gfoclt_ex1.tm")?;

    let mut confine = Cell::with_capacity(2);
    let mut result = Cell::with_capacity(MAXWIN);

    let et0 = spice.str2et("2027 JAN 01 00:00:00 TDB")?;
    let et1 = spice.str2et("2029 JAN 01 00:00:00 TDB")?;

    spice.wninsd(et0, et1, &mut confine)?;

    spice.gfoclt(
        "ANY",
        "MOON", "ellipsoid", "IAU_MOON",
        "SUN", "ellipsoid", "IAU_SUN",
        "LT", "EARTH", 180.0, &confine,
        &mut result,
    )?;

    for i in 1..=spice.wncard(&result)? {
        let (left, right) = spice.wnfetd(&result, i)?;
        println!(
            "Interval {i}: {} - {}",
            spice.timout(left, TIMFMT)?,
            spice.timout(right, TIMFMT)?
        );
    }

    Ok(())
}
```

See more [examples](https://github.com/zaynar/rsspice/tree/main/examples)
and [lessons](https://github.com/zaynar/rsspice/tree/main/lessons).

For more details, see the [documentation](https://docs.rs/rsspice/).
