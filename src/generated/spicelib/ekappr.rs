//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

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
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;

/// EK, append record onto segment
///
/// Append a new, empty record at the end of a specified E-kernel
/// segment.
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
///  SEGNO      I   Segment number.
///  RECNO      O   Number of appended record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is a file handle of an EK open for write access.
///
///  SEGNO    is the number of the segment to which the record
///           is to be added.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECNO    is the number of the record appended by this
///           routine. RECNO is used to identify the record
///           when writing column entries to it.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If HANDLE is invalid, an error is signaled by a routine in the
///      call tree of this routine. The file will not be modified.
///
///  2)  If SEGNO is out of range, the error SPICE(INVALIDINDEX)
///      is signaled. The file will not be modified.
///
///  3)  If an I/O error occurs while reading or writing the indicated
///      file, the error is signaled by a routine in the call tree of
///      this routine. The file may be corrupted.
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
///  This routine operates by side effects: It appends a new, empty
///  record structure to an EK segment. The ordinal position of the
///  new record is one greater than the previous number of records in
///  in the segment.
///
///  After a new record has been appended to a segment by this routine,
///  the record must be populated with data using the EKACEx
///  routines.  EKs are valid only when all of their column entries
///  are initialized.
///
///  To insert a record into a segment at a specified ordinal position,
///  use the routine EKAPPR.
///
///  This routine cannot be used with the "fast write" suite of
///  routines. See the EK Required Reading ek.req for a discussion of
///  the fast writers.
///
///  When a record is inserted into an EK file that is not shadowed,
///  the status of the record starts out set to OLD. The status
///  does not change when data is added to the record.
///
///  If the target EK is shadowed, the new record will be given the
///  status NEW. Updating column values in the record does not change
///  its status. When changes are committed, the status is set to OLD.
///  If a rollback is performed before changes are committed, the
///  record is deleted. Closing the target file without committing
///  changes implies a rollback.
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
///  1) Append a record to a specified segment.
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
///     This examples creates such EK, with no records in either
///     table, and after re-opening the file, inserts a new record
///     into the DATAITEMS table.
///
///
///     Example code begins here.
///
///
///           PROGRAM EKAPPR_EX1
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
///           PARAMETER           ( EKNAME  = 'ekappr_ex1.bdb' )
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
///           PARAMETER           ( NCOLS  = 6   )
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
///     C
///     C     Start the first segment. Since we have no data for this
///     C     segment, start the segment by just defining the new
///     C     segment's schema.
///     C
///           CALL EKBSEG ( HANDLE, 'DATAORDERS', NCOLS,
///          .              CNAMES, CDECLS,       SEGNO )
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
///     C     End the file by a call to EKCLS.
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
///           ISNULL =  .FALSE.
///           ESIZE  =  1
///
///     C
///     C     The following variables will contain the data for
///     C     the new record.
///     C
///           ORDID  =   10011
///           ITEMID =   531
///           ITEMNM =  'Sample item'
///           DESCRP =  'This sample item is used only in tests.'
///           PRICE  =   1345.678D0
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
/// -    SPICELIB Version 1.1.0, 02-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
/// -    SPICELIB Version 1.0.1, 09-JAN-2002 (NJB)
///
///         Documentation change: instances of the phrase "fast load"
///         were replaced with "fast write."
///
/// -    SPICELIB Version 1.0.0, 19-DEC-1995 (NJB)
/// ```
pub fn ekappr(
    ctx: &mut SpiceContext,
    handle: i32,
    segno: i32,
    recno: &mut i32,
) -> crate::Result<()> {
    EKAPPR(handle, segno, recno, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKAPPR ( EK, append record onto segment )
pub fn EKAPPR(
    HANDLE: i32,
    SEGNO: i32,
    RECNO: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut MBASE: i32 = 0;
    let mut MP: i32 = 0;
    let mut NREC: i32 = 0;
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
        CHKIN(b"EKAPPR", ctx)?;
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
        CHKOUT(b"EKAPPR", ctx)?;
        return Ok(());
    }

    //
    // Look up the integer metadata page and page base for the segment.
    // Given the base address, we can read the pertinent metadata in
    // one shot.
    //
    ZZEKMLOC(HANDLE, SEGNO, &mut MP, &mut MBASE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"EKAPPR", ctx)?;
        return Ok(());
    }

    DASRDI(
        HANDLE,
        (MBASE + 1),
        (MBASE + SDSCSZ),
        SEGDSC.as_slice_mut(),
        ctx,
    )?;

    //
    // Obtain the number of records already present.
    //
    NREC = SEGDSC[NRIDX];

    //
    // Insert the new record at the end of the segment.
    //
    *RECNO = (NREC + 1);

    EKINSR(HANDLE, SEGNO, *RECNO, ctx)?;

    CHKOUT(b"EKAPPR", ctx)?;
    Ok(())
}
