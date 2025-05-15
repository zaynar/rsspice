//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NWI: i32 = 256;
const INT: i32 = 3;

/// DAS, read data, integer
///
/// Read integer data from a range of DAS logical addresses.
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
///  FIRST,
///  LAST       I   Bounds of range of DAS integer logical addresses.
///  DATA       O   Data having addresses FIRST through LAST.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is a file handle for an open DAS file.
///
///  FIRST,
///  LAST     are the lower and upper bounds of a range of DAS integer
///           logical addresses. The range includes these bounds. FIRST
///           and LAST must be greater than or equal to 1 and less than
///           or equal to the highest integer DAS address in the DAS
///           file designated by HANDLE.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DATA     is an array of integers. DATA should have length
///           at least LAST - FIRST + 1.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input file handle is invalid, an error is signaled
///      by a routine in the call tree of this routine. DATA will
///      not be modified.
///
///  2)  If FIRST or LAST are out of range, an error is signaled
///      by a routine in the call tree of this routine.
///
///  3)  If FIRST is greater than LAST, DATA is left unchanged.
///
///  4)  If DATA is declared with length less than FIRST - LAST + 1,
///      the error cannot be diagnosed by this routine.
///
///  5)  If a file read error occurs, the error is signaled by a
///      routine in the call tree of this routine.
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
///  This routine provides random read access to the integer data in
///  a DAS file. This data are logically structured as a
///  one-dimensional array of integers.
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
///           PROGRAM DASRDI_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         FNAME
///           PARAMETER           ( FNAME = 'dasrdi_ex1.das' )
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
///      Data from "dasrdi_ex1.das":
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
/// -    SPICELIB Version 1.3.0, 09-OCT-2021 (JDR) (NJB)
///
///         Added IMPLICIT NONE statement.
///
///         Added FAILED call following DASA2L call.
///
///         Updated entries in $Revisions section.
///
///         Edited the header to comply with NAIF standard. Fixed
///         bugs in the code example and modified the output presentation
///         to comply with the maximum line length for header comments.
///
///         Added entry #5 to $Exceptions section.
///
/// -    SPICELIB Version 1.2.1, 19-DEC-1995 (NJB)
///
///         Corrected title of permuted index entry section.
///
/// -    SPICELIB Version 1.2.0, 30-OCT-1995 (NJB)
///
///         Routine now uses discovery check-in. FAILED test moved inside
///         loop.
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
///         new routine DASONW, open new for write, which makes use of the
///         file type. Also,  a variable for the type of the file to be
///         created was added.
///
/// -    SPICELIB Version 1.0.0, 13-JUN-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-MAY-1994 (KRG) (NJB)
///
///         Test of FAILED() added to loop termination condition. Without
///         this test, an infinite loop could result if DASA2L or DASRRI
///         signaled an error inside the loop.
/// ```
pub fn dasrdi(
    ctx: &mut SpiceContext,
    handle: i32,
    first: i32,
    last: i32,
    data: &mut [i32],
) -> crate::Result<()> {
    DASRDI(handle, first, last, data, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASRDI ( DAS, read data, integer )
pub fn DASRDI(
    HANDLE: i32,
    FIRST: i32,
    LAST: i32,
    DATA: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DATA = DummyArrayMut::new(DATA, 1..);
    let mut CLBASE: i32 = 0;
    let mut CLSIZE: i32 = 0;
    let mut N: i32 = 0;
    let mut NREAD: i32 = 0;
    let mut NUMINT: i32 = 0;
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
    // Find out the physical location of the first integer.  If FIRST
    // is invalid, DASA2L will take care of the problem.
    //
    DASA2L(
        HANDLE,
        INT,
        FIRST,
        &mut CLBASE,
        &mut CLSIZE,
        &mut RECNO,
        &mut WORDNO,
        ctx,
    )?;

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // Decide how many integers to read.
    //
    NUMINT = ((LAST - FIRST) + 1);
    NREAD = 0;

    //
    // Read as much data from record RECNO as necessary.
    //
    N = intrinsics::MIN0(&[NUMINT, ((NWI - WORDNO) + 1)]);

    DASRRI(
        HANDLE,
        RECNO,
        WORDNO,
        ((WORDNO + N) - 1),
        DATA.as_slice_mut(),
        ctx,
    )?;

    NREAD = N;
    RECNO = (RECNO + 1);

    //
    // Read from as many additional records as necessary.
    //
    while (NREAD < NUMINT) {
        if FAILED(ctx) {
            return Ok(());
        }

        //
        // At this point, RECNO is the correct number of the
        // record to read from next.  CLBASE is the number
        // of the first record of the cluster we're about
        // to read from.
        //
        if (RECNO < (CLBASE + CLSIZE)) {
            //
            // We can continue reading from the current
            // cluster.
            //
            N = intrinsics::MIN0(&[(NUMINT - NREAD), NWI]);

            DASRRI(HANDLE, RECNO, 1, N, DATA.subarray_mut((NREAD + 1)), ctx)?;

            NREAD = (NREAD + N);
            RECNO = (RECNO + 1);
        } else {
            //
            // We must find the next integer cluster to
            // read from.  The first integer in this
            // cluster has address FIRST + NREAD.
            //
            DASA2L(
                HANDLE,
                INT,
                (FIRST + NREAD),
                &mut CLBASE,
                &mut CLSIZE,
                &mut RECNO,
                &mut WORDNO,
                ctx,
            )?;
        }
    }

    Ok(())
}
