//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// S/P Kernel, solar system barycenter
///
/// Return the state (position and velocity) of a target body
/// relative to the solar system barycenter.
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  TARG       I   Target body.
///  ET         I   Target epoch.
///  REF        I   Target reference frame.
///  STARG      O   State of target.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TARG     is the standard NAIF ID code for a target body.
///
///  ET       is the epoch (ephemeris time) at which the state of the
///           target body is to be computed.
///
///  REF      is the name of the reference frame relative to which the
///           output state vector should be expressed. This may be any
///           frame supported by the SPICELIB frame system, including
///           dynamic and other non-inertial frames.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STARG    is a Cartesian state vector representing the position and
///           velocity of the target body, relative to the solar system
///           barycenter, at epoch ET. This vector is rotated into the
///           specified reference frame. Units are always km and
///           km/sec.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If sufficient information has not been "loaded" via the
///      routine FURNSH, SPKLEF or the PCK kernel loaders, an error is
///      signaled by a routine in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  See $Restrictions.
/// ```
///
/// # Particulars
///
/// ```text
///  In order to compute the state of one body relative to another,
///  the states of the two bodies must be known relative to a third
///  body. One simple solution is to use the solar system barycenter
///  as the third body.
///
///  Ephemeris data from more than one segment may be required
///  to determine the state of a body relative to the barycenter.
///  SPKSSB reads as many segments as necessary, from as many
///  files as necessary, using files that have been loaded by
///  previous calls to FURNSH or SPKLEF (load ephemeris file).
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, SPKSSB is used to display
///  the distance from Earth (Body 399) to Mars (body 499) at
///  a series of epochs.
///
///     CALL SPKLEF ( 'DE125.SPK', HANDLE )
///      .
///      .
///
///     EARTH = 399
///     MARS  = 499
///
///     DO WHILE ( EPOCH .LE. END )
///        CALL SPKSSB ( EARTH, EPOCH, 'J2000', SEARTH )
///        CALL SPKSSB ( MARS,  EPOCH, 'J2000', SMARS  )
///
///        CALL VSUB ( SMARS, SEARTH, SMARS )
///        WRITE (*,*) EPOCH, VNORM ( SMARS )
///
///        EPOCH = EPOCH + DELTA
///     END DO
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The ephemeris files to be used by SPKSSB must be loaded
///      by FURNSH or SPKLEF before SPKSSB is called.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.4, 13-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Updated
///         detailed description of arguments REF and STARG.
///
/// -    SPICELIB Version 2.0.3, 18-MAY-2010 (BVS)
///
///         Removed "C$" marker from text in the header.
///
/// -    SPICELIB Version 2.0.2, 20-NOV-2004 (NJB)
///
///         Updated description of input argument REF to indicate all
///         frames supported by SPICELIB are allowed.
///
/// -    SPICELIB Version 2.0.1, 24-JUN-1999 (WLT)
///
///         Corrected code in $Examples section of the headers
///
/// -    SPICELIB Version 2.0.0, 19-SEP-1995 (WLT)
///
///         The routine was simplified by replacing all of the
///         main body of code with a call to SPKGEO. By making
///         this change the routine now supports non-inertial frames.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 19-SEP-1995 (WLT)
///
///         The routine was simplified by replacing all of the
///         main body of code with a call to SPKGEO. By making
///         this change the routine now supports non-inertial frames.
/// ```
pub fn spkssb(
    ctx: &mut SpiceContext,
    targ: i32,
    et: f64,
    ref_: &str,
    starg: &mut [f64; 6],
) -> crate::Result<()> {
    SPKSSB(targ, et, ref_.as_bytes(), starg, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKSSB ( S/P Kernel, solar system barycenter )
pub fn SPKSSB(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    STARG: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut STARG = DummyArrayMut::new(STARG, 1..=6);
    let mut LT: f64 = 0.0;
    let mut BARY: i32 = 0;

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
        CHKIN(b"SPKSSB", ctx)?;
    }

    BARY = 0;

    SPKGEO(TARG, ET, REF, BARY, STARG.as_slice_mut(), &mut LT, ctx)?;

    CHKOUT(b"SPKSSB", ctx)?;
    Ok(())
}
