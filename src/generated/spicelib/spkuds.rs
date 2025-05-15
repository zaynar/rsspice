//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 6;

/// SPK - unpack segment descriptor
///
/// Unpack the contents of an SPK segment descriptor.
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
///  DESCR      I   An SPK segment descriptor.
///  BODY       O   The NAIF ID code for the body of the segment.
///  CENTER     O   The center of motion for BODY.
///  FRAME      O   The code for the frame of this segment.
///  TYPE       O   The type of SPK segment.
///  FIRST      O   The first epoch for which the segment is valid.
///  LAST       O   The last  epoch for which the segment is valid.
///  BADDRS     O   Beginning DAF address of the segment.
///  EADDRS     O   Ending DAF address of the segment.
/// ```
///
/// # Detailed Input
///
/// ```text
///  DESCR    is an SPK segment descriptor.
/// ```
///
/// # Detailed Output
///
/// ```text
///  BODY     is the NAIF ID code for the body of the segment.
///
///  CENTER   is the center of motion for BODY.
///
///  FRAME    is the SPICE integer code for the frame to which states
///           for the body are be referenced.
///
///  TYPE     is the type of SPK segment.
///
///  FIRST    is the first epoch for which the segment has
///           ephemeris data.
///
///  LAST     is the last epoch for which the segment has
///           ephemeris data.
///
///  BADDRS   is the starting address of the data associated
///           with this descriptor.
///
///  EADDRS   is the last address of the data associated with
///           this descriptor.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the input descriptor DESCR is invalid, it's possible for
///      the output times to contain bit patterns that don't represent
///      normal double precision values. This error is not diagnosed.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine extracts the contents of an SPK segment
///  descriptor into the components needed for reading and
///  evaluating the data stored in the segment. It serves
///  as a macro for expanding the SPK segment descriptor.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you wished to summarize a particular SPK segment
///  and that you have the descriptor for that segment in hand.
///  The following code fragment shows how you might use this
///  routine to create a summary message concerning the segment.
///
///     CALL SPKUDS ( DESCR, BODY,  CENTER, FRAME,
///    .              TYPE,  FIRST, LAST,   BADDR, EADDR )
///
///  Convert the start and stop times to ephemeris calendar strings
///
///     CALL ETCAL ( FIRST, FSTCAL )
///     CALL ETCAL ( LAST,  LSTCAL )
///
///     WRITE (*,*)
///     WRITE (*,*) 'Body     : ', BODY
///     WRITE (*,*) 'Center   : ', CENTER
///     WRITE (*,*) 'Frame ID : ', FRAME
///     WRITE (*,*) 'Data Type: ', TYPE
///     WRITE (*,*)
///     WRITE (*,*) 'Segment Start : ', FSTCAL
///     WRITE (*,*) 'Segment Stop  : ', LSTCAL
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 02-OCT-2021 (JDR) (NJB)
///
///         Changed output argument names "BEGIN" and "END" to "BADDRS" and
///         "EADDRS" for consistency with other routines.
///
///         Edited the header to comply with NAIF standard. Added entry
///         #1 to $Exceptions section and declared the routine error free.
///
/// -    SPICELIB Version 1.0.0, 04-JAN-1994 (WLT) (KRG)
/// ```
pub fn spkuds(
    ctx: &mut SpiceContext,
    descr: &[f64],
    body: &mut i32,
    center: &mut i32,
    frame: &mut i32,
    type_: &mut i32,
    first: &mut f64,
    last: &mut f64,
    baddrs: &mut i32,
    eaddrs: &mut i32,
) -> crate::Result<()> {
    SPKUDS(
        descr,
        body,
        center,
        frame,
        type_,
        first,
        last,
        baddrs,
        eaddrs,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKUDS ( SPK - unpack segment descriptor )
pub fn SPKUDS(
    DESCR: &[f64],
    BODY: &mut i32,
    CENTER: &mut i32,
    FRAME: &mut i32,
    TYPE: &mut i32,
    FIRST: &mut f64,
    LAST: &mut f64,
    BADDRS: &mut i32,
    EADDRS: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut DPPART = StackArray::<f64, 2>::new(1..=ND);
    let mut IPART = StackArray::<i32, 6>::new(1..=NI);

    //
    // Spicelib Functions
    //

    //
    // Local Parameters
    //
    // Values of ND and NI for SPK files.
    //

    //
    // Local Variables
    //

    //
    // Standard introductory error handling preparations.
    //
    if RETURN(ctx) {
        return Ok(());
    }
    CHKIN(b"SPKUDS", ctx)?;

    //
    // No judgments are made about the descriptor when we
    // unpack it.  If things were done right when the descriptor
    // was created, it should be fine now.
    //
    DAFUS(
        DESCR.as_slice(),
        ND,
        NI,
        DPPART.as_slice_mut(),
        IPART.as_slice_mut(),
    );

    if FAILED(ctx) {
        CHKOUT(b"SPKUDS", ctx)?;
        return Ok(());
    }

    *BODY = IPART[1];
    *CENTER = IPART[2];
    *FRAME = IPART[3];
    *TYPE = IPART[4];
    *BADDRS = IPART[5];
    *EADDRS = IPART[6];

    *FIRST = DPPART[1];
    *LAST = DPPART[2];

    CHKOUT(b"SPKUDS", ctx)?;
    Ok(())
}
