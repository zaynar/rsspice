//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const IDXSYS: i32 = 1;
const IDXOS: i32 = (IDXSYS + 1);
const IDXCMP: i32 = (IDXOS + 1);
const IDXFMT: i32 = (IDXCMP + 1);
const IDXTFM: i32 = (IDXFMT + 1);
const IDXRBF: i32 = (IDXTFM + 1);
const KYSIZE: i32 = 64;
const WDSIZE: i32 = 32;
const NATTR: i32 = 6;
const DEFRPY: &[u8; WDSIZE as usize] = &fstr::extend_const::<{ WDSIZE as usize }>(b"<UNAVAILABLE>");

struct SaveVars {
    KEYVAL: ActualCharArray,
    ATTCPY: ActualCharArray,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut KEYVAL = ActualCharArray::new(KYSIZE, 1..=NATTR);
        let mut ATTCPY = ActualCharArray::new(WDSIZE, 0..=NATTR);
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            KEYVAL,
            ATTCPY,
            FIRST,
        }
    }
}

//$Procedure ZZPLATFM ( Private --- Get platform attributes )
pub fn ZZPLATFM(KEY: &[u8], VALUE: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut KEYCPY = [b' '; KYSIZE as usize];
    let mut INDEX: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Array index parameters for each of the key/value pairs.
    //
    // SYSTEM Index.
    //

    //
    // O/S Index.
    //

    //
    // Compiler Index.
    //

    //
    // Binary File Format Index.
    //

    //
    // Text File Format Index
    //

    //
    // Reads Binary File Format Index.
    //

    //
    // Size of the buffer in which KEY is placed.
    //

    //
    // Maximum Size of local string returned in VALUE
    //

    //
    // Number of Platform Dependent values stored here.
    //

    //
    // Default Reply in the event of an invalid KEY.
    //

    //
    // Local Variables
    //

    //
    // Saved Variables
    //

    //
    // Data Statements
    //

    //
    // Make the initial assignments to the saved character array.
    //
    if save.FIRST {
        //
        // Store the keys in the KEYVAL array.
        //
        fstr::assign(save.KEYVAL.get_mut(IDXSYS), b"SYSTEM");
        fstr::assign(save.KEYVAL.get_mut(IDXOS), b"O/S");
        fstr::assign(save.KEYVAL.get_mut(IDXCMP), b"COMPILER");
        fstr::assign(save.KEYVAL.get_mut(IDXFMT), b"FILE_FORMAT");
        fstr::assign(save.KEYVAL.get_mut(IDXTFM), b"TEXT_FORMAT");
        fstr::assign(save.KEYVAL.get_mut(IDXRBF), b"READS_BFF");

        //
        // Set the default reply to be the zero'th component of ATTCPY.
        // This obviates IF-THEN-ELSE branching all together.
        //
        fstr::assign(save.ATTCPY.get_mut(0), DEFRPY);

        //
        // Platform/Environment specific assignments follow.
        //

        fstr::assign(save.ATTCPY.get_mut(IDXSYS), b"PC");

        fstr::assign(save.ATTCPY.get_mut(IDXOS), b"RUST");

        fstr::assign(save.ATTCPY.get_mut(IDXCMP), b"F2RUST");

        fstr::assign(save.ATTCPY.get_mut(IDXFMT), b"LTL-IEEE");

        fstr::assign(save.ATTCPY.get_mut(IDXTFM), b"LF");

        fstr::assign(save.ATTCPY.get_mut(IDXRBF), b"BIG-IEEE LTL-IEEE");

        //
        // Don't execute these assignments again.
        //
        save.FIRST = false;
    }

    //
    // Determine which KEY was passed in; do this by converting KEY
    // to the known member of the equivalence class of possible
    // values.
    //
    UCASE(KEY, &mut KEYCPY, ctx);
    LJUST(&KEYCPY.clone(), &mut KEYCPY);

    //
    // Find out which key we were given.  In the event that one of the
    // KEYVALs (or some equivalent string) was not passed in, ISRCHC
    // returns a value of zero.
    //
    INDEX = ISRCHC(&KEYCPY, NATTR, save.KEYVAL.as_arg());
    fstr::assign(VALUE, save.ATTCPY.get(INDEX));
}
