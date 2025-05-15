//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 320;
const VTIGHT: f64 = 0.00000000000001;
const TIGHT: f64 = 0.000000000001;

struct SaveVars {
    TITLE: Vec<u8>,
    A: f64,
    B: f64,
    C: f64,
    C1: StackArray<f64, 2>,
    C2: StackArray<f64, 2>,
    F: f64,
    UB: f64,
    R1: f64,
    R2: f64,
    TOL: f64,
    XR1: f64,
    XR2: f64,
    XVAL1: f64,
    XVAL2: f64,
    N: i32,
    XN: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut A: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut C1 = StackArray::<f64, 2>::new(1..=2);
        let mut C2 = StackArray::<f64, 2>::new(1..=2);
        let mut F: f64 = 0.0;
        let mut UB: f64 = 0.0;
        let mut R1: f64 = 0.0;
        let mut R2: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XR1: f64 = 0.0;
        let mut XR2: f64 = 0.0;
        let mut XVAL1: f64 = 0.0;
        let mut XVAL2: f64 = 0.0;
        let mut N: i32 = 0;
        let mut XN: i32 = 0;

        Self {
            TITLE,
            A,
            B,
            C,
            C1,
            C2,
            F,
            UB,
            R1,
            R2,
            TOL,
            XR1,
            XR2,
            XVAL1,
            XVAL2,
            N,
            XN,
        }
    }
}

//$Procedure F_ZZCNQUAD ( ZZCNQUAD tests )
pub fn F_ZZCNQUAD(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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
    // Save variables in order to avoid stack room problems.
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZCNQUAD", ctx)?;

    //**********************************************************************
    //
    //     Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Bound has absolute values that are too large.",
    );
    testutil::TCASE(&save.TITLE, ctx)?;

    save.A = 1.0;
    save.B = 1.0;
    save.C = 1.0;
    save.UB = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.UB = -10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //**********************************************************************
    //
    //     Non-error exceptions
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Complex roots");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // The equation is
    //
    //    ( X + 2i ) * ( X - 2i ) = 0
    //
    // or
    //
    //     2
    //    X  + 4 = 0
    //
    //
    // Use an upper bound of 10.
    //

    save.A = 1.0;
    save.B = 0.0;
    save.C = 4.0;
    save.UB = 10.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 0;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = 0.0;
    save.XR2 = 0.0;

    save.TOL = 0.0;

    testutil::CHCKSD(b"R1", save.R1, b"=", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"=", save.XR2, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"First order equation, no roots excluded");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // The equation is
    //
    //
    //    X + 4 = 0
    //
    //
    // Use an upper bound of 10.
    //

    save.A = 0.0;
    save.B = 1.0;
    save.C = 4.0;
    save.UB = 10.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 1;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = -4.0;
    save.XR2 = -4.0;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"~/", save.XR2, save.TOL, OK, ctx)?;

    //
    // Make sure the root of larger magnitude is the second
    // one.
    //
    testutil::CHCKSD(
        b"|R1|",
        f64::abs(save.R1),
        b"<=",
        f64::abs(save.R2),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"First order equation, both roots excluded",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // The equation is
    //
    //
    //    X + 4 = 0
    //
    //
    // Use an upper bound of 4 - 1e-12.
    //

    save.A = 0.0;
    save.B = 1.0;
    save.C = 4.0;
    save.UB = (4.0 - 0.000000000001);

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 0;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = 0.0;
    save.XR2 = 0.0;

    save.TOL = 0.0;

    testutil::CHCKSD(b"R1", save.R1, b"=", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"=", save.XR2, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Degenerate equation; form is 0 = 0. N should be set to -1.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // The equation is
    //
    //       2
    //    0*X  + 0*X + 0 = 0
    //
    //
    // Use an upper bound of 10.
    //

    save.A = 0.0;
    save.B = 0.0;
    save.C = 0.0;
    save.UB = 10.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = -1;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = 0.0;
    save.XR2 = 0.0;

    save.TOL = 0.0;

    testutil::CHCKSD(b"R1", save.R1, b"=", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"=", save.XR2, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Degenerate equation; form is 1 = 0. N should be set to -2.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // The equation is
    //
    //       2
    //    0*X  + 0*X + 1 = 0
    //
    //
    // Use an upper bound of 10.
    //

    save.A = 0.0;
    save.B = 0.0;
    save.C = 1.0;
    save.UB = 10.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = -2;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = 0.0;
    save.XR2 = 0.0;

    save.TOL = 0.0;

    testutil::CHCKSD(b"R1", save.R1, b"=", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"=", save.XR2, save.TOL, OK, ctx)?;

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Trivial case: equation with two small real roots. The upper bound does not exclude roots.");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // The equation is
    //
    //    ( X - 1 ) * ( X - 2 ) = 0
    //
    // or
    //
    //     2
    //    X  - 3X + 2 = 0
    //
    //
    // Use an upper bound of 10.
    //
    save.A = 1.0;
    save.B = -3.0;
    save.C = 2.0;
    save.UB = 10.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 2;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = 1.0;
    save.XR2 = 2.0;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"~/", save.XR2, save.TOL, OK, ctx)?;

    //
    // Make sure the root of larger magnitude is the second
    // one.
    //
    testutil::CHCKSD(
        b"|R1|",
        f64::abs(save.R1),
        b"<=",
        f64::abs(save.R2),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Equation with two small real roots; the root of larger magnitude is positive. The upper bound excludes the larger root.");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // The equation is
    //
    //    ( X - 1 ) * ( X - 200 ) = 0
    //
    // or
    //
    //     2
    //    X  - 201X + 200 = 0
    //
    //
    // Use an upper bound of 200 - 1e-12
    //
    save.A = 1.0;
    save.B = -201.0;
    save.C = 200.0;
    save.UB = (200.0 - 0.000000000001);

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 1;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = 1.0;
    save.XR2 = 0.0;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"=", save.XR2, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Equation with two small real roots; the root of larger magnitude is positive. The upper bound excludes both roots.");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Use the equation above but change UB.
    //
    save.UB = (1.0 - 0.000000000001);

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 0;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = 0.0;
    save.XR2 = 0.0;

    save.TOL = 0.0;

    testutil::CHCKSD(b"R1", save.R1, b"=", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"=", save.XR2, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Equation with two small real roots; the root of larger magnitude is negative. No roots are excluded.");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // The equation is
    //
    //    ( X - 1 ) * ( X + 200 ) = 0
    //
    // or
    //
    //     2
    //    X  + 199 X - 200 = 0
    //
    //
    // Use an upper bound of 200 + 1e-12
    //
    save.A = 1.0;
    save.B = 199.0;
    save.C = -200.0;
    save.UB = (200.0 + 0.000000000001);

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 2;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = 1.0;
    save.XR2 = -200.0;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"~/", save.XR2, save.TOL, OK, ctx)?;

    //
    // Make sure the root of larger magnitude is the second
    // one.
    //
    testutil::CHCKSD(
        b"|R1|",
        f64::abs(save.R1),
        b"<=",
        f64::abs(save.R2),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Equation with two small real roots; the root of larger magnitude is negative. The upper bound excludes the root of larger magnitude.");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // The equation is
    //
    //    ( X - 1 ) * ( X + 200 ) = 0
    //
    // or
    //
    //     2
    //    X  + 199 X - 200 = 0
    //
    //
    // Use an upper bound of 200 - 1e-12
    //
    save.A = 1.0;
    save.B = 199.0;
    save.C = -200.0;
    save.UB = (200.0 - 0.000000000001);

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 1;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = 1.0;
    save.XR2 = 0.0;

    save.TOL = TIGHT;
    testutil::CHCKSD(b"R1", save.R1, b"~/", save.XR1, save.TOL, OK, ctx)?;

    save.TOL = 0.0;
    testutil::CHCKSD(b"R2", save.R2, b"=", save.XR2, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Equation with two small real roots; the root of larger magnitude is negative. The upper bound excludes both roots.");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // The equation is
    //
    //    ( X - 1 ) * ( X + 200 ) = 0
    //
    // or
    //
    //     2
    //    X  + 199 X - 200 = 0
    //
    //
    // Use an upper bound of 1 - 1e-12
    //
    save.A = 1.0;
    save.B = 199.0;
    save.C = -200.0;
    save.UB = (1.0 - 0.000000000001);

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 0;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = 0.0;
    save.XR2 = 0.0;

    save.TOL = 0.0;

    testutil::CHCKSD(b"R1", save.R1, b"=", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"=", save.XR2, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Equation with two large real roots. The upper bound excludes the larger root.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // The equation is
    //
    //    ( X - 1.D70 ) * ( X - 2.D70 ) = 0
    //
    // or
    //
    //     2
    //    X  - 3.D70 X + 2.D140 = 0
    //
    //
    // Use an upper bound of 1.5D70
    //
    save.A = 1.0;
    save.B = -30000000000000000000000000000000000000000000000000000000000000000000000.0;
    save.C = 200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    save.UB = 15000000000000000000000000000000000000000000000000000000000000000000000.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 1;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = 10000000000000000000000000000000000000000000000000000000000000000000000.0;
    save.XR2 = 0.0;

    save.TOL = TIGHT;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"=", save.XR2, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Equation with one small real root, one root having magnitude near 1. The upper bound excludes the larger root. The quadratic and zero-order coefficients are small; the linear coefficient has magnitude near 1.");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // The equation is
    //
    //    ( 1.D-5 X - 1.D-10 ) * ( 1.D-5 X - 1.D5 ) = 0
    //
    // or
    //
    //            2
    //    1.D-10 X  -(1+1.D-15) X + 1.D-5 =  0
    //
    //
    // Use an upper bound of 1.D9
    //
    save.A = 0.0000000001;
    save.B = -(1.0 + 0.000000000000001);
    save.C = 0.00001;
    save.UB = 1000000000.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 1;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = 0.00001;
    save.XR2 = 0.0;

    save.TOL = TIGHT;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"=", save.XR2, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Equation with one small real root, one root having magnitude near 1. The upper bound excludes both roots. The quadratic and zero-order coefficients are small; the linear coefficient has magnitude near 1.");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // The equation is
    //
    //    ( 1.D-5 X - 1.D-10 ) * ( 1.D-5 X - 1.D5 ) = 0
    //
    // or
    //
    //            2
    //    1.D-10 X  -(1+1.D-15) X + 1.D-5 =  0
    //
    //
    // Use an upper bound of 1.D9
    //
    save.A = 0.0000000001;
    save.B = -(1.0 + 0.000000000000001);
    save.C = 0.00001;
    save.UB = 0.000001;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 0;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = 0.0;
    save.XR2 = 0.0;

    save.TOL = TIGHT;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"=", save.XR2, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Reciprocal solution case: negative discriminant.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    save.A = 0.0000000001;
    save.B = 1.0;
    save.C = 100000000000.0;
    save.UB = 1000000000.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 0;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Reciprocal solution case: zero discriminant. Root has multiplicity 2. Root magnitude is less than bound.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.A = f64::powf(2.0, -37.0);
    save.B = 1.0;
    save.C = f64::powf(2.0, 35.0);
    save.UB = 1000000000000000.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 1;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Reciprocal solution case: zero discriminant. Root has multiplicity 2. Root is too large to accept.");

    testutil::TCASE(&save.TITLE, ctx)?;

    save.A = f64::powf(2.0, -37.0);
    save.B = 1.0;
    save.C = f64::powf(2.0, 35.0);
    save.UB = 1000000000.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 0;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Equation with two large real roots. The upper bound excludes both roots.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // The equation is
    //
    //    ( X - 1.D70 ) * ( X - 2.D70 ) = 0
    //
    // or
    //
    //     2
    //    X  - 3.D70 X + 2.D140 = 0
    //
    //
    // Use an upper bound of 1.D70 - 1.D-56
    //
    save.A = 1.0;
    save.B = -30000000000000000000000000000000000000000000000000000000000000000000000.0;
    save.C = 200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    save.UB = (10000000000000000000000000000000000000000000000000000000000000000000000.0
        - 100000000000000000000000000000000000000000000000000000000.0);

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 0;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XR1 = 0.0;
    save.XR2 = 0.0;

    save.TOL = 0.0;

    testutil::CHCKSD(b"R1", save.R1, b"=", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"=", save.XR2, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Equation has very large B compared to both A and C.",
    );
    testutil::TCASE(&save.TITLE, ctx)?;

    save.A = 3.0;
    save.B = 400000000000.0;
    save.C = 500.0;
    save.UB = 100000000000000000000.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 2, 0, OK, ctx)?;

    save.F = spicelib::TOUCHD((((save.A * f64::powi(save.R1, 2)) + (save.B * save.R1)) + save.C));

    save.TOL = 0.000001;

    testutil::CHCKSD(b"F(R1)", save.F, b"~", 0.0, save.TOL, OK, ctx)?;

    // WRITE (*,*) 'f(R1): ', F

    //
    // Compare R1 and R2 to results from RQUAD:
    //
    spicelib::RQUAD(
        save.A,
        save.B,
        save.C,
        save.C1.as_slice_mut(),
        save.C2.as_slice_mut(),
        ctx,
    )?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.C1[1], save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"R2", save.R2, b"~/", save.C2[1], save.TOL, OK, ctx)?;

    //
    // Make sure the root of larger magnitude is the second
    // one.
    //
    testutil::CHCKSD(
        b"|R1|",
        f64::abs(save.R1),
        b"<=",
        f64::abs(save.R2),
        save.TOL,
        OK,
        ctx,
    )?;

    // F  = TOUCHD( A * R2**2  +  B * R2  +  C )

    // WRITE (*,*) 'f(R2): ', F

    // WRITE (*,*) 'C1 = ', C1
    // WRITE (*,*) 'C2 = ', C2

    // WRITE (*,*) 'C1(1) - R1 = ', C1(1) - R1
    // WRITE (*,*) 'C2(1) - R2 = ', C2(1) - R2

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Equation has very large B compared to both A and C; B is negative.",
    );
    testutil::TCASE(&save.TITLE, ctx)?;

    save.A = 3.0;
    save.B = -400000000000.0;
    save.C = 500.0;
    save.UB = 100000000000000000000.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 2, 0, OK, ctx)?;

    save.F = spicelib::TOUCHD((((save.A * f64::powi(save.R1, 2)) + (save.B * save.R1)) + save.C));

    save.TOL = 0.000001;

    testutil::CHCKSD(b"F(R1)", save.F, b"~", 0.0, save.TOL, OK, ctx)?;

    // WRITE (*,*) 'f(R1): ', F

    //
    // Compare R1 and R2 to results from RQUAD:
    //
    spicelib::RQUAD(
        save.A,
        save.B,
        save.C,
        save.C1.as_slice_mut(),
        save.C2.as_slice_mut(),
        ctx,
    )?;

    save.TOL = VTIGHT;

    if (f64::abs(save.C1[1]) < f64::abs(save.C2[1])) {
        save.XVAL1 = save.C1[1];
        save.XVAL2 = save.C2[1];
    } else {
        save.XVAL2 = save.C1[1];
        save.XVAL1 = save.C2[1];
    }

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.XVAL1, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"R2", save.R2, b"~/", save.XVAL2, save.TOL, OK, ctx)?;

    //
    // Make sure the root of larger magnitude is the second
    // one.
    //
    testutil::CHCKSD(
        b"|R1|",
        f64::abs(save.R1),
        b"<=",
        f64::abs(save.R2),
        save.TOL,
        OK,
        ctx,
    )?;

    save.F = spicelib::TOUCHD((((save.A * f64::powi(save.R2, 2)) + (save.B * save.R2)) + save.C));

    // WRITE (*,*) 'f(R2): ', F

    // WRITE (*,*) 'C1 = ', C1
    // WRITE (*,*) 'C2 = ', C2

    // WRITE (*,*) 'C1(1) - R1 = ', C1(1) - R1
    // WRITE (*,*) 'C2(1) - R2 = ', C2(1) - R2

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Equation has very large B compared to A; C has the same scale as B.",
    );
    testutil::TCASE(&save.TITLE, ctx)?;

    save.A = 2.0;
    save.B = 3000000000000.0;
    save.C = 4000000000000.0;
    save.UB = 10000000000000000000000000000000000000000.0;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 2, 0, OK, ctx)?;

    //
    // Compare R1 and R2 to results from RQUAD:
    //
    spicelib::RQUAD(
        save.A,
        save.B,
        save.C,
        save.C1.as_slice_mut(),
        save.C2.as_slice_mut(),
        ctx,
    )?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.C1[1], save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"R2", save.R2, b"~/", save.C2[1], save.TOL, OK, ctx)?;

    //
    // Make sure the root of larger magnitude is the second
    // one.
    //
    testutil::CHCKSD(
        b"|R1|",
        f64::abs(save.R1),
        b"<=",
        f64::abs(save.R2),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Equation has very large C compared to A,B.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // This is a case that led to severe loss of precision
    // in ZZBQUAD:
    //
    save.A = -0.00000000000000011102230246251565;
    save.B = 518.7410655978441;
    save.C = -920626.7664155905;

    spicelib::ZZCNQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", 2, 0, OK, ctx)?;

    save.F = spicelib::TOUCHD((((save.A * f64::powi(save.R1, 2)) + (save.B * save.R1)) + save.C));

    save.TOL = 0.000001;

    testutil::CHCKSD(b"F(R1)", save.F, b"~", 0.0, save.TOL, OK, ctx)?;

    //
    // Compare R1 and R2 to results from RQUAD:
    //
    spicelib::RQUAD(
        save.A,
        save.B,
        save.C,
        save.C1.as_slice_mut(),
        save.C2.as_slice_mut(),
        ctx,
    )?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.C1[1], save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"R2", save.R2, b"~/", save.C2[1], save.TOL, OK, ctx)?;

    //
    // Make sure the root of larger magnitude is the second
    // one.
    //
    testutil::CHCKSD(
        b"|R1|",
        f64::abs(save.R1),
        b"<=",
        f64::abs(save.R2),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
