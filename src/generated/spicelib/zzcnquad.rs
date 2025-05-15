//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SMALL: f64 = 0.00000001;

struct SaveVars {
    INVUB: f64,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut INVUB: f64 = 0.0;
        let mut FIRST: bool = false;

        FIRST = true;
        INVUB = -1.0;

        Self { INVUB, FIRST }
    }
}

//$Procedure  ZZCNQUAD ( Solve quadratic equation for cone intercept )
pub fn ZZCNQUAD(
    A: f64,
    B: f64,
    C: f64,
    UB: f64,
    N: &mut i32,
    R1: &mut f64,
    R2: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut COEFFS = StackArray::<f64, 3>::new(1..=3);
    let mut INV1: f64 = 0.0;
    let mut INV2: f64 = 0.0;
    let mut MAXMAG: f64 = 0.0;
    let mut MAXIX: i32 = 0;
    let mut NX: i32 = 0;

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
    // Saved values
    //

    //
    // Initial values
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZCNQUAD", ctx)?;

    //
    // On the first pass, set the upper bound for the reciprocal
    // solution.
    //
    if save.FIRST {
        save.INVUB = (f64::sqrt(DPMAX()) / 200.0);

        save.FIRST = false;
    }

    //
    // Handle the degenerate cases first.
    //
    if ((A == 0.0) && (B == 0.0)) {
        *R1 = 0.0;
        *R2 = 0.0;

        if (C == 0.0) {
            *N = -1;
        } else {
            *N = -2;
        }

        CHKOUT(b"ZZCNQUAD", ctx)?;
        return Ok(());
    }

    //
    // Scale the input coefficients.
    //
    MAXMAG = intrinsics::DMAX1(&[f64::abs(A), f64::abs(B), f64::abs(C)]);

    COEFFS[1] = TOUCHD((A / MAXMAG));
    COEFFS[2] = TOUCHD((B / MAXMAG));
    COEFFS[3] = TOUCHD((C / MAXMAG));

    //
    // Identify the coefficient of largest magnitude.
    //
    MAXIX = 1;

    for I in 2..=3 {
        if (f64::abs(COEFFS[I]) > f64::abs(COEFFS[MAXIX])) {
            //
            // Record the index of the maximum magnitude.
            //
            MAXIX = I;
        }
    }

    //
    // Make sure the value of maximum magnitude is +/- 1.
    //
    COEFFS[MAXIX] = f64::copysign(1.0, COEFFS[MAXIX]);

    //
    // Find roots in a manner suited to the coefficients we have.
    //
    if ((f64::abs(COEFFS[1]) >= SMALL) || (COEFFS[1] == 0.0)) {
        //
        // This is a numerically well-behaved case. Delegate the
        // job to ZZBQUAD.
        //
        ZZBQUAD(COEFFS[1], COEFFS[2], COEFFS[3], UB, N, &mut NX, R1, R2, ctx)?;
    } else if (f64::abs(COEFFS[3]) >= SMALL) {
        //
        // The zero-order coefficient has magnitude >= SMALL.
        //
        // The original equation
        //
        //       2
        //    a x  + b x + c = 0
        //
        // can be replaced by
        //
        //       2
        //    c y  + b y + a = 0
        //
        // where
        //
        //    y = 1/x
        //
        // Here
        //
        //   |c| >= SMALL
        //   |c| <= 1
        //
        //   |a|  < SMALL
        //
        //
        // Because the quadratic coefficient is bounded away from zero,
        // the roots of the reciprocal equation are not in danger of
        // overflowing. So we can safely solve for 1/x. We might have
        // complex roots; these are rejected.
        //
        // The roots of the transformed equation don't have a maximum
        // magnitude restriction imposed by UB. We set the upper bound
        // to a value that ZZBQUAD will allow.
        //
        ZZBQUAD(
            COEFFS[3], COEFFS[2], COEFFS[1], save.INVUB, N, &mut NX, &mut INV1, &mut INV2, ctx,
        )?;

        if (*N == 1) {
            //
            // We have one real root. Make sure we can invert it.
            //
            if (f64::abs((INV1 * UB)) >= 1.0) {
                //
                //
                // |1/INV1| <= UB
                //
                //
                *R1 = (1.0 / INV1);
            } else {
                //
                // There are no real roots having magnitude within the
                // bound.
                //
                *N = 0;
            }

            //
            // There is no second root.
            //
            *R2 = 0.0;
        } else if (*N == 2) {
            //
            // We have two real roots. The one of larger magnitude is
            // the second one. The reciprocal of this root will be
            // the smaller root of the original equation, as long
            // as the reciprocal is within bounds.
            //
            if (f64::abs((INV2 * UB)) >= 1.0) {
                //
                //
                // |1/INV2| <= UB
                //
                //
                *R1 = (1.0 / INV2);

                //
                // Proceed to the first root of the transformed equation.
                //
                if (f64::abs((INV1 * UB)) >= 1.0) {
                    //
                    //
                    // |1/INV1| <= UB
                    //
                    //
                    *R2 = (1.0 / INV1);
                } else {
                    //
                    // Only the second root qualifies for inversion.
                    //
                    *N = 1;

                    *R2 = 0.0;
                }
            } else {
                //
                // The reciprocal of the larger root is too big; the
                // reciprocal of the smaller root will be even larger.
                // There are no real roots having magnitude within the
                // bound.
                //
                *N = 0;

                *R1 = 0.0;
                *R2 = 0.0;
            }
        } else {
            //
            // We have no viable roots of the transformed equation, so
            // we have no viable roots of the original one.
            //
            *N = 0;

            *R1 = 0.0;
            *R2 = 0.0;
        }
    } else {
        //
        // The linear coefficient B has the greatest magnitude, which
        // is 1. The quadratic coefficient A is "small":  0 < |A| < 1.D-8.
        // The zero-order coefficient is "small" as well.
        //
        // It will be convenient to make B equal to 1; do this now.
        //
        if (B < 0.0) {
            COEFFS[1] = -COEFFS[1];
            COEFFS[2] = -COEFFS[2];
            COEFFS[3] = -COEFFS[3];
        }

        //
        // In this case we use a low-order Taylor expansion about
        // x = 0 for the square root term of the formula for the roots:
        //
        //                           inf
        //                           __
        //                    1/2    \    (k)     k
        //    T(x) = ( 1 + x )    =  /_  f   (0) x / (k!)
        //
        //                           k=0
        //
        //
        //                       2      3         4
        //         =  1 + x/2 - x /8 + x /16 + O(x )
        //
        //
        // Apply this formula to that for the solution having the
        // positive square root term. Here let `x' be
        //
        //
        //            2
        //    -4ac / b
        //
        // which equals
        //
        //    -4ac
        //
        // since we've set b = 1.
        //
        //
        // Then the root is
        //
        //
        //           -1 + sqrt( 1 - 4ac )
        //    x  =   --------------------
        //     1             2a
        //
        //
        //                               2 2          3
        //           -1 + ( 1 - 2ac - 16a c /8 + O((ac)) )
        //       =   -------------------------------------
        //                            2a
        //
        // Discarding the high-order terms in a, we have
        //
        //
        //    x  ~=  ( -1 + 1 - 2ac ) / 2a  =  -c
        //     1
        //
        // Similarly, we have
        //
        //
        //    x  ~=  ( -1 - 1 + 2ac ) / 2a  =  ( ac - 1 )/a  = c - 1/a
        //     2
        //
        //
        // Based on the conditions that got us here, we know
        //
        //    |c| < 1
        //
        //    |c - 1/a| ~= |1/a| > 1.e8
        //

        *N = 0;
        *R1 = 0.0;
        *R2 = 0.0;

        if (f64::abs(COEFFS[3]) <= UB) {
            *R1 = -COEFFS[3];

            *N = 1;

            if (f64::abs(((COEFFS[1] * COEFFS[3]) - 1.0)) < f64::abs((COEFFS[1] * UB))) {
                *R2 = (COEFFS[3] - (1.0 / COEFFS[1]));

                *N = 2;
            }
        }
    }

    CHKOUT(b"ZZCNQUAD", ctx)?;
    Ok(())
}
