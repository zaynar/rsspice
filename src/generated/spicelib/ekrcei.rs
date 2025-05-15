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

/// EK, read column entry element, integer
///
/// Read data from an integer column in a specified EK record.
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
///  HANDLE     I   Handle attached to EK file.
///  SEGNO      I   Index of segment containing record.
///  RECNO      I   Record from which data is to be read.
///  COLUMN     I   Column name.
///  NVALS      O   Number of values in column entry.
///  IVALS      O   Integer values in column entry.
///  ISNULL     O   Flag indicating whether column entry is null.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is an EK file handle. The file may be open for read or
///           write access.
///
///  SEGNO    is the index of the segment from which data is to be
///           read.
///
///  RECNO    is the index of the record from which data is to be read.
///           This record number is relative to the start of the
///           segment indicated by SEGNO; the first record in the
///           segment has index 1.
///
///  COLUMN   is the name of the column from which data is to be read.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NVALS,
///  IVALS    are, respectively, the number of values found in the
///           specified column entry and the set of values themselves.
///
///           For columns having fixed-size entries, when a a column
///           entry is null, NVALS is still set to the column entry
///           size. For columns having variable-size entries, NVALS is
///           set to 1 for null entries.
///
///  ISNULL   is a logical flag indicating whether the returned column
///           entry is null.
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
///  3)  If RECNO is out of range, an error is signaled by a routine in
///      the call tree of this routine.
///
///  4)  If COLUMN is not the name of a declared column, an error
///      is signaled by a routine in the call tree of this routine.
///
///  5)  If COLUMN specifies a column of whose data type is not
///      integer, the error SPICE(WRONGDATATYPE) is signaled.
///
///  6)  If COLUMN specifies a column of whose class is not
///      an integer class known to this routine, the error
///      SPICE(NOCLASS) is signaled.
///
///  7)  If an attempt is made to read an uninitialized column entry,
///      an error is signaled by a routine in the call tree of this
///      routine. A null entry is considered to be initialized, but
///      entries do not contain null values by default.
///
///  8)  If an I/O error occurs while reading the indicated file, the
///      error is signaled by a routine in the call tree of this
///      routine.
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
///  This routine is a utility that allows an EK file to be read
///  directly without using the high-level query interface.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Read the value in the third record of the column ICOL in
///      the fifth segment of an EK file designated by HANDLE.
///
///         CALL EKRCEI ( HANDLE, 5, 3, 'ICOL', N, IVAL, ISNULL )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  EK files open for write access are not necessarily readable.
///      In particular, a column entry can be read only if it has been
///      initialized. The caller is responsible for determining
///      when it is safe to read from files open for write access.
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
/// -    SPICELIB Version 1.3.1, 26-MAY-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.3.0, 06-FEB-2015 (NJB)
///
///         Now uses ERRHAN to insert DAS file name into
///         long error messages.
///
/// -    SPICELIB Version 1.2.0, 20-JUN-1999 (WLT)
///
///         Removed unbalanced call to CHKOUT.
///
/// -    SPICELIB Version 1.1.0, 28-JUL-1997 (NJB)
///
///         Bug fix: Record number, not record pointer, is now supplied
///         to look up data in the class 7 case. Miscellaneous header
///         changes were made as well.
///
/// -    SPICELIB Version 1.0.0, 06-NOV-1995 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 28-JUL-1997 (NJB)
///
///         Bug fix: Record number, not record pointer, is now supplied
///         to look up data in the class 7 case. For class 7 columns,
///         column entry locations are calculated directly from record
///         numbers; no indirection is used.
///
///         Miscellaneous header changes were made as well.
/// ```
pub fn ekrcei(
    ctx: &mut SpiceContext,
    handle: i32,
    segno: &mut i32,
    recno: i32,
    column: &str,
    nvals: &mut i32,
    ivals: &mut [i32],
    isnull: &mut bool,
) -> crate::Result<()> {
    EKRCEI(
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

//$Procedure EKRCEI ( EK, read column entry element, integer )
pub fn EKRCEI(
    HANDLE: i32,
    SEGNO: &mut i32,
    RECNO: i32,
    COLUMN: &[u8],
    NVALS: &mut i32,
    IVALS: &mut [i32],
    ISNULL: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut IVALS = DummyArrayMut::new(IVALS, 1..);
    let mut COLDSC = StackArray::<i32, 11>::new(1..=CDSCSZ);
    let mut CLASS: i32 = 0;
    let mut DTYPE: i32 = 0;
    let mut RECPTR: i32 = 0;
    let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);
    let mut FOUND: bool = false;

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
        CHKIN(b"EKRCEI", ctx)?;
        SETMSG(b"Column # is of type #; EKRCEI only works with integer columns.  RECNO = #; SEGNO = #; EK = #.", ctx);
        ERRCH(b"#", COLUMN, ctx);
        ERRINT(b"#", DTYPE, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", *SEGNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(WRONGDATATYPE)", ctx)?;
        CHKOUT(b"EKRCEI", ctx)?;
        return Ok(());
    }

    //
    // Now it's time to read data from the file.  Call the low-level
    // reader appropriate to the column's class.
    //
    CLASS = COLDSC[CLSIDX];

    if (CLASS == 1) {
        //
        // Look up the record pointer for the target record.
        //
        ZZEKTRDP(HANDLE, SEGDSC[RTIDX], RECNO, &mut RECPTR, ctx)?;

        ZZEKRD01(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            RECPTR,
            IVALS.first_mut(),
            ISNULL,
            ctx,
        )?;
        *NVALS = 1;
    } else if (CLASS == 4) {
        ZZEKTRDP(HANDLE, SEGDSC[RTIDX], RECNO, &mut RECPTR, ctx)?;

        *NVALS = ZZEKESIZ(HANDLE, SEGDSC.as_slice(), COLDSC.as_slice(), RECPTR, ctx)?;

        ZZEKRD04(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            RECPTR,
            1,
            *NVALS,
            IVALS.as_slice_mut(),
            ISNULL,
            &mut FOUND,
            ctx,
        )?;
    } else if (CLASS == 7) {
        //
        // Records in class 7 columns are identified by a record number
        // rather than a pointer.
        //
        ZZEKRD07(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            RECNO,
            IVALS.first_mut(),
            ISNULL,
            ctx,
        )?;
        *NVALS = 1;
    } else {
        //
        // This is an unsupported integer column class.
        //
        *SEGNO = SEGDSC[SNOIDX];

        CHKIN(b"EKRCEI", ctx)?;
        SETMSG(b"Class # from input column descriptor is not a supported integer class.  COLUMN = #; RECNO = #; SEGNO = #; EK = #.", ctx);
        ERRINT(b"#", CLASS, ctx);
        ERRCH(b"#", COLUMN, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", *SEGNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(NOCLASS)", ctx)?;
        CHKOUT(b"EKRCEI", ctx)?;
        return Ok(());
    }

    Ok(())
}
