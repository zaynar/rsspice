//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

//$Procedure      F_SPKEZP (Family of tests for SPKEZP)
pub fn F_SPKEZP(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut HANDLE: i32 = 0;
    let mut ET: f64 = 0.0;
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut ELT: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut ESTATE = StackArray::<f64, 6>::new(1..=6);
    let mut TSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut SOBS = StackArray::<f64, 6>::new(1..=6);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut CLT: f64 = 0.0;
    let mut ISTATE = StackArray::<f64, 6>::new(1..=6);

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
    testutil::TOPEN(b"F_SPKEZP", ctx)?;

    testutil::KILFIL(b"phoenix2.bsp", ctx)?;
    testutil::KILFIL(b"phoenix2.ker", ctx)?;

    testutil::TSTSPK(b"phoenix2.bsp", true, &mut HANDLE, ctx)?;

    testutil::TSTPCK(b"phoenix2.ker", true, false, ctx)?;
    spicelib::TPARSE(b"1 JAN 1995", &mut ET, &mut ERROR, ctx)?;

    testutil::TCASE(b"Make sure that SPKEZP and SPKGEO return the same information when the correction requested is NONE. ", ctx)?;

    spicelib::SPKEZP(
        401001,
        ET,
        b"IAU_EARTH",
        b"NONE",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    spicelib::SPKGEO(
        401001,
        ET,
        b"IAU_EARTH",
        301001,
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        3,
        0.0000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000000000001, OK, ctx)?;

    testutil::TCASE(b"Make sure that SPKEZP returns the same thing as the combination of SPKSSB and SPKAPO when an inertial frame is the requested output frame. Light time only correction.", ctx)?;

    spicelib::SPKEZP(
        301001,
        ET,
        b"J2000",
        b"LT",
        399001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    spicelib::SPKSSB(399001, ET, b"J2000", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        301001,
        ET,
        b"J2000",
        SOBS.as_slice(),
        b"LT",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"=",
        ESTATE.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"=", ELT, 0.0, OK, ctx)?;

    testutil::TCASE(b"Repeat the previous test for the transmission case.", ctx)?;

    spicelib::SPKEZP(
        301001,
        ET,
        b"J2000",
        b"XLT",
        399001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    spicelib::SPKSSB(399001, ET, b"J2000", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        301001,
        ET,
        b"J2000",
        SOBS.as_slice(),
        b"XLT",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"=",
        ESTATE.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"=", ELT, 0.0, OK, ctx)?;

    testutil::TCASE(b"Make sure that SPKEZP returns the same thing as the combination of SPKSSB and SPKAPO when an inertial frame is the requested output frame. Converged Newtonian plus stellar aberation", ctx)?;

    spicelib::SPKEZP(
        301001,
        ET,
        b"J2000",
        b"CN+S",
        399001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    spicelib::SPKSSB(399001, ET, b"J2000", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        301001,
        ET,
        b"J2000",
        SOBS.as_slice(),
        b"CN+S",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"=",
        ESTATE.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"=", ELT, 0.0, OK, ctx)?;

    testutil::TCASE(b"Repeat the previous test for the transmission case.", ctx)?;

    spicelib::SPKEZP(
        301001,
        ET,
        b"J2000",
        b"XCN+S",
        399001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    spicelib::SPKSSB(399001, ET, b"J2000", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        301001,
        ET,
        b"J2000",
        SOBS.as_slice(),
        b"XCN+S",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"=",
        ESTATE.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"=", ELT, 0.0, OK, ctx)?;

    testutil::TCASE(b"Perform an independent test to see if \"apparent\" positions in non-inertial frames are properly computed. ", ctx)?;

    spicelib::SPKEZP(
        401001,
        ET,
        b"IAU_EARTH",
        b"CN+S",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    spicelib::SPKSSB(301001, ET, b"J2000", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPP(
        401001,
        ET,
        b"J2000",
        SOBS.as_slice(),
        b"CN+S",
        ISTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;
    spicelib::SPKAPP(
        399,
        ET,
        b"J2000",
        SOBS.as_slice(),
        b"CN+S",
        TSTATE.as_slice_mut(),
        &mut CLT,
        ctx,
    )?;
    spicelib::SXFORM(
        b"J2000",
        b"IAU_EARTH",
        (ET - CLT),
        XFORM.as_slice_mut(),
        ctx,
    )?;

    spicelib::MXVG(
        XFORM.as_slice(),
        ISTATE.as_slice(),
        6,
        6,
        ESTATE.as_slice_mut(),
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        3,
        0.00000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.000000000001, OK, ctx)?;

    testutil::TCASE(b"Repeat the previous test for the transmission case.", ctx)?;

    spicelib::SPKEZP(
        401001,
        ET,
        b"IAU_EARTH",
        b"XCN+S",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    spicelib::SPKSSB(301001, ET, b"J2000", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPP(
        401001,
        ET,
        b"J2000",
        SOBS.as_slice(),
        b"XCN+S",
        ISTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;
    spicelib::SPKAPP(
        399,
        ET,
        b"J2000",
        SOBS.as_slice(),
        b"XCN+S",
        TSTATE.as_slice_mut(),
        &mut CLT,
        ctx,
    )?;
    spicelib::SXFORM(
        b"J2000",
        b"IAU_EARTH",
        (ET + CLT),
        XFORM.as_slice_mut(),
        ctx,
    )?;

    spicelib::MXVG(
        XFORM.as_slice(),
        ISTATE.as_slice(),
        6,
        6,
        ESTATE.as_slice_mut(),
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        3,
        0.000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.000000000001, OK, ctx)?;

    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::KILFIL(b"phoenix2.bsp", ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
