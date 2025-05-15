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
const TNAMSZ: i32 = 64;
const TKKEY: i32 = 1;
const TKID: i32 = (TKKEY + 1);
const TKINT: i32 = (TKID + 1);
const TKDP: i32 = (TKINT + 1);
const TKQSTR: i32 = (TKDP + 1);
const TKLPAR: i32 = (TKQSTR + 1);
const TKRPAR: i32 = (TKLPAR + 1);
const TKCOMA: i32 = (TKRPAR + 1);
const TKDOT: i32 = (TKCOMA + 1);
const TKSTAR: i32 = (TKDOT + 1);
const TKEOQ: i32 = (TKSTAR + 1);
const LBCELL: i32 = -5;
const LONG: i32 = 160;

//$Procedure  ZZEKPCOL ( Private: EK, parse column name )
pub fn ZZEKPCOL(
    QCOL: &[u8],
    EQRYI: &[i32],
    EQRYC: &[u8],
    TABLE: &mut [u8],
    ALIAS: &mut [u8],
    TABIDX: &mut i32,
    COLUMN: &mut [u8],
    COLIDX: &mut i32,
    ERROR: &mut bool,
    ERRMSG: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let EQRYI = DummyArray::new(EQRYI, LBCELL..);
    let mut ALSLST = ActualCharArray::new(TNAMSZ, 1..=MAXTAB);
    let mut CHRBUF = [b' '; LONG as usize];
    let mut TABLST = ActualCharArray::new(TNAMSZ, 1..=MAXTAB);
    let mut TMPCOL = [b' '; CNAMSZ as usize];
    let mut TMPTAB = [b' '; TNAMSZ as usize];
    let mut NUMVLS = StackArray::<f64, 3>::new(1..=3);
    let mut ATTDSC = StackArray::<i32, 6>::new(1..=ADSCSZ);
    let mut CC: i32 = 0;
    let mut CHBEGS = StackArray::<i32, 3>::new(1..=3);
    let mut CHENDS = StackArray::<i32, 3>::new(1..=3);
    let mut I: i32 = 0;
    let mut ICHECK: i32 = 0;
    let mut J: i32 = 0;
    let mut LXBEGS = StackArray::<i32, 3>::new(1..=3);
    let mut LXENDS = StackArray::<i32, 3>::new(1..=3);
    let mut NMATCH: i32 = 0;
    let mut NTAB: i32 = 0;
    let mut NTOKEN: i32 = 0;
    let mut TOKENS = StackArray::<i32, 3>::new(1..=3);
    let mut VALUES = StackArray::<i32, 3>::new(1..=3);
    let mut FND: bool = false;
    let mut QUAL: bool = false;

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
    // Use discovery check-in.
    //
    *ERROR = false;
    fstr::assign(ERRMSG, b" ");

    ZZEKREQI(EQRYI.as_slice(), b"SEM_CHECKED", &mut ICHECK, ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // Make sure the encoded query is in order before proceeding.
    //
    if (ICHECK == IFALSE) {
        CHKIN(b"ZZEKPCOL", ctx)?;
        SETMSG(b"Encoded query has not yet been semantically checked.", ctx);
        SIGERR(b"SPICE(NOTSEMCHECKED)", ctx)?;
        CHKOUT(b"ZZEKPCOL", ctx)?;
        return Ok(());
    }

    //
    // Scan the input column name.  There are only two valid token
    // sequences possible:
    //
    //    <identifier>
    //
    //    <identifier> . <identifier>
    //
    // ZZEKSCAN should therefore return 1 or 3 tokens.
    //
    ZZEKSCAN(
        QCOL,
        3,
        0,
        &mut NTOKEN,
        TOKENS.as_slice_mut(),
        LXBEGS.as_slice_mut(),
        LXENDS.as_slice_mut(),
        VALUES.as_slice_mut(),
        NUMVLS.as_slice_mut(),
        &mut CHRBUF,
        CHBEGS.as_slice_mut(),
        CHENDS.as_slice_mut(),
        ERROR,
        ERRMSG,
        ctx,
    )?;

    if *ERROR {
        return Ok(());
    }

    if (NTOKEN == 1) {
        if (TOKENS[1] != TKID) {
            *ERROR = true;
            fstr::assign(
                ERRMSG,
                b"Invalid column name; name should consist of an identifier.",
            );
            return Ok(());
        }

        UCASE(QCOL, COLUMN, ctx);
        QUAL = false;
    } else if (NTOKEN == 3) {
        if (TOKENS[1] != TKID) {
            *ERROR = true;
            fstr::assign(
                ERRMSG,
                b"Invalid table name; name should consist of an identifier.",
            );
            return Ok(());
        } else if (TOKENS[2] != TKDOT) {
            *ERROR = true;
            fstr::assign(
                ERRMSG,
                b"Invalid qualified column name; table name should be followed by a period.",
            );
            return Ok(());
        } else if (TOKENS[3] != TKID) {
            *ERROR = true;
            fstr::assign(
                ERRMSG,
                b"Invalid column name; name should consist of an identifier.",
            );
            return Ok(());
        }

        I = VALUES[1];
        J = VALUES[3];
        fstr::assign(&mut TMPTAB, fstr::substr(&CHRBUF, CHBEGS[I]..=CHENDS[I]));
        fstr::assign(COLUMN, fstr::substr(&CHRBUF, CHBEGS[J]..=CHENDS[J]));
        QUAL = true;
    } else {
        *ERROR = true;
        fstr::assign(ERRMSG, b"Invalid tokens present in qualified column name. Valid syntax is <column> or <table>.<column>");
        return Ok(());
    }

    //
    // At this point, COLUMN and QUAL are set.  If a qualifying table
    // or alias was supplied, that string is stored in TMPTAB.  Both
    // COLUMN and TMPTAB are in upper case.
    //
    // If we got this far, we'll need to look up the table names and
    // aliases from the query.
    //
    ZZEKREQI(EQRYI.as_slice(), b"NUM_TABLES", &mut NTAB, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NTAB;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            ZZEKQTAB(
                EQRYI.as_slice(),
                EQRYC,
                I,
                &mut TABLST[I],
                &mut ALSLST[I],
                ctx,
            )?;
            I += m3__;
        }
    }

    //
    // If QCOL contains a table name, look for that name in the
    // table list, and if necessary, in the alias list.
    //
    if QUAL {
        *TABIDX = ISRCHC(&TMPTAB, NTAB, TABLST.as_arg());

        if (*TABIDX == 0) {
            *TABIDX = ISRCHC(&TMPTAB, NTAB, ALSLST.as_arg());
        }

        //
        // If we didn't find the table name in either list, it's just
        // plain wrong.
        //
        if (*TABIDX == 0) {
            *ERROR = true;
            fstr::assign(
                ERRMSG,
                b"Table name <#> does not match table or alias from query.",
            );
            REPMC(&ERRMSG.to_vec(), b"#", &TMPTAB, ERRMSG);
            return Ok(());
        }

        //
        // At this point, TABIDX is valid.  Locate the column within
        // the table.
        //
        EKCCNT(&TABLST[*TABIDX], &mut CC, ctx)?;

        if FAILED(ctx) {
            return Ok(());
        }

        FND = false;
        J = 1;

        while ((J <= CC) && !FND) {
            EKCII(&TABLST[*TABIDX], J, &mut TMPCOL, ATTDSC.as_slice_mut(), ctx)?;

            if fstr::eq(&TMPCOL, COLUMN) {
                *COLIDX = J;
                FND = true;
            } else {
                J = (J + 1);
            }
        }

        if !FND {
            *ERROR = true;
            fstr::assign(
                ERRMSG,
                b"Column name <#> does not appear in the qualifying table <#>.",
            );
            REPMC(&ERRMSG.to_vec(), b"#", COLUMN, ERRMSG);
            REPMC(&ERRMSG.to_vec(), b"#", &TMPTAB, ERRMSG);
            return Ok(());
        }
    //
    // At this point, TABIDX and COLIDX are set correctly.
    //
    } else {
        //
        // No qualifying table name was supplied.  COLUMN had better
        // be a unique column name among the set of columns belong to
        // tables in the FROM clause of the input query.  Check the
        // columns for each table in the FROM clause, looking for
        // matches.
        //
        NMATCH = 0;

        {
            let m1__: i32 = 1;
            let m2__: i32 = NTAB;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                EKCCNT(&TABLST[I], &mut CC, ctx)?;

                if FAILED(ctx) {
                    return Ok(());
                }

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = CC;
                    let m3__: i32 = 1;
                    J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        EKCII(&TABLST[I], J, &mut TMPCOL, ATTDSC.as_slice_mut(), ctx)?;

                        if fstr::eq(&TMPCOL, COLUMN) {
                            NMATCH = (NMATCH + 1);
                            fstr::assign(COLUMN, &TMPCOL);
                            *COLIDX = J;
                            *TABIDX = I;
                        }

                        J += m3__;
                    }
                }

                I += m3__;
            }
        }
        //
        // Check to see whether we have the unique identification we're
        // hoping for.
        //
        if (NMATCH == 0) {
            *ERROR = true;
            fstr::assign(
                ERRMSG,
                b"Column name <#> does not appear in any table in FROM clause of query.",
            );
            REPMC(&ERRMSG.to_vec(), b"#", COLUMN, ERRMSG);
            return Ok(());
        } else if (NMATCH > 1) {
            *ERROR = true;
            fstr::assign(
                ERRMSG,
                b"Column name <#> is ambiguous without a qualifying table name.",
            );
            REPMC(&ERRMSG.to_vec(), b"#", COLUMN, ERRMSG);
            return Ok(());
        }
        //
        // At this point, COLUMN, TABIDX and COLIDX are set correctly.
        //
    }

    //
    // At this point, COLUMN, TABIDX and COLIDX are set correctly,
    // regardless of whether the input name was qualified.  Fill the rest
    // of our output variables.
    //
    fstr::assign(TABLE, TABLST.get(*TABIDX));
    fstr::assign(ALIAS, ALSLST.get(*TABIDX));

    Ok(())
}
