//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
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
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
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

//$Procedure      ZZEKVMCH ( EK, vector match )
pub fn ZZEKVMCH(
    NCNSTR: i32,
    ACTIVE: &[bool],
    LHANS: &[i32],
    LSDSCS: &[i32],
    LCDSCS: &[i32],
    LROWS: &[i32],
    LELTS: &[i32],
    OPS: &[i32],
    RHANS: &[i32],
    RSDSCS: &[i32],
    RCDSCS: &[i32],
    RROWS: &[i32],
    RELTS: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let ACTIVE = DummyArray::new(ACTIVE, 1..);
    let LHANS = DummyArray::new(LHANS, 1..);
    let LSDSCS = DummyArray2D::new(LSDSCS, 1..=SDSCSZ, 1..);
    let LCDSCS = DummyArray2D::new(LCDSCS, 1..=CDSCSZ, 1..);
    let LROWS = DummyArray::new(LROWS, 1..);
    let LELTS = DummyArray::new(LELTS, 1..);
    let OPS = DummyArray::new(OPS, 1..);
    let RHANS = DummyArray::new(RHANS, 1..);
    let RSDSCS = DummyArray2D::new(RSDSCS, 1..=SDSCSZ, 1..);
    let RCDSCS = DummyArray2D::new(RCDSCS, 1..=CDSCSZ, 1..);
    let RROWS = DummyArray::new(RROWS, 1..);
    let RELTS = DummyArray::new(RELTS, 1..);
    let mut ZZEKVMCH: bool = false;
    let mut CVAL = ActualCharArray::new(MAXSTR, 1..=2);
    let mut CLDSCS = StackArray2D::<i32, 22>::new(1..=CDSCSZ, 1..=2);
    let mut CMPLEN = StackArray::<i32, 2>::new(1..=2);
    let mut CVLEN = StackArray::<i32, 2>::new(1..=2);
    let mut ELTS = StackArray::<i32, 2>::new(1..=2);
    let mut HANS = StackArray::<i32, 2>::new(1..=2);
    let mut N: i32 = 0;
    let mut REL: i32 = 0;
    let mut ROWS = StackArray::<i32, 2>::new(1..=2);
    let mut SGDSCS = StackArray2D::<i32, 48>::new(1..=SDSCSZ, 1..=2);
    let mut FOUND: bool = false;
    let mut NULL = StackArray::<bool, 2>::new(1..=2);

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
    // Use discovery check-in for speed.  Don't check RETURN.
    //
    // The function value defaults to .TRUE.  As we test the constraints,
    // we may find one that the input row vector doesn't satisfy, at
    // which point we can terminate the comparison.
    //
    ZZEKVMCH = true;
    N = 1;

    while ((N <= NCNSTR) && ZZEKVMCH) {
        if ACTIVE[N] {
            //
            // Apply the Nth join constraint to the input row vector.
            //
            // Compare the entries in the two rows in the columns indicated
            // by the Nth column descriptor pair.  To do this, find the
            // address ranges for each column entry.  We don't check the
            // found flag because every column entry has at least one
            // element.
            //
            //
            // We'll start out setting REL to EQ.  If we find out
            // otherwise, we'll change it.
            //
            HANS[1] = LHANS[N];
            HANS[2] = RHANS[N];

            MOVEI(LSDSCS.subarray([1, N]), SDSCSZ, SGDSCS.subarray_mut([1, 1]));
            MOVEI(RSDSCS.subarray([1, N]), SDSCSZ, SGDSCS.subarray_mut([1, 2]));

            ROWS[1] = LROWS[N];
            ROWS[2] = RROWS[N];

            ELTS[1] = LELTS[N];
            ELTS[2] = RELTS[N];

            MOVEI(LCDSCS.subarray([1, N]), CDSCSZ, CLDSCS.subarray_mut([1, 1]));
            MOVEI(RCDSCS.subarray([1, N]), CDSCSZ, CLDSCS.subarray_mut([1, 2]));

            REL = ZZEKECMP(
                HANS.as_slice(),
                SGDSCS.as_slice(),
                CLDSCS.as_slice(),
                ROWS.as_slice(),
                ELTS.as_slice(),
                ctx,
            )?;

            //
            // Determine the truth of the Nth input relational expression,
            // and set ZZEKVMCH accordingly.
            //
            if (OPS[N] == EQ) {
                ZZEKVMCH = (REL == EQ);
            } else if (OPS[N] == LT) {
                ZZEKVMCH = (REL == LT);
            } else if (OPS[N] == LE) {
                ZZEKVMCH = (REL != GT);
            } else if (OPS[N] == GT) {
                ZZEKVMCH = (REL == GT);
            } else if (OPS[N] == GE) {
                ZZEKVMCH = (REL != LT);
            } else if (OPS[N] == NE) {
                ZZEKVMCH = (REL != EQ);
            } else if ((OPS[N] == LIKE) && (CLDSCS[[TYPIDX, 1]] == CHR)) {
                for I in 1..=2 {
                    ZZEKRSC(
                        HANS[I],
                        SGDSCS.subarray([1, I]),
                        CLDSCS.subarray([1, I]),
                        ROWS[I],
                        ELTS[I],
                        &mut CVLEN[I],
                        &mut CVAL[I],
                        &mut NULL[I],
                        &mut FOUND,
                        ctx,
                    )?;

                    if !FOUND {
                        CHKIN(b"ZZEKVMCH", ctx)?;
                        SETMSG(b"EK = #; COLIDX = #; ROW = #; ELTIDX  = #.  Column entry  element was not found.", ctx);
                        ERRHAN(b"#", HANS[I], ctx)?;
                        ERRINT(b"#", CLDSCS[[ORDIDX, I]], ctx);
                        ERRINT(b"#", ROWS[I], ctx);
                        ERRINT(b"#", ELTS[I], ctx);
                        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
                        CHKOUT(b"ZZEKVMCH", ctx)?;
                        return Ok(ZZEKVMCH);
                    }

                    if (FOUND && !NULL[I]) {
                        CMPLEN[I] = intrinsics::MIN0(&[CVLEN[I], MAXSTR]);
                    } else {
                        CMPLEN[I] = 0;
                    }
                }

                ZZEKVMCH = MATCHI(
                    fstr::substr(&CVAL[1], 1..=CMPLEN[1]),
                    fstr::substr(&CVAL[2], 1..=CMPLEN[2]),
                    WSTR,
                    WCHR,
                    ctx,
                );
            } else if ((OPS[N] == UNLIKE) && (CLDSCS[[TYPIDX, 1]] == CHR)) {
                for I in 1..=2 {
                    ZZEKRSC(
                        HANS[I],
                        SGDSCS.subarray([1, I]),
                        CLDSCS.subarray([1, I]),
                        ROWS[I],
                        ELTS[I],
                        &mut CVLEN[I],
                        &mut CVAL[I],
                        &mut NULL[I],
                        &mut FOUND,
                        ctx,
                    )?;

                    if !FOUND {
                        CHKIN(b"ZZEKVMCH", ctx)?;
                        SETMSG(b"EK = #; COLIDX = #; ROW = #; ELTIDX  = #.  Column entry  element was not found.", ctx);
                        ERRHAN(b"#", HANS[I], ctx)?;
                        ERRINT(b"#", CLDSCS[[ORDIDX, I]], ctx);
                        ERRINT(b"#", ROWS[I], ctx);
                        ERRINT(b"#", ELTS[I], ctx);
                        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
                        CHKOUT(b"ZZEKVMCH", ctx)?;
                        return Ok(ZZEKVMCH);
                    }

                    if (FOUND && !NULL[I]) {
                        CMPLEN[I] = intrinsics::MIN0(&[CVLEN[I], MAXSTR]);
                    } else {
                        CMPLEN[I] = 0;
                    }
                }

                ZZEKVMCH = !MATCHI(
                    fstr::substr(&CVAL[1], 1..=CMPLEN[1]),
                    fstr::substr(&CVAL[2], 1..=CMPLEN[2]),
                    WSTR,
                    WCHR,
                    ctx,
                );
            } else {
                //
                // Sorry, we couldn't resist.
                //
                ZZEKVMCH = false;

                CHKIN(b"ZZEKVMCH", ctx)?;
                SETMSG(b"The relational operator # was not recognized.", ctx);
                ERRINT(b"#", OPS[N], ctx);
                SIGERR(b"SPICE(UNNATURALRELATION)", ctx)?;
                CHKOUT(b"ZZEKVMCH", ctx)?;
                return Ok(ZZEKVMCH);
            }
        }

        //
        // We've completed the test for the Nth constraint, if that
        // constraint was active.
        //
        N = (N + 1);
    }

    Ok(ZZEKVMCH)
}
