//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const TOL: f64 = 0.0000000000001;

/// Parabolic time of flight
///
/// Solve the time of flight equation MA = D + (D**3) / 3
/// for the parabolic eccentric anomaly D, given mean anomaly.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  MA         I   Mean anomaly at epoch.
///  D          O   Parabolic eccentric anomaly.
/// ```
///
/// # Detailed Input
///
/// ```text
///  MA       is the parabolic mean anomaly of an orbiting body at
///           some epoch t,
///
///                                3  1/2
///              MA = (t-T) (mu/(2q ))
///
///           where T is the time of periapsis passage, mu is
///           the gravitational parameter of the primary body,
///           and q is the perifocal distance.
/// ```
///
/// # Detailed Output
///
/// ```text
///  D        is the corresponding parabolic anomaly. This is the
///           solution to the time of flight equation
///
///                        3
///              MA = D + D / 3
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  Iterate to solve
///
///                      3
///     f(D,MA,p) = D + D / 3 - MA = 0
/// ```
///
/// # Examples
///
/// ```text
///  ELLTOF, HYPTOF, and PARTOF are used by CONICS.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  R. Bate, D. Mueller, and J. White, "Fundamentals of
///       Astrodynamics," Dover Publications Inc., 1971.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 14-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 19-APR-1991 (WLT)
///
///         A write statement left over from debugging days was removed.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn partof(ctx: &mut SpiceContext, ma: f64, d: &mut f64) -> crate::Result<()> {
    PARTOF(ma, d, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PARTOF ( Parabolic time of flight )
pub fn PARTOF(MA: f64, D: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut M: f64 = 0.0;
    let mut FN: f64 = 0.0;
    let mut DERIV: f64 = 0.0;
    let mut DERIV2: f64 = 0.0;
    let mut CHANGE: f64 = 0.0;

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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"PARTOF", ctx)?;
    }

    //
    // If the mean anomaly is zero, the eccentric anomaly is also zero
    // (by inspection). If the mean anomaly is negative, we can pretend
    // that it's positive (by symmetry).
    //

    if (MA == 0.0) {
        *D = 0.0;
        CHKOUT(b"PARTOF", ctx)?;
        return Ok(());
    } else {
        M = f64::abs(MA);
    }

    //
    // We need an initial guess for the eccentric anomaly D. The function
    // is well behaved, so just about any guess will do.
    //
    *D = DCBRT((3.0 * M));

    //
    // Use the Newton second-order method,
    //
    //                                       2
    //      F    = F  - (f/f')*(1 + f*f''/2f' )
    //       i+1    i
    //
    // where
    //
    //                 3
    //      f   = D + D / 3 - M
    //
    //                 2
    //      f'  = 1 + D
    //
    //
    //      f'' = 2 D
    //

    CHANGE = 1.0;

    while (f64::abs(CHANGE) > TOL) {
        FN = ((*D + (f64::powi(*D, 3) / 3.0)) - M);
        DERIV = (1.0 + f64::powi(*D, 2));
        DERIV2 = (2.0 * *D);

        CHANGE = ((FN / DERIV) * (1.0 + ((FN * DERIV2) / (2.0 * f64::powi(DERIV, 2)))));
        *D = (*D - CHANGE);
    }

    if (MA < 0.0) {
        *D = -*D;
    }

    CHKOUT(b"PARTOF", ctx)?;
    Ok(())
}
