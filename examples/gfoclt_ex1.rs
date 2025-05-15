//! GFOCLT example 1
//!
//! Find occultations of the Sun by the Moon (that is, solar
//! eclipses) as seen from the center of the Earth over the month
//! December, 2001.
//!
//! Use light time corrections to model apparent positions of Sun
//! and Moon. Stellar aberration corrections are not specified
//! because they don't affect occultation computations.

use rsspice::*;

const TIMFMT: &str = "YYYY MON DD HR:MN:SC.###### (TDB)::TDB";
const MAXWIN: usize = 2 * 100;

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
        "MOON",
        "ellipsoid",
        "IAU_MOON",
        "SUN",
        "ellipsoid",
        "IAU_SUN",
        "LT",
        "EARTH",
        180.0,
        &confine,
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
