//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      T_TAYHRM ( Taylor expansions of Hermite polynomials )
pub fn T_TAYHRM(
    N: i32,
    NEQ: i32,
    MBUFF: &mut [f64],
    RSYS: &mut [f64],
    DERIVS: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut MBUFF = DummyArrayMut::new(MBUFF, 1..=((4 * N) * N));
    let mut RSYS = DummyArrayMut2D::new(RSYS, 1..=(2 * N), 1..=NEQ);
    let mut DERIVS = DummyArrayMut2D::new(DERIVS, 1..=(2 * N), 1..=NEQ);
    let mut FACT: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_TAYHRM", ctx)?;

    //
    // Solve the systems of equations
    //
    //
    //    M * COEFFS = RSYS(1,*)
    //
    // where M is the coefficient matrix packed into MBUFF.
    //

    T_SOLVEG_2(
        RSYS.as_slice_mut(),
        (2 * N),
        NEQ,
        MBUFF.as_slice_mut(),
        DERIVS.as_slice_mut(),
        FOUND,
        ctx,
    );

    if !*FOUND {
        spicelib::CHKOUT(b"T_TAYHRM", ctx)?;
        return Ok(());
    }

    //
    // Compute the derivatives of the Taylor expansion at X.
    //
    FACT = 1.0;

    for I in 1..=(2 * N) {
        if (I > 2) {
            FACT = (FACT * (I - 1) as f64);
        }

        for J in 1..=NEQ {
            DERIVS[[I, J]] = (DERIVS[[I, J]] * FACT);
        }
    }

    spicelib::CHKOUT(b"T_TAYHRM", ctx)?;
    Ok(())
}
