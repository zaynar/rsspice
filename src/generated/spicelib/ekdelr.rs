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

/// EK, delete record from segment
///
/// Delete a specified record from a specified E-kernel segment.
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
///  SEGNO    is the number of the segment from which to
///           delete the specified record.
///
///  RECNO    is the index of the record to delete. RECNO must
///           be in the range 1 : NREC, where NREC is the
///           number of records in the segment prior to the
///           insertion.
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
///  This routine operates by side effects: it deletes a record
///  from an EK segment. Deleting a record implies:
///
///     1) All column entries in the record are deleted.
///
///     2) Link counts are decremented for data pages containing
///        column entries in the record to be deleted. Pages whose
///        link counts drop to zero are freed.
///
///     3) All column indexes are updated for the parent segment.
///
///     4) The link count is decremented for the page containing the
///        record pointer structure of the record to be deleted. If
///        the link count drops to zero, the page is freed.
///
///     5) The pointer to the deleted record is deleted from the
///        record tree for the parent segment.
///
///     6) The segment's metadata is updated to reflect the new
///        record count.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Suppose the second segment of an EK file designated by
///      HANDLE contains 5 records:
///
///         +-----------------+
///         |     Record 1    |
///         +-----------------+
///         |     Record 2    |
///         +-----------------+
///         |     Record 3    |
///         +-----------------+
///         |     Record 4    |
///         +-----------------+
///         |     Record 5    |
///         +-----------------+
///
///      Then the call
///
///         CALL EKDELR ( HANDLE, 2, 3 )
///
///      deletes the third record from the segment, leaving the
///      segment's contents as follows:
///
///         +-----------------+
///         |     Record 1    |
///         +-----------------+
///         |     Record 2    |
///         +-----------------+
///         |     Record 4    |
///         +-----------------+
///         |     Record 5    |
///         +-----------------+
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
/// -    SPICELIB Version 1.1.0, 26-MAY-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 19-DEC-1995 (NJB)
/// ```
pub fn ekdelr(
    ctx: &mut SpiceContext,
    handle: i32,
    segno: i32,
    recno: &mut i32,
) -> crate::Result<()> {
    EKDELR(handle, segno, recno, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKDELR ( EK, delete record from segment )
pub fn EKDELR(
    HANDLE: i32,
    SEGNO: i32,
    RECNO: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut COLUMN = [b' '; CNAMSZ as usize];
    let mut BASE: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut COLDSC = StackArray::<i32, 11>::new(1..=CDSCSZ);
    let mut DSCBAS: i32 = 0;
    let mut MBASE: i32 = 0;
    let mut MP: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut NLINKS: i32 = 0;
    let mut NREC: i32 = 0;
    let mut P: i32 = 0;
    let mut RECPTR: i32 = 0;
    let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);
    let mut UNIT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Non-SPICELIB functions
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
        CHKIN(b"EKDELR", ctx)?;
    }

    //
    // Before trying to actually modify the file, do every error
    // check we can.
    //
    // Is this file handle valid--is the file open for paged write
    // access?  Signal an error if not.
    //
    ZZEKPGCH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"EKDELR", ctx)?;
        return Ok(());
    }

    //
    // Look up the integer metadata page and page base for the segment.
    // Given the base address, we can read the pertinent metadata in
    // one shot.
    //
    ZZEKMLOC(HANDLE, SEGNO, &mut MP, &mut MBASE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"EKDELR", ctx)?;
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
    // In case the target EK is shadowed, let the shadow system know
    // about the deletion.  This must be done before the data is
    // deleted.  The argument COLDSC is unused on this call.
    //
    ZZEKRBCK(
        b"DELETE",
        HANDLE,
        SEGDSC.as_slice(),
        COLDSC.as_slice(),
        *RECNO,
    );

    //
    // We'll need to know how many columns the segment has in order to
    // compute the size of the record pointer.  The record pointer
    // contains DPTBAS items plus two elements for each column.
    //
    NCOLS = SEGDSC[NCIDX];

    //
    // Check the number of records already present.  RECNO must not
    // exceed this count.
    //
    NREC = SEGDSC[NRIDX];

    if ((*RECNO < 1) || (*RECNO > NREC)) {
        SETMSG(b"Record number = #; valid range is 1:#.", ctx);
        ERRINT(b"#", *RECNO, ctx);
        ERRINT(b"#", NREC, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"EKDELR", ctx)?;
        return Ok(());
    }

    //
    // Delete all of the column entries in the record.  The deletion
    // routines handle updating column indexes and freeing unlinked
    // pages.
    //
    ZZEKTRDP(HANDLE, SEGDSC[RTIDX], *RECNO, &mut RECPTR, ctx)?;

    for I in 1..=NCOLS {
        //
        // Get the descriptor of the Ith column.
        //
        DSCBAS = ((MBASE + CDOFF) + ((I - 1) * CDSCSZ));

        DASRDI(
            HANDLE,
            (DSCBAS + 1),
            (DSCBAS + CDSCSZ),
            COLDSC.as_slice_mut(),
            ctx,
        )?;

        CLASS = COLDSC[CLSIDX];

        //
        // Delete the entry in the current column.
        //
        if (CLASS == 1) {
            ZZEKDE01(
                HANDLE,
                SEGDSC.as_slice_mut(),
                COLDSC.as_slice(),
                RECPTR,
                ctx,
            )?;
        } else if (CLASS == 2) {
            ZZEKDE02(
                HANDLE,
                SEGDSC.as_slice_mut(),
                COLDSC.as_slice(),
                RECPTR,
                ctx,
            )?;
        } else if (CLASS == 3) {
            ZZEKDE03(
                HANDLE,
                SEGDSC.as_slice_mut(),
                COLDSC.as_slice(),
                RECPTR,
                ctx,
            )?;
        } else if (CLASS == 4) {
            ZZEKDE04(
                HANDLE,
                SEGDSC.as_slice_mut(),
                COLDSC.as_slice(),
                RECPTR,
                ctx,
            )?;
        } else if (CLASS == 5) {
            ZZEKDE05(
                HANDLE,
                SEGDSC.as_slice_mut(),
                COLDSC.as_slice(),
                RECPTR,
                ctx,
            )?;
        } else if (CLASS == 6) {
            ZZEKDE06(
                HANDLE,
                SEGDSC.as_slice_mut(),
                COLDSC.as_slice(),
                RECPTR,
                ctx,
            )?;
        } else {
            //
            // This is an unsupported class.
            //
            *RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

            DASHLU(HANDLE, &mut UNIT, ctx)?;
            ZZEKCNAM(HANDLE, COLDSC.as_slice(), &mut COLUMN, ctx)?;

            SETMSG(b"Class # from input column descriptor is not supported.  COLUMN = #; RECNO = #; SEGNO = #; EK = #.", ctx);
            ERRINT(b"#", CLASS, ctx);
            ERRCH(b"#", &COLUMN, ctx);
            ERRINT(b"#", *RECNO, ctx);
            ERRINT(b"#", SEGDSC[SNOIDX], ctx);
            ERRFNM(b"#", UNIT, ctx)?;
            SIGERR(b"SPICE(NOCLASS)", ctx)?;
            CHKOUT(b"EKDELR", ctx)?;
            return Ok(());
        }
    }

    //
    // Find the page containing the record pointer.
    //
    ZZEKPGPG(INT, (RECPTR + 1), &mut P, &mut BASE, ctx)?;

    //
    // Get the link count for the page.  If we have more
    // than one link to the page, decrement the link count.  If
    // we're down to one link, this deletion will finish off the
    // page:  we'll deallocate it.
    //
    ZZEKGLNK(HANDLE, INT, P, &mut NLINKS, ctx)?;

    if (NLINKS > 1) {
        ZZEKSLNK(HANDLE, INT, P, (NLINKS - 1), ctx)?;
    } else {
        //
        // If we removed the last item from the page, we can delete
        // the page.  ZZEKDPS adjusts the segment's metadata
        // to reflect the deallocation.
        //
        ZZEKDPS(HANDLE, SEGDSC.as_slice_mut(), INT, P, ctx)?;
    }

    //
    // The entry corresponding to the record is deleted from
    // the data record tree at index RECNO.  The record count gets
    // decremented.
    //
    ZZEKTRDL(HANDLE, SEGDSC[RTIDX], *RECNO, ctx)?;

    SEGDSC[NRIDX] = (SEGDSC[NRIDX] - 1);

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

    CHKOUT(b"EKDELR", ctx)?;
    Ok(())
}
