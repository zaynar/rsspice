//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MAXN: i32 = 50;

//$Procedure T_TAYMAT ( Taylor expansion coefficient matrix )
pub fn T_TAYMAT(
    N: i32,
    XVALS: &[f64],
    X: f64,
    MBUFF: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let XVALS = DummyArray::new(XVALS, 1..=N);
    let mut MBUFF = DummyArrayMut::new(MBUFF, 1..=((4 * N) * N));
    let mut M = ActualArray2D::<f64>::new(1..=MAXN, 0..=(MAXN - 1));
    let mut DEG: i32 = 0;
    let mut J: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_TAYMAT", ctx)?;

    if (N > (MAXN / 2)) {
        spicelib::SETMSG(b"Grid size # exceeds maximum allowed value #.", ctx);
        spicelib::ERRINT(b"#", N, ctx);
        spicelib::ERRINT(b"#", (MAXN / 2), ctx);
        spicelib::SIGERR(b"SPICE(OVERFLOW)", ctx)?;
        spicelib::CHKOUT(b"T_TAYMAT", ctx)?;
        return Ok(());
    }

    //
    // Let DEG be the Hermite polynomial degree.
    //
    DEG = ((2 * N) - 1);

    //
    // Fill in the top half of M.
    //
    for I in 1..=N {
        M[[I, 0]] = 1.0;

        {
            let m1__: i32 = 1;
            let m2__: i32 = DEG;
            let m3__: i32 = 1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                M[[I, J]] = ((XVALS[I] - X) * M[[I, (J - 1)]]);

                J += m3__;
            }
        }
    }

    //
    // Fill in the bottom half of M.
    //
    for I in (N + 1)..=(2 * N) {
        M[[I, 0]] = 0.0;
        M[[I, 1]] = 1.0;

        {
            let m1__: i32 = 2;
            let m2__: i32 = DEG;
            let m3__: i32 = 1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                M[[I, J]] = (M[[(I - N), (J - 1)]] * J as f64);

                J += m3__;
            }
        }
    }

    //
    // Pack the significant portion of M into a buffer of
    // contiguous elements.
    //
    J = 1;

    for I in 0..=DEG {
        spicelib::MOVED(M.subarray([1, I]), (2 * N), MBUFF.subarray_mut(J));
        J = (J + (2 * N));
    }

    spicelib::CHKOUT(b"T_TAYMAT", ctx)?;
    Ok(())
}
