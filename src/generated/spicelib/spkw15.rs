//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NS: i32 = 5;
const SIDLEN: i32 = 40;
const TYPE: i32 = 15;
const FPRINT: i32 = 32;
const LPRINT: i32 = 126;
const DATSIZ: i32 = 16;

/// SPK, write a type 15 segment
///
/// Write an SPK segment of type 15 given a type 15 data record.
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
///  HANDLE     I   Handle of an SPK file open for writing.
///  BODY       I   Body code for ephemeris object.
///  CENTER     I   Body code for the center of motion of the body.
///  FRAME      I   The reference frame of the states.
///  FIRST      I   First valid time for which states can be computed.
///  LAST       I   Last valid time for which states can be computed.
///  SEGID      I   Segment identifier.
///  EPOCH      I   Epoch of the periapse.
///  TP         I   Trajectory pole vector.
///  PA         I   Periapsis vector.
///  P          I   Semi-latus rectum.
///  ECC        I   Eccentricity.
///  J2FLG      I   J2 processing flag.
///  PV         I   Central body pole vector.
///  GM         I   Central body GM.
///  J2         I   Central body J2.
///  RADIUS     I   Equatorial radius of central body.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of an SPK file that has been
///           opened for writing.
///
///  BODY     is the NAIF ID for the body whose states are
///           to be recorded in an SPK file.
///
///  CENTER   is the NAIF ID for the center of motion associated
///           with BODY.
///
///  FRAME    is the reference frame that states are referenced to,
///           for example 'J2000'.
///
///  FIRST    are the bounds on the ephemeris times, expressed as
///  LAST     seconds past J2000.
///
///  SEGID    is the segment identifier. An SPK segment identifier
///           may contain up to 40 characters.
///
///  EPOCH    is the epoch of the orbit elements at periapse
///           in ephemeris seconds past J2000.
///
///  TP       is a vector parallel to the angular momentum vector
///           of the orbit at epoch expressed relative to FRAME. A
///           unit vector parallel to TP will be stored in the
///           output segment.
///
///  PA       is a vector parallel to the position vector of the
///           trajectory at periapsis of EPOCH expressed relative
///           to FRAME. A unit vector parallel to PA will be
///           stored in the output segment.
///
///  P        is the semi-latus rectum--- p in the equation:
///
///              r = p/(1 + ECC*COS(Nu))
///
///  ECC      is the eccentricity.
///
///  J2FLG    is the J2 processing flag describing what J2
///           corrections are to be applied when the orbit is
///           propagated.
///
///           All J2 corrections are applied if the value of J2FLG
///           is not 1, 2 or 3.
///
///           If the value of the flag is 3 no corrections are
///           done.
///
///           If the value of the flag is 1 no corrections are
///           computed for the precession of the line of apsides.
///           However, regression of the line of nodes is
///           performed.
///
///           If the value of the flag is 2 no corrections are
///           done for the regression of the line of nodes.
///           However, precession of the line of apsides is
///           performed.
///
///           Note that J2 effects are computed only if the orbit
///           is elliptic and does not intersect the central body.
///
///  PV       is a vector parallel to the north pole vector of the
///           central body expressed relative to FRAME. A unit
///           vector parallel to PV will be stored in the output
///           segment.
///
///  GM       is the central body GM.
///
///  J2       is the central body J2 (dimensionless).
///
///  RADIUS   is the equatorial radius of the central body.
///
///  Units are radians, km, seconds.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. A type 15 segment is written to the file attached
///  to HANDLE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the eccentricity is less than zero, the error
///      SPICE(BADECCENTRICITY) is signaled.
///
///  2)  If the semi-latus rectum is 0, the error
///      SPICE(BADLATUSRECTUM) is signaled.
///
///  3)  If the pole vector, trajectory pole vector or periapsis vector
///      have zero length, the error SPICE(BADVECTOR) is signaled.
///
///  4)  If the trajectory pole vector and the periapsis vector are
///      not orthogonal, the error SPICE(BADINITSTATE) is signaled.
///      The test for orthogonality is very crude. The routine simply
///      checks that the dot product of the unit vectors parallel
///      to the trajectory pole and periapse vectors is less than
///      0.00001. This check is intended to catch blunders, not to
///      enforce orthogonality to double precision capacity.
///
///  5)  If the mass of the central body is non-positive, the error
///      SPICE(NONPOSITIVEMASS) is signaled.
///
///  6)  If the radius of the central body is negative, the error
///      SPICE(BADRADIUS) is signaled.
///
///  7)  If the segment identifier has more than 40 non-blank
///      characters, the error SPICE(SEGIDTOOLONG) is signaled.
///
///  8)  If the segment identifier contains non-printing characters,
///      the error SPICE(NONPRINTABLECHARS) is signaled.
///
///  9)  If there are inconsistencies in the BODY, CENTER, FRAME or
///      FIRST and LAST times, an error is signaled by
///      a routine in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  A new type 15 SPK segment is written to the SPK file attached
///  to HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes an SPK type 15 data segment to the open SPK
///  file according to the format described in the type 15 section of
///  the SPK Required Reading. The SPK file must have been opened with
///  write access.
///
///  This routine is provided to provide direct support for the MASL
///  precessing orbit formulation.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that at time EPOCH you have the J2000 periapsis
///   state of some object relative to some central body and would
///   like to create a type 15 SPK segment to model the motion of
///   the object using simple regression and precession of the
///   line of nodes and apsides. The following code fragment
///   illustrates how you can prepare such a segment. We shall
///   assume that you have in hand the J2000 direction of the
///   central body's pole vector, its GM, J2 and equatorial
///   radius. In addition we assume that you have opened an SPK
///   file for write access and that it is attached to HANDLE.
///
///  (If your state is at an epoch other than periapse the
///   fragment below will NOT produce a "correct" type 15 segment
///   for modeling the motion of your object.)
///
///   C
///   C     First we get the osculating elements.
///   C
///         CALL OSCELT ( STATE, EPOCH, GM, ELTS )
///
///   C
///   C     From these collect the eccentricity and semi-latus rectum.
///   C
///         ECC = ELTS ( 2 )
///         P   = ELTS ( 1 ) * ( 1.0D0 + ECC )
///   C
///   C     Next get the trajectory pole vector and the
///   C     periapsis vector.
///   C
///         CALL UCRSS ( STATE(1), STATE(4), TP )
///         CALL VHAT  ( STATE(1),           PA )
///
///   C
///   C     Enable both J2 corrections.
///   C
///
///        J2FLG = 0.0D0
///
///   C
///   C     Now add the segment.
///   C
///
///         CALL SPKW15 ( HANDLE, BODY,  CENTER, FRAME,  FIRST, LAST,
///         .              SEGID,  EPOCH, TP,     PA,    P,     ECC,
///         .              J2FLG,  PV,    GM,     J2,    RADIUS      )
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
/// -    SPICELIB Version 2.1.0, 03-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 29-MAY-2012 (NJB)
///
///         Input vectors that nominally have unit length
///         are mapped to local copies that actually do
///         have unit length. The applicable inputs are TP, PA,
///         and PV. The Detailed Input header section was updated
///         to reflect the change.
///
///         Some typos in error messages were corrected.
///
/// -    SPICELIB Version 1.0.0, 28-NOV-1994 (WLT)
/// ```
pub fn spkw15(
    ctx: &mut SpiceContext,
    handle: i32,
    body: i32,
    center: i32,
    frame: &str,
    first: f64,
    last: f64,
    segid: &str,
    epoch: f64,
    tp: &[f64; 3],
    pa: &[f64; 3],
    p: f64,
    ecc: f64,
    j2flg: f64,
    pv: &[f64; 3],
    gm: f64,
    j2: f64,
    radius: f64,
) -> crate::Result<()> {
    SPKW15(
        handle,
        body,
        center,
        frame.as_bytes(),
        first,
        last,
        segid.as_bytes(),
        epoch,
        tp,
        pa,
        p,
        ecc,
        j2flg,
        pv,
        gm,
        j2,
        radius,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKW15 ( SPK, write a type 15 segment )
pub fn SPKW15(
    HANDLE: i32,
    BODY: i32,
    CENTER: i32,
    FRAME: &[u8],
    FIRST: f64,
    LAST: f64,
    SEGID: &[u8],
    EPOCH: f64,
    TP: &[f64],
    PA: &[f64],
    P: f64,
    ECC: f64,
    J2FLG: f64,
    PV: &[f64],
    GM: f64,
    J2: f64,
    RADIUS: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let TP = DummyArray::new(TP, 1..=3);
    let PA = DummyArray::new(PA, 1..=3);
    let PV = DummyArray::new(PV, 1..=3);
    let mut ANGLE: f64 = 0.0;
    let mut DOT: f64 = 0.0;
    let mut DESCR = StackArray::<f64, 5>::new(1..=NS);
    let mut MYTP = StackArray::<f64, 3>::new(1..=3);
    let mut MYPA = StackArray::<f64, 3>::new(1..=3);
    let mut RECORD = StackArray::<f64, 16>::new(1..=DATSIZ);
    let mut VALUE: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //
    //
    // Segment descriptor size
    //

    //
    // Segment identifier size
    //

    //
    // SPK data type
    //

    //
    // Range of printing characters
    //

    //
    // Number of items in a segment
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SPKW15", ctx)?;

    //
    // Fetch the various entities from the inputs and put them into
    // the data record, first the epoch.
    //
    RECORD[1] = EPOCH;

    //
    // Convert TP and PA to unit vectors.
    //
    VHAT(PA.as_slice(), MYPA.as_slice_mut());
    VHAT(TP.as_slice(), MYTP.as_slice_mut());

    //
    // The trajectory pole vector.
    //
    VEQU(MYTP.as_slice(), RECORD.subarray_mut(2));

    //
    // The periapsis vector.
    //
    VEQU(MYPA.as_slice(), RECORD.subarray_mut(5));

    //
    // Semi-latus rectum ( P in the P/(1 + ECC*COS(Nu)  ),
    // and eccentricity.
    //
    RECORD[8] = P;
    RECORD[9] = ECC;

    //
    // J2 processing flag.
    //
    RECORD[10] = J2FLG;

    //
    // Central body pole vector.
    //
    VHAT(PV.as_slice(), RECORD.subarray_mut(11));

    //
    // The central mass, J2 and radius of the central body.
    //
    RECORD[14] = GM;
    RECORD[15] = J2;
    RECORD[16] = RADIUS;

    //
    // Check all the inputs here for obvious failures.  It's much
    // better to check them now and quit than it is to get a bogus
    // segment into an SPK file and diagnose it later.
    //

    if (P <= 0 as f64) {
        SETMSG(b"The semi-latus rectum supplied to the SPK type 15 evaluator was non-positive.  This value must be positive. The value supplied was #.", ctx);
        ERRDP(b"#", P, ctx);
        SIGERR(b"SPICE(BADLATUSRECTUM)", ctx)?;
        CHKOUT(b"SPKW15", ctx)?;
        return Ok(());
    } else if (ECC < 0.0) {
        SETMSG(b"The eccentricity supplied for a type 15 segment is negative.  It must be non-negative. The value supplied to the type 15 evaluator was #. ", ctx);
        ERRDP(b"#", ECC, ctx);
        SIGERR(b"SPICE(BADECCENTRICITY)", ctx)?;
        CHKOUT(b"SPKW15", ctx)?;
        return Ok(());
    } else if (GM <= 0.0) {
        SETMSG(b"The mass supplied for the central body of a type 15 segment was non-positive. Masses must be positive.  The value supplied was #. ", ctx);
        ERRDP(b"#", GM, ctx);
        SIGERR(b"SPICE(NONPOSITIVEMASS)", ctx)?;
        CHKOUT(b"SPKW15", ctx)?;
        return Ok(());
    } else if VZERO(TP.as_slice()) {
        SETMSG(b"The trajectory pole vector supplied to SPKW15 had length zero. The most likely cause of this problem is an uninitialized vector.", ctx);
        SIGERR(b"SPICE(BADVECTOR)", ctx)?;
        CHKOUT(b"SPKW15", ctx)?;
        return Ok(());
    } else if VZERO(PA.as_slice()) {
        SETMSG(b"The periapse vector supplied to SPKW15 had length zero. The most likely cause of this problem is an uninitialized vector.", ctx);
        SIGERR(b"SPICE(BADVECTOR)", ctx)?;
        CHKOUT(b"SPKW15", ctx)?;
        return Ok(());
    } else if VZERO(PV.as_slice()) {
        SETMSG(b"The central pole vector supplied to SPKW15 had length zero. The most likely cause of this problem is an uninitialized vector. ", ctx);
        SIGERR(b"SPICE(BADVECTOR)", ctx)?;
        CHKOUT(b"SPKW15", ctx)?;
        return Ok(());
    } else if (RADIUS < 0.0) {
        SETMSG(b"The central body radius was negative. It must be zero or positive.  The value supplied was #. ", ctx);
        ERRDP(b"#", RADIUS, ctx);
        SIGERR(b"SPICE(BADRADIUS)", ctx)?;
        CHKOUT(b"SPKW15", ctx)?;
        return Ok(());
    }

    //
    // One final check.  Make sure the pole and periapse vectors are
    // orthogonal. (We will use a very crude check but this should
    // rule out any obvious errors.)
    //
    DOT = VDOT(MYPA.as_slice(), MYTP.as_slice());

    if (f64::abs(DOT) > 0.00001) {
        ANGLE = (VSEP(PA.as_slice(), TP.as_slice(), ctx) * DPR(ctx));

        SETMSG(b"The periapsis and trajectory pole vectors are not orthogonal. The angle between them is # degrees. ", ctx);
        ERRDP(b"#", ANGLE, ctx);
        SIGERR(b"SPICE(BADINITSTATE)", ctx)?;
        CHKOUT(b"SPKW15", ctx)?;
        return Ok(());
    }
    //
    // Make sure the segment identifier is not too long.
    //
    if (LASTNB(SEGID) > SIDLEN) {
        SETMSG(b"Segment identifier contains more than 40 characters.", ctx);
        SIGERR(b"SPICE(SEGIDTOOLONG)", ctx)?;
        CHKOUT(b"SPKW15", ctx)?;
        return Ok(());
    }
    //
    // Make sure it has only printing characters.
    //
    for I in 1..=LASTNB(SEGID) {
        VALUE = intrinsics::ICHAR(fstr::substr(SEGID, I..=I));

        if ((VALUE < FPRINT) || (VALUE > LPRINT)) {
            SETMSG(
                b"The segment identifier contains the nonprintable character having ascii code #.",
                ctx,
            );
            ERRINT(b"#", VALUE, ctx);
            SIGERR(b"SPICE(NONPRINTABLECHARS)", ctx)?;
            CHKOUT(b"SPKW15", ctx)?;
            return Ok(());
        }
    }
    //
    // All of the obvious checks have been performed on the input
    // record.  Create the segment descriptor. (FIRST and LAST are
    // checked by SPKPDS as well as consistency between BODY and CENTER).
    //
    SPKPDS(
        BODY,
        CENTER,
        FRAME,
        TYPE,
        FIRST,
        LAST,
        DESCR.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SPKW15", ctx)?;
        return Ok(());
    }

    //
    // Begin a new segment.
    //
    DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKW15", ctx)?;
        return Ok(());
    }

    DAFADA(RECORD.as_slice(), DATSIZ, ctx)?;

    if !FAILED(ctx) {
        DAFENA(ctx)?;
    }

    CHKOUT(b"SPKW15", ctx)?;
    Ok(())
}
