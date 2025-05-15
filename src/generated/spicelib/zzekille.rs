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

//$Procedure    ZZEKILLE ( EK, indirect, last elt less than or equal to )
pub fn ZZEKILLE(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    NROWS: i32,
    DTYPE: i32,
    CVAL: &[u8],
    DVAL: f64,
    IVAL: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<i32> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut ZZEKILLE: i32 = 0;
    let mut DNUM: f64 = 0.0;
    let mut COLTYP: i32 = 0;
    let mut INUM: i32 = 0;
    let mut REC: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Initialize the function's return value.
    //
    ZZEKILLE = 0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(ZZEKILLE);
    } else {
        CHKIN(b"ZZEKILLE", ctx)?;
    }

    //
    // Validate the number of rows in the column.
    //
    if (NROWS < 1) {
        //
        // There's nobody home---that is, there is nothing in the array
        // to compare against.  Zero is the only sensible thing to return.
        //
        ZZEKILLE = 0;
        SETMSG(b"Number of rows must be positive; was #.", ctx);
        ERRINT(b"#", NROWS, ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(b"ZZEKILLE", ctx)?;
        return Ok(ZZEKILLE);
    }

    //
    // Hand off the problem to the LLE routine of the correct type.
    //
    COLTYP = COLDSC[TYPIDX];

    if (COLTYP == CHR) {
        ZZEKLLEC(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            CVAL,
            &mut ZZEKILLE,
            &mut REC,
            ctx,
        )?;
    } else if (COLTYP == DP) {
        if (DTYPE == DP) {
            DNUM = DVAL;
        } else {
            DNUM = IVAL as f64;
        }

        ZZEKLLED(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            DNUM,
            &mut ZZEKILLE,
            &mut REC,
            ctx,
        )?;
    } else if (COLTYP == TIME) {
        ZZEKLLED(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            DVAL,
            &mut ZZEKILLE,
            &mut REC,
            ctx,
        )?;
    } else if (COLTYP == INT) {
        if (DTYPE == DP) {
            INUM = intrinsics::IDNINT(DVAL);
        } else {
            INUM = IVAL;
        }

        ZZEKLLEI(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            INUM,
            &mut ZZEKILLE,
            &mut REC,
            ctx,
        )?;
    } else {
        SETMSG(b"The data type # is not supported.", ctx);
        ERRINT(b"#", COLTYP, ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(b"ZZEKILLE", ctx)?;
        return Ok(ZZEKILLE);
    }

    CHKOUT(b"ZZEKILLE", ctx)?;
    Ok(ZZEKILLE)
}
