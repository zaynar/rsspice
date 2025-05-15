//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXFNM: i32 = 80;
const MAXCNM: i32 = 1000;
const LNGSTR: i32 = 1200;
const WDSIZE: i32 = 32;
const MAXPLC: i32 = 7;

struct SaveVars {
    SUCCES: bool,
    CASEOK: bool,
    CASENO: i32,
    NFAIL: i32,
    DESCR: Vec<u8>,
    MYNAME: Vec<u8>,
    MYACT: Vec<u8>,
    TRACE: Vec<u8>,
    RNAME: Vec<u8>,
    DIGIT: ActualCharArray2D,
    D: StackArray<i32, 7>,
    J: i32,
    INCR: bool,
    ANYBAD: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SUCCES: bool = false;
        let mut CASEOK: bool = false;
        let mut CASENO: i32 = 0;
        let mut NFAIL: i32 = 0;
        let mut DESCR = vec![b' '; MAXCNM as usize];
        let mut MYNAME = vec![b' '; MAXCNM as usize];
        let mut MYACT = vec![b' '; WDSIZE as usize];
        let mut TRACE = vec![b' '; LNGSTR as usize];
        let mut RNAME = vec![b' '; MAXFNM as usize];
        let mut DIGIT = ActualCharArray2D::new(1, 1..=10, 1..=MAXPLC);
        let mut D = StackArray::<i32, 7>::new(1..=MAXPLC);
        let mut J: i32 = 0;
        let mut INCR: bool = false;
        let mut ANYBAD: bool = false;

        ANYBAD = false;
        SUCCES = true;
        CASEOK = true;
        CASENO = 0;
        NFAIL = 0;
        fstr::assign(&mut MYNAME, b" ");
        fstr::assign(&mut DESCR, b" ");
        fstr::assign(&mut RNAME, b" ");

        Self {
            SUCCES,
            CASEOK,
            CASENO,
            NFAIL,
            DESCR,
            MYNAME,
            MYACT,
            TRACE,
            RNAME,
            DIGIT,
            D,
            J,
            INCR,
            ANYBAD,
        }
    }
}

//$Procedure      T_STAT ( Test Status )
pub fn T_STAT(ACT: &[u8], NAME: &[u8], OK: bool, NUMBER: i32) {

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // We enough digits to have unique text versions of case numbers
    // for 10**7 cases.
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //
}

//$Procedure T_BEGIN ( Initialize test case family )
pub fn T_BEGIN(NAME: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Until something goes wrong.  All tests are assumed to pass.
    // Store the name and current case number.
    //
    save.SUCCES = true;
    fstr::assign(&mut save.RNAME, NAME);
    save.CASENO = 0;
    save.NFAIL = 0;
    //
    // Set up the "odometer" for the case counter.  Note we put
    // blanks instead of zeros for every digit but the least
    // significant digit.
    //
    // Here's the idea.  At each test case, we increment CASENO
    // and flip over one digit on the case odometer.  When we
    // reach the 10 on any digit, we flip back to zero and
    // cause the digit to the left to increment.  The actual
    // text version for the current case number will be:
    //
    // DIGIT(D(1),1)//DIGIT(D(2),2)// ... //DIGIT(D(MAXPLC),MAXPLC).
    //

    for I in 1..=MAXPLC {
        fstr::assign(save.DIGIT.get_mut([1, I]), b" ");
        fstr::assign(save.DIGIT.get_mut([2, I]), b"1");
        fstr::assign(save.DIGIT.get_mut([3, I]), b"2");
        fstr::assign(save.DIGIT.get_mut([4, I]), b"3");
        fstr::assign(save.DIGIT.get_mut([5, I]), b"4");
        fstr::assign(save.DIGIT.get_mut([6, I]), b"5");
        fstr::assign(save.DIGIT.get_mut([7, I]), b"6");
        fstr::assign(save.DIGIT.get_mut([8, I]), b"7");
        fstr::assign(save.DIGIT.get_mut([9, I]), b"8");
        fstr::assign(save.DIGIT.get_mut([10, I]), b"9");
        save.D[I] = 1;
    }

    fstr::assign(save.DIGIT.get_mut([1, MAXPLC]), b"0");
}

//$Procedure T_NAME ( Return the name of the current family of tests )
pub fn T_NAME(NAME: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(NAME, &save.RNAME);
}

//$Procedure T_SUCCESS ( Return information on overall status of tests )
pub fn T_SUCCESS(OK: &mut bool, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *OK = save.SUCCES;
}

//$Procedure T_FCOUNT ( Number of failed test cases )
pub fn T_FCOUNT(NUMBER: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *NUMBER = save.NFAIL;
}

//$Procedure T_CASE ( Set up for the next test case )
pub fn T_CASE(NAME: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Store the name, set this case success value to .TRUE. and
    // increment the case number.
    //
    fstr::assign(&mut save.DESCR, NAME);
    save.CASEOK = true;
    save.CASENO = (save.CASENO + 1);
    //
    // Reset the error handling.
    //
    spicelib::RESET(ctx);

    //
    // Next we increment the odometer.  We always increment the least
    // significant digit.
    //
    save.INCR = true;

    for I in intrinsics::range(MAXPLC, 1, -1) {
        if save.INCR {
            //
            // If we incremented this digit, we will never want to use
            // a blank (while in this test case) for this digit again.
            // Set the first digit to zero "0".
            //
            save.D[I] = (save.D[I] + 1);
            fstr::assign(save.DIGIT.get_mut([1, I]), b"0");
            //
            // If the current digit is not greater than 10, we need
            // to set it back to 1 and cascade up to the next digit.
            // That is, make sure INCR stays .TRUE.
            //
            if (save.D[I] > 10) {
                save.D[I] = 1;
            } else {
                //
                // In this case we do not need to increment any digit
                // that is more significant than the current digit.
                //
                save.INCR = false;
            }
        }
    }
}

//$Procedure T_CFAIL ( Records some aspects of failed test case )
pub fn T_CFAIL(ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if save.CASEOK {
        save.NFAIL = (save.NFAIL + 1);
    }

    save.ANYBAD = true;
    save.CASEOK = false;
    save.SUCCES = false;
}

//$Procedure T_CPASS ( Return status of current test case )
pub fn T_CPASS(OK: &mut bool, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *OK = save.CASEOK;
}

//$Procedure T_CNUM ( Number of current test case )
pub fn T_CNUM(NUMBER: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *NUMBER = save.CASENO;
}

//$Procedure T_CNAME ( Return the name of the current test case )
pub fn T_CNAME(NAME: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(&mut save.MYNAME, b"Test Case ");
    save.J = 10;
    //
    // Append each non-blank digit (starting with the most
    // significant) to the current value of MYNAME.
    //
    for I in 1..=MAXPLC {
        if fstr::ne(save.DIGIT.get([save.D[I], I]), b" ") {
            save.J = (save.J + 1);
            fstr::assign(
                fstr::substr_mut(&mut save.MYNAME, save.J..=save.J),
                save.DIGIT.get([save.D[I], I]),
            );
        }
    }
    //
    // Place a period after the case number.
    //
    save.J = (save.J + 1);
    fstr::assign(fstr::substr_mut(&mut save.MYNAME, save.J..=save.J), b".");
    //
    // Follow that up with the case description.
    //
    save.J = (save.J + 2);
    fstr::assign(fstr::substr_mut(&mut save.MYNAME, save.J..), &save.DESCR);
    fstr::assign(NAME, &save.MYNAME);
}

//$Procedure T_ANYBAD ( Was there any failed case? )
pub fn T_ANYBAD(OK: &mut bool, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *OK = save.ANYBAD;
}

//$Procedure T_TRACE ( Set or get the last stored value of traceback )
pub fn T_TRACE(ACT: &[u8], NAME: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    spicelib::LJUST(ACT, &mut save.MYACT);
    spicelib::UCASE(&save.MYACT.to_vec(), &mut save.MYACT, ctx);

    if fstr::eq(&save.MYACT, b"SET") {
        fstr::assign(&mut save.TRACE, NAME);
    } else if fstr::eq(&save.MYACT, b"GET") {
        fstr::assign(NAME, &save.TRACE);
    }
}
