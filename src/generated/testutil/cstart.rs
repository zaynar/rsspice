//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;
const MAXHND: i32 = 1000;

//$Procedure      CSTART ( Clean Start )
pub fn CSTART(ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut HANDLS = ActualArray::<i32>::new(LBCELL..=MAXHND);
    let mut N: i32 = 0;
    let mut TRIES: i32 = 0;

    //
    // Spicelib Functions
    //

    //
    // Local Variables
    //

    //
    // First, clear the kernel pool
    //
    spicelib::CLPOOL(ctx)?;

    //
    // Clear the built-in body list.
    //
    spicelib::ZZBODRST(ctx)?;

    //
    // Next unload every SPK, CK, and binary PCK file.
    //
    spicelib::SSIZEI(MAXHND, HANDLS.as_slice_mut(), ctx)?;
    spicelib::DAFHOF(HANDLS.as_slice_mut(), ctx)?;
    N = spicelib::CARDI(HANDLS.as_slice(), ctx)?;

    for I in 1..=N {
        spicelib::SPKUEF(HANDLS[I], ctx)?;
        spicelib::CKUPF(HANDLS[I], ctx)?;
        spicelib::PCKUOF(HANDLS[I], ctx)?;
    }
    //
    // We have probably now closed every DAF, but just in case, we
    // get the remaining open handles and continue closing them until
    // all DAFs are closed.
    //
    spicelib::SSIZEI(MAXHND, HANDLS.as_slice_mut(), ctx)?;
    spicelib::DAFHOF(HANDLS.as_slice_mut(), ctx)?;
    N = spicelib::CARDI(HANDLS.as_slice(), ctx)?;

    TRIES = 0;

    while ((N > 0) && (TRIES < 100)) {
        for I in 1..=N {
            spicelib::DAFCLS(HANDLS[I], ctx)?;
        }

        spicelib::SSIZEI(MAXHND, HANDLS.as_slice_mut(), ctx)?;
        spicelib::DAFHOF(HANDLS.as_slice_mut(), ctx)?;
        N = spicelib::CARDI(HANDLS.as_slice(), ctx)?;
        TRIES = (TRIES + 1);
    }

    //
    // Finally, unload any EK's or DAS's
    //
    spicelib::DASHOF(HANDLS.as_slice_mut(), ctx)?;
    N = spicelib::CARDI(HANDLS.as_slice(), ctx)?;

    for I in 1..=N {
        spicelib::EKUEF(HANDLS[I], ctx)?;
    }
    //
    // We have probably now closed every DAS, but just in case, we
    // get the remaining open HANDLS(I)s and continue closing them until
    // all DAS files are closed.  But first, unload the scratch file
    // used by the EK scratch area system.
    //
    spicelib::ZZEKSCLN(ctx)?;

    //
    // Ok, now clean up the other DAS files.
    //
    TRIES = 0;
    spicelib::SSIZEI(MAXHND, HANDLS.as_slice_mut(), ctx)?;
    spicelib::DASHOF(HANDLS.as_slice_mut(), ctx)?;
    N = spicelib::CARDI(HANDLS.as_slice(), ctx)?;

    while ((N > 0) && (TRIES < 100)) {
        for I in 1..=N {
            spicelib::DASCLS(HANDLS[I], ctx)?;
        }

        TRIES = (TRIES + 1);
        spicelib::SSIZEI(MAXHND, HANDLS.as_slice_mut(), ctx)?;
        spicelib::DASHOF(HANDLS.as_slice_mut(), ctx)?;
        N = spicelib::CARDI(HANDLS.as_slice(), ctx)?;
    }

    //
    // Wipe out all test files.
    //
    KFILES(ctx)?;

    Ok(())
}
