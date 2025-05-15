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
    UB: f64,
    R1: f64,
    R2: f64,
    TOL: f64,
    XR1: f64,
    XR2: f64,
    N: i32,
    NX: i32,
    XN: i32,
    XNX: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut A: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut UB: f64 = 0.0;
        let mut R1: f64 = 0.0;
        let mut R2: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XR1: f64 = 0.0;
        let mut XR2: f64 = 0.0;
        let mut N: i32 = 0;
        let mut NX: i32 = 0;
        let mut XN: i32 = 0;
        let mut XNX: i32 = 0;

        Self {
            TITLE,
            A,
            B,
            C,
            UB,
            R1,
            R2,
            TOL,
            XR1,
            XR2,
            N,
            NX,
            XN,
            XNX,
        }
    }
}

//$Procedure F_ZZBQUAD ( ZZBQUAD tests )
pub fn F_ZZBQUAD(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_ZZBQUAD", ctx)?;

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
        b"Coefficients have absolute values that are too large.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    save.A = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    save.B = 1.0;
    save.C = 1.0;
    save.UB = 10.0;

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.A = -10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.A = 1.0;
    save.B = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.B = -10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.B = 1.0;
    save.C = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.B = 1.0;
    save.C = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.C = -10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.A = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    save.B = 1.0;
    save.C = 1.0;
    save.UB = 10.0;

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Non-postive upper bound.");
    testutil::TCASE(&save.TITLE, ctx)?;

    save.A = 1.0;
    save.B = 1.0;
    save.C = -1.0;

    save.UB = 0.0;

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.UB = -1.0;

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

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

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
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

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 0;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XNX = 0;
    testutil::CHCKSI(b"NX", save.NX, b"=", save.XNX, 0, OK, ctx)?;

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

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 1;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XNX = 0;
    testutil::CHCKSI(b"NX", save.NX, b"=", save.XNX, 0, OK, ctx)?;

    save.XR1 = -4.0;
    save.XR2 = -4.0;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"~/", save.XR2, save.TOL, OK, ctx)?;

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

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 0;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XNX = 1;
    testutil::CHCKSI(b"NX", save.NX, b"=", save.XNX, 0, OK, ctx)?;

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

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = -1;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XNX = 0;
    testutil::CHCKSI(b"NX", save.NX, b"=", save.XNX, 0, OK, ctx)?;

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

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = -2;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XNX = 0;
    testutil::CHCKSI(b"NX", save.NX, b"=", save.XNX, 0, OK, ctx)?;

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
        b"Single root of multiplicity 2; root is too large to compute.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Set coefficients to produce the root -4, having multiplicity 2.
    //
    save.A = 4.0;
    save.B = 32.0;
    save.C = 64.0;
    save.UB = 3.0;

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 0;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XNX = 1;
    testutil::CHCKSI(b"NX", save.NX, b"=", save.XNX, 0, OK, ctx)?;

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

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 2;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XNX = 0;
    testutil::CHCKSI(b"NX", save.NX, b"=", save.XNX, 0, OK, ctx)?;

    save.XR1 = 1.0;
    save.XR2 = 2.0;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"~/", save.XR2, save.TOL, OK, ctx)?;

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

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 1;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XNX = 1;
    testutil::CHCKSI(b"NX", save.NX, b"=", save.XNX, 0, OK, ctx)?;

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

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 0;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XNX = 2;
    testutil::CHCKSI(b"NX", save.NX, b"=", save.XNX, 0, OK, ctx)?;

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

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 2;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XNX = 0;
    testutil::CHCKSI(b"NX", save.NX, b"=", save.XNX, 0, OK, ctx)?;

    save.XR1 = 1.0;
    save.XR2 = -200.0;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"~/", save.XR2, save.TOL, OK, ctx)?;

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

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 1;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XNX = 1;
    testutil::CHCKSI(b"NX", save.NX, b"=", save.XNX, 0, OK, ctx)?;

    save.XR1 = 1.0;
    save.XR2 = 0.0;

    save.TOL = 0.0;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.XR1, save.TOL, OK, ctx)?;
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

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 0;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XNX = 2;
    testutil::CHCKSI(b"NX", save.NX, b"=", save.XNX, 0, OK, ctx)?;

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

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 1;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XNX = 1;
    testutil::CHCKSI(b"NX", save.NX, b"=", save.XNX, 0, OK, ctx)?;

    save.XR1 = 10000000000000000000000000000000000000000000000000000000000000000000000.0;
    save.XR2 = 0.0;

    save.TOL = TIGHT;

    testutil::CHCKSD(b"R1", save.R1, b"~/", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"=", save.XR2, 0.0, OK, ctx)?;

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

    spicelib::ZZBQUAD(
        save.A,
        save.B,
        save.C,
        save.UB,
        &mut save.N,
        &mut save.NX,
        &mut save.R1,
        &mut save.R2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XN = 0;
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    save.XNX = 2;
    testutil::CHCKSI(b"NX", save.NX, b"=", save.XNX, 0, OK, ctx)?;

    save.XR1 = 0.0;
    save.XR2 = 0.0;

    save.TOL = 0.0;

    testutil::CHCKSD(b"R1", save.R1, b"=", save.XR1, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"R2", save.R2, b"=", save.XR2, save.TOL, OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
