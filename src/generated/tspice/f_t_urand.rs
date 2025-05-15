//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const MSGLEN: i32 = 800;
const NRANDM: i32 = 2000;

//$Procedure F_T_URAND ( T_URAND tests )
pub fn F_T_URAND(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut NAME = [b' '; LNSIZE as usize];
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut U: f64 = 0.0;
    let mut U2: f64 = 0.0;
    let mut N: i32 = 0;
    let mut SEED: i32 = 0;
    let mut SEED2: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_T_URAND", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    fstr::assign(
        &mut TITLE,
        b"Generate a series of random numbers, starting with an initializing seed.",
    );
    testutil::TCASE(&TITLE, ctx)?;

    N = 100000;

    SEED = -1;
    SEED2 = -1;

    for I in 1..=N {
        U = testutil::T_URAND(&mut SEED, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        U2 = T_URAND2(&mut SEED2, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut NAME, b"Case 1 iteration #.");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKSD(&NAME, U, b"=", U2, 0.0, OK, ctx)?;

        // IF ( U .LT. 1.D-3 ) THEN
        //    WRITE (*,*) 'U  = ', U
        //    WRITE (*,*) 'U2 = ', U2
        //    WRITE (*,*) ' '
        // END IF
    }

    Ok(())
}
