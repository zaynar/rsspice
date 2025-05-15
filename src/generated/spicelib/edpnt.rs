//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Ellipsoid point
///
/// Scale a point so that it lies on the surface of a specified
/// triaxial ellipsoid that is centered at the origin and aligned
/// with the Cartesian coordinate axes.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  P          I   A point in three-dimensional space.
///  A          I   Semi-axis length in the X direction.
///  B          I   Semi-axis length in the Y direction.
///  C          I   Semi-axis length in the Z direction.
///  EP         O   Point on ellipsoid.
/// ```
///
/// # Detailed Input
///
/// ```text
///  P        is a non-zero point in three-dimensional space.
///
///  A,
///  B,
///  C        are, respectively, the semi-axis lengths of a triaxial
///           ellipsoid in the X, Y, and Z directions. The axes of
///           the ellipsoid are aligned with the axes of the
///           Cartesian coordinate system.
/// ```
///
/// # Detailed Output
///
/// ```text
///  EP       is the result of scaling the input point P so that
///           it lies on the surface of the triaxial ellipsoid
///           defined by the input semi-axis lengths.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If any of the target ellipsoid's semi-axis lengths is
///      non-positive, the error SPICE(INVALIDAXES) is signaled.
///
///  2)  If P is the zero vector, the error SPICE(ZEROVECTOR) is
///      signaled.
///
///  3)  If the level surface parameter of the input point
///      underflows, the error SPICE(POINTTOOSMALL) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine efficiently computes the ellipsoid surface point
///  corresponding to a specified ray emanating from the origin.
///  Practical examples of this computation occur in the SPICELIB
///  routines LATSRF and SRFREC.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///
///  1) Find the surface intercept point on an ellipsoid having radii
///
///         ( 3, 2, 1 )
///
///     of the ray emanating from the origin and having direction
///     vector
///
///         ( 1, 1, 1 )
///
///
///     Example code begins here.
///
///
///           PROGRAM EDPNT_EX1
///           IMPLICIT NONE
///
///           CHARACTER*(*)         FMT1
///           PARAMETER           ( FMT1 = '(A,F18.14)'  )
///
///           CHARACTER*(*)         FMT3
///           PARAMETER           ( FMT3 = '(A,3F18.14)' )
///
///           DOUBLE PRECISION      A
///           DOUBLE PRECISION      B
///           DOUBLE PRECISION      C
///           DOUBLE PRECISION      V      ( 3 )
///           DOUBLE PRECISION      EP     ( 3 )
///           DOUBLE PRECISION      LEVEL
///
///           A = 3.D0
///           B = 2.D0
///           C = 1.D0
///
///           CALL VPACK ( 1.D0, 1.D0, 1.D0, V )
///
///           CALL EDPNT ( V, A, B, C, EP )
///
///           WRITE (*,FMT3) 'EP    = ', EP
///
///     C
///     C     Verify that EP is on the ellipsoid.
///     C
///           LEVEL = (EP(1)/A)**2 + (EP(2)/B)**2 + (EP(3)/C)**2
///
///           WRITE (*,FMT1) 'LEVEL = ', LEVEL
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     EP    =   0.85714285714286  0.85714285714286  0.85714285714286
///     LEVEL =   1.00000000000000
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 09-JUL-2020 (JDR)
///
///         Minor edits to the header and code example.
///
/// -    SPICELIB Version 2.0.0, 19-APR-2016 (NJB) (EDW)
/// ```
pub fn edpnt(
    ctx: &mut SpiceContext,
    p: &[f64; 3],
    a: f64,
    b: f64,
    c: f64,
    ep: &mut [f64; 3],
) -> crate::Result<()> {
    EDPNT(p, a, b, c, ep, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EDPNT ( Ellipsoid point  )
pub fn EDPNT(
    P: &[f64],
    A: f64,
    B: f64,
    C: f64,
    EP: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let P = DummyArray::new(P, 1..=3);
    let mut EP = DummyArrayMut::new(EP, 1..=3);
    let mut LEVEL: f64 = 0.0;
    let mut SQ: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    if FAILED(ctx) {
        return Ok(());
    }

    if (((A <= 0.0) || (B <= 0.0)) || (C <= 0.0)) {
        CHKIN(b"EDPNT", ctx)?;
        SETMSG(
            b"Ellipsoid radii must be strictly positive but are (#, #, #).",
            ctx,
        );
        ERRDP(b"#", A, ctx);
        ERRDP(b"#", B, ctx);
        ERRDP(b"#", C, ctx);
        SIGERR(b"SPICE(INVALIDRADII)", ctx)?;
        CHKOUT(b"EDPNT", ctx)?;
        return Ok(());
    }

    //
    // The input point must be non-zero, or we can't scale it
    // to the ellipsoid.
    //
    if VZERO(P.as_slice()) {
        CHKIN(b"EDPNT", ctx)?;
        SETMSG(
            b"Input point was the zero vector. A non-zero vector is required.",
            ctx,
        );
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"EDPNT", ctx)?;
        return Ok(());
    }

    //
    // Find the level surface parameter of the input point with respect
    // to the scaled ellipsoid.
    //
    LEVEL =
        TOUCHD(((f64::powi((P[1] / A), 2) + f64::powi((P[2] / B), 2)) + f64::powi((P[3] / C), 2)));

    if (LEVEL <= 0.0) {
        //
        // We expect that LEVEL will be non-negative, but it could
        // be zero. We check for negative values as a precaution.
        //
        CHKIN(b"EDPNT", ctx)?;
        SETMSG(b"Input point\'s level surface parameter was non-positive. The point is too close to the origin to be scaled to the ellipsoid. The point was (#, #, #).", ctx);
        ERRDP(b"#", P[1], ctx);
        ERRDP(b"#", P[2], ctx);
        ERRDP(b"#", P[3], ctx);
        SIGERR(b"SPICE(POINTTOOSMALL)", ctx)?;
        CHKOUT(b"EDPNT", ctx)?;
        return Ok(());
    }

    //
    // Scale the point to one for which the level surface parameter is 1.
    //
    SQ = f64::sqrt(LEVEL);

    EP[1] = (P[1] / SQ);
    EP[2] = (P[2] / SQ);
    EP[3] = (P[3] / SQ);

    Ok(())
}
