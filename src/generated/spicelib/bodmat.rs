//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Return transformation matrix for a body
///
/// Return the J2000 to body Equator and Prime Meridian coordinate
/// transformation matrix for a specified body.
///
/// # Required Reading
///
/// * [PCK](crate::required_reading::pck)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  BODY       I   ID code of body.
///  ET         I   Epoch of transformation.
///  TIPM       O   Transformation from Inertial to PM for BODY at ET.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BODY     is the integer ID code of the body for which the
///           transformation is requested. Bodies are numbered
///           according to the standard NAIF numbering scheme.
///
///  ET       is the epoch at which the transformation is
///           requested. (This is typically the epoch of
///           observation minus the one-way light time from
///           the observer to the body at the epoch of
///           observation.)
/// ```
///
/// # Detailed Output
///
/// ```text
///  TIPM     is the transformation matrix from Inertial to body
///           Equator and Prime Meridian. The X axis of the PM
///           system is directed to the intersection of the
///           equator and prime meridian. The Z axis points north.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If data required to define the body-fixed frame associated
///      with BODY are not found in the binary PCK system or the kernel
///      pool, the error SPICE(FRAMEDATANOTFOUND) is signaled. In
///      the case of IAU style body-fixed frames, the absence of
///      prime meridian polynomial data (which are required) is used
///      as an indicator of missing data.
///
///  2)  If the test for exception (1) passes, but in fact requested
///      data are not available in the kernel pool, an error is
///      signaled by a routine in the call tree of this routine.
///
///  3)  If the kernel pool does not contain all of the data required
///      to define the number of nutation precession angles
///      corresponding to the available nutation precession
///      coefficients, the error SPICE(INSUFFICIENTANGLES) is
///      signaled.
///
///  4)  If the reference frame REF is not recognized, an error is
///      signaled by a routine in the call tree of this routine.
///
///  5)  If the specified body code BODY is not recognized, an error is
///      signaled by a routine in the call tree of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is related to the more general routine TIPBOD
///  which returns a matrix that transforms vectors from a
///  specified inertial reference frame to body equator and
///  prime meridian coordinates. TIPBOD accepts an input argument
///  REF that allows the caller to specify an inertial reference
///  frame.
///
///  The transformation represented by BODMAT's output argument TIPM
///  is defined as follows:
///
///     TIPM = [W] [DELTA] [PHI]
///              3        1     3
///
///  If there exists high-precision binary PCK kernel information
///  for the body at the requested time, these angles, W, DELTA
///  and PHI are computed directly from that file. The most
///  recently loaded binary PCK file has first priority followed
///  by previously loaded binary PCK files in backward time order.
///  If no binary PCK file has been loaded, the text P_constants
///  kernel file is used.
///
///  If there is only text PCK kernel information, it is
///  expressed in terms of RA, DEC and W (same W as above), where
///
///     RA    = PHI - HALFPI()
///     DEC   = HALFPI() - DELTA
///
///  RA, DEC, and W are defined as follows in the text PCK file:
///
///     RA  = RA0  + RA1*T  + RA2*T*T   + a  sin theta
///                                        i          i
///
///     DEC = DEC0 + DEC1*T + DEC2*T*T  + d  cos theta
///                                        i          i
///
///     W   = W0   + W1*d   + W2*d*d    + w  sin theta
///                                        i          i
///
///  where:
///
///     d = days past J2000.
///
///     T = Julian centuries past J2000.
///
///     a , d , and w  arrays apply to satellites only.
///      i   i       i
///
///     theta  = THETA0 * THETA1*T are specific to each planet.
///          i
///
///  These angles -- typically nodal rates -- vary in number and
///  definition from one planetary system to the next.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, BODMAT is used to rotate
///  the position vector (POS) from a target body (BODY) to a
///  spacecraft from inertial coordinates to body-fixed coordinates
///  at a specific epoch (ET), in order to compute the planetocentric
///  longitude (PCLONG) of the spacecraft.
///
///     CALL BODMAT ( BODY, ET, TIPM )
///     CALL MXV    ( TIPM, POS, POS )
///     CALL RECLAT ( POS, RADIUS, PCLONG, LAT )
///
///  To compute the equivalent planetographic longitude (PGLONG),
///  it is necessary to know the direction of rotation of the target
///  body, as shown below.
///
///     CALL BODVCD ( BODY, 'PM', 3, DIM, VALUES )
///
///     IF ( VALUES(2) .GT. 0.D0 ) THEN
///        PGLONG = PCLONG
///     ELSE
///        PGLONG = TWOPI() - PCLONG
///     END IF
///
///  Note that the items necessary to compute the transformation
///  TIPM must have been loaded into the kernel pool (by one or more
///  previous calls to FURNSH).
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  K.S. Zukor         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.2.1, 14-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Moved NAIF_IDS
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 4.2.0, 27-JUL-2016 (BVS)
///
///         Updated to use the 3x3 top-left corner of the 6x6 matrix
///         returned by TISBOD instead of fetching kernel data and doing
///         computations in-line.
///
/// -    SPICELIB Version 4.1.1, 01-FEB-2008 (NJB)
///
///         The routine was updated to improve the error messages created
///         when required PCK data are not found. Now in most cases the
///         messages are created locally rather than by the kernel pool
///         access routines. In particular missing binary PCK data will
///         be indicated with a reasonable error message.
///
/// -    SPICELIB Version 4.1.0, 25-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in MXM call.
///
///         Calls to ZZBODVCD have been replaced with calls to
///         BODVCD.
///
/// -    SPICELIB Version 4.0.0, 12-FEB-2004 (NJB)
///
///         Code has been updated to support satellite ID codes in the
///         range 10000 to 99999 and to allow nutation precession angles
///         to be associated with any object.
///
///         Implementation changes were made to improve robustness
///         of the code.
///
/// -    SPICELIB Version 3.2.0, 22-MAR-1995 (KSZ)
///
///         Gets TSIPM matrix from PCKMAT (instead of Euler angles
///         from PCKEUL.)
///
/// -    SPICELIB Version 3.0.0, 10-MAR-1994 (KSZ)
///
///         Ability to get Euler angles from binary PCK file added.
///         This uses the new routine PCKEUL.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 04-SEP-1991 (NJB)
///
///         Updated to handle P_constants referenced to different epochs
///         and inertial reference frames.
///
///         The header was updated to specify that the inertial reference
///         frame used by BODMAT is restricted to be J2000.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 4.2.0, 27-JUL-2016 (BVS)
///
///         Updated to use the 3x3 top-left corner of the 6x6 matrix
///         returned by TISBOD instead of fetching kernel data and doing
///         computations in-line.
///
/// -    SPICELIB Version 4.1.0, 25-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in MXM call.
///
///         Calls to ZZBODVCD have been replaced with calls to
///         BODVCD.
///
/// -    SPICELIB Version 4.0.0, 12-FEB-2004 (NJB)
///
///         Code has been updated to support satellite ID codes in the
///         range 10000 to 99999 and to allow nutation precession angles
///         to be associated with any object.
///
///         Calls to deprecated kernel pool access routine RTPOOL
///         were replaced by calls to GDPOOL.
///
///         Calls to BODVAR have been replaced with calls to
///         ZZBODVCD.
///
/// -    SPICELIB Version 3.2.0, 22-MAR-1995 (KSZ)
///
///         BODMAT now get the TSIPM matrix from PCKMAT, and
///         unpacks TIPM from it. Also the calculated but unused
///         variable LAMBDA was removed.
///
/// -    SPICELIB Version 3.0.0, 10-MAR-1994 (KSZ)
///
///         BODMAT now uses new software to check for the
///         existence of binary PCK files, search the for
///         data corresponding to the requested body and time,
///         and return the appropriate Euler angles, using the
///         new routine PCKEUL. Otherwise the code calculates
///         the Euler angles from the P_constants kernel file.
///
/// -    SPICELIB Version 2.0.0, 04-SEP-1991 (NJB)
///
///         Updated to handle P_constants referenced to different epochs
///         and inertial reference frames.
///
///         The header was updated to specify that the inertial reference
///         frame used by BODMAT is restricted to be J2000.
///
///         BODMAT now checks the kernel pool for presence of the
///         variables
///
///            BODY#_CONSTANTS_REF_FRAME
///
///         and
///
///            BODY#_CONSTANTS_JED_EPOCH
///
///         where # is the NAIF integer code of the barycenter of a
///         planetary system or of a body other than a planet or
///         satellite. If either or both of these variables are
///         present, the P_constants for BODY are presumed to be
///         referenced to the specified inertial frame or epoch.
///         If the epoch of the constants is not J2000, the input
///         time ET is converted to seconds past the reference epoch.
///         If the frame of the constants is not J2000, the rotation from
///         the P_constants' frame to body-fixed coordinates is
///         transformed to the rotation from J2000 coordinates to
///         body-fixed coordinates.
///
///         For efficiency reasons, this routine now duplicates much
///         of the code of BODEUL so that it doesn't have to call BODEUL.
///         In some cases, BODEUL must covert Euler angles to a matrix,
///         rotate the matrix, and convert the result back to Euler
///         angles. If this routine called BODEUL, then in such cases
///         this routine would convert the transformed angles back to
///         a matrix. That would be a bit much....
///
/// -    Beta Version 1.1.0, 16-FEB-1989 (IMU) (NJB)
///
///         $Examples section completed. Declaration of unused variable
///         FOUND removed.
/// ```
pub fn bodmat(
    ctx: &mut SpiceContext,
    body: i32,
    et: f64,
    tipm: &mut [[f64; 3]; 3],
) -> crate::Result<()> {
    BODMAT(body, et, tipm.as_flattened_mut(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure BODMAT ( Return transformation matrix for a body )
pub fn BODMAT(BODY: i32, ET: f64, TIPM: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TIPM = DummyArrayMut2D::new(TIPM, 1..=3, 1..=3);
    let mut TSIPM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE Error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"BODMAT", ctx)?;
    }

    //
    // Get 6x6 state transformation from TISBOD. If succeeded, pull out
    // left-top 3x3 matrix.
    //
    TISBOD(b"J2000", BODY, ET, TSIPM.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"BODMAT", ctx)?;
        return Ok(());
    }

    for I in 1..=3 {
        for J in 1..=3 {
            TIPM[[I, J]] = TSIPM[[I, J]];
        }
    }

    CHKOUT(b"BODMAT", ctx)?;
    Ok(())
}
