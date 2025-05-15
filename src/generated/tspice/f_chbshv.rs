//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.000000000001;
const MAXDEG: i32 = 15;

//$Procedure F_CHBSHV ( Chebyshev expansion tests )
pub fn F_CHBSHV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CP = StackArray::<f64, 15>::new(1..=MAXDEG);
    let mut DPDX: f64 = 0.0;
    let mut DPDXS = StackArray::<f64, 16>::new(0..=MAXDEG);
    let mut P: f64 = 0.0;
    let mut PARTDP = StackArray2D::<f64, 48>::new(1..=3, 0..=MAXDEG);
    let mut X: f64 = 0.0;
    let mut X2S = StackArray::<f64, 2>::new(1..=2);
    let mut DEGP: i32 = 0;
    let mut NDERIV: i32 = 0;

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
    // Open the test family.
    //
    testutil::TOPEN(b"F_CHBSHV", ctx)?;

    //
    // *****************************************************************
    //
    // Error cases
    //
    // *****************************************************************
    //
    //
    // None of the routines participate in error handling.
    //
    //
    // *****************************************************************
    //
    // Normal cases
    //
    // *****************************************************************
    //
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CHBDER normal case #1: find value of polynomial and its first 3 derivatives at X=1",
        ctx,
    )?;

    CP[1] = 1.0;
    CP[2] = 3.0;
    CP[3] = 0.5;
    CP[4] = 1.0;
    CP[5] = 0.5;
    CP[6] = -1.0;
    CP[7] = 1.0;

    X2S[1] = 0.5;
    X2S[2] = 3.0;

    DEGP = 6;
    NDERIV = 3;
    X = 1.0;

    spicelib::CHBDER(
        CP.as_slice(),
        DEGP,
        X2S.as_slice(),
        X,
        NDERIV,
        PARTDP.as_slice_mut(),
        DPDXS.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"f(1)", DPDXS[0], b"~/", -0.34087791495199, TIGHT, OK, ctx)?;
    testutil::CHCKSD(
        b"df\'(1)",
        DPDXS[1],
        b"~/",
        0.38271604938272,
        TIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"df\"(1)", DPDXS[2], b"~/", 4.2880658436214, TIGHT, OK, ctx)?;
    testutil::CHCKSD(
        b"df\'\"(1)",
        DPDXS[3],
        b"~/",
        -1.5144032921811,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CHBINT normal case #1: find value of polynomial and its first derivative at X=1",
        ctx,
    )?;

    CP[1] = 1.0;
    CP[2] = 3.0;
    CP[3] = 0.5;
    CP[4] = 1.0;
    CP[5] = 0.5;
    CP[6] = -1.0;
    CP[7] = 1.0;

    X2S[1] = 0.5;
    X2S[2] = 3.0;

    DEGP = 6;
    X = 1.0;

    spicelib::CHBINT(CP.as_slice(), DEGP, X2S.as_slice(), X, &mut P, &mut DPDX);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"f(1)", P, b"~/", -0.34087791495199, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df\'(1)", DPDX, b"~/", 0.38271604938272, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CHBVAL normal case #1: find value of polynomial at X=1",
        ctx,
    )?;

    CP[1] = 1.0;
    CP[2] = 3.0;
    CP[3] = 0.5;
    CP[4] = 1.0;
    CP[5] = 0.5;
    CP[6] = -1.0;
    CP[7] = 1.0;

    X2S[1] = 0.5;
    X2S[2] = 3.0;

    DEGP = 6;
    X = 1.0;

    spicelib::CHBVAL(CP.as_slice(), DEGP, X2S.as_slice(), X, &mut P);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"f(1)", P, b"~/", -0.34087791495199, TIGHT, OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
