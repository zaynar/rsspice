//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      HAVE ( Do we have an error? )
pub fn HAVE(ERROR: CharArrayMut, ctx: &mut Context) -> f2rust_std::Result<bool> {
    let mut ERROR = DummyCharArrayMut::new(ERROR, None, 1..=2);
    let mut HAVE: bool = false;
    let mut DEPTH: i32 = 0;
    let mut NAME = [b' '; 32 as usize];
    let mut SMS = [b' '; 80 as usize];

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Check to see if an error occurred.
    //
    if (fstr::ne(fstr::substr(ERROR.get(1), 1..=1), b" ") || spicelib::FAILED(ctx)) {
        HAVE = true;
    } else {
        HAVE = false;
        return Ok(HAVE);
    }

    //
    // If an error was detected by the SPICELIB error handling and
    // the ERROR message is blank, we need to get the SPICELIB error
    // message. After that, reset the error handling.
    //
    if (spicelib::FAILED(ctx) && fstr::eq(ERROR.get(1), b" ")) {
        spicelib::GETSMS(&mut SMS, ctx);
        spicelib::GETLMS(&mut ERROR[1], ctx);

        spicelib::PREFIX(b"--", 0, &mut ERROR[1]);
        spicelib::PREFIX(&SMS, 0, &mut ERROR[1]);

        fstr::assign(ERROR.get_mut(2), b"SPICELIB Trace>");
        spicelib::TRCDEP(&mut DEPTH, ctx);

        for I in 1..=DEPTH {
            spicelib::TRCNAM(I, &mut NAME, ctx)?;

            if (I == 1) {
                spicelib::SUFFIX(&NAME, 1, &mut ERROR[2]);
            } else {
                spicelib::SUFFIX(&NAME, 0, &mut ERROR[2]);
            }

            if (I != DEPTH) {
                spicelib::SUFFIX(b":", 0, &mut ERROR[2]);
            }
        }

        spicelib::RESET(ctx);

    //
    // It is possible that FAILED() is true, even though we already
    // had a recorded error.  To avoid having this show up in a later
    // command, we reset the SPICELIB error handling now.  This isn't
    // really a good solution, but a better one doesn't come to mind
    // at the moment.
    //
    } else if spicelib::FAILED(ctx) {
        spicelib::RESET(ctx);
    }

    Ok(HAVE)
}
