//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
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
const LBCELL: i32 = -5;
const SHORT: i32 = 32;

//$Procedure  ZZEKNRES ( Private: EK, resolve names in encoded query )
pub fn ZZEKNRES(
    QUERY: &[u8],
    EQRYI: &mut [i32],
    EQRYC: &[u8],
    ERROR: &mut bool,
    ERRMSG: &mut [u8],
    ERRPTR: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut EQRYI = DummyArrayMut::new(EQRYI, LBCELL..);
    let mut ALIAS = ActualCharArray::new(TNAMSZ, 1..=MAXTAB);
    let mut LTABLE = [b' '; TNAMSZ as usize];
    let mut TABLE = ActualCharArray::new(TNAMSZ, 1..=MAXTAB);
    let mut BASE: i32 = 0;
    let mut CC = StackArray::<i32, 10>::new(1..=MAXTAB);
    let mut CNSTYP: i32 = 0;
    let mut IPARSE: i32 = 0;
    let mut J: i32 = 0;
    let mut LXB: i32 = 0;
    let mut LXE: i32 = 0;
    let mut NCNJ: i32 = 0;
    let mut NCNS: i32 = 0;
    let mut NLOAD: i32 = 0;
    let mut NORD: i32 = 0;
    let mut NSEL: i32 = 0;
    let mut NTAB: i32 = 0;
    let mut FND: bool = false;

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

    //
    // The query must have been parsed at this point, or it's no go.
    //
    ZZEKREQI(EQRYI.as_slice(), b"PARSED", &mut IPARSE, ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    if (IPARSE == IFALSE) {
        CHKIN(b"ZZEKNRES", ctx)?;
        SETMSG(b"Encoded query has not been parsed.", ctx);
        SIGERR(b"SPICE(QUERYNOTPARSED)", ctx)?;
        CHKOUT(b"ZZEKNRES", ctx)?;
        return Ok(());
    }

    //
    // Get the important counts from the query.
    //
    ZZEKREQI(EQRYI.as_slice(), b"NUM_TABLES", &mut NTAB, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_CONSTRAINTS", &mut NCNS, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_CONJUNCTIONS", &mut NCNJ, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_ORDERBY_COLS", &mut NORD, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_SELECT_COLS", &mut NSEL, ctx)?;

    //
    // Start out by fetching the table names and their aliases.
    //
    for I in 1..=NTAB {
        ZZEKQTAB(
            EQRYI.as_slice(),
            EQRYC,
            I,
            &mut TABLE[I],
            &mut ALIAS[I],
            ctx,
        )?;
    }

    //
    // Make sure that the aliases are distinct.  Rather than sorting
    // them, we'll check them in left-to-right order.
    //
    for I in 1..=(NTAB - 1) {
        {
            let m1__: i32 = (I + 1);
            let m2__: i32 = NTAB;
            let m3__: i32 = 1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if (fstr::eq(ALIAS.get(I), ALIAS.get(J)) && fstr::ne(ALIAS.get(I), b" ")) {
                    *ERROR = true;
                    fstr::assign(ERRMSG, b"Non-distinct alias <#> was found.");

                    BASE = (EQVBAS + ((((J - 1) * 2) + 1) * EQVDSZ));
                    LXB = EQRYI[(BASE + EQBLEX)];
                    LXE = EQRYI[(BASE + EQELEX)];

                    REPMC(
                        &ERRMSG.to_vec(),
                        b"#",
                        fstr::substr(QUERY, LXB..=LXE),
                        ERRMSG,
                    );

                    *ERRPTR = LXB;

                    return Ok(());
                }
                //
                // We've checked the Jth alias for a match.
                //
                J += m3__;
            }
        }
    }

    //
    // Make sure that no alias matches a table name other than that of
    // the table it corresponds to.
    //
    for I in 1..=NTAB {
        J = ISRCHC(&ALIAS[I], NTAB, TABLE.as_arg());

        if (J != 0) {
            if (J != I) {
                *ERROR = true;
                fstr::assign(ERRMSG, b"Alias <#> conflicts with table name.");

                BASE = (EQVBAS + ((((I - 1) * 2) + 1) * EQVDSZ));
                LXB = EQRYI[(BASE + EQBLEX)];
                LXE = EQRYI[(BASE + EQELEX)];

                REPMC(
                    &ERRMSG.to_vec(),
                    b"#",
                    fstr::substr(QUERY, LXB..=LXE),
                    ERRMSG,
                );

                *ERRPTR = LXB;

                return Ok(());
            }
        }
    }

    //
    // Make sure that all of the tables are loaded in the EK system.
    //
    EKNTAB(&mut NLOAD, ctx)?;

    for I in 1..=NTAB {
        FND = false;
        J = 1;

        while ((J <= NLOAD) && !FND) {
            EKTNAM(J, &mut LTABLE, ctx)?;

            if fstr::eq(TABLE.get(I), &LTABLE) {
                //
                // When we find a loaded table, save the column count for
                // that table.
                //
                FND = true;
                EKCCNT(TABLE.first(), &mut CC[I], ctx)?;
            } else {
                J = (J + 1);
            }
        }

        if !FND {
            *ERROR = true;
            fstr::assign(ERRMSG, b"Table <#> is not currently loaded.");
            //
            // In order to set the error pointer, we'll need the
            // lexeme begin value for the offending table.
            //
            BASE = (EQVBAS + (((I - 1) * 2) * EQVDSZ));

            LXB = EQRYI[(BASE + EQBLEX)];
            LXE = EQRYI[(BASE + EQELEX)];

            REPMC(
                &ERRMSG.to_vec(),
                b"#",
                fstr::substr(QUERY, LXB..=LXE),
                ERRMSG,
            );

            *ERRPTR = LXB;

            return Ok(());
        }
    }

    //
    // At this point, the tables and aliases are deemed correct.  For
    // safety, fill in each table and alias descriptor with its
    // ordinal position.
    //
    for I in 1..=NTAB {
        BASE = (EQVBAS + (((I - 1) * 2) * EQVDSZ));

        EQRYI[(BASE + EQTORD)] = I;
        EQRYI[((BASE + EQVDSZ) + EQTORD)] = I;
    }

    //
    // Check the column names used in the constraints.
    //
    for I in 1..=NCNS {
        //
        // Calculate the base address of the constraint.
        //
        BASE = ((EQVBAS + ((NTAB * 2) * EQVDSZ)) + ((I - 1) * EQCDSZ));

        //
        // Obtain the constraint type.
        //
        CNSTYP = EQRYI[(BASE + EQCTYP)];

        //
        // Check the column and table on the LHS of the constraint.
        //
        ZZEKCCHK(
            QUERY,
            EQRYI.as_slice_mut(),
            EQRYC,
            NTAB,
            TABLE.as_arg(),
            ALIAS.as_arg(),
            ((BASE + EQLTAB) - 1),
            ERROR,
            ERRMSG,
            ERRPTR,
            ctx,
        )?;

        if *ERROR {
            return Ok(());
        }

        if (CNSTYP == EQCOL) {
            //
            // Check the column and table on the RHS of the constraint.
            //
            ZZEKCCHK(
                QUERY,
                EQRYI.as_slice_mut(),
                EQRYC,
                NTAB,
                TABLE.as_arg(),
                ALIAS.as_arg(),
                ((BASE + EQRTAB) - 1),
                ERROR,
                ERRMSG,
                ERRPTR,
                ctx,
            )?;

            if *ERROR {
                return Ok(());
            }
        }
    }

    //
    // Do the same checks and assignments for the SELECT columns.
    //
    for I in 1..=NSEL {
        //
        // Calculate the base address of the SELECT column descriptor.
        //
        BASE = (((((EQVBAS + ((NTAB * 2) * EQVDSZ)) + NCNJ) + (NCNS * EQCDSZ)) + (NORD * EQODSZ))
            + ((I - 1) * EQSDSZ));

        ZZEKCCHK(
            QUERY,
            EQRYI.as_slice_mut(),
            EQRYC,
            NTAB,
            TABLE.as_arg(),
            ALIAS.as_arg(),
            BASE,
            ERROR,
            ERRMSG,
            ERRPTR,
            ctx,
        )?;

        if *ERROR {
            return Ok(());
        }
    }

    //
    // Do the same checks and assignments for the order-by columns.
    //
    for I in 1..=NORD {
        //
        // Calculate the base address of the order-by column descriptor.
        //
        BASE = ((((EQVBAS + ((NTAB * 2) * EQVDSZ)) + NCNJ) + (NCNS * EQCDSZ)) + ((I - 1) * EQODSZ));

        ZZEKCCHK(
            QUERY,
            EQRYI.as_slice_mut(),
            EQRYC,
            NTAB,
            TABLE.as_arg(),
            ALIAS.as_arg(),
            BASE,
            ERROR,
            ERRMSG,
            ERRPTR,
            ctx,
        )?;

        if *ERROR {
            return Ok(());
        }
    }

    //
    // Indicate completion of name resolution.
    //
    ZZEKWEQI(b"NAMES_RESOLVED", ITRUE, EQRYI.as_slice_mut(), ctx)?;

    Ok(())
}
