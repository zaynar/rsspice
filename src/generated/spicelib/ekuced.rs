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

/// EK, update d.p. column entry
///
/// Update a double precision column entry in a specified EK record.
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
///  RECNO      I   Record in which entry is to be updated.
///  COLUMN     I   Column name.
///  NVALS      I   Number of values in new column entry.
///  DVALS      I   Double precision values to add to column.
///  ISNULL     I   Flag indicating whether column entry is null.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is a file handle attached to an EK open for
///           write access.
///
///  SEGNO    is the index of the segment containing the column
///           entry to be updated.
///
///  RECNO    is the index of the record containing the column
///           entry to be updated. This record number is
///           relative to the start of the segment indicated by
///           SEGNO; the first record in the segment has index 1.
///
///  COLUMN   is the name of the column containing the entry to
///           be updated.
///
///  NVALS,
///  DVALS    are, respectively, the number of values to add to
///           the specified column and the set of values
///           themselves. The data values are written in to the
///           specified column and record.
///
///           If the  column has fixed-size entries, then NVALS
///           must equal the entry size for the specified column.
///
///           For columns with variable-sized entries, the size
///           of the new entry need not match the size of the
///           entry it replaces. In particular, the new entry
///           may be larger.
///
///  ISNULL   is a logical flag indicating whether the entry is
///           null. If ISNULL is .FALSE., the column entry
///           defined by NVALS and DVALS is added to the
///           specified kernel file.
///
///           If ISNULL is .TRUE., NVALS and DVALS are ignored.
///           The contents of the column entry are undefined.
///           If the column has fixed-length, variable-size
///           entries, the number of entries is considered to
///           be 1.
///
///           The new entry may be null even though it replaces
///           a non-null value, and vice versa.
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
///  4)  If COLUMN specifies a column of whose data type is not DOUBLE
///      PRECISION or TIME, the error SPICE(WRONGDATATYPE) is signaled.
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
///      a double precision class known to this routine, the error
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
///  be added one logical element at a time. Partial assignments of
///  logical elements are not supported.
///
///  Since columns of data type TIME are implemented using double
///  precision column classes, this routine may be used to update
///  columns of type TIME.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Replace the value in the third record of the column DCOL in
///      the fifth segment of an EK file designated by HANDLE. Set
///      the new value to 999.D0.
///
///         CALL EKUCED ( HANDLE, 5, 3, 'DCOL', 1, 999.D0, .FALSE. )
///
///
///  2)  Same as (1), but this time add a null value. The argument
///      999.D0 is ignored because the null flag is set to .TRUE.
///
///         CALL EKUCED ( HANDLE, 5, 3, 'DCOL', 1, 999.D0, .TRUE. )
///
///
///  3)  Replace the entry in the third record of the column DARRAY in
///      the fifth segment of an EK file designated by HANDLE. Set
///      the new value using an array DBUFF of 10 d.p. values.
///
///         CALL EKUCED ( HANDLE, 5, 3, 'DARRAY', 10, DBUFF, .FALSE. )
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
/// -    SPICELIB Version 1.2.1, 06-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 06-FEB-2015 (NJB)
///
///         Now uses ERRHAN to insert DAS file name into
///         long error messages.
///
/// -    SPICELIB Version 1.1.0, 20-JUN-1999 (WLT)
///
///         Removed unbalanced call to CHKOUT.
///
/// -    SPICELIB Version 1.0.0, 26-SEP-1995 (NJB)
/// ```
pub fn ekuced(
    ctx: &mut SpiceContext,
    handle: i32,
    segno: &mut i32,
    recno: i32,
    column: &str,
    nvals: i32,
    dvals: &[f64],
    isnull: bool,
) -> crate::Result<()> {
    EKUCED(
        handle,
        segno,
        recno,
        column.as_bytes(),
        nvals,
        dvals,
        isnull,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKUCED ( EK, update d.p. column entry )
pub fn EKUCED(
    HANDLE: i32,
    SEGNO: &mut i32,
    RECNO: i32,
    COLUMN: &[u8],
    NVALS: i32,
    DVALS: &[f64],
    ISNULL: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DVALS = DummyArray::new(DVALS, 1..);
    let mut COLDSC = StackArray::<i32, 11>::new(1..=CDSCSZ);
    let mut CLASS: i32 = 0;
    let mut DTYPE: i32 = 0;
    let mut RECPTR: i32 = 0;
    let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);
    let mut ISSHAD: bool = false;

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
    // This column had better be of double precision or `time' type.
    //
    DTYPE = COLDSC[TYPIDX];

    if ((DTYPE != DP) && (DTYPE != TIME)) {
        CHKIN(b"EKUCED", ctx)?;
        SETMSG(b"Column # is of type #; EKUCED only works with d.p. or TIME columns.  RECNO = #; SEGNO = #; EK = #.", ctx);
        ERRCH(b"#", COLUMN, ctx);
        ERRINT(b"#", DTYPE, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", *SEGNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(WRONGDATATYPE)", ctx)?;
        CHKOUT(b"EKUCED", ctx)?;
        return Ok(());
    }

    //
    // Look up the record pointer for the target record.
    //
    ZZEKTRDP(HANDLE, SEGDSC[RTIDX], RECNO, &mut RECPTR, ctx)?;

    //
    // Determine whether the EK is shadowed.
    //
    EKSHDW(HANDLE, &mut ISSHAD);

    //
    // If the EK is shadowed, we must back up the current column entry
    // if the entry has not already been backed up.  ZZEKRBCK will
    // handle this task.
    //
    if ISSHAD {
        ZZEKRBCK(
            b"UPDATE",
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            RECNO,
        );
    }

    //
    // Now it's time to carry out the replacement.
    //
    CLASS = COLDSC[CLSIDX];

    if (CLASS == 2) {
        //
        // Class 2 columns contain scalar d.p. data.
        //
        ZZEKUE02(
            HANDLE,
            SEGDSC.as_slice_mut(),
            COLDSC.as_slice(),
            RECPTR,
            *DVALS.first(),
            ISNULL,
            ctx,
        )?;
    } else if (CLASS == 5) {
        //
        // Class 5 columns contain array-valued d.p. data.
        //
        ZZEKUE05(
            HANDLE,
            SEGDSC.as_slice_mut(),
            COLDSC.as_slice(),
            RECPTR,
            NVALS,
            DVALS.as_slice(),
            ISNULL,
            ctx,
        )?;
    } else {
        //
        // This is an unsupported d.p. column class.
        //
        *SEGNO = SEGDSC[SNOIDX];

        CHKIN(b"EKUCED", ctx)?;
        SETMSG(b"Class # from input column descriptor is not a supported d.p. class.  COLUMN = #; RECNO = #; SEGNO = #; EK = #.", ctx);
        ERRINT(b"#", CLASS, ctx);
        ERRCH(b"#", COLUMN, ctx);
        ERRINT(b"#", RECNO, ctx);
        ERRINT(b"#", *SEGNO, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(NOCLASS)", ctx)?;
        CHKOUT(b"EKUCED", ctx)?;
        return Ok(());
    }

    Ok(())
}
