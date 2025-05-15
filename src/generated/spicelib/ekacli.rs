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

/// EK, add integer column to segment
///
/// Add an entire integer column to an EK segment.
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
///  SEGNO      I   Number of segment to add column to.
///  COLUMN     I   Column name.
///  IVALS      I   Integer values to add to column.
///  ENTSZS     I   Array of sizes of column entries.
///  NLFLGS     I   Array of null flags for column entries.
///  RCPTRS     I   Record pointers for segment.
///  WKINDX    I-O  Work space for column index.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of an EK file that is open for writing.
///           A "begin segment for fast write" operation must
///           have already been performed for the designated
///           segment.
///
///  SEGNO    is the number of the segment to which
///           data is to be added.
///
///  COLUMN   is the name of the column to be added. All of
///           the data for the named column will be added in
///           one shot.
///
///  IVALS    is an array containing the entire set of column
///           entries for the specified column. The entries
///           are listed in row-order:  the column entry for the
///           first row of the segment is first, followed by the
///           column entry for the second row, and so on. The
///           number of column entries must match the declared
///           number of rows in the segment. For columns having
///           fixed-size entries, a null entry must be allocated
///           the same amount of space occupied by a non-null
///           entry in the array IVALS. For columns having
///           variable-size entries, null entries do not require
///           any space in the IVALS array, but in any case must
///           have their allocated space described correctly by
///           the corresponding element of the ENTSZS array
///           (described below).
///
///  ENTSZS   is an array containing sizes of column entries.
///           The Ith element of ENTSZS gives the size of the
///           Ith column entry. ENTSZS is used only for columns
///           having variable-size entries. For such columns,
///           the dimension of ENTSZS must be at least NROWS.
///           The size of null entries should be set to zero.
///
///           For columns having fixed-size entries, the
///           dimension of this array may be any positive value.
///
///  NLFLGS   is an array of logical flags indicating whether
///           the corresponding entries are null. If the Ith
///           element of NLFLGS is .FALSE., the Ith column entry
///           defined by IVALS and ENTSZS is added to the
///           current segment in the specified kernel file.
///
///           If the Ith element of NLFGLS is .TRUE., the
///           contents of the Ith column entry are undefined.
///
///           NLFLGS is used only for columns that allow null
///           values; it's ignored for other columns.
///
///  RCPTRS   is an array of record pointers for the input
///           segment. This array is obtained as an output
///           from EKIFLD, the routine called to initiate a
///           fast write.
///
///  WKINDX   is a work space array used for building a column
///           index. If the column is indexed, the dimension of
///           WKINDX must be at NROWS, where NROWS is the number
///           of rows in the column. If the column is not
///           indexed, this work space is not used, so the
///           dimension may be any positive value.
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
///  2)  If COLUMN is not the name of a declared column, an error is
///      signaled by a routine in the call tree of this routine.
///
///  3)  If COLUMN specifies a column of whose data type is not
///      integer, the error SPICE(WRONGDATATYPE) is signaled.
///
///  4)  If the specified column already contains ANY entries, an error
///      is signaled by a routine in the call tree of this routine.
///
///  5)  If an I/O error occurs while reading or writing the indicated
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
///  EK file by adding data to the specified column. This routine
///  writes the entire contents of the specified column in one shot.
///  This routine creates columns much more efficiently than can be
///  done by sequential calls to EKACEI, but has the drawback that
///  the caller must use more memory for the routine's inputs. This
///  routine cannot be used to add data to a partially completed
///  column.
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
///     named 'ekacli_ex1.bes' which contains records of orders for
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
///     The file "ekacli_ex1.bdb" will contain two segments, the first
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
///           PROGRAM EKACLI_EX1
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
///           CHARACTER*(CNAMSZ)    CNAMES ( NCOLS )
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
///           CALL EKOPN ( 'ekacli_ex1.bes', IFNAME, NRESVC, HANDLE )
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
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. and
///         created complete code example from existing fragment.
///
/// -    SPICELIB Version 1.0.1, 09-JAN-2002 (NJB)
///
///         Documentation change: instances of the phrase "fast load"
///         were replaced with "fast write."
///
///      Beta Version 1.0.0, 08-NOV-1995 (NJB)
/// ```
pub fn ekacli(
    ctx: &mut SpiceContext,
    handle: i32,
    segno: i32,
    column: &str,
    ivals: &[i32],
    entszs: &[i32],
    nlflgs: &[bool],
    rcptrs: &[i32],
    wkindx: &mut [i32],
) -> crate::Result<()> {
    EKACLI(
        handle,
        segno,
        column.as_bytes(),
        ivals,
        entszs,
        nlflgs,
        rcptrs,
        wkindx,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKACLI ( EK, add integer column to segment )
pub fn EKACLI(
    HANDLE: i32,
    SEGNO: i32,
    COLUMN: &[u8],
    IVALS: &[i32],
    ENTSZS: &[i32],
    NLFLGS: &[bool],
    RCPTRS: &[i32],
    WKINDX: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let IVALS = DummyArray::new(IVALS, 1..);
    let ENTSZS = DummyArray::new(ENTSZS, 1..);
    let NLFLGS = DummyArray::new(NLFLGS, 1..);
    let RCPTRS = DummyArray::new(RCPTRS, 1..);
    let mut WKINDX = DummyArrayMut::new(WKINDX, 1..);
    let mut CLASS: i32 = 0;
    let mut COLDSC = StackArray::<i32, 11>::new(1..=CDSCSZ);
    let mut DTYPE: i32 = 0;
    let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);

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
        CHKIN(b"EKACLI", ctx)?;
    }

    //
    // Find the descriptors for the specified segment and column.
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
        CHKOUT(b"EKACLI", ctx)?;
        return Ok(());
    }

    //
    // This column had better be of integer type.
    //
    CLASS = COLDSC[CLSIDX];
    DTYPE = COLDSC[TYPIDX];

    if (DTYPE != INT) {
        SETMSG(
            b"Column # is of type #; EKACLI only works with integer columns.",
            ctx,
        );
        ERRCH(b"#", COLUMN, ctx);
        ERRINT(b"#", DTYPE, ctx);
        SIGERR(b"SPICE(WRONGDATATYPE)", ctx)?;
        CHKOUT(b"EKACLI", ctx)?;
        return Ok(());
    }

    //
    // Hand off the task to the routine of the appropriate class.
    //
    if (CLASS == 1) {
        //
        // Class 1 columns contain integer scalars.
        //
        ZZEKAC01(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            IVALS.as_slice(),
            NLFLGS.as_slice(),
            RCPTRS.as_slice(),
            WKINDX.as_slice_mut(),
            ctx,
        )?;
    } else if (CLASS == 4) {
        //
        // Class 4 columns contain integer arrays.
        //
        ZZEKAC04(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            IVALS.as_slice(),
            ENTSZS.as_slice(),
            NLFLGS.as_slice(),
            ctx,
        )?;
    } else if (CLASS == 7) {
        //
        // Class 7 columns contain fixed-count integer scalars.
        //
        ZZEKAC07(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            IVALS.as_slice(),
            NLFLGS.as_slice(),
            WKINDX.as_slice_mut(),
            ctx,
        )?;
    } else {
        //
        // This is an unsupported column class.
        //
        SETMSG(
            b"Unsupported column class code # found in descriptor for column #.",
            ctx,
        );
        ERRINT(b"#", CLASS, ctx);
        ERRCH(b"#", COLUMN, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"EKACLI", ctx)?;
        return Ok(());
    }

    CHKOUT(b"EKACLI", ctx)?;
    Ok(())
}
