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

/// EK, add character data to column
///
/// Add data to a character column in a specified EK record.
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
///  CVALS      I   Character values to add to column.
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
///  CVALS    are, respectively, the number of values to add to
///           the specified column and the set of values
///           themselves. The data values are written into the
///           specified column and record.
///
///           If the  column has fixed-size entries, then NVALS
///           must equal the entry size for the specified column.
///
///           Only one value can be added to a virtual column.
///
///
///  ISNULL   is a logical flag indicating whether the entry is
///           null. If ISNULL is .FALSE., the column entry
///           defined by NVALS and CVALS is added to the
///           specified kernel file.
///
///           If ISNULL is .TRUE., NVALS and CVALS are ignored.
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
///      character, the error SPICE(WRONGDATATYPE) is signaled.
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
///  1) The following program demonstrates how to create a new EK and
///     add data to a character column in a given record within the
///     file, and how to read the data from it.
///
///     Example code begins here.
///
///
///           PROGRAM EKACEC_EX1
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
///           PARAMETER           ( EKNAME  = 'ekacec_ex1.bdb' )
///
///           CHARACTER*(*)         TABLE
///           PARAMETER           ( TABLE   = 'TABLENAME'      )
///
///           INTEGER               CBUFSZ
///           PARAMETER           ( CBUFSZ = 4   )
///
///           INTEGER               DECLEN
///           PARAMETER           ( DECLEN = 200 )
///
///           INTEGER               LINESZ
///           PARAMETER           ( LINESZ = 6   )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 40  )
///
///           INTEGER               NCOLS
///           PARAMETER           ( NCOLS  = 2   )
///
///           INTEGER               STRLEN
///           PARAMETER           ( STRLEN = 6   )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(STRLEN)    CBUF   ( CBUFSZ )
///           CHARACTER*(DECLEN)    CDECLS ( NCOLS  )
///           CHARACTER*(CNAMSZ)    CNAMES ( NCOLS  )
///           CHARACTER*(LINESZ)    CVALS  ( CBUFSZ )
///           CHARACTER*(NAMLEN)    IFNAME
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               J
///           INTEGER               K
///           INTEGER               NRESVC
///           INTEGER               NVALS
///           INTEGER               RECNO
///           INTEGER               SEGNO
///
///           LOGICAL               ISNULL
///
///     C
///     C     Create a list of character strings.
///     C
///           DATA                  CBUF / 'CHSTR1', 'CHSTR2',
///          .                             'CHSTR3', 'CHSTR4' /
///
///     C
///     C     Open a new EK file.  For simplicity, we will not
///     C     reserve any space for the comment area, so the
///     C     number of reserved comment characters is zero.
///     C     The variable IFNAME is the internal file name.
///     C
///           NRESVC  =  0
///           IFNAME  =  'Test EK/Created 31-MAY-2019'
///
///           CALL EKOPN ( EKNAME, IFNAME, NRESVC, HANDLE )
///
///     C
///     C     Define the column names and formats.
///     C
///           CNAMES(1) = 'CCOL'
///           CDECLS(1) = 'DATATYPE = CHARACTER*(*), ' //
///          .            'INDEXED = TRUE, NULLS_OK = TRUE'
///
///           CNAMES(2) = 'CARRAY'
///           CDECLS(2) = 'DATATYPE = CHARACTER*(6), ' //
///          .            'SIZE = VARIABLE, NULLS_OK = TRUE'
///
///     C
///     C     Start the segment.
///     C
///           CALL EKBSEG ( HANDLE, TABLE,  NCOLS,
///          .              CNAMES, CDECLS, SEGNO )
///
///     C
///     C     Append a new record to the EK.
///     C
///           CALL EKAPPR ( HANDLE, SEGNO, RECNO )
///
///     C
///     C     Add the value '999' to the first record of the column
///     C     CCOL in the SEGNO segment of the EK file designated
///     C     by HANDLE.
///     C
///           CALL EKACEC ( HANDLE, SEGNO, RECNO,
///          .              'CCOL', 1,     '999', .FALSE. )
///
///     C
///     C     Add an array CBUF of 4 values to the first record of
///     C     the column CARRAY in the SEGNO segment of the EK file
///     C     designated by HANDLE.
///     C
///           CALL EKACEC ( HANDLE,   SEGNO,  RECNO,
///          .              'CARRAY', CBUFSZ, CBUF, .FALSE. )
///
///     C
///     C     Append a second record to the EK.
///     C
///           CALL EKAPPR ( HANDLE, SEGNO, RECNO )
///
///     C
///     C     Repeat the operation again for the second record, but
///     C     this time, add only 2 values of CBUF.
///     C
///           CALL EKACEC ( HANDLE,   SEGNO, RECNO,
///          .              'CARRAY', 2,     CBUF, .FALSE. )
///
///     C
///     C     Add a null value to the CCOL in the second record.
///     C     The argument 999 is ignored because the null flag is
///     C     set to .TRUE.
///     C
///           CALL EKACEC ( HANDLE, SEGNO, RECNO,
///          .              'CCOL', 1,     '999', .TRUE. )
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
///     C
///     C     The file we have created has only one segment and
///     C     two records within. Each record has two columns.
///     C
///           SEGNO = 1
///
///     C
///     C     Go over each record...
///     C
///           DO I = 1, 2
///
///              WRITE(*,'(A,I4)') 'Record', I
///
///     C
///     C        ... and each column.
///     C
///              DO J = 1, NCOLS
///
///     C
///     C        Read the data from the first column.
///     C
///                 CALL EKRCEC ( HANDLE, SEGNO, I, CNAMES(J),
///          .                    NVALS,  CVALS,    ISNULL    )
///
///                 IF ( ISNULL ) THEN
///
///                    WRITE(*,'(A,A6,A)') '   ', CNAMES(J), ': NULL '
///
///                 ELSE
///
///                    WRITE(*,'(A,A6,A,4A9)') '   ', CNAMES(J), ': ',
///          .                              ( CVALS(K), K = 1, NVALS )
///
///                 END IF
///
///              END DO
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Record   1
///        CCOL  :    999
///        CARRAY:    CHSTR1   CHSTR2   CHSTR3   CHSTR4
///     Record   2
///        CCOL  : NULL
///        CARRAY:    CHSTR1   CHSTR2
///
///
///     Note that after run completion, a new EK file exists in the
///     output directory.
///
///
///  2) A more detailed example.
///
///     Suppose we have an E-kernel which contains records of orders
///     for data products. The E-kernel has a table called DATAORDERS
///     that consists of the set of columns listed below:
///
///        DATAORDERS
///
///           Column Name     Data Type
///           -----------     ---------
///           ORDER_ID        INTEGER
///           CUSTOMER_ID     INTEGER
///           LAST_NAME       CHARACTER*(*)
///           FIRST_NAME      CHARACTER*(*)
///           ORDER_DATE      TIME
///           COST            DOUBLE PRECISION
///
///     The order database also has a table of items that have been
///     ordered. The columns of this table are shown below:
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
///     We'll suppose that the EK file contains two segments, the
///     first containing the DATAORDERS table and the second
///     containing the DATAITEMS table.
///
///     This examples demonstrates how to open a new EK file; create
///     the two segments described above, using fast writers; and
///     how to insert a new record into one of the tables.
///
///
///     Use the LSK kernel below to load the leap seconds and time
///     constants required for the conversions.
///
///        naif0012.tls
///
///
///     Example code begins here.
///
///
///           PROGRAM EKACEC_EX2
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
///           PARAMETER           ( EKNAME  = 'ekacec_ex2.bes' )
///
///           CHARACTER*(*)         LSK
///           PARAMETER           ( LSK     = 'naif0012.tls'   )
///
///           CHARACTER*(*)         TABLE
///           PARAMETER           ( TABLE   = 'DATAORDERS'     )
///
///           INTEGER               DECLEN
///           PARAMETER           ( DECLEN = 200 )
///
///           INTEGER               DESCLN
///           PARAMETER           ( DESCLN = 80  )
///
///           INTEGER               FNMLEN
///           PARAMETER           ( FNMLEN = 50  )
///
///           INTEGER               LNMLEN
///           PARAMETER           ( LNMLEN = 50  )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 40  )
///
///           INTEGER               NCOLS
///           PARAMETER           ( NCOLS  = 6   )
///
///           INTEGER               NROWS
///           PARAMETER           ( NROWS  = 9   )
///
///           INTEGER               UTCLEN
///           PARAMETER           ( UTCLEN = 30  )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(DECLEN)    CDECLS ( NCOLS )
///           CHARACTER*(CNAMSZ)    CNAMES ( NCOLS )
///           CHARACTER*(DESCLN)    DESCRP
///           CHARACTER*(FNMLEN)    FNAMES ( NROWS )
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(NAMLEN)    ITEMNM
///           CHARACTER*(LNMLEN)    LNAMES ( NROWS )
///           CHARACTER*(UTCLEN)    ODATE
///
///           DOUBLE PRECISION      COSTS  ( NROWS )
///           DOUBLE PRECISION      ETS    ( NROWS )
///           DOUBLE PRECISION      PRICE
///
///           INTEGER               CSTIDS ( NROWS )
///           INTEGER               ESIZE
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               ITEMID
///           INTEGER               NRESVC
///           INTEGER               ORDID
///           INTEGER               ORDIDS ( NROWS )
///           INTEGER               RCPTRS ( NROWS )
///           INTEGER               RECNO
///           INTEGER               SEGNO
///           INTEGER               SIZES  ( NROWS )
///           INTEGER               WKINDX ( NROWS )
///
///           LOGICAL               ISNULL
///           LOGICAL               NLFLGS ( NROWS )
///
///     C
///     C     Load a leapseconds kernel for UTC/ET conversion.
///     C
///           CALL FURNSH ( 'naif0012.tls' )
///
///     C
///     C     Open a new EK file.  For simplicity, we will not
///     C     reserve any space for the comment area, so the
///     C     number of reserved comment characters is zero.
///     C     The variable IFNAME is the internal file name.
///     C
///           NRESVC  =  0
///           IFNAME  =  'Test EK/Created 01-JUN-2019'
///
///           CALL EKOPN ( EKNAME, IFNAME, NRESVC, HANDLE )
///
///     C
///     C     Set up the table and column names and declarations
///     C     for the DATAORDERS segment.  We'll index all of
///     C     the columns.  All columns are scalar, so we omit
///     C     the size declaration.  Only the COST column may take
///     C     null values.
///     C
///           CNAMES(1) =  'ORDER_ID'
///           CDECLS(1) =  'DATATYPE = INTEGER, INDEXED = TRUE'
///
///           CNAMES(2) =  'CUSTOMER_ID'
///           CDECLS(2) =  'DATATYPE = INTEGER, INDEXED = TRUE'
///
///           CNAMES(3) =  'LAST_NAME'
///           CDECLS(3) =  'DATATYPE = CHARACTER*(*), ' //
///          .             'INDEXED  = TRUE'
///
///           CNAMES(4) =  'FIRST_NAME'
///           CDECLS(4) =  'DATATYPE = CHARACTER*(*), ' //
///          .             'INDEXED  = TRUE'
///
///           CNAMES(5) =  'ORDER_DATE'
///           CDECLS(5) =  'DATATYPE = TIME, INDEXED  = TRUE'
///
///           CNAMES(6) =  'COST'
///           CDECLS(6) =  'DATATYPE = DOUBLE PRECISION,' //
///          .             'INDEXED  = TRUE,'             //
///          .             'NULLS_OK = TRUE'
///
///
///     C
///     C     Start the segment.  We presume the number of  rows
///     C     of data is known in advance.
///     C
///           CALL EKIFLD ( HANDLE,  TABLE,  NCOLS, NROWS,
///          .              CNAMES,  CDECLS, SEGNO, RCPTRS )
///
///
///     C
///     C     At this point, arrays containing data for the
///     C     segment's columns may be filled in.  The names
///     C     of the data arrays are shown below.
///     C
///     C        Column           Data array
///     C
///     C        'ORDER_ID'       ORDIDS
///     C        'CUSTOMER_ID'    CSTIDS
///     C        'LAST_NAME'      LNAMES
///     C        'FIRST_NAME'     FNAMES
///     C        'ORDER_DATE'     ETS
///     C        'COST'           COSTS
///     C
///           DO I = 1, NROWS
///
///              ORDIDS(I) = I
///              CSTIDS(I) = I * 100
///              COSTS(I)  = I * 100.D0
///
///              CALL REPMI ( 'Order # Customer first name',
///          .                '#', I, FNAMES(I)             )
///              CALL REPMI ( 'Order # Customer last name',
///          .                '#', I, LNAMES(I)             )
///              CALL REPMI ( '1998 Mar #', '#', I, ODATE   )
///
///              CALL UTC2ET ( ODATE,  ETS(I) )
///
///              NLFLGS(I) = .FALSE.
///
///           END DO
///
///           NLFLGS(2) = .TRUE.
///
///     C
///     C     The SIZES array shown below is ignored for scalar
///     C     and fixed-size array columns, so we need not
///     C     initialize it.  For variable-size arrays, the
///     C     Ith element of the SIZES array must contain the size
///     C     of the Ith column entry in the column being written.
///     C     Normally, the SIZES array would be reset for each
///     C     variable-size column.
///     C
///     C     The NLFLGS array indicates which entries are null.
///     C     It is ignored for columns that don't allow null
///     C     values.  In this case, only the COST column allows
///     C     nulls.
///     C
///     C     Add the columns of data to the segment.  All of the
///     C     data for each column is written in one shot.
///     C
///           CALL EKACLI ( HANDLE, SEGNO,  'ORDER_ID',
///          .              ORDIDS, SIZES,  NLFLGS,  RCPTRS, WKINDX )
///
///           CALL EKACLI ( HANDLE, SEGNO,  'CUSTOMER_ID',
///          .              CSTIDS, SIZES,  NLFLGS,  RCPTRS, WKINDX )
///
///           CALL EKACLC ( HANDLE, SEGNO,  'LAST_NAME',
///          .              LNAMES, SIZES,  NLFLGS,  RCPTRS, WKINDX )
///
///           CALL EKACLC ( HANDLE, SEGNO,  'FIRST_NAME',
///          .              FNAMES, SIZES,  NLFLGS,  RCPTRS, WKINDX )
///
///           CALL EKACLD ( HANDLE, SEGNO,  'ORDER_DATE',
///          .              ETS,    SIZES,  NLFLGS,  RCPTRS, WKINDX )
///
///           CALL EKACLD ( HANDLE, SEGNO,  'COST',
///          .              COSTS,  SIZES,  NLFLGS,  RCPTRS, WKINDX )
///
///     C
///     C     Complete the segment.  The RCPTRS array is that
///     C     returned by EKIFLD.
///     C
///           CALL EKFFLD ( HANDLE, SEGNO, RCPTRS )
///
///     C
///     C     At this point, the second segment could be
///     C     created by an analogous process.  In fact, the
///     C     second segment could be created at any time; it is
///     C     not necessary to populate the first segment with
///     C     data before starting the second segment.
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
///     C     Start the new segment. Since we have no data for this
///     C     segment, start the segment by just defining the new
///     C     segment's schema.
///     C
///           CALL EKBSEG ( HANDLE, 'DATAITEMS', 5,
///          .              CNAMES, CDECLS,      SEGNO )
///
///     C
///     C     Close the file by a call to EKCLS.
///     C
///           CALL EKCLS ( HANDLE )
///
///     C
///     C     Now, we want to insert a new record into the DATAITEMS
///     C     table.
///     C
///     C     Open the database for write access.  This call is
///     C     made when the file already exists.
///     C
///           CALL EKOPW ( EKNAME, HANDLE )
///
///     C
///     C     Append a new, empty record to the DATAITEMS
///     C     table. Recall that the DATAITEMS table
///     C     is in segment number 2.  The call will return
///     C     the number of the new, empty record.
///     C
///           SEGNO = 2
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
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard and
///         created complete code example from existing fragment.
///
/// -    SPICELIB Version 1.1.0, 05-FEB-2015 (NJB)
///
///         Updated to use ERRHAN.
///
/// -    Beta Version 1.0.0, 26-SEP-1995 (NJB)
/// ```
pub fn ekacec(
    ctx: &mut SpiceContext,
    handle: i32,
    segno: i32,
    recno: i32,
    column: &str,
    nvals: i32,
    cvals: CharArray,
    isnull: bool,
) -> crate::Result<()> {
    EKACEC(
        handle,
        segno,
        recno,
        column.as_bytes(),
        nvals,
        cvals,
        isnull,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKACEC ( EK, add character data to column )
pub fn EKACEC(
    HANDLE: i32,
    SEGNO: i32,
    RECNO: i32,
    COLUMN: &[u8],
    NVALS: i32,
    CVALS: CharArray,
    ISNULL: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CVALS = DummyCharArray::new(CVALS, None, 1..);
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
    ZZEKSDSC(HANDLE, SEGNO, SEGDSC.as_slice_mut(), ctx)?;
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
    // This column had better be of character type.
    //
    DTYPE = COLDSC[TYPIDX];

    if (DTYPE != CHR) {
        CHKIN(b"EKACEC", ctx)?;
        SETMSG(b"Column # is of type #; EKACEC only works with character columns.  RECNO = #; SEGNO = #; EK = #.", ctx);
        ERRCH(b"#", COLUMN, ctx);
        ERRINT(b"#", DTYPE, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", SEGNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(WRONGDATATYPE)", ctx)?;
        CHKOUT(b"EKACEC", ctx)?;
        return Ok(());
    }

    //
    // Look up the record pointer for the target record.
    //
    ZZEKTRDP(HANDLE, SEGDSC[RTIDX], RECNO, &mut RECPTR, ctx)?;

    //
    // Now it's time to add data to the file.
    //
    CLASS = COLDSC[CLSIDX];

    if (CLASS == 3) {
        //
        // Class 3 columns contain scalar character data.
        //
        ZZEKAD03(
            HANDLE,
            SEGDSC.as_slice_mut(),
            COLDSC.as_slice(),
            RECPTR,
            CVALS.first(),
            ISNULL,
            ctx,
        )?;
    } else if (CLASS == 6) {
        //
        // Class 6 columns contain array-valued character data.
        //
        ZZEKAD06(
            HANDLE,
            SEGDSC.as_slice_mut(),
            COLDSC.as_slice(),
            RECPTR,
            NVALS,
            CVALS.as_arg(),
            ISNULL,
            ctx,
        )?;
    } else {
        //
        // This is an unsupported character column class.
        //
        CHKIN(b"EKACEC", ctx)?;
        SETMSG(b"Class # from input column descriptor is not a supported character class.  COLUMN = #; RECNO = #; SEGNO = #; EK = #.", ctx);
        ERRINT(b"#", CLASS, ctx);
        ERRCH(b"#", COLUMN, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", SEGNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(NOCLASS)", ctx)?;
        CHKOUT(b"EKACEC", ctx)?;
        return Ok(());
    }

    Ok(())
}
