//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const ROOM: i32 = 128;
const NMARKS: i32 = 6;
const STRSIZ: i32 = (2 * ROOM);

struct SaveVars {
    OP: ActualCharArray,
    TCLASS: ActualCharArray,
    TYPE: ActualCharArray,
    O: Vec<u8>,
    STRING: Vec<u8>,
    VALUE: f64,
    B: i32,
    BEG: StackArray<i32, 128>,
    BLANK: i32,
    CLASS: i32,
    DIV: i32,
    E: i32,
    END: StackArray<i32, 128>,
    EXP: i32,
    F: i32,
    I: i32,
    IDENT: StackArray<i32, 128>,
    SIZE: StackArray<i32, 6>,
    LPAREN: i32,
    MULT: i32,
    NEST: i32,
    NOP: i32,
    NTOKNS: i32,
    OPLEN: StackArray<i32, 6>,
    OPPTR: StackArray<i32, 20>,
    PASS: i32,
    RPAREN: i32,
    S: i32,
    START: i32,
    FIRST: bool,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut OP = ActualCharArray::new(2, 1..=NMARKS);
        let mut TCLASS = ActualCharArray::new(8, 1..=5);
        let mut TYPE = ActualCharArray::new(32, 0..=5);
        let mut O = vec![b' '; 256 as usize];
        let mut STRING = vec![b' '; 256 as usize];
        let mut VALUE: f64 = 0.0;
        let mut B: i32 = 0;
        let mut BEG = StackArray::<i32, 128>::new(1..=ROOM);
        let mut BLANK: i32 = 0;
        let mut CLASS: i32 = 0;
        let mut DIV: i32 = 0;
        let mut E: i32 = 0;
        let mut END = StackArray::<i32, 128>::new(1..=ROOM);
        let mut EXP: i32 = 0;
        let mut F: i32 = 0;
        let mut I: i32 = 0;
        let mut IDENT = StackArray::<i32, 128>::new(1..=ROOM);
        let mut SIZE = StackArray::<i32, 6>::new(0..=5);
        let mut LPAREN: i32 = 0;
        let mut MULT: i32 = 0;
        let mut NEST: i32 = 0;
        let mut NOP: i32 = 0;
        let mut NTOKNS: i32 = 0;
        let mut OPLEN = StackArray::<i32, 6>::new(1..=NMARKS);
        let mut OPPTR = StackArray::<i32, 20>::new(1..=20);
        let mut PASS: i32 = 0;
        let mut RPAREN: i32 = 0;
        let mut S: i32 = 0;
        let mut START: i32 = 0;
        let mut FIRST: bool = false;
        let mut FOUND: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"ANGLE"),
                Val::C(b"LENGTH"),
                Val::C(b"TIME"),
                Val::C(b"MASS"),
                Val::C(b"CHARGE"),
            ]
            .into_iter();
            TCLASS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FIRST = true;
        NOP = NMARKS;
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
            TCLASS,
            TYPE,
            O,
            STRING,
            VALUE,
            B,
            BEG,
            BLANK,
            CLASS,
            DIV,
            E,
            END,
            EXP,
            F,
            I,
            IDENT,
            SIZE,
            LPAREN,
            MULT,
            NEST,
            NOP,
            NTOKNS,
            OPLEN,
            OPPTR,
            PASS,
            RPAREN,
            S,
            START,
            FIRST,
            FOUND,
        }
    }
}

//$Procedure      CHANGU ( Change units )
pub fn CHANGU(
    ANGLE: &[u8],
    LENGTH: &[u8],
    TIME: &[u8],
    MASS: &[u8],
    CHARGE: &[u8],
    IN: &[u8],
    OUT: &[u8],
    ERROR: &[u8],
) {

    //
    // SPICELIB functions.
    //
    //
    // Local Parameters
    //

    //
    // Local Variables
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
    // Initial Values
    //
}

pub fn OUNITS(
    ANGLE: &[u8],
    LENGTH: &[u8],
    TIME: &[u8],
    MASS: &[u8],
    CHARGE: &[u8],
    ERROR: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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

    fstr::assign(save.TYPE.get_mut(0), b"1");
    fstr::assign(save.TYPE.get_mut(1), ANGLE);
    fstr::assign(save.TYPE.get_mut(2), LENGTH);
    fstr::assign(save.TYPE.get_mut(3), TIME);
    fstr::assign(save.TYPE.get_mut(4), MASS);
    fstr::assign(save.TYPE.get_mut(5), CHARGE);

    save.I = 1;
    fstr::assign(ERROR, b" ");

    while (save.I <= 5) {
        FNDUCV(
            &save.TYPE[save.I],
            &mut save.FOUND,
            &mut save.CLASS,
            &mut save.VALUE,
            ctx,
        )?;

        if !save.FOUND {
            fstr::assign(
                ERROR,
                &fstr::concat(b"Unrecognized unit: ", save.TYPE.get(save.I)),
            );
        } else if (save.CLASS != save.I) {
            spicelib::SUFFIX(b"The", 1, ERROR);
            spicelib::SUFFIX(&save.TCLASS[save.I], 1, ERROR);
            spicelib::SUFFIX(b"argument is \'", 1, ERROR);
            spicelib::SUFFIX(&save.TYPE[save.I], 1, ERROR);
            spicelib::SUFFIX(b"\'. This is not a unit ", 0, ERROR);
            spicelib::SUFFIX(b"of type", 1, ERROR);
            spicelib::SUFFIX(&save.TCLASS[save.I], 1, ERROR);
            spicelib::SUFFIX(b".", 0, ERROR);
        }

        save.I = (save.I + 1);
    }

    {
        let m1__: i32 = 0;
        let m2__: i32 = 5;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SIZE[save.I] = spicelib::LASTNB(&save.TYPE[save.I]);
            save.I += m3__;
        }
    }

    Ok(())
}

//
// Construct the units having the same dimensions as the input
// but that have fundamentals (angle, length, time, ... ) in the
// form that are expected by the calling program.
//
pub fn TRANSU(IN: &[u8], OUT: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(&mut save.STRING, IN);
    fstr::assign(&mut save.O, b" ");
    save.NEST = 0;
    save.START = 1;
    save.F = 0;

    spicelib::SCAN(
        &save.STRING,
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

    save.I = 1;

    while (save.I <= save.NTOKNS) {
        save.B = save.BEG[save.I];
        save.E = save.END[save.I];

        if (save.IDENT[save.I] == save.BLANK) {
            //
            // Don't do anything....
            //
        } else if (save.IDENT[save.I] != 0) {
            save.S = (save.F + 1);
            save.F = ((save.S + save.E) - save.B);
            fstr::assign(
                fstr::substr_mut(&mut save.O, save.S..=save.F),
                fstr::substr(&save.STRING, save.B..=save.E),
            );
            //
            // We have to excercise a bit of caution.  If this
            // is an exponentiation operation, we need to just copy
            // the exponent to the output string.
            //
            if (save.IDENT[save.I] == save.EXP) {
                save.NEST = 0;
                save.PASS = 0;

                while ((save.PASS < 1) || (save.NEST > 0)) {
                    save.I = (save.I + 1);
                    save.PASS = (save.PASS + 1);

                    save.B = save.BEG[save.I];
                    save.E = save.END[save.I];
                    save.S = (save.F + 1);
                    save.F = ((save.S + save.B) - save.E);

                    fstr::assign(
                        fstr::substr_mut(&mut save.O, save.S..=save.F),
                        fstr::substr(&save.STRING, save.B..=save.E),
                    );

                    if (save.IDENT[save.I] == save.RPAREN) {
                        save.NEST = (save.NEST - 1);
                    } else if (save.IDENT[save.I] == save.LPAREN) {
                        save.NEST = (save.NEST + 1);
                    }
                }
            }
        } else {
            //
            // If you get to this point, just copy the units
            // associated with the class of this token.
            //
            FNDUCV(
                fstr::substr(&save.STRING, save.B..=save.E),
                &mut save.FOUND,
                &mut save.CLASS,
                &mut save.VALUE,
                ctx,
            )?;

            save.S = (save.F + 1);
            save.F = ((save.SIZE[save.CLASS] - 1) + save.S);
            fstr::assign(
                fstr::substr_mut(&mut save.O, save.S..=save.F),
                save.TYPE.get(save.CLASS),
            );
        }

        save.I = (save.I + 1);
    }

    fstr::assign(OUT, &save.O);

    Ok(())
}
