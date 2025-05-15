//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXSIG: i32 = 14;
const ESGN: i32 = (MAXSIG + 4);
const EFST: i32 = (ESGN + 1);
const ESCD: i32 = (EFST + 1);
const WDSIZE: i32 = (MAXSIG + 16);
const YES: bool = true;
const NO: bool = false;

struct SaveVars {
    STRING: Vec<u8>,
    MYFILL: Vec<u8>,
    LETTER: Vec<u8>,
    BLANK: i32,
    CODE: i32,
    CODE0: i32,
    EXP: i32,
    I: i32,
    J: i32,
    LSUB: i32,
    SLOT: i32,
    VALUE: i32,
    MINUS: bool,
    INCR: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STRING = vec![b' '; WDSIZE as usize];
        let mut MYFILL = vec![b' '; 1 as usize];
        let mut LETTER = vec![b' '; 1 as usize];
        let mut BLANK: i32 = 0;
        let mut CODE: i32 = 0;
        let mut CODE0: i32 = 0;
        let mut EXP: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut LSUB: i32 = 0;
        let mut SLOT: i32 = 0;
        let mut VALUE: i32 = 0;
        let mut MINUS: bool = false;
        let mut INCR: bool = false;

        fstr::assign(&mut STRING, b" 0.0000000000000E+00 ");
        EXP = 0;
        fstr::assign(&mut MYFILL, b" ");

        Self {
            STRING,
            MYFILL,
            LETTER,
            BLANK,
            CODE,
            CODE0,
            EXP,
            I,
            J,
            LSUB,
            SLOT,
            VALUE,
            MINUS,
            INCR,
        }
    }
}

//$Procedure       ZZVSTRNG ( Virtual String )
pub fn ZZVSTRNG(
    X: f64,
    FILL: &[u8],
    FROM: i32,
    TO: i32,
    RND: bool,
    EXPONT: i32,
    SUBSTR: &[u8],
    DID: bool,
) {

    //
    // Local Variables
    //

    //
    // Although we don't anticipate ever needing these values
    // we set some initial values for EXP and STRING.
    //
    //
    // This routine doesn't do anything.
    //
}

//$Procedure      ZZVSTSTR ( Set Virtual String)
pub fn ZZVSTSTR(X: f64, FILL: &[u8], EXPONT: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let FILL = &FILL[..1 as usize];

    fstr::assign(&mut save.MYFILL, FILL);

    DPSTR(X, MAXSIG, &mut save.STRING, ctx);

    //
    // Parse the exponent, string looks like the pattern presented
    // below:
    //
    //                MAXSIG + 2
    //                |
    //                v
    // by.xxxxxxxxxxxxxEsxxx
    // 1234567890123456789
    //                  ^^
    //                  ||
    //                  |EFST = ESGN + 1
    //                  |
    //                  ESGN = MAXSIG + 4
    //

    save.CODE0 = intrinsics::ICHAR(b"0");
    save.BLANK = intrinsics::ICHAR(b" ");

    save.MINUS =
        (intrinsics::ICHAR(fstr::substr(&save.STRING, ESGN..=ESGN)) == intrinsics::ICHAR(b"-"));
    save.CODE = intrinsics::ICHAR(fstr::substr(&save.STRING, EFST..=EFST));
    save.EXP = (save.CODE - save.CODE0);

    save.I = ESCD;
    save.CODE = intrinsics::ICHAR(fstr::substr(&save.STRING, save.I..=save.I));

    while (save.CODE != save.BLANK) {
        save.EXP = ((save.EXP * 10) + (save.CODE - save.CODE0));
        save.I = (save.I + 1);
        save.CODE = intrinsics::ICHAR(fstr::substr(&save.STRING, save.I..=save.I));
    }

    if save.MINUS {
        save.EXP = -save.EXP;
    }

    *EXPONT = save.EXP;
}

//$Procedure      ZZVSBSTR ( Virtual String Character )
pub fn ZZVSBSTR(
    FROM: i32,
    TO: i32,
    RND: bool,
    SUBSTR: &mut [u8],
    DID: &mut bool,
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // The buffered numeric string has the form:
    //
    //   by.xxxxxxxxxxxxxEseee...
    //   123456789012345678901234
    //            1         2
    //
    // Ignoring the exponent we can regard this as being the
    // decimal equivalent of the number with the decimal point
    // in the wrong position.  We'll need to remedy this.
    //
    //   by.xxxxxxxxxxxxx
    //   1234567890123456
    //            1
    //
    //  We can think of this decimal representation as being a
    //  simplification of the "infinite string" representation
    //  below.
    //
    //                 b    y   .   x   x       x
    //    d-4 d-3 d-2 d-1  d00  p  d01 d02 ... d13  0   0   0   0
    //    -2  -1   0   1    2   3   4   5       16
    //
    //
    // From this its clear that i'th digit can be easily computed
    // via following decision block.
    //
    //
    //    if ( i .lt. 0 ) then
    //       digit  = '0'
    //    else if ( i .eq. 0 ) then
    //       digit  = string(2:2)
    //    else if ( i .lt. maxsig ) then
    //       digit = string(i+3:i+3)
    //    else
    //       digit = '0'
    //    end if
    //
    // To have an accurate representation of the number (one that
    // accounts for the exponent) we shift the decimal point ('p')
    // "right" by EXP slots. (If EXP is negative we shift right a
    // negative number of slots).  In the sequence of characters the
    // decimal point will follow d_EXP.
    //
    // IF we renumber the slots so that the decimal point is in
    // slot 0 then for S < 0 slot S contains digit d_EXP+1+S
    //
    // For S > 0 slot S contains digit d_EXP+S
    //
    // Combining these observations we can compute the SLOT'th character
    // of the virtual string as follows.
    //
    //
    // If the character requested is character zero of the virtual
    // string, we just get the decimal point.
    //
    // If the character requested is in a slot whose index is
    // greater than zero it is to the
    // right of the decimal point so it must be D_exp+slot.
    //
    // If the character requested is in a slot whose index is negative
    // it is to the left of the decimal point.  Since the slot
    // just to the left of the decimal point contains D_exp it follows
    // by induction that for any negative slot, the decimal is
    // D_exp+slot+1
    //
    //
    // Since we may need to round the output, we will work from right
    // to left.  First thing we do is get the index of the right most
    // significant portion of SUBSTR that we will manipulate.
    //
    save.J = ((TO - FROM) + 1);
    save.LSUB = intrinsics::LEN(SUBSTR);
    //
    // Blank pad to the right of J (if there's anything to pad).
    //
    if (save.J < save.LSUB) {
        fstr::assign(fstr::substr_mut(SUBSTR, (save.J + 1)..), b" ");
    }
    //
    // If we need to round the output string, locate the first numeric
    // slot after TO.
    //
    if RND {
        save.SLOT = (TO + 1);
        //
        // If this points to the decimal point, move one more to the
        // right.
        //
        if (save.SLOT == 0) {
            save.SLOT = (save.SLOT + 1);
        }
        //
        // Determine which digit D_i corresponds to SLOT.
        //
        if (save.SLOT < 0) {
            save.I = ((save.EXP + save.SLOT) + 1);
        } else {
            save.I = (save.EXP + save.SLOT);
        }
        //
        // We will need to round in D_i is 5 or more.
        //
        if (save.I < 0) {
            fstr::assign(&mut save.LETTER, b"0");
        } else if (save.I == 0) {
            fstr::assign(&mut save.LETTER, fstr::substr(&save.STRING, 2..=2));
        } else if (save.I < MAXSIG) {
            fstr::assign(
                &mut save.LETTER,
                fstr::substr(&save.STRING, (3 + save.I)..=(3 + save.I)),
            );
        } else {
            fstr::assign(&mut save.LETTER, b"0");
        }

        save.INCR = fstr::ge(&save.LETTER, b"5");
    } else {
        save.INCR = NO;
    }
    //
    // Starting at the right most slot, we work left incrementing
    // digits as required.  Note that once we don't round up
    // some value, we are done incrementing.
    //
    {
        let m1__: i32 = TO;
        let m2__: i32 = FROM;
        let m3__: i32 = -1;
        save.SLOT = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (save.SLOT == 0) {
                fstr::assign(&mut save.LETTER, b".");
            } else {
                //
                // Otherwise we need to first see which digit, d_I, is being
                // requested.
                //
                if (save.SLOT < 0) {
                    save.I = ((save.EXP + save.SLOT) + 1);
                } else {
                    save.I = (save.EXP + save.SLOT);
                }

                //
                // Now just look up d_I according to the rule we established
                // earlier.
                //
                if (save.I < 0) {
                    //
                    // If the SLOT is prior to the first significant character
                    // or the virtual string, we use the fill character.
                    // Otherwise we use a zero.
                    //

                    if save.INCR {
                        fstr::assign(&mut save.LETTER, b"1");
                        save.INCR = NO;
                    } else {
                        if (save.SLOT < -1) {
                            fstr::assign(&mut save.LETTER, &save.MYFILL);
                        } else {
                            fstr::assign(&mut save.LETTER, b"0");
                        }
                    }
                } else if (save.I == 0) {
                    fstr::assign(&mut save.LETTER, fstr::substr(&save.STRING, 2..=2));
                    //
                    // If necessary, increment LETTER.
                    //
                    if save.INCR {
                        save.VALUE = ((intrinsics::ICHAR(&save.LETTER) - save.CODE0) + 1);
                        //
                        // If value is 10 or more we will need to
                        // increment the next character too.  If VALUE
                        // is less than 10, we are done incrementing set
                        // INCR to NO.
                        //
                        if (save.VALUE == 10) {
                            fstr::assign(&mut save.LETTER, b"0");
                        } else {
                            fstr::assign(
                                &mut save.LETTER,
                                &intrinsics::CHAR((save.VALUE + save.CODE0)),
                            );
                            save.INCR = NO;
                        }
                    }
                } else if (save.I < MAXSIG) {
                    //
                    // This case is virtually identical to the previous
                    // case, except that we need to pick off a different
                    // letter from STRING.
                    //
                    fstr::assign(
                        &mut save.LETTER,
                        fstr::substr(&save.STRING, (save.I + 3)..=(save.I + 3)),
                    );

                    if save.INCR {
                        save.VALUE = ((intrinsics::ICHAR(&save.LETTER) - save.CODE0) + 1);

                        if (save.VALUE == 10) {
                            fstr::assign(&mut save.LETTER, b"0");
                        } else {
                            fstr::assign(
                                &mut save.LETTER,
                                &intrinsics::CHAR((save.VALUE + save.CODE0)),
                            );
                            save.INCR = NO;
                        }
                    }
                } else {
                    fstr::assign(&mut save.LETTER, b"0");
                    save.INCR = NO;
                }
            }

            if (save.J <= save.LSUB) {
                fstr::assign(fstr::substr_mut(SUBSTR, save.J..=save.J), &save.LETTER);
            }

            save.J = (save.J - 1);

            save.SLOT += m3__;
        }
    }

    *DID = save.INCR;
}
