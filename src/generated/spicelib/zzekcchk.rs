//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ADSCSZ: i32 = 6;
const ATTCLS: i32 = 1;
const ATTTYP: i32 = (ATTCLS + 1);
const ATTLEN: i32 = (ATTTYP + 1);
const ATTSIZ: i32 = (ATTLEN + 1);
const ATTIDX: i32 = (ATTSIZ + 1);
const ATTNFL: i32 = (ATTIDX + 1);
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
const CNAMSZ: i32 = 32;
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
const LBCELL: i32 = -5;

//$Procedure  ZZEKCCHK ( Private: EK, check names in encoded query )
pub fn ZZEKCCHK(
    QUERY: &[u8],
    EQRYI: &mut [i32],
    EQRYC: &[u8],
    NTAB: i32,
    TABLST: CharArray,
    ALSLST: CharArray,
    BASE: i32,
    ERROR: &mut bool,
    ERRMSG: &mut [u8],
    ERRPTR: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut EQRYI = DummyArrayMut::new(EQRYI, LBCELL..);
    let TABLST = DummyCharArray::new(TABLST, None, 1..);
    let ALSLST = DummyCharArray::new(ALSLST, None, 1..);
    let mut COLUMN = [b' '; CNAMSZ as usize];
    let mut CTOUCH = [b' '; 1 as usize];
    let mut ATTDSC = StackArray::<i32, 6>::new(1..=ADSCSZ);
    let mut CB: i32 = 0;
    let mut CC: i32 = 0;
    let mut CE: i32 = 0;
    let mut COLIDX: i32 = 0;
    let mut I: i32 = 0;
    let mut IPARSE: i32 = 0;
    let mut LXB = StackArray::<i32, 2>::new(1..=2);
    let mut NMATCH: i32 = 0;
    let mut TABIDX: i32 = 0;
    let mut TB: i32 = 0;
    let mut TE: i32 = 0;
    let mut FND: bool = false;
    let mut NONAME: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // No error to start with.
    //
    *ERROR = false;
    fstr::assign(ERRMSG, b" ");
    *ERRPTR = 0;

    fstr::assign(&mut CTOUCH, fstr::substr(QUERY, 1..=1));

    //
    // The query must have been parsed at this point, or it's no go.
    //
    ZZEKREQI(EQRYI.as_slice(), b"PARSED", &mut IPARSE, ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    if (IPARSE == IFALSE) {
        CHKIN(b"ZZEKCCHK", ctx)?;
        SETMSG(b"Encoded query has not been parsed.", ctx);
        SIGERR(b"SPICE(QUERYNOTPARSED)", ctx)?;
        CHKOUT(b"ZZEKCCHK", ctx)?;
        return Ok(());
    }

    //
    // Get the name and lexeme pointers for both the table and column.
    // Decide whether a table name was supplied.
    //
    TB = EQRYI[(BASE + EQBSTR)];
    TE = EQRYI[(BASE + EQESTR)];
    LXB[1] = EQRYI[(BASE + EQBLEX)];

    CB = EQRYI[((BASE + EQVDSZ) + EQBSTR)];
    CE = EQRYI[((BASE + EQVDSZ) + EQESTR)];
    LXB[2] = EQRYI[((BASE + EQVDSZ) + EQBLEX)];

    if ((CB <= 0) || (CE <= 0)) {
        CHKIN(b"ZZEKCCHK", ctx)?;
        SETMSG(
            b"Invalid string bounds #:# for column.  Column name descriptor base is #.",
            ctx,
        );
        ERRINT(b"#", CB, ctx);
        ERRINT(b"#", CE, ctx);
        ERRINT(b"#", BASE, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKCCHK", ctx)?;
        return Ok(());
    }

    NONAME = (TB == 0);

    if NONAME {
        //
        // If no table name is present, search for the LHS column among
        // the tables in the FROM clause.  If exactly one table
        // contains the column, that table is considered to be the
        // qualifying table.  Otherwise, the qualification is in error.
        //
        NMATCH = 0;

        {
            let m1__: i32 = 1;
            let m2__: i32 = NTAB;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                EKCCNT(&TABLST[I], &mut CC, ctx)?;

                for J in 1..=CC {
                    EKCII(&TABLST[I], J, &mut COLUMN, ATTDSC.as_slice_mut(), ctx)?;

                    if fstr::eq(fstr::substr(EQRYC, CB..=CE), &COLUMN) {
                        NMATCH = (NMATCH + 1);
                        COLIDX = J;
                        TABIDX = I;
                    }
                }

                I += m3__;
            }
        }

        if (NMATCH == 0) {
            *ERROR = true;
            fstr::assign(
                ERRMSG,
                b"Column <#> is not present in any table in FROM clause.",
            );

            REPMC(&ERRMSG.to_vec(), b"#", fstr::substr(EQRYC, CB..=CE), ERRMSG);

            *ERRPTR = LXB[2];
            return Ok(());
        } else if (NMATCH > 1) {
            *ERROR = true;
            fstr::assign(
                ERRMSG,
                b"Column name <#> is ambiguous; a qualifying table name or alias is required.",
            );

            REPMC(&ERRMSG.to_vec(), b"#", fstr::substr(EQRYC, CB..=CE), ERRMSG);

            *ERRPTR = LXB[2];
            return Ok(());
        }
    } else {
        //
        // Find the qualifying name in the FROM table list.  If the
        // name is not there, look in the alias list.
        //
        TABIDX = ISRCHC(fstr::substr(EQRYC, TB..=TE), NTAB, TABLST.as_arg());

        if (TABIDX == 0) {
            TABIDX = ISRCHC(fstr::substr(EQRYC, TB..=TE), NTAB, ALSLST.as_arg());
        }

        //
        // If the table name wasn't in either list, we can't use it.
        //
        if (TABIDX == 0) {
            *ERROR = true;
            fstr::assign(ERRMSG, b"Table name <#> is not present in FROM clause.");

            REPMC(&ERRMSG.to_vec(), b"#", fstr::substr(EQRYC, TB..=TE), ERRMSG);

            *ERRPTR = LXB[1];
            return Ok(());
        }

        //
        // Check the column.  This column must be present in the
        // table that qualifies it.
        //
        EKCCNT(&TABLST[TABIDX], &mut CC, ctx)?;

        FND = false;
        I = 1;

        while ((I <= CC) && !FND) {
            EKCII(&TABLST[TABIDX], I, &mut COLUMN, ATTDSC.as_slice_mut(), ctx)?;

            if fstr::eq(fstr::substr(EQRYC, CB..=CE), &COLUMN) {
                FND = true;
                COLIDX = I;
            } else {
                I = (I + 1);
            }
        }

        if !FND {
            *ERROR = true;
            fstr::assign(ERRMSG, b"Column <#> does not exist in table <#>.");

            REPMC(&ERRMSG.to_vec(), b"#", fstr::substr(EQRYC, CB..=CE), ERRMSG);
            REPMC(&ERRMSG.to_vec(), b"#", fstr::substr(EQRYC, TB..=TE), ERRMSG);

            *ERRPTR = LXB[2];
            return Ok(());
        }
    }

    //
    // If we got this far, the table and column check out.  Fill in the
    // table and column indices in their respective descriptors.
    //
    EQRYI[(BASE + EQTORD)] = TABIDX;
    EQRYI[((BASE + EQVDSZ) + EQCIDX)] = COLIDX;

    Ok(())
}
