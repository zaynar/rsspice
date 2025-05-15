//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// DAF, record/word to address
///
/// Convert a record/word pair to its equivalent address within
/// a DAF.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  RECNO,
///  WORDNO     I   Record, word numbers of a location within DAF.
///  ADDR       O   Corresponding address.
/// ```
///
/// # Detailed Input
///
/// ```text
///  RECNO,
///  WORDNO   are the record and word numbers of an arbitrary
///           location within a DAF.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ADDR     is the corresponding address within the DAF.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If either RECNO or WORDNO is zero or negative, the error
///      SPICE(DAFNOSUCHADDR) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  To the user, the data in a DAF appear to be a contiguous
///  collection of double precision numbers, each of which has an
///  address. To the DAF software, however, the data appear to be
///  a collection of records, each containing 128 double precision
///  words. The routines DAFARW and DAFRWA translate between these
///  two representations.
/// ```
///
/// # Examples
///
/// ```text
///  Routines DAFRDA and DAFWDA illustrate the use of DAFARW and
///  DAFRWA.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 14-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Moved DAF
///         required reading from $Literature_References to
///         $Required_Reading section.
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
pub fn dafrwa(
    ctx: &mut SpiceContext,
    recno: i32,
    wordno: i32,
    addr: &mut i32,
) -> crate::Result<()> {
    DAFRWA(recno, wordno, addr, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFRWA ( DAF, record/word to address )
pub fn DAFRWA(
    RECNO: i32,
    WORDNO: i32,
    ADDR: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else if ((RECNO <= 0) || (WORDNO <= 0)) {
        CHKIN(b"DAFRWA", ctx)?;
        SETMSG(b"No address for record #, word #.", ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", WORDNO, ctx);
        SIGERR(b"SPICE(DAFNOSUCHADDR)", ctx)?;
        CHKOUT(b"DAFRWA", ctx)?;
        return Ok(());
    }

    //
    // If the record and word numbers are legal, the computation is
    // straightforward.
    //
    *ADDR = (WORDNO + ((RECNO - 1) * 128));

    Ok(())
}

/// DAF, address to record/word
///
/// Convert an address within a DAF to its equivalent
/// record/word representation.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ADDR       I   Address within DAF.
///  RECNO,
///  WORDNO     O   Corresponding record, word numbers.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ADDR     is an arbitrary address within a DAF.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECNO,
///  WORDNO   are the corresponding record and word numbers
///           within the DAF.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If ADDR is zero or negative, the error SPICE(DAFNOSUCHADDR)
///      is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  To the user, the data in a DAF appear to be a contiguous
///  collection of double precision numbers, each of which has an
///  address. To the DAF software, however, the data appear to be
///  a collection of records, each containing 128 double precision
///  words. The routines DAFARW and DAFRWA translate between these
///  two representations.
/// ```
///
/// # Examples
///
/// ```text
///  Routines DAFRDA and DAFWDA illustrate the use of DAFARW and
///  DAFRWA.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 14-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Moved DAF
///         required reading from $Literature_References to
///         $Required_Reading section.
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
pub fn dafarw(
    ctx: &mut SpiceContext,
    addr: i32,
    recno: &mut i32,
    wordno: &mut i32,
) -> crate::Result<()> {
    DAFARW(addr, recno, wordno, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFARW ( DAF, address to record/word )
pub fn DAFARW(
    ADDR: i32,
    RECNO: &mut i32,
    WORDNO: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else if (ADDR <= 0) {
        CHKIN(b"DAFARW", ctx)?;
        SETMSG(b"No record, word for address #.", ctx);
        ERRINT(b"#", ADDR, ctx);
        SIGERR(b"SPICE(DAFNOSUCHADDR)", ctx)?;
        CHKOUT(b"DAFARW", ctx)?;
        return Ok(());
    }

    //
    // If the address is legal, the computation is straightforward.
    //
    *RECNO = (((ADDR - 1) / 128) + 1);
    *WORDNO = (ADDR - ((*RECNO - 1) * 128));

    Ok(())
}
