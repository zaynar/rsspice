//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const UBPL: i32 = 4;
const MAGTOL: f64 = 0.00000000000001;

/// Vector projection onto plane
///
/// Project a vector onto a specified plane, orthogonally.
///
/// # Required Reading
///
/// * [PLANES](crate::required_reading::planes)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  VIN        I   Vector to be projected.
///  PLANE      I   A SPICE plane onto which VIN is projected.
///  VOUT       O   Vector resulting from projection.
///  UBPL       P   SPICE plane upper bound.
/// ```
///
/// # Detailed Input
///
/// ```text
///  VIN      is a 3-vector that is to be orthogonally projected
///           onto a specified plane.
///
///  PLANE    is a SPICE plane that represents the geometric
///           plane onto which VIN is to be projected.
///
///           The normal vector component of a SPICE plane has
///           unit length.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is the vector resulting from the orthogonal
///           projection of VIN onto PLANE. VOUT is the closest
///           point in the specified plane to VIN.
/// ```
///
/// # Parameters
///
/// ```text
///  UBPL     is the upper bound of a SPICE plane array.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the normal vector of the input plane does not have unit
///      length (allowing for round-off error), the error
///      SPICE(NONUNITNORMAL) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Projecting a vector VIN orthogonally onto a plane can be thought
///  of as finding the closest vector in the plane to VIN. This
///  "closest vector" always exists; it may be coincident with the
///  original vector.
///
///  Two related routines are VPRJPI, which inverts an orthogonal
///  projection of a vector onto a plane, and VPROJ, which projects
///  a vector orthogonally onto another vector.
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
///  1) Find the closest point in the ring plane of a planet to a
///     spacecraft located at a point (in body-fixed coordinates).
///
///
///     Example code begins here.
///
///
///           PROGRAM VPRJP_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///     C     Upper bound of plane length.
///     C
///           INTEGER                 UBPL
///           PARAMETER             ( UBPL = 4 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      NORM   ( 3    )
///           DOUBLE PRECISION      ORIG   ( 3    )
///           DOUBLE PRECISION      PROJ   ( 3    )
///           DOUBLE PRECISION      RINGPL ( UBPL )
///           DOUBLE PRECISION      SCPOS  ( 3    )
///
///     C
///     C     Set the spacecraft location and define the normal
///     C     vector as the normal to the equatorial plane, and
///     C     the origin at the body/ring center.
///     C
///           DATA                  SCPOS /  -29703.16955D0,
///          .                               879765.72163D0,
///          .                              -137280.21757D0   /
///
///           DATA                  NORM  /  0.D0, 0.D0, 1.D0 /
///
///           DATA                  ORIG  /  0.D0, 0.D0, 0.D0 /
///
///     C
///     C     Create the plane structure.
///     C
///           CALL NVP2PL ( NORM, ORIG, RINGPL )
///
///     C
///     C     Project the position vector onto the ring plane.
///     C
///           CALL VPRJP ( SCPOS, RINGPL, PROJ )
///
///           WRITE(*,'(A)') 'Projection of S/C position onto ring '
///          .            // 'plane:'
///           WRITE(*,'(3F17.5)') PROJ
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Projection of S/C position onto ring plane:
///          -29703.16955     879765.72163          0.00000
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  It is recommended that the input plane be created by one of
///      the SPICELIB routines
///
///         NVC2PL ( Normal vector and constant to plane )
///         NVP2PL ( Normal vector and point to plane    )
///         PSV2PL ( Point and spanning vectors to plane )
///
///      In any case the input plane must have a unit length normal
///      vector and a plane constant consistent with the normal
///      vector.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  G. Thomas and R. Finney, "Calculus and Analytic Geometry,"
///       7th Edition, Addison Wesley, 1988.
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
/// -    SPICELIB Version 1.1.0, 24-AUG-2021 (NJB) (JDR)
///
///         Added error check for non-unit plane normal vector.
///         Changed check-in style to discovery.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example. Added documentation of the parameter UBPL.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 01-NOV-1990 (NJB)
/// ```
pub fn vprjp(
    ctx: &mut SpiceContext,
    vin: &[f64; 3],
    plane: &[f64; 4],
    vout: &mut [f64; 3],
) -> crate::Result<()> {
    VPRJP(vin, plane, vout, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure VPRJP ( Vector projection onto plane )
pub fn VPRJP(
    VIN: &[f64],
    PLANE: &[f64],
    VOUT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let VIN = DummyArray::new(VIN, 1..=3);
    let PLANE = DummyArray::new(PLANE, 1..=UBPL);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=3);
    let mut CONST: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // Tolerance for deviation from unit length of the normal
    // vector of the input plane.
    //

    //
    // Local variables
    //

    //
    // Check RETURN but use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Obtain a unit vector normal to the input plane, and a constant
    // for the plane.
    //
    PL2NVC(PLANE.as_slice(), NORMAL.as_slice_mut(), &mut CONST);

    //
    // The normal vector returned by PL2NVC should be a unit vector.
    //
    if !APPROX(VNORM(NORMAL.as_slice()), 1.0, MAGTOL) {
        CHKIN(b"VPRJP", ctx)?;
        SETMSG(b"Normal vector returned by PL2NVC does not have unit length; the difference of the length from 1 is #. The input plane is invalid. ", ctx);
        ERRDP(b"#", (VNORM(NORMAL.as_slice()) - 1.0), ctx);
        SIGERR(b"SPICE(NONUNITNORMAL)", ctx)?;
        CHKOUT(b"VPRJP", ctx)?;
        return Ok(());
    }

    //
    // Let the notation < a, b > indicate the inner product of vectors
    // a and b.
    //
    // VIN differs from its projection onto PLANE by some multiple of
    // NORMAL. That multiple is
    //
    //
    //           < VIN - VOUT, NORMAL >                 *  NORMAL
    //
    //    =   (  < VIN, NORMAL > - < VOUT, NORMAL >  )  *  NORMAL
    //
    //    =   (  < VIN, NORMAL > - CONST             )  *  NORMAL
    //
    //
    // Subtracting this multiple of NORMAL from VIN yields VOUT.
    //
    VLCOM(
        1.0,
        VIN.as_slice(),
        (CONST - VDOT(VIN.as_slice(), NORMAL.as_slice())),
        NORMAL.as_slice(),
        VOUT.as_slice_mut(),
    );

    Ok(())
}
