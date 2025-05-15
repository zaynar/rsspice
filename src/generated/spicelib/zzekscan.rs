//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const KWLEN: i32 = 32;
const NKEYWD: i32 = 29;
const KWALL: i32 = 1;
const KWAND: i32 = (KWALL + 1);
const KWASND: i32 = (KWAND + 1);
const KWAVG: i32 = (KWASND + 1);
const KWBETW: i32 = (KWAVG + 1);
const KWBY: i32 = (KWBETW + 1);
const KWCNT: i32 = (KWBY + 1);
const KWDSND: i32 = (KWCNT + 1);
const KWDSTN: i32 = (KWDSND + 1);
const KWEQ: i32 = (KWDSTN + 1);
const KWFROM: i32 = (KWEQ + 1);
const KWGE: i32 = (KWFROM + 1);
const KWGRP: i32 = (KWGE + 1);
const KWGT: i32 = (KWGRP + 1);
const KWHAV: i32 = (KWGT + 1);
const KWIS: i32 = (KWHAV + 1);
const KWLE: i32 = (KWIS + 1);
const KWLIKE: i32 = (KWLE + 1);
const KWLT: i32 = (KWLIKE + 1);
const KWMAX: i32 = (KWLT + 1);
const KWMIN: i32 = (KWMAX + 1);
const KWNE: i32 = (KWMIN + 1);
const KWNOT: i32 = (KWNE + 1);
const KWNULL: i32 = (KWNOT + 1);
const KWOR: i32 = (KWNULL + 1);
const KWORDR: i32 = (KWOR + 1);
const KWSEL: i32 = (KWORDR + 1);
const KWSUM: i32 = (KWSEL + 1);
const KWWHER: i32 = (KWSUM + 1);
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
const DQUOTE: &[u8] = b"\"";
const SQUOTE: &[u8] = b"\'";
const LNSIZE: i32 = 80;
const NXTTOK: i32 = 1;
const NEWTOK: i32 = (NXTTOK + 1);
const TERM: i32 = (NEWTOK + 1);
const QSTR: i32 = (TERM + 1);
const DOT: i32 = (QSTR + 1);
const NUMBER: i32 = (DOT + 1);
const IDENT: i32 = (NUMBER + 1);
const SPCIAL: i32 = (IDENT + 1);
const LBCELL: i32 = -5;
const MXSPEC: i32 = 512;
const NSPEC: i32 = 13;
const SPCLEN: i32 = 2;

struct SaveVars {
    CHR: Vec<u8>,
    SPCSTR: ActualCharArray,
    HDCHRS: Vec<u8>,
    KEYWDS: ActualCharArray,
    TLCHRS: Vec<u8>,
    TQUERY: Vec<u8>,
    CHCARD: i32,
    CPTR: i32,
    I: i32,
    IDSPEC: ActualArray<i32>,
    J: i32,
    KWVALS: StackArray<i32, 29>,
    L: i32,
    LAST: i32,
    LENGTH: i32,
    NCHARS: i32,
    NNUMS: i32,
    NSTRS: i32,
    PTR: i32,
    ROOM: i32,
    STATE: i32,
    SPCTOK: StackArray<i32, 13>,
    SPCVAL: StackArray<i32, 13>,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CHR = vec![b' '; 1 as usize];
        let mut SPCSTR = ActualCharArray::new(SPCLEN, 1..=NSPEC);
        let mut HDCHRS = vec![b' '; LNSIZE as usize];
        let mut KEYWDS = ActualCharArray::new(KWLEN, 1..=NKEYWD);
        let mut TLCHRS = vec![b' '; LNSIZE as usize];
        let mut TQUERY = vec![b' '; MAXQRY as usize];
        let mut CHCARD: i32 = 0;
        let mut CPTR: i32 = 0;
        let mut I: i32 = 0;
        let mut IDSPEC = ActualArray::<i32>::new(LBCELL..=MXSPEC);
        let mut J: i32 = 0;
        let mut KWVALS = StackArray::<i32, 29>::new(1..=NKEYWD);
        let mut L: i32 = 0;
        let mut LAST: i32 = 0;
        let mut LENGTH: i32 = 0;
        let mut NCHARS: i32 = 0;
        let mut NNUMS: i32 = 0;
        let mut NSTRS: i32 = 0;
        let mut PTR: i32 = 0;
        let mut ROOM: i32 = 0;
        let mut STATE: i32 = 0;
        let mut SPCTOK = StackArray::<i32, 13>::new(1..=NSPEC);
        let mut SPCVAL = StackArray::<i32, 13>::new(1..=NSPEC);
        let mut PASS1: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"ALL"),
                Val::I(KWALL),
                Val::C(b"AND"),
                Val::I(KWAND),
                Val::C(b"ASC "),
                Val::I(KWASND),
                Val::C(b"AVG "),
                Val::I(KWAVG),
                Val::C(b"BETWEEN"),
                Val::I(KWBETW),
                Val::C(b"BY"),
                Val::I(KWBY),
                Val::C(b"COUNT"),
                Val::I(KWCNT),
                Val::C(b"DESC"),
                Val::I(KWDSND),
                Val::C(b"DISTINCT"),
                Val::I(KWDSTN),
                Val::C(b"EQ"),
                Val::I(KWEQ),
            ]
            .into_iter();
            for I in intrinsics::range(1, 10, 1) {
                fstr::assign(KEYWDS.get_mut(I), clist.next().unwrap().into_str());
                KWVALS[I] = clist.next().unwrap().into_i32();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"FROM"),
                Val::I(KWFROM),
                Val::C(b"GE"),
                Val::I(KWGE),
                Val::C(b"GROUP"),
                Val::I(KWGRP),
                Val::C(b"GT"),
                Val::I(KWGT),
                Val::C(b"HAVING"),
                Val::I(KWHAV),
                Val::C(b"IS"),
                Val::I(KWIS),
                Val::C(b"LE"),
                Val::I(KWLE),
                Val::C(b"LIKE"),
                Val::I(KWLIKE),
                Val::C(b"LT"),
                Val::I(KWLT),
                Val::C(b"MAX"),
                Val::I(KWMAX),
            ]
            .into_iter();
            for I in intrinsics::range(11, 20, 1) {
                fstr::assign(KEYWDS.get_mut(I), clist.next().unwrap().into_str());
                KWVALS[I] = clist.next().unwrap().into_i32();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"MIN"),
                Val::I(KWMIN),
                Val::C(b"NE"),
                Val::I(KWNE),
                Val::C(b"NOT"),
                Val::I(KWNOT),
                Val::C(b"NULL"),
                Val::I(KWNULL),
                Val::C(b"OR"),
                Val::I(KWOR),
                Val::C(b"ORDER"),
                Val::I(KWORDR),
                Val::C(b"SELECT"),
                Val::I(KWSEL),
                Val::C(b"SUM"),
                Val::I(KWSUM),
                Val::C(b"WHERE"),
                Val::I(KWWHER),
            ]
            .into_iter();
            for I in intrinsics::range(21, NKEYWD, 1) {
                fstr::assign(KEYWDS.get_mut(I), clist.next().unwrap().into_str());
                KWVALS[I] = clist.next().unwrap().into_i32();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"!="),
                Val::I(TKKEY),
                Val::I(KWNE),
                Val::C(b"^="),
                Val::I(TKKEY),
                Val::I(KWNE),
                Val::C(b"<>"),
                Val::I(TKKEY),
                Val::I(KWNE),
                Val::C(b"<="),
                Val::I(TKKEY),
                Val::I(KWLE),
                Val::C(b">="),
                Val::I(TKKEY),
                Val::I(KWGE),
                Val::C(b"<"),
                Val::I(TKKEY),
                Val::I(KWLT),
                Val::C(b">"),
                Val::I(TKKEY),
                Val::I(KWGT),
                Val::C(b"="),
                Val::I(TKKEY),
                Val::I(KWEQ),
                Val::C(b"("),
                Val::I(TKLPAR),
                Val::I(0),
                Val::C(b")"),
                Val::I(TKRPAR),
                Val::I(0),
                Val::C(b","),
                Val::I(TKCOMA),
                Val::I(0),
                Val::C(b"."),
                Val::I(TKDOT),
                Val::I(0),
                Val::C(b"*"),
                Val::I(TKSTAR),
                Val::I(0),
            ]
            .into_iter();
            for I in intrinsics::range(1, NSPEC, 1) {
                fstr::assign(SPCSTR.get_mut(I), clist.next().unwrap().into_str());
                SPCTOK[I] = clist.next().unwrap().into_i32();
                SPCVAL[I] = clist.next().unwrap().into_i32();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        PASS1 = true;

        Self {
            CHR,
            SPCSTR,
            HDCHRS,
            KEYWDS,
            TLCHRS,
            TQUERY,
            CHCARD,
            CPTR,
            I,
            IDSPEC,
            J,
            KWVALS,
            L,
            LAST,
            LENGTH,
            NCHARS,
            NNUMS,
            NSTRS,
            PTR,
            ROOM,
            STATE,
            SPCTOK,
            SPCVAL,
            PASS1,
        }
    }
}

fn ISALPH(CHR: &[u8]) -> bool {
    let CHR = &CHR[..1 as usize];
    (((intrinsics::ICHAR(CHR) >= intrinsics::ICHAR(b"A"))
        && (intrinsics::ICHAR(CHR) <= intrinsics::ICHAR(b"Z")))
        || ((intrinsics::ICHAR(CHR) >= intrinsics::ICHAR(b"a"))
            && (intrinsics::ICHAR(CHR) <= intrinsics::ICHAR(b"z"))))
}

fn ISDIGT(CHR: &[u8]) -> bool {
    let CHR = &CHR[..1 as usize];
    ((intrinsics::ICHAR(CHR) >= intrinsics::ICHAR(b"0"))
        && (intrinsics::ICHAR(CHR) <= intrinsics::ICHAR(b"9")))
}

fn ISDOT(CHR: &[u8]) -> bool {
    let CHR = &CHR[..1 as usize];
    fstr::eq(CHR, b".")
}

fn ISSIGN(CHR: &[u8]) -> bool {
    let CHR = &CHR[..1 as usize];
    (fstr::eq(CHR, b"+") || fstr::eq(CHR, b"-"))
}

fn ISNUM(CHR: &[u8]) -> bool {
    let CHR = &CHR[..1 as usize];
    ((ISDIGT(CHR) || ISSIGN(CHR)) || fstr::eq(CHR, b"."))
}

fn ISQUOT(CHR: &[u8]) -> bool {
    let CHR = &CHR[..1 as usize];
    (fstr::eq(CHR, SQUOTE) || fstr::eq(CHR, DQUOTE))
}

//$Procedure      ZZEKSCAN ( EK, scan query )
pub fn ZZEKSCAN(
    QUERY: &[u8],
    MAXNTK: i32,
    MAXNUM: i32,
    NTOKEN: &mut i32,
    TOKENS: &mut [i32],
    LXBEGS: &mut [i32],
    LXENDS: &mut [i32],
    VALUES: &mut [i32],
    NUMVLS: &mut [f64],
    CHRBUF: &mut [u8],
    CHBEGS: &mut [i32],
    CHENDS: &mut [i32],
    SCNERR: &mut bool,
    ERRMSG: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TOKENS = DummyArrayMut::new(TOKENS, 1..);
    let mut LXBEGS = DummyArrayMut::new(LXBEGS, 1..);
    let mut LXENDS = DummyArrayMut::new(LXENDS, 1..);
    let mut VALUES = DummyArrayMut::new(VALUES, 1..);
    let mut NUMVLS = DummyArrayMut::new(NUMVLS, 1..);
    let mut CHBEGS = DummyArrayMut::new(CHBEGS, 1..);
    let mut CHENDS = DummyArrayMut::new(CHENDS, 1..);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Number of tokens made up of special characters:
    //

    //
    // Max length of any such token:
    //

    //
    // Local variables
    //

    //
    // Statement Functions
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // These keyword declarations must be made in alphabetical order!
    //

    //
    // The following tokens are sequences of special characters.  Some
    // of these are synonyms for keywords; some have other meanings.  In
    // this data statement, the longer sequences must precede the shorter
    // ones, in order for the matching algorithm to work properly.
    //

    //
    // Statement Function Definitions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZEKSCAN", ctx)?;
    }

    //
    // The first time through, set up our identifier character set.
    //
    if save.PASS1 {
        //
        // Each identifier must start with a letter (of either case).
        // The subsequent characters must be letters, numbers, dollar
        // signs or underscores.
        //
        fstr::assign(
            &mut save.HDCHRS,
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
        );

        fstr::assign(
            &mut save.TLCHRS,
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789$_",
        );

        SSIZEI(MXSPEC, save.IDSPEC.as_slice_mut(), ctx)?;
        LXCSID(&save.HDCHRS, &save.TLCHRS, save.IDSPEC.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // We'll work with a local copy of the query.
    //
    save.L = RTRIM(QUERY);
    fstr::assign(&mut save.TQUERY, fstr::substr(QUERY, 1..=save.L));

    //
    // Initialize pointers and counts.
    //
    save.CPTR = 1;
    save.NNUMS = 0;
    save.NSTRS = 0;
    save.CHCARD = 0;
    *NTOKEN = 0;

    //
    // Start out in the token search state.
    //
    save.STATE = NXTTOK;

    while (save.STATE != TERM) {
        if (save.STATE == NXTTOK) {
            //
            // In our initial state, we're looking for a new token.
            // We stop when we have enough characters to determine
            // which kind of token we have, or if we run out of
            // characters.
            //
            // Set our character pointer to the beginning of the next
            // token.
            //
            if (*NTOKEN > 0) {
                save.CPTR = (LXENDS[*NTOKEN] + 1);
            }

            if (save.CPTR > save.L) {
                save.STATE = TERM;
            } else {
                while (fstr::eq(fstr::substr(&save.TQUERY, save.CPTR..=save.CPTR), b" ")
                    && (save.CPTR < save.L))
                {
                    save.CPTR = (save.CPTR + 1);
                }

                if fstr::eq(fstr::substr(&save.TQUERY, save.CPTR..=save.CPTR), b" ") {
                    //
                    // We're out of non-blank characters to look at.
                    //
                    save.STATE = TERM;
                } else {
                    fstr::assign(
                        &mut save.CHR,
                        fstr::substr(&save.TQUERY, save.CPTR..=save.CPTR),
                    );
                    save.STATE = NEWTOK;
                }
            }
        //
        // STATE is in the set {NEWTOK, TERM}.
        //
        } else if (save.STATE == NEWTOK) {
            //
            // If we got this far, we have the initial character of
            // something that could be a valid token.  We test for
            //
            //    - quoted strings
            //    - numbers
            //    - identifiers
            //    - special symbols
            //
            // in that order.  Of course, we must have room in our output
            // arrays for the token.
            //
            if (*NTOKEN == MAXNTK) {
                fstr::assign(ERRMSG, b"Maximum allowed number of tokens is #; at least # tokens are present in QUERY.");
                REPMI(&ERRMSG.to_vec(), b"#", MAXNTK, ERRMSG, ctx);
                REPMI(&ERRMSG.to_vec(), b"#", (MAXNTK + 1), ERRMSG, ctx);

                *SCNERR = true;
                CHKOUT(b"ZZEKSCAN", ctx)?;
                return Ok(());
            }

            if ISQUOT(&save.CHR) {
                save.STATE = QSTR;
            } else if ISDOT(&save.CHR) {
                save.STATE = DOT;
            } else if ISNUM(&save.CHR) {
                save.STATE = NUMBER;
            } else if ISALPH(&save.CHR) {
                save.STATE = IDENT;
            } else {
                save.STATE = SPCIAL;
            }

        //
        // At this point, the next value of STATE has been determined.
        // STATE is in the set
        //
        //    {QSTR, NUMBER, IDENT, SPCIAL}
        //
        } else if (save.STATE == QSTR) {
            //
            // Look for a quoted string starting at location CPTR.
            // Use the current character as the quote character.
            //
            LXQSTR(
                &save.TQUERY,
                &save.CHR,
                save.CPTR,
                &mut save.LAST,
                &mut save.NCHARS,
            );

            if (save.NCHARS == 0) {
                fstr::assign(ERRMSG, b"Invalid quoted string at location #.");
                REPMI(&ERRMSG.to_vec(), b"#", save.CPTR, ERRMSG, ctx);

                *SCNERR = true;
                CHKOUT(b"ZZEKSCAN", ctx)?;
                return Ok(());
            }

            //
            // We've located a quoted string lexeme.  Parse the lexeme
            // and obtain the corresponding string value.  First make
            // sure we have enough room for the parsed string.
            //
            save.ROOM = (intrinsics::LEN(CHRBUF) - save.CHCARD);

            if (save.NCHARS > save.ROOM) {
                fstr::assign(ERRMSG, b"Insufficient space to store quoted string at location #; # chars needed; only # are available.");

                REPMI(&ERRMSG.to_vec(), b"#", save.CPTR, ERRMSG, ctx);
                REPMI(&ERRMSG.to_vec(), b"#", save.NCHARS, ERRMSG, ctx);
                REPMI(&ERRMSG.to_vec(), b"#", save.ROOM, ERRMSG, ctx);

                *SCNERR = true;
                CHKOUT(b"ZZEKSCAN", ctx)?;
                return Ok(());
            }

            PARSQS(
                fstr::substr(&save.TQUERY, save.CPTR..=((save.CPTR + save.NCHARS) - 1)),
                &save.CHR,
                fstr::substr_mut(CHRBUF, (save.CHCARD + 1)..),
                &mut save.LENGTH,
                SCNERR,
                ERRMSG,
                &mut save.PTR,
            );

            if *SCNERR {
                PREFIX(b"#", 2, ERRMSG);

                REPMC(
                    &ERRMSG.to_vec(),
                    b"#",
                    b"Error occurred while parsing quoted string token at location #:",
                    ERRMSG,
                );

                REPMI(&ERRMSG.to_vec(), b"#", save.CPTR, ERRMSG, ctx);

                *SCNERR = true;

                CHKOUT(b"ZZEKSCAN", ctx)?;
                return Ok(());
            }

            //
            // We've found a valid quoted string.  Set our outputs.
            //
            *NTOKEN = (*NTOKEN + 1);
            TOKENS[*NTOKEN] = TKQSTR;
            save.NSTRS = (save.NSTRS + 1);
            VALUES[*NTOKEN] = save.NSTRS;
            CHBEGS[save.NSTRS] = (save.CHCARD + 1);
            CHENDS[save.NSTRS] = (save.CHCARD + save.LENGTH);
            save.CHCARD = CHENDS[save.NSTRS];
            LXBEGS[*NTOKEN] = save.CPTR;
            LXENDS[*NTOKEN] = save.LAST;

            save.STATE = NXTTOK;

        //
        // STATE is now NXTTOK.
        //
        } else if (save.STATE == DOT) {
            //
            // The token begins with a period.  We could be looking at
            // a floating point number, or we could be looking at a
            // period in a compound identifier.
            //
            // Look for a number starting at location CPTR.
            //
            LX4NUM(
                &save.TQUERY,
                save.CPTR,
                &mut save.LAST,
                &mut save.NCHARS,
                ctx,
            );

            if (save.NCHARS > 0) {
                save.STATE = NUMBER;
            } else {
                save.STATE = SPCIAL;
            }

        //
        // STATE has been set to NUMBER or SPCIAL.  CPTR and NTOKEN
        // remain unchanged.
        //
        } else if (save.STATE == NUMBER) {
            //
            // Look for a number starting at location CPTR.
            //
            LX4NUM(
                &save.TQUERY,
                save.CPTR,
                &mut save.LAST,
                &mut save.NCHARS,
                ctx,
            );

            if (save.NCHARS == 0) {
                fstr::assign(ERRMSG, b"Invalid numeric token at location #.");
                REPMI(&ERRMSG.to_vec(), b"#", save.CPTR, ERRMSG, ctx);

                *SCNERR = true;
                CHKOUT(b"ZZEKSCAN", ctx)?;
                return Ok(());
            }

            //
            // Parse the token, but only do so if there's enough
            // room to store the result.
            //
            save.ROOM = (MAXNUM - save.NNUMS);

            if (save.ROOM < 1) {
                fstr::assign(ERRMSG, b"Insufficient space to store value of number at location #; # elements are available in the NUMVLS array; # are required.");

                REPMI(&ERRMSG.to_vec(), b"#", save.CPTR, ERRMSG, ctx);
                REPMI(&ERRMSG.to_vec(), b"#", MAXNUM, ERRMSG, ctx);
                REPMI(&ERRMSG.to_vec(), b"#", (MAXNUM + 1), ERRMSG, ctx);

                *SCNERR = true;
                CHKOUT(b"ZZEKSCAN", ctx)?;
                return Ok(());
            }

            NPARSD(
                fstr::substr(&save.TQUERY, save.CPTR..=save.LAST),
                &mut NUMVLS[(save.NNUMS + 1)],
                ERRMSG,
                &mut save.PTR,
                ctx,
            );

            if fstr::ne(ERRMSG, b" ") {
                //
                // This check is done for safety; by construction, we
                // should always have a valid number if LX4NUM
                // thinks we have a valid number, so in fact ERRMSG
                // should always be blank.
                //

                PREFIX(b"#", 2, ERRMSG);

                REPMC(
                    &ERRMSG.to_vec(),
                    b"#",
                    b"Error found in numeric token at location #:",
                    ERRMSG,
                );

                REPMI(
                    &ERRMSG.to_vec(),
                    b"#",
                    ((save.CPTR + save.PTR) - 1),
                    ERRMSG,
                    ctx,
                );

                *SCNERR = true;
                CHKOUT(b"ZZEKSCAN", ctx)?;
                return Ok(());
            }

            //
            // We found a valid numeric token.  We distinguish
            // between integers and d.p. numbers; set the token
            // to the most restrictive category possible.
            //
            *NTOKEN = (*NTOKEN + 1);

            if BEINT(fstr::substr(&save.TQUERY, save.CPTR..=save.LAST)) {
                TOKENS[*NTOKEN] = TKINT;
            } else {
                TOKENS[*NTOKEN] = TKDP;
            }

            //
            // Set the rest of our outputs.
            //
            save.NNUMS = (save.NNUMS + 1);
            VALUES[*NTOKEN] = save.NNUMS;
            LXBEGS[*NTOKEN] = save.CPTR;
            LXENDS[*NTOKEN] = save.LAST;

            save.STATE = NXTTOK;

        //
        // STATE is now NXTTOK.
        //
        } else if (save.STATE == IDENT) {
            //
            // Look for an identifier starting at location CPTR.
            //
            LXIDNT(
                save.IDSPEC.as_slice(),
                &save.TQUERY,
                save.CPTR,
                &mut save.LAST,
                &mut save.NCHARS,
            );

            if (save.NCHARS == 0) {
                //
                // This check is done for safety; by construction, we
                // should always have a valid identifier of at least one
                // character if we get to the IDENT state, so in fact
                // NCHARS should never equal zero.
                //
                fstr::assign(ERRMSG, b"Invalid identifier at location #.");
                REPMI(&ERRMSG.to_vec(), b"#", save.CPTR, ERRMSG, ctx);

                *SCNERR = true;
                CHKOUT(b"ZZEKSCAN", ctx)?;
                return Ok(());
            }

            //
            // We've located an identifier lexeme.  Make sure we have
            // enough room for the string.
            //
            save.ROOM = (intrinsics::LEN(CHRBUF) - save.CHCARD);

            if (save.NCHARS > save.ROOM) {
                fstr::assign(ERRMSG, b"Insufficient space to store identifier string at location #; # chars needed; only # are available.");

                REPMI(&ERRMSG.to_vec(), b"#", save.CPTR, ERRMSG, ctx);
                REPMI(&ERRMSG.to_vec(), b"#", save.NCHARS, ERRMSG, ctx);
                REPMI(&ERRMSG.to_vec(), b"#", save.ROOM, ERRMSG, ctx);

                *SCNERR = true;
                CHKOUT(b"ZZEKSCAN", ctx)?;
                return Ok(());
            }

            //
            // We've found a valid identifier or keyword.  Set our
            // outputs.  Convert the string to upper case.
            //
            *NTOKEN = (*NTOKEN + 1);

            UCASE(
                fstr::substr(&save.TQUERY, save.CPTR..=save.LAST),
                fstr::substr_mut(CHRBUF, (save.CHCARD + 1)..=(save.CHCARD + save.NCHARS)),
                ctx,
            );

            save.I = BSRCHC(
                fstr::substr(CHRBUF, (save.CHCARD + 1)..=(save.CHCARD + save.NCHARS)),
                NKEYWD,
                save.KEYWDS.as_arg(),
            );

            if (save.I > 0) {
                //
                // It's a keyword.
                //
                TOKENS[*NTOKEN] = TKKEY;
                VALUES[*NTOKEN] = save.KWVALS[save.I];
                LXBEGS[*NTOKEN] = save.CPTR;
                LXENDS[*NTOKEN] = save.LAST;

                save.STATE = NXTTOK;
            } else {
                //
                // It's an identifier.
                //
                save.NSTRS = (save.NSTRS + 1);
                CHBEGS[save.NSTRS] = (save.CHCARD + 1);
                CHENDS[save.NSTRS] = (save.CHCARD + save.NCHARS);
                save.CHCARD = CHENDS[save.NSTRS];
                TOKENS[*NTOKEN] = TKID;
                VALUES[*NTOKEN] = save.NSTRS;
                LXBEGS[*NTOKEN] = save.CPTR;
                LXENDS[*NTOKEN] = save.LAST;

                save.STATE = NXTTOK;

                //
                // We finished scanning an identifier.
                //
                // STATE is set to NXTTOK.
                //
            }
        //
        // We scanned a keyword or an identifier.
        //
        // STATE is set to NXTTOK.
        //
        } else if (save.STATE == SPCIAL) {
            //
            // Look for a valid token starting with a special character at
            // location CPTR. We attempt to match the longest possible
            // special token.
            //
            save.I = intrinsics::MIN0(&[SPCLEN, ((save.L - save.CPTR) + 1)]);
            save.J = 0;

            while ((save.I >= 1) && (save.J == 0)) {
                save.LAST = ((save.CPTR + save.I) - 1);
                save.J = ISRCHC(
                    fstr::substr(&save.TQUERY, save.CPTR..=save.LAST),
                    NSPEC,
                    save.SPCSTR.as_arg(),
                );

                if (save.J == 0) {
                    save.I = (save.I - 1);
                }
            }

            if (save.J > 0) {
                //
                // We've identified a valid token.
                //
                *NTOKEN = (*NTOKEN + 1);

                TOKENS[*NTOKEN] = save.SPCTOK[save.J];
                VALUES[*NTOKEN] = save.SPCVAL[save.J];
                LXBEGS[*NTOKEN] = save.CPTR;
                LXENDS[*NTOKEN] = ((save.CPTR - 1) + RTRIM(&save.SPCSTR[save.J]));

                save.STATE = NXTTOK;
            } else {
                fstr::assign(ERRMSG, b"Invalid character found at location #. ");
                REPMI(&ERRMSG.to_vec(), b"#", save.CPTR, ERRMSG, ctx);
                //
                // If the offending character is printable, include it
                // in the error message.  Otherwise, include the integer
                // code for the character.
                //
                if (FRSTPC(&save.CHR) > 0) {
                    SUFFIX(b"<character> = \'#\'", 2, ERRMSG);
                    REPMC(&ERRMSG.to_vec(), b"#", &save.CHR, ERRMSG);
                } else {
                    SUFFIX(b"ICHAR(<character>) = #", 2, ERRMSG);
                    REPMI(
                        &ERRMSG.to_vec(),
                        b"#",
                        intrinsics::ICHAR(&save.CHR),
                        ERRMSG,
                        ctx,
                    );
                }

                *SCNERR = true;
                CHKOUT(b"ZZEKSCAN", ctx)?;
                return Ok(());
            }
            //
            // STATE is now NXTTOK.
            //
        }
    }

    //
    // If we got this far, we've found the tokens in the query.
    //
    *SCNERR = false;
    fstr::assign(ERRMSG, b" ");

    CHKOUT(b"ZZEKSCAN", ctx)?;
    Ok(())
}
