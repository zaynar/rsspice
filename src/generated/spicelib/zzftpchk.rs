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

struct SaveVars {
    LFTBKT: Vec<u8>,
    MEMSTR: Vec<u8>,
    RGTBKT: Vec<u8>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LFTBKT = vec![b' '; SIZEND as usize];
        let mut MEMSTR = vec![b' '; SIZSTR as usize];
        let mut RGTBKT = vec![b' '; SIZEND as usize];
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            LFTBKT,
            MEMSTR,
            RGTBKT,
            FIRST,
        }
    }
}

//$Procedure ZZFTPCHK ( Private --- Check for FTP Errors )
pub fn ZZFTPCHK(STRING: &[u8], FTPERR: &mut bool, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DELIM = [b' '; SIZDLM as usize];
    let mut FILSTR = [b' '; SIZEXP as usize];
    let mut FSMIDX: i32 = 0;
    let mut LENGTH: i32 = 0;
    let mut MSFIDX: i32 = 0;
    let mut ISTHER: bool = false;

    //
    // SPICELIB Functions
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
    // On the first pass through, fetch a copy of the current FTP
    // validation string.
    //
    if save.FIRST {
        ZZFTPSTR(
            &mut save.MEMSTR,
            &mut save.LFTBKT,
            &mut save.RGTBKT,
            &mut DELIM,
            ctx,
        );

        //
        // Don't fetch the string on subsequent calls to this routine.
        //
        save.FIRST = false;
    }

    //
    // Extract the FTP validation string from the block of text that
    // was passed into the routine via the argument STRING. Note,
    // if the bracketed substring in the text block STRING is larger
    // than the FILSTR string size, ZZRBRKST will truncate the data
    // that does not fit.  This loss of data is not an issue, since in
    // this case we may only validate the part of the substring near
    // the head, for which we have enough room in FILSTR.
    //
    ZZRBRKST(
        STRING,
        fstr::substr(&save.LFTBKT, 1..=RTRIM(&save.LFTBKT)),
        fstr::substr(&save.RGTBKT, 1..=RTRIM(&save.RGTBKT)),
        &mut FILSTR,
        &mut LENGTH,
        &mut ISTHER,
    );

    //
    // Now check ISTHER to see if either LFTBKT or RGTBKT was present
    // in the block of text from the file. If both are absent, then
    // we must assume that this text is from a pre-FTP validation file,
    // and as such do not return any indication of an error.
    //
    if !ISTHER {
        *FTPERR = false;

    //
    // If one of the brackets is present, then we may proceed with
    // validation.  First check to see if the length is 0.  If it is,
    // then at least one of the brackets was present, but ZZRBRKST was
    // unable to extract a properly bracketed substring.  This is an
    // error.
    //
    } else if (LENGTH <= 0) {
        *FTPERR = true;

    //
    // Now we make it to this ELSE statement only if ISTHER is TRUE, and
    // LENGTH is a positive number.  Compare the contents of FILSTR
    // and MEMSTR.
    //
    } else {
        //
        // First determine if the data from the file is a subset of
        // what is stored in memory.
        //
        FSMIDX = POS(&save.MEMSTR, fstr::substr(&FILSTR, 1..=RTRIM(&FILSTR)), 1);

        //
        // In the event that FSMIDX is non-zero, we know that FILSTR
        // is a substring of MEMSTR, and thus we have validated all the
        // test clusters from the file.
        //
        if (FSMIDX != 0) {
            *FTPERR = false;

        //
        // If FSMIDX is 0, then we do not yet know whether or not the
        // file is valid.  Now it may be the case that this file contains
        // a newer FTP validation string than this version of the
        // toolkit is aware.  Check to see whether what's in memory
        // is a substring of what's in FILSTR.
        //
        } else {
            MSFIDX = POS(
                &FILSTR,
                fstr::substr(&save.MEMSTR, 1..=RTRIM(&save.MEMSTR)),
                1,
            );

            //
            // If this comes back as zero, then we definitely have
            // an FTP error. Set FTPERR appropriately.
            //
            *FTPERR = (MSFIDX == 0);
        }
    }
}
