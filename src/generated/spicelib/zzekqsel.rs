//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
const LBCELL: i32 = -5;

//$Procedure  ZZEKQSEL ( Private: EK, read SELECT columns from query )
pub fn ZZEKQSEL(
    EQRYI: &[i32],
    EQRYC: &[u8],
    N: i32,
    LXBEG: &mut i32,
    LXEND: &mut i32,
    TABLE: &mut [u8],
    TABIDX: &mut i32,
    COLUMN: &mut [u8],
    COLIDX: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let EQRYI = DummyArray::new(EQRYI, LBCELL..);
    let mut BASE: i32 = 0;
    let mut BUFLEN: i32 = 0;
    let mut CB: i32 = 0;
    let mut CE: i32 = 0;
    let mut IPARSE: i32 = 0;
    let mut NCNJ: i32 = 0;
    let mut NCNS: i32 = 0;
    let mut NORD: i32 = 0;
    let mut NSEL: i32 = 0;
    let mut NTAB: i32 = 0;
    let mut RESOLV: i32 = 0;
    let mut TB: i32 = 0;
    let mut TE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //

    ZZEKREQI(EQRYI.as_slice(), b"PARSED", &mut IPARSE, ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    if (IPARSE == IFALSE) {
        CHKIN(b"ZZEKQSEL", ctx)?;
        SETMSG(b"Encoded query has not yet been parsed.", ctx);
        SIGERR(b"SPICE(UNPARSEDQUERY)", ctx)?;
        CHKOUT(b"ZZEKQSEL", ctx)?;
        return Ok(());
    }

    ZZEKREQI(EQRYI.as_slice(), b"NUM_SELECT_COLS", &mut NSEL, ctx)?;

    if ((N < 1) || (N > NSEL)) {
        CHKIN(b"ZZEKQSEL", ctx)?;
        SETMSG(b"Column index # is out of valid range 1:#.", ctx);
        ERRINT(b"#", N, ctx);
        ERRINT(b"#", NSEL, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKQSEL", ctx)?;
        return Ok(());
    }

    ZZEKREQI(EQRYI.as_slice(), b"NUM_TABLES", &mut NTAB, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_CONJUNCTIONS", &mut NCNJ, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_CONSTRAINTS", &mut NCNS, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_ORDERBY_COLS", &mut NORD, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"CHR_BUF_SIZE", &mut BUFLEN, ctx)?;

    //
    // The lexeme begin and end values start out as invalid values.
    // We'll set these when we discover what form the SELECT item has.
    //

    //
    *LXBEG = 0;
    *LXEND = 0;

    //
    // Get the Nth table and column from the query.  The table
    // descriptor lies beyond the fixed-size portion of the query, the
    // conjunction size list, the constraint descriptors, the order-by
    // column descriptors, as well as the (N-1) previous SELECT column
    // descriptors.
    //
    BASE = (((((EQVBAS + ((NTAB * 2) * EQVDSZ)) + NCNJ) + (NCNS * EQCDSZ)) + (NORD * EQODSZ))
        + ((N - 1) * EQSDSZ));

    //
    // Pick up the column name first.
    //
    CB = EQRYI[((BASE + EQVDSZ) + EQBSTR)];
    CE = EQRYI[((BASE + EQVDSZ) + EQESTR)];

    if (((((CB > 0) && (CE > 0)) && (CB <= BUFLEN)) && (CE <= BUFLEN)) && (CB <= CE)) {
        fstr::assign(COLUMN, fstr::substr(EQRYC, CB..=CE));

        //
        // The end of the column name is always the end of the SELECT
        // item, at least until we handle more general expressions.
        //
        *LXEND = EQRYI[((BASE + EQVDSZ) + EQELEX)];
    } else {
        //
        // We should never see invalid pointers in a parsed, encoded
        // query, but let's not take chances.
        //
        CHKIN(b"ZZEKQSEL", ctx)?;
        SETMSG(b"Invalid string bounds #:# for column #.", ctx);
        ERRINT(b"#", CB, ctx);
        ERRINT(b"#", CE, ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKQSEL", ctx)?;
        return Ok(());
    }

    //
    // Same deal for the qualifying table or alias, except that the begin
    // pointer is set to zero if there's no name.
    //
    TB = EQRYI[(BASE + EQBSTR)];
    TE = EQRYI[(BASE + EQESTR)];

    if (TB > 0) {
        if ((((TE > 0) && (TB <= BUFLEN)) && (TE <= BUFLEN)) && (TB <= TE)) {
            fstr::assign(TABLE, fstr::substr(EQRYC, TB..=TE));

            //
            // The start position of the table name is the start of
            // the SELECT item.
            //
            *LXBEG = EQRYI[(BASE + EQBLEX)];
        } else {
            //
            // If the first pointer is non-zero, both pointers should have
            // been valid.
            //
            CHKIN(b"ZZEKQSEL", ctx)?;
            SETMSG(
                b"Invalid string bounds #:# for the table qualifying column #.",
                ctx,
            );
            ERRINT(b"#", TB, ctx);
            ERRINT(b"#", TE, ctx);
            ERRINT(b"#", N, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZEKQSEL", ctx)?;
            return Ok(());
        }
    } else {
        //
        // No table was supplied.
        //
        fstr::assign(TABLE, b" ");

        //
        // The start position of the column name is the start of
        // the SELECT item.
        //
        *LXBEG = EQRYI[((BASE + EQVDSZ) + EQBLEX)];
    }

    //
    // If names have been resolved already, we can determine the index
    // of the table to which the specified order-by column belongs.
    //
    ZZEKREQI(EQRYI.as_slice(), b"NAMES_RESOLVED", &mut RESOLV, ctx)?;

    if (RESOLV == ITRUE) {
        //
        // The qualifying table's index in the FROM clause is available.
        // So is the index of the column within the table.
        //
        *TABIDX = EQRYI[(BASE + EQTORD)];
        *COLIDX = EQRYI[(((BASE + EQSCOL) - 1) + EQCIDX)];
    } else {
        *TABIDX = 0;
        *COLIDX = 0;
    }

    Ok(())
}
