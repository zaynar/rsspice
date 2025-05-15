//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CDOFF: i32 = 24;
const CDSCSZ: i32 = 11;
const CLSIDX: i32 = 1;
const TYPIDX: i32 = (CLSIDX + 1);
const LENIDX: i32 = (TYPIDX + 1);
const SIZIDX: i32 = (LENIDX + 1);
const NAMIDX: i32 = (SIZIDX + 1);
const IXTIDX: i32 = (NAMIDX + 1);
const IXPIDX: i32 = (IXTIDX + 1);
const NFLIDX: i32 = (IXPIDX + 1);
const ORDIDX: i32 = (NFLIDX + 1);
const METIDX: i32 = (ORDIDX + 1);
const SDSCSZ: i32 = 24;
const EKTIDX: i32 = 1;
const SNOIDX: i32 = (EKTIDX + 1);
const IMDIDX: i32 = (SNOIDX + 1);
const TNMIDX: i32 = (IMDIDX + 1);
const NCIDX: i32 = (TNMIDX + 1);
const NRIDX: i32 = (NCIDX + 1);
const RTIDX: i32 = (NRIDX + 1);
const CPTIDX: i32 = (RTIDX + 1);
const DPTIDX: i32 = (CPTIDX + 1);
const IPTIDX: i32 = (DPTIDX + 1);
const MFLIDX: i32 = (IPTIDX + 1);
const IFLIDX: i32 = (MFLIDX + 1);
const SHDIDX: i32 = (IFLIDX + 1);
const CFHIDX: i32 = (SHDIDX + 1);
const CSNIDX: i32 = (CFHIDX + 1);
const LCPIDX: i32 = (CSNIDX + 1);
const LDPIDX: i32 = (LCPIDX + 1);
const LIPIDX: i32 = (LDPIDX + 1);
const LCWIDX: i32 = (LIPIDX + 1);
const LDWIDX: i32 = (LCWIDX + 1);
const LIWIDX: i32 = (LDWIDX + 1);
const NMLIDX: i32 = (LIWIDX + 1);
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;

/// EK, add integer data to column
///
/// Add data to an integer column in a specified EK record.
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
///  HANDLE     I   EK file handle.
///  SEGNO      I   Index of segment containing record.
///  RECNO      I   Record to which data is to be added.
///  COLUMN     I   Column name.
///  NVALS      I   Number of values to add to column.
///  IVALS      I   Integer values to add to column.
///  ISNULL     I   Flag indicating whether column entry is null.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of an EK file open for write access.
///
///  SEGNO    is the index of the segment to which data is to
///           be added.
///
///  RECNO    is the index of the record to which data is to be
///           added. This record number is relative to the start
///           of the segment indicated by SEGNO; the first
///           record in the segment has index 1.
///
///  COLUMN   is the name of the column to which data is to be
///           added.
///
///  NVALS,
///  IVALS    are, respectively, the number of values to add to
///           the specified column and the set of values
///           themselves. The data values are written into the
///           specified column and record.
///
///           If the column has fixed-size entries, then NVALS
///           must equal the entry size for the specified column.
///
///           Only one value can be added to a virtual column.
///
///
///  ISNULL   is a logical flag indicating whether the entry is
///           null. If ISNULL is .FALSE., the column entry
///           defined by NVALS and IVALS is added to the
///           specified kernel file.
///
///           If ISNULL is .TRUE., NVALS and IVALS are ignored.
///           The contents of the column entry are undefined.
///           If the column has fixed-length, variable-size
///           entries, the number of entries is considered to
///           be 1.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the effect of this
///  routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If HANDLE is invalid, an error is signaled by a routine in the
///      call tree of this routine.
///
///  2)  If SEGNO is out of range, an error is signaled by a routine in
///      the call tree of this routine.
///
///  3)  If COLUMN is not the name of a declared column, an error
///      is signaled by a routine in the call tree of this routine.
///
///  4)  If COLUMN specifies a column of whose data type is not
///      integer, the error SPICE(WRONGDATATYPE) is signaled.
///
///  5)  If RECNO is out of range, an error is signaled by a routine in
///      the call tree of this routine.
///
///  6)  If the specified column has fixed-size entries and NVALS does
///      not match this size, an error is signaled by a routine in the
///      call tree of this routine.
///
///  7)  If the specified column has variable-size entries and NVALS is
///      non-positive, an error is signaled by a routine in the call
///      tree of this routine.
///
///  8)  If an attempt is made to add a null value to a column that
///      doesn't take null values, an error is signaled by a routine in
///      the call tree of this routine.
///
///  9)  If COLUMN specifies a column of whose class is not
///      an character class known to this routine, the error
///      SPICE(NOCLASS) is signaled.
///
///  10) If an I/O error occurs while reading or writing the indicated
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
///  This routine operates by side effects: it modifies the named
///  EK file by adding data to the specified record in the specified
///  column. Data may be added to a segment in random order; it is not
///  necessary to fill in columns or rows sequentially. Data may only
///  be added one column entry at a time.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) This example demonstrates how to add integer values
///     to a column in three different cases: single values,
///     variable-size arrays and static-size arrays.
///
///     Create an EK that contains a table TAB that has the following
///     columns:
///
///        Column name   Data Type   Size
///        -----------   ---------   ----
///        INT_COL_1     INT         1
///        INT_COL_2     INT         VARIABLE
///        INT_COL_3     INT         3
///
///
///     Issue the following query
///
///         QUERY = 'SELECT INT_COL_1, INT_COL2, INT_COL3 FROM TAB'
///
///     to fetch and dump column values from the rows that satisfy the
///     query.
///
///
///     Example code begins here.
///
///
///           PROGRAM EKACEI_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Column Name Size (CNAMSZ)
///     C     and EK Query Limit Parameters (MAXQRY)
///     C
///           INCLUDE 'ekcnamsz.inc'
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME  = 'ekacei_ex1.bdb' )
///
///           CHARACTER*(*)         TABLE
///           PARAMETER           ( TABLE   = 'TAB' )
///
///           INTEGER               COL3SZ
///           PARAMETER           ( COL3SZ = 3   )
///
///           INTEGER               DECLEN
///           PARAMETER           ( DECLEN = 200 )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///           INTEGER               MXC2SZ
///           PARAMETER           ( MXC2SZ = 4   )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 40  )
///
///           INTEGER               NCOLS
///           PARAMETER           ( NCOLS  = 3   )
///
///           INTEGER               NROWS
///           PARAMETER           ( NROWS  = 4   )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(DECLEN)    CDECLS ( NCOLS  )
///           CHARACTER*(CNAMSZ)    CNAMES ( NCOLS  )
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(MAXQRY)    QUERY
///
///           INTEGER               COL1
///           INTEGER               COL2   ( MXC2SZ )
///           INTEGER               COL3   ( COL3SZ )
///           INTEGER               ELTIDX
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               IVALS  ( MXC2SZ )
///           INTEGER               J
///           INTEGER               NELT
///           INTEGER               NMROWS
///           INTEGER               NRESVC
///           INTEGER               RECNO
///           INTEGER               ROW
///           INTEGER               SEGNO
///           INTEGER               SELIDX
///
///           LOGICAL               ERROR
///           LOGICAL               FOUND
///           LOGICAL               ISNULL
///
///     C
///     C     Open a new EK file.  For simplicity, we will not
///     C     reserve any space for the comment area, so the
///     C     number of reserved comment characters is zero.
///     C     The variable IFNAME is the internal file name.
///     C
///           NRESVC  =  0
///           IFNAME  =  'Test EK/Created 13-JUN-2019'
///
///           CALL EKOPN ( EKNAME, IFNAME, NRESVC, HANDLE )
///
///     C
///     C     Set up the column names and declarations
///     C     for the TAB segment.  We'll index all of
///     C     the columns.
///     C
///           CNAMES(1) = 'INT_COL_1'
///           CDECLS(1) = 'DATATYPE = INTEGER, INDEXED  = TRUE'
///
///           CNAMES(2) = 'INT_COL_2'
///           CDECLS(2) = 'DATATYPE = INTEGER, SIZE = VARIABLE, ' //
///          .            'NULLS_OK = TRUE'
///
///           CNAMES(3) = 'INT_COL_3'
///           CDECLS(3) = 'DATATYPE = INTEGER, SIZE = 3'
///
///     C
///     C     Start the segment.
///     C
///           CALL EKBSEG ( HANDLE, TABLE,  NCOLS,
///          .              CNAMES, CDECLS, SEGNO )
///
///     C
///     C     At the records to the table.
///     C
///           DO I = 1, NROWS
///
///     C
///     C        Append a new record to the EK.
///     C
///              CALL EKAPPR ( HANDLE, SEGNO, RECNO )
///
///     C
///     C        Add INT_COL_1
///     C
///              COL1 = I * 100
///
///              CALL EKACEI ( HANDLE,    SEGNO, RECNO,
///          .                 CNAMES(1), 1,     COL1,  .FALSE. )
///
///     C
///     C        Add I items to INT_COL_2
///     C
///              DO J = 1, I
///                 COL2(J) = J + I*200
///              END DO
///
///              ISNULL = ( I .EQ. 2 )
///
///              CALL EKACEI ( HANDLE,    SEGNO, RECNO,
///          .                 CNAMES(2), I,     COL2,  ISNULL )
///
///     C
///     C        Add 3 items to INT_COL_3
///     C
///              DO J = 1, 3
///                 COL3(J) =  I + J*100.D0
///              END DO
///
///              CALL EKACEI ( HANDLE,    SEGNO, RECNO,
///          .                 CNAMES(3), 3,     COL3, .FALSE. )
///
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL EKCLS ( HANDLE )
///
///     C
///     C     Open the created file. Perform the query and show the
///     C     results.
///     C
///           CALL FURNSH ( EKNAME )
///
///           QUERY = 'SELECT INT_COL_1, INT_COL_2, INT_COL_3 '
///          .   //   'FROM TAB'
///
///     C
///     C     Query the EK system for data rows matching the
///     C     SELECT constraints.
///     C
///           CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///     C
///     C     Check whether an error occurred while processing the
///     C     SELECT clause. If so, output the error message.
///     C
///           IF ( ERROR ) THEN
///
///              WRITE(*,*) 'SELECT clause error: ', ERRMSG
///
///           ELSE
///
///              DO ROW = 1, NMROWS
///
///                 WRITE(*,*) ' '
///                 WRITE(*,'(A,I3)') 'ROW  = ', ROW
///
///     C
///     C           Fetch values from column INT_COL_1.  Since
///     C           INT_COL_1 was the first column selected, the
///     C           selection index SELIDX is set to 1.
///     C
///                 SELIDX = 1
///                 ELTIDX = 1
///                 CALL EKGI ( SELIDX,    ROW,     ELTIDX,
///          .                  IVALS(1),  ISNULL,  FOUND   )
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE(*,*) '  COLUMN = INT_COL_1: <Null>'
///
///                 ELSE
///
///                    WRITE(*,'(A,I6)') '   COLUMN = INT_COL_1:',
///          .                           IVALS(1)
///
///                 END IF
///
///     C
///     C           Fetch values from column INT_COL_2 in the current
///     C           row.  Since INT_COL_2 contains variable-size array
///     C           elements, we call EKNELT to determine how many
///     C           elements to fetch.
///     C
///                 SELIDX = 2
///                 CALL EKNELT ( SELIDX, ROW, NELT )
///
///                 ELTIDX = 1
///                 ISNULL = .FALSE.
///
///                 DO WHILE (       ( ELTIDX .LE.  NELT   )
///          .                 .AND. (        .NOT. ISNULL )  )
///
///                    CALL EKGI ( SELIDX,         ROW,     ELTIDX,
///          .                     IVALS(ELTIDX),  ISNULL,  FOUND   )
///
///                    ELTIDX = ELTIDX + 1
///
///     C
///     C           If the column entry is null, we'll be kicked
///     C           out of this loop after the first iteration.
///     C
///                 END DO
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE(*,*) '  COLUMN = INT_COL_2: <Null>'
///
///                 ELSE
///
///                    WRITE(*,'(A,4I6)') '   COLUMN = INT_COL_2:',
///          .                            ( IVALS(I), I = 1, NELT )
///
///                 END IF
///
///     C
///     C           Fetch values from column INT_COL_3 in the current
///     C           row.  We need not call EKNELT since we know how
///     C           many elements are in each column entry.
///     C
///                 SELIDX = 3
///                 ELTIDX = 1
///                 ISNULL = .FALSE.
///
///                 DO WHILE (       ( ELTIDX .LE.  COL3SZ )
///          .                 .AND. (        .NOT. ISNULL )  )
///
///                    CALL EKGI ( SELIDX,         ROW,     ELTIDX,
///          .                     IVALS(ELTIDX),  ISNULL,  FOUND   )
///
///                    ELTIDX = ELTIDX + 1
///
///                 END DO
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE(*,*) '  COLUMN = INT_COL_3: <Null>'
///
///                 ELSE
///
///                    WRITE(*,'(A,3I6)') '   COLUMN = INT_COL_3:',
///          .                            ( IVALS(I), I = 1, COL3SZ )
///
///                 END IF
///
///              END DO
///
///     C
///     C     We either parsed the SELECT clause or had an error.
///     C
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     ROW  =   1
///        COLUMN = INT_COL_1:   100
///        COLUMN = INT_COL_2:   201
///        COLUMN = INT_COL_3:   101   201   301
///
///     ROW  =   2
///        COLUMN = INT_COL_1:   200
///        COLUMN = INT_COL_2: <Null>
///        COLUMN = INT_COL_3:   102   202   302
///
///     ROW  =   3
///        COLUMN = INT_COL_1:   300
///        COLUMN = INT_COL_2:   601   602   603
///        COLUMN = INT_COL_3:   103   203   303
///
///     ROW  =   4
///        COLUMN = INT_COL_1:   400
///        COLUMN = INT_COL_2:   801   802   803   804
///        COLUMN = INT_COL_3:   104   204   304
///
///
///     Note that after run completion, a new EK file exists in the
///     output directory.
///
///
///  2) Suppose we want to create an E-kernel which contains a table
///     of items that have been ordered. The columns of this table
///     are shown below:
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
///     This EK file will have one segment containing the DATAITEMS
///     table.
///
///     This examples demonstrates how to open a new EK file; create
///     the segment described above and how to insert a new record
///     into it.
///
///
///     Example code begins here.
///
///
///           PROGRAM EKACEI_EX2
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Column Name Size (CNAMSZ)
///     C
///           INCLUDE 'ekcnamsz.inc'
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME  = 'ekacei_ex2.bdb' )
///
///           CHARACTER*(*)         TABLE
///           PARAMETER           ( TABLE   = 'DATAITEMS'      )
///
///           INTEGER               DECLEN
///           PARAMETER           ( DECLEN = 200 )
///
///           INTEGER               DESCLN
///           PARAMETER           ( DESCLN = 80  )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 40  )
///
///           INTEGER               NCOLS
///           PARAMETER           ( NCOLS  = 5   )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(DECLEN)    CDECLS ( NCOLS )
///           CHARACTER*(CNAMSZ)    CNAMES ( NCOLS )
///           CHARACTER*(DESCLN)    DESCRP
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(NAMLEN)    ITEMNM
///
///           DOUBLE PRECISION      PRICE
///
///           INTEGER               ESIZE
///           INTEGER               HANDLE
///           INTEGER               ITEMID
///           INTEGER               NRESVC
///           INTEGER               ORDID
///           INTEGER               RECNO
///           INTEGER               SEGNO
///
///           LOGICAL               ISNULL
///
///     C
///     C     Open a new EK file.  For simplicity, we will not
///     C     reserve any space for the comment area, so the
///     C     number of reserved comment characters is zero.
///     C     The variable IFNAME is the internal file name.
///     C
///           NRESVC  =  0
///           IFNAME  =  'Test EK;Created 21-JUN-2019'
///
///           CALL EKOPN ( EKNAME, IFNAME, NRESVC, HANDLE )
///
///     C
///     C     Set up the table and column names and declarations
///     C     for the DATAITEMS segment.  We'll index all of
///     C     the columns.  All columns are scalar, so we omit
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
///           CALL EKBSEG ( HANDLE, TABLE,  NCOLS,
///          .              CNAMES, CDECLS, SEGNO )
///
///     C
///     C     Append a new, empty record to the DATAITEMS
///     C     table. Recall that the DATAITEMS table
///     C     is in segment number 1.  The call will return
///     C     the number of the new, empty record.
///     C
///           SEGNO = 1
///           CALL EKAPPR ( HANDLE, SEGNO, RECNO )
///
///     C
///     C     At this point, the new record is empty.  A valid EK
///     C     cannot contain empty records.  We fill in the data
///     C     here.  Data items are filled in one column at a time.
///     C     The order in which the columns are filled in is not
///     C     important.  We use the EKACEx (add column entry)
///     C     routines to fill in column entries.  We'll assume
///     C     that no entries are null.  All entries are scalar,
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
///           PRICE    =   1345.678D0
///
///     C
///     C     Note that the names of the routines called
///     C     correspond to the data types of the columns:  the
///     C     last letter of the routine name is C, I, or D,
///     C     depending on the data type.
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
///     C
///     C     Close the file to make the update permanent.
///     C
///           CALL EKCLS ( HANDLE )
///
///           END
///
///
///     When this program is executed, no output is presented on
///     screen. After run completion, a new EK file exists in the
///     output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code examples from existing fragments.
///
/// -    SPICELIB Version 1.2.0, 05-FEB-2015 (NJB)
///
///         Updated to use ERRHAN.
///
/// -    SPICELIB Version 1.1.0, 18-JUN-1999 (WLT)
///
///         Removed an unbalanced call to CHKOUT.
///
/// -    SPICELIB Version 1.0.0, 26-SEP-1995 (NJB)
/// ```
pub fn ekacei(
    ctx: &mut SpiceContext,
    handle: i32,
    segno: &mut i32,
    recno: i32,
    column: &str,
    nvals: i32,
    ivals: &[i32],
    isnull: bool,
) -> crate::Result<()> {
    EKACEI(
        handle,
        segno,
        recno,
        column.as_bytes(),
        nvals,
        ivals,
        isnull,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKACEI ( EK, add integer data to column )
pub fn EKACEI(
    HANDLE: i32,
    SEGNO: &mut i32,
    RECNO: i32,
    COLUMN: &[u8],
    NVALS: i32,
    IVALS: &[i32],
    ISNULL: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let IVALS = DummyArray::new(IVALS, 1..);
    let mut COLDSC = StackArray::<i32, 11>::new(1..=CDSCSZ);
    let mut CLASS: i32 = 0;
    let mut DTYPE: i32 = 0;
    let mut RECPTR: i32 = 0;
    let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    // First step:  find the descriptor for the named segment.  Using
    // this descriptor, get the column descriptor.
    //
    ZZEKSDSC(HANDLE, *SEGNO, SEGDSC.as_slice_mut(), ctx)?;
    ZZEKCDSC(
        HANDLE,
        SEGDSC.as_slice(),
        COLUMN,
        COLDSC.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // This column had better be of integer type.
    //
    DTYPE = COLDSC[TYPIDX];

    if (DTYPE != INT) {
        CHKIN(b"EKACEI", ctx)?;
        SETMSG(b"Column # is of type #; EKACEI only works with integer columns.  RECNO = #; SEGNO = #; EK = #.", ctx);
        ERRCH(b"#", COLUMN, ctx);
        ERRINT(b"#", DTYPE, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", *SEGNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(WRONGDATATYPE)", ctx)?;
        CHKOUT(b"EKACEI", ctx)?;
        return Ok(());
    }

    //
    // Look up the record pointer for the target record.
    //
    ZZEKTRDP(HANDLE, SEGDSC[RTIDX], RECNO, &mut RECPTR, ctx)?;
    //
    //
    // Now it's time to add data to the file.
    //
    CLASS = COLDSC[CLSIDX];

    if (CLASS == 1) {
        //
        // Class 1 columns contain scalar integer data.
        //
        ZZEKAD01(
            HANDLE,
            SEGDSC.as_slice_mut(),
            COLDSC.as_slice(),
            RECPTR,
            *IVALS.first(),
            ISNULL,
            ctx,
        )?;
    } else if (CLASS == 4) {
        //
        // Class 4 columns contain array-valued integer data.
        //
        ZZEKAD04(
            HANDLE,
            SEGDSC.as_slice_mut(),
            COLDSC.as_slice(),
            RECPTR,
            NVALS,
            IVALS.as_slice(),
            ISNULL,
            ctx,
        )?;
    } else {
        //
        // This is an unsupported integer column class.
        //
        *SEGNO = SEGDSC[SNOIDX];

        CHKIN(b"EKACEI", ctx)?;
        SETMSG(b"Class # from input column descriptor is not a supported integer class.  COLUMN = #; RECNO = #; SEGNO = #; EK = #.", ctx);
        ERRINT(b"#", CLASS, ctx);
        ERRCH(b"#", COLUMN, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", *SEGNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(NOCLASS)", ctx)?;
        CHKOUT(b"EKACEI", ctx)?;
        return Ok(());
    }

    Ok(())
}
