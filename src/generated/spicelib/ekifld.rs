//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

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

/// EK, initialize segment for fast write
///
/// Initialize a new E-kernel segment to allow fast writing.
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
///  HANDLE     I   File handle.
///  TABNAM     I   Table name.
///  NCOLS      I   Number of columns in the segment.
///  NROWS      I   Number of rows in the segment.
///  CNAMES     I   Names of columns.
///  DECLS      I   Declarations of columns.
///  SEGNO      O   Segment number.
///  RCPTRS     O   Array of record pointers.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of an EK file open for write access. A new
///           segment is to be created in this file.
///
///  TABNAM   is the name of the EK table to which the current segment
///           belongs. All segments in the EK file designated by HANDLE
///           must have identical column attributes. TABNAM must not
///           exceed TNAMSZ (64) characters in length. Case is not
///           significant. Table names must start with a letter and
///           contain only characters from the set {A-Z,a-z,0-9,$,_}.
///
///  NCOLS    is the number of columns in a new segment.
///
///  NROWS    is the number of rows in a new segment. Each column to be
///           added to the segment must contain the number of entries
///           indicated by NROWS.
///
///  CNAMES,
///  DECLS    are, respectively, an array of column names and their
///           corresponding declarations: the Ith element of CNAMES and
///           the Ith element of DECLS apply to the Ith column in the
///           segment.
///
///           Column names must not exceed CNAMSZ (32) characters in
///           length. Case is not significant. Column names must start
///           with a letter and contain only characters from the set
///           {A-Z,a-z,0-9,$,_}.
///
///           The declarations are strings that contain "keyword=value"
///           assignments that define the attributes of the columns to
///           which they apply. The column attributes that are defined
///           by a column declaration are:
///
///              DATATYPE
///              SIZE
///              <is the column indexed?>
///              <does the column allow null values?>
///
///           The form of a declaration is
///
///              'DATATYPE  = <type>,
///               SIZE      = <size>,
///               INDEXED   = <boolean>,
///               NULLS_OK  = <boolean>'
///
///           For example, an indexed, scalar, integer column that
///           allows null values would have the declaration
///
///              'DATATYPE  = INTEGER,
///               SIZE      = 1,
///               INDEXED   = TRUE,
///               NULLS_OK  = TRUE'
///
///           Commas are required to separate the assignments within
///           declarations; white space is optional; case is not
///           significant.
///
///           The order in which the attribute keywords are listed in
///           declaration is not significant.
///
///           Every column in a segment must be declared.
///
///           Each column entry is effectively an array, each element
///           of which has the declared data type. The SIZE keyword
///           indicates how many elements are in each entry of the
///           column in whose declaration the keyword appears. Note
///           that only scalar-valued columns (those for which SIZE =
///           1) may be referenced in query constraints. A size
///           assignment has the syntax
///
///              'SIZE = <integer>'
///
///           or
///              'SIZE = VARIABLE'
///
///           The size value defaults to 1 if omitted.
///
///           The DATATYPE keyword defines the data type of column
///           entries. The DATATYPE assignment syntax has any of the
///           forms
///
///              'DATATYPE = CHARACTER*(<length>)'
///              'DATATYPE = CHARACTER*(*)'
///              'DATATYPE = DOUBLE PRECISION'
///              'DATATYPE = INTEGER'
///              'DATATYPE = TIME'
///
///           As the datatype declaration syntax suggests, character
///           strings may have fixed or variable length.
///           Variable-length strings are allowed only in columns of
///           size 1.
///
///           Optionally, scalar-valued columns may be indexed. To
///           create an index for a column, use the assignment
///
///              'INDEXED = TRUE'
///
///           By default, columns are not indexed.
///
///           Optionally, any column can allow null values. To indicate
///           that a column may allow null values, use the assignment
///
///              'NULLS_OK = TRUE'
///
///           in the column declaration. By default, null values are
///           not allowed in column entries.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SEGNO    is the number of the segment created by this routine.
///           Segment numbers are used as unique identifiers by other
///           EK access routines.
///
///  RCPTRS   is an array of record pointers for the input segment.
///           This array must not be modified by the caller.
///
///           The array RCPTRS must be passed as an input to each
///           column addition routine called while writing the
///           specified segment.
///
///           RCPTRS must be declared with dimension NROWS.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If HANDLE is invalid, an error is signaled by a routine in the
///      call tree of this routine.
///
///  2)  If TABNAM is more than TNAMSZ characters long, an error
///      is signaled by a routine in the call tree of this routine.
///
///  3)  If TABNAM contains any nonprintable characters, an error
///      is signaled by a routine in the call tree of this routine.
///
///  4)  If NCOLS is non-positive or greater than the maximum allowed
///      number MXCLSG, an error is signaled by a routine in the call
///      tree of this routine.
///
///  5)  If NROWS is non-positive, the error SPICE(INVALIDCOUNT)
///      is signaled.
///
///  6)  If any column name exceeds CNAMSZ characters in length, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  7)  If any column name contains non-printable characters, an error
///      is signaled by a routine in the call tree of this routine.
///
///  8)  If a declaration cannot be understood by this routine, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  9)  If an non-positive string length or element size is specified,
///      an error is signaled by a routine in the call tree of this
///      routine.
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
///  This routine prepares an EK for the creation of a new segment via
///  the fast column writer routines. After this routine is called,
///  the columns of the segment are filled in by calls to the fast
///  column writer routines of the appropriate data types. The fast
///  column writer routines are:
///
///     EKACLC {EK, add column, character}
///     EKACLD {EK, add column, double precision}
///     EKACLI {EK, add column, integer}
///
///  When all of the columns have been added, the write operation is
///  completed by a call to EKFFLD {EK, finish fast write}.
///
///  The segment is not valid until EKFFLD has been called.
///
///  The EK system supports only one fast write at a time. It is
///  not possible use the fast write routines to simultaneously write
///  multiple segments, either in the same EK file or in different
///  files.
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
///  1) Suppose we want to create an Sequence Component E-kernel
///     named 'ekifld_ex1.bes' which contains records of orders for
///     data products. The E-kernel has a table called DATAORDERS that
///     consists of the set of columns listed below:
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
///     The file "ekifld_ex1.bdb" will contain two segments, the first
///     containing the DATAORDERS table and the second containing the
///     DATAITEMS table.
///
///     This example demonstrates how to open a new EK file and create
///     the first of the segments described above.
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
///           PROGRAM EKIFLD_EX1
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
///           CHARACTER*(*)         LSK
///           PARAMETER           ( LSK    = 'naif0012.tls' )
///
///           CHARACTER*(*)         TABLE
///           PARAMETER           ( TABLE  = 'DATAORDERS'   )
///
///           INTEGER               DECLEN
///           PARAMETER           ( DECLEN = 200 )
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
///           CHARACTER*(32)        CNAMES ( NCOLS )
///           CHARACTER*(FNMLEN)    FNAMES ( NROWS )
///           CHARACTER*(LNMLEN)    LNAMES ( NROWS )
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(UTCLEN)    ODATE
///
///           DOUBLE PRECISION      COSTS  ( NROWS )
///           DOUBLE PRECISION      ETS    ( NROWS )
///
///           INTEGER               CSTIDS ( NROWS )
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               NRESVC
///           INTEGER               ORDIDS ( NROWS )
///           INTEGER               RCPTRS ( NROWS )
///           INTEGER               SEGNO
///           INTEGER               SIZES  ( NROWS )
///           INTEGER               WKINDX ( NROWS )
///
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
///           IFNAME  =  'Test EK/Created 20-SEP-1995'
///
///           CALL EKOPN ( 'ekifld_ex1.bes', IFNAME, NRESVC, HANDLE )
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
///           CDECLS(3) =  'DATATYPE = CHARACTER*(*),' //
///          .             'INDEXED  = TRUE'
///
///           CNAMES(4) =  'FIRST_NAME'
///           CDECLS(4) =  'DATATYPE = CHARACTER*(*),' //
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
///     C     The file must be closed by a call to EKCLS.
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
/// # Restrictions
///
/// ```text
///  1)  Only one segment can be created at a time using the fast
///      write routines.
///
///  2)  No other EK operation may interrupt a fast write. For
///      example, it is not valid to issue a query while a fast write
///      is in progress.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 17-JUN-2021 (JDR) (BVS)
///
///         Added missing IMPLICIT NONE.
///
///         Edited the header to comply with NAIF standard. and
///         created complete code example from existing fragment.
///
///         Corrected entry #4 in $Exceptions: the same error is signaled
///         in the requested number of columns exceeds the maximum allowed
///         per segment.
///
/// -    SPICELIB Version 1.1.1, 10-JAN-2002 (NJB)
///
///         Documentation change: instances of the phrase "fast load"
///         were replaced with "fast write." Corrected value of table
///         name size in header comment.
///
/// -    SPICELIB Version 1.1.0, 18-JUN-1999 (WLT)
///
///         Balanced CHKIN/CHKOUT calls.
///
/// -    SPICELIB Version 1.0.0, 25-OCT-1995 (NJB)
/// ```
pub fn ekifld(
    ctx: &mut SpiceContext,
    handle: i32,
    tabnam: &str,
    ncols: i32,
    nrows: i32,
    cnames: CharArray,
    decls: CharArray,
    segno: &mut i32,
    rcptrs: &mut [i32],
) -> crate::Result<()> {
    EKIFLD(
        handle,
        tabnam.as_bytes(),
        ncols,
        nrows,
        cnames,
        decls,
        segno,
        rcptrs,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKIFLD ( EK, initialize segment for fast write )
pub fn EKIFLD(
    HANDLE: i32,
    TABNAM: &[u8],
    NCOLS: i32,
    NROWS: i32,
    CNAMES: CharArray,
    DECLS: CharArray,
    SEGNO: &mut i32,
    RCPTRS: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CNAMES = DummyCharArray::new(CNAMES, None, 1..);
    let DECLS = DummyCharArray::new(DECLS, None, 1..);
    let mut RCPTRS = DummyArrayMut::new(RCPTRS, 1..);
    let mut MBASE: i32 = 0;
    let mut P: i32 = 0;
    let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);
    let mut STYPE: i32 = 0;

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
        CHKIN(b"EKIFLD", ctx)?;
    }

    //
    // Check out NROWS.
    //
    if (NROWS < 1) {
        SETMSG(b"Number of rows must be > 0, was #. ", ctx);
        ERRINT(b"#", NROWS, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"EKIFLD", ctx)?;
        return Ok(());
    }

    //
    // Create the segment's metadata.
    //
    EKBSEG(
        HANDLE,
        TABNAM,
        NCOLS,
        CNAMES.as_arg(),
        DECLS.as_arg(),
        SEGNO,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"EKIFLD", ctx)?;
        return Ok(());
    }

    //
    // Fill the number of rows into the (file's) segment descriptor.
    //
    ZZEKMLOC(HANDLE, *SEGNO, &mut P, &mut MBASE, ctx)?;
    DASUDI(HANDLE, (MBASE + NRIDX), (MBASE + NRIDX), &[NROWS], ctx)?;

    //
    // Read in the segment descriptor, and get the segment's type.
    //
    ZZEKSDSC(HANDLE, *SEGNO, SEGDSC.as_slice_mut(), ctx)?;

    STYPE = SEGDSC[EKTIDX];

    //
    // Complete the fast write preparations appropriate to the segment's
    // type.
    //
    if (STYPE == 1) {
        ZZEKIF01(HANDLE, *SEGNO, RCPTRS.as_slice_mut(), ctx)?;
    } else if (STYPE == 2) {
        ZZEKIF02(HANDLE, *SEGNO, ctx)?;
    } else {
        SETMSG(b"Segment type # is not currently supported.", ctx);
        ERRINT(b"#", STYPE, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"EKIFLD", ctx)?;
        return Ok(());
    }

    CHKOUT(b"EKIFLD", ctx)?;
    Ok(())
}
