//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// EK, open file for writing
///
/// Open an existing E-kernel file for writing.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of EK file.
///  HANDLE     O   Handle attached to EK file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of an existing E-kernel file to be
///           opened for write access.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the DAS file handle of the EK designate by
///           FNAME. This handle is used to identify the file
///           to other EK routines.
/// ```
///
/// # Parameters
///
/// ```text
///  FTSIZE   is the maximum number of DAS files that a user can
///           have open simultaneously. This includes any files used
///           by the DAS system.
///
///           See the include file das.inc for the actual value of
///           this parameter.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the indicated file cannot be opened, an error is signaled
///      by a routine in the call tree of this routine.
///
///  2)  If the indicated file has the wrong architecture version, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  3)  If an I/O error occurs while reading or writing the indicated
///      file, the error is signaled by a routine in the call tree of
///      this routine.
/// ```
///
/// # Files
///
/// ```text
///  See the EK Required Reading ek.req for a discussion of the EK file
///  format.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine should be used to open an EK existing file for write
///  access.
///
///  Opening an EK file with this routine makes the EK accessible to
///  the following SPICELIB EK access routines, all of which modify
///  the target EK file:
///
///     Begin segment:
///
///        EKBSEG
///
///     Append, insert, delete records:
///
///        EKAPPR
///        EKINSR
///        EKDELR
///
///     Add column entries:
///
///        EKACEC
///        EKACED
///        EKACEI
///
///     Update existing column entries:
///
///        EKUCEC
///        EKUCED
///        EKUCEI
///
///     Execute fast write:
///
///        EKIFLD
///        EKFFLD
///        EKACLC
///        EKACLD
///        EKACLI
///
///  An EK opened for write access is also accessible for reading.
///  The file may be accessed by the SPICELIB EK readers
///
///        EKRCEC
///        EKRCED
///        EKRCEI
///
///     and summary routines:
///
///        EKNSEG
///        EKSSUM
///
///
///  An EK opened for write access cannot be queried. To make an EK
///  available to the EK query system, the file must be loaded via
///  FURNSH or EKLEF, rather than by this routine. See the EK Required
///  Reading for further information.
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
///  1) The following program demonstrates how to create a new EK and
///     add data to a character column in a given record within the
///     file, how to re-open the file for write access and update the
///     data, and how to read the data from it.
///
///
///     Example code begins here.
///
///
///           PROGRAM EKOPW_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Column Name Size (CNAMSZ)
///     C
///           INCLUDE 'ekcnamsz.inc'
///
///     C
///     C     Local constants.
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME = 'ekopw_ex1.bdb' )
///
///           CHARACTER*(*)         IFNAME
///           PARAMETER           ( IFNAME = 'Test EK'  )
///
///           CHARACTER*(*)         TABLE
///           PARAMETER           ( TABLE  = 'CHR_DATA' )
///
///           INTEGER               CVLEN
///           PARAMETER           ( CVLEN  = 9  )
///
///           INTEGER               DECLEN
///           PARAMETER           ( DECLEN = 200 )
///
///           INTEGER               MAXVAL
///           PARAMETER           ( MAXVAL = 4  )
///
///           INTEGER               NCOLS
///           PARAMETER           ( NCOLS  = 2  )
///
///           INTEGER               NROWS
///           PARAMETER           ( NROWS  = 6  )
///
///           INTEGER               NRESVC
///           PARAMETER           ( NRESVC = 0  )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(DECLEN)    CDECLS ( NCOLS  )
///           CHARACTER*(CNAMSZ)    CNAMES ( NCOLS  )
///           CHARACTER*(CVLEN)     CVALS  ( MAXVAL )
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               J
///           INTEGER               NVALS
///           INTEGER               RECNO
///           INTEGER               SEGNO
///
///           LOGICAL               ISNULL
///
///
///     C
///     C     Open a new EK file.  For simplicity, we won't
///     C     reserve space for the comment area, so the
///     C     number of reserved comment characters is zero.
///     C     The constant IFNAME is the internal file name.
///     C
///           CALL EKOPN ( EKNAME, IFNAME, NRESVC, HANDLE )
///
///     C
///     C     Set up the table and column names and declarations
///     C     for the CHR_DATA segment.  We'll index all of
///     C     the columns.
///     C
///           CNAMES(1) =  'CHR_COL_1'
///           CDECLS(1) =  'DATATYPE = CHARACTER*(*), '
///          .       //    'INDEXED = TRUE, NULLS_OK = TRUE'
///
///           CNAMES(2) =  'CHR_COL_2'
///           CDECLS(2) =  'DATATYPE = CHARACTER*(9), '
///          .       //    'SIZE = VARIABLE, NULLS_OK = TRUE'
///
///     C
///     C     Start the segment.
///     C
///           CALL EKBSEG ( HANDLE, TABLE,  NCOLS,
///          .              CNAMES, CDECLS, SEGNO )
///
///           DO I = 0, NROWS-1
///
///              CALL EKAPPR ( HANDLE, SEGNO, RECNO )
///
///              ISNULL = ( I .EQ. 1 )
///
///              CALL INTSTR ( I, CVALS(1) )
///              CALL EKACEC ( HANDLE, SEGNO, RECNO, CNAMES(1),
///          .                 1,      CVALS, ISNULL           )
///
///     C
///     C        Array-valued columns follow.
///     C
///              CALL INTSTR ( 10*I,     CVALS(1) )
///              CALL INTSTR ( 10*I + 1, CVALS(2) )
///              CALL INTSTR ( 10*I + 2, CVALS(3) )
///              CALL INTSTR ( 10*I + 3, CVALS(4) )
///              CALL EKACEC ( HANDLE, SEGNO, RECNO, CNAMES(2),
///          .                 4,      CVALS, ISNULL           )
///
///           END DO
///
///     C
///     C     End the file.
///     C
///           CALL EKCLS ( HANDLE )
///
///     C
///     C     Open the EK for write access.
///     C
///           CALL EKOPW ( EKNAME, HANDLE )
///
///     C
///     C     Negate the values in the even-numbered records
///     C     using the update routines.
///     C
///           DO I = 1, NROWS, 2
///
///              RECNO  = I+1
///
///              ISNULL = ( I .EQ. 1 )
///
///              CALL INTSTR ( -I, CVALS(1) )
///              CALL EKUCEC ( HANDLE, SEGNO, RECNO, CNAMES(1),
///          .                 1,      CVALS, ISNULL           )
///
///     C
///     C        Array-valued columns follow.
///     C
///              CALL INTSTR ( -10*I,       CVALS(1) )
///              CALL INTSTR ( -(10*I + 1), CVALS(2) )
///              CALL INTSTR ( -(10*I + 2), CVALS(3) )
///              CALL INTSTR ( -(10*I + 3), CVALS(4) )
///              CALL EKUCEC ( HANDLE, SEGNO, RECNO, CNAMES(2),
///          .                 4,      CVALS, ISNULL           )
///
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL EKCLS ( HANDLE )
///
///     C
///     C     Open the created file. Show the values added.
///     C
///           CALL EKOPR ( EKNAME, HANDLE )
///
///           DO I = 1, NROWS
///
///              CALL EKRCEC ( HANDLE, SEGNO, I,     CNAMES(1),
///          .                 NVALS,  CVALS, ISNULL           )
///
///              IF ( .NOT. ISNULL ) THEN
///
///                 WRITE(*,*) 'Data from column: ', CNAMES(1)
///                 WRITE(*,*) '   record number: ', I
///                 WRITE(*,*) '   values       : ',
///          .                                ( CVALS(J), J=1,NVALS )
///                 WRITE(*,*) ' '
///
///              ELSE
///
///                 WRITE(*,*) 'Record ', I, 'flag is NULL.'
///                 WRITE(*,*) ' '
///
///              END IF
///
///     C
///     C        Array-valued columns follow.
///     C
///              CALL EKRCEC ( HANDLE, SEGNO, I,     CNAMES(2),
///          .                 NVALS,  CVALS, ISNULL           )
///
///              IF ( .NOT. ISNULL ) THEN
///
///                 WRITE(*,*) 'Data from column: ', CNAMES(2)
///                 WRITE(*,*) '   record number: ', I
///                 WRITE(*,*) '   values       : ',
///          .                                ( CVALS(J), J=1,NVALS )
///                 WRITE(*,*) ' '
///
///              END IF
///
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL EKCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Data from column: CHR_COL_1
///         record number:            1
///         values       : 0
///
///      Data from column: CHR_COL_2
///         record number:            1
///         values       : 0        1        2        3
///
///      Record            2 flag is NULL.
///
///      Data from column: CHR_COL_1
///         record number:            3
///         values       : 2
///
///      Data from column: CHR_COL_2
///         record number:            3
///         values       : 20       21       22       23
///
///      Data from column: CHR_COL_1
///         record number:            4
///         values       : -3
///
///      Data from column: CHR_COL_2
///         record number:            4
///         values       : -30      -31      -32      -33
///
///      Data from column: CHR_COL_1
///         record number:            5
///         values       : 4
///
///      Data from column: CHR_COL_2
///         record number:            5
///         values       : 40       41       42       43
///
///      Data from column: CHR_COL_1
///         record number:            6
///         values       : -5
///
///      Data from column: CHR_COL_2
///         record number:            6
///         values       : -50      -51      -52      -53
///
///
///     Note that the second record does not appear due to setting the
///     ISNULL flag to .TRUE. for that record. The odd value record
///     numbers have negative values as a result of the update calls.
///
///     After run completion, a new EK exists in the output directory.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  No more than FTSIZE DAS files may be opened simultaneously.
///      See the include file das.inc for the value of FTSIZE.
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
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Updated exception #1 to remove the statement about deleting the
///         opened file upon failure.
///
///         Updated "fast write" list of API, which was listing the wrong
///         APIs for adding data.
///
///         Added FTSIZE parameter description. Updated index entry.
///
/// -    SPICELIB Version 1.0.1, 09-JAN-2002 (NJB)
///
///         Documentation change: instances of the phrase "fast load"
///         were replaced with "fast write."
///
/// -    SPICELIB Version 1.0.0, 26-SEP-1995 (NJB)
/// ```
pub fn ekopw(ctx: &mut SpiceContext, fname: &str, handle: &mut i32) -> crate::Result<()> {
    EKOPW(fname.as_bytes(), handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKOPW ( EK, open file for writing )
pub fn EKOPW(FNAME: &[u8], HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKOPW", ctx)?;
    }

    //
    // Open the file as a DAS file.
    //
    DASOPW(FNAME, HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"EKOPW", ctx)?;
        return Ok(());
    }

    //
    // Nothing doing unless the architecture is correct.  This file
    // should be a paged DAS EK.
    //
    ZZEKPGCH(*HANDLE, b"WRITE", ctx)?;

    CHKOUT(b"EKOPW", ctx)?;
    Ok(())
}
