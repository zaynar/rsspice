//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 6;
const CALSIZ: i32 = 40;

/// SPK pack descriptor
///
/// Perform routine error checks and if all check pass, pack the
/// descriptor for an SPK segment
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
///  BODY       I   The NAIF ID code for the body of the segment.
///  CENTER     I   The center of motion for BODY.
///  FRAME      I   The frame for this segment.
///  TYPE       I   The type of SPK segment to create.
///  FIRST      I   The first epoch for which the segment is valid.
///  LAST       I   The last  epoch for which the segment is valid.
///  DESCR      O   An SPK segment descriptor.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BODY     is the NAIF ID code for the body of the segment.
///
///  CENTER   is the center of motion for BODY.
///
///  FRAME    is a string that names the frame to which states for
///           the body shall be referenced.
///
///  TYPE     is the type of SPK segment to create.
///
///  FIRST    is the first epoch for which the segment will have
///           ephemeris data.
///
///  LAST     is the last epoch for which the segment will have
///           ephemeris data.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DESCR    is a valid SPK segment descriptor to use
///           when creating a DAF segment for this body.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the value of BODY is 0, the error SPICE(BARYCENTEREPHEM) is
///      signaled.
///
///  2)  If the values of BODY and CENTER are the same, the error
///      SPICE(BODYANDCENTERSAME) is signaled.
///
///  3)  If FRAME is not one of the known SPICE reference frames, the
///      error SPICE(INVALIDREFFRAME) is signaled.
///
///  4)  If FIRST is greater than or equal to LAST, the error
///      SPICE(BADDESCRTIMES) is signaled.
///
///  5)  If the value of TYPE is outside the range 1 to 1000
///      (inclusive), the error SPICE(UNKNOWNSPKTYPE) is signaled. This
///      does not ensure that the TYPE is a legitimate SPK segment
///      type, but it is a simple check that helps avoid problems that
///      arise from uninitialized values or improperly ordered calling
///      arguments.
/// ```
///
/// # Particulars
///
/// ```text
///  This is a utility routine for validating and creating
///  the descriptor for an SPK segment. It is intended for
///  use only by routines that create SPK segments.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you wish to create an SPK segment of type X
///  and that you are writing a routine to handle the details
///  of the segment creation. This routine can be used to
///  ensure that the descriptor needed for the segment is
///  properly formed and that the information in that descriptor
///  is reasonable.
///
///  Having collected the needed information you can create the
///  descriptor and then begin a new segment as shown below.
///
///  CALL SPKPDS ( BODY,   CENTER, FRAME, TYPE, FIRST, LAST, DESCR )
///  CALL DAFBNA ( HANDLE, DESCR,  SEGID )
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 05-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 19-SEP-1995 (WLT)
///
///         Upgraded the routine to support non-inertial frames.
///
/// -    SPICELIB Version 1.0.0, 04-JAN-1994 (WLT) (KRG)
/// ```
pub fn spkpds(
    ctx: &mut SpiceContext,
    body: i32,
    center: i32,
    frame: &str,
    type_: i32,
    first: f64,
    last: f64,
    descr: &mut [f64],
) -> crate::Result<()> {
    SPKPDS(
        body,
        center,
        frame.as_bytes(),
        type_,
        first,
        last,
        descr,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKPDS ( SPK pack descriptor )
pub fn SPKPDS(
    BODY: i32,
    CENTER: i32,
    FRAME: &[u8],
    TYPE: i32,
    FIRST: f64,
    LAST: f64,
    DESCR: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DESCR = DummyArrayMut::new(DESCR, 1..);
    let mut CALFST = [b' '; CALSIZ as usize];
    let mut CALLST = [b' '; CALSIZ as usize];
    let mut DPPART = StackArray::<f64, 2>::new(1..=2);
    let mut IPART = StackArray::<i32, 6>::new(1..=6);
    let mut REFCOD: i32 = 0;

    //
    // Spicelib Functions
    //

    //
    // Local Parameters
    //
    // ND and NI values for an SPK file.
    //

    //
    // Length of a calender string.
    //
    //
    // Local Variables
    //

    //
    // Standard SPICELIB error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPKPDS", ctx)?;
    }
    //
    // We do not support ephemerides for the solar system barycenter
    // (at least not yet anyway).
    //
    if (BODY == 0) {
        SETMSG(b"You\'ve attempted to create a segment for the solar system barycenter.  This is not supported by the ephemeris system.", ctx);
        SIGERR(b"SPICE(BARYCENTEREPHEM)", ctx)?;
        CHKOUT(b"SPKPDS", ctx)?;

        return Ok(());
    }
    //
    // There is no point in having an ephemeris for a body relative
    // to itself.
    //
    if (BODY == CENTER) {
        SETMSG(b"You\'ve attempted to create a segment for a body relative to itself. The body ID code was: #.", ctx);
        ERRINT(b"#", BODY, ctx);
        SIGERR(b"SPICE(BODYANDCENTERSAME)", ctx)?;
        CHKOUT(b"SPKPDS", ctx)?;
        return Ok(());
    }
    //
    // Get the NAIF integer code for the reference frame.
    //
    NAMFRM(FRAME, &mut REFCOD, ctx)?;

    if (REFCOD == 0) {
        SETMSG(b"The reference frame # is not supported.", ctx);
        ERRCH(b"#", FRAME, ctx);
        SIGERR(b"SPICE(INVALIDREFFRAME)", ctx)?;
        CHKOUT(b"SPKPDS", ctx)?;
        return Ok(());
    }
    //
    // The segment stop time should be greater then the begin time.
    //
    if (FIRST >= LAST) {
        //
        // We've got an error. Get the calendar string for the first
        // and last epochs.
        //
        ETCAL(FIRST, &mut CALFST, ctx);
        ETCAL(LAST, &mut CALLST, ctx);

        SETMSG(
            b"The segment start time: # (#) is at or after the segment stop time # (#).",
            ctx,
        );

        ERRDP(b"#", FIRST, ctx);
        ERRCH(b"#", &CALFST, ctx);
        ERRDP(b"#", LAST, ctx);
        ERRCH(b"#", &CALLST, ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"SPKPDS", ctx)?;
        return Ok(());
    }
    //
    // The type must be something reasonable.  The interval from
    // 1 to 1000 is what we are calling reasonable these days.
    //
    if ((TYPE <= 0) || (TYPE > 1000)) {
        SETMSG(
            b"The type specified, #, is not supported within the SPK system.",
            ctx,
        );
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(UNKNOWNSPKTYPE)", ctx)?;
        CHKOUT(b"SPKPDS", ctx)?;
        return Ok(());
    }
    //
    // Well, that's it.  As far as we can determine these seem to be
    // reasonable values to put into a descriptor.   Do it.
    //
    IPART[1] = BODY;
    IPART[2] = CENTER;
    IPART[3] = REFCOD;
    IPART[4] = TYPE;
    IPART[5] = 0;
    IPART[6] = 0;

    DPPART[1] = FIRST;
    DPPART[2] = LAST;

    DAFPS(
        ND,
        NI,
        DPPART.as_slice(),
        IPART.as_slice(),
        DESCR.as_slice_mut(),
    );

    CHKOUT(b"SPKPDS", ctx)?;
    Ok(())
}
