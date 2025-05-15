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
const WSTR: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b"*");
const WCHR: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b"%");

//$Procedure      ZZEKSCMP ( EK, scalar value comparison )
pub fn ZZEKSCMP(
    OP: i32,
    HANDLE: i32,
    SEGDSC: &[i32],
    COLDSC: &[i32],
    ROW: i32,
    ELTIDX: i32,
    DTYPE: i32,
    CVAL: &[u8],
    DVAL: f64,
    IVAL: i32,
    NULL: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let COLDSC = DummyArray::new(COLDSC, 1..=CDSCSZ);
    let mut ZZEKSCMP: bool = false;
    let mut ELTC = [b' '; MAXSTR as usize];
    let mut ELTD: f64 = 0.0;
    let mut NUMVAL: f64 = 0.0;
    let mut CMPLEN: i32 = 0;
    let mut CVLEN: i32 = 0;
    let mut COLTYP: i32 = 0;
    let mut ELTI: i32 = 0;
    let mut REL: i32 = 0;
    let mut STRLEN: i32 = 0;
    let mut ENULL: bool = false;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in for speed.
    //

    //
    // The function value defaults to .FALSE.
    //
    ZZEKSCMP = false;

    //
    // Look up the specified column element.
    //
    COLTYP = COLDSC[TYPIDX];

    if (COLTYP == CHR) {
        //
        // We'll use at most the first MAXSTR characters of the input
        // string.
        //
        CVLEN = intrinsics::MIN0(&[intrinsics::LEN(CVAL), MAXSTR]);

        //
        // Fetch the column entry to be compared. Note that ROW
        // is a polymorphic identifier. See ZZEKRSC for details
        // on how ROW is used.
        //
        ZZEKRSC(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            ROW,
            ELTIDX,
            &mut STRLEN,
            &mut ELTC,
            &mut ENULL,
            &mut FOUND,
            ctx,
        )?;

        if FAILED(ctx) {
            //
            // Don't check out here because we haven't checked in.
            //
            return Ok(ZZEKSCMP);
        }

        //
        // Let CMPLEN be the string length to use in comparisons.
        //
        if (FOUND && !ENULL) {
            CMPLEN = intrinsics::MIN0(&[STRLEN, MAXSTR]);
        } else {
            CMPLEN = 0;
        }
    } else if ((COLTYP == DP) || (COLTYP == TIME)) {
        ZZEKRSD(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            ROW,
            ELTIDX,
            &mut ELTD,
            &mut ENULL,
            &mut FOUND,
            ctx,
        )?;
    } else if (COLTYP == INT) {
        ZZEKRSI(
            HANDLE,
            SEGDSC.as_slice(),
            COLDSC.as_slice(),
            ROW,
            ELTIDX,
            &mut ELTI,
            &mut ENULL,
            &mut FOUND,
            ctx,
        )?;
    } else {
        CHKIN(b"ZZEKSCMP", ctx)?;
        SETMSG(b"Data type code # not recognized.", ctx);
        ERRINT(b"#", COLTYP, ctx);
        SIGERR(b"SPICE(INVALIDDATATYPE)", ctx)?;
        CHKOUT(b"ZZEKSCMP", ctx)?;
        return Ok(ZZEKSCMP);
    }

    if !FOUND {
        CHKIN(b"ZZEKSCMP", ctx)?;
        SETMSG(
            b"EK = #; COLIDX = #; ROW = #; ELTIDX = #. Column entry element was not found.",
            ctx,
        );
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", COLDSC[ORDIDX], ctx);
        ERRINT(b"#", ROW, ctx);
        ERRINT(b"#", ELTIDX, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKSCMP", ctx)?;
        return Ok(ZZEKSCMP);
    }

    //
    // Handle the ISNULL and NOTNUL operators, if perchance we see them.
    //
    if (OP == ISNULL) {
        ZZEKSCMP = ENULL;
        return Ok(ZZEKSCMP);
    } else if (OP == NOTNUL) {
        ZZEKSCMP = !ENULL;
        return Ok(ZZEKSCMP);
    }

    //
    // Find the order relation that applies to the input values.
    //
    // Null values precede all others.
    //
    if ENULL {
        if NULL {
            REL = EQ;
        } else {
            REL = LT;
        }
    } else if NULL {
        if ENULL {
            REL = EQ;
        } else {
            REL = GT;
        }
    } else {
        //
        //
        // Compare the value we looked up with the input scalar value.
        //
        if (COLTYP == CHR) {
            if (DTYPE != CHR) {
                CHKIN(b"ZZEKSCMP", ctx)?;
                SETMSG(b"Column type is #; value type is #.", ctx);
                ERRINT(b"#", COLTYP, ctx);
                ERRINT(b"#", DTYPE, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZEKSCMP", ctx)?;
                return Ok(ZZEKSCMP);
            }

            if fstr::lt(
                fstr::substr(&ELTC, 1..=CMPLEN),
                fstr::substr(CVAL, 1..=CVLEN),
            ) {
                REL = LT;
            } else if fstr::gt(
                fstr::substr(&ELTC, 1..=CMPLEN),
                fstr::substr(CVAL, 1..=CVLEN),
            ) {
                REL = GT;
            } else {
                REL = EQ;
            }
        } else if (COLTYP == TIME) {
            if ((DTYPE != TIME) && (DTYPE != DP)) {
                CHKIN(b"ZZEKSCMP", ctx)?;
                SETMSG(b"Column type is #; value type is #.", ctx);
                ERRINT(b"#", COLTYP, ctx);
                ERRINT(b"#", DTYPE, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZEKSCMP", ctx)?;
                return Ok(ZZEKSCMP);
            }

            if (ELTD < DVAL) {
                REL = LT;
            } else if (ELTD > DVAL) {
                REL = GT;
            } else {
                REL = EQ;
            }
        } else if (COLTYP == DP) {
            if (DTYPE == INT) {
                NUMVAL = IVAL as f64;
            } else if ((DTYPE == DP) || (DTYPE == TIME)) {
                NUMVAL = DVAL;
            } else {
                CHKIN(b"ZZEKSCMP", ctx)?;
                SETMSG(b"Column type is #; value type is #.", ctx);
                ERRINT(b"#", COLTYP, ctx);
                ERRINT(b"#", DTYPE, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZEKSCMP", ctx)?;
                return Ok(ZZEKSCMP);
            }

            if (ELTD < NUMVAL) {
                REL = LT;
            } else if (ELTD > NUMVAL) {
                REL = GT;
            } else {
                REL = EQ;
            }
        } else if (COLTYP == INT) {
            if (DTYPE == INT) {
                NUMVAL = IVAL as f64;
            } else if (DTYPE == DP) {
                NUMVAL = DVAL;
            } else {
                CHKIN(b"ZZEKSCMP", ctx)?;
                SETMSG(b"Column type is #; value type is #.", ctx);
                ERRINT(b"#", COLTYP, ctx);
                ERRINT(b"#", DTYPE, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZEKSCMP", ctx)?;
                return Ok(ZZEKSCMP);
            }

            if ((ELTI as f64) < NUMVAL) {
                REL = LT;
            } else if ((ELTI as f64) > NUMVAL) {
                REL = GT;
            } else {
                REL = EQ;
            }
        } else {
            //
            // Something untoward has happened in our column descriptor
            // argument.
            //
            CHKIN(b"ZZEKSCMP", ctx)?;
            SETMSG(b"The data type code # was not recognized.", ctx);
            ERRINT(b"#", COLTYP, ctx);
            SIGERR(b"SPICE(INVALIDDATATYPE)", ctx)?;
            CHKOUT(b"ZZEKSCMP", ctx)?;
            return Ok(ZZEKSCMP);
        }
    }

    //
    // Determine the truth of the input relational expression.
    //
    if (OP == EQ) {
        ZZEKSCMP = (REL == EQ);
    } else if (OP == LT) {
        ZZEKSCMP = (REL == LT);
    } else if (OP == LE) {
        ZZEKSCMP = (REL != GT);
    } else if (OP == GT) {
        ZZEKSCMP = (REL == GT);
    } else if (OP == GE) {
        ZZEKSCMP = (REL != LT);
    } else if (OP == NE) {
        ZZEKSCMP = (REL != EQ);
    } else if ((OP == LIKE) && (DTYPE == CHR)) {
        if (NULL || ENULL) {
            ZZEKSCMP = false;
        } else {
            ZZEKSCMP = MATCHI(
                fstr::substr(&ELTC, 1..=CMPLEN),
                fstr::substr(CVAL, 1..=CVLEN),
                WSTR,
                WCHR,
                ctx,
            );
        }
    } else if ((OP == UNLIKE) && (DTYPE == CHR)) {
        if (NULL || ENULL) {
            ZZEKSCMP = false;
        } else {
            ZZEKSCMP = !MATCHI(
                fstr::substr(&ELTC, 1..=CMPLEN),
                fstr::substr(CVAL, 1..=CVLEN),
                WSTR,
                WCHR,
                ctx,
            );
        }
    } else {
        //
        // Sorry, we couldn't resist.
        //
        CHKIN(b"ZZEKSCMP", ctx)?;
        SETMSG(
            b"The relational operator # was not recognized or was not applicable for data type #.",
            ctx,
        );
        ERRINT(b"#", OP, ctx);
        ERRINT(b"#", DTYPE, ctx);
        SIGERR(b"SPICE(UNNATURALRELATION)", ctx)?;
        CHKOUT(b"ZZEKSCMP", ctx)?;
        return Ok(ZZEKSCMP);
    }

    Ok(ZZEKSCMP)
}
