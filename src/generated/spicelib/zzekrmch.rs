//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CNAMSZ: i32 = 32;
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
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
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
const WSTR: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b"*");
const WCHR: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b"%");
const MAXQRY: i32 = 2000;
const MAXSEL: i32 = 50;
const MAXTAB: i32 = 10;
const MAXCON: i32 = 1000;
const MXJOIN: i32 = 10;
const MXJCON: i32 = 100;
const MAXORD: i32 = 10;
const MAXTOK: i32 = 500;
const MAXQNM: i32 = 100;
const MAXCLN: i32 = MAXQRY;
const MAXSTR: i32 = 1024;
const MAXCOL: i32 = 10;

//$Procedure      ZZEKRMCH ( EK, row match )
pub fn ZZEKRMCH(
    NCNSTR: i32,
    ACTIVE: &[bool],
    HANDLE: i32,
    SEGDSC: &[i32],
    CDSCRS: &[i32],
    ROW: i32,
    ELTS: &[i32],
    OPS: &[i32],
    VTYPES: &[i32],
    CHRBUF: &[u8],
    CBEGS: &[i32],
    CENDS: &[i32],
    DVALS: &[f64],
    IVALS: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let ACTIVE = DummyArray::new(ACTIVE, 1..);
    let SEGDSC = DummyArray::new(SEGDSC, 1..);
    let CDSCRS = DummyArray2D::new(CDSCRS, 1..=CDSCSZ, 1..);
    let ELTS = DummyArray::new(ELTS, 1..);
    let OPS = DummyArray::new(OPS, 1..);
    let VTYPES = DummyArray::new(VTYPES, 1..);
    let CBEGS = DummyArray::new(CBEGS, 1..);
    let CENDS = DummyArray::new(CENDS, 1..);
    let DVALS = DummyArray::new(DVALS, 1..);
    let IVALS = DummyArray::new(IVALS, 1..);
    let mut ZZEKRMCH: bool = false;
    let mut I: i32 = 0;

    //
    // Non-SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    // For each active constraint in the list, see whether the specified
    // row satisfies the constraint.  If any constraint is not satisfied,
    // return immediately.
    //
    I = 1;
    ZZEKRMCH = true;

    while ((I <= NCNSTR) && ZZEKRMCH) {
        if ACTIVE[I] {
            //
            // See whether the row satisfies the Ith constraint.
            //
            ZZEKRMCH = ZZEKSCMP(
                OPS[I],
                HANDLE,
                SEGDSC.as_slice(),
                CDSCRS.subarray([1, I]),
                ROW,
                ELTS[I],
                VTYPES[I],
                fstr::substr(CHRBUF, CBEGS[I]..=CENDS[I]),
                DVALS[I],
                IVALS[I],
                false,
                ctx,
            )?;
        }

        //
        // Take a look at the next constraint.
        //
        I = (I + 1);
    }

    //
    // It's a match if we got this far.
    //
    Ok(ZZEKRMCH)
}
