//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 5;

/// PCK, unpack segment descriptor
///
/// Unpack the contents of a PCK segment descriptor
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
///  DESCR      I   A PCK segment descriptor.
///  BODY       O   The NAIF ID code for the body of the segment.
///  FRAME      O   The code for the inertial frame of this segment.
///  TYPE       O   The type of PCK segment.
///  FIRST      O   The first epoch for which the segment is valid.
///  LAST       O   The last  epoch for which the segment is valid.
///  BEGIN      O   Beginning DAF address of the segment.
///  END        O   Ending DAF address of the segment.
/// ```
///
/// # Detailed Input
///
/// ```text
///  DESCR    is a PCK segment descriptor.
/// ```
///
/// # Detailed Output
///
/// ```text
///  BODY     is the NAIF ID code for the body of the segment.
///
///  FRAME    is the SPICE ID code for the inertial frame to which
///           the body fixed orientation is referenced.
///
///  TYPE     is the type of PCK segment.
///
///  FIRST    is the first epoch for which the segment has
///           orientation data.
///
///  LAST     is the last epoch for which the segment has
///           orientation data.
///
///  BEGIN    is the starting address of the data associated
///           with this descriptor.
///
///  END      is the last address of the data associated with
///           this descriptor.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine extracts the contents of a PCK segment
///  descriptor into the components needed for reading and
///  evaluating the data stored in the segment. It serves
///  as a macro for expanding the PCK segment descriptor.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you wished to summarize a particular PCK segment
///  and that you have the descriptor for that segment in hand.
///  The following code fragment shows how you might use this
///  routine to create a summary message concerning the segment.
///
///  CALL PCKUDS ( DESCR, BODY, FRAME, TYPE, FIRST, LAST )
///
///  Convert the start and stop times to ephemeris calendar strings
///
///  CALL ETCAL ( FIRST, FSTCAL )
///  CALL ETCAL ( LAST,  LSTCAL )
///
///  WRITE (*,*)
///  WRITE (*,*) 'Body     : ', BODY
///  WRITE (*,*) 'Frame ID : ', FRAME
///  WRITE (*,*) 'Data Type: ', TYPE
///  WRITE (*,*)
///  WRITE (*,*) 'Segment Start : ', FSTCAL
///  WRITE (*,*) 'Segment Stop  : ', LSTCAL
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.2, 02-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 03-JAN-2014 (EDW)
///
///         Minor edits to $Procedure; clean trailing whitespace.
///         Corrected order of header sections to conform to NAIF
///         standard.
///
/// -    SPICELIB Version 1.0.0, 1994-JAN-4 (WLT)
/// ```
pub fn pckuds(
    ctx: &mut SpiceContext,
    descr: &[f64],
    body: &mut i32,
    frame: &mut i32,
    type_: &mut i32,
    first: &mut f64,
    last: &mut f64,
    begin: &mut i32,
    end: &mut i32,
) -> crate::Result<()> {
    PCKUDS(
        descr,
        body,
        frame,
        type_,
        first,
        last,
        begin,
        end,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PCKUDS (PCK, unpack segment descriptor )
pub fn PCKUDS(
    DESCR: &[f64],
    BODY: &mut i32,
    FRAME: &mut i32,
    TYPE: &mut i32,
    FIRST: &mut f64,
    LAST: &mut f64,
    BEGIN: &mut i32,
    END: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut DPPART = StackArray::<f64, 2>::new(1..=ND);
    let mut IPART = StackArray::<i32, 5>::new(1..=NI);

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
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
        CHKIN(b"PCKUDS", ctx)?;
    }
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
        CHKOUT(b"PCKUDS", ctx)?;
        return Ok(());
    }

    *BODY = IPART[1];
    *FRAME = IPART[2];
    *TYPE = IPART[3];
    *BEGIN = IPART[4];
    *END = IPART[5];

    *FIRST = DPPART[1];
    *LAST = DPPART[2];

    CHKOUT(b"PCKUDS", ctx)?;
    Ok(())
}
