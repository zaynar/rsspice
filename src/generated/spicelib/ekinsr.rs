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

/// EK, insert record into segment
///
/// Add a new, empty record to a specified E-kernel segment at
/// a specified index.
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
///  RECNO      I   Record number.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is a file handle of an EK open for write access.
///
///  SEGNO    is the number of the segment to which the record
///           is to be added.
///
///  RECNO    is the index of the new record. RECNO must be
///           in the range 1 : (NREC+1), where NREC is the
///           number of records in the segment prior to the
///           insertion. If RECNO is equal to NREC+1, the
///           new record is appended. Otherwise, the new
///           record has the ordinal position specified by
///           RECNO, and the records previously occupying
///           positions RECNO : NREC have their indexes
///           incremented by 1.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See the $Particulars section for a description of the
///  effect of this routine.
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
///  3)  If RECNO is out of range, the error SPICE(INVALIDINDEX)
///      is signaled. The file will not be modified.
///
///  4)  If an I/O error occurs while reading or writing the indicated
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
///  This routine operates by side effects: It adds a new, empty
///  record structure to an EK segment at a specified ordinal position.
///
///  After a record has been inserted into a segment by this routine,
///  the record must be populated with data using the EKACEx
///  routines. EKs are valid only when all of their column entries
///  are initialized.
///
///  To append a record to a segment, use the routine EKAPPR.
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
///  1)  Insert a record into a specified E-kernel segment at a
///      specified ordinal position.
///
///      Suppose we have an E-kernel named ORDER_DB.EK which contains
///      records of orders for data products. The E-kernel has a
///      table called DATAORDERS that consists of the set of columns
///      listed below:
///
///         DATAORDERS
///
///            Column Name     Data Type
///            -----------     ---------
///            ORDER_ID        INTEGER
///            CUSTOMER_ID     INTEGER
///            LAST_NAME       CHARACTER*(*)
///            FIRST_NAME      CHARACTER*(*)
///            ORDER_DATE      TIME
///            COST            DOUBLE PRECISION
///
///      The order database also has a table of items that have been
///      ordered. The columns of this table are shown below:
///
///         DATAITEMS
///
///            Column Name     Data Type
///            -----------     ---------
///            ITEM_ID         INTEGER
///            ORDER_ID        INTEGER
///            ITEM_NAME       CHARACTER*(*)
///            DESCRIPTION     CHARACTER*(*)
///            PRICE           DOUBLE PRECISION
///
///
///      We'll suppose that the file ORDER_DB.EK contains two segments,
///      the first containing the DATAORDERS table and the second
///      containing the DATAITEMS table.
///
///      If we wanted to insert a new record into the DATAORDERS
///      table in position 1, we'd make the following calls:
///
///         C
///         C     Open the database for write access.
///         C
///               CALL EKOPW ( 'ORDER_DB.EK', HANDLE )
///
///         C
///         C     Insert a new, empty record into the DATAORDERS
///         C     table at record number 1.  This moves the existing
///         C     records down, so the old record 1 becomes record 2,
///         C     and so on.  Recall that the DATAORDERS table
///         C     is in segment number 1.
///         C
///               RECNO = 1
///               SEGNO = 1
///
///               CALL EKINSR ( HANDLE, SEGNO, RECNO )
///
///         C
///         C     At this point, the new record is empty.  A valid EK
///         C     cannot contain empty records.  We fill in the data
///         C     here.  Data items are filled in one column at a time.
///         C     The order in which the columns are filled in is not
///         C     important.  We use the EKACEx (add column entry)
///         C     routines to fill in column entries.  We'll assume
///         C     that no entries are null.  All entries are scalar,
///         C     so the entry size is 1.
///         C
///               ISNULL   =  .FALSE.
///               ESIZE    =  1
///
///         C
///         C     The following variables will contain the data for
///         C     the new record.
///         C
///               ORDID    =   10011
///               CUSTID   =   531
///               LNAME    =   'Scientist'
///               FNAME    =   'Joe'
///               ODATE    =   '1995-SEP-20'
///               COST     =   0.D0
///
///         C
///         C     Note that the names of the routines called
///         C     correspond to the data types of the columns: the
///         C     last letter of the routine name is C, I, or D,
///         C     depending on the data type. Time values are
///         C     converted to ET for storage.
///         C
///               CALL EKACEI ( HANDLE, SEGNO,  RECNO, 'ORDER_ID',
///              .              SIZE,   ORDID,  ISNULL               )
///
///               CALL EKACEI ( HANDLE, SEGNO,  RECNO, 'CUSTOMER_ID',
///              .              SIZE,   CUSTID, ISNULL               )
///
///               CALL EKACEC ( HANDLE, SEGNO,  RECNO, 'LAST_NAME',
///              .              SIZE,   LNAME,  ISNULL               )
///
///               CALL EKACEC ( HANDLE, SEGNO,  RECNO, 'FIRST_NAME',
///              .              SIZE,   FNAME,  ISNULL               )
///
///
///               CALL UTC2ET ( ODATE,  ET )
///               CALL EKACED ( HANDLE, SEGNO,  RECNO, 'ORDER_DATE',
///              .              SIZE,   ET,     ISNULL               )
///
///               CALL EKACED ( HANDLE, SEGNO,  RECNO, 'COST',
///              .              SIZE,   COST,   ISNULL               )
///
///         C
///         C     Close the file to make the update permanent.
///         C
///               CALL EKCLS ( HANDLE )
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
/// -    SPICELIB Version 1.1.0, 25-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 09-JAN-2002 (NJB)
///
///         Documentation change: instances of the phrase "fast load"
///         were replaced with "fast write."
///
/// -    Beta Version 1.0.0, 19-DEC-1995 (NJB)
/// ```
pub fn ekinsr(ctx: &mut SpiceContext, handle: i32, segno: i32, recno: i32) -> crate::Result<()> {
    EKINSR(handle, segno, recno, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKINSR ( EK, insert record into segment )
pub fn EKINSR(HANDLE: i32, SEGNO: i32, RECNO: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BASE: i32 = 0;
    let COLDSC = StackArray::<i32, 11>::new(1..=CDSCSZ);
    let mut LASTP: i32 = 0;
    let mut LASTW: i32 = 0;
    let mut MBASE: i32 = 0;
    let mut MP: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut NLINKS: i32 = 0;
    let mut NREC: i32 = 0;
    let mut P: i32 = 0;
    let mut RECBAS: i32 = 0;
    let mut RECPTR = StackArray::<i32, 254>::new(1..=IPSIZE);
    let mut ROOM: i32 = 0;
    let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);
    let mut SIZE: i32 = 0;
    let mut ISSHAD: bool = false;

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
        CHKIN(b"EKINSR", ctx)?;
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
        CHKOUT(b"EKINSR", ctx)?;
        return Ok(());
    }

    //
    // Look up the integer metadata page and page base for the segment.
    // Given the base address, we can read the pertinent metadata in
    // one shot.
    //
    ZZEKMLOC(HANDLE, SEGNO, &mut MP, &mut MBASE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"EKINSR", ctx)?;
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
    // We'll need to know how many columns the segment has in order to
    // compute the size of the record pointer.  The record pointer
    // contains DPTBAS items plus two elements for each column.
    //
    NCOLS = SEGDSC[NCIDX];
    SIZE = (DPTBAS + NCOLS);

    //
    // We're assuming the record pointer can fit on an integer page.
    // If this is not the case, we've got a bug.
    //
    if (SIZE > IPSIZE) {
        SETMSG(b"Record pointer requires # integer words; EK software assumes size is <= #.  This is an EK software bug.  Contact NAIF.", ctx);
        ERRINT(b"#", SIZE, ctx);
        ERRINT(b"#", IPSIZE, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"EKINSR", ctx)?;
        return Ok(());
    }

    //
    // Check the number of records already present.  RECNO must not
    // exceed this count by more than 1.
    //
    NREC = SEGDSC[NRIDX];

    if ((RECNO < 1) || (RECNO > (NREC + 1))) {
        SETMSG(b"Record number = #; valid range is 1:#.", ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", (NREC + 1), ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"EKINSR", ctx)?;
        return Ok(());
    }

    //
    // Find the last integer data page and the last word in use in that
    // page.  If there's enough room, we can store the record pointer
    // in the current page.
    //
    LASTP = SEGDSC[LIPIDX];
    LASTW = SEGDSC[LIWIDX];
    ROOM = (IPSIZE - LASTW);

    //
    // Initialize the record pointer:  set the record's status and
    // set the data pointers to indicate no data is present.  To
    // determine the status, we must know whether the parent file is
    // shadowed.
    //
    CLEARI(IPSIZE, RECPTR.as_slice_mut());
    FILLI(UNINIT, (IPSIZE - DPTBAS), RECPTR.as_slice_mut());

    EKSHDW(HANDLE, &mut ISSHAD);

    if ISSHAD {
        RECPTR[STAIDX] = NEW;
    } else {
        RECPTR[STAIDX] = OLD;
    }

    //
    // Find a place to write the record pointer.
    //
    if (SIZE <= ROOM) {
        //
        // Just write the record pointer into the current integer page.
        //
        ZZEKPGBS(INT, LASTP, &mut BASE, ctx)?;

        RECBAS = (BASE + LASTW);

        DASUDI(
            HANDLE,
            (RECBAS + 1),
            (RECBAS + SIZE),
            RECPTR.as_slice(),
            ctx,
        )?;

        //
        // Update the page's metadata to reflect the addition.  The
        // page gains a link.
        //
        DASRDI(
            HANDLE,
            (BASE + ILCIDX),
            (BASE + ILCIDX),
            std::slice::from_mut(&mut NLINKS),
            ctx,
        )?;
        DASUDI(
            HANDLE,
            (BASE + ILCIDX),
            (BASE + ILCIDX),
            &[(NLINKS + 1)],
            ctx,
        )?;

        //
        // The last integer word in use has changed too.
        //
        SEGDSC[LIWIDX] = (SEGDSC[LIWIDX] + SIZE);
    } else {
        //
        // Allocate an integer page.
        //
        ZZEKAPS(
            HANDLE,
            SEGDSC.as_slice(),
            INT,
            false,
            &mut P,
            &mut RECBAS,
            ctx,
        )?;

        //
        // Write out the record pointer.
        //
        DASUDI(
            HANDLE,
            (RECBAS + 1),
            (RECBAS + SIZE),
            RECPTR.as_slice(),
            ctx,
        )?;

        //
        // Update the page's metadata to reflect the addition.  The
        // page starts out with one link.
        //
        DASUDI(HANDLE, (RECBAS + ILCIDX), (RECBAS + ILCIDX), &[1], ctx)?;

        //
        // Update the segment's metadata to reflect the addition of a
        // data page.  The last page in use is the one we just wrote to.
        // The last word in use is the last word of the record pointer.
        //
        SEGDSC[LIPIDX] = P;
        SEGDSC[LIWIDX] = SIZE;
    }

    //
    // Update the segment's metadata to reflect the addition of the
    // new record.  The base address of the record is inserted into
    // the data record tree at index RECNO.  The record count gets
    // incremented.
    //
    ZZEKTRIN(HANDLE, SEGDSC[RTIDX], RECNO, RECBAS, ctx)?;

    SEGDSC[NRIDX] = (SEGDSC[NRIDX] + 1);

    //
    // If the segment is shadowed but no backup segment exists yet, we
    // need to create one.  We'll let ZZEKRBCK take care of the details.
    // Note that for data additions, the input argument COLDSC is
    // ignored.
    //
    ZZEKRBCK(b"ADD", HANDLE, SEGDSC.as_slice(), COLDSC.as_slice(), RECNO);

    //
    // Write out the updated segment descriptor.
    //
    DASUDI(
        HANDLE,
        (MBASE + 1),
        (MBASE + SDSCSZ),
        SEGDSC.as_slice(),
        ctx,
    )?;

    CHKOUT(b"EKINSR", ctx)?;
    Ok(())
}
