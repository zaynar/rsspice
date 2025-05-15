//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const DIGIT: i32 = 1;
const COLON: i32 = (DIGIT + 1);
const POINT: i32 = (COLON + 1);
const OTHER: i32 = (POINT + 1);

struct SaveVars {
    M2TIME: bool,
    CLASS: StackArray<i32, 256>,
    COMP: i32,
    COUNT: i32,
    END: i32,
    FACTOR: StackArray<i32, 4>,
    I: i32,
    LIMIT: StackArray<i32, 4>,
    N: i32,
    START: i32,
    UBND: StackArray<i32, 4>,
    ZERO: i32,
    COLOK: StackArray<bool, 4>,
    FIRST: bool,
    PNTOK: StackArray<bool, 4>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut M2TIME: bool = false;
        let mut CLASS = StackArray::<i32, 256>::new(0..=255);
        let mut COMP: i32 = 0;
        let mut COUNT: i32 = 0;
        let mut END: i32 = 0;
        let mut FACTOR = StackArray::<i32, 4>::new(1..=4);
        let mut I: i32 = 0;
        let mut LIMIT = StackArray::<i32, 4>::new(1..=4);
        let mut N: i32 = 0;
        let mut START: i32 = 0;
        let mut UBND = StackArray::<i32, 4>::new(1..=4);
        let mut ZERO: i32 = 0;
        let mut COLOK = StackArray::<bool, 4>::new(1..=4);
        let mut FIRST: bool = false;
        let mut PNTOK = StackArray::<bool, 4>::new(1..=4);

        FIRST = true;

        Self {
            M2TIME,
            CLASS,
            COMP,
            COUNT,
            END,
            FACTOR,
            I,
            LIMIT,
            N,
            START,
            UBND,
            ZERO,
            COLOK,
            FIRST,
            PNTOK,
        }
    }
}

//$Procedure      M2TIME ( Determine whether or not a word is a time )
pub fn M2TIME(WORD: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    if save.FIRST {
        save.FIRST = false;
        {
            let m1__: i32 = 0;
            let m2__: i32 = 255;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.CLASS[save.I] = OTHER;
                save.I += m3__;
            }
        }

        save.CLASS[intrinsics::ICHAR(b"0")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"1")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"2")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"3")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"4")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"5")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"6")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"7")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"8")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"9")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b":")] = COLON;
        save.CLASS[intrinsics::ICHAR(b".")] = POINT;
        //
        // The following are the maximum values that are allowed
        // for each of the various components of the time string
        //
        save.UBND[1] = 23;
        save.UBND[2] = 59;
        save.UBND[3] = 60;
        save.UBND[4] = 10;
        //
        // The following are the maximum number of digits that
        // are allowed for each of the components of the time
        //
        save.LIMIT[1] = 2;
        save.LIMIT[2] = 2;
        save.LIMIT[3] = 2;
        save.LIMIT[4] = 100;
        //
        // The following logicals indicate whether or not it is
        // ok to end the N'th component of time with a colon.
        //
        save.COLOK[1] = true;
        save.COLOK[2] = true;
        save.COLOK[3] = false;
        save.COLOK[4] = false;
        //
        // The following logicals indicate whether or not it is
        // ok to end the N'th component of time with a decimal point.
        //
        save.PNTOK[1] = false;
        save.PNTOK[2] = false;
        save.PNTOK[3] = true;
        save.PNTOK[4] = false;
        //
        // The following are the factors used to construct the
        // integer value of a component COMP = FACTOR*COMP + Next digit.
        // Note that for the decimal portion of seconds we don't
        // really compute the value of the decimal part.  The
        // factor term just ensures that the loop below doesn't
        // have any special cases.
        //
        save.FACTOR[1] = 10;
        save.FACTOR[2] = 10;
        save.FACTOR[3] = 10;
        save.FACTOR[4] = 0;

        save.ZERO = intrinsics::ICHAR(b"0");
    }

    save.START = spicelib::LTRIM(WORD);
    save.END = QRTRIM(WORD);
    save.COMP = 0;
    save.N = 1;
    save.COUNT = 0;
    save.I = save.START;
    save.M2TIME = true;

    if ((save.END - save.START) < 4) {
        save.M2TIME = false;
        return save.M2TIME;
    }

    while ((save.I <= save.END) && save.M2TIME) {
        //
        // If the next character is a digit, compute the accumulated
        // value of this component of the time.  Then check to
        // make sure that we don't have too many digits so far
        // in this component and that the value of this component
        // does not exceed the limits for this component.
        //
        if (save.CLASS[intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I))] == DIGIT) {
            save.COUNT = (save.COUNT + 1);
            save.COMP = (((save.FACTOR[save.N] * save.COMP)
                + intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)))
                - save.ZERO);

            save.M2TIME = ((save.COUNT <= save.LIMIT[save.N]) && (save.COMP <= save.UBND[save.N]));

        //
        // If the next character is a colon ':' then we are starting
        // a new component.  Make sure this is ok and that we actually
        // had a digit or two for the last component.  Increment the
        // component counter, set the number of characters found in
        // the next component to 0 and set the value of the next
        // component to zero.
        //
        } else if (save.CLASS[intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I))] == COLON) {
            save.M2TIME = (save.COLOK[save.N] && (save.COUNT > 0));
            save.COUNT = 0;
            save.COMP = 0;
            save.N = (save.N + 1);
        //
        // If the next character is decimal point, we are ending a
        // component and starting it's decimal portion.  Make sure
        // that a decimal point is allowed for this component and
        // that we had at least one digit in the component we were
        // examining up to this point.
        //
        } else if (save.CLASS[intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I))] == POINT) {
            save.M2TIME = (save.PNTOK[save.N] && (save.COUNT > 0));
            save.COUNT = 0;
            save.COMP = 0;
            save.N = (save.N + 1);
        //
        // If we hit some other character we don't have a time
        // word.
        //
        } else {
            save.M2TIME = false;
        }

        save.I = (save.I + 1);
    }

    save.M2TIME = (save.M2TIME && (save.N >= 3));

    save.M2TIME
}
