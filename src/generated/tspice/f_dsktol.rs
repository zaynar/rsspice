//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const XFRACT: f64 = 0.0000000001;
const KEYXFR: i32 = 1;
const SGREED: f64 = 0.00000001;
const KEYSGR: i32 = (KEYXFR + 1);
const SGPADM: f64 = 0.0000000001;
const KEYSPM: i32 = (KEYSGR + 1);
const PTMEMM: f64 = 0.0000001;
const KEYPTM: i32 = (KEYSPM + 1);
const ANGMRG: f64 = 0.000000000001;
const KEYAMG: i32 = (KEYPTM + 1);
const LONALI: f64 = 0.000000000001;
const KEYLAL: i32 = (KEYAMG + 1);

//$Procedure F_DSKTOL ( DSKTOL tests )
pub fn F_DSKTOL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DPVAL: f64 = 0.0;
    let mut NEWVAL: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved values
    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_DSKTOL", ctx)?;

    //***********************************************************************
    //
    //     DSKGTL tests
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DSKGTL test: get expansion fraction", ctx)?;

    spicelib::DSKGTL(KEYXFR, &mut DPVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact equality, since we're comparing against
    // a value declared in an include file common to this
    // test family and the DSKTOL package.
    //
    testutil::CHCKSD(b"XFRACT", DPVAL, b"=", XFRACT, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DSKGTL test: get greedy segment parameter", ctx)?;

    spicelib::DSKGTL(KEYSGR, &mut DPVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact equality, since we're comparing against
    // a value declared in an include file common to this
    // test family and the DSKTOL package.
    //
    testutil::CHCKSD(b"SGREED", DPVAL, b"=", SGREED, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DSKGTL test: get segment pad margin", ctx)?;

    spicelib::DSKGTL(KEYSPM, &mut DPVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact equality, since we're comparing against
    // a value declared in an include file common to this
    // test family and the DSKTOL package.
    //
    testutil::CHCKSD(b"SGPADM", DPVAL, b"=", SGPADM, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DSKGTL test: get surface-point membership margin", ctx)?;

    spicelib::DSKGTL(KEYPTM, &mut DPVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact equality, since we're comparing against
    // a value declared in an include file common to this
    // test family and the DSKTOL package.
    //
    testutil::CHCKSD(b"PTMEMM", DPVAL, b"=", PTMEMM, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DSKGTL test: get angular rounding margin", ctx)?;

    spicelib::DSKGTL(KEYAMG, &mut DPVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact equality, since we're comparing against
    // a value declared in an include file common to this
    // test family and the DSKTOL package.
    //
    testutil::CHCKSD(b"ANGMRG", DPVAL, b"=", ANGMRG, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DSKGTL test: get longitude alias margin", ctx)?;

    spicelib::DSKGTL(KEYLAL, &mut DPVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact equality, since we're comparing against
    // a value declared in an include file common to this
    // test family and the DSKTOL package.
    //
    testutil::CHCKSD(b"LONALI", DPVAL, b"=", LONALI, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DSKGTL test: keyword out of range", ctx)?;

    spicelib::DSKGTL(0, &mut DPVAL, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    spicelib::DSKGTL(7, &mut DPVAL, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    //***********************************************************************
    //
    //     DSKSTL tests
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DSKSTL test: set expansion fraction", ctx)?;

    NEWVAL = 0.001;

    spicelib::DSKSTL(KEYXFR, NEWVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKGTL(KEYXFR, &mut DPVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"XFRACT", DPVAL, b"=", NEWVAL, 0.0, OK, ctx)?;

    //
    // Restore default.
    //
    spicelib::DSKSTL(KEYXFR, XFRACT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DSKSTL test: set greedy parameter", ctx)?;

    NEWVAL = 0.01;

    spicelib::DSKSTL(KEYSGR, NEWVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKGTL(KEYSGR, &mut DPVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"SGREED", DPVAL, b"=", NEWVAL, 0.0, OK, ctx)?;

    //
    // Restore default.
    //
    spicelib::DSKSTL(KEYSGR, SGREED, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DSKSTL test: set pad margin", ctx)?;

    NEWVAL = 0.1;

    spicelib::DSKSTL(KEYSPM, NEWVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKGTL(KEYSPM, &mut DPVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"SGPADM", DPVAL, b"=", NEWVAL, 0.0, OK, ctx)?;

    //
    // Restore default.
    //
    spicelib::DSKSTL(KEYSPM, SGPADM, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DSKSTL test: set plate membership margin", ctx)?;

    NEWVAL = 0.001;

    spicelib::DSKSTL(KEYPTM, NEWVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKGTL(KEYPTM, &mut DPVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"PTMEMM", DPVAL, b"=", NEWVAL, 0.0, OK, ctx)?;

    //
    // Restore default.
    //
    spicelib::DSKSTL(KEYPTM, PTMEMM, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DSKSTL test: try to set angular rounding margin", ctx)?;

    //
    // This parameter is not adjustable.
    //
    NEWVAL = 0.001;

    spicelib::DSKSTL(KEYAMG, NEWVAL, ctx)?;
    testutil::CHCKXC(true, b"SPICE(IMMUTABLEVALUE)", OK, ctx)?;

    //
    // Make sure no change occurred.
    //
    spicelib::DSKGTL(KEYAMG, &mut DPVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact equality, since we're comparing against
    // a value declared in an include file common to this
    // test family and the DSKTOL package.
    //
    testutil::CHCKSD(b"ANGMRG", DPVAL, b"=", ANGMRG, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DSKSTL test: try to set longitude alias margin", ctx)?;

    //
    // This parameter is not adjustable.
    //
    NEWVAL = 0.001;

    spicelib::DSKSTL(KEYLAL, NEWVAL, ctx)?;
    testutil::CHCKXC(true, b"SPICE(IMMUTABLEVALUE)", OK, ctx)?;

    //
    // Make sure no change occurred.
    //
    spicelib::DSKGTL(KEYLAL, &mut DPVAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact equality, since we're comparing against
    // a value declared in an include file common to this
    // test family and the DSKTOL package.
    //
    testutil::CHCKSD(b"LONALI", DPVAL, b"=", LONALI, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"DSKSTL test: keyword out of range", ctx)?;

    spicelib::DSKSTL(0, DPVAL, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    spicelib::DSKSTL(7, DPVAL, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
