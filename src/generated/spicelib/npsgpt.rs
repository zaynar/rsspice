//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Nearest point on line segment
///
/// Find the nearest point on a line segment to a given point.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  EP1,
///  EP2        I   Endpoints of a line segment.
///  POINT      I   A point in 3-dimensional space.
///  PNEAR      O   Nearest point on the line segment to POINT.
///  DIST       O   Distance between PNEAR and POINT.
/// ```
///
/// # Detailed Input
///
/// ```text
///  EP1,
///  EP2      are the endpoints of a line segment in 3-dimensional
///           space. EP1 and EP2 need not be distinct.
///
///  POINT    is an arbitrary point in 3-dimensional space.
/// ```
///
/// # Detailed Output
///
/// ```text
///  PNEAR    is the closest point on the line segment to POINT.
///
///  DIST     is the distance between POINT and PNEAR.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  The input segment is allowed to be degenerate: it may be
///      a single point.
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
///  1) Compute the nearest point on a line segment to a given
///     point in a simple case for which the results can easily be
///     checked.
///
///
///     Example code begins here.
///
///
///           PROGRAM NPSGPT_EX1
///           IMPLICIT NONE
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         FMT1
///           PARAMETER           ( FMT1 = '(A,3F13.8)' )
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      DIST
///           DOUBLE PRECISION      ENDPT1 ( 3 )
///           DOUBLE PRECISION      ENDPT2 ( 3 )
///           DOUBLE PRECISION      PNEAR  ( 3 )
///           DOUBLE PRECISION      POINT  ( 3 )
///
///     C
///     C     Initialize the line segment's endpoints.
///     C
///           CALL VPACK ( 1.D0, -2.D0, 3.D0, ENDPT1 )
///           CALL VPACK ( 1.D0,  2.D0, 3.D0, ENDPT2 )
///     C
///     C     Set the input point.
///     C
///           CALL VPACK ( 1.D0,  0.D0, 0.D0, POINT )
///     C
///     C     Find the near point on the segment.
///     C
///           CALL NPSGPT ( ENDPT1, ENDPT2, POINT, PNEAR, DIST )
///
///           WRITE (*,*) ' '
///           WRITE (*,FMT1) 'Endpoint 1:  ', ENDPT1
///           WRITE (*,FMT1) 'Endpoint 2:  ', ENDPT2
///           WRITE (*,FMT1) 'Point:       ', POINT
///           WRITE (*,*) ' '
///           WRITE (*,FMT1) 'Near point:  ', PNEAR
///           WRITE (*,FMT1) 'Distance:    ', DIST
///           WRITE (*,*) ' '
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Endpoint 1:     1.00000000  -2.00000000   3.00000000
///     Endpoint 2:     1.00000000   2.00000000   3.00000000
///     Point:          1.00000000   0.00000000   0.00000000
///
///     Near point:     1.00000000   0.00000000   3.00000000
///     Distance:       3.00000000
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
/// -    SPICELIB Version 1.0.1, 06-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 02-FEB-2016 (NJB)
///
///         Updated from DSKLIB Version 1.0.0, 20-MAR-2015 (NJB)
/// ```
pub fn npsgpt(
    ctx: &mut SpiceContext,
    ep1: &[f64; 3],
    ep2: &[f64; 3],
    point: &[f64; 3],
    pnear: &mut [f64; 3],
    dist: &mut f64,
) -> crate::Result<()> {
    NPSGPT(ep1, ep2, point, pnear, dist, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure NPSGPT ( Nearest point on line segment )
pub fn NPSGPT(
    EP1: &[f64],
    EP2: &[f64],
    POINT: &[f64],
    PNEAR: &mut [f64],
    DIST: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let EP1 = DummyArray::new(EP1, 1..=3);
    let EP2 = DummyArray::new(EP2, 1..=3);
    let POINT = DummyArray::new(POINT, 1..=3);
    let mut PNEAR = DummyArrayMut::new(PNEAR, 1..=3);
    let mut SEG = StackArray::<f64, 3>::new(1..=3);
    let mut SEGDOT: f64 = 0.0;
    let mut LNEAR = StackArray::<f64, 3>::new(1..=3);
    let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
    let mut OFFDOT: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Find a direction vector defined by the endpoints.
    //
    VSUB(EP2.as_slice(), EP1.as_slice(), SEG.as_slice_mut());

    if VZERO(SEG.as_slice()) {
        //
        // The endpoints coincide, and both coincide with the
        // near point.
        //
        VEQU(EP1.as_slice(), PNEAR.as_slice_mut());

        *DIST = VDIST(EP1.as_slice(), POINT.as_slice());

        return Ok(());
    }

    //
    // Find the nearest point to POINT on the line defined by
    // EP1 and SEG.
    //
    NPLNPT(
        EP1.as_slice(),
        SEG.as_slice(),
        POINT.as_slice(),
        LNEAR.as_slice_mut(),
        DIST,
        ctx,
    )?;

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // Determine whether LNEAR is on the segment, "before" EP1, or
    // "after" EP2, where SEG points in the "increasing" direction.
    //
    VSUB(LNEAR.as_slice(), EP1.as_slice(), OFFSET.as_slice_mut());

    OFFDOT = VDOT(OFFSET.as_slice(), SEG.as_slice());

    if (OFFDOT < 0.0) {
        //
        // The nearest point on the line precedes the first endpoint.
        // The closest point on the segment is the first endpoint.
        //
        VEQU(EP1.as_slice(), PNEAR.as_slice_mut());

        *DIST = VDIST(EP1.as_slice(), POINT.as_slice());
    } else {
        //
        // See whether OFFSET is past the second endpoint. Compare
        // the dot product of OFFSET with SEG to that of SEG with
        // itself, since SEG is the offset of EP2 from EP1.
        //
        SEGDOT = VDOT(SEG.as_slice(), SEG.as_slice());

        if (OFFDOT > SEGDOT) {
            //
            // The nearest point on the line follows the last endpoint.
            // The closest point on the segment is the last endpoint.
            //
            VEQU(EP2.as_slice(), PNEAR.as_slice_mut());

            *DIST = VDIST(EP2.as_slice(), POINT.as_slice());
        } else {
            //
            // The near point is on the segment. LNEAR is actually the
            // solution.
            //
            VEQU(LNEAR.as_slice(), PNEAR.as_slice_mut());

            //
            // DIST was correctly set by the call to NPLNPT.
            //
        }
    }

    Ok(())
}
