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
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const NATTR: i32 = 5;
const MAXTOK: i32 = 20;
const TOKLEN: i32 = 32;
const MSGLEN: i32 = 320;
const NRKEY: i32 = 1;
const TYPPOS: i32 = 1;
const SIZPOS: i32 = (TYPPOS + 1);
const IDXPOS: i32 = (SIZPOS + 1);
const NFLPOS: i32 = (IDXPOS + 1);
const FXCPOS: i32 = (NFLPOS + 1);

struct SaveVars {
    ATTKEY: ActualCharArray,
    REQKEY: StackArray<i32, 1>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ATTKEY = ActualCharArray::new(TOKLEN, 1..=NATTR);
        let mut REQKEY = StackArray::<i32, 1>::new(1..=NRKEY);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"DATATYPE"),
                Val::C(b"SIZE"),
                Val::C(b"INDEXED"),
                Val::C(b"NULLS_OK"),
                Val::C(b"FIXED_COUNT"),
            ]
            .into_iter();
            ATTKEY
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(1)].into_iter();
            REQKEY
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { ATTKEY, REQKEY }
    }
}

//$Procedure      ZZEKPDEC ( EK, parse column declaration )
pub fn ZZEKPDEC(DECL: &[u8], PARDSC: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut PARDSC = DummyArrayMut::new(PARDSC, 1..);
    let mut MSG = [b' '; MSGLEN as usize];
    let mut TOKENS = ActualCharArray::new(TOKLEN, 1..=MAXTOK);
    let mut ATTLOC = StackArray::<i32, 11>::new(1..=CDSCSZ);
    let mut J: i32 = 0;
    let mut N: i32 = 0;
    let mut PTR: i32 = 0;
    let mut TOKLOC: i32 = 0;
    let mut ATTFND = StackArray::<bool, 11>::new(1..=CDSCSZ);
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Parameters naming indices of keywords in the attribute list
    // ATTKEY:
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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZEKPDEC", ctx)?;
    }

    //
    // Start with a clean slate.
    //
    CLEARI(CDSCSZ, PARDSC.as_slice_mut());

    //
    // Our declaration language has been cleverly designed so that the
    // characters
    //
    //    ','
    //    '='
    //
    // act as delimiters that LPARSM can make use
    // of.  LPARSM will hand us back a token list that contains these
    // pairs of consecutive tokens:
    //
    //    +----------------------+
    //    | CLASS                |
    //    +----------------------+
    //    | <integer>            |
    //    +----------------------+
    //
    //    +----------------------+
    //    | DATATYPE             |
    //    +----------------------+
    //    | <type>               |
    //    +----------------------+
    //
    //    +----------------------+
    //    | SIZE                 |
    //    +----------------------+
    //    | <size specification> |  ( 'VARIABLE' or <integer> )
    //    +----------------------+
    //
    //    +----------------------+
    //    | INDEXED              |  (fixed-size columns only, optional)
    //    +----------------------+
    //    | <TRUE/FALSE>         |
    //    +----------------------+
    //
    //    +----------------------+
    //    | NULLS_OK             |  (optional)
    //    +----------------------+
    //    | <TRUE/FALSE>         |
    //    +----------------------+
    //
    //
    // The order of the token pairs is not necessarily as shown.
    //
    //
    LPARSM(DECL, b",=", MAXTOK, &mut N, TOKENS.as_arg_mut());

    //
    // Make sure the tokens are in upper case.  They are already
    // left-justified.
    //
    for I in 1..=N {
        UCASE(&TOKENS[I].to_vec(), &mut TOKENS[I], ctx);
    }

    //
    // See which clauses are present in the declaration, and keep track
    // of the token indices of the keywords that start the clauses.
    //
    for I in 1..=NATTR {
        ATTFND[I] = false;
    }

    for I in 1..=N {
        J = 1;
        FOUND = false;

        while ((J <= NATTR) && !FOUND) {
            if fstr::eq(TOKENS.get(I), save.ATTKEY.get(J)) {
                FOUND = true;
                ATTFND[J] = true;
                ATTLOC[J] = I;
            } else {
                J = (J + 1);
            }
        }
    }

    //
    // Make sure we got the required keyword tokens we were expecting.
    //
    for I in 1..=NRKEY {
        if !ATTFND[save.REQKEY[I]] {
            SETMSG(
                b"Required keyword # was not found in column declaration #.",
                ctx,
            );
            ERRCH(b"#", &save.ATTKEY[save.REQKEY[I]], ctx);
            ERRCH(b"#", DECL, ctx);
            SIGERR(b"SPICE(BADCOLUMDECL)", ctx)?;
            CHKOUT(b"ZZEKPDEC", ctx)?;
            return Ok(());
        }
    }

    //
    // If we got this far, we can start to fill in the data type
    // descriptor.  Starting at the location of the DATATYPE keyword,
    // we should see one of the following token sequences:
    //
    //    DATATYPE  =  DOUBLE PRECISION
    //    DATATYPE  =  INTEGER
    //    DATATYPE  =  TIME
    //    DATATYPE  =  CHARACTER*<integer>
    //    DATATYPE  =  CHARACTER*(<integer>)
    //    DATATYPE  =  CHARACTER**
    //    DATATYPE  =  CHARACTER*(*)
    //
    // The character declarations may have white space surrounding
    // the length specifier.
    //
    // Find the location where the data type token should be.
    //
    TOKLOC = (ATTLOC[TYPPOS] + 1);

    if (N < TOKLOC) {
        SETMSG(
            b"Column data type specification did not follow \"DATATYPE\" keyword in declaration #.",
            ctx,
        );
        ERRCH(b"#", DECL, ctx);
        SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
        CHKOUT(b"ZZEKPDEC", ctx)?;
        return Ok(());
    }

    if fstr::eq(TOKENS.get(TOKLOC), b"INTEGER") {
        PARDSC[TYPIDX] = INT;
        PARDSC[LENIDX] = 1;
    } else if EQSTR(&TOKENS[TOKLOC], b"DOUBLE PRECISION") {
        PARDSC[TYPIDX] = DP;
        PARDSC[LENIDX] = 1;
    } else if EQSTR(&TOKENS[TOKLOC], b"TIME") {
        PARDSC[TYPIDX] = TIME;
        PARDSC[LENIDX] = 1;
    } else if fstr::eq(fstr::substr(TOKENS.get(TOKLOC), 1..=9), b"CHARACTER") {
        PARDSC[TYPIDX] = CHR;
        //
        // To simplify picking up the length specification, compress
        // out blanks and parentheses.  This should leave us with
        // a token of the form
        //
        //    CHARACTER*<integer>
        //
        // or
        //
        //    CHARACTER**
        //
        //
        CMPRSS(b" ", 0, &TOKENS[TOKLOC].to_vec(), &mut TOKENS[TOKLOC]);
        CMPRSS(b"(", 0, &TOKENS[TOKLOC].to_vec(), &mut TOKENS[TOKLOC]);
        CMPRSS(b")", 0, &TOKENS[TOKLOC].to_vec(), &mut TOKENS[TOKLOC]);

        if fstr::ne(fstr::substr(TOKENS.get(TOKLOC), 10..=10), b"*") {
            SETMSG(b"Required asterisk missing from character column declaration:  #  in declaration:  #", ctx);
            ERRCH(b"#", &TOKENS[TOKLOC], ctx);
            ERRCH(b"#", DECL, ctx);
            SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
            CHKOUT(b"ZZEKPDEC", ctx)?;
            return Ok(());
        }

        if fstr::eq(fstr::substr(TOKENS.get(TOKLOC), 11..=11), b"*") {
            //
            // The string length is variable.
            //
            PARDSC[LENIDX] = IFALSE;
        } else {
            //
            // The portion of the token following the asterisk should be a
            // string length.
            //
            fstr::assign(&mut MSG, b" ");
            NPARSI(
                fstr::substr(&TOKENS[TOKLOC], 11..),
                &mut PARDSC[LENIDX],
                &mut MSG,
                &mut PTR,
                ctx,
            );

            if fstr::ne(&MSG, b" ") {
                SETMSG(
                    b"String length specification # didn\'t parse as an integer in declaration   #",
                    ctx,
                );
                ERRCH(b"#", fstr::substr(&TOKENS[TOKLOC], 11..), ctx);
                ERRCH(b"#", DECL, ctx);
                SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
                CHKOUT(b"ZZEKPDEC", ctx)?;
                return Ok(());
            }
        }
    } else {
        //
        // The type specification is invalid.
        //
        SETMSG(
            b"Data type specification # is unrecognized in declaration #.",
            ctx,
        );
        ERRCH(b"#", &TOKENS[TOKLOC], ctx);
        ERRCH(b"#", DECL, ctx);
        SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
        CHKOUT(b"ZZEKPDEC", ctx)?;
        return Ok(());
    }

    //
    // Next, parse the size specification, if we have one.  If it's
    // valid, it's either the string 'VARIABLE' or it's an integer.
    //
    if ATTFND[SIZPOS] {
        TOKLOC = (ATTLOC[SIZPOS] + 1);

        if (N < TOKLOC) {
            SETMSG(
                b"Column size specification did not follow \"SIZE\" keyword in declaration #.",
                ctx,
            );
            ERRCH(b"#", DECL, ctx);
            SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
            CHKOUT(b"ZZEKPDEC", ctx)?;
            return Ok(());
        }

        if fstr::eq(TOKENS.get(TOKLOC), b"VARIABLE") {
            //
            // Variable size entries are not allowed for CHARACTER*(*)
            // columns.
            //
            if (PARDSC[TYPIDX] == CHR) {
                if (PARDSC[LENIDX] == IFALSE) {
                    SETMSG(b"Column size specification was VARIABLE for a CHARACTER*(*) column in  declaration #.", ctx);
                    ERRCH(b"#", DECL, ctx);
                    SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
                    CHKOUT(b"ZZEKPDEC", ctx)?;
                    return Ok(());
                }
            }

            PARDSC[SIZIDX] = IFALSE;
        } else {
            NPARSI(
                &TOKENS[TOKLOC],
                &mut PARDSC[SIZIDX],
                &mut MSG,
                &mut PTR,
                ctx,
            );

            if fstr::ne(&MSG, b" ") {
                SETMSG(b"Column element size  specification # didn\'t parse as an integer in in declaration #", ctx);
                ERRCH(b"#", &TOKENS[TOKLOC], ctx);
                ERRCH(b"#", DECL, ctx);
                SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
                CHKOUT(b"ZZEKPDEC", ctx)?;
                return Ok(());
            }
        }
    } else {
        //
        // If the size is not specified, it defaults to 1.
        //
        PARDSC[SIZIDX] = 1;
    }

    //
    // The data type and entry size determine the column's class.
    //
    if (PARDSC[TYPIDX] == CHR) {
        //
        // The character classes are 3 for scalars, 6 for arrays.
        //
        if (PARDSC[SIZIDX] == 1) {
            PARDSC[CLSIDX] = 3;
        } else {
            PARDSC[CLSIDX] = 6;
        }
    } else if (PARDSC[TYPIDX] == INT) {
        //
        // The integer classes are 1 for scalars, 4 for arrays.
        //
        if (PARDSC[SIZIDX] == 1) {
            PARDSC[CLSIDX] = 1;
        } else {
            PARDSC[CLSIDX] = 4;
        }
    } else if ((PARDSC[TYPIDX] == DP) || (PARDSC[TYPIDX] == TIME)) {
        //
        // The d.p. classes are 2 for scalars, 6 for arrays.  TIME
        // values are represented using d.p. classes as well.
        //
        if (PARDSC[SIZIDX] == 1) {
            PARDSC[CLSIDX] = 2;
        } else {
            PARDSC[CLSIDX] = 5;
        }
    }

    //
    // Parse the `NULLS_OK' clause, if we have one.
    //
    if ATTFND[NFLPOS] {
        TOKLOC = (ATTLOC[NFLPOS] + 1);

        if (N < TOKLOC) {
            SETMSG(
                b"Boolean value did not follow \"NULLS_OK\" keyword in declaration #.",
                ctx,
            );
            ERRCH(b"#", DECL, ctx);
            SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
            CHKOUT(b"ZZEKPDEC", ctx)?;
            return Ok(());
        }

        if fstr::eq(TOKENS.get(TOKLOC), b"TRUE") {
            PARDSC[NFLIDX] = ITRUE;
        } else if fstr::eq(TOKENS.get(TOKLOC), b"FALSE") {
            PARDSC[NFLIDX] = IFALSE;
        } else {
            SETMSG(
                b"Invalid token # follows NULLS_OK keyword in declaration #. ",
                ctx,
            );
            ERRCH(b"#", &TOKENS[TOKLOC], ctx);
            ERRCH(b"#", DECL, ctx);
            SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
            CHKOUT(b"ZZEKPDEC", ctx)?;
            return Ok(());
        }
    } else {
        //
        // As a default, nulls are not allowed.
        //
        PARDSC[NFLIDX] = IFALSE;
    }

    //
    //
    // Parse the `INDEXED' clause, if we have one.
    //
    if ATTFND[IDXPOS] {
        TOKLOC = (ATTLOC[IDXPOS] + 1);

        if (N < TOKLOC) {
            SETMSG(
                b"Boolean value did not follow \"INDEXED\" keyword in declaration #.",
                ctx,
            );
            ERRCH(b"#", DECL, ctx);
            SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
            CHKOUT(b"ZZEKPDEC", ctx)?;
            return Ok(());
        }

        if fstr::eq(TOKENS.get(TOKLOC), b"TRUE") {
            //
            // If we have a fixed-size column whose size is 1, then it's
            // possible to index that column.  Otherwise, we should not
            // have an `INDEXED' clause.
            //
            if (PARDSC[SIZIDX] != 1) {
                SETMSG(
                    b"Non-scalar columns cannot be indexed. Declaration was #.",
                    ctx,
                );
                ERRCH(b"#", DECL, ctx);
                SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
                CHKOUT(b"ZZEKPDEC", ctx)?;
                return Ok(());
            }

            PARDSC[IXTIDX] = ITRUE;
        } else if fstr::eq(TOKENS.get(TOKLOC), b"FALSE") {
            PARDSC[IXTIDX] = IFALSE;
        } else {
            SETMSG(
                b"Invalid token # follows INDEXED keyword in declaration #. ",
                ctx,
            );
            ERRCH(b"#", &TOKENS[TOKLOC], ctx);
            ERRCH(b"#", DECL, ctx);
            SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
            CHKOUT(b"ZZEKPDEC", ctx)?;
            return Ok(());
        }
    } else {
        //
        // As a default, the column is not indexed.
        //
        PARDSC[IXTIDX] = IFALSE;
    }

    //
    // Parse the `FIXED_COUNT' clause, if we have one.
    //
    if ATTFND[FXCPOS] {
        TOKLOC = (ATTLOC[FXCPOS] + 1);

        if (N < TOKLOC) {
            SETMSG(
                b"Boolean value did not follow \"FIXED_COUNT\" keyword in declaration #.",
                ctx,
            );
            ERRCH(b"#", DECL, ctx);
            SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
            CHKOUT(b"ZZEKPDEC", ctx)?;
            return Ok(());
        }

        if fstr::eq(TOKENS.get(TOKLOC), b"TRUE") {
            //
            // The column is a fixed-count column.  Only scalar columns
            // are permitted to have fixed count.  We adjust the column
            // class to indicate fixed-count columns.
            //
            if (PARDSC[CLSIDX] == 1) {
                //
                // Map scalar integers.
                //
                PARDSC[CLSIDX] = 7;
            } else if (PARDSC[CLSIDX] == 2) {
                //
                // Map scalar d.p. numbers.
                //
                PARDSC[CLSIDX] = 8;
            } else if (PARDSC[CLSIDX] == 3) {
                //
                // Map scalar strings.
                //
                PARDSC[CLSIDX] = 9;
            } else {
                SETMSG(
                    b"FIXED_COUNT attribute used in non-scalar column declaration #. ",
                    ctx,
                );
                ERRCH(b"#", DECL, ctx);
                SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
                CHKOUT(b"ZZEKPDEC", ctx)?;
                return Ok(());
            }
        } else if fstr::ne(TOKENS.get(TOKLOC), b"FALSE") {
            //
            // No action is required if the FIXED_COUNT keyword is set
            // to FALSE, but no value other than FALSE or TRUE may appear
            // on the RHS.
            //
            SETMSG(
                b"Invalid token # follows NULLS_OK keyword in declaration #. ",
                ctx,
            );
            ERRCH(b"#", &TOKENS[TOKLOC], ctx);
            ERRCH(b"#", DECL, ctx);
            SIGERR(b"SPICE(BADCOLUMNDECL)", ctx)?;
            CHKOUT(b"ZZEKPDEC", ctx)?;
            return Ok(());
        }
    }

    CHKOUT(b"ZZEKPDEC", ctx)?;
    Ok(())
}
