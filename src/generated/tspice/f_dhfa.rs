//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.000000000001;
const RBODY: f64 = 1000.0;

struct SaveVars {
    ZERO6: StackArray<f64, 6>,
    ONE6: StackArray<f64, 6>,
    VAL: f64,
    ADOT: f64,
    R: f64,
    BODYR: f64,
    STATE: StackArray<f64, 6>,
    SEED: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ZERO6 = StackArray::<f64, 6>::new(1..=6);
        let mut ONE6 = StackArray::<f64, 6>::new(1..=6);
        let mut VAL: f64 = 0.0;
        let mut ADOT: f64 = 0.0;
        let mut R: f64 = 0.0;
        let mut BODYR: f64 = 0.0;
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut SEED: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
            ]
            .into_iter();
            ZERO6
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
            ]
            .into_iter();
            ONE6.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ZERO6,
            ONE6,
            VAL,
            ADOT,
            R,
            BODYR,
            STATE,
            SEED,
        }
    }
}

//$Procedure F_DHFA ( DHFA, DVSEP family tests )
pub fn F_DHFA(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_DHFA", ctx)?;

    //
    // Case 1
    //
    testutil::TCASE(b"Non-positive body radius.", ctx)?;

    save.SEED = -1804030;
    save.BODYR = testutil::T_RANDD(-199999999.0, 0.0, &mut save.SEED, ctx)?;

    save.VAL = spicelib::DHFA(save.ONE6.as_slice(), save.BODYR, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADRADIUS)", OK, ctx)?;

    //
    // Case 2
    //
    testutil::TCASE(b"Position vector equals zero vector.", ctx)?;

    save.VAL = spicelib::DHFA(save.ZERO6.as_slice(), RBODY, ctx)?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // Case 3
    //
    testutil::TCASE(b"Range less-than body radius.", ctx)?;

    save.VAL = spicelib::DHFA(save.ONE6.as_slice(), RBODY, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADGEOMETRY)", OK, ctx)?;

    //
    // Case 4
    //
    // Circular orbits; constant half angle. Physically
    // realistic orbits not required, only geometry with
    // -   -
    // R . V = 0
    //
    testutil::TCASE(b"DHFA Circular orbits.", ctx)?;

    //     .
    // x * x = 1
    //     .
    // y * y = -1/2
    //     .
    // z * z = -1/2
    //
    save.STATE[1] = 100000000.0;
    save.STATE[2] = 1100000000000.0;
    save.STATE[3] = -23000000000.0;
    save.STATE[4] = 0.00000001;
    save.STATE[5] = -(0.5 * 0.0000000000011);
    save.STATE[6] = (0.5 * 23000000000.0);

    save.VAL = spicelib::DHFA(save.STATE.as_slice(), RBODY, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"DHFA Circular", save.VAL, b"~", 0.0, TIGHT, OK, ctx)?;

    //
    // Case 5
    //
    // Linear orbits.
    // Physically realistic orbits not required, only geometry with
    // -   -
    // R . V = 1
    //
    testutil::TCASE(b"DHFA Linear orbits.", ctx)?;

    //     .
    // x * x = 1/3
    //     .
    // y * y = 1/3
    //     .
    // z * z = 1/3
    //
    save.STATE[1] = 10000.0;
    save.STATE[2] = 110000000.0;
    save.STATE[3] = -2300000.0;
    save.STATE[4] = (((1.0 / 3.0) * 1.0) - 4 as f64);
    save.STATE[5] = (((1.0 / 3.0) * 1.1) - 8 as f64);
    save.STATE[6] = (-((1.0 / 3.0) * 2.3) - 6 as f64);

    //
    // Reduce the function for the half angle derivative given
    // -   -
    // R . V = 1
    //
    // to the form
    //
    // d              - body_radius                 1
    // --(ALPHA) =  --------------------- *      ------
    // dt                  2             2  1/2       2
    //               (range - body_radius )      range
    //
    save.R = spicelib::VNORM(save.STATE.subarray(1));
    save.ADOT = -(RBODY / f64::sqrt((f64::powi(save.R, 2) - f64::powi(RBODY, 2))));
    save.ADOT = (save.ADOT / f64::powi(save.R, 2));

    save.VAL = spicelib::DHFA(save.STATE.as_slice(), RBODY, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"DHFA Linear", save.VAL, b"~", save.ADOT, TIGHT, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
