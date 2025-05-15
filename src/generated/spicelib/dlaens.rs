//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const VERIDX: i32 = 1;
const LLBIDX: i32 = (VERIDX + 1);
const LLEIDX: i32 = (LLBIDX + 1);
const NULPTR: i32 = -1;
const BWDIDX: i32 = 1;
const FWDIDX: i32 = (BWDIDX + 1);
const IBSIDX: i32 = (FWDIDX + 1);
const ISZIDX: i32 = (IBSIDX + 1);
const DBSIDX: i32 = (ISZIDX + 1);
const DSZIDX: i32 = (DBSIDX + 1);
const CBSIDX: i32 = (DSZIDX + 1);
const CSZIDX: i32 = (CBSIDX + 1);
const DLADSZ: i32 = CSZIDX;
const FMTVER: i32 = 1000000;
const NCHREC: i32 = 1024;

/// DLA, end new segment
///
/// End a new segment in a DLA file.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
/// * [DLA](crate::required_reading::dla)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of open DLA file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the integer handle associated with the DLA file to
///           be updated. This handle is used to identify the file
///           in subsequent calls to other DLA or DAS routines.
///
///           The DLA file must be open for write access. A new DLA
///           segment is completed in the indicated file. The file
///           is left open, since data may be written to the file
///           following a call to this routine.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See the $Particulars and $Examples header sections for
///  a description of the actions performed by this routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input file handle does not refer to a DAS file that is
///      open for write access, an error is signaled by a routine
///      in the call tree of this routine.
///
///  2)  If an error occurs while reading or writing to the DLA file,
///      the error is signaled by a routine in the call tree of
///      this routine.
/// ```
///
/// # Files
///
/// ```text
///  See description of input argument HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  DLA files are built using the DAS low-level format; DLA files are
///  a specialized type of DAS file in which data are organized as a
///  doubly linked list of segments. Each segment's data belong to
///  contiguous components of character, double precision, and integer
///  type.
///
///  This routine supports creation of a DLA segment. DLA segments
///  are created by appending data to the DAS integer, double
///  precision, and character address spaces of a DLA file. The new
///  segment's descriptor is located immediately before the integer
///  component of the segment's data.
///
///  When a new segment is added to a DLA file, the segment is
///  inserted into the file's doubly linked segment list. If the new
///  segment is the first, the DLA file's first and last list entry
///  pointers are updated to point to the new segment; specifically,
///  these pointers point to the first integer of the new segment's
///  descriptor. The backward pointer of the new segment is set to
///  null in this case.
///
///  If the new segment is not the first, the DLA file's list end
///  pointer is updated to point to the new segment, and the forward
///  pointer of the previous segment also is updated to point to the
///  first integer of the new segment's descriptor. The backward
///  pointer of the new segment points to to point to the first
///  integer of the previous segment's descriptor.
///
///  The normal sequence of operations required to create a DLA
///  segment is as follows:
///
///     Call DLAOPN to create a new, empty DLA file.
///
///     For each segment to be created,
///
///        Call DLABNS to begin a segment.
///
///        Use the DAS "add" and "update" routines to populate
///        the segment with data.
///
///        Call DLAENS to end the segment.
///
///     Call DASCLS to segregate and close the DLA file.
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
///  1) Create a DLA file containing one segment; the segment
///     contains character, double precision, and integer data.
///     After writing and closing the file, open the file for
///     read access; dump the data to standard output.
///
///
///     Example code begins here.
///
///
///           PROGRAM DLAENS_EX1
///           IMPLICIT NONE
///
///           INCLUDE 'dla.inc'
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         DLA
///           PARAMETER           ( DLA    = 'dlaens_ex1.dla' )
///
///           INTEGER               IFNLEN
///           PARAMETER           ( IFNLEN =  60 )
///
///           INTEGER               LNSIZE
///           PARAMETER           ( LNSIZE =  61 )
///
///           INTEGER               MAXC
///           PARAMETER           ( MAXC   =  5 )
///
///           INTEGER               MAXD
///           PARAMETER           ( MAXD   =  50 )
///
///           INTEGER               MAXI
///           PARAMETER           ( MAXI   =  100 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(LNSIZE)    CVALS   ( MAXC )
///           CHARACTER*(LNSIZE)    CVALS2  ( MAXC )
///           CHARACTER*(IFNLEN)    IFNAME
///
///           DOUBLE PRECISION      DVALS   ( MAXD )
///           DOUBLE PRECISION      DVALS2  ( MAXD )
///
///           INTEGER               BASE
///           INTEGER               DESCR   ( DLADSZ )
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               IVALS   ( MAXI )
///           INTEGER               IVALS2  ( MAXI )
///           INTEGER               J
///           INTEGER               K
///           INTEGER               N
///           INTEGER               NCOMCH
///
///           LOGICAL               FOUND
///
///     C
///     C     Set the internal file name.  Don't reserve characters in
///     C     the DAS comment area.
///     C
///           IFNAME = 'Example DLA file for testing'
///           NCOMCH = 0
///
///     C
///     C     Open a new DLA file.
///     C
///           CALL DLAOPN ( DLA, 'DLA', IFNAME, NCOMCH, HANDLE )
///
///     C
///     C     Begin a new segment.
///     C
///           CALL DLABNS ( HANDLE )
///
///     C
///     C     Add character data to the segment.
///     C
///           DO I = 1, MAXC
///
///              DO J = 1, LNSIZE
///
///                 K = MOD( J+I-1, 10 )
///
///                 CALL INTSTR ( K,  CVALS(I)(J:J) )
///
///              END DO
///
///           END DO
///
///           CALL DASADC ( HANDLE, MAXC*LNSIZE, 1, LNSIZE, CVALS )
///
///     C
///     C     Add integer and double precision data to the segment.
///     C
///           DO I = 1, MAXI
///              IVALS(I) = I
///           END DO
///
///           CALL DASADI ( HANDLE, MAXI, IVALS )
///
///           DO I = 1, MAXD
///              DVALS(I) = I
///           END DO
///
///           CALL DASADD ( HANDLE, MAXD, DVALS )
///
///     C
///     C     End the segment.
///     C
///           CALL DLAENS ( HANDLE )
///
///     C
///     C     Close the file.  The routine DASCLS flushes the DAS
///     C     buffers and segregates the file before closing it.
///     C
///           CALL DASCLS ( HANDLE )
///
///     C
///     C     Now read the file and check the data.
///     C
///           CALL DASOPR ( DLA, HANDLE )
///
///     C
///     C     Obtain the segment descriptor for the sole segment
///     C     in the file. We need not check the found flag
///     C     in this case because we know there is one segment
///     C     in the file.
///     C
///           CALL DLABFS ( HANDLE, DESCR, FOUND )
///
///     C
///     C     Fetch character data from the segment.  Obtain the
///     C     base address of the character data and the
///     C     character count from the descriptor.
///     C
///           BASE = DESCR(CBSIDX)
///           N    = DESCR(CSZIDX)
///
///           CALL DASRDC ( HANDLE, BASE+1, BASE+N, 1, LNSIZE, CVALS2 )
///
///     C
///     C     Display the character data.
///     C
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Character array:'
///
///           DO I = 1, N/LNSIZE
///              WRITE (*,*) CVALS2(I)
///           END DO
///
///     C
///     C     Fetch and display the integer and double precision data.
///     C
///           BASE = DESCR(IBSIDX)
///           N    = DESCR(ISZIDX)
///
///           CALL DASRDI( HANDLE, BASE+1, BASE+N, IVALS2 )
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Integer array:'
///           DO I = 1, N/10
///              WRITE (*,'(10I6)') (IVALS2((I-1)*10 + J), J=1, 10)
///           END DO
///
///           BASE = DESCR(DBSIDX)
///           N    = DESCR(DSZIDX)
///
///           CALL DASRDD( HANDLE, BASE+1, BASE+N, DVALS2 )
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Double precision array:'
///           DO I = 1, N/10
///              WRITE (*,'(10F6.1)') (DVALS2((I-1)*10 + J), J=1, 10)
///           END DO
///
///     C
///     C     Close the file.  This step is unnecessary in this
///     C     program, but is a good practice in general
///     C     because closing the file frees resources.
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
///      Character array:
///      1234567890123456789012345678901234567890123456789012345678901
///      2345678901234567890123456789012345678901234567890123456789012
///      3456789012345678901234567890123456789012345678901234567890123
///      4567890123456789012345678901234567890123456789012345678901234
///      5678901234567890123456789012345678901234567890123456789012345
///
///      Integer array:
///          1     2     3     4     5     6     7     8     9    10
///         11    12    13    14    15    16    17    18    19    20
///         21    22    23    24    25    26    27    28    29    30
///         31    32    33    34    35    36    37    38    39    40
///         41    42    43    44    45    46    47    48    49    50
///         51    52    53    54    55    56    57    58    59    60
///         61    62    63    64    65    66    67    68    69    70
///         71    72    73    74    75    76    77    78    79    80
///         81    82    83    84    85    86    87    88    89    90
///         91    92    93    94    95    96    97    98    99   100
///
///      Double precision array:
///        1.0   2.0   3.0   4.0   5.0   6.0   7.0   8.0   9.0  10.0
///       11.0  12.0  13.0  14.0  15.0  16.0  17.0  18.0  19.0  20.0
///       21.0  22.0  23.0  24.0  25.0  26.0  27.0  28.0  29.0  30.0
///       31.0  32.0  33.0  34.0  35.0  36.0  37.0  38.0  39.0  40.0
///       41.0  42.0  43.0  44.0  45.0  46.0  47.0  48.0  49.0  50.0
///
///
///     Note that after run completion, a new DLA file exists in the
///     output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 14-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Modified
///         the presentation of the output in the code example to comply
///         with the maximum line length for the header comments.
///
/// -    SPICELIB Version 1.0.0, 08-FEB-2017 (NJB)
///
///         Updated version info.
///
///         08-OCT-2009 (NJB)
///
///            Updated header.
///
///         11-FEB-2005 (NJB)
/// ```
pub fn dlaens(ctx: &mut SpiceContext, handle: i32) -> crate::Result<()> {
    DLAENS(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DLAENS ( DLA, end new segment )
pub fn DLAENS(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DESCR = StackArray::<i32, 8>::new(1..=DLADSZ);
    let mut LASTC: i32 = 0;
    let mut LASTD: i32 = 0;
    let mut LASTI: i32 = 0;
    let mut THIS: i32 = 0;

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
    }

    CHKIN(b"DLAENS", ctx)?;
    //
    // Now that the segment has been written, our only task is
    // to update the corresponding segment descriptor to reflect
    // the sizes of each component.
    //
    // Look up the pointer to the last DLA segment descriptor in the
    // file.  Then look up the segment descriptor itself.
    //
    DASRDI(HANDLE, LLEIDX, LLEIDX, std::slice::from_mut(&mut THIS), ctx)?;
    DASRDI(
        HANDLE,
        THIS,
        ((THIS + DLADSZ) - 1),
        DESCR.as_slice_mut(),
        ctx,
    )?;

    //
    // Find the last DAS logical addresses in use for each data type.
    //
    DASLLA(HANDLE, &mut LASTC, &mut LASTD, &mut LASTI, ctx)?;

    //
    // Set the component sizes in the descriptor. The sizes are easily
    // computed from the last addresses in use and the component base
    // addresses already stored in the segment descriptor.
    //
    DESCR[ISZIDX] = (LASTI - DESCR[IBSIDX]);
    DESCR[DSZIDX] = (LASTD - DESCR[DBSIDX]);
    DESCR[CSZIDX] = (LASTC - DESCR[CBSIDX]);

    //
    // Update the descriptor in the file.
    //
    DASUDI(HANDLE, THIS, ((THIS + DLADSZ) - 1), DESCR.as_slice(), ctx)?;

    //
    // Leave the file open.  The file is now ready for the
    // addition of a new segment.
    //
    CHKOUT(b"DLAENS", ctx)?;
    Ok(())
}
