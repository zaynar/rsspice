//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    BIG: f64,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BIG: f64 = 0.0;
        let mut FIRST: bool = false;

        BIG = 0.0;
        FIRST = true;

        Self { BIG, FIRST }
    }
}

//$Procedure  ZZBQUAD ( Solve quadratic equation with bounds )
pub fn ZZBQUAD(
    A: f64,
    B: f64,
    C: f64,
    UB: f64,
    N: &mut i32,
    NX: &mut i32,
    R1: &mut f64,
    R2: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DENOM: f64 = 0.0;
    let mut DSCRIM: f64 = 0.0;
    let mut NUM1: f64 = 0.0;
    let mut NUM2: f64 = 0.0;
    let mut SQDISC: f64 = 0.0;

    //
    // SPICELIB functions
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
    // Use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    if save.FIRST {
        save.BIG = (f64::sqrt(DPMAX()) / 100 as f64);
        save.FIRST = false;
    }

    //
    // Set invalid counts to start out. Initialize R1 and R2.
    //
    *N = -3;
    *NX = -3;
    *R1 = 0.0;
    *R2 = 0.0;

    //
    // Reject all large magnitude coefficients.
    //
    if (((f64::abs(A) > save.BIG) || (f64::abs(B) > save.BIG)) || (f64::abs(C) > save.BIG)) {
        CHKIN(b"ZZBQUAD", ctx)?;
        SETMSG(b"Coefficients must have magnitude less than or equal to #, but were A = #; B = #; C = #.", ctx);
        ERRDP(b"#", save.BIG, ctx);
        ERRDP(b"#", A, ctx);
        ERRDP(b"#", B, ctx);
        ERRDP(b"#", C, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZBQUAD", ctx)?;
        return Ok(());
    }

    //
    // Reject large magnitude upper bounds as well.
    //
    if (f64::abs(UB) > save.BIG) {
        CHKIN(b"ZZBQUAD", ctx)?;
        SETMSG(
            b"Upper bounds must have magnitude less than or equal to #, but was #.",
            ctx,
        );
        ERRDP(b"#", save.BIG, ctx);
        ERRDP(b"#", UB, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZBQUAD", ctx)?;
        return Ok(());
    }

    //
    // The upper bound must be positive.
    //
    if (UB <= 0.0) {
        CHKIN(b"ZZBQUAD", ctx)?;
        SETMSG(b"Upper bound must be positive but was #.", ctx);
        ERRDP(b"#", UB, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZBQUAD", ctx)?;
        return Ok(());
    }

    //
    // Handle the degenerate cases first.
    //
    if (A == 0.0) {
        if (B == 0.0) {
            //
            // The equation is of the form
            //
            //    C = 0
            //
            if (C == 0.0) {
                //
                // The equation is satisfied for all real numbers.
                //
                *N = -1;
                *NX = 0;
            } else {
                //
                // There are no solutions.
                //
                *N = -2;
                *NX = 0;
            }
        } else {
            //
            // The equation is first-order:
            //
            //    B*X + C = 0
            //
            // In this branch, B is non-zero.
            //
            if (f64::abs(C) <= f64::abs((UB * B))) {
                *N = 1;
                *NX = 0;

                *R1 = -(C / B);
                *R2 = *R1;
            } else {
                //
                // The magnitude of the solution is too large.
                //
                *N = 0;
                *NX = 1;
            }
        }
    } else {
        //
        // The leading coefficient of the equation is non-zero.
        //
        // We can safely compute the discriminant now, due the
        // check we've already performed.
        //
        DSCRIM = TOUCHD(((B * B) - (((4 as f64) * A) * C)));

        if (DSCRIM < 0.0) {
            //
            // We have complex roots, so we're done.
            //
            *N = 0;
            *NX = 0;
        } else if (DSCRIM == 0.0) {
            //
            // We have a single real root of multiplicity 2.
            //
            // Compare the magnitude of the root to the upper bound.
            //
            NUM1 = -B;
            DENOM = ((2 as f64) * A);

            if (f64::abs(NUM1) >= f64::abs((DENOM * UB))) {
                //
                // The root is too large; we won't compute it.
                //
                *N = 0;
                *NX = 1;
            } else {
                //
                // Set both roots to the same value. In this branch,
                // A is non-zero.
                //
                *N = 1;
                *NX = 0;

                *R1 = ((NUM1 / A) / 2 as f64);
                *R2 = *R1;
            }
        } else {
            //
            // We have two nominally distinct real roots. Whether
            // they're distinct double precision numbers depends
            // on the relative magnitudes of A and DSCRIM.
            //
            DENOM = ((2 as f64) * A);
            SQDISC = f64::sqrt(DSCRIM);

            if (B > 0.0) {
                NUM2 = (-B - SQDISC);
                NUM1 = (-B + SQDISC);
            } else {
                NUM2 = (-B + SQDISC);
                NUM1 = (-B - SQDISC);
            }

            //
            // See whether the root of larger magnitude is computable.
            //
            if (f64::abs(NUM2) <= f64::abs((UB * DENOM))) {
                //
                // The root is computable.

                *N = 2;
                *NX = 0;
                //
                // In this branch, A is non-zero.
                //
                *R2 = ((NUM2 / A) / 2 as f64);

                if (f64::abs(*R2) > 0.0) {
                    //
                    // Compute R1 using R2 and C; this avoids loss
                    // of precision that may occur when NUM1 is computed.
                    //
                    // We know R1 has smaller magnitude than R2 and R2
                    // is computable, and we know A is non-zero, so R1
                    // can be computed without a divide-by-zero error,
                    // and it is computable as long as no intermediate
                    // results overflow. The bounds on A and R2 ensure
                    // that A*R2 is computable.
                    //
                    *R1 = (C / (A * *R2));
                } else {
                    //
                    // The root of larger magnitude has magnitude 0. This
                    // doesn't leave many possible values for the root of
                    // smaller magnitude.
                    //
                    *R1 = 0.0;
                }
            } else {
                //
                // The root of larger magnitude is not computable.
                // Check the root of smaller magnitude.
                //
                if (f64::abs(NUM1) <= f64::abs((UB * DENOM))) {
                    //
                    // The root is computable.
                    //
                    *N = 1;
                    *NX = 1;

                    *R1 = ((NUM1 / A) / 2 as f64);
                } else {
                    //
                    // Neither root is computable.
                    //
                    *N = 0;
                    *NX = 2;
                }
            }
        }
    }

    Ok(())
}
