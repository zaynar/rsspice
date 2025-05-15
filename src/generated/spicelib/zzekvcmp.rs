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
const EQARCH: i32 = 2;
const EQINIT: i32 = (EQARCH + 1);
const EQPARS: i32 = (EQINIT + 1);
const EQNRES: i32 = (EQPARS + 1);
const EQTRES: i32 = (EQNRES + 1);
const EQSCHK: i32 = (EQTRES + 1);
const EQNTAB: i32 = (EQSCHK + 1);
const EQNCNS: i32 = (EQNTAB + 1);
const EQMXML: i32 = -1;
const EQNCNJ: i32 = (EQNCNS + 1);
const EQNORD: i32 = (EQNCNJ + 1);
const EQNSEL: i32 = (EQNORD + 1);
const EQNSIZ: i32 = (EQNSEL + 1);
const EQNPTR: i32 = (EQNSIZ + 1);
const EQCSIZ: i32 = (EQNPTR + 1);
const EQCPTR: i32 = (EQCSIZ + 1);
const EQBSEL: i32 = (EQCPTR + 1);
const EQBCON: i32 = (EQBSEL + 1);
const EQBCNJ: i32 = (EQBCON + 1);
const EQBORD: i32 = (EQBCON + 1);
const EQVBAS: i32 = EQBORD;
const EQDTYP: i32 = 1;
const EQBLEX: i32 = (EQDTYP + 1);
const EQELEX: i32 = (EQBLEX + 1);
const EQBSTR: i32 = (EQELEX + 1);
const EQESTR: i32 = (EQBSTR + 1);
const EQVPTR: i32 = (EQELEX + 1);
const EQVDSZ: i32 = 6;
const EQBCOL: i32 = 1;
const EQCIDX: i32 = EQVDSZ;
const EQBTAB: i32 = 1;
const EQTORD: i32 = EQVDSZ;
const EQCTYP: i32 = 1;
const EQCOL: i32 = 1;
const EQVAL: i32 = 2;
const EQLTAB: i32 = (EQCTYP + 1);
const EQLCOL: i32 = (EQLTAB + EQVDSZ);
const EQOPCD: i32 = (EQLCOL + EQVDSZ);
const EQRTAB: i32 = (EQOPCD + 1);
const EQRCOL: i32 = (EQRTAB + EQVDSZ);
const EQBVAL: i32 = (EQOPCD + 1);
const EQCDSZ: i32 = (2 + (4 * EQVDSZ));
const EQOTAB: i32 = 1;
const EQOCOL: i32 = (EQOTAB + EQVDSZ);
const EQODIR: i32 = (EQOCOL + EQVDSZ);
const EQODSZ: i32 = (1 + (2 * EQVDSZ));
const EQASND: i32 = 0;
const EQDSND: i32 = 1;
const EQSTAB: i32 = 1;
const EQSCOL: i32 = (EQSTAB + EQVDSZ);
const EQSDSZ: i32 = (2 * EQVDSZ);
const EQIMIN: i32 =
    (((((EQVBAS + ((10 * EQVDSZ) * 2)) + (1000 * EQCDSZ)) + 1000) + (10 * EQODSZ)) + (50 * EQSDSZ));
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
const LBPOOL: i32 = -5;

//$Procedure      ZZEKVCMP ( EK, row vector comparison )
pub fn ZZEKVCMP(
    OP: i32,
    NCOLS: i32,
    TABS: &[i32],
    COLS: &[i32],
    ELTS: &[i32],
    SENSES: &[i32],
    STHAN: &[i32],
    STSDSC: &[i32],
    STDTPT: &[i32],
    DTPOOL: &[i32],
    DTDSCS: &[i32],
    SGVEC1: &[i32],
    RWVEC1: &[i32],
    SGVEC2: &[i32],
    RWVEC2: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let TABS = DummyArray::new(TABS, 1..);
    let COLS = DummyArray::new(COLS, 1..);
    let ELTS = DummyArray::new(ELTS, 1..);
    let SENSES = DummyArray::new(SENSES, 1..);
    let STHAN = DummyArray::new(STHAN, 1..);
    let STSDSC = DummyArray2D::new(STSDSC, 1..=SDSCSZ, 1..);
    let STDTPT = DummyArray::new(STDTPT, 1..);
    let DTPOOL = DummyArray2D::new(DTPOOL, 1..=2, LBPOOL..);
    let DTDSCS = DummyArray2D::new(DTDSCS, 1..=CDSCSZ, 1..);
    let SGVEC1 = DummyArray::new(SGVEC1, 1..);
    let RWVEC1 = DummyArray::new(RWVEC1, 1..);
    let SGVEC2 = DummyArray::new(SGVEC2, 1..);
    let RWVEC2 = DummyArray::new(RWVEC2, 1..);
    let mut ZZEKVCMP: bool = false;
    let mut CLDSCS = StackArray2D::<i32, 22>::new(1..=CDSCSZ, 1..=2);
    let mut COL: i32 = 0;
    let mut COLIDX: i32 = 0;
    let mut COLPTR = StackArray::<i32, 2>::new(1..=2);
    let mut DTYPE = StackArray::<i32, 2>::new(1..=2);
    let mut ELIDXS = StackArray::<i32, 2>::new(1..=2);
    let mut HANS = StackArray::<i32, 2>::new(1..=2);
    let mut REL: i32 = 0;
    let mut ROWS = StackArray::<i32, 2>::new(1..=2);
    let mut SEGS = StackArray::<i32, 2>::new(1..=2);
    let mut SGDSCS = StackArray2D::<i32, 48>::new(1..=SDSCSZ, 1..=2);
    let mut TABIDX: i32 = 0;

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
    // Use discovery check-in for speed.
    //

    //
    // The function value defaults to .FALSE.
    //
    ZZEKVCMP = false;

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
    REL = EQ;
    COL = 1;

    while ((COL <= NCOLS) && (REL == EQ)) {
        //
        // Compare the entries in the two rows in the columns indicated
        // by the Nth column descriptor pair.
        //
        TABIDX = TABS[COL];
        COLIDX = COLS[COL];

        SEGS[1] = SGVEC1[TABIDX];
        SEGS[2] = SGVEC2[TABIDX];
        ROWS[1] = RWVEC1[TABIDX];
        ROWS[2] = RWVEC2[TABIDX];

        //
        // Identify the handles, segment descriptors, and column
        // descriptors we'll use to apply the constraint having index
        // COL.
        //
        for I in 1..=2 {
            HANS[I] = STHAN[SEGS[I]];
            COLPTR[I] = STDTPT[SEGS[I]];

            for J in 2..=COLIDX {
                COLPTR[I] = LNKNXT(COLPTR[I], DTPOOL.as_slice(), ctx)?;
            }

            MOVEI(
                DTDSCS.subarray([1, COLPTR[I]]),
                CDSCSZ,
                CLDSCS.subarray_mut([1, I]),
            );
            MOVEI(
                STSDSC.subarray([1, SEGS[I]]),
                SDSCSZ,
                SGDSCS.subarray_mut([1, I]),
            );

            DTYPE[I] = DTDSCS[[TYPIDX, COLPTR[I]]];
            ELIDXS[I] = ELTS[COL];
        }

        if (DTYPE[1] == DTYPE[2]) {
            //
            // Find the order of the rows according to the order-by
            // column having index COL.  If the order sense for this
            // column is descending, adjust REL to reflect this.
            //
            REL = ZZEKECMP(
                HANS.as_slice(),
                SGDSCS.as_slice(),
                CLDSCS.as_slice(),
                ROWS.as_slice(),
                ELIDXS.as_slice(),
                ctx,
            )?;

            if (SENSES[COL] == EQDSND) {
                if (REL == LT) {
                    REL = GT;
                } else if (REL == GT) {
                    REL = LT;
                }
            }
        } else {
            CHKIN(b"ZZEKVCMP", ctx)?;
            SETMSG(b"Data type mismatch for order-by column having index #; type for segment # = #; type for segment # is #", ctx);
            ERRINT(b"#", COL, ctx);
            ERRINT(b"#", SEGS[1], ctx);
            ERRINT(b"#", DTYPE[1], ctx);
            ERRINT(b"#", SEGS[2], ctx);
            ERRINT(b"#", DTYPE[2], ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZEKVCMP", ctx)?;
            return Ok(ZZEKVCMP);
        }

        COL = (COL + 1);
    }

    //
    // Determine the truth of the input relational expression.
    //
    if (OP == EQ) {
        ZZEKVCMP = (REL == EQ);
    } else if (OP == LT) {
        ZZEKVCMP = (REL == LT);
    } else if (OP == LE) {
        ZZEKVCMP = (REL != GT);
    } else if (OP == GT) {
        ZZEKVCMP = (REL == GT);
    } else if (OP == GE) {
        ZZEKVCMP = (REL != LT);
    } else if (OP == NE) {
        ZZEKVCMP = (REL != EQ);
    } else {
        //
        // Sorry, we couldn't resist.
        //
        ZZEKVCMP = false;

        CHKIN(b"ZZEKVCMP", ctx)?;
        SETMSG(b"The relational operator # was not recognized.", ctx);
        ERRINT(b"#", OP, ctx);
        SIGERR(b"SPICE(UNNATURALRELATION)", ctx)?;
        CHKOUT(b"ZZEKVCMP", ctx)?;
        return Ok(ZZEKVCMP);
    }

    Ok(ZZEKVCMP)
}
