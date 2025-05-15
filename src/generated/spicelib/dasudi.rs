//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const INT: i32 = 3;
const NWI: i32 = 256;

/// DAS, update data, integer
///
/// Update data in a specified range of integer addresses in a DAS
/// file.
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
///  LAST       I   Range of integer addresses to write to.
///  DATA       I   An array of integers.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is a file handle of a DAS file opened for writing.
///
///  FIRST,
///  LAST     are the first and last of a range of DAS logical
///           addresses of integers to update. These addresses satisfy
///           the inequality
///
///              1  <=   FIRST   <=   LAST   <=   LASTI
///
///           where LASTI is the last integer logical address in
///           use in the DAS file designated by HANDLE.
///
///  DATA     is an array of integers. The array elements
///           DATA(1) through DATA(N) will be written to the
///           indicated DAS file, where N is LAST - FIRST + 1.
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
///  1)  If the input file handle is invalid, an error is
///      signaled by a routine in the call tree of this routine.
///
///  2)  Only logical addresses that already contain data may be
///      updated: if either FIRST or LAST are outside the range
///
///         [ 1,  LASTI ]
///
///      where LASTI is the last integer logical address that
///      currently contains data in the indicated DAS file, the error
///      SPICE(INVALIDADDRESS) is signaled. The DAS file will not be
///      modified.
///
///  3)  If FIRST > LAST but both addresses are valid, this routine
///      will not modify the indicated DAS file. No error will be
///      signaled.
///
///  4)  If an I/O error occurs during the data update attempted
///      by this routine, the error is signaled by a routine in the
///      call tree of this routine.
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
///  This routine replaces the integer data in the specified range of
///  logical addresses within a DAS file with the contents of the
///  input array DATA.
///
///  The actual physical write operations that update the indicated
///  DAS file with the contents of the input array DATA might not take
///  place before this routine returns, since the DAS system buffers
///  data that is written as well as data that is read. In any case,
///  the data will be flushed to the file at the time the file is
///  closed, if not earlier. A physical write of all buffered
///  records can be forced by calling the SPICELIB routine DASWBR
///  (DAS, write buffered records).
///
///  In order to append integer data to a DAS file, filling in a range
///  of integer logical addresses that starts immediately after the
///  last integer logical address currently in use, the SPICELIB
///  routine DASADI (DAS add data, integer) should be used.
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
///  1) Write to addresses 1 through 200 in a DAS file in random-access
///     fashion by updating the file. Recall that data must be present
///     in the file before it can be updated.
///
///
///     Example code begins here.
///
///
///           PROGRAM DASUDI_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         FNAME
///           PARAMETER           ( FNAME = 'dasudi_ex1.das' )
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
///     C     Append 200 integers to the file; after the data are
///     C     present, we're free to update it in any order we
///     C     please.  (CLEARI zeros out an integer array.)
///     C
///           CALL CLEARI (           200,  DATA )
///           CALL DASADI (  HANDLE,  200,  DATA )
///
///     C
///     C     Now the integer logical addresses 1:200 can be
///     C     written to in random-access fashion.  We'll fill them
///     C     in reverse order.
///     C
///           DO I = 200, 1, -1
///              CALL DASUDI ( HANDLE, I, I, I )
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL DASCLS ( HANDLE )
///
///     C
///     C     Now make sure that we updated the file properly.
///     C     Open the file for reading and dump the contents
///     C     of the integer logical addresses 1:200.
///     C
///           CALL DASOPR ( FNAME, HANDLE )
///
///           CALL CLEARI (              200,  DATA  )
///           CALL DASRDI (  HANDLE,  1, 200,  DATA  )
///
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
///      Data from "dasudi_ex1.das":
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
///       101  102  103  104  105  106  107  108  109  110
///       111  112  113  114  115  116  117  118  119  120
///       121  122  123  124  125  126  127  128  129  130
///       131  132  133  134  135  136  137  138  139  140
///       141  142  143  144  145  146  147  148  149  150
///       151  152  153  154  155  156  157  158  159  160
///       161  162  163  164  165  166  167  168  169  170
///       171  172  173  174  175  176  177  178  179  180
///       181  182  183  184  185  186  187  188  189  190
///       191  192  193  194  195  196  197  198  199  200
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
/// -    SPICELIB Version 1.3.0, 16-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Updated entries in $Revisions section.
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
///         new routine DASONW, open new for write, which makes use of the
///         file type. Also,  a variable for the type of the file to be
///         created was added.
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
///         this test, an infinite loop could result if DASA2L or DASURI
///         signaled an error inside the loop.
/// ```
pub fn dasudi(
    ctx: &mut SpiceContext,
    handle: i32,
    first: i32,
    last: i32,
    data: &[i32],
) -> crate::Result<()> {
    DASUDI(handle, first, last, data, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASUDI ( DAS, update data, integer )
pub fn DASUDI(
    HANDLE: i32,
    FIRST: i32,
    LAST: i32,
    DATA: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DATA = DummyArray::new(DATA, 1..);
    let mut CLBASE: i32 = 0;
    let mut CLSIZE: i32 = 0;
    let mut LASTC: i32 = 0;
    let mut LASTD: i32 = 0;
    let mut LASTI: i32 = 0;
    let mut N: i32 = 0;
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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DASUDI", ctx)?;
    }

    //
    // Get the last logical addresses in use in this DAS file.
    //
    DASLLA(HANDLE, &mut LASTC, &mut LASTD, &mut LASTI, ctx)?;

    //
    // Validate the input addresses.
    //
    if ((((FIRST < 1) || (FIRST > LASTI)) || (LAST < 1)) || (LAST > LASTI)) {
        SETMSG(b"FIRST was #. LAST was #. Valid range is [1,#].", ctx);
        ERRINT(b"#", FIRST, ctx);
        ERRINT(b"#", LAST, ctx);
        ERRINT(b"#", LASTI, ctx);
        SIGERR(b"SPICE(INVALIDADDRESS)", ctx)?;
        CHKOUT(b"DASUDI", ctx)?;
        return Ok(());
    }

    //
    // Let N be the number of addresses to update.
    //
    N = ((LAST - FIRST) + 1);

    //
    // We will use the variables RECNO and WORDNO to determine where to
    // write data in the DAS file.  RECNO will be the record containing
    // the physical location to write to;  WORDNO will be the word
    // location that we will write to next.
    //
    // Find the first location to write to.  CLBASE and CLSIZE are the
    // base record number and size of the cluster of integer records that
    // the address FIRST lies within.
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
        // record.  We assume that CLBASE, RECNO, WORDNO, and NWRITN have
        // been set correctly at this point.
        //
        // Find out how many words to write into the current record.
        // There may be no space left in the current record.
        //
        NUMINT = intrinsics::MIN0(&[(N - NWRITN), ((NWI - WORDNO) + 1)]);

        if (NUMINT > 0) {
            //
            // Write NUMINT words into the current record.
            //
            DASURI(
                HANDLE,
                RECNO,
                WORDNO,
                ((WORDNO + NUMINT) - 1),
                DATA.subarray((NWRITN + 1)),
                ctx,
            )?;

            NWRITN = (NWRITN + NUMINT);
            WORDNO = (WORDNO + NUMINT);
        } else {
            //
            // It's time to start on a new record.  If the record we
            // just finished writing to (or just attempted writing to,
            // if it was full) was not the last of the cluster, the next
            // record to write to is the immediate successor of the last
            // one.  Otherwise, we'll have to look up the location of the
            // next integer logical address.
            //
            if (RECNO < ((CLBASE + CLSIZE) - 1)) {
                RECNO = (RECNO + 1);
                WORDNO = 1;
            } else {
                DASA2L(
                    HANDLE,
                    INT,
                    (FIRST + NWRITN),
                    &mut CLBASE,
                    &mut CLSIZE,
                    &mut RECNO,
                    &mut WORDNO,
                    ctx,
                )?;
            }
        }
    }

    CHKOUT(b"DASUDI", ctx)?;
    Ok(())
}
