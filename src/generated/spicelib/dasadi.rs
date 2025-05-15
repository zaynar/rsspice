//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const INT: i32 = 3;
const NWI: i32 = 256;

struct SaveVars {
    RECORD: StackArray<i32, 256>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut RECORD = StackArray::<i32, 256>::new(1..=NWI);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), NWI as usize))
                .chain([]);

            RECORD
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { RECORD }
    }
}

/// DAS, add data, integer
///
/// Add an array of integers to a DAS file.
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
///  N          I   Number of integers to add to DAS file.
///  DATA       I   Array of integers to add.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is a file handle of a DAS file opened for writing.
///
///  N        is the number of integer "words" to add to the DAS file
///           specified by HANDLE.
///
///  DATA     is an array of integers to be added to the specified DAS
///           file. Elements 1 through N are appended to the integer
///           data in the file.
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
///  This routine adds integer data to a DAS file by "appending" them
///  after any integer data already in the file. The sense in which
///  the data are "appended" is that the data will occupy a range of
///  logical addresses for integer data that immediately follow the
///  last logical address of a integer that is occupied at the time
///  this routine is called. The diagram below illustrates this
///  addition:
///
///     +-------------------------+
///     |    (already in use)     |  Integer logical address 1
///     +-------------------------+
///                 .
///                 .
///                 .
///     +-------------------------+
///     |    (already in use)     |  Last integer logical address
///     +-------------------------+  in use before call to DASADI
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
///  The logical organization of the integers in the DAS file is
///  independent of the location in the file of any data of double
///  precision or character type.
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
///  In order to update integer logical addresses that already contain
///  data, the SPICELIB routine DASUDI (DAS update data, integer)
///  should be used.
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
///  1) Create a new DAS file and add 200 integers to it. Close the
///     file, then re-open it and read the data back out.
///
///
///     Example code begins here.
///
///
///           PROGRAM DASADI_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         FNAME
///           PARAMETER           ( FNAME = 'dasadi_ex1.das' )
///
///           CHARACTER*(*)         TYPE
///           PARAMETER           ( TYPE  = 'TEST' )
///
///     C
///     C     Local variables.
///     C
///           INTEGER               DATA   ( 200 )
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
///     C     Fill the array DATA with the integers 1 through
///     C     100, and add this array to the file.
///     C
///           DO I = 1, 100
///              DATA(I) = I
///           END DO
///
///           CALL DASADI ( HANDLE, 100, DATA )
///
///     C
///     C     Now append the array DATA to the file again.
///     C
///           CALL DASADI ( HANDLE, 100, DATA )
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
///           CALL DASRDI ( HANDLE, 1, 200, DATA )
///
///     C
///     C     Dump the data to the screen.  We should see the
///     C     sequence  1, 2, ..., 100, 1, 2, ... , 100.
///     C
///           WRITE (*,*)
///           WRITE (*,*) 'Data from "', FNAME, '":'
///           WRITE (*,*)
///           DO I = 1, 20
///              WRITE (*,'(10I5)') (DATA((I-1)*10+J), J = 1, 10)
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
///      Data from "dasadi_ex1.das":
///
///         1    2    3    4    5    6    7    8    9   10
///        11   12   13   14   15   16   17   18   19   20
///        21   22   23   24   25   26   27   28   29   30
///        31   32   33   34   35   36   37   38   39   40
///        41   42   43   44   45   46   47   48   49   50
///        51   52   53   54   55   56   57   58   59   60
///        61   62   63   64   65   66   67   68   69   70
///        71   72   73   74   75   76   77   78   79   80
///        81   82   83   84   85   86   87   88   89   90
///        91   92   93   94   95   96   97   98   99  100
///         1    2    3    4    5    6    7    8    9   10
///        11   12   13   14   15   16   17   18   19   20
///        21   22   23   24   25   26   27   28   29   30
///        31   32   33   34   35   36   37   38   39   40
///        41   42   43   44   45   46   47   48   49   50
///        51   52   53   54   55   56   57   58   59   60
///        61   62   63   64   65   66   67   68   69   70
///        71   72   73   74   75   76   77   78   79   80
///        81   82   83   84   85   86   87   88   89   90
///        91   92   93   94   95   96   97   98   99  100
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
/// -    SPICELIB Version 1.3.0, 07-OCT-2021 (JDR) (NJB)
///
///         Added IMPLICIT NONE statement. Updated the code to prevent
///         DASCUD from being called with a negative number of integer
///         words when the input count N is negative.
///
///         Made local variable RECORD a saved variable which is
///         initialized by a DATA statement.
///
///         Bug fix: added FAILED call after DASHFS call.
///
///         Edited the header to comply with NAIF standard. Fixed
///         bugs in the code example and modified the output presentation
///         to comply with the maximum line length for header comments.
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
///         Test of FAILED() added to loop termination conditions.
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
///         this test, an infinite loop could result if DASA2L, DASURI or
///         DASWRI signaled an error inside the loop.
/// ```
pub fn dasadi(ctx: &mut SpiceContext, handle: i32, n: i32, data: &[i32]) -> crate::Result<()> {
    DASADI(handle, n, data, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASADI ( DAS, add data, integer )
pub fn DASADI(HANDLE: i32, N: i32, DATA: &[i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DATA = DummyArray::new(DATA, 1..);
    let mut CLBASE: i32 = 0;
    let mut CLSIZE: i32 = 0;
    let mut FREE: i32 = 0;
    let mut LASTI: i32 = 0;
    let mut LASTLA = StackArray::<i32, 3>::new(1..=3);
    let mut LASTRC = StackArray::<i32, 3>::new(1..=3);
    let mut LASTWD = StackArray::<i32, 3>::new(1..=3);
    let mut NCOMC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NRESVR: i32 = 0;
    let mut NUMINT: i32 = 0;
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

    CHKIN(b"DASADI", ctx)?;

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
        CHKOUT(b"DASADI", ctx)?;
        return Ok(());
    }

    LASTI = LASTLA[INT];

    //
    // We will keep track of the location that we wish to write to
    // with the variables RECNO and WORDNO.  RECNO will be the record
    // number of the record we'll write to; WORDNO will be the number
    // preceding the word index, within record number RECNO, that we'll
    // write to.  For example, if we're about to write to the first
    // integer in record 10, RECNO will be 10 and WORDNO will be 0.  Of
    // course, when WORDNO reaches NWI, we'll have to find a free record
    // before writing anything.
    //
    // Prepare the variables RECNO and WORDNO:  use the physical
    // location of the last integer address, if there are any integer
    // data in the file.  Otherwise, RECNO becomes the first record
    // available for integer data.
    //
    if (LASTI >= 1) {
        DASA2L(
            HANDLE,
            INT,
            LASTI,
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
    // Set the number of integer words already written.  Keep
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
        NUMINT = intrinsics::MIN0(&[(N - NWRITN), (NWI - WORDNO)]);

        if (NUMINT > 0) {
            //
            // Write NUMINT words into the current record.  If the record
            // is new, write the entire record.  Otherwise, just update
            // the part we're interested in.
            //
            if (WORDNO == 0) {
                MOVEI(
                    DATA.subarray((NWRITN + 1)),
                    NUMINT,
                    save.RECORD.as_slice_mut(),
                );
                DASWRI(HANDLE, RECNO, save.RECORD.as_slice(), ctx)?;
            } else {
                DASURI(
                    HANDLE,
                    RECNO,
                    (WORDNO + 1),
                    (WORDNO + NUMINT),
                    DATA.subarray((NWRITN + 1)),
                    ctx,
                )?;
            }

            NWRITN = (NWRITN + NUMINT);
            WORDNO = (WORDNO + NUMINT);
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
    // integer words.  DASCUD will also update the file summary
    // accordingly.
    //
    DASCUD(HANDLE, INT, NWRITN, ctx)?;

    CHKOUT(b"DASADI", ctx)?;
    Ok(())
}
