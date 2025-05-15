//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CHAR: i32 = 1;
const NWC: i32 = 1024;

struct SaveVars {
    RECORD: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut RECORD = vec![b' '; NWC as usize];

        fstr::assign(&mut RECORD, b" ");

        Self { RECORD }
    }
}

/// DAS, add data, character
///
/// Add character data to a DAS file.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   DAS file handle.
///  N          I   Number of characters to add to file.
///  BPOS,
///  EPOS       I   Begin and end positions of substrings.
///  DATA       I   Array providing the set of substrings to be added
///                 to the character data in the DAS file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is a file handle of a DAS file opened for writing.
///
///  N        is the total number of characters to add to the specified
///           DAS file.
///
///  BPOS,
///  EPOS     are the begin and end character positions that define the
///           substrings in each of the elements of the input array.
///           This routine writes the first N characters from the
///           specified set of substrings to the specified DAS file.
///
///  DATA     is an array of strings, some portion of whose contents
///           are to be added to the specified DAS file. Specifically,
///           the first N characters of the substrings
///
///              DATA(I)(BPOS:EPOS),    I = 1, ...
///
///           are appended to the character data in the file.
///
///           DATA must be declared at least as
///
///              CHARACTER*(EPOS)      DATA   ( R )
///
///           with the dimension R being at least
///
///              R = INT( ( N + SUBLEN - 1 ) / SUBLEN )
///
///           and SUBLEN, the length of each of the substrings in
///           the array to be added to the DAS file, being
///
///              SUBLEN  =  EPOS - BPOS + 1
///
///           The order of characters in the input substrings is
///           considered to increase from left to right within each
///           element of DATA, and to increase with the indices of the
///           elements of DATA.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None.
///
///  See $Particulars for a description of the effect of this routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input file handle is invalid, an error is signaled
///      by a routine in the call tree of this routine.
///
///  2)  If EPOS or BPOS are outside of the range
///
///         [  1,  LEN( DATA(1) )  ]
///
///      or if EPOS < BPOS, the error SPICE(BADSUBSTRINGBOUNDS) is
///      signaled.
///
///  3)  If the input count N is less than 1, no data will be
///      added to the specified DAS file.
///
///  4)  If an I/O error occurs during the data addition attempted
///      by this routine, the error is signaled by a routine in the
///      call tree of this routine.
///
///  5)  If N is greater than the number of characters in the
///      specified set of input substrings, the results of calling
///      this routine are unpredictable. This routine cannot
///      detect this error.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  DAS is a low-level format meant to store and transmit data. As
///  such, character data in DAS files are not interpreted by SPICELIB
///  DAS input or output routines. There are no limits on which
///  character values may be placed in the virtual character array of a
///  DAS file.
///
///  This routine adds character data to a DAS file by "appending" them
///  after any character data already in the file. The sense in which
///  the data are "appended" is that the data will occupy a range of
///  logical addresses for character data that immediately follow the
///  last logical address of a character that is occupied at the time
///  this routine is called. The diagram below illustrates this
///  addition:
///
///     +-------------------------+
///     |    (already in use)     |  Character logical address 1
///     +-------------------------+
///                 .
///                 .
///                 .
///     +-------------------------+  Last character logical address
///     |   (already in use)      |  in use before call to DASADC
///     +-------------------------+
///     |  DATA(1)(BPOS:BPOS)     |  First added character
///     +-------------------------+
///     |  DATA(1)(BPOS+1:BPOS+1) |
///     +-------------------------+
///                  .
///                  .
///                  .
///     +-------------------------+
///     |  DATA(1)(EPOS:EPOS)     |
///     +-------------------------+
///     |  DATA(2)(BPOS:BPOS)     |
///     +-------------------------+
///                  .
///                  .
///                  .
///     +-------------------------+
///     |  DATA(R)(C:C)           |  N'th added character---here R is
///     +-------------------------+
///                                     INT( (N+L-1)/L )
///
///                                  where L = EPOS - BPOS + 1, and
///                                  C is
///
///                                     BPOS + ( N - (R-1)*L ) - 1
///
///
///  The logical organization of the characters in the DAS file is
///  independent of the order of addition to the file or physical
///  location of any data of integer or double precision type.
///
///  The actual physical write operations that add the input array
///  DATA to the indicated DAS file may not take place before this
///  routine returns, since the DAS system buffers data that are
///  written as well as data that are read. In any case, the data
///  will be flushed to the file at the time the file is closed, if
///  not earlier. A physical write of all buffered records can be
///  forced by calling the SPICELIB routine DASWBR (DAS, write
///  buffered records).
///
///  In order to update character logical addresses that already
///  contain data, the SPICELIB routine DASUDC (DAS, update data,
///  character) should be used.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) The following example demonstrates the capabilities of the
///     DAS character data routines. The reader should notice that
///     in these interfaces, the character data are treated not as
///     strings (or arrays of strings) but as a stream of single
///     characters: DAS character data are not limited to
///     human-readable text. For example, one can store images or
///     DEM data as DAS character data.
///
///     The example shows how to add a variable amount of character
///     data to a new DAS file, how to update some of the character
///     logical addresses within that file, and how to read that
///     data out to a different array.
///
///
///     Example code begins here.
///
///
///           PROGRAM DASADC_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         FNAME
///           PARAMETER           ( FNAME = 'dasadc_ex1.das' )
///
///           CHARACTER*(*)         TYPE
///           PARAMETER           ( TYPE  = 'TEST'           )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(22)        CDATIN ( 3  )
///           CHARACTER*(30)        CDATOU ( 10 )
///
///           INTEGER               HANDLE
///           INTEGER               I
///
///           DATA CDATOU  / '..............................',
///          .               '..............................',
///          .               '..............................',
///          .               '..............................',
///          .               '..............................',
///          .               '..............................',
///          .               '..............................',
///          .               '..............................',
///          .               '         1         2         3',
///          .               '123456789012345678901234567890' /
///
///     C
///     C     Open a new DAS file. Use the file name as the internal
///     C     file name, and reserve no records for comments.
///     C
///           CALL DASONW ( FNAME, TYPE, FNAME, 0, HANDLE )
///
///     C
///     C     Set the input data. Note that these data will be
///     C     considered as a binary data stream: DAS character data
///     C     are not limited to human-readable text. For example,
///     C     one can store images or DEM data as DAS character data.
///     C
///           CDATIN ( 1 ) = '--F-345678901234567890'
///           CDATIN ( 2 ) = '--S-345678901234567890'
///           CDATIN ( 3 ) = '--T-IRDxxxxxxxxxxxxxxx'
///
///     C
///     C     Add the last 20 characters of the first two elements
///     C     of CDATIN, and the 3rd character from the third one.
///     C
///           CALL DASADC ( HANDLE, 41, 3, 22, CDATIN )
///
///     C
///     C     Update the 10th, 20th and 30th character in the DAS
///     C     file with a vertical bar.
///     C
///           DO I = 1, 3
///
///              CALL DASUDC ( HANDLE, I*10, I*10, 1, 1, '|' )
///
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL DASCLS ( HANDLE )
///
///     C
///     C     Now verify the addition of data by opening the
///     C     file for read access and retrieving the data.
///     C
///           CALL DASOPR ( FNAME, HANDLE )
///
///     C
///     C     Read the 41 characters that we stored on the DAS
///     C     file. Update the data on the CDATOU array, placing
///     C     6 characters on each element, starting from the
///     C     10th position.
///     C
///           CALL DASRDC ( HANDLE, 1, 41, 10, 15, CDATOU )
///
///     C
///     C     Dump the data to the screen. Note that the last
///     C     three lines should remain unmodified, and that
///     C     only 5 characters will be written on the 7th line.
///     C
///           WRITE (*,*)
///           WRITE (*,*) 'Data from "', FNAME, '":'
///           WRITE (*,*)
///
///           DO I = 1, 10
///              WRITE (*,*) CDATOU(I)
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL DASCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Data from "dasadc_ex1.das":
///
///      .........F-3456...............
///      .........789|12...............
///      .........345678...............
///      .........9|S-34...............
///      .........56789|...............
///      .........123456...............
///      .........7890T................
///      ..............................
///               1         2         3
///      123456789012345678901234567890
///
///
///     Note that after run completion, a new DAS file exists in the
///     output directory.
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
/// -    SPICELIB Version 1.3.0, 08-OCT-2021 (JDR) (NJB)
///
///         Added IMPLICIT NONE statement. Updated the code to avoid that
///         DASCUD is called with a negative number of character words
///         when the input count N is negative.
///
///         Made local variable RECORD a saved variable which is
///         initialized by a DATA statement.
///
///         Bug fix: added FAILED call after DASHFS call.
///
///         Edited the header to comply with NAIF standard.
///
///         Replaced example code with one that demonstrates the usage and
///         effect of all DAS character data routines.
///
///         Updated entries in the $Revisions section.
///
/// -    SPICELIB Version 1.2.0, 10-APR-2014 (NJB)
///
///         Deleted declarations of unused parameters.
///
///         Corrected header comments: routine that flushes
///         written, buffered records is DASWBR, not DASWUR.
///
/// -    SPICELIB Version 1.1.1, 19-DEC-1995 (NJB)
///
///         Corrected title of permuted index entry section.
///
/// -    SPICELIB Version 1.1.0, 12-MAY-1994 (KRG) (NJB)
///
///         Test of FAILED() added to loop termination condition.
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
///         Modified the $Examples section to demonstrate the new ID word
///         format which includes a file type and to include a call to the
///         new routine DASONW, open new, which makes use of the file
///         type. Also, a variable for the type of the file to be created
///         was added.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-MAY-1994 (KRG) (NJB)
///
///         Test of FAILED() added to loop termination condition. Without
///         this test, an infinite loop could result if DASA2L, DASURC or
///         DASWRC signaled an error inside the loop.
/// ```
pub fn dasadc(
    ctx: &mut SpiceContext,
    handle: i32,
    n: i32,
    bpos: i32,
    epos: i32,
    data: CharArray,
) -> crate::Result<()> {
    DASADC(handle, n, bpos, epos, data, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASADC ( DAS, add data, character )
pub fn DASADC(
    HANDLE: i32,
    N: i32,
    BPOS: i32,
    EPOS: i32,
    DATA: CharArray,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DATA = DummyCharArray::new(DATA, None, 1..);
    let mut CHR: i32 = 0;
    let mut CLBASE: i32 = 0;
    let mut CLSIZE: i32 = 0;
    let mut ELT: i32 = 0;
    let mut FREE: i32 = 0;
    let mut LASTC: i32 = 0;
    let mut LASTLA = StackArray::<i32, 3>::new(1..=3);
    let mut LASTRC = StackArray::<i32, 3>::new(1..=3);
    let mut LASTWD = StackArray::<i32, 3>::new(1..=3);
    let mut NCOMC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NMOVE: i32 = 0;
    let mut NMOVED: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NRESVR: i32 = 0;
    let mut NUMCHR: i32 = 0;
    let mut NWRITN: i32 = 0;
    let mut RCPOS: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut WORDNO: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }
    CHKIN(b"DASADC", ctx)?;

    //
    // Make sure BPOS and EPOS are OK; stop here if not.
    //
    if ((((BPOS < 1) || (EPOS < 1)) || (BPOS > intrinsics::LEN(&DATA[1])))
        || (EPOS > intrinsics::LEN(&DATA[1])))
    {
        SETMSG(
            b"Substring bounds must be in range [1,#]. Actual range [BPOS,EPOS] was [#,#].",
            ctx,
        );
        ERRINT(b"#", intrinsics::LEN(&DATA[1]), ctx);
        ERRINT(b"#", BPOS, ctx);
        ERRINT(b"#", EPOS, ctx);
        SIGERR(b"SPICE(BADSUBSTRINGBOUNDS)", ctx)?;
        CHKOUT(b"DASADC", ctx)?;
        return Ok(());
    } else if (EPOS < BPOS) {
        SETMSG(b"Substring upper bound must not be less than lower bound.  Actual range [BPOS,EPOS] was [#,#].", ctx);
        ERRINT(b"#", BPOS, ctx);
        ERRINT(b"#", EPOS, ctx);
        SIGERR(b"SPICE(BADSUBSTRINGBOUNDS)", ctx)?;
        CHKOUT(b"DASADC", ctx)?;
        return Ok(());
    }

    //
    // Get the file summary for this DAS.
    //
    DASHFS(
        HANDLE,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"DASADC", ctx)?;
        return Ok(());
    }

    LASTC = LASTLA[CHAR];

    //
    // We will keep track of the location that we wish to write to
    // with the variables RECNO and WORDNO.  RECNO will be the record
    // number of the record we'll write to; WORDNO will be the number
    // preceding the word index, within record number RECNO, that we'll
    // write to.  For example, if we're about to write to the first
    // character in record 10, RECNO will be 10 and WORDNO will be 0.  Of
    // course, when WORDNO reaches NWC, we'll have to find a free record
    // before writing anything.
    //
    // Prepare the variables RECNO and WORDNO:  use the physical location
    // of the last character address, if there are any character data in
    // the file.  Otherwise, RECNO becomes the first record available for
    // character data.
    //
    if (LASTC >= 1) {
        DASA2L(
            HANDLE,
            CHAR,
            LASTC,
            &mut CLBASE,
            &mut CLSIZE,
            &mut RECNO,
            &mut WORDNO,
            ctx,
        )?;
    } else {
        RECNO = FREE;
        WORDNO = 0;
    }

    //
    // Set the number of character words already written.  Keep
    // writing to the file until this number equals the number of
    // elements in DATA.
    //
    // Note that if N is non-positive, the loop doesn't get
    // exercised.
    //
    // Also initialize the array element index and position of the
    // character to be moved next.
    //
    NWRITN = 0;
    ELT = 1;
    CHR = BPOS;

    while ((NWRITN < N) && !FAILED(ctx)) {
        //
        // Write as much data as we can (or need to) into the current
        // record.  We assume that RECNO, WORDNO, and NWRITN have
        // been set correctly at this point.
        //
        // Find out how many words to write into the current record.
        // There may be no space left in the current record.
        //
        NUMCHR = intrinsics::MIN0(&[(N - NWRITN), (NWC - WORDNO)]);

        if (NUMCHR > 0) {
            //
            // Write NUMCHR words into the current record.  If the record
            // is new, write the entire record.  Otherwise, just update
            // the part we're interested in.
            //
            // In either case, we'll first fill in characters WORDNO+1
            // through WORDNO + NUMCHR of the string RECORD.
            //
            //
            // So far, we haven't moved any characters.
            //
            NMOVED = 0;
            RCPOS = WORDNO;

            while (NMOVED < NUMCHR) {
                //
                // Find out how many characters in the current array
                // element we should move.
                //
                if (CHR > EPOS) {
                    ELT = (ELT + 1);
                    CHR = BPOS;
                }

                NMOVE = intrinsics::MIN0(&[(NUMCHR - NMOVED), ((EPOS - CHR) + 1)]);

                fstr::assign(
                    fstr::substr_mut(&mut save.RECORD, (RCPOS + 1)..=(RCPOS + NMOVE)),
                    fstr::substr(DATA.get(ELT), CHR..),
                );

                NMOVED = (NMOVED + NMOVE);
                RCPOS = (RCPOS + NMOVE);
                CHR = (CHR + NMOVE);
            }

            //
            // Now we can write or update the file with RECORD.
            //
            if (WORDNO == 0) {
                //
                // The record has not yet been written, so write out the
                // entire record.
                //
                DASWRC(HANDLE, RECNO, &save.RECORD, ctx)?;
            } else {
                //
                // Update elements WORDNO+1 through WORDNO+NUMCHR.
                //
                DASURC(
                    HANDLE,
                    RECNO,
                    (WORDNO + 1),
                    (WORDNO + NUMCHR),
                    fstr::substr(&save.RECORD, (WORDNO + 1)..=(WORDNO + NUMCHR)),
                    ctx,
                )?;
            }

            NWRITN = (NWRITN + NUMCHR);
            WORDNO = (WORDNO + NUMCHR);
        } else {
            //
            // It's time to start on a new record.  If the record we
            // just finished writing to (or just attempted writing to,
            // if it was full) was FREE or a higher-numbered record,
            // then we are writing to a contiguous set of data records:
            // the next record to write to is the immediate successor
            // of the last one.  Otherwise, FREE is the next record
            // to write to.
            //
            // We intentionally leave FREE at the value it had before
            // we starting adding data to the file.
            //
            if (RECNO >= FREE) {
                RECNO = (RECNO + 1);
            } else {
                RECNO = FREE;
            }

            WORDNO = 0;
        }
    }

    //
    // Update the DAS file directories to reflect the addition of NWRITN
    // character words.  DASCUD will also update the file summary
    // accordingly.
    //
    DASCUD(HANDLE, CHAR, NWRITN, ctx)?;

    CHKOUT(b"DASADC", ctx)?;
    Ok(())
}
