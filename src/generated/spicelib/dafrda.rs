//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BSIZE: i32 = 128;

/// DAF, read data from address
///
/// Deprecated: This routine has been superseded by the SPICELIB
/// routines DAFGDA and DAFGSR. This routine is supported for purposes
/// of backward compatibility only.
///
/// Read the double precision data bounded by two addresses within
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
///  HANDLE     I   Handle of a DAF.
///  BEGIN,
///  END        I   Initial, final address within file.
///  DATA       O   Data contained between BEGIN and END.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF.
///
///  BEGIN,
///  END      are the initial and final addresses of a contiguous
///           set of double precision numbers within a DAF.
///           Presumably, these make up all or part of a particular
///           array.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DATA     are the double precision data contained between
///           the specified addresses within the specified file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If BEGIN is zero or negative, the error SPICE(DAFNEGADDR)
///      is signaled.
///
///  2)  If BEGIN > END, the error SPICE(DAFBEGGTEND) is signaled.
///
///  3)  If the file associated with HANDLE is not of the native
///      binary file format, the error SPICE(UNSUPPORTEDBFF) is
///      signaled.
///
///  4)  If HANDLE is invalid, an error is signaled by a routine in
///      the call tree of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  The principal reason that DAFs are so easy to use is that
///  the data in each DAF are considered to be one long contiguous
///  set of double precision numbers. You can grab data from anywhere
///  within a DAF without knowing (or caring) about the physical
///  records in which they are stored.
///
///  This routine has been made obsolete by the routines DAFGDA and
///  DAFGSR. This routine is supported for reasons of backward
///  compatibility only. New software development should utilize
///  DAFGDA or DAFGSR.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates the use of DAFRDA
///  to read data from an imaginary array. The array begins with a
///  directory containing 11 epochs. Each pair of epochs bounds
///  an interval, and each interval is covered by a set of eight
///  osculating elements.
///
///     CALL DAFUS ( SUM, ND, NI, DC, IC )
///     BEGIN = IC(5)
///     END   = IC(6)
///
///     CALL DAFRDA ( HANDLE, BEGIN, BEGIN+10, EPOCHS )
///
///     DO I = 1, 10
///        IF ( ET .GE. EPOCHS(I)  .AND.  ET .LE. EPOCHS(I+1) ) THEN
///           OFFSET = IC(5) + 11 + (I - 1) * 8
///
///           CALL DAFRDA ( HANDLE, OFFSET+1, OFFSET+8, ELEMENTS )
///           RETURN
///        END IF
///     END DO
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
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.2, 18-MAY-2010 (BVS)
///
///         Index line now states that this routine is deprecated.
///
/// -    SPICELIB Version 2.0.1, 27-OCT-2003 (NJB)
///
///         The header now states that this routine is deprecated.
///         The $Exceptions header section has been extended.
///         Minor additional header updates were made.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         Added SPICE(UNSUPPORTEDBFF) exception to the routine.
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
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         The exception SPICE(UNSUPPORTEDBFF) was added to guarantee
///         this routine's functionality remains unchanged as a result
///         of the updates to the underlying DAF software's utilization of
///         the handle manager. In versions of the Toolkit prior to this,
///         all DAFs loaded were of the native binary file format.
///         While rather unlikely, this routine could be used to read
///         the contents of summary records in addition to the usual
///         data records. The non-native to native translation process
///         for these two different types of records in general are not
///         the same. Rather than attempt to interpret the caller's
///         intent, this routine is deprecated and restricted to
///         functioning only on DAFs of the native binary file format.
///
/// -    Beta Version 1.1.0, 1-NOV-1989 (RET)
///
///         DAFRDA now only checks in and checks out if one of the two
///         possible exceptions occurs. The purpose of this change was to
///         help speed up a routine that gets called constantly by higher
///         level DAF routines.
/// ```
pub fn dafrda(
    ctx: &mut SpiceContext,
    handle: i32,
    begin: i32,
    end: i32,
    data: &mut [f64],
) -> crate::Result<()> {
    DAFRDA(handle, begin, end, data, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFRDA ( DAF, read data from address )
pub fn DAFRDA(
    HANDLE: i32,
    BEGIN: i32,
    END: i32,
    DATA: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DATA = DummyArrayMut::new(DATA, 1..);
    let mut BEGR: i32 = 0;
    let mut BEGW: i32 = 0;
    let mut ENDR: i32 = 0;
    let mut ENDW: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut LAST: i32 = 0;
    let mut NEXT: i32 = 0;
    let mut FOUND: bool = false;
    let mut NATIVE: bool = false;

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
    }

    //
    // Check to see if HANDLE is associated with a DAF of the native
    // binary file format.
    //
    ZZDDHISN(HANDLE, &mut NATIVE, &mut FOUND, ctx)?;

    //
    // If the HANDLE was located, then check whether the binary file
    // format is native.  Otherwise, defer diagnosing the missing
    // handle to DAFRDR.
    //
    if (FOUND && !NATIVE) {
        CHKIN(b"DAFRDA", ctx)?;
        SETMSG(b"The binary file format for file \'#\' is not native. This routine operates only on files of the native format.", ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(UNSUPPORTEDBFF)", ctx)?;
        CHKOUT(b"DAFRDA", ctx)?;
        return Ok(());
    }

    //
    // Bad addresses?
    //
    if (BEGIN <= 0) {
        CHKIN(b"DAFRDA", ctx)?;
        SETMSG(b"Negative value for BEGIN address: #", ctx);
        ERRINT(b"#", BEGIN, ctx);
        SIGERR(b"SPICE(DAFNEGADDR)", ctx)?;
        CHKOUT(b"DAFRDA", ctx)?;
        return Ok(());
    } else if (BEGIN > END) {
        CHKIN(b"DAFRDA", ctx)?;
        SETMSG(
            b"Beginning address (#) greater than ending address (#).",
            ctx,
        );
        ERRINT(b"#", BEGIN, ctx);
        ERRINT(b"#", END, ctx);
        SIGERR(b"SPICE(DAFBEGGTEND)", ctx)?;
        CHKOUT(b"DAFRDA", ctx)?;
        return Ok(());
    }

    //
    // Convert raw addresses to record/word representations.
    //
    DAFARW(BEGIN, &mut BEGR, &mut BEGW, ctx)?;
    DAFARW(END, &mut ENDR, &mut ENDW, ctx)?;

    //
    // Get as many records as needed. Return the last part of the
    // first record, the first part of the last record, and all of
    // every record in between. Any record not found is assumed to
    // be filled with zeros.
    //

    NEXT = 1;

    for RECNO in BEGR..=ENDR {
        if (BEGR == ENDR) {
            FIRST = BEGW;
            LAST = ENDW;
        } else if (RECNO == BEGR) {
            FIRST = BEGW;
            LAST = BSIZE;
        } else if (RECNO == ENDR) {
            FIRST = 1;
            LAST = ENDW;
        } else {
            FIRST = 1;
            LAST = BSIZE;
        }

        DAFRDR(
            HANDLE,
            RECNO,
            FIRST,
            LAST,
            DATA.subarray_mut(NEXT),
            &mut FOUND,
            ctx,
        )?;

        if !FOUND {
            CLEARD(((LAST - FIRST) + 1), DATA.subarray_mut(NEXT));
        }

        NEXT = (NEXT + ((LAST - FIRST) + 1));
    }

    Ok(())
}
