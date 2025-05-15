//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
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
const CNAMSZ: i32 = 32;
const ENCSIZ: i32 = 5;
const CPSIZE: i32 = 1014;
const CFPIDX: i32 = (CPSIZE + 1);
const CLCIDX: i32 = (CFPIDX + ENCSIZ);
const DPSIZE: i32 = 126;
const DFPIDX: i32 = (DPSIZE + 1);
const DLCIDX: i32 = (DFPIDX + 1);
const IPSIZE: i32 = 254;
const IFPIDX: i32 = (IPSIZE + 1);
const ILCIDX: i32 = (IFPIDX + 1);
const FPARSZ: i32 = 1;
const SGTIDX: i32 = 1;
const MXCLSG: i32 = 100;
const EPARCH: i32 = 1;
const EPNIPT: i32 = 5;
const EPPSZC: i32 = (EPARCH + 1);
const EPBASC: i32 = (EPPSZC + 1);
const EPNPC: i32 = (EPBASC + 1);
const EPNFPC: i32 = (EPNPC + 1);
const EPFPC: i32 = (EPNFPC + 1);
const EPPSZD: i32 = (EPPSZC + EPNIPT);
const EPBASD: i32 = (EPPSZD + 1);
const EPNPD: i32 = (EPBASD + 1);
const EPNFPD: i32 = (EPNPD + 1);
const EPFPD: i32 = (EPNFPD + 1);
const EPPSZI: i32 = (EPPSZD + EPNIPT);
const EPBASI: i32 = (EPPSZI + 1);
const EPNPI: i32 = (EPBASI + 1);
const EPNFPI: i32 = (EPNPI + 1);
const EPFPI: i32 = (EPNFPI + 1);
const EPMDSZ: i32 = (1 + (3 * EPNIPT));
const PGSIZC: i32 = 1024;
const PGSIZD: i32 = 128;
const PGSIZI: i32 = 256;
const PGBASC: i32 = 0;
const PGBASD: i32 = 0;
const PGBASI: i32 = 256;
const OLD: i32 = 1;
const UPDATE: i32 = (OLD + 1);
const NEW: i32 = (UPDATE + 1);
const DELOLD: i32 = (NEW + 1);
const DELNEW: i32 = (DELOLD + 1);
const DELUPD: i32 = (DELNEW + 1);
const STAIDX: i32 = 1;
const RCPIDX: i32 = (STAIDX + 1);
const DPTBAS: i32 = 2;
const MXRPSZ: i32 = 254;
const UNINIT: i32 = -1;
const NULL: i32 = (UNINIT - 1);
const NOBACK: i32 = (NULL - 1);
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
const TNAMSZ: i32 = 64;
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const LBCELL: i32 = -5;
const MXSPEC: i32 = 512;
const NAMLIM: i32 = 32;

struct SaveVars {
    IDSPEC: ActualArray<i32>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut IDSPEC = ActualArray::<i32>::new(LBCELL..=MXSPEC);
        let mut FIRST: bool = false;

        FIRST = true;

        Self { IDSPEC, FIRST }
    }
}

/// EK, start new segment
///
/// Start a new segment in an E-kernel.
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
///  CNAMES     I   Names of columns.
///  DECLS      I   Declarations of columns.
///  SEGNO      O   Segment number.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of an EK file that is open for writing.
///
///  TABNAM   is the name of the EK table to which the current
///           segment belongs. All segments in the EK file
///           designated by HANDLE must have identical column
///           attributes. TABNAM must not exceed 32 characters
///           in length. Case is not significant. Table names
///           must start with a letter and contain only
///           characters from the set {A-Z,a-z,0-9,$,_}.
///
///  NCOLS    is the number of columns in a new segment.
///
///  CNAMES,
///  DECLS    are, respectively, and array of column names and
///           their corresponding declarations: the Ith element
///           of CNAMES and the Ith element of DECLS apply to
///           the Ith column in the segment.
///
///           Column names must not exceed CNAMSZ (32) characters
///           in length. Case is not significant. Column names
///           must start with a letter and contain only
///           characters from the set {A-Z,a-z,0-9,$,_}.
///
///           The declarations are strings that contain
///           `keyword=value' assignments that define the
///           attributes of the columns to which they apply. The
///           column attributes that are defined by a column
///           declaration are:
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
///           For example, an indexed, scalar, integer column
///           that allows null values would have the declaration
///
///              'DATATYPE  = INTEGER,
///               SIZE      = 1,
///               INDEXED   = TRUE,
///               NULLS_OK  = TRUE'
///
///           Commas are required to separate the assignments
///           within declarations; white space is optional;
///           case is not significant.
///
///           The order in which the attribute keywords are
///           listed in declaration is not significant.
///
///           Every column in a segment must be declared.
///
///           Each column entry is effectively an array, each
///           element of which has the declared data type. The
///           SIZE keyword indicates how many elements are in
///           each entry of the column in whose declaration the
///           keyword appears. Note that only scalar-valued
///           columns (those for which SIZE = 1) may be
///           referenced in query constraints. A size
///           assignment has the syntax
///
///              'SIZE = <integer>'
///
///           or
///
///              'SIZE = VARIABLE'
///
///           The size value defaults to 1 if omitted.
///
///           The DATATYPE keyword defines the data type of
///           column entries. The DATATYPE assignment syntax
///           has any of the forms
///
///              'DATATYPE = CHARACTER*(<length>)'
///              'DATATYPE = CHARACTER*(*)'
///              'DATATYPE = DOUBLE PRECISION'
///              'DATATYPE = INTEGER'
///              'DATATYPE = TIME'
///
///           As the datatype declaration syntax suggests,
///           character strings may have fixed or variable
///           length. Variable-length strings are allowed only
///           in columns of size 1.
///
///           Optionally, scalar-valued columns may be indexed.
///           To create an index for a column, use the assignment
///
///              'INDEXED = TRUE'
///
///           By default, columns are not indexed.
///
///           Optionally, any column can allow null values. To
///           indicate that a column may allow null values, use
///           the assignment
///
///              'NULLS_OK = TRUE'
///
///           in the column declaration. By default, null
///           values are not allowed in column entries.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SEGNO    is the number of the segment created by this
///           routine. Segment numbers are used as unique
///           identifiers by other EK access routines.
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
///      number MXCLSG, the error SPICE(INVALIDCOUNT) is signaled.
///
///  5)  If any column name exceeds CNAMSZ characters in length, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  6)  If any column name contains non-printable characters, an error
///      is signaled by a routine in the call tree of this routine.
///
///  7)  If a declaration cannot be understood by this routine, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  8)  If an non-positive string length or element size is specified,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  9)  If an I/O error occurs while reading or writing the indicated
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
///  This routine operates by side effects: it prepares an EK for
///  the addition of a new segment. It is not necessary to take
///  any special action to `complete' a segment; segments are readable
///  after the completion of any record insertion, deletion, write,
///  or update operation.
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
///           PROGRAM EKBSEG_EX1
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
///           PARAMETER           ( EKNAME  = 'ekbseg_ex1.bdb' )
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
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICI NONE statement.
///
///         Edited the header to comply with NAIF standard and
///         created complete code example from existing fragment.
///
/// -    SPICELIB Version 1.1.0, 07-JUL-1996 (NJB)
///
///         Erroneous error message for invalid column names was fixed.
///         Previous version line was changed from "Beta" to "SPICELIB."
///
/// -    SPICELIB Version 1.0.0, 06-NOV-1995 (NJB)
/// ```
pub fn ekbseg(
    ctx: &mut SpiceContext,
    handle: i32,
    tabnam: &str,
    ncols: i32,
    cnames: CharArray,
    decls: CharArray,
    segno: &mut i32,
) -> crate::Result<()> {
    EKBSEG(
        handle,
        tabnam.as_bytes(),
        ncols,
        cnames,
        decls,
        segno,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKBSEG ( EK, start new segment )
pub fn EKBSEG(
    HANDLE: i32,
    TABNAM: &[u8],
    NCOLS: i32,
    CNAMES: CharArray,
    DECLS: CharArray,
    SEGNO: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CNAMES = DummyCharArray::new(CNAMES, None, 1..);
    let DECLS = DummyCharArray::new(DECLS, None, 1..);
    let mut CDSCRS = ActualArray2D::<i32>::new(1..=CDSCSZ, 1..=MXCLSG);
    let mut IDEND: i32 = 0;
    let mut NCHARS: i32 = 0;
    let mut STYPE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Non-SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKBSEG", ctx)?;
    }

    //
    // Before trying to actually write anything, do every error
    // check we can.
    //
    // Is this file handle valid--is the file open for paged write
    // access?  Signal an error if not.
    //
    ZZEKPGCH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"EKBSEG", ctx)?;
        return Ok(());
    }

    //
    // Get the default identifier specification the first time through.
    //
    if save.FIRST {
        SSIZEI(MXSPEC, save.IDSPEC.as_slice_mut(), ctx)?;
        LXDFID(save.IDSPEC.as_slice_mut(), ctx)?;
        save.FIRST = false;
    }

    //
    // The table name must not be too long, and all of its characters
    // must be printable (it's ok for it to unprintable).
    //
    CHCKID(b"EK table name", NAMLIM, TABNAM, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"EKBSEG", ctx)?;
        return Ok(());
    }

    //
    // Make sure the table name satisfies all of our restrictions on
    // allowed characters.
    //
    LXIDNT(save.IDSPEC.as_slice(), TABNAM, 1, &mut IDEND, &mut NCHARS);

    if ((NCHARS == 0) || (NCHARS < LASTNB(TABNAM))) {
        SETMSG(b"Table name <#> violates syntax rules.", ctx);
        ERRCH(b"#", TABNAM, ctx);
        SIGERR(b"SPICE(INVALIDNAME)", ctx)?;
        CHKOUT(b"EKBSEG", ctx)?;
        return Ok(());
    }

    //
    // Check out NCOLS.
    //
    if ((NCOLS < 1) || (NCOLS > MXCLSG)) {
        SETMSG(b"Number of columns must be in range 1:#, was #.", ctx);
        ERRINT(b"#", MXCLSG, ctx);
        ERRINT(b"#", NCOLS, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"EKBSEG", ctx)?;
        return Ok(());
    }

    //
    // Check the column names for length and printability.
    //
    for I in 1..=NCOLS {
        CHCKID(b"EK column name", CNAMSZ, &CNAMES[I], ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"EKBSEG", ctx)?;
            return Ok(());
        }

        //
        // Make sure each column name satisfies all of our restrictions on
        // allowed characters.
        //
        LXIDNT(
            save.IDSPEC.as_slice(),
            &CNAMES[I],
            1,
            &mut IDEND,
            &mut NCHARS,
        );

        if ((NCHARS == 0) || (NCHARS < LASTNB(&CNAMES[I]))) {
            SETMSG(b"Column name <#> violates syntax rules.", ctx);
            ERRCH(b"#", &CNAMES[I], ctx);
            SIGERR(b"SPICE(INVALIDNAME)", ctx)?;
            CHKOUT(b"EKBSEG", ctx)?;
            return Ok(());
        }
    }

    //
    // Parse the column declarations before proceeding.
    //
    for I in 1..=NCOLS {
        //
        // Parse the declaration of the Ith column.  The descriptor is
        // returned with all elements other than pointers initialized.
        //
        ZZEKPDEC(&DECLS[I], CDSCRS.subarray_mut([1, I]), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"EKBSEG", ctx)?;
            return Ok(());
        }
    }

    //
    // Determine the segment type.
    //
    STYPE = ZZEKSTYP(NCOLS, CDSCRS.as_slice(), ctx)?;

    //
    // Create the segment metadata according to the segment's type.
    //
    if (STYPE == 1) {
        ZZEKBS01(
            HANDLE,
            TABNAM,
            NCOLS,
            CNAMES.as_arg(),
            CDSCRS.as_slice_mut(),
            SEGNO,
            ctx,
        )?;
    } else if (STYPE == 2) {
        ZZEKBS02(
            HANDLE,
            TABNAM,
            NCOLS,
            CNAMES.as_arg(),
            CDSCRS.as_slice_mut(),
            SEGNO,
            ctx,
        )?;
    } else {
        SETMSG(b"Segment type # is not currently supported.", ctx);
        ERRINT(b"#", STYPE, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"EKBSEG", ctx)?;
        return Ok(());
    }

    CHKOUT(b"EKBSEG", ctx)?;
    Ok(())
}
