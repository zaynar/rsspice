//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const WDSIZE: i32 = 32;
const LWSIZE: i32 = 40;

//$Procedure      F_SPKGXC (Family of tests for SPKGEO frame exceptions)
pub fn F_SPKGXC(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut REF = [b' '; WDSIZE as usize];
    let mut ET: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut STATE = StackArray::<f64, 6>::new(1..=6);

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_SPKGXC", ctx)?;

    ET = 0.0;

    testutil::TCASE(
        b"Check that we get reasonable unrecognized frame diagnostics subcase 1.",
        ctx,
    )?;

    fstr::assign(&mut REF, b"TRANSGALCTIC");
    spicelib::SPKGEO(-10, ET, &REF, 399, STATE.as_slice_mut(), &mut LT, ctx)?;
    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    testutil::TCASE(
        b"Check that we get reasonable unrecognized frame diagnostics subcase 2.",
        ctx,
    )?;

    fstr::assign(&mut REF, b" ");
    spicelib::SPKGEO(-10, ET, &REF, 399, STATE.as_slice_mut(), &mut LT, ctx)?;
    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    testutil::TCASE(
        b"Check that we get reasonable unrecognized frame diagnostics subcase 3.",
        ctx,
    )?;

    fstr::assign(
        &mut REF,
        &fstr::concat(&fstr::concat(b"MY", &intrinsics::CHAR(9)), b"FRAME"),
    );
    spicelib::SPKGEO(-10, ET, &REF, 399, STATE.as_slice_mut(), &mut LT, ctx)?;
    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
