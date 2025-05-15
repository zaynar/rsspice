//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const DP: i32 = 2;
const NWD: i32 = 128;

struct SaveVars {
    RECORD: StackArray<f64, 128>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut RECORD = StackArray::<f64, 128>::new(1..=NWD);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), NWD as usize))
                .chain([]);

            RECORD
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { RECORD }
    }
}

/// DAS, add data, double precision
///
/// Add an array of double precision numbers to a DAS file.
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
///  N          I   Number of d.p. numbers to add to DAS file.
///  DATA       I   Array of d.p. numbers to add.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is a file handle of a DAS file opened for writing.
///
///  N        is the number of double precision "words" to add to the
///           DAS file specified by HANDLE.
///
///  DATA     is an array of double precision numbers to be added to
///           the specified DAS file. Elements 1 through N are appended
///           to the double precision data in the file.
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
///  1)  If the input file handle is invalid, an error is signaled by a
///      routine in the call tree of this routine.
///
///  2)  If an I/O error occurs during the data addition attempted by
///      this routine, the error is signaled by a routine in the call
///      tree of this routine.
///
///  3)  If the input count N is less than 1, no data will be added to
///      the specified DAS file. No error will be signaled.
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
///  This routine adds double precision data to a DAS file by
///  "appending" them after any double precision data already in the
///  file. The sense in which the data are "appended" is that the
///  data will occupy a range of logical addresses for double precision
///  data that immediately follow the last logical address of a double
///  precision number that is occupied at the time this routine is
///  called. The diagram below illustrates this addition:
///
///     +-------------------------+
///     |    (already in use)     |  D.p. logical address 1
///     +-------------------------+
///                 .
///                 .
///                 .
///     +-------------------------+
///     |    (already in use)     |  Last d.p. logical address
///     +-------------------------+  in use before call to DASADD
///     |        DATA(1)          |
///     +-------------------------+
///                 .
///                 .
///                 .
///     +-------------------------+
///     |        DATA(N)          |
///     +-------------------------+
///
///
///  The logical organization of the double precision numbers in the
///  DAS file is independent of the location in the file of any data
///  of integer or character type.
///
///  The actual physical write operations that add the input array
///  DATA to the indicated DAS file might not take place before this
///  routine returns, since the DAS system buffers data that are
///  written as well as data that are read. In any case, the data
///  will be flushed to the file at the time the file is closed, if
///  not earlier. A physical write of all buffered records can be
///  forced by calling the SPICELIB routine DASWBR (DAS, write
///  buffered records).
///
///  In order to update double precision logical addresses that
///  already contain data, the SPICELIB routine DASUDD
///  (DAS update data, double precision) should be used.
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
///  1) Create a new DAS file and add 200 double precision numbers
///     to it. Close the file, then re-open it and read the data back
///     out.
///
///
///     Example code begins here.
///
///
///           PROGRAM DASADD_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         FNAME
///           PARAMETER           ( FNAME = 'dasadd_ex1.das' )
///
///           CHARACTER*(*)         TYPE
///           PARAMETER           ( TYPE  = 'TEST' )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      DATA   ( 200 )
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Open a new DAS file. Use the file name as the internal
///     C     file name, and reserve no records for comments.
///     C
///           CALL DASONW ( FNAME, TYPE, FNAME, 0, HANDLE )
///
///     C
///     C     Fill the array DATA with the double precision
///     C     numbers 1.D0 through 100.D0, and add this array
///     C     to the file.
///     C
///           DO I = 1, 100
///              DATA(I) = DBLE(I)
///           END DO
///
///           CALL DASADD ( HANDLE, 100, DATA )
///
///     C
///     C     Now append the array DATA to the file again.
///     C
///           CALL DASADD ( HANDLE, 100, DATA )
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
///           CALL DASRDD ( HANDLE, 1, 200, DATA )
///
///     C
///     C     Dump the data to the screen. We should see the
///     C     sequence 1.0, 2.0, ..., 100.0, 1.0, 2.0, ... , 100.0.
///     C     The numbers will be represented as double precision
///     C     numbers in the output.
///     C
///           WRITE (*,*)
///           WRITE (*,*) 'Data from "', FNAME, '":'
///           WRITE (*,*)
///           DO I = 1, 25
///              WRITE (*,'(8F7.1)') (DATA((I-1)*8+J), J = 1, 8)
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
///      Data from "dasadd_ex1.das":
///
///         1.0    2.0    3.0    4.0    5.0    6.0    7.0    8.0
///         9.0   10.0   11.0   12.0   13.0   14.0   15.0   16.0
///        17.0   18.0   19.0   20.0   21.0   22.0   23.0   24.0
///        25.0   26.0   27.0   28.0   29.0   30.0   31.0   32.0
///        33.0   34.0   35.0   36.0   37.0   38.0   39.0   40.0
///        41.0   42.0   43.0   44.0   45.0   46.0   47.0   48.0
///        49.0   50.0   51.0   52.0   53.0   54.0   55.0   56.0
///        57.0   58.0   59.0   60.0   61.0   62.0   63.0   64.0
///        65.0   66.0   67.0   68.0   69.0   70.0   71.0   72.0
///        73.0   74.0   75.0   76.0   77.0   78.0   79.0   80.0
///        81.0   82.0   83.0   84.0   85.0   86.0   87.0   88.0
///        89.0   90.0   91.0   92.0   93.0   94.0   95.0   96.0
///        97.0   98.0   99.0  100.0    1.0    2.0    3.0    4.0
///         5.0    6.0    7.0    8.0    9.0   10.0   11.0   12.0
///        13.0   14.0   15.0   16.0   17.0   18.0   19.0   20.0
///        21.0   22.0   23.0   24.0   25.0   26.0   27.0   28.0
///        29.0   30.0   31.0   32.0   33.0   34.0   35.0   36.0
///        37.0   38.0   39.0   40.0   41.0   42.0   43.0   44.0
///        45.0   46.0   47.0   48.0   49.0   50.0   51.0   52.0
///        53.0   54.0   55.0   56.0   57.0   58.0   59.0   60.0
///        61.0   62.0   63.0   64.0   65.0   66.0   67.0   68.0
///        69.0   70.0   71.0   72.0   73.0   74.0   75.0   76.0
///        77.0   78.0   79.0   80.0   81.0   82.0   83.0   84.0
///        85.0   86.0   87.0   88.0   89.0   90.0   91.0   92.0
///        93.0   94.0   95.0   96.0   97.0   98.0   99.0  100.0
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
///         Added IMPLICIT NONE statement. Updated the code to prevent
///         DASCUD from being called with a negative number of double
///         precision words when the input count N is negative.
///
///         Edited the header to comply with NAIF standard. Fixed
///         bugs in the code example and modified the output presentation
///         to comply with the maximum line length for header comments.
///
///         Made local variable RECORD a saved variable which is
///         initialized by a DATA statement.
///
///         Bug fix: added FAILED call after DASHFS call.
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
///         this test, an infinite loop could result if DASA2L, DASURD or
///         DASWRD signaled an error inside the loop.
/// ```
pub fn dasadd(ctx: &mut SpiceContext, handle: i32, n: i32, data: &[f64]) -> crate::Result<()> {
    DASADD(handle, n, data, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASADD ( DAS, add data, double precision )
pub fn DASADD(HANDLE: i32, N: i32, DATA: &[f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DATA = DummyArray::new(DATA, 1..);
    let mut CLBASE: i32 = 0;
    let mut CLSIZE: i32 = 0;
    let mut FREE: i32 = 0;
    let mut LASTD: i32 = 0;
    let mut LASTLA = StackArray::<i32, 3>::new(1..=3);
    let mut LASTRC = StackArray::<i32, 3>::new(1..=3);
    let mut LASTWD = StackArray::<i32, 3>::new(1..=3);
    let mut NCOMC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NRESVR: i32 = 0;
    let mut NUMDP: i32 = 0;
    let mut NWRITN: i32 = 0;
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
    CHKIN(b"DASADD", ctx)?;

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
        CHKOUT(b"DASADD", ctx)?;
        return Ok(());
    }

    LASTD = LASTLA[DP];

    //
    // We will keep track of the location that we wish to write to
    // with the variables RECNO and WORDNO.  RECNO will be the record
    // number of the record we'll write to; WORDNO will be the number
    // preceding the word index, within record number RECNO, that we'll
    // write to.  For example, if we're about to write to the first
    // double precision number in record 10, RECNO will be 10 and
    // WORDNO will be 0.  Of course, when WORDNO reaches NWD, we'll
    // have to find a free record before writing anything.
    //
    // Prepare the variables RECNO and WORDNO:  use the physical
    // location of the last double precision address, if there are any
    // double precision data in the file.  Otherwise, RECNO becomes the
    // first record available for double precision data.
    //
    if (LASTD >= 1) {
        DASA2L(
            HANDLE,
            DP,
            LASTD,
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
    // Set the number of double precision words already written.  Keep
    // writing to the file until this number equals the number of
    // elements in DATA.
    //
    // Note that if N is non-positive, the loop doesn't get exercised.
    //
    //
    NWRITN = 0;

    while ((NWRITN < N) && !FAILED(ctx)) {
        //
        // Write as much data as we can (or need to) into the current
        // record.  We assume that RECNO, WORDNO, and NWRITN have been
        // set correctly at this point.
        //
        // Find out how many words to write into the current record.
        // There may be no space left in the current record.
        //
        NUMDP = intrinsics::MIN0(&[(N - NWRITN), (NWD - WORDNO)]);

        if (NUMDP > 0) {
            //
            // Write NUMDP words into the current record.  If the record
            // is new, write the entire record.  Otherwise, just update
            // the part we're interested in.
            //
            if (WORDNO == 0) {
                MOVED(
                    DATA.subarray((NWRITN + 1)),
                    NUMDP,
                    save.RECORD.as_slice_mut(),
                );
                DASWRD(HANDLE, RECNO, save.RECORD.as_slice(), ctx)?;
            } else {
                DASURD(
                    HANDLE,
                    RECNO,
                    (WORDNO + 1),
                    (WORDNO + NUMDP),
                    DATA.subarray((NWRITN + 1)),
                    ctx,
                )?;
            }

            NWRITN = (NWRITN + NUMDP);
            WORDNO = (WORDNO + NUMDP);
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
    // double precision words.  DASCUD will also update the file summary
    // accordingly.
    //
    DASCUD(HANDLE, DP, NWRITN, ctx)?;

    CHKOUT(b"DASADD", ctx)?;
    Ok(())
}
