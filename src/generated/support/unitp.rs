//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NMARKS: i32 = 6;
const ROOM: i32 = 32;

struct SaveVars {
    OP: ActualCharArray,
    BLANK: i32,
    DIV: i32,
    EXP: i32,
    MULT: i32,
    LPAREN: i32,
    NOP: i32,
    OPLEN: StackArray<i32, 6>,
    RPAREN: i32,
    OPPTR: StackArray<i32, 20>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut OP = ActualCharArray::new(2, 1..=NMARKS);
        let mut BLANK: i32 = 0;
        let mut DIV: i32 = 0;
        let mut EXP: i32 = 0;
        let mut MULT: i32 = 0;
        let mut LPAREN: i32 = 0;
        let mut NOP: i32 = 0;
        let mut OPLEN = StackArray::<i32, 6>::new(1..=NMARKS);
        let mut RPAREN: i32 = 0;
        let mut OPPTR = StackArray::<i32, 20>::new(1..=20);
        let mut FIRST: bool = false;

        FIRST = true;
        NOP = 6;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b" "),
                Val::C(b"("),
                Val::C(b")"),
                Val::C(b"*"),
                Val::C(b"**"),
                Val::C(b"/"),
            ]
            .into_iter();
            OP.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            OP,
            BLANK,
            DIV,
            EXP,
            MULT,
            LPAREN,
            NOP,
            OPLEN,
            RPAREN,
            OPPTR,
            FIRST,
        }
    }
}

//$Procedure UNITP ( Determine whether a string represents units)
pub fn UNITP(STRING: &[u8], ctx: &mut Context) -> f2rust_std::Result<bool> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut UNITP: bool = false;
    let mut VALUE: f64 = 0.0;
    let mut B: i32 = 0;
    let mut BEG = StackArray::<i32, 32>::new(1..=ROOM);
    let mut CLASS: i32 = 0;
    let mut E: i32 = 0;
    let mut END = StackArray::<i32, 32>::new(1..=ROOM);
    let mut EXPLEV: i32 = 0;
    let mut IDENT = StackArray::<i32, 32>::new(1..=ROOM);
    let mut LASTTK: i32 = 0;
    let mut NEST: i32 = 0;
    let mut NTOKNS: i32 = 0;
    let mut START: i32 = 0;
    let mut EXPGRP: bool = false;
    let mut KNOWN: bool = false;
    let mut PHYSCL: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Parameters
    //

    //
    // Local variables
    //

    //
    //   Here is the range of       Character      ASCII code
    //   initial characters that    ---------      ----------
    //   will be used by the        ' '             32
    //   "known" marks.             '('             40
    //                              ')'             41
    //                              '*'             42
    //                              '/'             47
    //
    // So the required number of pointers is 47 - 32 + 5 = 20.
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // On the first pass through this routine, set up the stuff
    // required for scanning the input string.
    //
    if save.FIRST {
        save.FIRST = false;

        spicelib::SCANPR(
            &mut save.NOP,
            save.OP.as_arg_mut(),
            save.OPLEN.as_slice_mut(),
            save.OPPTR.as_slice_mut(),
        );

        save.BLANK = spicelib::BSRCHC(b" ", save.NOP, save.OP.as_arg());
        save.LPAREN = spicelib::BSRCHC(b"(", save.NOP, save.OP.as_arg());
        save.RPAREN = spicelib::BSRCHC(b")", save.NOP, save.OP.as_arg());
        save.MULT = spicelib::BSRCHC(b"*", save.NOP, save.OP.as_arg());
        save.EXP = spicelib::BSRCHC(b"**", save.NOP, save.OP.as_arg());
        save.DIV = spicelib::BSRCHC(b"/", save.NOP, save.OP.as_arg());
    }

    //
    // To get started we will assume that the last token (before we
    // started looking at the string) was an introductory left
    // parenthesis.
    //
    LASTTK = save.LPAREN;
    NEST = 0;
    PHYSCL = false;
    EXPGRP = false;

    START = 1;

    spicelib::SCAN(
        STRING,
        save.OP.as_arg(),
        save.OPLEN.as_slice(),
        save.OPPTR.as_slice(),
        ROOM,
        &mut START,
        &mut NTOKNS,
        IDENT.as_slice_mut(),
        BEG.as_slice_mut(),
        END.as_slice_mut(),
    );

    while (NTOKNS > 0) {
        for I in 1..=NTOKNS {
            //
            // Look at the identity of the next token ...
            //
            if (IDENT[I] == 0) {
                //
                // A non-recognized item cannot follow a right parenthesis
                // or a non-recognized item.
                //
                if ((LASTTK == save.RPAREN) || (LASTTK == 0)) {
                    UNITP = false;
                    return Ok(UNITP);
                }

                //
                // So far, so good.  Determine whether this object is
                // a recognized unit or number.
                //
                B = BEG[I];
                E = END[I];

                FNDUCV(
                    fstr::substr(STRING, B..=E),
                    &mut KNOWN,
                    &mut CLASS,
                    &mut VALUE,
                    ctx,
                )?;

                //
                // If it wasn't recognized we don't have a unit.
                //
                if !KNOWN {
                    UNITP = false;
                    return Ok(UNITP);
                }

                //
                // We also need to make sure we don't have anything of
                // the form **UNIT or **( ... UNIT ... ) where UNIT is a
                // physical unit.
                //
                if (CLASS > 0) {
                    if ((LASTTK == save.EXP) || EXPGRP) {
                        UNITP = false;
                        return Ok(UNITP);
                    }
                }

                //
                // Finally, we need to keep track of whether or not
                // we've seen a physical unit.
                //
                PHYSCL = (PHYSCL || (CLASS > 0));
            } else if (IDENT[I] == save.RPAREN) {
                //
                // A right parenthesis can only follow a right parenthesis,
                // a unit or a number.
                //
                if ((LASTTK != 0) && (LASTTK != save.RPAREN)) {
                    UNITP = false;
                    return Ok(UNITP);
                }

                NEST = (NEST - 1);
            } else if (((IDENT[I] == save.EXP) || (IDENT[I] == save.MULT))
                || (IDENT[I] == save.DIV))
            {
                //
                // An arithmetic operation can only follow a right
                // parenthesis, a unit or a number.
                //
                if ((LASTTK != save.RPAREN) && (LASTTK != 0)) {
                    UNITP = false;
                    return Ok(UNITP);
                }
            } else if (IDENT[I] == save.LPAREN) {
                //
                // A left parenthesis must be the first thing in the
                // string or follow one of the following:
                //
                //       '(', '*', '**', '/'
                //
                // (Note by construction the last token prior to the
                // beginning of the string was '(' ).  If this is _not_
                // the case then this is not a unit.
                //
                if ((((LASTTK != save.LPAREN) && (LASTTK != save.MULT)) && (LASTTK != save.DIV))
                    && (LASTTK != save.EXP))
                {
                    UNITP = false;
                    return Ok(UNITP);
                }

                //
                // If the last token was exponentiation (and we were not
                // already in some exponentiation group), we can't have
                // anything but numbers until the nesting level returns
                // to the current level.
                //
                if ((LASTTK == save.EXP) && !EXPGRP) {
                    EXPLEV = NEST;
                    EXPGRP = true;
                }

                //
                // Increase the nesting level of the expression.
                //
                NEST = (NEST + 1);
            } else if (IDENT[I] == save.BLANK) {

                //
                // Don't do anything.
                //
            }

            //
            // Copy the identity of this token.
            //
            LASTTK = IDENT[I];

            //
            // Now for a few quick checks.  If the nesting level ever drops
            // below zero, we don't have a unit.
            //
            if (NEST < 0) {
                UNITP = false;
                return Ok(UNITP);
            }

            //
            // We need to see if its ok to relax the restriction on the
            // use of physical units.
            //
            if EXPGRP {
                EXPGRP = (NEST > EXPLEV);
            }
        }

        //
        // Just in case we didn't get everything the first time,
        // scan the string again.
        //
        spicelib::SCAN(
            STRING,
            save.OP.as_arg(),
            save.OPLEN.as_slice(),
            save.OPPTR.as_slice(),
            ROOM,
            &mut START,
            &mut NTOKNS,
            IDENT.as_slice_mut(),
            BEG.as_slice_mut(),
            END.as_slice_mut(),
        );
    }

    //
    // One last check.  If we didn't get a physical unit somewhere in
    // the string or if the nesting did not return to zero, we don't
    // have a unit.
    //
    if (NEST == 0) {
        UNITP = PHYSCL;
    } else {
        UNITP = false;
    }

    Ok(UNITP)
}
