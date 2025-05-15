//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SIZSTR: i32 = 16;
const SIZEXP: i32 = (3 * SIZSTR);
const SIZEND: i32 = 6;
const SIZFTP: i32 = (SIZSTR + (2 * SIZEND));
const SIZDLM: i32 = 1;
const NUMTST: i32 = 6;
const SIZTSQ: i32 = 5;
const INT000: i32 = 0;
const INT010: i32 = 10;
const INT013: i32 = 13;
const INT016: i32 = 16;
const INT129: i32 = 129;
const INT206: i32 = 206;

struct SaveVars {
    LOCDLM: Vec<u8>,
    LOCLND: Vec<u8>,
    LOCRND: Vec<u8>,
    LOCSTR: Vec<u8>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LOCDLM = vec![b' '; SIZDLM as usize];
        let mut LOCLND = vec![b' '; SIZEND as usize];
        let mut LOCRND = vec![b' '; SIZEND as usize];
        let mut LOCSTR = vec![b' '; SIZSTR as usize];
        let mut FIRST: bool = false;

        FIRST = true;
        fstr::assign(&mut LOCDLM, b":");
        fstr::assign(&mut LOCLND, b"FTPSTR");
        fstr::assign(&mut LOCRND, b"ENDFTP");

        Self {
            LOCDLM,
            LOCLND,
            LOCRND,
            LOCSTR,
            FIRST,
        }
    }
}

//$Procedure ZZFTPSTR ( Private --- Fetch FTP Validation String )
pub fn ZZFTPSTR(
    TSTCOM: &mut [u8],
    LEND: &mut [u8],
    REND: &mut [u8],
    DELIM: &mut [u8],
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TESTSQ = ActualCharArray::new(SIZTSQ, 1..=NUMTST);
    let mut ASC000 = [b' '; 1 as usize];
    let mut ASC010 = [b' '; 1 as usize];
    let mut ASC013 = [b' '; 1 as usize];
    let mut ASC016 = [b' '; 1 as usize];
    let mut ASC129 = [b' '; 1 as usize];
    let mut ASC206 = [b' '; 1 as usize];

    //
    // Local Parameters
    //
    // Maximum size of an individual test cluster component
    // including the ':'.
    //

    //
    // Integer codes of characters appearing in test clusters.
    //

    //
    //
    // Local Variables
    //

    //
    // Non-printing character values.
    //

    //
    // Saved Variables
    //

    //
    // Data Statements
    //

    //
    // Set up the components of the FTP validation string that
    // are not supposed to change for forward and backward
    // compatibility.
    //

    //
    // On the first invocation initialize the string values.
    //
    if save.FIRST {
        //
        // Convert the integer parameters to their non-printing ASCII
        // equivalents.
        //
        fstr::assign(&mut ASC000, &intrinsics::CHAR(INT000));
        fstr::assign(&mut ASC010, &intrinsics::CHAR(INT010));
        fstr::assign(&mut ASC013, &intrinsics::CHAR(INT013));
        fstr::assign(&mut ASC016, &intrinsics::CHAR(INT016));
        fstr::assign(&mut ASC129, &intrinsics::CHAR(INT129));
        fstr::assign(&mut ASC206, &intrinsics::CHAR(INT206));

        //
        // Now build the individual components of the test clusters.
        // Make certain the first component begins and ends with a ':',
        // and that the remaining pieces end in ':'. If you intend to
        // add some clusters, then append them to the end of the
        // sequence so as not to break the existing detection code.
        //

        //
        // Cluster #1 : <CR> - <13> - Macintosh Line Terminator
        //
        fstr::assign(
            TESTSQ.get_mut(1),
            &fstr::concat(&fstr::concat(&save.LOCDLM, &ASC013), &save.LOCDLM),
        );

        //
        // Cluster #2 : <LF> - <10> - Unix Line Terminator
        //
        fstr::assign(TESTSQ.get_mut(2), &fstr::concat(&ASC010, &save.LOCDLM));

        //
        // Cluster #3 : <CR><LF> - <10><13> - Microsoft Line Terminator
        //
        fstr::assign(
            TESTSQ.get_mut(3),
            &fstr::concat(&fstr::concat(&ASC013, &ASC010), &save.LOCDLM),
        );

        //
        // Cluster #4 : <13><0>
        //
        fstr::assign(
            TESTSQ.get_mut(4),
            &fstr::concat(&fstr::concat(&ASC013, &ASC000), &save.LOCDLM),
        );

        //
        // Cluster #5 : <129> - Macintosh Permutation of Parity Codes
        //
        fstr::assign(TESTSQ.get_mut(5), &fstr::concat(&ASC129, &save.LOCDLM));

        //
        // Cluster #6 : <16><206>
        //
        fstr::assign(
            TESTSQ.get_mut(6),
            &fstr::concat(&fstr::concat(&ASC016, &ASC206), &save.LOCDLM),
        );

        //
        // Sample cluster addition code follows
        //
        // Cluster #7 : <xxx> - Description
        //
        // TESTSQ(7) = ASCxxx // ... // LOCDLM
        //

        //
        // Now build the local copy of TSTCOM, LOCSTR. First clear the
        // uninitialized contents.
        //
        fstr::assign(&mut save.LOCSTR, b" ");

        for I in 1..=NUMTST {
            //
            // Append TESTSQ(I) to LOCSTR to properly construct the
            // test component of the FTP validation string.
            //
            SUFFIX(&TESTSQ[I], 0, &mut save.LOCSTR);
        }

        //
        // Prevent execution of this initialization code after first pass.
        //
        save.FIRST = false;
    }
    //
    // Copy the local copies of the FTP string components to the
    // arguments passed in from the caller.
    //
    fstr::assign(TSTCOM, &save.LOCSTR);
    fstr::assign(LEND, &save.LOCLND);
    fstr::assign(REND, &save.LOCRND);
    fstr::assign(DELIM, &save.LOCDLM);
}
