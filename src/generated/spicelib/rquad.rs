//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Roots of a quadratic equation
///
/// Find the roots of a quadratic equation.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  A          I   Coefficient of quadratic term.
///  B          I   Coefficient of linear term.
///  C          I   Constant.
///  ROOT1      O   Root built from positive discriminant term.
///  ROOT2      O   Root built from negative discriminant term.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A,
///  B,
///  C        are the coefficients of a quadratic polynomial
///
///                   2
///              A * x   +  B * x  +  C.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ROOT1,
///  ROOT2    are the roots of the equation
///
///                   2
///              A * x   +  B * x  +  C = 0.
///
///
///           ROOT1 and ROOT2 are both arrays of length 2. The first
///           element of each array is the real part of a root; the
///           second element contains the complex part of the same
///           root.
///
///           When A is non-zero, ROOT1 represents the root
///
///                            _____________
///                           /  2
///              - B   +    \/  B    -   4AC
///              ---------------------------
///                            2A
///
///
///           and ROOT2 represents the root
///
///                            _____________
///                           /  2
///              - B   -    \/  B    -   4AC
///              --------------------------- .
///                            2A
///
///
///           When A is zero and B is non-zero, ROOT1 and ROOT2 both
///           represent the root
///
///              - C / B.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input coefficients A and B are both zero, the error
///      SPICE(DEGENERATECASE) is signaled. The output arguments
///      are not modified.
/// ```
///
/// # Examples
///
/// ```text
///  1)   Humor us and suppose we want to compute the "golden ratio."
///
///       The quantity r is defined by the equation
///
///          1/r = r/(1-r),
///
///       which is equivalent to
///
///           2
///          r   +  r  -  1  =  0.
///
///       The following code fragment does the job.
///
///
///          C
///          C     Compute "golden ratio." The root we want,
///          C
///          C                ___
///          C               /
///          C        -1 + \/  5
///          C        -----------,
///          C             2
///          C
///          C
///          C     is contained in ROOT1.
///          C
///
///                CALL RQUAD ( 1.D0, 1.D0, -1.D0, ROOT1, ROOT2 )
///
///                PRINT *, 'The "golden ratio" is ', ROOT1(1)
///
///
///  2)   The equation,
///
///           2
///          x   +  1  =  0
///
///       can be solved by the code fragment
///
///
///          C
///          C     Let's do one with imaginary roots just for fun.
///          C
///
///                CALL RQUAD ( 1.D0,  0.D0,  1.D0,  ROOT1,  ROOT2 )
///
///                PRINT *, 'ROOT1 is ', ROOT1
///                PRINT *, 'ROOT2 is ', ROOT2
///
///       The printed results will be something like:
///
///
///          ROOT1 is 0.000000000000000    1.000000000000000
///          ROOT2 is 0.000000000000000   -1.000000000000000
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  No checks for overflow of the roots are performed.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 10-JUL-1990 (NJB)
/// ```
pub fn rquad(
    ctx: &mut SpiceContext,
    a: f64,
    b: f64,
    c: f64,
    root1: &mut [f64; 2],
    root2: &mut [f64; 2],
) -> crate::Result<()> {
    RQUAD(a, b, c, root1, root2, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RQUAD ( Roots of a quadratic equation )
pub fn RQUAD(
    A: f64,
    B: f64,
    C: f64,
    ROOT1: &mut [f64],
    ROOT2: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut ROOT1 = DummyArrayMut::new(ROOT1, 1..=2);
    let mut ROOT2 = DummyArrayMut::new(ROOT2, 1..=2);
    let mut CON: f64 = 0.0;
    let mut DISCRM: f64 = 0.0;
    let mut LIN: f64 = 0.0;
    let mut SCALE: f64 = 0.0;
    let mut SQR: f64 = 0.0;
    let mut ZEROED: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"RQUAD", ctx)?;
    }

    //
    // The degree of the equation is zero unless at least one of the
    // second or first degree coefficients is non-zero.
    //
    if ((A == 0.0) && (B == 0.0)) {
        SETMSG(b"Both 1st and 2nd degree coefficients are zero.", ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"RQUAD", ctx)?;
        return Ok(());
    }

    //
    // If we can scale the coefficients without zeroing any of them out,
    // we will do so, to help prevent overflow.
    //
    SCALE = intrinsics::DMAX1(&[f64::abs(A), f64::abs(B), f64::abs(C)]);

    ZEROED = ((((A != 0.0) && ((A / SCALE) == 0.0)) || ((B != 0.0) && ((B / SCALE) == 0.0)))
        || ((C != 0.0) && ((C / SCALE) == 0.0)));

    if !ZEROED {
        SQR = (A / SCALE);
        LIN = (B / SCALE);
        CON = (C / SCALE);
    } else {
        SQR = A;
        LIN = B;
        CON = C;
    }

    //
    // If the second-degree coefficient is non-zero, we have a bona fide
    // quadratic equation, as opposed to a linear equation.
    //
    if (SQR != 0.0) {
        //
        // Compute the discriminant.
        //
        DISCRM = (f64::powi(LIN, 2) - ((4.0 * SQR) * CON));

        //
        // A non-negative discriminant indicates that the roots are
        // real.
        //
        if (DISCRM >= 0.0) {
            //
            // The imaginary parts of both roots are zero.
            //
            ROOT1[2] = 0.0;
            ROOT2[2] = 0.0;
            //
            // We can take advantage of the fact that CON/SQR is the
            // product of the roots to improve the accuracy of the root
            // having the smaller magnitude.  We compute the larger root
            // first and then divide CON/SQR by it to obtain the smaller
            // root.
            //
            if (LIN < 0.0) {
                //
                // ROOT1 will contain the root of larger magnitude.
                //
                ROOT1[1] = ((-LIN + f64::sqrt(DISCRM)) / (2.0 * SQR));

                ROOT2[1] = ((CON / SQR) / ROOT1[1]);
            } else if (LIN > 0.0) {
                //
                // ROOT2 will contain the root of larger magnitude.
                //
                ROOT2[1] = ((-LIN - f64::sqrt(DISCRM)) / (2.0 * SQR));

                ROOT1[1] = ((CON / SQR) / ROOT2[1]);
            } else {
                //
                // The roots have the same magnitude.
                //
                ROOT1[1] = (f64::sqrt(DISCRM) / (2.0 * SQR));

                ROOT2[1] = -ROOT1[1];
            }

        //
        // The only other possibility is that the roots are complex.
        //
        } else {
            //
            // The roots are complex conjugates, so they have equal
            // magnitudes.
            //
            ROOT1[1] = -(LIN / (2.0 * SQR));
            ROOT1[2] = (f64::sqrt(-DISCRM) / (2.0 * SQR));

            ROOT2[1] = ROOT1[1];
            ROOT2[2] = -ROOT1[2];
        }

    //
    // If the second-degree coefficient is zero, we actually have a
    // linear equation.
    //
    } else if (LIN != 0.0) {
        ROOT1[1] = -(CON / LIN);
        ROOT1[2] = 0.0;

        //
        // We set the second root equal to the first, rather than
        // leaving it undefined.
        //
        MOVED(ROOT1.as_slice(), 2, ROOT2.as_slice_mut());
    }

    CHKOUT(b"RQUAD", ctx)?;
    Ok(())
}
