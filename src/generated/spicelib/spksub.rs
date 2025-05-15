//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// S/P Kernel, subset
///
/// Extract a subset of the data in an SPK segment into a
/// separate segment.
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of source segment.
///  DESCR      I   Descriptor of source segment.
///  IDENT      I   Identifier of source segment.
///  BEGIN      I   Beginning (initial epoch) of subset.
///  END        I   End (final epoch) of subset.
///  NEWH       I   Handle of new segment.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE,
///  DESCR,
///  IDENT    are the file handle assigned to a SPK file, the
///           descriptor for a segment within the file, and the
///           identifier for that segment. Together they determine a
///           complete set of ephemeris data, from which a subset is to
///           be extracted.
///
///  BEGIN,
///  END      are the initial and final epochs (ephemeris time) of the
///           subset.
///
///  NEWH     is the file handle assigned to the file in which the new
///           segment is to be written. The file must be open for write
///           access. NEWH and HANDLE may refer to the same file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  See $Files section.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the condition
///
///         ALPHA  <=  BEGIN  <=  END  <=  OMEGA
///
///      is not satisfied (where ALPHA and OMEGA are the initial and
///      final epochs of the segment respectively), the error
///      SPICE(SPKNOTASUBSET) is signaled.
///
///  2)  If the segment type is not supported by the current version of
///      SPKSUB, the error SPICE(SPKTYPENOTSUPP) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  A new segment, which contains a subset of the data in the
///  segment specified by DESCR and HANDLE, is written to the SPK
///  file attached to NEWH.
/// ```
///
/// # Particulars
///
/// ```text
///  Sometimes, the segments in official source files---planetary
///  Developmental Ephemeris (DE) files, archival spacecraft
///  ephemeris files, and so on---contain more data than is needed
///  by a particular user. SPKSUB allows a user to extract from a
///  segment the smallest amount of ephemeris data sufficient to
///  cover a specific interval.
///
///  The new segment is written with the same identifier as the
///  original segment, and with the same descriptor, with the
///  following components changed:
///
///  1)  ALPHA and OMEGA (DC(1) and DC(2)) are assigned the values
///      specified by BEGIN and END.
///
///  2)  The beginning and ending segment addresses (IC(5) and IC(6))
///      are changed to reflect the location of the new segment.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, the descriptor for each segment
///  in a source SPK file is examined. For each segment that covers a
///  specified time interval, the smallest possible subset of data
///  from that segment, sufficient to cover the interval, is extracted
///  into a custom SPK file.
///
///  Assume that the source and custom files have been opened, for
///  read and write access, with handles SRC and CUST respectively.
///
///     CALL DAFBFS ( SRC    )
///     CALL DAFFNA ( FOUND  )
///
///     DO WHILE ( FOUND )
///        CALL DAFGS ( DESCR )
///        CALL DAFUS ( DESCR, 2, 6, DC, IC )
///
///        IF ( DC(1) .LE. BEGIN  .AND.  END .LE. DC(2) ) THEN
///           CALL DAFGN  ( IDENT )
///           CALL SPKSUB ( SRC, DESCR, IDENT, BEGIN, END, CUST )
///        END IF
///
///        CALL DAFFNA ( FOUND )
///     END DO
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  There is no way for SPKSUB to verify that the descriptor and
///      identifier are the original ones for the segment. Changing
///      the descriptor can cause the data in the new segment to be
///      evaluated incorrectly; changing the identifier can destroy
///      the path from the data back to its original source.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 9.1.0, 14-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section. Moved SPK required reading from
///         $Literature_References to $Required_Reading section.
///
/// -    SPICELIB Version 9.0.0, 23-DEC-2013 (NJB)
///
///         The routine was updated to handle types 19, 20 and 21. Some
///         minor changes were made to comments.
///
/// -    SPICELIB Version 8.0.0, 12-AUG-2002 (NJB)
///
///         The routine was updated to handle type 18.
///
/// -    SPICELIB Version 7.0.0, 06-NOV-1999 (NJB)
///
///         The routine was updated to handle types 12 and 13.
///
/// -    SPICELIB Version 6.0.0, 30-JUN-1997 (WLT)
///
///         The routine was updated to handle types 10 and 17.
///
/// -    SPICELIB Version 5.0.0, 10-MAR-1995 (KRG)
///
///         The routine was updated to handle type 14.
///
/// -    SPICELIB Version 4.0.0, 07-NOV-1994 (WLT)
///
///         The routine was updated to handle type 15.
///
/// -    SPICELIB Version 3.0.0, 05-AUG-1993 (NJB)
///
///         The routine was updated to handle types 08 and 09.
///
/// -    SPICELIB Version 2.0.0, 01-APR-1992 (JML)
///
///         1) The routine was updated to handle type 05.
///
///         2) DESCR was being used as both an input and output
///            variable when it was only supposed to be used for
///            input. A new local variable, NDSCR, was added where DESCR
///            was being altered.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (RET)
/// ```
pub fn spksub(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    ident: &str,
    begin: f64,
    end: f64,
    newh: i32,
) -> crate::Result<()> {
    SPKSUB(
        handle,
        descr,
        ident.as_bytes(),
        begin,
        end,
        newh,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKSUB ( S/P Kernel, subset )
pub fn SPKSUB(
    HANDLE: i32,
    DESCR: &[f64],
    IDENT: &[u8],
    BEGIN: f64,
    END: f64,
    NEWH: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut ALPHA: f64 = 0.0;
    let mut DC = StackArray::<f64, 2>::new(1..=2);
    let mut NDSCR = StackArray::<f64, 5>::new(1..=5);
    let mut OMEGA: f64 = 0.0;
    let mut BADDR: i32 = 0;
    let mut EADDR: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=6);
    let mut TYPE: i32 = 0;
    let mut OKAY: bool = false;

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
        CHKIN(b"SPKSUB", ctx)?;
    }

    //
    // Unpack the descriptor.
    //
    DAFUS(DESCR.as_slice(), 2, 6, DC.as_slice_mut(), IC.as_slice_mut());

    ALPHA = DC[1];
    OMEGA = DC[2];
    TYPE = IC[4];
    BADDR = IC[5];
    EADDR = IC[6];

    //
    // Make sure the epochs check out.
    //
    OKAY = (((ALPHA <= BEGIN) && (BEGIN <= END)) && (END <= OMEGA));

    if !OKAY {
        SETMSG(
            b"Specified interval [#, #] is not a subset of segment interval [#, #].",
            ctx,
        );
        ERRDP(b"#", BEGIN, ctx);
        ERRDP(b"#", END, ctx);
        ERRDP(b"#", ALPHA, ctx);
        ERRDP(b"#", OMEGA, ctx);
        SIGERR(b"SPICE(SPKNOTASUBSET)", ctx)?;

        CHKOUT(b"SPKSUB", ctx)?;
        return Ok(());
    }

    //
    // Begin the new segment, with a descriptor containing the subset
    // epochs.
    //
    DC[1] = BEGIN;
    DC[2] = END;
    DAFPS(2, 6, DC.as_slice(), IC.as_slice(), NDSCR.as_slice_mut());

    //
    // Let the type-specific (SPKSnn) routines decide what to move.
    //
    if (TYPE == 1) {
        DAFBNA(NEWH, NDSCR.as_slice(), IDENT, ctx)?;
        SPKS01(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;
        DAFENA(ctx)?;
    } else if (TYPE == 2) {
        DAFBNA(NEWH, NDSCR.as_slice(), IDENT, ctx)?;
        SPKS02(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;
        DAFENA(ctx)?;
    } else if (TYPE == 3) {
        DAFBNA(NEWH, NDSCR.as_slice(), IDENT, ctx)?;
        SPKS03(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;
        DAFENA(ctx)?;

    //
    // Type 04 has not been yet been added to SPICELIB.
    //
    // ELSE IF ( TYPE .EQ. 04 ) THEN
    //    CALL DAFBNA ( NEWH, NDSCR,  IDENT )
    //    CALL SPKS04 ( HANDLE, BADDR, EADDR, BEGIN, END )
    //    CALL DAFENA
    } else if (TYPE == 5) {
        DAFBNA(NEWH, NDSCR.as_slice(), IDENT, ctx)?;
        SPKS05(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;
        DAFENA(ctx)?;
    } else if (TYPE == 8) {
        DAFBNA(NEWH, NDSCR.as_slice(), IDENT, ctx)?;
        SPKS08(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;
        DAFENA(ctx)?;
    } else if (TYPE == 9) {
        DAFBNA(NEWH, NDSCR.as_slice(), IDENT, ctx)?;
        SPKS09(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;
        DAFENA(ctx)?;
    } else if (TYPE == 10) {
        SPKS10(HANDLE, DESCR.as_slice(), NEWH, NDSCR.as_slice(), IDENT, ctx)?;
    } else if (TYPE == 12) {
        DAFBNA(NEWH, NDSCR.as_slice(), IDENT, ctx)?;
        SPKS12(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;
        DAFENA(ctx)?;
    } else if (TYPE == 13) {
        DAFBNA(NEWH, NDSCR.as_slice(), IDENT, ctx)?;
        SPKS13(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;
        DAFENA(ctx)?;
    } else if (TYPE == 14) {
        SPKS14(HANDLE, DESCR.as_slice(), NEWH, NDSCR.as_slice(), IDENT, ctx)?;
    } else if (TYPE == 15) {
        DAFBNA(NEWH, NDSCR.as_slice(), IDENT, ctx)?;
        SPKS15(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;
        DAFENA(ctx)?;
    } else if (TYPE == 17) {
        DAFBNA(NEWH, NDSCR.as_slice(), IDENT, ctx)?;
        SPKS17(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;
        DAFENA(ctx)?;
    } else if (TYPE == 18) {
        DAFBNA(NEWH, NDSCR.as_slice(), IDENT, ctx)?;
        SPKS18(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;
        DAFENA(ctx)?;
    } else if (TYPE == 19) {
        DAFBNA(NEWH, NDSCR.as_slice(), IDENT, ctx)?;
        SPKS19(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;
        DAFENA(ctx)?;
    } else if (TYPE == 20) {
        DAFBNA(NEWH, NDSCR.as_slice(), IDENT, ctx)?;
        SPKS20(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;
        DAFENA(ctx)?;
    } else if (TYPE == 21) {
        DAFBNA(NEWH, NDSCR.as_slice(), IDENT, ctx)?;
        SPKS21(HANDLE, BADDR, EADDR, BEGIN, END, ctx)?;
        DAFENA(ctx)?;
    } else {
        SETMSG(b"SPK data type # is not supported.", ctx);
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(SPKTYPENOTSUPP)", ctx)?;

        CHKOUT(b"SPKSUB", ctx)?;
        return Ok(());
    }

    CHKOUT(b"SPKSUB", ctx)?;
    Ok(())
}
