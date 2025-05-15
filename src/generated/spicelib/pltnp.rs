//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Nearest point on triangular plate
///
/// Find the nearest point on a triangular plate to a given point.
///
/// # Required Reading
///
/// * [DSK](crate::required_reading::dsk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  POINT      I   A point in 3-dimensional space.
///  V1,
///  V2,
///  V3         I   Vertices of a triangular plate.
///  PNEAR      O   Nearest point on the plate to POINT.
///  DIST       O   Distance between PNEAR and POINT.
/// ```
///
/// # Detailed Input
///
/// ```text
///  POINT    is an arbitrary point in 3-dimensional space.
///
///  V1,
///  V2,
///  V3       are 3-vectors constituting the vertices of
///           a triangular plate.
///
///           The plate is allowed to be degenerate: it may
///           consist of a line segment or of a single point.
/// ```
///
/// # Detailed Output
///
/// ```text
///  PNEAR    is the closest point on the plate to POINT.
///           PNEAR is unique, since the plate is convex.
///
///  DIST     is the distance between POINT and PNEAR.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  The input plate is allowed to be degenerate: it may be
///      a line segment or a single point.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as input
///  (if any), the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///
///  1) Find the nearest point to the point (2,2,2) on a plate having
///     vertices at the unit basis vectors that lie along the positive
///     X, Y, and Z coordinate axes.
///
///
///     Example code begins here.
///
///
///           PROGRAM PLTNP_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         FMT1
///           PARAMETER           ( FMT1 = '(A,3E15.7)' )
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      DIST
///           DOUBLE PRECISION      POINT  ( 3 )
///           DOUBLE PRECISION      PNEAR  ( 3 )
///           DOUBLE PRECISION      V1     ( 3 )
///           DOUBLE PRECISION      V2     ( 3 )
///           DOUBLE PRECISION      V3     ( 3 )
///
///     C
///     C     POINT is the input point.
///     C
///           CALL VPACK ( 2.D0, 2.D0, 2.D0, POINT )
///     C
///     C     V1, V2, V3 are the vertices of a plate.
///     C
///           CALL VPACK ( 1.D0, 0.D0, 0.D0, V1 )
///           CALL VPACK ( 0.D0, 1.D0, 0.D0, V2 )
///           CALL VPACK ( 0.D0, 0.D0, 1.D0, V3 )
///     C
///     C     Find the near point on the plate.
///     C
///           CALL PLTNP ( POINT, V1, V2, V3, PNEAR, DIST )
///
///           WRITE (*,*) ' '
///           WRITE (*,FMT1) 'Plate vertex 1 = ', V1
///           WRITE (*,FMT1) 'Plate vertex 2 = ', V2
///           WRITE (*,FMT1) 'Plate vertex 3 = ', V3
///           WRITE (*,FMT1) 'Input point    = ', POINT
///           WRITE (*,*)    ' '
///           WRITE (*,FMT1) 'Near point     = ', PNEAR
///           WRITE (*,FMT1) 'Distance       = ', DIST
///           WRITE (*,*) ' '
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Plate vertex 1 =   0.1000000E+01  0.0000000E+00  0.0000000E+00
///     Plate vertex 2 =   0.0000000E+00  0.1000000E+01  0.0000000E+00
///     Plate vertex 3 =   0.0000000E+00  0.0000000E+00  0.1000000E+01
///     Input point    =   0.2000000E+01  0.2000000E+01  0.2000000E+01
///
///     Near point     =   0.3333333E+00  0.3333333E+00  0.3333333E+00
///     Distance       =   0.2886751E+01
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.3, 04-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
///         Edited code example output format for the solution to fit
///         within the $Examples section without modifications. Added
///         DSK to $Required_Readings.
///
/// -    SPICELIB Version 1.1.2, 01-FEB-2016 (NJB)
///
///         Added code example to header.
///
///      DSKLIB Version 1.1.1, 19-MAR-2015 (NJB)
///
///         Fixed spelling error in header.
///
///      DSKLIB Version 1.1.0, 31-DEC-2014 (NJB)
///
///         Bug fix: vertex indices for outside case, near
///         point on 3rd edge were corrected.
///
///      DSKLIB Version 1.0.0, 29-SEP-2014 (NJB)
/// ```
pub fn pltnp(
    ctx: &mut SpiceContext,
    point: &[f64; 3],
    v1: &[f64; 3],
    v2: &[f64; 3],
    v3: &[f64; 3],
    pnear: &mut [f64; 3],
    dist: &mut f64,
) -> crate::Result<()> {
    PLTNP(point, v1, v2, v3, pnear, dist, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PLTNP ( Nearest point on triangular plate )
pub fn PLTNP(
    POINT: &[f64],
    V1: &[f64],
    V2: &[f64],
    V3: &[f64],
    PNEAR: &mut [f64],
    DIST: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let POINT = DummyArray::new(POINT, 1..=3);
    let V1 = DummyArray::new(V1, 1..=3);
    let V2 = DummyArray::new(V2, 1..=3);
    let V3 = DummyArray::new(V3, 1..=3);
    let mut PNEAR = DummyArrayMut::new(PNEAR, 1..=3);
    let mut D1: f64 = 0.0;
    let mut D2: f64 = 0.0;
    let mut D3: f64 = 0.0;
    let mut E1 = StackArray::<f64, 3>::new(1..=3);
    let mut E2 = StackArray::<f64, 3>::new(1..=3);
    let mut E3 = StackArray::<f64, 3>::new(1..=3);
    let mut ENORM1 = StackArray::<f64, 3>::new(1..=3);
    let mut ENORM2 = StackArray::<f64, 3>::new(1..=3);
    let mut ENORM3 = StackArray::<f64, 3>::new(1..=3);
    let mut L1: f64 = 0.0;
    let mut L2: f64 = 0.0;
    let mut L3: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut NP1 = StackArray::<f64, 3>::new(1..=3);
    let mut NP2 = StackArray::<f64, 3>::new(1..=3);
    let mut PDIFF = StackArray::<f64, 3>::new(1..=3);
    let mut PERP = StackArray::<f64, 3>::new(1..=3);
    let mut DEGEN: bool = false;
    let mut IN1: bool = false;
    let mut IN2: bool = false;
    let mut IN3: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Use discovery check-in.
    //
    //
    // Compute the plate's edges.
    //
    VSUB(V2.as_slice(), V1.as_slice(), E1.as_slice_mut());
    VSUB(V3.as_slice(), V2.as_slice(), E2.as_slice_mut());
    VSUB(V1.as_slice(), V3.as_slice(), E3.as_slice_mut());

    //
    // Compute a normal vector for the plate, if possible.
    // If the plate is degenerate, we'll find out at this point.
    //
    VCRSS(E1.as_slice(), E2.as_slice(), NORMAL.as_slice_mut());

    //
    // Compute the outward normals of the plate's edges in the
    // plate containing the plate.
    //
    VCRSS(E1.as_slice(), NORMAL.as_slice(), ENORM1.as_slice_mut());
    VCRSS(E2.as_slice(), NORMAL.as_slice(), ENORM2.as_slice_mut());
    VCRSS(E3.as_slice(), NORMAL.as_slice(), ENORM3.as_slice_mut());

    DEGEN = (((VZERO(NORMAL.as_slice()) || VZERO(ENORM1.as_slice())) || VZERO(ENORM2.as_slice()))
        || VZERO(ENORM3.as_slice()));

    if DEGEN {
        //
        // The "plate" is a line segment or point. Determine
        // which case we have.
        //
        L1 = VNORM(E1.as_slice());
        L2 = VNORM(E2.as_slice());
        L3 = VNORM(E3.as_slice());

        if ((L1 == 0.0) && (L2 == 0.0)) {
            //
            // Up to round-off error, the vertices coincide.
            // The vertex V1 for practical purposes is the plate.
            //
            VEQU(V1.as_slice(), PNEAR.as_slice_mut());

            *DIST = VDIST(PNEAR.as_slice(), POINT.as_slice());
        } else {
            //
            // The plate is a line segment having positive length.
            // One of the edges will coincide with the segment.
            // Determine which vertices are the endpoints.
            //
            if (L1 > intrinsics::DMAX1(&[L2, L3])) {
                //
                // The segment is bounded by V1 and V2.
                //
                NPSGPT(
                    V1.as_slice(),
                    V2.as_slice(),
                    POINT.as_slice(),
                    PNEAR.as_slice_mut(),
                    DIST,
                    ctx,
                )?;
            } else if (L2 > intrinsics::DMAX1(&[L3, L1])) {
                //
                // The segment is bounded by V2 and V3.
                //
                NPSGPT(
                    V2.as_slice(),
                    V3.as_slice(),
                    POINT.as_slice(),
                    PNEAR.as_slice_mut(),
                    DIST,
                    ctx,
                )?;
            } else {
                //
                // The segment is bounded by V3 and V1.
                //
                NPSGPT(
                    V3.as_slice(),
                    V1.as_slice(),
                    POINT.as_slice(),
                    PNEAR.as_slice_mut(),
                    DIST,
                    ctx,
                )?;
            }
        }
        //
        // The outputs are set for the degenerate cases.
        //
        return Ok(());
    }

    //
    // We have a non-degenerate plate. NORMAL has unit length.
    //
    // We'll treat V1 as an origin in the plane containing
    // the plate. Find the offset of the POINT from V1, and
    // find the component of this offset orthogonal to NORMAL.
    //
    VSUB(POINT.as_slice(), V1.as_slice(), PDIFF.as_slice_mut());
    VPERP(PDIFF.as_slice(), NORMAL.as_slice(), PERP.as_slice_mut());

    //
    // Determine whether V1+PERP is inside the plate.
    //
    // Note that the "line constants" for edges 1 and 3
    // are zero, since these edges contain V1. The line
    // constant for edge 2 is that of the offset of V2
    // from V1; this offset is edge 1.
    //
    IN1 = (VDOT(PERP.as_slice(), ENORM1.as_slice()) <= 0.0);
    IN2 = (VDOT(PERP.as_slice(), ENORM2.as_slice()) <= VDOT(E1.as_slice(), ENORM2.as_slice()));
    IN3 = (VDOT(PERP.as_slice(), ENORM3.as_slice()) <= 0.0);

    if ((IN1 && IN2) && IN3) {
        //
        // V1+PERP is inside the plate. It is the closest
        // point on the plate to POINT.
        //
        VADD(V1.as_slice(), PERP.as_slice(), PNEAR.as_slice_mut());
        //
        // We have the near point; set the distance.
        //
        *DIST = VDIST(PNEAR.as_slice(), POINT.as_slice());
    } else {
        //
        // PERP is outside the plate. The nearest point
        // on the plate to POINT is on one of the edges.
        //
        // We'll use the "in" flags to reduce the number
        // of point-edge distance computations.
        //
        if (!IN1 && (IN2 && IN3)) {
            //
            // The solution must be on the first edge.
            //
            NPSGPT(
                V1.as_slice(),
                V2.as_slice(),
                POINT.as_slice(),
                PNEAR.as_slice_mut(),
                DIST,
                ctx,
            )?;
        } else if (!IN2 && (IN3 && IN1)) {
            //
            // The solution must be on the second edge.
            //
            NPSGPT(
                V2.as_slice(),
                V3.as_slice(),
                POINT.as_slice(),
                PNEAR.as_slice_mut(),
                DIST,
                ctx,
            )?;
        } else if (!IN3 && (IN1 && IN2)) {
            //
            // The solution must be on the third edge.
            //
            NPSGPT(
                V3.as_slice(),
                V1.as_slice(),
                POINT.as_slice(),
                PNEAR.as_slice_mut(),
                DIST,
                ctx,
            )?;
        } else {
            //
            // Compute solutions on all three edges and pick
            // the best one.
            //
            NPSGPT(
                V1.as_slice(),
                V2.as_slice(),
                POINT.as_slice(),
                NP1.as_slice_mut(),
                &mut D1,
                ctx,
            )?;
            NPSGPT(
                V2.as_slice(),
                V3.as_slice(),
                POINT.as_slice(),
                NP2.as_slice_mut(),
                &mut D2,
                ctx,
            )?;
            NPSGPT(
                V3.as_slice(),
                V1.as_slice(),
                POINT.as_slice(),
                PNEAR.as_slice_mut(),
                &mut D3,
                ctx,
            )?;

            if (D1 <= intrinsics::DMIN1(&[D2, D3])) {
                VEQU(NP1.as_slice(), PNEAR.as_slice_mut());
                *DIST = D1;
            } else if (D2 <= intrinsics::DMIN1(&[D3, D1])) {
                VEQU(NP2.as_slice(), PNEAR.as_slice_mut());
                *DIST = D2;
            } else {
                //
                // PNEAR is already set.
                //
                *DIST = D3;
            }
        }
    }

    Ok(())
}
