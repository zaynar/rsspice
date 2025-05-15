//
// GENERATED FILE
//

use super::*;
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
const LBCELL: i32 = -5;

//$Procedure  ZZEKKEY  ( EK, determine key column )
pub fn ZZEKKEY(
    HANDLE: i32,
    SEGDSC: &[i32],
    NROWS: i32,
    NCNSTR: i32,
    CLIDXS: &[i32],
    DSCLST: &[i32],
    OPS: &[i32],
    DTYPES: &[i32],
    CHRBUF: &[u8],
    CBEGS: &[i32],
    CENDS: &[i32],
    DVALS: &[f64],
    IVALS: &[i32],
    ACTIVE: &mut [bool],
    KEY: &mut i32,
    KEYDSC: &mut [i32],
    BEGIDX: &mut i32,
    ENDIDX: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let CLIDXS = DummyArray::new(CLIDXS, 1..);
    let DSCLST = DummyArray2D::new(DSCLST, 1..=CDSCSZ, 1..);
    let OPS = DummyArray::new(OPS, 1..);
    let DTYPES = DummyArray::new(DTYPES, 1..);
    let CBEGS = DummyArray::new(CBEGS, 1..);
    let CENDS = DummyArray::new(CENDS, 1..);
    let DVALS = DummyArray::new(DVALS, 1..);
    let IVALS = DummyArray::new(IVALS, 1..);
    let mut ACTIVE = DummyArrayMut::new(ACTIVE, 1..);
    let mut KEYDSC = DummyArrayMut::new(KEYDSC, 1..);
    let mut B: i32 = 0;
    let mut BEST: i32 = 0;
    let mut COL: i32 = 0;
    let mut CONMAP = ActualArray::<i32>::new(1..=MAXCON);
    let mut DTYPE: i32 = 0;
    let mut E: i32 = 0;
    let mut ELTIDX: i32 = 0;
    let mut I: i32 = 0;
    let mut IDXSET = ActualArray::<i32>::new(LBCELL..=MAXCON);
    let mut J: i32 = 0;
    let mut LASTLE: i32 = 0;
    let mut LASTLT: i32 = 0;
    let mut MAXPTR: i32 = 0;
    let mut MINPTR: i32 = 0;
    let mut NMATCH: i32 = 0;
    let mut ELIM: bool = false;
    let mut FND: bool = false;
    let mut INDEXD: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Local parameters
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
        CHKIN(b"ZZEKKEY", ctx)?;
    }

    //
    // There's no key column to begin with.
    //
    *FOUND = false;

    if ((NCNSTR < 0) || (NCNSTR > MAXCON)) {
        SETMSG(b"The number of constraints was #; valid range is 0:#", ctx);
        ERRINT(b"#", NCNSTR, ctx);
        ERRINT(b"#", MAXCON, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKKEY", ctx)?;
        return Ok(());
    }

    //
    // Make a set out of the indices of indexed columns referenced
    // in active constraints.  Maintain a mapping from each column
    // to the index of some constraint that references that column.
    //
    SSIZEI(MAXCON, IDXSET.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NCNSTR;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if ACTIVE[I] {
                INDEXD = (DSCLST[[IXTIDX, I]] != IFALSE);

                if INDEXD {
                    INSRTI(CLIDXS[I], IDXSET.as_slice_mut(), ctx)?;
                }
            }

            I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = CARDI(IDXSET.as_slice(), ctx)?;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            FND = false;
            J = 1;

            while ((J <= NCNSTR) && !FND) {
                if (ACTIVE[J] && (CLIDXS[J] == IDXSET[I])) {
                    FND = true;
                    CONMAP[I] = J;
                } else {
                    J = (J + 1);
                }
            }

            I += m3__;
        }
    }

    //
    // We finish up now if there are no indexed columns
    // on which there are active constraints.
    //
    if (CARDI(IDXSET.as_slice(), ctx)? == 0) {
        CHKOUT(b"ZZEKKEY", ctx)?;
        return Ok(());
    }

    //
    // For each column in the `indexed' set, find out how many
    // candidate rows we'd have if we picked that column as the key
    // column.  If we find that the constraints on some column eliminate
    // all matching rows, we can stop.
    //
    *BEGIDX = 1;
    *ENDIDX = NROWS;
    BEST = IDXSET[1];
    NMATCH = NROWS;

    ELIM = false;
    ELTIDX = 1;

    while ((ELTIDX <= CARDI(IDXSET.as_slice(), ctx)?) && !ELIM) {
        //
        // Get the attribute list pointer for the current column.
        //
        COL = IDXSET[ELTIDX];

        //
        // Set the initial values of MINPTR, MAXPTR, and NMATCH
        //
        MINPTR = 1;
        MAXPTR = NROWS;
        I = 1;

        while ((I <= NCNSTR) && !ELIM) {
            //
            // For each constraint, increase MINPTR or decrease MAXPTR
            // if the constraint allows us to do so.
            //

            if ((CLIDXS[I] == COL) && ACTIVE[I]) {
                //
                // The Ith constraint is active and applies to this column.
                //
                // If the column has character type, set the bounds of the
                // token on the right hand side of the constraint.
                // Otherwise, set the bounds to default valid values to
                // avoid subscript bounds errors.
                //
                DTYPE = DSCLST[[TYPIDX, I]];

                if (DTYPE == CHR) {
                    B = CBEGS[I];
                    E = CENDS[I];
                } else {
                    B = 1;
                    E = 1;
                }

                //
                // At this point, MINPTR and MAXPTR are in the range
                // 1:NROWS, and MINPTR is less than or equal to MAXPTR.
                //

                if (OPS[I] == LT) {
                    //
                    // Find the index of the pointer to the last row
                    // whose value in this column is less than the
                    // value cited in the Ith constraint.
                    //
                    LASTLT = ZZEKILLT(
                        HANDLE,
                        SEGDSC.as_slice(),
                        DSCLST.subarray([1, I]),
                        NROWS,
                        DTYPES[I],
                        fstr::substr(CHRBUF, B..=E),
                        DVALS[I],
                        IVALS[I],
                        ctx,
                    )?;
                    //
                    // If all column elements were greater than or equal
                    // to the specified value, MAXPTR will be set to zero.
                    //
                    MAXPTR = intrinsics::MIN0(&[LASTLT, MAXPTR]);
                    ELIM = (MAXPTR == 0);
                } else if (OPS[I] == LE) {
                    //
                    // Find the index of the pointer to the last row
                    // whose value in this column is less or equal to
                    // the value cited in the Ith constraint.
                    //
                    LASTLE = ZZEKILLE(
                        HANDLE,
                        SEGDSC.as_slice(),
                        DSCLST.subarray([1, I]),
                        NROWS,
                        DTYPES[I],
                        fstr::substr(CHRBUF, B..=E),
                        DVALS[I],
                        IVALS[I],
                        ctx,
                    )?;

                    MAXPTR = intrinsics::MIN0(&[LASTLE, MAXPTR]);
                    ELIM = (MAXPTR == 0);
                } else if (OPS[I] == EQ) {
                    //
                    // Find both the pointer to the last row whose
                    // value in this column is less than the value cited in
                    // the Ith constraint, and the pointer to the last row
                    // whose value in this column is less than or equal to
                    // the value cited in the Ith constraint.  The
                    // successor of the former pointer, together with
                    // the latter pointer, bound the range of pointers
                    // to possible matching rows.
                    //
                    LASTLT = ZZEKILLT(
                        HANDLE,
                        SEGDSC.as_slice(),
                        DSCLST.subarray([1, I]),
                        NROWS,
                        DTYPES[I],
                        fstr::substr(CHRBUF, B..=E),
                        DVALS[I],
                        IVALS[I],
                        ctx,
                    )?;

                    LASTLE = ZZEKILLE(
                        HANDLE,
                        SEGDSC.as_slice(),
                        DSCLST.subarray([1, I]),
                        NROWS,
                        DTYPES[I],
                        fstr::substr(CHRBUF, B..=E),
                        DVALS[I],
                        IVALS[I],
                        ctx,
                    )?;

                    if (LASTLT < LASTLE) {
                        //
                        // There is at least one row whose value in the
                        // current column matches the value cited in the Ith
                        // constraint, and LASTLE is the index of the pointer
                        // to the last such row.  The successor of LASTLT is
                        // the first pointer to such a row (even if LASTLT is
                        // zero).
                        //
                        MINPTR = intrinsics::MAX0(&[(LASTLT + 1), MINPTR]);
                        MAXPTR = intrinsics::MIN0(&[LASTLE, MAXPTR]);
                    } else {
                        //
                        // No rows match this constraint.
                        //
                        ELIM = true;
                    }
                } else if (OPS[I] == GT) {
                    //
                    // Find the index of the pointer to the last row
                    // whose value in this column is less or equal to
                    // the value cited in the Ith constraint.  The index of
                    // the pointer to the first row satisfying all of the
                    // constraints on this column is the successor of
                    // this pointer or a greater pointer.
                    //
                    LASTLE = ZZEKILLE(
                        HANDLE,
                        SEGDSC.as_slice(),
                        DSCLST.subarray([1, I]),
                        NROWS,
                        DTYPES[I],
                        fstr::substr(CHRBUF, B..=E),
                        DVALS[I],
                        IVALS[I],
                        ctx,
                    )?;

                    MINPTR = intrinsics::MAX0(&[(LASTLE + 1), MINPTR]);
                    ELIM = (LASTLE == NROWS);
                } else if (OPS[I] == GE) {
                    //
                    // Find the index of the pointer to the last row
                    // whose value in this column is less than the
                    // value cited in the Ith constraint.  The index of the
                    // pointer to the first row satisfying all of the
                    // constraints on this column is the successor of
                    // this pointer or a greater pointer.
                    //
                    LASTLT = ZZEKILLT(
                        HANDLE,
                        SEGDSC.as_slice(),
                        DSCLST.subarray([1, I]),
                        NROWS,
                        DTYPES[I],
                        fstr::substr(CHRBUF, B..=E),
                        DVALS[I],
                        IVALS[I],
                        ctx,
                    )?;

                    MINPTR = intrinsics::MAX0(&[(LASTLT + 1), MINPTR]);
                    ELIM = (LASTLT == NROWS);
                }
                //
                // We've checked the Ith constraint to see whether
                // it applied to the current column, and if it did,
                // we adjusted MINPTR and MAXPTR to reflect the
                // constraint.
                //
            }
            //
            // We've applied the Ith constraint, if it was active.
            //
            if (MINPTR > MAXPTR) {
                ELIM = true;
            }

            if !ELIM {
                I = (I + 1);
            }

            if FAILED(ctx) {
                CHKOUT(b"ZZEKKEY", ctx)?;
                return Ok(());
            }
        }

        //
        // We've applied all of active, applicable constraints to column
        // COL.  If these constraints did not eliminate the current
        // segment entirely, save the number of candidate rows we'd have
        // if we kept this column as the key column.
        //
        if !ELIM {
            NMATCH = ((MAXPTR - MINPTR) + 1);

            if (NMATCH < ((*ENDIDX - *BEGIDX) + 1)) {
                //
                // This is our new key column, until a better one comes
                // along.
                //
                BEST = COL;
                *BEGIDX = MINPTR;
                *ENDIDX = MAXPTR;
            }

            ELTIDX = (ELTIDX + 1);
        }
    }

    if ELIM {
        //
        // If the segment was eliminated by constraints on the last column
        // we looked at, set BEGIDX and ENDIDX to indicate the absence of
        // matching rows.
        //
        *KEY = COL;
        *BEGIDX = 1;
        *ENDIDX = 0;
    } else {
        //
        // BEST, BEGIDX, and ENDIDX are set to reflect the key column.
        // Set KEY and grab the descriptor of the key column.
        //
        *KEY = BEST;
    }

    I = CONMAP[ORDI(*KEY, IDXSET.as_slice(), ctx)?];

    MOVEI(DSCLST.subarray([1, I]), CDSCSZ, KEYDSC.as_slice_mut());

    //
    // De-activate constraints on the key column that we've already
    // applied.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NCNSTR;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (ACTIVE[I] && (CLIDXS[I] == *KEY)) {
                if (((((OPS[I] == LT) || (OPS[I] == LE)) || (OPS[I] == EQ)) || (OPS[I] == GE))
                    || (OPS[I] == GT))
                {
                    //
                    // This constraint is met by the candidate rows; we can
                    // turn it off.
                    //
                    ACTIVE[I] = false;
                }
            }

            I += m3__;
        }
    }

    //
    // At this point, we've found a key column.
    //
    *FOUND = true;

    CHKOUT(b"ZZEKKEY", ctx)?;
    Ok(())
}
