//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.000000000001;

//$Procedure F_GFREFN ( GFREFN family tests )
pub fn F_GFREFN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut T: f64 = 0.0;
    let mut T1: f64 = 0.0;
    let mut T2: f64 = 0.0;
    let mut TEXP: f64 = 0.0;
    let mut RLOG: f64 = 0.0;
    let mut SEED1: i32 = 0;
    let mut TXT = [b' '; 128 as usize];
    let S1: bool = false;
    let S2: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFREFN", ctx)?;

    SEED1 = -82763653;

    //
    // Note, 5000 is an arbitrary value.
    //
    for I in 1..=5000 {
        //
        // Set T1 and T2 so that TEXP = (T1 + T2)/2.
        //
        RLOG = testutil::T_RANDD(-6.0, 14.0, &mut SEED1, ctx)?;
        T1 = (f64::powf(10 as f64, RLOG) * 1.0);
        T2 = (f64::powf(10 as f64, RLOG) * 3.0);
        TEXP = (f64::powf(10 as f64, RLOG) * 2.0);

        fstr::assign(&mut TXT, b"T1 = +/- #1, T2 = +/-#2, random test");

        spicelib::REPMD(&TXT.clone(), b"#1", T1, 12, &mut TXT, ctx);
        spicelib::REPMD(&TXT.clone(), b"#2", T2, 12, &mut TXT, ctx);

        testutil::TCASE(&TXT, ctx)?;

        //
        // T1 < T2.
        //
        spicelib::GFREFN(T1, T2, S1, S2, &mut T);
        testutil::CHCKSD(&TXT, T, b"~/", TEXP, TIGHT, OK, ctx)?;

        //
        // Negate the lot; T2 < T1
        //
        spicelib::GFREFN(-T1, -T2, S1, S2, &mut T);
        testutil::CHCKSD(&TXT, T, b"~/", -TEXP, TIGHT, OK, ctx)?;
    }

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
