//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MAXSIZ: i32 = 25;
const MAX2: i32 = (MAXSIZ * MAXSIZ);
const MAX3: i32 = (MAXSIZ * MAX2);

struct SaveVars {
    RTAB: ActualArray2D<f64>,
    TTAB: ActualArray3D<f64>,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut RTAB = ActualArray2D::<f64>::new(1..=MAXSIZ, 1..=MAXSIZ);
        let mut TTAB = ActualArray3D::<f64>::new(1..=MAXSIZ, 1..=MAXSIZ, 1..=MAXSIZ);
        let mut PASS1: bool = false;

        PASS1 = true;

        Self { RTAB, TTAB, PASS1 }
    }
}

//$Procedure      CHBFIT ( Chebyshev fit )
pub fn CHBFIT(
    FUNC: fn(f64, &mut Context) -> f2rust_std::Result<f64>,
    LEFT: f64,
    RIGHT: f64,
    N: i32,
    WORK: &mut [f64],
    COEFFS: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut WORK = DummyArrayMut::new(WORK, 1..);
    let mut COEFFS = DummyArrayMut::new(COEFFS, 1..);
    let mut ARG: f64 = 0.0;
    let mut MIDPT: f64 = 0.0;
    let mut RADIUS: f64 = 0.0;
    let mut X: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Check in only if an error is detected.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    //
    // Make sure the requested expansion order is not too large.
    //
    if (N > MAXSIZ) {
        spicelib::CHKIN(b"CHBFIT", ctx)?;
        spicelib::SETMSG(
            b"The requested expansion order # exceeds the maximum supported order #.",
            ctx,
        );
        spicelib::ERRINT(b"#", N, ctx);
        spicelib::ERRINT(b"#", MAXSIZ, ctx);
        spicelib::ERRINT(b"#", N, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        spicelib::CHKOUT(b"CHBFIT", ctx)?;
        return Ok(());
    }

    //
    // No data, no interpolation.
    //
    if (N < 1) {
        spicelib::CHKIN(b"CHBFIT", ctx)?;
        spicelib::SETMSG(b"Array size must be positive; was #.", ctx);
        spicelib::ERRINT(b"#", N, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        spicelib::CHKOUT(b"CHBFIT", ctx)?;
        return Ok(());
    }

    //
    // Make sure the input interval is OK.
    //
    if (LEFT >= RIGHT) {
        spicelib::CHKIN(b"CHBFIT", ctx)?;
        spicelib::SETMSG(b"Left endpoint = #; right endpoint = #.", ctx);
        spicelib::ERRDP(b"#", LEFT, ctx);
        spicelib::ERRDP(b"#", RIGHT, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDENDPTS)", ctx)?;
        spicelib::CHKOUT(b"CHBFIT", ctx)?;
        return Ok(());
    }

    if save.PASS1 {
        //
        // On the first pass, compute a table of roots of all
        // Cheby polynomials from degree 1 to degree N.  The Ith
        // column of the table contains roots of the Ith polynomial.
        //
        spicelib::CLEARD(MAX2, save.RTAB.as_slice_mut());

        for I in 1..=MAXSIZ {
            for K in 1..=I {
                save.RTAB[[K, I]] = f64::cos(((spicelib::PI(ctx) * ((K as f64) - 0.5)) / I as f64));
            }
        }

        //
        // Also compute a table of Chebyshev function values.  For
        // each expansion size J from 1 to N, we compute the values
        // of
        //
        //    T   (x ) ... T   ( x )
        //     0    1       0     J
        //
        //             .
        //             .
        //             .
        //
        //    T   (x ) ... T   ( x )
        //     J-1  1       J-1   J
        //
        // where
        //
        //    x
        //     K
        //
        // is the Kth root of
        //
        //    T
        //     J
        //
        // In our 3-dimensional table, the (K,I,J) entry is the value
        // of
        //
        //    T    ( x  )
        //     I-1    K
        //
        // where
        //
        //    x
        //     K
        //
        // is the Kth root of
        //
        //    T
        //     J
        //

        spicelib::CLEARD(MAX3, save.TTAB.as_slice_mut());

        for J in 1..=MAXSIZ {
            //
            // Compute Cheby values needed to implement an expansion
            // of size J.
            //
            for I in 1..=J {
                //
                // Compute values of
                //
                //    T
                //     I-1
                //
                // on the roots of
                //
                //    T
                //     J
                //
                //
                for K in 1..=J {
                    //
                    // Evaluate
                    //
                    //    T
                    //     I-1
                    //
                    // at the Kth root of
                    //
                    //    T
                    //     J
                    //
                    ARG = ((spicelib::PI(ctx) * ((K as f64) - 0.5)) / J as f64);

                    save.TTAB[[K, I, J]] = f64::cos((((I - 1) as f64) * ARG));
                }
            }
        }

        save.PASS1 = false;
    }

    //
    // Find the transformation parameters.
    //
    MIDPT = ((RIGHT + LEFT) / 2.0);
    RADIUS = ((RIGHT - LEFT) / 2.0);

    //
    // Compute the input function values at the transformed Chebyshev
    // roots.
    //
    for K in 1..=N {
        X = ((RADIUS * save.RTAB[[K, N]]) + MIDPT);
        WORK[K] = FUNC(X, ctx)?;
    }

    //
    // Compute the coefficients.
    //
    for J in 1..=N {
        COEFFS[J] = 0.0;

        for K in 1..=N {
            COEFFS[J] = ((WORK[K] * save.TTAB[[K, J, N]]) + COEFFS[J]);
        }

        COEFFS[J] = ((2.0 * COEFFS[J]) / N as f64);
    }

    //
    // Scale the zero-order coefficient to simplify the form of the
    // Chebyshev expansion.
    //
    COEFFS[1] = (COEFFS[1] * 0.5);

    Ok(())
}
