//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

//$Procedure      F_CKGP (Family of tests for CKGP)
pub fn F_CKGP(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CKNM = [b' '; LNSIZE as usize];
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut PCKNM = [b' '; LNSIZE as usize];
    let mut SCLKNM = [b' '; LNSIZE as usize];
    let mut ANGVEL = StackArray::<f64, 3>::new(1..=3);
    let mut CMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ECMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut EOUT: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TICKS: f64 = 0.0;
    let mut TMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TOL: f64 = 0.0;
    let mut TOUT: f64 = 0.0;
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut HANDLE: i32 = 0;
    let mut ID: i32 = 0;
    let mut SCLK: i32 = 0;
    let mut EFOUND: bool = false;
    let mut FOUND: bool = false;
    let mut KEEPPC: bool = false;
    let mut KEEPSC: bool = false;
    let mut LOADCK: bool = false;
    let mut LOADPC: bool = false;
    let mut LOADSC: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_CKGP", ctx)?;

    fstr::assign(&mut CKNM, b"phoenix.bc");
    fstr::assign(&mut PCKNM, b"phoenix.pck");
    fstr::assign(&mut SCLKNM, b"phoenix.sclk");
    LOADCK = true;
    LOADSC = true;
    KEEPSC = true;
    LOADPC = true;
    KEEPPC = false;

    testutil::KILFIL(&CKNM, ctx)?;
    testutil::KILFIL(&PCKNM, ctx)?;
    testutil::KILFIL(&SCLKNM, ctx)?;

    testutil::TSTCKN(&CKNM, &SCLKNM, LOADCK, LOADSC, KEEPSC, &mut HANDLE, ctx)?;
    testutil::TSTPCK(&PCKNM, LOADPC, KEEPPC, ctx)?;
    spicelib::CKMETA(-10000, b"SCLK", &mut SCLK, ctx)?;

    spicelib::TPARSE(b"1 JAN 1995", &mut ET, &mut ERROR, ctx)?;
    spicelib::SCE2T(SCLK, ET, &mut TICKS, ctx)?;

    testutil::TCASE(b"Check that the old version of CKGP and the new version yield the same result when the frame requested is an inertial frame. ", ctx)?;

    ID = -10000;

    spicelib::CKGP(
        ID,
        TICKS,
        0.0,
        b"J2000",
        CMAT.as_slice_mut(),
        &mut TOUT,
        &mut FOUND,
        ctx,
    )?;
    CKGP_O(
        ID,
        TICKS,
        0.0,
        b"J2000",
        ECMAT.as_slice_mut(),
        &mut EOUT,
        &mut EFOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKAD(
        b"CMAT",
        CMAT.as_slice(),
        b"=",
        ECMAT.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"TOUT", TOUT, b"=", TICKS, 0.0, OK, ctx)?;

    testutil::TCASE(b"Perform an independent computation of the attitude of object -9999 and compare this with the value returned by CKGP. ", ctx)?;

    ID = -9999;
    TOL = 0.0;

    spicelib::CKMETA(ID, b"SCLK", &mut SCLK, ctx)?;

    spicelib::TPARSE(b"1 JAN 1995", &mut ET, &mut ERROR, ctx)?;
    spicelib::SCE2T(SCLK, ET, &mut TICKS, ctx)?;
    //
    // Now convert TICKS back to ET so that ET and TIKS point to
    // the same epoch (remember TICKS get pushed to an integer value
    // by SCE2T).
    //
    spicelib::SCT2E(SCLK, TICKS, &mut ET, ctx)?;

    spicelib::CKGP(
        ID,
        TICKS,
        TOL,
        b"IAU_EARTH",
        CMAT.as_slice_mut(),
        &mut TOUT,
        &mut FOUND,
        ctx,
    )?;
    //
    // CMAT now contains the transformation from IAU_EARTH to the
    // spacecraft frame (or it's supposed to anyway).
    //

    testutil::TSTATD(ET, TMAT.as_slice_mut(), ANGVEL.as_slice_mut(), ctx);
    //
    // TMAT gives the orientation of object -9999 relative to the
    // GALACTIC reference frame.  (TMAT is the transformation from
    // galactic to S.C. frame.)
    //
    spicelib::SXFORM(b"IAU_EARTH", b"GALACTIC", ET, XFORM.as_slice_mut(), ctx)?;

    for I in 1..=3 {
        for J in 1..=3 {
            ROT[[I, J]] = XFORM[[I, J]];
        }
    }

    //
    // ROT      : IAU_EARTH ---> Galactic.
    // TMAT     : Galactic  ---> S.C. frame
    // TMAT*ROT : IAU_EARTH ---> S.C. frame
    //
    spicelib::MXM(TMAT.as_slice(), ROT.as_slice(), ECMAT.as_slice_mut());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, EFOUND, OK, ctx)?;
    testutil::CHCKAD(
        b"CMAT",
        CMAT.as_slice(),
        b"~/",
        ECMAT.as_slice(),
        9,
        0.0000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"TOUT", TOUT, b"=", TICKS, 0.0, OK, ctx)?;

    spicelib::CKUPF(HANDLE, ctx)?;
    testutil::KILFIL(&CKNM, ctx)?;
    testutil::KILFIL(&PCKNM, ctx)?;
    testutil::KILFIL(&SCLKNM, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
