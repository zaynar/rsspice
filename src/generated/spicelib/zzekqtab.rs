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

//$Procedure  ZZEKQTAB ( Private: EK, read table names from query )
pub fn ZZEKQTAB(
    EQRYI: &[i32],
    EQRYC: &[u8],
    N: i32,
    TABLE: &mut [u8],
    ALIAS: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let EQRYI = DummyArray::new(EQRYI, LBCELL..);
    let mut AB: i32 = 0;
    let mut AE: i32 = 0;
    let mut BASE: i32 = 0;
    let mut BUFLEN: i32 = 0;
    let mut IPARSE: i32 = 0;
    let mut NTAB: i32 = 0;
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
        CHKIN(b"ZZEKQTAB", ctx)?;
        SETMSG(b"Encoded query has not yet been parsed.", ctx);
        SIGERR(b"SPICE(UNPARSEDQUERY)", ctx)?;
        CHKOUT(b"ZZEKQTAB", ctx)?;
        return Ok(());
    }

    ZZEKREQI(EQRYI.as_slice(), b"CHR_BUF_SIZE", &mut BUFLEN, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_TABLES", &mut NTAB, ctx)?;

    if ((N < 1) || (N > NTAB)) {
        CHKIN(b"ZZEKQTAB", ctx)?;
        SETMSG(b"Table index # is out of valid range 1:#.", ctx);
        ERRINT(b"#", N, ctx);
        ERRINT(b"#", NTAB, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKQTAB", ctx)?;
        return Ok(());
    }

    //
    // Get the Nth table and alias from the query.  The table
    // descriptor lies beyond the fixed-size portion of the query, as
    // well as the (N-1) previous descriptors, each one of which has
    // size 2*EQVDSZ.
    //
    BASE = (EQVBAS + (((N - 1) * 2) * EQVDSZ));

    TB = EQRYI[(BASE + EQBSTR)];
    TE = EQRYI[(BASE + EQESTR)];

    if (((((TB > 0) && (TE > 0)) && (TB <= BUFLEN)) && (TE <= BUFLEN)) && (TB <= TE)) {
        fstr::assign(TABLE, fstr::substr(EQRYC, TB..=TE));
    } else {
        //
        // We should never see invalid pointers in a parsed, encoded
        // query, but let's not take chances.
        //
        CHKIN(b"ZZEKQTAB", ctx)?;
        SETMSG(b"Invalid string bounds #:# for table #.", ctx);
        ERRINT(b"#", TB, ctx);
        ERRINT(b"#", TE, ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKQTAB", ctx)?;
        return Ok(());
    }

    //
    // Same deal for the alias, except that the begin pointer is
    // set to zero if there's no alias.
    //
    AB = EQRYI[((BASE + EQVDSZ) + EQBSTR)];
    AE = EQRYI[((BASE + EQVDSZ) + EQESTR)];

    if (AB > 0) {
        if ((((AE > 0) && (AB <= BUFLEN)) && (AE <= BUFLEN)) && (AB <= AE)) {
            fstr::assign(ALIAS, fstr::substr(EQRYC, AB..=AE));
        } else {
            //
            // If the first pointer is non-zero, both pointers should have
            // been valid.
            //
            CHKIN(b"ZZEKQTAB", ctx)?;
            SETMSG(b"Invalid string bounds #:# for the alias of table #.", ctx);
            ERRINT(b"#", AB, ctx);
            ERRINT(b"#", AE, ctx);
            ERRINT(b"#", N, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZEKQTAB", ctx)?;
            return Ok(());
        }
    } else {
        //
        // No alias was supplied.
        //
        fstr::assign(ALIAS, b" ");
    }

    Ok(())
}
