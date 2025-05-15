//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXDEG: i32 = 15;
const TIGHT: f64 = 0.00000000000001;

fn F(X: f64) -> f64 {
    ((((((1.0 + (3.0 * X)) + (0.5 * f64::powi(X, 2))) + f64::powi(X, 3))
        + (0.5 * f64::powi(X, 4)))
        - f64::powi(X, 5))
        + f64::powi(X, 6))
}

fn DF1(X: f64) -> f64 {
    (((((3.0 + X) + (3.0 * f64::powi(X, 2))) + (2.0 * f64::powi(X, 3))) - (5.0 * f64::powi(X, 4)))
        + (6.0 * f64::powi(X, 5)))
}

fn DF2(X: f64) -> f64 {
    ((((1.0 + (6.0 * X)) + (6.0 * f64::powi(X, 2))) - (20.0 * f64::powi(X, 3)))
        + (30.0 * f64::powi(X, 4)))
}

fn DF3(X: f64) -> f64 {
    (((6.0 + (12.0 * X)) - (60.0 * f64::powi(X, 2))) + (120.0 * f64::powi(X, 3)))
}

//$Procedure F_POLYDS ( POLYDS tests )
pub fn F_POLYDS(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut COEFFS = StackArray::<f64, 16>::new(0..=MAXDEG);
    let mut P = StackArray::<f64, 16>::new(0..=MAXDEG);
    let mut T: f64 = 0.0;
    let mut DEG: i32 = 0;
    let mut NDERIV: i32 = 0;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Function for (a particular) polynomial:
    //
    //                          2    3        4    5    6
    //    f(x) = 1 + 3*x + 0.5*x  + x  + 0.5*x  - x  + x
    //

    //
    // Function for (a particular) polynomial first derivative:
    //

    //
    // Function for (a particular) polynomial second derivative:
    //

    //
    // Function for (a particular) polynomial third derivative:
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_POLYDS", ctx)?;

    //
    // *****************************************************************
    //
    //    Normal cases: POLYDS
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // For the polynomial
    //
    //                          2    3        4    5    6
    //    f(x) = 1 + 3*x + 0.5*x  + x  + 0.5*x  - x  + x
    //
    // calculate the value of the polynomial and its first 3
    // derivatives at input COEFF values.
    //
    testutil::TCASE(b"POLYDS normal case #1: Calculate the value of the polynomial and its first 3 derivatives. Evaluate at input COEFF values.", ctx)?;

    COEFFS[0] = 1.0;
    COEFFS[1] = 3.0;
    COEFFS[2] = 0.5;
    COEFFS[3] = 1.0;
    COEFFS[4] = 0.5;
    COEFFS[5] = -1.0;
    COEFFS[6] = 1.0;

    DEG = 6;
    NDERIV = 3;

    for I in 0..=DEG {
        T = COEFFS[I];

        spicelib::POLYDS(COEFFS.as_slice(), DEG, NDERIV, T, P.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(b"f(coeffs(i))", P[0], b"~", F(T), TIGHT, OK, ctx)?;
        testutil::CHCKSD(b"df1(coeffs(i))", P[1], b"~", DF1(T), TIGHT, OK, ctx)?;
        testutil::CHCKSD(b"df2(coeffs(i))", P[2], b"~", DF2(T), TIGHT, OK, ctx)?;
        testutil::CHCKSD(b"df3(coeffs(i))", P[3], b"~", DF3(T), TIGHT, OK, ctx)?;
    }

    //
    // --- Case: -------------------------------------------------------
    //
    // For the polynomial
    //
    //                          2    3        4    5    6
    //    f(x) = 1 + 3*x + 0.5*x  + x  + 0.5*x  - x  + x
    //
    // calculate the value of the polynomial and its first 3
    // derivatives at input T = 0.3D0.
    //
    testutil::TCASE(b"POLYDS normal case #2: Calculate the value of the polynomial and its first 3 derivatives. Evaluate at input T = 0.3D0.", ctx)?;

    T = 0.3;

    spicelib::POLYDS(COEFFS.as_slice(), DEG, NDERIV, T, P.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"f(0.3)", P[0], b"~", F(T), TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df1(0.3)", P[1], b"~", DF1(T), TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df2(0.3)", P[2], b"~", DF2(T), TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df3(0.3)", P[3], b"~", DF3(T), TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"POLYDS normal case #3: linear polynomial.", ctx)?;

    COEFFS[0] = 1.0;
    COEFFS[1] = 3.0;
    DEG = 1;
    NDERIV = 3;

    T = 0.3;

    spicelib::POLYDS(COEFFS.as_slice(), DEG, NDERIV, T, P.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"f(0.3)", P[0], b"=", 1.9, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df1(0.3)", P[1], b"=", 3.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df2(0.3)", P[2], b"=", 0.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df3(0.3)", P[3], b"=", 0.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"POLYDS normal case #4: constant polynomial.", ctx)?;

    COEFFS[0] = 1.0;
    DEG = 0;
    NDERIV = 3;

    T = 0.3;

    spicelib::POLYDS(COEFFS.as_slice(), DEG, NDERIV, T, P.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"f(0.3)", P[0], b"=", 1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df1(0.3)", P[1], b"=", 0.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df2(0.3)", P[2], b"=", 0.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df3(0.3)", P[3], b"=", 0.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"POLYDS exceptional case #1: NDERIV less than zero.", ctx)?;

    COEFFS[0] = 1.0;
    DEG = 0;
    NDERIV = -1;
    P[0] = 10.0;
    P[1] = 7.0;
    P[2] = -2.0;
    P[3] = 13.0;

    T = 0.3;

    spicelib::POLYDS(COEFFS.as_slice(), DEG, NDERIV, T, P.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the routine is expected to return without
    // doing anything. The values that were set in P should be
    // not modified.
    //
    testutil::CHCKSD(b"f(0.3)", P[0], b"=", 10.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df1(0.3)", P[1], b"=", 7.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df2(0.3)", P[2], b"=", -2.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df3(0.3)", P[3], b"=", 13.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"POLYDS exceptional case #2: DEG less than zero.", ctx)?;

    COEFFS[0] = 1.0;
    DEG = -1;
    NDERIV = 3;
    P[0] = 10.0;
    P[1] = 7.0;
    P[2] = -2.0;
    P[3] = 13.0;

    T = 0.3;

    spicelib::POLYDS(COEFFS.as_slice(), DEG, NDERIV, T, P.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the routine is expected to return but the
    // array P should be initialized to zeros.
    //
    testutil::CHCKSD(b"f(0.3)", P[0], b"=", 0.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df1(0.3)", P[1], b"=", 0.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df2(0.3)", P[2], b"=", 0.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"df3(0.3)", P[3], b"=", 0.0, TIGHT, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
