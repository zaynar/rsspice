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

//$Procedure ZZEKRCMP ( EK, row comparison )
pub fn ZZEKRCMP(
    OP: i32,
    NCOLS: i32,
    HAN1: i32,
    SGDSC1: &[i32],
    CDLST1: &[i32],
    ROW1: i32,
    ELTS1: &[i32],
    HAN2: i32,
    SGDSC2: &[i32],
    CDLST2: &[i32],
    ROW2: i32,
    ELTS2: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let SGDSC1 = DummyArray::new(SGDSC1, 1..=SDSCSZ);
    let CDLST1 = DummyArray2D::new(CDLST1, 1..=CDSCSZ, 1..);
    let ELTS1 = DummyArray::new(ELTS1, 1..);
    let SGDSC2 = DummyArray::new(SGDSC2, 1..=SDSCSZ);
    let CDLST2 = DummyArray2D::new(CDLST2, 1..=CDSCSZ, 1..);
    let ELTS2 = DummyArray::new(ELTS2, 1..);
    let mut ZZEKRCMP: bool = false;
    let mut CLDSCS = StackArray2D::<i32, 22>::new(1..=CDSCSZ, 1..=2);
    let mut COL: i32 = 0;
    let mut ELTS = StackArray::<i32, 2>::new(1..=2);
    let mut HANS = StackArray::<i32, 2>::new(1..=2);
    let mut REL: i32 = 0;
    let mut ROWS = StackArray::<i32, 2>::new(1..=2);
    let mut SGDSCS = StackArray2D::<i32, 48>::new(1..=SDSCSZ, 1..=2);

    //
    // Non-SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in for speed.
    //
    // The function value defaults to .FALSE.
    //
    ZZEKRCMP = false;

    //
    // The input column descriptors identify the columns to be used
    // to define an order relation on the input rows.  The order
    // relation is `dictionary' ordering:  if the elements of the
    // first n columns of both rows are equal, the corresponding
    // elements in the (n+1)st columns are compared to attempt to
    // break the tie.
    //
    // The first step is to determine the relation that holds between
    // the rows.  We start out assuming we have equality.
    //
    HANS[1] = HAN1;
    HANS[2] = HAN2;

    MOVEI(SGDSC1.as_slice(), SDSCSZ, SGDSCS.subarray_mut([1, 1]));
    MOVEI(SGDSC2.as_slice(), SDSCSZ, SGDSCS.subarray_mut([1, 2]));

    ROWS[1] = ROW1;
    ROWS[2] = ROW2;

    REL = EQ;
    COL = 1;

    while ((COL <= NCOLS) && (REL == EQ)) {
        //
        // Compare the entries in the two rows in the columns indicated
        // by the Nth column descriptor pair.
        //
        MOVEI(
            CDLST1.subarray([1, COL]),
            CDSCSZ,
            CLDSCS.subarray_mut([1, 1]),
        );
        MOVEI(
            CDLST2.subarray([1, COL]),
            CDSCSZ,
            CLDSCS.subarray_mut([1, 2]),
        );

        ELTS[1] = ELTS1[COL];
        ELTS[2] = ELTS2[COL];

        REL = ZZEKECMP(
            HANS.as_slice(),
            SGDSCS.as_slice(),
            CLDSCS.as_slice(),
            ROWS.as_slice(),
            ELTS.as_slice(),
            ctx,
        )?;

        //
        // We've completed the comparison for the column numbered COL.
        //
        COL = (COL + 1);
    }

    //
    // Determine the truth of the input relational expression.
    //
    if (OP == EQ) {
        ZZEKRCMP = (REL == EQ);
    } else if (OP == LT) {
        ZZEKRCMP = (REL == LT);
    } else if (OP == LE) {
        ZZEKRCMP = (REL != GT);
    } else if (OP == GT) {
        ZZEKRCMP = (REL == GT);
    } else if (OP == GE) {
        ZZEKRCMP = (REL != LT);
    } else if (OP == NE) {
        ZZEKRCMP = (REL != EQ);
    } else {
        //
        // Sorry, we couldn't resist.
        //
        ZZEKRCMP = false;

        CHKIN(b"ZZEKRCMP", ctx)?;
        SETMSG(b"The relational operator # was not recognized.", ctx);
        ERRINT(b"#", OP, ctx);
        SIGERR(b"SPICE(UNNATURALRELATION)", ctx)?;
        CHKOUT(b"ZZEKRCMP", ctx)?;
        return Ok(ZZEKRCMP);
    }

    Ok(ZZEKRCMP)
}
