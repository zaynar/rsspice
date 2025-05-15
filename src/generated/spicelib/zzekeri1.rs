//
// GENERATED FILE
//

use super::*;
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
const EQ: i32 = 1;
const GE: i32 = (EQ + 1);
const GT: i32 = (GE + 1);
const LE: i32 = (GT + 1);
const LT: i32 = (LE + 1);
const NE: i32 = (LT + 1);
const LIKE: i32 = (NE + 1);
const UNLIKE: i32 = (LIKE + 1);
const ISNULL: i32 = (UNLIKE + 1);
const NOTNUL: i32 = (ISNULL + 1);
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

//$Procedure ZZEKERI1 ( EK, LLE using record pointers, integer, type 1 )
pub fn ZZEKERI1(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    IKEY: i32,
    RECPTR: i32,
    NULL: bool,
    PRVIDX: &mut i32,
    PRVPTR: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut BEGIN: i32 = 0;
    let mut BEGPTR: i32 = 0;
    let mut END: i32 = 0;
    let mut ENDPTR: i32 = 0;
    let mut MIDDLE: i32 = 0;
    let mut MIDPTR: i32 = 0;
    let mut NREC: i32 = 0;
    let mut TREE: i32 = 0;
    let mut TSIZE: i32 = 0;
    let mut LEQ: bool = false;

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
    if FAILED(ctx) {
        return Ok(());
    }

    //
    // Make sure the number of records in the segment is at least as
    // large as the number of entries in the index:  we must not look
    // up any entries that don't exist!
    //
    TREE = COLDSC[IXPIDX];
    TSIZE = ZZEKTRSZ(HANDLE, TREE, ctx)?;
    NREC = SEGDSC[NRIDX];

    if (TSIZE > NREC) {
        CHKIN(b"ZZEKERI1", ctx)?;
        SETMSG(b"Index size = # but column contains # records.", ctx);
        ERRINT(b"#", TSIZE, ctx);
        ERRINT(b"#", NREC, ctx);
        SIGERR(b"SPICE(SIZEMISMATCH)", ctx)?;
        CHKOUT(b"ZZEKERI1", ctx)?;
        return Ok(());
    }

    //
    // Handle the case of an empty tree gracefully.
    //
    if (TSIZE == 0) {
        *PRVIDX = 0;
        *PRVPTR = 0;

        return Ok(());
    }

    //
    // The algorithm used here is very like unto that used in LSTLED.
    //
    BEGIN = 1;
    END = TSIZE;

    //
    // Get the record pointers BEGPTR and ENDPTR of the least and
    // greatest elements in the column.
    //
    ZZEKTRDP(HANDLE, TREE, BEGIN, &mut BEGPTR, ctx)?;
    ZZEKTRDP(HANDLE, TREE, END, &mut ENDPTR, ctx)?;

    //
    // Compare the input value to the smallest value in the column.
    //
    if ZZEKSCMP(
        GT,
        HANDLE,
        SEGDSC.as_slice(),
        COLDSC.as_slice(),
        BEGPTR,
        1,
        INT,
        b" ",
        0.0,
        IKEY,
        NULL,
        ctx,
    )? {
        //
        // The smallest entry of the column is greater than the input
        // value, so none of the entries are less than or equal to the
        // input value.
        //
        *PRVIDX = 0;
        *PRVPTR = 0;

        return Ok(());
    } else if (ZZEKSCMP(
        EQ,
        HANDLE,
        SEGDSC.as_slice(),
        COLDSC.as_slice(),
        BEGPTR,
        1,
        INT,
        b" ",
        0.0,
        IKEY,
        NULL,
        ctx,
    )? && (RECPTR < BEGPTR))
    {
        //
        // The smallest entry of the column is greater than the input
        // value, based on a comparison of record pointers, so none of the
        // entries are less than or equal to the input value.
        //
        *PRVIDX = 0;
        *PRVPTR = 0;

        return Ok(());
    }

    //
    // At this point, we know the input value is greater than or equal
    // to the smallest element of the column.
    //
    // Compare the input value to the greatest value in the column.
    //
    if ZZEKSCMP(
        LT,
        HANDLE,
        SEGDSC.as_slice(),
        COLDSC.as_slice(),
        ENDPTR,
        1,
        INT,
        b" ",
        0.0,
        IKEY,
        NULL,
        ctx,
    )? {
        //
        // The last element of the column is less than the
        // input value.
        //
        *PRVIDX = TSIZE;

        ZZEKTRDP(HANDLE, TREE, *PRVIDX, PRVPTR, ctx)?;

        return Ok(());
    } else if (ZZEKSCMP(
        EQ,
        HANDLE,
        SEGDSC.as_slice(),
        COLDSC.as_slice(),
        ENDPTR,
        1,
        INT,
        b" ",
        0.0,
        IKEY,
        NULL,
        ctx,
    )? && (ENDPTR <= RECPTR))
    {
        //
        // The last element of the column is less than or equal to the
        // input value, based on a comparison of record pointers.
        //
        *PRVIDX = TSIZE;
        *PRVPTR = ENDPTR;

        return Ok(());
    }

    //
    // The input value lies between some pair of column entries.
    // The value is greater than or equal to the smallest column entry
    // and less than the greatest entry, according to the dictionary
    // ordering we're using.
    //
    // Below, we'll use the variable LEQ to indicate whether the "middle"
    // element in our search is less than or equal to the input value.
    //
    while (END > (BEGIN + 1)) {
        //
        // Find the record pointer of the element whose ordinal position
        // is halfway between BEGIN and END.
        //
        MIDDLE = ((BEGIN + END) / 2);

        ZZEKTRDP(HANDLE, TREE, MIDDLE, &mut MIDPTR, ctx)?;

        //
        // Determine the order relation between IKEY and the column
        // entry at record MIDPTR.
        //
        if ZZEKSCMP(
            LT,
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            MIDPTR,
            1,
            INT,
            b" ",
            0.0,
            IKEY,
            NULL,
            ctx,
        )? {
            //
            // The column element at record MIDPTR is strictly less than
            // IKEY, based on data values.
            //
            LEQ = true;
        } else if ZZEKSCMP(
            EQ,
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            MIDPTR,
            1,
            INT,
            b" ",
            0.0,
            IKEY,
            NULL,
            ctx,
        )? {
            //
            // The column entry's value matches IKEY.  We must
            // compare record pointers at this point.
            //
            LEQ = (MIDPTR <= RECPTR);
        } else {
            //
            // The inequality of data values is strict.
            //
            LEQ = false;
        }

        if LEQ {
            //
            // The middle value is less than or equal to the input
            // value.
            //
            BEGIN = MIDDLE;
        } else {
            END = MIDDLE;
        }
        //
        // The input value is greater than or equal to the element
        // having ordinal position BEGIN and strictly less than the
        // element having ordinal position END.
        //
    }

    *PRVIDX = BEGIN;
    ZZEKTRDP(HANDLE, TREE, *PRVIDX, PRVPTR, ctx)?;

    Ok(())
}
