//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBPOOL: i32 = -5;
pub const LINLEN: i32 = 132;
pub const MAXLEN: i32 = 32;
const BEGIN: i32 = 1;
const DONE: i32 = (BEGIN + 1);
const INVAR: i32 = (DONE + 1);
const Q: i32 = 1;
const NQ: i32 = (Q + 1);
const BV: i32 = (NQ + 1);
const EV: i32 = (BV + 1);
const EQ: i32 = (EV + 1);
const EQP: i32 = (EQ + 1);
const STRTYP: i32 = 1;
const NUMTYP: i32 = (STRTYP + 1);
const UNKNWN: i32 = (NUMTYP + 1);
const PREV: i32 = 2;
const NEXT: i32 = 1;
const FILSIZ: i32 = 255;

struct SaveVars {
    IBLANK: i32,
    ICOMMA: i32,
    IEQUAL: i32,
    ILPARN: i32,
    IPLUS: i32,
    IQUOTE: i32,
    IRPARN: i32,
    ITAB: i32,
    ITMARK: i32,
    LINE: Vec<u8>,
    NAME: Vec<u8>,
    ERROR: Vec<u8>,
    DVALUE: f64,
    AT: i32,
    B: i32,
    BADAT: i32,
    BEGS: StackArray<i32, 132>,
    CHNODE: i32,
    CODE: i32,
    COUNT: i32,
    DATAHD: i32,
    DIRCTV: i32,
    DPNODE: i32,
    E: i32,
    ENDS: StackArray<i32, 132>,
    FREE: i32,
    HEAD: i32,
    I: i32,
    J: i32,
    LOOKAT: i32,
    NAMEAT: i32,
    NCOMP: i32,
    NODE: i32,
    NXTTOK: i32,
    R1: i32,
    R2: i32,
    STATUS: i32,
    TAIL: i32,
    TYPE: StackArray<i32, 132>,
    VARLEN: i32,
    VARTYP: i32,
    EVEN: bool,
    INTOKN: bool,
    INQUOT: bool,
    INSEPF: bool,
    FULL: bool,
    FOUND: bool,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut IBLANK: i32 = 0;
        let mut ICOMMA: i32 = 0;
        let mut IEQUAL: i32 = 0;
        let mut ILPARN: i32 = 0;
        let mut IPLUS: i32 = 0;
        let mut IQUOTE: i32 = 0;
        let mut IRPARN: i32 = 0;
        let mut ITAB: i32 = 0;
        let mut ITMARK: i32 = 0;
        let mut LINE = vec![b' '; LINLEN as usize];
        let mut NAME = vec![b' '; LINLEN as usize];
        let mut ERROR = vec![b' '; 256 as usize];
        let mut DVALUE: f64 = 0.0;
        let mut AT: i32 = 0;
        let mut B: i32 = 0;
        let mut BADAT: i32 = 0;
        let mut BEGS = StackArray::<i32, 132>::new(1..=LINLEN);
        let mut CHNODE: i32 = 0;
        let mut CODE: i32 = 0;
        let mut COUNT: i32 = 0;
        let mut DATAHD: i32 = 0;
        let mut DIRCTV: i32 = 0;
        let mut DPNODE: i32 = 0;
        let mut E: i32 = 0;
        let mut ENDS = StackArray::<i32, 132>::new(1..=LINLEN);
        let mut FREE: i32 = 0;
        let mut HEAD: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut LOOKAT: i32 = 0;
        let mut NAMEAT: i32 = 0;
        let mut NCOMP: i32 = 0;
        let mut NODE: i32 = 0;
        let mut NXTTOK: i32 = 0;
        let mut R1: i32 = 0;
        let mut R2: i32 = 0;
        let mut STATUS: i32 = 0;
        let mut TAIL: i32 = 0;
        let mut TYPE = StackArray::<i32, 132>::new(1..=LINLEN);
        let mut VARLEN: i32 = 0;
        let mut VARTYP: i32 = 0;
        let mut EVEN: bool = false;
        let mut INTOKN: bool = false;
        let mut INQUOT: bool = false;
        let mut INSEPF: bool = false;
        let mut FULL: bool = false;
        let mut FOUND: bool = false;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            IBLANK,
            ICOMMA,
            IEQUAL,
            ILPARN,
            IPLUS,
            IQUOTE,
            IRPARN,
            ITAB,
            ITMARK,
            LINE,
            NAME,
            ERROR,
            DVALUE,
            AT,
            B,
            BADAT,
            BEGS,
            CHNODE,
            CODE,
            COUNT,
            DATAHD,
            DIRCTV,
            DPNODE,
            E,
            ENDS,
            FREE,
            HEAD,
            I,
            J,
            LOOKAT,
            NAMEAT,
            NCOMP,
            NODE,
            NXTTOK,
            R1,
            R2,
            STATUS,
            TAIL,
            TYPE,
            VARLEN,
            VARTYP,
            EVEN,
            INTOKN,
            INQUOT,
            INSEPF,
            FULL,
            FOUND,
            FIRST,
        }
    }
}

fn ISSEP(
    CODE: i32,
    IBLANK: i32,
    ICOMMA: i32,
    ILPARN: i32,
    IRPARN: i32,
    IEQUAL: i32,
    ITAB: i32,
) -> bool {
    ((((((CODE == IBLANK) || (CODE == ICOMMA)) || (CODE == ILPARN)) || (CODE == IRPARN))
        || (CODE == IEQUAL))
        || (CODE == ITAB))
}

fn ISQUOT(CODE: i32, IQUOTE: i32) -> bool {
    (CODE == IQUOTE)
}

fn ISEQU(CODE: i32, IEQUAL: i32) -> bool {
    (CODE == IEQUAL)
}

fn ISRPAR(CODE: i32, IRPARN: i32) -> bool {
    (CODE == IRPARN)
}

fn ISLPAR(CODE: i32, ILPARN: i32) -> bool {
    (CODE == ILPARN)
}

fn ISPLUS(CODE: i32, IPLUS: i32) -> bool {
    (CODE == IPLUS)
}

fn ISTIME(CODE: i32, ITMARK: i32) -> bool {
    (CODE == ITMARK)
}

fn ISBAD(DIRCTV: i32) -> bool {
    ((DIRCTV != EQ) && (DIRCTV != EQP))
}

//$Procedure ZZRVBF ( Private --- Pool, read the next buffer variable )
pub fn ZZRVBF(
    BUFFER: CharArray,
    BSIZE: i32,
    LINNUM: &mut i32,
    NAMLST: &mut [i32],
    NMPOOL: &mut [i32],
    NAMES: CharArrayMut,
    DATLST: &mut [i32],
    DPPOOL: &mut [i32],
    DPVALS: &mut [f64],
    CHPOOL: &mut [i32],
    CHVALS: CharArrayMut,
    VARNAM: &mut [u8],
    EOF: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let BUFFER = DummyCharArray::new(BUFFER, None, 1..);
    let mut NAMLST = DummyArrayMut::new(NAMLST, 1..);
    let mut NMPOOL = DummyArrayMut2D::new(NMPOOL, 1..=2, LBPOOL..);
    let mut NAMES = DummyCharArrayMut::new(NAMES, None, 1..);
    let mut DATLST = DummyArrayMut::new(DATLST, 1..);
    let mut DPPOOL = DummyArrayMut2D::new(DPPOOL, 1..=2, LBPOOL..);
    let mut DPVALS = DummyArrayMut::new(DPVALS, 1..);
    let mut CHPOOL = DummyArrayMut2D::new(CHPOOL, 1..=2, LBPOOL..);
    let mut CHVALS = DummyCharArrayMut::new(CHVALS, None, 1..);

    //
    //
    // SPICELIB functions
    //

    //
    // Local parameters.
    //
    // Below are a collection of enumerated lists that are used
    // to discern what part of the processing we are in and what
    // kind of entity we are dealing with.  First the overall
    // processing flow of a variable assignment.
    //

    //
    // Next we have the various types of tokens that can be found
    // in the parsing of an input line
    //
    // Q   --- quoted (or protected tokens)
    // NQ  --- unquoted tokens
    // BV  --- beginning of a vector
    // EV  --- ending of a vector
    // EQ  --- equal sign
    // EQP --- equal sign plus
    //

    //
    // A variable can have one of three types as we process
    // it.  It can have an unknown type UNKNWN, STRTYP or NUMTYP.
    //
    //

    //
    // The next two parameters indicate which component of a linked
    // list node point to the previous node and the next node.
    //

    //
    // The next collection of variables are set up in first pass
    // through this routine.  They would be parameters if FORTRAN
    // allowed us to do this in a standard way.
    //

    //
    // The logicals below are used to take apart the tokens in an
    // input line.
    //

    //
    // The following logicals are in-line functions that are used
    // when processing the input strings.
    //
    //
    // Save everything.
    //

    //
    // Below are a collection of In-line function definitions that are
    // intended to make the code a bit easier to write and read.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZRVBF", ctx)?;
    }
    //
    // Initializations.
    //
    if save.FIRST {
        save.FIRST = false;
        save.ICOMMA = intrinsics::ICHAR(b",");
        save.IBLANK = intrinsics::ICHAR(b" ");
        save.IQUOTE = intrinsics::ICHAR(b"\'");
        save.ILPARN = intrinsics::ICHAR(b"(");
        save.IRPARN = intrinsics::ICHAR(b")");
        save.IEQUAL = intrinsics::ICHAR(b"=");
        save.IPLUS = intrinsics::ICHAR(b"+");
        save.ITMARK = intrinsics::ICHAR(b"@");
        save.ITAB = 9;
    }

    //
    // No variable yet and no parsing errors so far.
    //
    fstr::assign(&mut save.NAME, b" ");
    fstr::assign(&mut save.ERROR, b" ");
    save.NCOMP = 0;

    //
    // Get the next data line. Unless something is terribly wrong,
    // this will begin a new variable definition. We have to read
    // the whole variable, unless we get an error, in which case
    // we can quit.
    //
    save.STATUS = BEGIN;

    while ((save.STATUS != DONE) && !FAILED(ctx)) {
        fstr::assign(&mut save.LINE, b" ");
        //
        // We need to skip blank lines...
        //
        while fstr::eq(&save.LINE, b" ") {
            *EOF = (*LINNUM > BSIZE);

            if *EOF {
                CHKOUT(b"ZZRVBF", ctx)?;
                return Ok(());
            }

            fstr::assign(&mut save.LINE, BUFFER.get(*LINNUM));
            *LINNUM = (*LINNUM + 1);
        }

        //
        // Find the "tokens" in the input line. As you scan from left
        // to right along the line, exactly one of the following
        // conditions is true.
        //
        // 1) You are in a separator field
        // 4) You are in a quoted substring
        // 5) You are in a non-quoted substring that isn't a separator
        //    field.
        //
        // Stuff between separator fields are regarded as tokens.  Note
        // this includes quoted strings.
        //
        // In addition we keep track of 3 separators: '=', '(', ')'
        // Finally, whenever we encounters the separator '=', we back
        // up and see if it is preceded by a '+', if so we attach
        // it to the '=' and treat the pair of characters as a single
        // separator.
        //

        save.EVEN = true;
        save.INTOKN = false;
        save.INQUOT = false;
        save.INSEPF = true;

        save.COUNT = 0;
        save.I = 0;

        while (save.I < intrinsics::LEN(&save.LINE)) {
            //
            // The current character is either a separator, quote or
            // some other character.
            //
            save.I = (save.I + 1);
            save.CODE = intrinsics::ICHAR(fstr::substr(&save.LINE, save.I..=save.I));

            if ISSEP(
                save.CODE,
                save.IBLANK,
                save.ICOMMA,
                save.ILPARN,
                save.IRPARN,
                save.IEQUAL,
                save.ITAB,
            ) {
                //
                // There are 3 possible states we could be in
                //    Separation Field
                //    A quoted substring with the last quote an odd one.
                //    A quoted substring with the last quote an even one.
                //    A non-quoted token.
                // In the first two cases nothing changes, but in the
                // next two cases we transition to a separation field.
                //
                if (save.INTOKN || (save.INQUOT && save.EVEN)) {
                    save.INQUOT = false;
                    save.INTOKN = false;
                    save.INSEPF = true;
                }

                if save.INSEPF {
                    //
                    // We need to see if this is one of the special
                    // separators
                    //
                    if ISEQU(save.CODE, save.IEQUAL) {
                        save.COUNT = (save.COUNT + 1);
                        save.BEGS[save.COUNT] = save.I;
                        save.TYPE[save.COUNT] = EQ;
                        save.ENDS[save.COUNT] = save.I;

                        if (save.I > 1) {
                            //
                            // Look back at the previous character.
                            // See if it is a plus character.
                            //
                            save.CODE = intrinsics::ICHAR(fstr::substr(
                                &save.LINE,
                                (save.I - 1)..=(save.I - 1),
                            ));

                            if ISPLUS(save.CODE, save.IPLUS) {
                                //
                                // This is the directive '+=' we need
                                // to set the beginning of this token
                                // to the one before this and adjust
                                // the end of the last token.
                                //
                                save.TYPE[save.COUNT] = EQP;
                                save.BEGS[save.COUNT] = (save.I - 1);

                                if (save.BEGS[(save.COUNT - 1)] == save.ENDS[(save.COUNT - 1)]) {
                                    save.COUNT = (save.COUNT - 1);
                                    save.BEGS[save.COUNT] = (save.I - 1);
                                    save.ENDS[save.COUNT] = save.I;
                                    save.TYPE[save.COUNT] = EQP;
                                } else {
                                    save.ENDS[(save.COUNT - 1)] = (save.ENDS[(save.COUNT - 1)] - 1);
                                }
                            }
                        }
                    } else if ISRPAR(save.CODE, save.IRPARN) {
                        save.COUNT = (save.COUNT + 1);
                        save.BEGS[save.COUNT] = save.I;
                        save.ENDS[save.COUNT] = save.I;
                        save.TYPE[save.COUNT] = EV;
                    } else if ISLPAR(save.CODE, save.ILPARN) {
                        save.COUNT = (save.COUNT + 1);
                        save.BEGS[save.COUNT] = save.I;
                        save.ENDS[save.COUNT] = save.I;
                        save.TYPE[save.COUNT] = BV;
                    }
                }
            } else if ISQUOT(save.CODE, save.IQUOTE) {
                //
                // There are 3 cases of interest.
                //    We are in a quoted substring already
                //    We are in a separator field
                //    We are in a non-quoted token.
                // In the first case nothing changes.  In the second
                // two cases we change to being in a quoted substring.
                //
                save.EVEN = !save.EVEN;

                if !save.INQUOT {
                    save.INSEPF = false;
                    save.INTOKN = false;
                    save.INQUOT = true;
                    save.COUNT = (save.COUNT + 1);
                    save.BEGS[save.COUNT] = save.I;
                    save.TYPE[save.COUNT] = Q;
                }

                save.ENDS[save.COUNT] = save.I;
            } else {
                //
                // This is some character other than a quote, or
                // separator character.
                //
                // We are in one of four situations.
                //
                //    1) We are in a quoted substring with an odd number of
                //       quotes.
                //    2) We are in a quoted substring with an even number of
                //       quotes.
                //    2) We are in a separator field
                //    3) We are in a non-quoted token.
                //
                // In cases 1 and 3 nothing changes. So we won't check
                // those cases.
                //

                if (save.INSEPF || (save.INQUOT && save.EVEN)) {
                    save.INQUOT = false;
                    save.INSEPF = false;
                    save.INTOKN = true;
                    save.COUNT = (save.COUNT + 1);
                    save.BEGS[save.COUNT] = save.I;
                    save.TYPE[save.COUNT] = NQ;
                }

                save.ENDS[save.COUNT] = save.I;
            }
        }

        //
        // The first word on the first line should be the name of a
        // variable. The second word should be a directive: = or +=.
        //
        if (save.STATUS == BEGIN) {
            //
            // There must be at least 3 contributing tokens on this line.
            //
            if (save.COUNT < 3) {
                SETMSG(b"A kernel variable was not properly formed on line # text buffer.Such an assignment should have the form: \'<variable name> [+]= <values>\'. This line was \'#\'. ", ctx);

                save.R2 = RTRIM(&save.LINE);

                ERRINT(b"#", *LINNUM, ctx);
                ERRCH(b"#", fstr::substr(&save.LINE, 1..=save.R2), ctx);
                SIGERR(b"SPICE(BADVARASSIGN)", ctx)?;
                CHKOUT(b"ZZRVBF", ctx)?;
                return Ok(());
            }
            //
            // See if the variable name is legitimate:
            //
            save.BADAT = LASTPC(fstr::substr(&save.LINE, save.BEGS[1]..=save.ENDS[1]));

            if (save.BADAT <= (save.ENDS[1] - save.BEGS[1])) {
                //
                // There is a non-printing character in the variable
                // name.  This isn't allowed.
                //
                save.AT = (save.BEGS[1] + save.BADAT);

                SETMSG(b"There is a non-printing character embedded in line # of the text buffer.  Non-printing characters are not allowed in kernel variable assignments.  The non-printing character has ASCII code #. ", ctx);

                ERRINT(b"#", *LINNUM, ctx);
                ERRINT(
                    b"#",
                    intrinsics::ICHAR(fstr::substr(&save.LINE, save.AT..=save.AT)),
                    ctx,
                );
                SIGERR(b"SPICE(NONPRINTINGCHAR)", ctx)?;
                CHKOUT(b"ZZRVBF", ctx)?;
                return Ok(());
            }

            //
            // Check the variable name length; signal an error
            // if longer than MAXLEN.
            //
            save.VARLEN = intrinsics::LEN(fstr::substr(&save.LINE, save.BEGS[1]..=save.ENDS[1]));

            if (save.VARLEN > MAXLEN) {
                SETMSG(b"A kernel pool variable name in the input buffer exceeds the maximum allowed length #1. The actual length of the variable name is #2, the offending variable name to #3 characters: \'#4\'.", ctx);

                ERRINT(b"#1", MAXLEN, ctx);
                ERRINT(b"#2", save.VARLEN, ctx);
                ERRINT(b"#3", LINLEN, ctx);
                ERRCH(
                    b"#4",
                    fstr::substr(&save.LINE, save.BEGS[1]..=save.ENDS[1]),
                    ctx,
                );
                SIGERR(b"SPICE(BADVARNAME)", ctx)?;
            }

            //
            // The variable name is ok. How about the directive.
            //
            fstr::assign(
                VARNAM,
                fstr::substr(&save.LINE, save.BEGS[1]..=save.ENDS[1]),
            );
            save.DIRCTV = save.TYPE[2];

            //
            // If this is replacement (=) and not an addition (+=),
            // delete the values currently associated with the variable.
            // They will be replaced later.
            //
            if ISBAD(save.DIRCTV) {
                SETMSG(b"A kernel variable was not properly formed on line # of the text buffer. Such an assignment should have the form: \'<variable name> [+]= <values>\'.  More specifically, the assignment operator did not have one of the expected forms: \'=\' or \'+=\'. The line was \'#\'. ", ctx);

                save.R2 = RTRIM(&save.LINE);

                ERRINT(b"#", *LINNUM, ctx);
                ERRCH(b"#", fstr::substr(&save.LINE, 1..=save.R2), ctx);
                SIGERR(b"SPICE(BADVARASSIGN)", ctx)?;
                CHKOUT(b"ZZRVBF", ctx)?;
                return Ok(());
            }
            //
            // Locate this variable name in the name pool or insert it
            // if it isn't there.  The location will be NAMEAT and
            // we will use the variable FOUND to indicate whether or
            // not it was already present.
            //
            save.LOOKAT = ZZHASH(VARNAM, ctx)?;
            save.NODE = NAMLST[save.LOOKAT];
            save.FULL = (LNKNFN(NMPOOL.as_slice()) <= 0);
            save.FOUND = false;
            //
            // See if this name (or one colliding with it in the
            // hash scheme) has already been stored in the name list.
            //
            if (save.NODE > 0) {
                save.HEAD = save.NODE;
                save.TAIL = -NMPOOL[[PREV, save.HEAD]];

                while ((save.NODE > 0) && !save.FOUND) {
                    save.FOUND = fstr::eq(NAMES.get(save.NODE), VARNAM);
                    save.NAMEAT = save.NODE;
                    save.NODE = NMPOOL[[NEXT, save.NODE]];
                }

                if (!save.FOUND && !save.FULL) {
                    //
                    // We didn't find this name on the conflict resolution
                    // list. Allocate a new slot for it.
                    //
                    LNKAN(NMPOOL.as_slice_mut(), &mut save.NODE, ctx)?;
                    LNKILA(save.TAIL, save.NODE, NMPOOL.as_slice_mut(), ctx)?;

                    fstr::assign(NAMES.get_mut(save.NODE), VARNAM);
                    save.NAMEAT = save.NODE;
                }
            } else if !save.FULL {
                //
                // Nothing like this variable name (in the hashing sense)
                // has been loaded so far.  We need to allocate
                // a name slot for this variable.
                //
                LNKAN(NMPOOL.as_slice_mut(), &mut save.NODE, ctx)?;

                NAMLST[save.LOOKAT] = save.NODE;
                fstr::assign(NAMES.get_mut(save.NODE), VARNAM);
                save.NAMEAT = save.NODE;
            }
            //
            // If the name pool was full and we didn't find this name
            // we've got an error. Diagnose it and return.
            //
            if (save.FULL && !save.FOUND) {
                SETMSG(b"The kernel pool does not have room for any more variables.  It filled up at line # of the text buffer. ", ctx);

                ERRINT(b"#", *LINNUM, ctx);
                SIGERR(b"SPICE(KERNELPOOLFULL)", ctx)?;
                CHKOUT(b"ZZRVBF", ctx)?;
                return Ok(());
            }

            //
            // Now depending upon the kind of directive, we will need
            // to remove data and allocate a new list or simply append
            // data to the existing list.
            //
            if (save.DIRCTV == EQ) {
                //
                // We are going to dump whatever is associated with
                // this name and then we will need to allocate a new
                // linked list for the data.
                //
                save.VARTYP = UNKNWN;

                if save.FOUND {
                    //
                    // We need to free the data associated with this
                    // variable.
                    //
                    save.DATAHD = DATLST[save.NAMEAT];
                    DATLST[save.NAMEAT] = 0;

                    if (save.DATAHD < 0) {
                        //
                        // This variable was character type we need to
                        // free a linked list from the character data
                        // pool.
                        //
                        save.HEAD = -save.DATAHD;
                        save.TAIL = -CHPOOL[[PREV, save.HEAD]];

                        LNKFSL(save.HEAD, save.TAIL, CHPOOL.as_slice_mut(), ctx)?;
                    } else {
                        //
                        // This variable was numeric type. We need to
                        // free a linked list from the numeric pool.
                        //
                        save.HEAD = save.DATAHD;
                        save.TAIL = -DPPOOL[[PREV, save.HEAD]];

                        LNKFSL(save.HEAD, save.TAIL, DPPOOL.as_slice_mut(), ctx)?;
                    }
                }
            } else if (save.DIRCTV == EQP) {
                //
                // We need to append to the current variable.
                //
                if save.FOUND {
                    if (DATLST[save.NAMEAT] > 0) {
                        save.VARTYP = NUMTYP;
                    } else if (DATLST[save.NAMEAT] < 0) {
                        save.VARTYP = STRTYP;
                    } else {
                        save.VARTYP = UNKNWN;
                    }
                } else {
                    save.VARTYP = UNKNWN;
                }
            }

            //
            // If this is a vector, the next thing on the line will be a
            // left parenthesis. Otherwise, assume that this is a scalar.
            // If it's a vector, get the first value. If it's a scalar,
            // plant a bogus right parenthesis, to make the following loop
            // terminate after one iteration.
            //

            if (save.TYPE[3] == BV) {
                save.NXTTOK = 4;
            } else {
                save.NXTTOK = 3;
                save.COUNT = (save.COUNT + 1);
                save.TYPE[save.COUNT] = EV;
            }

        //
        // For subsequent lines, treat everything as a new value.
        //
        } else {
            save.NXTTOK = 1;
        }

        //
        // We have a value anyway. Store it in the table.
        //
        // Keep going until the other shoe (the right parenthesis)
        // drops, or until the end of the line is reached.
        //
        // Dates begin with @; anything else is presumed to be a number.
        //
        while ((save.TYPE[save.NXTTOK] != EV) && (save.NXTTOK <= save.COUNT)) {
            //
            // Get the begin and end of this token.
            //
            save.B = save.BEGS[save.NXTTOK];
            save.E = save.ENDS[save.NXTTOK];

            if (save.VARTYP == UNKNWN) {
                //
                // We need to determine which category of variable we
                // have by looking at this token and deducing the
                // type.
                //
                if (save.TYPE[save.NXTTOK] == Q) {
                    save.VARTYP = STRTYP;
                } else if (save.TYPE[save.NXTTOK] == NQ) {
                    save.VARTYP = NUMTYP;
                } else {
                    //
                    // This is an error. We should have had one of the
                    // two previous types.
                    //
                    // First perform the clean up function.
                    //
                    ZZCLN(
                        save.LOOKAT,
                        save.NAMEAT,
                        NAMLST.as_slice_mut(),
                        DATLST.as_slice_mut(),
                        NMPOOL.as_slice_mut(),
                        CHPOOL.as_slice_mut(),
                        DPPOOL.as_slice_mut(),
                        ctx,
                    )?;

                    SETMSG(b"The first item following the assignment operator should be the value of a variable or a left parenthesis \'(\' followed by a value for a variable. This is not true on line # of the text buffer. ", ctx);

                    ERRINT(b"#", *LINNUM, ctx);
                    SIGERR(b"SPICE(BADVARASSIGN)", ctx)?;
                    CHKOUT(b"ZZRVBF", ctx)?;
                    return Ok(());
                }
            }

            if (save.VARTYP == STRTYP) {
                //
                // First make sure that this token represents a string.
                //
                if (save.TYPE[save.NXTTOK] != Q) {
                    //
                    // First perform the clean up function.
                    //
                    ZZCLN(
                        save.LOOKAT,
                        save.NAMEAT,
                        NAMLST.as_slice_mut(),
                        DATLST.as_slice_mut(),
                        NMPOOL.as_slice_mut(),
                        CHPOOL.as_slice_mut(),
                        DPPOOL.as_slice_mut(),
                        ctx,
                    )?;

                    save.R1 = RTRIM(VARNAM);

                    SETMSG(b"The kernel variable # has been set up as a string variable.  However, the value that you are attempting to assign to this variable on line # of the text buffer is not a string value. ", ctx);

                    ERRCH(b"#", fstr::substr(VARNAM, 1..=save.R1), ctx);
                    ERRINT(b"#", *LINNUM, ctx);
                    SIGERR(b"SPICE(TYPEMISMATCH)", ctx)?;
                    CHKOUT(b"ZZRVBF", ctx)?;
                    return Ok(());
                }
                //
                // Still going? Make sure there is something between
                // the quotes.
                //
                if ((save.B + 1) >= save.E) {
                    //
                    // First perform the clean up function.
                    //
                    ZZCLN(
                        save.LOOKAT,
                        save.NAMEAT,
                        NAMLST.as_slice_mut(),
                        DATLST.as_slice_mut(),
                        NMPOOL.as_slice_mut(),
                        CHPOOL.as_slice_mut(),
                        DPPOOL.as_slice_mut(),
                        ctx,
                    )?;

                    SETMSG(b"There is a quoted string with no characters on line # of the text buffer. ", ctx);

                    ERRINT(b"#", *LINNUM, ctx);
                    SIGERR(b"SPICE(TYPEMISMATCH)", ctx)?;
                    CHKOUT(b"ZZRVBF", ctx)?;
                    return Ok(());
                }

                //
                // We are ready to go.  Allocate a node for this data
                // item. First make sure there is room to do so.
                //
                save.FREE = LNKNFN(CHPOOL.as_slice());

                if (save.FREE <= 0) {
                    SETMSG(b"There is no room available for adding another character value to the kernel pool.  The character values buffer became full at line # of the text buffer. ", ctx);

                    ERRINT(b"#", *LINNUM, ctx);
                    SIGERR(b"SPICE(KERNELPOOLFULL)", ctx)?;
                    CHKOUT(b"ZZRVBF", ctx)?;
                    return Ok(());
                }
                //
                // Allocate a node for storing this string value:
                //
                LNKAN(CHPOOL.as_slice_mut(), &mut save.CHNODE, ctx)?;

                if (DATLST[save.NAMEAT] == 0) {
                    //
                    // There was no data for this name yet.  We make
                    // CHNODE be the head of the data list for this name.
                    //
                    DATLST[save.NAMEAT] = -save.CHNODE;
                } else {
                    //
                    // Put this node after the tail of the current list.
                    //
                    save.HEAD = -DATLST[save.NAMEAT];
                    save.TAIL = -CHPOOL[[PREV, save.HEAD]];

                    LNKILA(save.TAIL, save.CHNODE, CHPOOL.as_slice_mut(), ctx)?;
                }
                //
                // Finally insert this data item in the data buffer
                // at CHNODE.  Note any quotes will be doubled so we
                // have to undo this affect when we store the data.
                //
                fstr::assign(CHVALS.get_mut(save.CHNODE), b" ");
                save.NCOMP = (save.NCOMP + 1);

                save.I = 1;
                save.J = (save.B + 1);

                while (save.J < save.E) {
                    save.CODE = intrinsics::ICHAR(fstr::substr(&save.LINE, save.J..=save.J));

                    if ISQUOT(save.CODE, save.IQUOTE) {
                        save.J = (save.J + 1);
                    }

                    fstr::assign(
                        fstr::substr_mut(CHVALS.get_mut(save.CHNODE), save.I..=save.I),
                        fstr::substr(&save.LINE, save.J..=save.J),
                    );
                    save.I = (save.I + 1);
                    save.J = (save.J + 1);
                }
            //
            // That's all for this value. It's now time to loop
            // back through and get the next value.
            //
            } else {
                if (save.TYPE[save.NXTTOK] != NQ) {
                    //
                    // First perform the clean up function.
                    //
                    ZZCLN(
                        save.LOOKAT,
                        save.NAMEAT,
                        NAMLST.as_slice_mut(),
                        DATLST.as_slice_mut(),
                        NMPOOL.as_slice_mut(),
                        CHPOOL.as_slice_mut(),
                        DPPOOL.as_slice_mut(),
                        ctx,
                    )?;

                    save.R1 = RTRIM(VARNAM);

                    SETMSG(b"The kernel variable # has been set up as a numeric or time variable.  However, the value that you are attempting to assign to this variable on line # of the kernel buffer is not a numeric or time value. ", ctx);

                    ERRCH(b"#", fstr::substr(VARNAM, 1..=save.R1), ctx);
                    ERRINT(b"#", *LINNUM, ctx);
                    SIGERR(b"SPICE(TYPEMISMATCH)", ctx)?;
                    CHKOUT(b"ZZRVBF", ctx)?;
                    return Ok(());
                }
                //
                // Look at the first character to see if we have a time
                // or a number.
                //
                save.CODE = intrinsics::ICHAR(fstr::substr(&save.LINE, save.B..=save.B));

                if ISTIME(save.CODE, save.ITMARK) {
                    //
                    // We need to have more than a single character.
                    //
                    if (save.E == save.B) {
                        //
                        // First perform the clean up function.
                        //
                        ZZCLN(
                            save.LOOKAT,
                            save.NAMEAT,
                            NAMLST.as_slice_mut(),
                            DATLST.as_slice_mut(),
                            NMPOOL.as_slice_mut(),
                            CHPOOL.as_slice_mut(),
                            DPPOOL.as_slice_mut(),
                            ctx,
                        )?;
                        save.R1 = RTRIM(VARNAM);

                        SETMSG(b"At character # of  line # in the text buffer the character \'@\' appears.  This character is reserved for identifying time values in assignments to kernel pool variables.  However it is not being used in this fashion for the variable \'#\'. ", ctx);

                        ERRINT(b"#", save.B, ctx);
                        ERRINT(b"#", *LINNUM, ctx);
                        ERRCH(b"#", fstr::substr(VARNAM, 1..=save.R1), ctx);
                        SIGERR(b"SPICE(BADTIMESPEC)", ctx)?;
                        CHKOUT(b"ZZRVBF", ctx)?;
                        return Ok(());
                    }

                    TPARSE(
                        fstr::substr(&save.LINE, (save.B + 1)..=save.E),
                        &mut save.DVALUE,
                        &mut save.ERROR,
                        ctx,
                    )?;

                    if fstr::ne(&save.ERROR, b" ") {
                        //
                        // First perform the clean up function.
                        //
                        ZZCLN(
                            save.LOOKAT,
                            save.NAMEAT,
                            NAMLST.as_slice_mut(),
                            DATLST.as_slice_mut(),
                            NMPOOL.as_slice_mut(),
                            CHPOOL.as_slice_mut(),
                            DPPOOL.as_slice_mut(),
                            ctx,
                        )?;

                        SETMSG(b"Encountered \'#\' while attempting to parse a time on line # of the text buffer. ", ctx);

                        ERRCH(b"#", fstr::substr(&save.LINE, (save.B + 1)..=save.E), ctx);
                        ERRINT(b"#", *LINNUM, ctx);
                        SIGERR(b"SPICE(BADTIMESPEC)", ctx)?;
                        CHKOUT(b"ZZRVBF", ctx)?;
                        return Ok(());
                    }
                } else {
                    NPARSD(
                        fstr::substr(&save.LINE, save.B..=save.E),
                        &mut save.DVALUE,
                        &mut save.ERROR,
                        &mut save.I,
                        ctx,
                    );

                    if fstr::ne(&save.ERROR, b" ") {
                        ZZCLN(
                            save.LOOKAT,
                            save.NAMEAT,
                            NAMLST.as_slice_mut(),
                            DATLST.as_slice_mut(),
                            NMPOOL.as_slice_mut(),
                            CHPOOL.as_slice_mut(),
                            DPPOOL.as_slice_mut(),
                            ctx,
                        )?;

                        SETMSG(b"Encountered \'#\' while attempting to parse a number on line # of the text buffer", ctx);
                        ERRCH(b"#", fstr::substr(&save.LINE, save.B..=save.E), ctx);
                        ERRINT(b"#", *LINNUM, ctx);
                        SIGERR(b"SPICE(NUMBEREXPECTED)", ctx)?;
                        CHKOUT(b"ZZRVBF", ctx)?;
                        return Ok(());
                    }
                }
                //
                // OK. We have a parsed value.  See if there is room in
                // the numeric portion of the pool to store this value.
                //
                save.FREE = LNKNFN(DPPOOL.as_slice());

                if (save.FREE <= 0) {
                    SETMSG(b"There is no room available for adding another numeric value to the kernel pool.  The numeric values buffer became full at line # of the text buffer.", ctx);

                    ERRINT(b"#", *LINNUM, ctx);
                    SIGERR(b"SPICE(KERNELPOOLFULL)", ctx)?;
                    CHKOUT(b"ZZRVBF", ctx)?;
                    return Ok(());
                }

                //
                // Allocate a node for storing this numeric value:
                //
                LNKAN(DPPOOL.as_slice_mut(), &mut save.DPNODE, ctx)?;

                if (DATLST[save.NAMEAT] == 0) {
                    //
                    // There was no data for this name yet.  We make
                    // DPNODE be the head of the data list for this name.
                    //
                    DATLST[save.NAMEAT] = save.DPNODE;
                } else {
                    //
                    // Put this node after the tail of the current list.
                    //
                    save.HEAD = DATLST[save.NAMEAT];
                    save.TAIL = -DPPOOL[[PREV, save.HEAD]];

                    LNKILA(save.TAIL, save.DPNODE, DPPOOL.as_slice_mut(), ctx)?;
                }
                //
                // Finally insert this data item into the numeric buffer.
                //
                DPVALS[save.DPNODE] = save.DVALUE;
                save.NCOMP = (save.NCOMP + 1);
            }

            //
            // Now process the next token in the list of tokens.
            //
            save.NXTTOK = (save.NXTTOK + 1);
        }
        //
        // We could have ended the above loop in one of two ways.
        //
        // 1) NXTTOK now exceeds count.  This means we did not reach
        //    an end of vector marker.
        // 2) We hit an end of vector marker.
        //
        if (save.NXTTOK > save.COUNT) {
            save.STATUS = INVAR;
        } else {
            save.STATUS = DONE;
        }
    }

    //
    // It is possible that we reached this point without actually
    // assigning a value to the kernel pool variable.  This can
    // happen if there is a vector input of the form NAME = ( )
    //
    if (save.NCOMP < 1) {
        ZZCLN(
            save.LOOKAT,
            save.NAMEAT,
            NAMLST.as_slice_mut(),
            DATLST.as_slice_mut(),
            NMPOOL.as_slice_mut(),
            CHPOOL.as_slice_mut(),
            DPPOOL.as_slice_mut(),
            ctx,
        )?;

        SETMSG(b"The first item following the assignment operator should be the value of a variable or a left parenthesis \'(\' followed by a value for a variable. This is not true on line # of the text buffer. ", ctx);

        ERRINT(b"#", (*LINNUM - 1), ctx);
        SIGERR(b"SPICE(BADVARASSIGN)", ctx)?;
        CHKOUT(b"ZZRVBF", ctx)?;
        return Ok(());
    }

    //
    // Return the name of the variable.
    //
    fstr::assign(&mut save.NAME, VARNAM);

    CHKOUT(b"ZZRVBF", ctx)?;
    Ok(())
}
