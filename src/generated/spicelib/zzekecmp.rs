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

//$Procedure      ZZEKECMP ( EK, column entry element comparison )
pub fn ZZEKECMP(
    HANS: &[i32],
    SGDSCS: &[i32],
    CLDSCS: &[i32],
    ROWS: &[i32],
    ELTS: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<i32> {
    let HANS = DummyArray::new(HANS, 1..);
    let SGDSCS = DummyArray2D::new(SGDSCS, 1..=SDSCSZ, 1..);
    let CLDSCS = DummyArray2D::new(CLDSCS, 1..=CDSCSZ, 1..);
    let ROWS = DummyArray::new(ROWS, 1..);
    let ELTS = DummyArray::new(ELTS, 1..);
    let mut ZZEKECMP: i32 = 0;
    let mut CVAL = ActualCharArray::new(MAXSTR, 1..=2);
    let mut DVAL = StackArray::<f64, 2>::new(1..=2);
    let mut CMPLEN = StackArray::<i32, 2>::new(1..=2);
    let mut CVLEN = StackArray::<i32, 2>::new(1..=2);
    let mut IVAL = StackArray::<i32, 2>::new(1..=2);
    let mut LHSTYP: i32 = 0;
    let mut RHSTYP: i32 = 0;
    let mut FOUND: bool = false;
    let mut NULL = StackArray::<bool, 2>::new(1..=2);

    //
    // Local variables
    //

    //
    // Use discovery check-in for speed.
    //

    //
    // The function value defaults to `equal'.
    //
    ZZEKECMP = EQ;

    LHSTYP = CLDSCS[[TYPIDX, 1]];
    RHSTYP = CLDSCS[[TYPIDX, 2]];

    if (LHSTYP == INT) {
        //
        // The entities we're comparing are supposed to be
        // scalar.  The left hand side has integer type.  Either
        // integer or double precision types are acceptable on
        // the right hand side.
        //
        ZZEKRSI(
            HANS[1],
            SGDSCS.subarray([1, 1]),
            CLDSCS.subarray([1, 1]),
            ROWS[1],
            ELTS[1],
            &mut IVAL[1],
            &mut NULL[1],
            &mut FOUND,
            ctx,
        )?;

        if !FOUND {
            CHKIN(b"ZZEKECMP", ctx)?;
            SETMSG(
                b"EK = #; COLIDX = #; ROW = #; ELTIDX = #. Column entry element was not found.",
                ctx,
            );
            ERRHAN(b"#", HANS[1], ctx)?;
            ERRINT(b"#", CLDSCS[[ORDIDX, 1]], ctx);
            ERRINT(b"#", ROWS[1], ctx);
            ERRINT(b"#", ELTS[1], ctx);
            SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
            CHKOUT(b"ZZEKECMP", ctx)?;
            return Ok(ZZEKECMP);
        }

        if (RHSTYP == INT) {
            ZZEKRSI(
                HANS[2],
                SGDSCS.subarray([1, 2]),
                CLDSCS.subarray([1, 2]),
                ROWS[2],
                ELTS[1],
                &mut IVAL[2],
                &mut NULL[2],
                &mut FOUND,
                ctx,
            )?;

            if !FOUND {
                CHKIN(b"ZZEKECMP", ctx)?;
                SETMSG(
                    b"EK = #; COLIDX = #; ROW = #; ELTIDX  = #.Column entry element was not found.",
                    ctx,
                );
                ERRHAN(b"#", HANS[2], ctx)?;
                ERRINT(b"#", CLDSCS[[ORDIDX, 2]], ctx);
                ERRINT(b"#", ROWS[2], ctx);
                ERRINT(b"#", ELTS[2], ctx);
                SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
                CHKOUT(b"ZZEKECMP", ctx)?;
                return Ok(ZZEKECMP);
            }

            //
            // Null values precede all others.
            //
            if (NULL[1] || NULL[2]) {
                if !NULL[2] {
                    ZZEKECMP = LT;
                } else if !NULL[1] {
                    ZZEKECMP = GT;
                }
            } else {
                if (IVAL[1] < IVAL[2]) {
                    ZZEKECMP = LT;
                } else if (IVAL[1] > IVAL[2]) {
                    ZZEKECMP = GT;
                }
            }
        } else if (RHSTYP == DP) {
            ZZEKRSD(
                HANS[2],
                SGDSCS.subarray([1, 2]),
                CLDSCS.subarray([1, 2]),
                ROWS[2],
                ELTS[1],
                &mut DVAL[2],
                &mut NULL[2],
                &mut FOUND,
                ctx,
            )?;

            if !FOUND {
                CHKIN(b"ZZEKECMP", ctx)?;
                SETMSG(
                    b"EK = #; COLIDX = #; ROW = #; ELTIDX  = #.Column entry element was not found.",
                    ctx,
                );
                ERRHAN(b"#", HANS[2], ctx)?;
                ERRINT(b"#", CLDSCS[[ORDIDX, 2]], ctx);
                ERRINT(b"#", ROWS[2], ctx);
                ERRINT(b"#", ELTS[2], ctx);
                SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
                CHKOUT(b"ZZEKECMP", ctx)?;
                return Ok(ZZEKECMP);
            }

            if (NULL[1] || NULL[2]) {
                if !NULL[2] {
                    ZZEKECMP = LT;
                } else if !NULL[1] {
                    ZZEKECMP = GT;
                }
            } else {
                if ((IVAL[1] as f64) < DVAL[2]) {
                    ZZEKECMP = LT;
                } else if ((IVAL[1] as f64) > DVAL[2]) {
                    ZZEKECMP = GT;
                }
            }
        } else {
            //
            // This is a big-time semantic error.  We should
            // never arrive here.
            //
            CHKIN(b"ZZEKECMP", ctx)?;
            SETMSG(b"LHS data type is #; RHSTYP is #.", ctx);
            ERRINT(b"#", LHSTYP, ctx);
            ERRINT(b"#", RHSTYP, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZEKECMP", ctx)?;
            return Ok(ZZEKECMP);
        }
    } else if (LHSTYP == DP) {
        //
        // This is a mirror image of the INT case.
        //
        ZZEKRSD(
            HANS[1],
            SGDSCS.subarray([1, 1]),
            CLDSCS.subarray([1, 1]),
            ROWS[1],
            ELTS[1],
            &mut DVAL[1],
            &mut NULL[1],
            &mut FOUND,
            ctx,
        )?;

        if !FOUND {
            CHKIN(b"ZZEKECMP", ctx)?;
            SETMSG(
                b"EK = #; COLIDX = #; ROW = #; ELTIDX = #. Column entry element was not found.",
                ctx,
            );
            ERRHAN(b"#", HANS[1], ctx)?;
            ERRINT(b"#", CLDSCS[[ORDIDX, 1]], ctx);
            ERRINT(b"#", ROWS[1], ctx);
            ERRINT(b"#", ELTS[1], ctx);
            SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
            CHKOUT(b"ZZEKECMP", ctx)?;
            return Ok(ZZEKECMP);
        }

        if (RHSTYP == INT) {
            ZZEKRSI(
                HANS[2],
                SGDSCS.subarray([1, 2]),
                CLDSCS.subarray([1, 2]),
                ROWS[2],
                ELTS[1],
                &mut IVAL[2],
                &mut NULL[2],
                &mut FOUND,
                ctx,
            )?;

            if !FOUND {
                CHKIN(b"ZZEKECMP", ctx)?;
                SETMSG(
                    b"EK = #; COLIDX = #; ROW = #; ELTIDX  = #.Column entry element was not found.",
                    ctx,
                );
                ERRHAN(b"#", HANS[2], ctx)?;
                ERRINT(b"#", CLDSCS[[ORDIDX, 2]], ctx);
                ERRINT(b"#", ROWS[2], ctx);
                ERRINT(b"#", ELTS[2], ctx);
                SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
                CHKOUT(b"ZZEKECMP", ctx)?;
                return Ok(ZZEKECMP);
            }

            //
            // Null values precede all others.
            //
            if (NULL[1] || NULL[2]) {
                if !NULL[2] {
                    ZZEKECMP = LT;
                } else if !NULL[1] {
                    ZZEKECMP = GT;
                }
            } else {
                if (DVAL[1] < IVAL[2] as f64) {
                    ZZEKECMP = LT;
                } else if (DVAL[1] > IVAL[2] as f64) {
                    ZZEKECMP = GT;
                }
            }
        } else if (RHSTYP == DP) {
            ZZEKRSD(
                HANS[2],
                SGDSCS.subarray([1, 2]),
                CLDSCS.subarray([1, 2]),
                ROWS[2],
                ELTS[1],
                &mut DVAL[2],
                &mut NULL[2],
                &mut FOUND,
                ctx,
            )?;

            if !FOUND {
                CHKIN(b"ZZEKECMP", ctx)?;
                SETMSG(
                    b"EK = #; COLIDX = #; ROW = #; ELTIDX  = #.Column entry element was not found.",
                    ctx,
                );
                ERRHAN(b"#", HANS[2], ctx)?;
                ERRINT(b"#", CLDSCS[[ORDIDX, 2]], ctx);
                ERRINT(b"#", ROWS[2], ctx);
                ERRINT(b"#", ELTS[2], ctx);
                SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
                CHKOUT(b"ZZEKECMP", ctx)?;
                return Ok(ZZEKECMP);
            }

            if (NULL[1] || NULL[2]) {
                if !NULL[2] {
                    ZZEKECMP = LT;
                } else if !NULL[1] {
                    ZZEKECMP = GT;
                }
            } else {
                if (DVAL[1] < DVAL[2]) {
                    ZZEKECMP = LT;
                } else if (DVAL[1] > DVAL[2]) {
                    ZZEKECMP = GT;
                }
            }
        } else {
            //
            // This is a big-time semantic error.  We should
            // never arrive here.
            //
            CHKIN(b"ZZEKECMP", ctx)?;
            SETMSG(b"LHS data type is #; RHSTYP is #.", ctx);
            ERRINT(b"#", LHSTYP, ctx);
            ERRINT(b"#", RHSTYP, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZEKECMP", ctx)?;
            return Ok(ZZEKECMP);
        }
    } else if (LHSTYP == TIME) {
        //
        // The entities we're comparing are supposed to be time values.
        //
        if (RHSTYP != TIME) {
            //
            // This is a big-time semantic error.  We should
            // never arrive here.
            //
            CHKIN(b"ZZEKECMP", ctx)?;
            SETMSG(b"LHS data type is #; RHSTYP is #.", ctx);
            ERRINT(b"#", LHSTYP, ctx);
            ERRINT(b"#", RHSTYP, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZEKECMP", ctx)?;
            return Ok(ZZEKECMP);
        }

        for I in 1..=2 {
            ZZEKRSD(
                HANS[I],
                SGDSCS.subarray([1, I]),
                CLDSCS.subarray([1, I]),
                ROWS[I],
                ELTS[I],
                &mut DVAL[I],
                &mut NULL[I],
                &mut FOUND,
                ctx,
            )?;

            if !FOUND {
                CHKIN(b"ZZEKECMP", ctx)?;
                SETMSG(
                    b"EK = #; COLIDX = #; ROW = #; ELTIDX  = #.Column entry element was not found.",
                    ctx,
                );
                ERRHAN(b"#", HANS[I], ctx)?;
                ERRINT(b"#", CLDSCS[[ORDIDX, I]], ctx);
                ERRINT(b"#", ROWS[I], ctx);
                ERRINT(b"#", ELTS[I], ctx);
                SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
                CHKOUT(b"ZZEKECMP", ctx)?;
                return Ok(ZZEKECMP);
            }
        }

        if (NULL[1] || NULL[2]) {
            if !NULL[2] {
                ZZEKECMP = LT;
            } else if !NULL[1] {
                ZZEKECMP = GT;
            }
        } else {
            if (DVAL[1] < DVAL[2]) {
                ZZEKECMP = LT;
            } else if (DVAL[1] > DVAL[2]) {
                ZZEKECMP = GT;
            }
        }
    } else if (LHSTYP == CHR) {
        //
        // The entities we're comparing are supposed to be scalar.
        //
        if (RHSTYP != CHR) {
            //
            // You know what kind of semantic error this is.
            //
            CHKIN(b"ZZEKECMP", ctx)?;
            SETMSG(b"LHS data type is #; RHSTYP is #.", ctx);
            ERRINT(b"#", LHSTYP, ctx);
            ERRINT(b"#", RHSTYP, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZEKECMP", ctx)?;
            return Ok(ZZEKECMP);
        }

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
                CHKIN(b"ZZEKECMP", ctx)?;
                SETMSG(
                    b"EK = #; COLIDX = #; ROW = #; ELTIDX  = #.Column entry element was not found.",
                    ctx,
                );
                ERRHAN(b"#", HANS[I], ctx)?;
                ERRINT(b"#", CLDSCS[[ORDIDX, I]], ctx);
                ERRINT(b"#", ROWS[I], ctx);
                ERRINT(b"#", ELTS[I], ctx);
                SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
                CHKOUT(b"ZZEKECMP", ctx)?;
                return Ok(ZZEKECMP);
            }

            //
            // Let CMPLEN(I) be the string length to use in comparisons.
            //
            CMPLEN[I] = intrinsics::MIN0(&[CVLEN[I], MAXSTR]);
        }

        if (NULL[1] || NULL[2]) {
            if !NULL[2] {
                ZZEKECMP = LT;
            } else if !NULL[1] {
                ZZEKECMP = GT;
            }
        } else {
            if fstr::lt(
                fstr::substr(&CVAL[1], 1..=CMPLEN[1]),
                fstr::substr(&CVAL[2], 1..=CMPLEN[2]),
            ) {
                ZZEKECMP = LT;
            } else if fstr::gt(
                fstr::substr(&CVAL[1], 1..=CMPLEN[1]),
                fstr::substr(&CVAL[2], 1..=CMPLEN[2]),
            ) {
                ZZEKECMP = GT;
            } else {
                ZZEKECMP = EQ;
            }
        }
    } else {
        //
        // Something untoward has happened in our descriptor.
        //
        CHKIN(b"ZZEKECMP", ctx)?;
        SETMSG(b"The data type code # was not recognized.", ctx);
        ERRINT(b"#", LHSTYP, ctx);
        SIGERR(b"SPICE(INVALIDDATATYPE)", ctx)?;
        CHKOUT(b"ZZEKECMP", ctx)?;
        return Ok(ZZEKECMP);
    }

    Ok(ZZEKECMP)
}
