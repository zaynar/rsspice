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
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const LBCELL: i32 = -5;
const SHORT: i32 = 32;

struct SaveVars {
    TYPSTR: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TYPSTR = ActualCharArray::new(SHORT, 1..=4);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"CHARACTER"),
                Val::C(b"DOUBLE PRECISION"),
                Val::C(b"INTEGER"),
                Val::C(b"TIME"),
            ]
            .into_iter();
            TYPSTR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { TYPSTR }
    }
}

//$Procedure  ZZEKSEMC ( Private: EK, semantically check encoded query )
pub fn ZZEKSEMC(
    QUERY: &[u8],
    EQRYI: &mut [i32],
    EQRYC: &[u8],
    ERROR: &mut bool,
    ERRMSG: &mut [u8],
    ERRPTR: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut EQRYI = DummyArrayMut::new(EQRYI, LBCELL..);
    let mut ALIAS = [b' '; TNAMSZ as usize];
    let mut COLNAM = [b' '; CNAMSZ as usize];
    let mut LHSTAB = [b' '; TNAMSZ as usize];
    let mut ORDTAB = [b' '; TNAMSZ as usize];
    let mut RHSTAB = [b' '; TNAMSZ as usize];
    let mut ATTDSC = StackArray::<i32, 6>::new(1..=ADSCSZ);
    let mut BASE: i32 = 0;
    let mut CNSTYP: i32 = 0;
    let mut COLIDX: i32 = 0;
    let mut IRSOLV: i32 = 0;
    let mut LHSSIZ: i32 = 0;
    let mut LHSTYP: i32 = 0;
    let mut LXB = StackArray::<i32, 2>::new(1..=2);
    let mut LXE = StackArray::<i32, 2>::new(1..=2);
    let mut NCNJ: i32 = 0;
    let mut NCNS: i32 = 0;
    let mut NORD: i32 = 0;
    let mut NTAB: i32 = 0;
    let mut OPCODE: i32 = 0;
    let mut RHSSIZ: i32 = 0;
    let mut RHSTYP: i32 = 0;
    let mut TABIDX: i32 = 0;
    let mut TRSOLV: i32 = 0;
    let mut LIKEOP: bool = false;
    let mut NULVAL: bool = false;

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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Use discovery check-in.
    //
    *ERROR = false;
    fstr::assign(ERRMSG, b" ");
    *ERRPTR = 0;

    ZZEKREQI(EQRYI.as_slice(), b"NAMES_RESOLVED", &mut IRSOLV, ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    if (IRSOLV == IFALSE) {
        CHKIN(b"ZZEKSEMC", ctx)?;
        SETMSG(b"Encoded query has not had names resolved.", ctx);
        SIGERR(b"SPICE(UNRESOLVEDNAMES)", ctx)?;
        CHKOUT(b"ZZEKSEMC", ctx)?;
        return Ok(());
    }

    ZZEKREQI(EQRYI.as_slice(), b"TIMES_RESOLVED", &mut TRSOLV, ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    if (TRSOLV == IFALSE) {
        CHKIN(b"ZZEKSEMC", ctx)?;
        SETMSG(b"Encoded query has not had time values resolved.", ctx);
        SIGERR(b"SPICE(UNRESOLVEDTIMES)", ctx)?;
        CHKOUT(b"ZZEKSEMC", ctx)?;
        return Ok(());
    }

    //
    // Get the important counts from the query.
    //
    ZZEKREQI(EQRYI.as_slice(), b"NUM_TABLES", &mut NTAB, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_CONSTRAINTS", &mut NCNS, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_CONJUNCTIONS", &mut NCNJ, ctx)?;
    ZZEKREQI(EQRYI.as_slice(), b"NUM_ORDERBY_COLS", &mut NORD, ctx)?;

    //
    // Perform semantic checks applicable to constraints.
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
        // Get the index of the table containing the LHS column, and get
        // the index of this column within that table.  Look up the
        // table name.
        //
        TABIDX = EQRYI[(((BASE + EQLTAB) - 1) + EQTORD)];
        COLIDX = EQRYI[(((BASE + EQLCOL) - 1) + EQCIDX)];
        LXB[1] = EQRYI[(((BASE + EQLCOL) - 1) + EQBLEX)];
        LXE[1] = EQRYI[(((BASE + EQLCOL) - 1) + EQELEX)];

        ZZEKQTAB(
            EQRYI.as_slice(),
            EQRYC,
            TABIDX,
            &mut LHSTAB,
            &mut ALIAS,
            ctx,
        )?;

        //
        // Look up the name and attributes of the column on the LHS of the
        // constraint.
        //
        EKCII(&LHSTAB, COLIDX, &mut COLNAM, ATTDSC.as_slice_mut(), ctx)?;

        LHSTYP = ATTDSC[ATTTYP];
        LHSSIZ = ATTDSC[ATTSIZ];

        if (LHSSIZ != 1) {
            *ERROR = true;
            fstr::assign(
                ERRMSG,
                b"Non-scalar column <#> having size # found in query constraint.",
            );
            REPMC(
                &ERRMSG.to_vec(),
                b"#",
                fstr::substr(QUERY, LXB[1]..=LXE[2]),
                ERRMSG,
            );
            REPMI(&ERRMSG.to_vec(), b"#", LHSSIZ, ERRMSG, ctx);

            *ERRPTR = LXB[1];
            return Ok(());
        }

        //
        // Get the operator for the current constraint.
        //
        OPCODE = EQRYI[(BASE + EQOPCD)];

        //
        // Decide whether the constraint is an `IS NULL' or `IS NOT NULL'
        // test.
        //
        NULVAL = ((OPCODE == ISNULL) || (OPCODE == NOTNUL));

        //
        // Check for use of the LIKE or NOT LIKE operators.  These
        // operators may be used only if the LHS column has character
        // type.
        //
        LIKEOP = ((OPCODE == LIKE) || (OPCODE == UNLIKE));

        if (LIKEOP && (LHSTYP != CHR)) {
            *ERROR = true;
            fstr::assign(ERRMSG, b"LIKE and NOT LIKE operators may be used only with character columns.  Column <#> has type #.");

            REPMC(
                &ERRMSG.to_vec(),
                b"#",
                fstr::substr(QUERY, LXB[1]..=LXE[1]),
                ERRMSG,
            );
            REPMC(&ERRMSG.to_vec(), b"#", &save.TYPSTR[LHSTYP], ERRMSG);

            *ERRPTR = LXB[1];
            return Ok(());
        }

        //
        // If the constraint compares two columns, get the same
        // information for the RHS column.
        //
        if (CNSTYP == EQCOL) {
            TABIDX = EQRYI[(((BASE + EQRTAB) - 1) + EQTORD)];
            COLIDX = EQRYI[(((BASE + EQRCOL) - 1) + EQCIDX)];
            LXB[2] = EQRYI[(((BASE + EQRCOL) - 1) + EQBLEX)];
            LXE[2] = EQRYI[(((BASE + EQRCOL) - 1) + EQELEX)];

            ZZEKQTAB(
                EQRYI.as_slice(),
                EQRYC,
                TABIDX,
                &mut RHSTAB,
                &mut ALIAS,
                ctx,
            )?;

            //
            // Look up the name and attributes of the column on the RHS of
            // the constraint.
            //
            EKCII(&RHSTAB, COLIDX, &mut COLNAM, ATTDSC.as_slice_mut(), ctx)?;

            RHSTYP = ATTDSC[2];
            RHSSIZ = ATTDSC[4];

            if (RHSSIZ != 1) {
                *ERROR = true;
                fstr::assign(
                    ERRMSG,
                    b"Non-scalar column <#> having size # found in query constraint.",
                );
                REPMC(
                    &ERRMSG.to_vec(),
                    b"#",
                    fstr::substr(QUERY, LXB[1]..=LXE[2]),
                    ERRMSG,
                );
                REPMI(&ERRMSG.to_vec(), b"#", RHSSIZ, ERRMSG, ctx);

                *ERRPTR = LXB[2];
                return Ok(());
            }

            //
            // Check for data type mismatch.
            //
            if (RHSTYP != LHSTYP) {
                //
                // The only allowed mismatch is between integers and
                // d.p. numbers.
                //
                if ((((LHSTYP == TIME) || (LHSTYP == CHR)) || (RHSTYP == TIME)) || (RHSTYP == CHR))
                {
                    *ERROR = true;
                    fstr::assign(ERRMSG, b"Data type mismatch: column <#> has data type #; column <#> has data type #.");

                    REPMC(
                        &ERRMSG.to_vec(),
                        b"#",
                        fstr::substr(QUERY, LXB[1]..=LXE[1]),
                        ERRMSG,
                    );
                    REPMC(&ERRMSG.to_vec(), b"#", &save.TYPSTR[LHSTYP], ERRMSG);
                    REPMC(
                        &ERRMSG.to_vec(),
                        b"#",
                        fstr::substr(QUERY, LXB[2]..=LXE[2]),
                        ERRMSG,
                    );
                    REPMC(&ERRMSG.to_vec(), b"#", &save.TYPSTR[RHSTYP], ERRMSG);

                    *ERRPTR = LXB[2];
                    return Ok(());
                }
            }
        } else {
            //
            // The constraint compares a column against a value.  If the
            // operator is `IS NULL' or `IS NOT NULL', there are no
            // further semantic checks to be made.
            //
            if NULVAL {
                return Ok(());
            }

            //
            // Get the data type of the value on the RHS.
            //
            RHSTYP = EQRYI[(((BASE + EQBVAL) - 1) + EQDTYP)];
            LXB[2] = EQRYI[(((BASE + EQBVAL) - 1) + EQBLEX)];
            LXE[2] = EQRYI[(((BASE + EQBVAL) - 1) + EQELEX)];

            if (RHSTYP != LHSTYP) {
                //
                // The only allowed mismatch is between integers and
                // d.p. numbers.
                //
                if ((((LHSTYP == TIME) || (LHSTYP == CHR)) || (RHSTYP == TIME)) || (RHSTYP == CHR))
                {
                    *ERROR = true;
                    fstr::assign(ERRMSG, b"Data type mismatch: column <#> has data type #; value <#> has data type #.");

                    REPMC(
                        &ERRMSG.to_vec(),
                        b"#",
                        fstr::substr(QUERY, LXB[1]..=LXE[1]),
                        ERRMSG,
                    );
                    REPMC(&ERRMSG.to_vec(), b"#", &save.TYPSTR[LHSTYP], ERRMSG);
                    REPMC(
                        &ERRMSG.to_vec(),
                        b"#",
                        fstr::substr(QUERY, LXB[2]..=LXE[2]),
                        ERRMSG,
                    );
                    REPMC(&ERRMSG.to_vec(), b"#", &save.TYPSTR[RHSTYP], ERRMSG);

                    *ERRPTR = LXB[2];
                    return Ok(());
                }
            }
        }

        //
        // We've finished the checks on the current constraint.
        //
    }

    //
    // Now check the order-by columns, if any are present.  These
    // columns must have scalar type.
    //
    for I in 1..=NORD {
        //
        // Get the query column descriptor for the Ith order-by column.
        //
        BASE = ((((EQVBAS + ((NTAB * 2) * EQVDSZ)) + NCNJ) + (NCNS * EQCDSZ)) + ((I - 1) * EQODSZ));
        //
        // Look up the attributes of the column.  It's the size we're
        // after.
        //
        TABIDX = EQRYI[(BASE + EQTORD)];
        COLIDX = EQRYI[(((BASE + EQOCOL) - 1) + EQCIDX)];
        LXB[1] = EQRYI[(((BASE + EQOCOL) - 1) + EQBLEX)];
        LXE[1] = EQRYI[(((BASE + EQOCOL) - 1) + EQELEX)];

        ZZEKQTAB(
            EQRYI.as_slice(),
            EQRYC,
            TABIDX,
            &mut ORDTAB,
            &mut ALIAS,
            ctx,
        )?;

        EKCII(&ORDTAB, COLIDX, &mut COLNAM, ATTDSC.as_slice_mut(), ctx)?;

        if (ATTDSC[4] != 1) {
            *ERROR = true;
            fstr::assign(
                ERRMSG,
                b"Non-scalar column <#> having size # found in order-by column.",
            );
            REPMC(
                &ERRMSG.to_vec(),
                b"#",
                fstr::substr(QUERY, LXB[1]..=LXE[2]),
                ERRMSG,
            );
            REPMI(&ERRMSG.to_vec(), b"#", ATTDSC[4], ERRMSG, ctx);

            *ERRPTR = LXB[1];
            return Ok(());
        }
    }

    //
    // Indicate completion of semantic checking.
    //
    ZZEKWEQI(b"SEM_CHECKED", ITRUE, EQRYI.as_slice_mut(), ctx)?;

    Ok(())
}
