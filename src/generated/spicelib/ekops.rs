//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const FPARSZ: i32 = 1;
const SGTIDX: i32 = 1;

/// EK, open scratch file
///
/// Open a scratch (temporary) E-kernel file and prepare the file
/// for writing.
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
///  HANDLE     O   File handle attached to new EK file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the EK file handle of the file opened by this
///           routine. This handle is used to identify the file
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
///      by a routine in the call tree of this routine. The new file
///      will be deleted.
///
///  2)  If an I/O error occurs while reading or writing the indicated
///      file, the error is signaled by a routine in the call tree of
///      this routine.
/// ```
///
/// # Files
///
/// ```text
///  This routine creates a temporary EK file; the file is deleted
///  when the calling program terminates or when the file is closed
///  using the SPICELIB routine EKCLS.
///
///  See the EK Required Reading ek.req for a discussion of the EK file
///  format.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine operates by side effects: it opens and prepares
///  an EK for addition of data. "Scratch" files are automatically
///  deleted when the calling program terminates normally or when
///  closed using the SPICELIB routine EKCLS.
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
///  1) Suppose we want to create an E-kernel which contains a table
///     of items that have been ordered but we do not want to keep
///     the file. The columns of this table are shown below:
///
///        DATAITEMS
///
///           Column Name     Data Type
///           -----------     ---------
///           ITEM_ID         INTEGER
///           ORDER_ID        INTEGER
///           ITEM_NAME       CHARACTER*(*)
///           DESCRIPTION     CHARACTER*(*)
///           PRICE           DOUBLE PRECISION
///
///
///     This examples demonstrates how to open a scratch EK file;
///     create the segment described above, how to insert a new record
///     into it, and how to summarize its contents.
///
///
///     Example code begins here.
///
///
///           PROGRAM EKOPS_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Column Name Size (CNAMSZ); the maximum
///     C     length of an input query (MAXQRY), the maximum number of
///     C     columns per segment (MXCLSG); and the maximum length of
///     C     a table name (TNAMSZ).
///     C
///           INCLUDE 'ekcnamsz.inc'
///           INCLUDE 'ekglimit.inc'
///           INCLUDE 'ekqlimit.inc'
///           INCLUDE 'ektnamsz.inc'
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               EKNSEG
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         TABLE
///           PARAMETER           ( TABLE   = 'DATAITEMS'      )
///
///           INTEGER               DECLEN
///           PARAMETER           ( DECLEN = 200 )
///
///           INTEGER               DESCLN
///           PARAMETER           ( DESCLN = 80  )
///
///     C
///     C     One value per row/column element.
///     C
///           INTEGER               MAXVAL
///           PARAMETER           ( MAXVAL = 1  )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 40  )
///
///           INTEGER               COLSLN
///           PARAMETER           ( COLSLN = 5   )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(DECLEN)    CDECLS ( MXCLSG )
///           CHARACTER*(CNAMSZ)    CNAMES ( MXCLSG )
///           CHARACTER*(NAMLEN)    CVALS  ( MAXVAL )
///           CHARACTER*(DESCLN)    DESCRP
///           CHARACTER*(4)         DTYPES ( MXCLSG )
///           CHARACTER*(NAMLEN)    ITEMNM
///           CHARACTER*(TNAMSZ)    TABNAM
///
///           DOUBLE PRECISION      DVALS  ( MAXVAL )
///           DOUBLE PRECISION      PRICE
///
///           INTEGER               ESIZE
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               ITEMID
///           INTEGER               IVALS  ( MAXVAL )
///           INTEGER               NCOLS
///           INTEGER               NROWS
///           INTEGER               NSEG
///           INTEGER               NVALS
///           INTEGER               ORDID
///           INTEGER               RECNO
///           INTEGER               SEGNO
///           INTEGER               SIZES  ( MXCLSG )
///           INTEGER               STRLNS ( MXCLSG )
///
///           LOGICAL               INDEXD ( MXCLSG )
///           LOGICAL               ISNULL
///           LOGICAL               NULLOK ( MXCLSG )
///
///     C
///     C     Open a scratch EK file to use for temporary
///     C     storage.
///     C
///           CALL EKOPS ( HANDLE )
///
///     C
///     C     Set up the table and column names and declarations
///     C     for the DATAITEMS segment. We'll index all of
///     C     the columns. All columns are scalar, so we omit
///     C     the size declaration.
///     C
///           CNAMES(1) =  'ITEM_ID'
///           CDECLS(1) =  'DATATYPE = INTEGER, INDEXED = TRUE'
///
///           CNAMES(2) =  'ORDER_ID'
///           CDECLS(2) =  'DATATYPE = INTEGER, INDEXED = TRUE'
///
///           CNAMES(3) =  'ITEM_NAME'
///           CDECLS(3) =  'DATATYPE = CHARACTER*(*),' //
///          .             'INDEXED  = TRUE'
///
///           CNAMES(4) =  'DESCRIPTION'
///           CDECLS(4) =  'DATATYPE = CHARACTER*(*),' //
///          .             'INDEXED  = TRUE'
///
///           CNAMES(5) =  'PRICE'
///           CDECLS(5) =  'DATATYPE = DOUBLE PRECISION,' //
///          .             'INDEXED  = TRUE'
///
///
///     C
///     C     Start the segment. Since we have no data for this
///     C     segment, start the segment by just defining the new
///     C     segment's schema.
///     C
///           CALL EKBSEG ( HANDLE, TABLE,  COLSLN,
///          .              CNAMES, CDECLS, SEGNO  )
///
///     C
///     C     Append a new, empty record to the DATAITEMS
///     C     table. Recall that the DATAITEMS table
///     C     is in the first segment.  The call will return
///     C     the number of the new, empty record.
///     C
///           SEGNO = 1
///           CALL EKAPPR ( HANDLE, SEGNO, RECNO )
///
///     C
///     C     At this point, the new record is empty. We fill in the
///     C     data here. Data items are filled in one column at a
///     C     time. The order in which the columns are filled in is
///     C     not important.  We use the different add column entry
///     C     routines to fill in column entries.  We'll assume
///     C     that no entries are null. All entries are scalar,
///     C     so the entry size is 1.
///     C
///           ISNULL   =  .FALSE.
///           ESIZE    =  1
///
///     C
///     C     The following variables will contain the data for
///     C     the new record.
///     C
///           ORDID    =   10011
///           ITEMID   =   531
///           ITEMNM   =  'Sample item'
///           DESCRP   =  'This sample item is used only in tests.'
///           PRICE    =   1345.67D0
///
///     C
///     C     Note that the names of the routines called
///     C     correspond to the data types of the columns.
///     C
///           CALL EKACEI ( HANDLE, SEGNO,  RECNO, 'ORDER_ID',
///          .              ESIZE,  ORDID,  ISNULL               )
///
///           CALL EKACEI ( HANDLE, SEGNO,  RECNO, 'ITEM_ID',
///          .              ESIZE,  ITEMID, ISNULL               )
///
///           CALL EKACEC ( HANDLE, SEGNO,  RECNO, 'ITEM_NAME',
///          .              ESIZE,  ITEMNM, ISNULL               )
///
///           CALL EKACEC ( HANDLE, SEGNO,  RECNO, 'DESCRIPTION',
///          .              ESIZE,  DESCRP, ISNULL               )
///
///           CALL EKACED ( HANDLE, SEGNO,  RECNO, 'PRICE',
///          .              ESIZE,  PRICE,  ISNULL               )
///
///
///     C
///     C     At this point, we could perform read operations
///     C     on the EK.
///     C
///     C     Return the number of segments in the EK. Dump the
///     C     desired summary information for each one.
///     C
///           NSEG = EKNSEG( HANDLE )
///           WRITE(*,'(A,I3)') 'Number of segments =', NSEG
///           WRITE(*,*)
///
///           DO SEGNO = 1, NSEG
///
///              CALL EKSSUM (  HANDLE,  SEGNO,   TABNAM,  NROWS,
///          .                  NCOLS,   CNAMES,  DTYPES,  SIZES,
///          .                  STRLNS,  INDEXD,  NULLOK         )
///
///              WRITE(*,'(2A)')   'Table containing segment: ', TABNAM
///              WRITE(*,'(A,I2)') 'Number of rows          : ', NROWS
///              WRITE(*,'(A,I2)') 'Number of columns       : ', NCOLS
///              WRITE(*,'(A)')    'Table data              : '
///
///              DO I = 1, NCOLS
///
///                 WRITE(*,'(2A)') '  Column: ', CNAMES(I)
///                 WRITE(*,'(2A)') '  Type  : ', DTYPES(I)
///
///                 DO RECNO = 1, NROWS
///
///                    IF ( DTYPES(I) .EQ. 'CHR' ) THEN
///
///                       CALL EKRCEC ( HANDLE,    SEGNO, RECNO,
///          .                          CNAMES(I), NVALS,
///          .                          CVALS,     ISNULL       )
///
///                       IF ( ISNULL ) THEN
///
///                          WRITE(*,'(A)') '  Data  : <null>'
///
///                       ELSE
///
///                          WRITE(*,'(2A)') '  Data  : ', CVALS
///
///                       END IF
///
///                    ELSE IF ( DTYPES(I) .EQ. 'DP' ) THEN
///
///                       CALL EKRCED ( HANDLE,    SEGNO, RECNO,
///          .                          CNAMES(I), NVALS,
///          .                          DVALS,     ISNULL       )
///
///                       IF ( ISNULL ) THEN
///
///                          WRITE(*,'(A)') '  Data  : <null>'
///
///                       ELSE
///
///                          WRITE(*,'(A,F9.2)') '  Data  : ', DVALS
///
///                       END IF
///
///                    ELSE IF ( DTYPES(I) .EQ. 'INT' ) THEN
///
///                       CALL EKRCEI ( HANDLE,    SEGNO, RECNO,
///          .                          CNAMES(I), NVALS,
///          .                          IVALS,     ISNULL       )
///
///                       IF ( ISNULL ) THEN
///
///                          WRITE(*,'(A)') '  Data  : <null>'
///
///                       ELSE
///
///                          WRITE(*,'(A,I6)') '  Data  : ', IVALS
///
///                       END IF
///
///                    ENDIF
///
///     C
///     C              There is no time data. Otherwise, we would need
///     C              to use an LSK and EKRCED to read it
///     C              (internally, it is stored as double precision).
///     C
///                    WRITE(*,*)
///
///                 END DO
///
///              END DO
///
///              WRITE(*,*) '----------------------------------------'
///
///           END DO
///
///     C
///     C     Close the file. This will delete the scratch file
///     C     and all the data will be lost.
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
///     Number of segments =  1
///
///     Table containing segment: DATAITEMS
///     Number of rows          :  1
///     Number of columns       :  5
///     Table data              :
///       Column: ITEM_ID
///       Type  : INT
///       Data  :    531
///
///       Column: ORDER_ID
///       Type  : INT
///       Data  :  10011
///
///       Column: ITEM_NAME
///       Type  : CHR
///       Data  : Sample item
///
///       Column: DESCRIPTION
///       Type  : CHR
///       Data  : This sample item is used only in tests.
///
///       Column: PRICE
///       Type  : DP
///       Data  :   1345.67
///
///      ----------------------------------------
///
///
///     Note that after run completion, there is no EK file in the
///     output directory as scratch files are deleted when they are
///     closed or when the calling program terminates.
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
///         Edited the header to comply with NAIF standard and improved
///         the API documentation. Added complete code example and updated
///         $Parameters section.
///
/// -    SPICELIB Version 1.0.0, 26-SEP-1995 (NJB)
/// ```
pub fn ekops(ctx: &mut SpiceContext, handle: &mut i32) -> crate::Result<()> {
    EKOPS(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKOPS ( EK, open scratch file )
pub fn EKOPS(HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BASE: i32 = 0;
    let mut P: i32 = 0;

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
    } else {
        CHKIN(b"EKOPS", ctx)?;
    }

    DASOPS(HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"EKOPS", ctx)?;
        return Ok(());
    }

    //
    // Initialize the file for paged access.  The EK architecture
    // code is automatically set by the paging initialization routine.
    //
    ZZEKPGIN(*HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"EKOPS", ctx)?;
        return Ok(());
    }

    //
    // Allocate the first integer page for the file's metadata.  We
    // don't need to examine the page number; it's 1.
    //
    ZZEKPGAN(*HANDLE, INT, &mut P, &mut BASE, ctx)?;

    //
    // Initialize a new tree.  This tree will point to the file's
    // segments.
    //
    ZZEKTRIT(*HANDLE, &mut P, ctx)?;

    //
    // Save the segment pointer's root page number.
    //
    DASUDI(*HANDLE, (BASE + SGTIDX), (BASE + SGTIDX), &[P], ctx)?;

    //
    // That's it.  We're ready to add data to the file.
    //
    CHKOUT(b"EKOPS", ctx)?;
    Ok(())
}
