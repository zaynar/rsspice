//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 5;
const CALSIZ: i32 = 40;

/// PCK, pack descriptor
///
/// Perform routine error checks and if all checks pass, pack the
/// descriptor for a PCK segment
///
/// # Required Reading
///
/// * [PCK.](crate::required_reading::pck.)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  BODY       I   The NAIF ID code for the body of the segment.
///  FRAME      I   The inertial frame for this segment.
///  TYPE       I   The type of PCK segment to create.
///  FIRST      I   The first epoch for which the segment is valid.
///  LAST       I   The last  epoch for which the segment is valid.
///  DESCR      O   A PCK segment descriptor.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BODY     is the NAIF ID code for the body of the segment.
///
///  FRAME    is a string that names the inertial frame to which
///           states for the body shall be referenced.
///
///  TYPE     is the type of PCK segment to create.
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
///  DESCR    is a valid PCK segment descriptor to use
///           when creating a DAF segment for this body.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the value of BODY is the ID code of a barycenter --codes 0,
///      1, ..., 9, the error SPICE(BARYCENTERIDCODE) is signaled.
///
///  2)  If FRAME is not one of the known SPICE inertial reference
///      frames, the error SPICE(INVALIDREFFRAME) is signaled.
///
///  3)  If FIRST is greater than or equal to LAST, the error
///      SPICE(BADDESCRTIMES) is signaled.
///
///  4)  If the value of TYPE is outside the range 2 to 1000
///      (inclusive), the error SPICE(UNKNOWNPCKTYPE) is signaled. This
///      does not ensure that the TYPE is a legitimate PCK segment
///      type, but it is a simple check that helps avoid problems that
///      arise from uninitialized values or improperly ordered calling
///      arguments.
/// ```
///
/// # Particulars
///
/// ```text
///  This is a utility routine for validating and creating
///  the descriptor for a PCK segment. It is intended for
///  use only by routines that create PCK segments.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you wish to create a PCK segment of type X
///  and that you are writing a routine to handle the details
///  of the segment creation. This routine can be used to
///  ensure that the descriptor needed for the segment is
///  properly formed and that the information in that descriptor
///  is reasonable.
///
///  Having collected the needed information you can create the
///  descriptor and then begin a new segment as shown below.
///
///  CALL PCKPDS ( BODY, FRAME, TYPE, FIRST, LAST, DESCR )
///  CALL DAFBNA ( HANDLE, DESCR,  SEGID )
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.2, 20-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 03-JAN-2014 (EDW)
///
///         Minor edits to $Procedure; clean trailing whitespace.
///         Corrected order of header sections to conform to NAIF
///         standard.
///
/// -    SPICELIB Version 1.0.0, 04-JAN-1995 (WLT)
/// ```
pub fn pckpds(
    ctx: &mut SpiceContext,
    body: i32,
    frame: &str,
    type_: i32,
    first: f64,
    last: f64,
    descr: &mut [f64],
) -> crate::Result<()> {
    PCKPDS(
        body,
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

//$Procedure PCKPDS ( PCK, pack descriptor )
pub fn PCKPDS(
    BODY: i32,
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
    let mut DPPART = StackArray::<f64, 2>::new(1..=ND);
    let mut IPART = StackArray::<i32, 5>::new(1..=NI);
    let mut REFCOD: i32 = 0;

    //
    // Spicelib Functions
    //
    //
    // Local Parameters
    //
    // ND and NI values for a PCK file.
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
        CHKIN(b"PCKPDS", ctx)?;
    }
    //
    // We do not support orientation models for barycenters.
    //
    if ((BODY >= 0) && (BODY <= 9)) {
        SETMSG(b"You have attempted to create a segment  for for a barycenter, and the PCK system does not support this.", ctx);
        SIGERR(b"SPICE(BARYCENTERIDCODE)", ctx)?;
        CHKOUT(b"PCKPDS", ctx)?;

        return Ok(());
    }
    //
    // Get the NAIF integer code for the reference frame.
    //
    IRFNUM(FRAME, &mut REFCOD, ctx);

    if (REFCOD == 0) {
        SETMSG(b"The reference frame # is not supported.", ctx);
        ERRCH(b"#", FRAME, ctx);
        SIGERR(b"SPICE(INVALIDREFFRAME)", ctx)?;
        CHKOUT(b"PCKPDS", ctx)?;
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
            b"The segment start time: # (#) is at orafter the segment stop time # (#). ",
            ctx,
        );

        ERRDP(b"#", FIRST, ctx);
        ERRCH(b"#", &CALFST, ctx);
        ERRDP(b"#", LAST, ctx);
        ERRCH(b"#", &CALLST, ctx);
        SIGERR(b"SPICE(BADDESCRTIMES)", ctx)?;
        CHKOUT(b"PCKPDS", ctx)?;
        return Ok(());
    }
    //
    // The type must be something reasonable.  The interval from
    // 2 to 1000 is what we are calling reasonable these days.
    //
    if ((TYPE <= 1) || (TYPE > 1000)) {
        SETMSG(
            b"The type specified, #, is not supported within the PCK system. ",
            ctx,
        );
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(UNKNOWNPCKTYPE)", ctx)?;
        CHKOUT(b"PCKPDS", ctx)?;
        return Ok(());
    }
    //
    // Well, that's it.  As far as we can determine these seem to be
    // reasonable values to put into a descriptor.   Do it.
    //
    IPART[1] = BODY;
    IPART[2] = REFCOD;
    IPART[3] = TYPE;
    IPART[4] = 0;
    IPART[5] = 0;

    DPPART[1] = FIRST;
    DPPART[2] = LAST;

    DAFPS(
        ND,
        NI,
        DPPART.as_slice(),
        IPART.as_slice(),
        DESCR.as_slice_mut(),
    );

    CHKOUT(b"PCKPDS", ctx)?;
    Ok(())
}
