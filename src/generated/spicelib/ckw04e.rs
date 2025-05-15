//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 6;
const NDESCR: i32 = (ND + ((NI + 1) / 2));

/// CK type 04: End a segment
///
/// End the type 04 CK segment currently being written to the DAF
/// file associated with HANDLE. See also CKW04B and CKW04E.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   The handle of an CK file open for writing.
///  ENDTIM     I   The segment coverage end encoded SCLK time.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of an CK file that has been
///           opened for writing, and to which a type 4 CK segment
///           is being written.
///
///  ENDTIM   is the encoded SCLK time for the end of the segment
///           coverage.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None.
///
///  The type 04 segment in the DAF file associated with HANDLE will be
///  ended, making the addition of the data to the file permanent.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an error occurs while reading or writing the file
///      indicated by HANDLE, the error is signaled by a routine in
///      the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  See the argument HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine ends a type 4 CK segment which is being written to
///  the DAF file associated with HANDLE. Ending the DAF segment is a
///  necessary step in the process of making the data a permanent part
///  of the DAF file.
///
///  This routine is one of a set of three routines for creating and
///  adding data to type 4 CK segments. These routines are:
///
///     CKW04B: Begin a type 4 CK segment. This routine must be
///             called before any data may be added to a type 4
///             segment.
///
///     CKW04A: Add data to a type 4 CK segment. This routine may be
///             called any number of times after a call to CKW04B to
///             add type 4 records to the CK segment that was
///             started.
///
///     CKW04E: End a type 4 CK segment. This routine is called to
///             make the type 4 segment a permanent addition to the
///             DAF file. Once this routine is called, no further type
///             4 records may be added to the segment. A new segment
///             must be started.
///
///  A type 4 CK segment consists of coefficient sets for variable
///  order Chebyshev polynomials over consecutive time intervals of
///  a variable length. The gaps between intervals are allowed.
///  The Chebyshev polynomials represent individual quaternion
///  components q0, q1, q2 and q3 and individual angular velocities
///  AV1, AV2 and AV3 if they are included with the data.
///
///  The pointing data supplied to the type 4 CK writer (CKW04A)
///  is packed into an array as a sequence of records,
///
///     ----------------------------------------------------
///     | Record 1 | Record 2 | .. | Record N-1 | Record N |
///     ----------------------------------------------------
///
///  with each record in data packets has the following format.
///
///     ----------------------------------------------------
///     | The midpoint of the approximation interval       |
///     ----------------------------------------------------
///     | The radius of the approximation interval         |
///     ----------------------------------------------------
///     | Number of coefficients for q0                    |
///     ----------------------------------------------------
///     | Number of coefficients for q1                    |
///     ----------------------------------------------------
///     | Number of coefficients for q2                    |
///     ----------------------------------------------------
///     | Number of coefficients for q3                    |
///     ----------------------------------------------------
///     | Number of coefficients for AV1                   |
///     ----------------------------------------------------
///     | Number of coefficients for AV2                   |
///     ----------------------------------------------------
///     | Number of coefficients for AV3                   |
///     ----------------------------------------------------
///     | q0 Cheby coefficients                            |
///     ----------------------------------------------------
///     | q1 Cheby coefficients                            |
///     ----------------------------------------------------
///     | q2 Cheby coefficients                            |
///     ----------------------------------------------------
///     | q3 Cheby coefficients                            |
///     ----------------------------------------------------
///     | AV1 Cheby coefficients (optional)                |
///     ----------------------------------------------------
///     | AV2 Cheby coefficients (optional)                |
///     ----------------------------------------------------
///     | AV3 Cheby coefficients (optional)                |
///     ----------------------------------------------------
/// ```
///
/// # Examples
///
/// ```text
///  Assume that we have:
///
///     HANDLE   is the handle of an CK file opened with write
///              access.
///
///     SEGID    is a character string of no more than 40 characters
///              which provides a pedigree for the data in the CK
///              segment we will create.
///
///     INST     is the SPICE ID code for the instrument whose
///              pointing data is to be placed into the file.
///
///     AVFLAG   angular rates flag.
///
///     REFFRM   is the name of the SPICE reference frame for the
///              pointing data.
///
///     BEGTIM   is the starting encoded SCLK time for which the
///              segment is valid.
///
///     ENDTIM   is the ending encoded SCLK time for which the segment
///              is valid.
///
///     N        is the number of type 4 records that we want to
///              put into a segment in an CK file.
///
///     NPKTS    is integer array which contains the lengths of
///              variable size data packets
///
///     RECRDS   contains N type 4 records packaged for the CK
///              file.
///
///     SCSTRT   contains the initial encoded SC time for each of
///              the records contained in RECRDS, where
///
///                 SCSTRT(I) < SCSTRT(I+1), I = 1, N-1
///
///                 SCSTRT(1) <= FIRST, SCSTRT(N) < LAST
///
///  Then the following code fragment demonstrates how to create
///  a type 4 CK segment if all of the data for the segment is
///  available at one time.
///
///  C
///  C     Begin the segment.
///  C
///        CALL CKW04B ( HANDLE, BEGTIM, INST, REF, AVFLAG, SEGID )
///  C
///  C     Add the data to the segment all at once.
///  C
///        CALL CKW04A ( HANDLE, N, NPKTS, RECRDS, SCSTRT )
///  C
///  C     End the segment, making the segment a permanent
///  C     addition to the CK file.
///  C
///        CALL CKW04E ( HANDLE, ENDTIM )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The type 4 CK segment being closed must have been started by
///      the routine CKW04B, the routine which begins a type 4 CK
///      segment.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  Y.K. Zaiko         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.2, 02-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 18-APR-2014 (BVS)
///
///         Minor header edits.
///
/// -    SPICELIB Version 1.0.0, 05-MAY-1999 (YKZ) (BVS)
/// ```
pub fn ckw04e(ctx: &mut SpiceContext, handle: i32, endtim: f64) -> crate::Result<()> {
    CKW04E(handle, endtim, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKW04E ( CK type 04: End a segment )
pub fn CKW04E(HANDLE: i32, ENDTIM: f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DCD = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=NDESCR);
    let mut ICD = StackArray::<i32, 6>::new(1..=NI);
    let mut FOUND: bool = false;

    //
    // SPICELIB functions.
    //

    //
    // Local parameters.
    //

    //
    // DAF ND and NI values for CK files and length of a DAF descriptor.
    //

    //
    // Local variables.
    //

    //
    // Standard SPICELIB error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"CKW04E", ctx)?;
    }

    //
    // This is simple, just call the routine which ends a generic
    // segment.
    //
    SGWES(HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"CKW04E", ctx)?;
        return Ok(());
    }

    //
    // Now update the descriptor with the end time. Locate the segment
    // with a backward search.
    //
    DAFBBS(HANDLE, ctx)?;
    DAFFPA(&mut FOUND, ctx)?;

    if !FOUND {
        //
        // We have a bug.
        //
        SETMSG(b"The segment which was just written could not be found by a DAF search. This  indicates a serious error.  Contact NAIF.", ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"CKW04E", ctx)?;
        return Ok(());
    }

    //
    // Get the descriptor, set the end time, and update the descriptor
    // in the file.
    //
    DAFGS(DESCR.as_slice_mut(), ctx)?;

    DAFUS(
        DESCR.as_slice(),
        ND,
        NI,
        DCD.as_slice_mut(),
        ICD.as_slice_mut(),
    );

    DCD[2] = ENDTIM;

    DAFPS(ND, NI, DCD.as_slice(), ICD.as_slice(), DESCR.as_slice_mut());

    DAFRS(DESCR.as_slice(), ctx)?;

    //
    // All done.
    //
    CHKOUT(b"CKW04E", ctx)?;

    Ok(())
}
