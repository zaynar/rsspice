//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NWD: i32 = 128;
const DP: i32 = 2;

/// DAS, read data, double precision
///
/// Read double precision data from a range of DAS logical addresses.
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
///  LAST       I   Bounds of range of DAS double precision logical
///                 addresses.
///  DATA       O   Data having addresses FIRST through LAST.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is a file handle for an open DAS file.
///
///  FIRST,
///  LAST     are the lower and upper bounds of a range of DAS double
///           precision logical addresses. The range includes these
///           bounds. FIRST and LAST must be greater than or equal to 1
///           and less than or equal to the highest double precision
///           DAS address in the DAS file designated by HANDLE.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DATA     is an array of double precision numbers. DATA
///           should have length at least LAST - FIRST + 1.
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
///  This routine provides random read access to the double precision
///  data in a DAS file. This data are logically structured as a
///  one-dimensional array of double precision numbers.
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
///  1) Create a new DAS file TEST.DAS and add 200 double
///     precision numbers to it. Close the file, then re-open
///     it and read the data back out.
///
///
///     Example code begins here.
///
///
///           PROGRAM DASRDD_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         FNAME
///           PARAMETER           ( FNAME = 'dasrdd_ex1.das' )
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
///     C     Dump the data to the screen.  We should see the
///     C     sequence 1.0, 2.0, ..., 100.0, 1.0, 2.0, ..., 100.0.
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
///      Data from "dasrdd_ex1.das":
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
/// -    SPICELIB Version 1.2.0, 01-NOV-1995 (NJB)
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
///         this test, an infinite loop could result if DASA2L or DASRRD
///         signaled an error inside the loop.
/// ```
pub fn dasrdd(
    ctx: &mut SpiceContext,
    handle: i32,
    first: i32,
    last: i32,
    data: &mut [f64],
) -> crate::Result<()> {
    DASRDD(handle, first, last, data, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASRDD ( DAS, read data, double precision )
pub fn DASRDD(
    HANDLE: i32,
    FIRST: i32,
    LAST: i32,
    DATA: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DATA = DummyArrayMut::new(DATA, 1..);
    let mut CLBASE: i32 = 0;
    let mut CLSIZE: i32 = 0;
    let mut N: i32 = 0;
    let mut NREAD: i32 = 0;
    let mut NUMDP: i32 = 0;
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
    // Find out the physical location of the first double precision
    // number.  If FIRST is invalid, DASA2L will take care of the
    // problem.
    //
    DASA2L(
        HANDLE,
        DP,
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
    // Decide how many double precision numbers to read.
    //
    NUMDP = ((LAST - FIRST) + 1);
    NREAD = 0;

    //
    // Read as much data from record RECNO as necessary.
    //
    N = intrinsics::MIN0(&[NUMDP, ((NWD - WORDNO) + 1)]);

    DASRRD(
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
    while (NREAD < NUMDP) {
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
            N = intrinsics::MIN0(&[(NUMDP - NREAD), NWD]);

            DASRRD(HANDLE, RECNO, 1, N, DATA.subarray_mut((NREAD + 1)), ctx)?;

            NREAD = (NREAD + N);
            RECNO = (RECNO + 1);
        } else {
            //
            // We must find the next double precision cluster to
            // read from.  The first double precision number in this
            // cluster has address FIRST + NREAD.
            //
            DASA2L(
                HANDLE,
                DP,
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
