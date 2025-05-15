//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NDIM: i32 = 5;
const MAXDIM: i32 = NDIM;
const LBCELL: i32 = -5;
const MAXPAR: i32 = 128;
const NUMBER: i32 = 0;
const ANGLE: i32 = 1;
const LENGTH: i32 = 2;
const TIME: i32 = 3;
const MASS: i32 = 4;
const CHARGE: i32 = 5;
const INPUT: i32 = 1;
const OUTPUT: i32 = 2;
const EXPIAT: f64 = 3.0;
const DIVIDE: f64 = 2.0;
const MULPLY: f64 = 1.0;
const NMARKS: i32 = 6;
const ROOM: i32 = 128;

struct SaveVars {
    OP: ActualCharArray,
    DIFF: f64,
    DIM: StackArray<f64, 6>,
    DIMEN: StackArray<f64, 6>,
    DIMENI: StackArray<f64, 6>,
    DIMENO: StackArray<f64, 6>,
    EXPONT: StackArray<f64, 134>,
    INVAL: f64,
    KEEP: f64,
    OPVAL: StackArray<f64, 6>,
    OUTVAL: f64,
    PARSED: StackArray<f64, 134>,
    VALUE: f64,
    ACTIVE: i32,
    B: i32,
    BEG: StackArray<i32, 128>,
    BLANK: i32,
    CLASS: i32,
    CLAS_S: StackArray<i32, 134>,
    DIV: i32,
    E: i32,
    END: StackArray<i32, 128>,
    EXP: i32,
    IDENT: StackArray<i32, 128>,
    J: i32,
    L: i32,
    LPAREN: i32,
    MULT: i32,
    NOP: i32,
    NTOKNS: i32,
    O: i32,
    OPLEN: StackArray<i32, 6>,
    OPPTR: StackArray<i32, 20>,
    R: i32,
    RPAREN: i32,
    START: i32,
    DONE: bool,
    FIRST: bool,
    KNOWN: bool,
    MOVE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut OP = ActualCharArray::new(2, 1..=NMARKS);
        let mut DIFF: f64 = 0.0;
        let mut DIM = StackArray::<f64, 6>::new(0..=MAXDIM);
        let mut DIMEN = StackArray::<f64, 6>::new(0..=MAXDIM);
        let mut DIMENI = StackArray::<f64, 6>::new(0..=MAXDIM);
        let mut DIMENO = StackArray::<f64, 6>::new(0..=MAXDIM);
        let mut EXPONT = StackArray::<f64, 134>::new(LBCELL..=MAXPAR);
        let mut INVAL: f64 = 0.0;
        let mut KEEP: f64 = 0.0;
        let mut OPVAL = StackArray::<f64, 6>::new(1..=NMARKS);
        let mut OUTVAL: f64 = 0.0;
        let mut PARSED = StackArray::<f64, 134>::new(LBCELL..=MAXPAR);
        let mut VALUE: f64 = 0.0;
        let mut ACTIVE: i32 = 0;
        let mut B: i32 = 0;
        let mut BEG = StackArray::<i32, 128>::new(1..=ROOM);
        let mut BLANK: i32 = 0;
        let mut CLASS: i32 = 0;
        let mut CLAS_S = StackArray::<i32, 134>::new(LBCELL..=MAXPAR);
        let mut DIV: i32 = 0;
        let mut E: i32 = 0;
        let mut END = StackArray::<i32, 128>::new(1..=ROOM);
        let mut EXP: i32 = 0;
        let mut IDENT = StackArray::<i32, 128>::new(1..=ROOM);
        let mut J: i32 = 0;
        let mut L: i32 = 0;
        let mut LPAREN: i32 = 0;
        let mut MULT: i32 = 0;
        let mut NOP: i32 = 0;
        let mut NTOKNS: i32 = 0;
        let mut O: i32 = 0;
        let mut OPLEN = StackArray::<i32, 6>::new(1..=NMARKS);
        let mut OPPTR = StackArray::<i32, 20>::new(1..=20);
        let mut R: i32 = 0;
        let mut RPAREN: i32 = 0;
        let mut START: i32 = 0;
        let mut DONE: bool = false;
        let mut FIRST: bool = false;
        let mut KNOWN: bool = false;
        let mut MOVE: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
            ]
            .into_iter();
            DIM.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
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
            DIFF,
            DIM,
            DIMEN,
            DIMENI,
            DIMENO,
            EXPONT,
            INVAL,
            KEEP,
            OPVAL,
            OUTVAL,
            PARSED,
            VALUE,
            ACTIVE,
            B,
            BEG,
            BLANK,
            CLASS,
            CLAS_S,
            DIV,
            E,
            END,
            EXP,
            IDENT,
            J,
            L,
            LPAREN,
            MULT,
            NOP,
            NTOKNS,
            O,
            OPLEN,
            OPPTR,
            R,
            RPAREN,
            START,
            DONE,
            FIRST,
            KNOWN,
            MOVE,
        }
    }
}

//$Procedure CONVRT_3  ( Convert Units )
pub fn CONVRT_3(
    XIN: f64,
    UNIN: &[u8],
    UNOUT: &[u8],
    XOUT: &mut f64,
    STATUS: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // These are the class id's for each of the various entities
    // that make up the variables of a unit.
    //

    //
    // These are the codes will will use for the various
    // operations.
    //

    //
    // Scanning Parameters
    //

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    //  Here is the range of       Character      ASCII code
    //  initial characters that    ---------      ----------
    //  will be used by the        ' '             32
    //  "known" marks.             '('             40
    //                             ')'             41
    //                             '*'             42
    //                             '/'             47
    //
    // So the required number of pointers is 47 - 32 + 5 = 20.
    //

    //
    // Saved Variables
    //
    //
    // Initial Values
    //

    //
    // The game is afoot!
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"CONVRT_3", ctx)?;
    }

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

        save.OPVAL[save.BLANK] = 0.0;
        save.OPVAL[save.LPAREN] = 0.0;
        save.OPVAL[save.RPAREN] = 0.0;
        save.OPVAL[save.MULT] = MULPLY;
        save.OPVAL[save.EXP] = EXPIAT;
        save.OPVAL[save.DIV] = DIVIDE;
    }

    //
    // First make sure that both UNIN and UNOUT are recognized
    // units.
    //
    if !UNITP(UNIN, ctx)? {
        *STATUS = 1;
        spicelib::CHKOUT(b"CONVRT_3", ctx)?;
        return Ok(());
    }

    if !UNITP(UNOUT, ctx)? {
        *STATUS = 2;
        spicelib::CHKOUT(b"CONVRT_3", ctx)?;
        return Ok(());
    }

    //
    // We will need to keep track of the dimensions associated
    // with both input and output units.
    //
    save.DIMENI[ANGLE] = 0.0;
    save.DIMENI[LENGTH] = 0.0;
    save.DIMENI[TIME] = 0.0;
    save.DIMENI[MASS] = 0.0;
    save.DIMENI[CHARGE] = 0.0;

    save.DIMENO[ANGLE] = 0.0;
    save.DIMENO[LENGTH] = 0.0;
    save.DIMENO[TIME] = 0.0;
    save.DIMENO[MASS] = 0.0;
    save.DIMENO[CHARGE] = 0.0;

    //
    // We need to parse both the input and output units, we
    // do that in the loop that ranges from INPUT to OUTPUT.
    //
    for INOUT in INPUT..=OUTPUT {
        //
        // Initialize the various pods we will need to use to
        // parse this set of units.
        //
        spicelib::SSIZED(MAXPAR, save.PARSED.as_slice_mut(), ctx)?;
        spicelib::SSIZEI(MAXPAR, save.CLAS_S.as_slice_mut(), ctx)?;
        spicelib::SSIZED(MAXPAR, save.EXPONT.as_slice_mut(), ctx)?;

        //
        // Zero out the dimension vector.
        //
        save.DIMEN[ANGLE] = 0.0;
        save.DIMEN[LENGTH] = 0.0;
        save.DIMEN[TIME] = 0.0;
        save.DIMEN[MASS] = 0.0;
        save.DIMEN[CHARGE] = 0.0;

        //
        // We haven't finished scanning this unit yet.
        //
        save.DONE = false;

        //
        // We are beginning a group now.  After beginning a group we
        // ALWAYS append 1,0,0 and MULTPLY, -1, 0 to the PARSED,
        // CLAS_S, and EXPONT pod.  Why ask why?  Well in this case
        // we do it because it makes the processing MUCH simpler
        // (you'll see).
        //
        spicelib::APPNDD(1.0, save.PARSED.as_slice_mut(), ctx)?;
        spicelib::APPNDI(0, save.CLAS_S.as_slice_mut(), ctx)?;
        spicelib::APPNDD(0.0, save.EXPONT.as_slice_mut(), ctx)?;

        spicelib::APPNDD(MULPLY, save.PARSED.as_slice_mut(), ctx)?;
        spicelib::APPNDI(-1, save.CLAS_S.as_slice_mut(), ctx)?;
        spicelib::APPNDD(0.0, save.EXPONT.as_slice_mut(), ctx)?;
        //
        // We'll start scanning this string from the first character.
        //
        save.START = 1;

        if (INOUT == INPUT) {
            spicelib::SCAN(
                UNIN,
                save.OP.as_arg(),
                save.OPLEN.as_slice(),
                save.OPPTR.as_slice(),
                ROOM,
                &mut save.START,
                &mut save.NTOKNS,
                save.IDENT.as_slice_mut(),
                save.BEG.as_slice_mut(),
                save.END.as_slice_mut(),
            );
        } else if (INOUT == OUTPUT) {
            spicelib::SCAN(
                UNOUT,
                save.OP.as_arg(),
                save.OPLEN.as_slice(),
                save.OPPTR.as_slice(),
                ROOM,
                &mut save.START,
                &mut save.NTOKNS,
                save.IDENT.as_slice_mut(),
                save.BEG.as_slice_mut(),
                save.END.as_slice_mut(),
            );
        }

        //
        // For as long as there are tokens to look at...
        //
        while (save.NTOKNS > 0) {
            //
            // ... examine each in turn, classify it and take
            // an appropriate action.
            //
            for I in 1..=save.NTOKNS {
                //
                // If we have a left parenthesis ...
                //
                if (save.IDENT[I] == save.LPAREN) {
                    //
                    // We are beginning a group now.  After beginning a
                    // group we ALWAYS append 1,0,0 and MULTPLY, -1, 0 to
                    // the PARSED, CLAS_S, and EXPONT pod.
                    //
                    PODBGD(save.PARSED.as_slice_mut(), ctx)?;
                    PODBGI(save.CLAS_S.as_slice_mut(), ctx)?;
                    PODBGD(save.EXPONT.as_slice_mut(), ctx)?;

                    spicelib::APPNDD(1.0, save.PARSED.as_slice_mut(), ctx)?;
                    spicelib::APPNDI(0, save.CLAS_S.as_slice_mut(), ctx)?;
                    spicelib::APPNDD(0.0, save.EXPONT.as_slice_mut(), ctx)?;

                    spicelib::APPNDD(MULPLY, save.PARSED.as_slice_mut(), ctx)?;
                    spicelib::APPNDI(-1, save.CLAS_S.as_slice_mut(), ctx)?;
                    spicelib::APPNDD(0.0, save.EXPONT.as_slice_mut(), ctx)?;

                //
                // ... or if we have an arithmetic operations
                //
                } else if (((save.IDENT[I] == save.MULT) || (save.IDENT[I] == save.DIV))
                    || (save.IDENT[I] == save.EXP))
                {
                    //
                    // Append the operation to the current group.
                    //
                    spicelib::APPNDD(save.OPVAL[save.IDENT[I]], save.PARSED.as_slice_mut(), ctx)?;
                    spicelib::APPNDI(-1, save.CLAS_S.as_slice_mut(), ctx)?;
                    spicelib::APPNDD(0.0, save.EXPONT.as_slice_mut(), ctx)?;

                //
                // ...or if we have a unit or number ...
                //
                } else if (save.IDENT[I] == 0) {
                    //
                    // Look up the class and value for this token,
                    // append them to the current group.
                    //
                    save.B = save.BEG[I];
                    save.E = save.END[I];

                    if (INOUT == INPUT) {
                        FNDUCV(
                            fstr::substr(UNIN, save.B..=save.E),
                            &mut save.KNOWN,
                            &mut save.CLASS,
                            &mut save.VALUE,
                            ctx,
                        )?;
                    } else if (INOUT == OUTPUT) {
                        FNDUCV(
                            fstr::substr(UNOUT, save.B..=save.E),
                            &mut save.KNOWN,
                            &mut save.CLASS,
                            &mut save.VALUE,
                            ctx,
                        )?;
                    }

                    spicelib::APPNDD(save.VALUE, save.PARSED.as_slice_mut(), ctx)?;
                    spicelib::APPNDI(save.CLASS, save.CLAS_S.as_slice_mut(), ctx)?;
                    spicelib::APPNDD(save.DIM[save.CLASS], save.EXPONT.as_slice_mut(), ctx)?;

                //
                // ...or if we have a right parenthesis, close off
                // this group by evaluating it, then close the group
                // and append the last value computed onto its list
                // of value/operation pairs.
                //
                } else if (save.IDENT[I] == save.RPAREN) {
                    //
                    // We are ending a group.  It's time to perform all
                    // indicated operations in this group.  Note the
                    // structure of a completed group is:
                    //
                    //   Value  OP  Value OP Value ... OP Value
                    //
                    // Thus all operations are at even slots in the
                    // group.  The scheme for evaluating this expression
                    // is: identify the next operation to perform (more on
                    // how to locate the operation in a minute);
                    //
                    //                      Do this one
                    //                      _____^______
                    //                     '            `
                    //  Value OP Value OP  Value OP Value  OP Value OP ...
                    //
                    // replace the three entries by the result.
                    //
                    //     Value OP Value OP  result OP Value OP  ...
                    //
                    // The hierarchy of operations is
                    //
                    //    1.) exponentiation in left to right order.
                    //
                    //    2.) multiplication and division in left
                    //        to right order.
                    //
                    // Since the parsing is from left to right, as we
                    // simplify subexpression, we can shift items left
                    // to fill in the gaps left by the operator and
                    // second value of the expression that was simplified.
                    //
                    // To do all this we must fist identify the beginning
                    // and ends of this group.
                    //
                    PODBED(save.PARSED.as_slice(), &mut save.B, &mut save.E, ctx)?;

                    //
                    // First handle exponentiation.  So far we haven't
                    // moved anything, the ACTIVE left operand is at B;
                    // the first operator is located at B+1.  We will let
                    // ATOP (at operator) be the logical flag that indicates
                    // whether J points to an operator or an operand.
                    //
                    save.MOVE = false;
                    save.ACTIVE = save.B;
                    save.J = (save.B + 1);

                    while (save.J <= save.E) {
                        if (save.PARSED[save.J] == EXPIAT) {
                            //
                            // We are going to simplify an expression
                            // of the form  X ** Y to its computed value.
                            // This means we will be freeing up room to
                            // move items to the left.
                            //
                            //
                            save.MOVE = true;

                            save.PARSED[save.ACTIVE] =
                                f64::powf(save.PARSED[save.ACTIVE], save.PARSED[(save.J + 1)]);
                            save.EXPONT[save.ACTIVE] =
                                (save.EXPONT[save.ACTIVE] * save.PARSED[(save.J + 1)]);
                        } else {
                            //
                            // If we are moving operators and right
                            // operands to the left, now is the time
                            // to do it.
                            //
                            if save.MOVE {
                                save.O = (save.ACTIVE + 1);
                                save.L = (save.ACTIVE + 2);
                                save.R = (save.J + 1);

                                save.PARSED[save.O] = save.PARSED[save.J];
                                save.CLAS_S[save.O] = save.CLAS_S[save.J];
                                save.EXPONT[save.O] = save.EXPONT[save.J];

                                save.PARSED[save.L] = save.PARSED[save.R];
                                save.CLAS_S[save.L] = save.CLAS_S[save.R];
                                save.EXPONT[save.L] = save.EXPONT[save.R];
                            }

                            save.ACTIVE = (save.ACTIVE + 2);
                        }

                        //
                        // Make J point to the next operator.
                        //
                        save.J = (save.J + 2);
                    }

                    //
                    // Next handle multiplication and division.
                    //
                    save.E = save.ACTIVE;
                    save.ACTIVE = save.B;
                    save.J = (save.B + 1);

                    while (save.J <= save.E) {
                        save.R = (save.J + 1);
                        save.CLASS = save.CLAS_S[save.R];

                        if (save.PARSED[save.J] == MULPLY) {
                            save.PARSED[save.ACTIVE] =
                                (save.PARSED[save.ACTIVE] * save.PARSED[save.R]);
                            save.DIMEN[save.CLASS] = (save.DIMEN[save.CLASS] + save.EXPONT[save.R]);
                        } else if (save.PARSED[save.J] == DIVIDE) {
                            save.PARSED[save.ACTIVE] =
                                (save.PARSED[save.ACTIVE] / save.PARSED[save.R]);
                            save.DIMEN[save.CLASS] = (save.DIMEN[save.CLASS] - save.EXPONT[save.R]);
                        }

                        save.J = (save.J + 2);
                    }

                    //
                    // Finally, save the first value of the group, end the
                    // group, and append the saved value to the previous
                    // group.
                    //
                    save.KEEP = save.PARSED[save.ACTIVE];

                    PODEGD(save.PARSED.as_slice_mut(), ctx)?;
                    PODEGI(save.CLAS_S.as_slice_mut(), ctx)?;
                    PODEGD(save.EXPONT.as_slice_mut(), ctx)?;

                    spicelib::APPNDD(save.KEEP, save.PARSED.as_slice_mut(), ctx)?;
                    spicelib::APPNDI(0, save.CLAS_S.as_slice_mut(), ctx)?;
                    spicelib::APPNDD(0.0, save.EXPONT.as_slice_mut(), ctx)?;
                }
            }

            //
            // Just in case there are any left-overs, scan the
            // string for more tokens
            //
            if (INOUT == INPUT) {
                spicelib::SCAN(
                    UNIN,
                    save.OP.as_arg(),
                    save.OPLEN.as_slice(),
                    save.OPPTR.as_slice(),
                    ROOM,
                    &mut save.START,
                    &mut save.NTOKNS,
                    save.IDENT.as_slice_mut(),
                    save.BEG.as_slice_mut(),
                    save.END.as_slice_mut(),
                );
            } else if (INOUT == OUTPUT) {
                spicelib::SCAN(
                    UNOUT,
                    save.OP.as_arg(),
                    save.OPLEN.as_slice(),
                    save.OPPTR.as_slice(),
                    ROOM,
                    &mut save.START,
                    &mut save.NTOKNS,
                    save.IDENT.as_slice_mut(),
                    save.BEG.as_slice_mut(),
                    save.END.as_slice_mut(),
                );
            }

            //
            // If there are no more tokens left, we need to be sure
            // to close the last group (the one we opened before we
            // had even begun to scan UNIN or UNOUT.
            //
            if ((save.NTOKNS == 0) && !save.DONE) {
                save.DONE = true;
                save.NTOKNS = 1;
                save.IDENT[1] = save.RPAREN;
            }
        }

        //
        // Put the result of the parse into the input or output storage
        // area as appropriate.
        //
        if (INOUT == INPUT) {
            save.DIMENI[ANGLE] = save.DIMEN[ANGLE];
            save.DIMENI[LENGTH] = save.DIMEN[LENGTH];
            save.DIMENI[TIME] = save.DIMEN[TIME];
            save.DIMENI[MASS] = save.DIMEN[MASS];
            save.DIMENI[CHARGE] = save.DIMEN[CHARGE];
            save.INVAL = save.PARSED[1];
        } else if (INOUT == OUTPUT) {
            save.DIMENO[ANGLE] = save.DIMEN[ANGLE];
            save.DIMENO[LENGTH] = save.DIMEN[LENGTH];
            save.DIMENO[TIME] = save.DIMEN[TIME];
            save.DIMENO[MASS] = save.DIMEN[MASS];
            save.DIMENO[CHARGE] = save.DIMEN[CHARGE];
            save.OUTVAL = save.PARSED[1];
        }

        //
        // Finally, if this is only the first of the units that needs to
        // be parsed, loop back through the code above a second time.
        //
    }

    //
    // One final check must be performed.  The input and output
    // units must be dimensionally equivalent.
    //

    for I in 1..=NDIM {
        if (save.DIMENI[I] != save.DIMENO[I]) {
            save.DIFF = (save.DIMENI[I] - save.DIMENO[I]);

            *STATUS = 3;
            spicelib::CHKOUT(b"CONVRT_3", ctx)?;
            return Ok(());
        }
    }

    //
    // That was the last hurdle,  now we can just compute the output.
    //
    *XOUT = ((save.INVAL / save.OUTVAL) * XIN);
    *STATUS = 0;

    spicelib::CHKOUT(b"CONVRT_3", ctx)?;
    Ok(())
}
