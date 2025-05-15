//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

//$Procedure      F_SPKEZ ( Family of tests for SPKEZ)
pub fn F_SPKEZ(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut CLT: f64 = 0.0;
    let mut DLT: f64 = 0.0;
    let mut ELT: f64 = 0.0;
    let mut ESTATE = StackArray::<f64, 6>::new(1..=6);
    let mut ET: f64 = 0.0;
    let mut ISTATE = StackArray::<f64, 6>::new(1..=6);
    let mut LT: f64 = 0.0;
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STOBS = StackArray::<f64, 6>::new(1..=6);
    let mut TSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut HANDLE: i32 = 0;

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
    testutil::TOPEN(b"F_SPKEZ", ctx)?;

    testutil::KILFIL(b"phoenix2.bsp", ctx)?;
    testutil::KILFIL(b"phoenix2.ker", ctx)?;

    testutil::TSTSPK(b"phoenix2.bsp", true, &mut HANDLE, ctx)?;

    testutil::TSTPCK(b"phoenix2.ker", true, false, ctx)?;
    spicelib::TPARSE(b"1 JAN 1995", &mut ET, &mut ERROR, ctx)?;

    testutil::TCASE(b"Make sure that SPKEZ and SPKGEO return the same information when the correction requested is NONE. ", ctx)?;

    spicelib::SPKEZ(
        401001,
        ET,
        b"IAU_EARTH",
        b"NONE",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
        b"=",
        ESTATE.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"=", ELT, 0.0, OK, ctx)?;

    testutil::TCASE(b"Make sure that SPKEZ returns the same thing as SPKLTC when an inertial frame is the requested output frame. Light time only correction.", ctx)?;

    spicelib::SPKEZ(
        301001,
        ET,
        b"J2000",
        b"LT",
        399001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKSSB(399001, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        301001,
        ET,
        b"J2000",
        b"LT",
        STOBS.as_slice(),
        ESTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"=",
        ESTATE.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"=", ELT, 0.0, OK, ctx)?;

    testutil::TCASE(b"Repeat the previous test for the transmission case.", ctx)?;

    spicelib::SPKEZ(
        301001,
        ET,
        b"J2000",
        b"XLT",
        399001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKSSB(399001, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        301001,
        ET,
        b"J2000",
        b"XLT",
        STOBS.as_slice(),
        ESTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"=",
        ESTATE.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"=", ELT, 0.0, OK, ctx)?;

    testutil::TCASE(b"Make sure that SPKEZ returns the same thing as SPKACS when an inertial frame is the requested output frame. Converged Newtonian plus stellar aberation", ctx)?;

    spicelib::SPKEZ(
        301001,
        ET,
        b"J2000",
        b"CN+S",
        399001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        301001,
        ET,
        b"J2000",
        b"CN+S",
        399001,
        ESTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"=",
        ESTATE.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"=", ELT, 0.0, OK, ctx)?;

    testutil::TCASE(b"Repeat the previous test for the transmission case.", ctx)?;

    spicelib::SPKEZ(
        301001,
        ET,
        b"J2000",
        b"XCN+S",
        399001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        301001,
        ET,
        b"J2000",
        b"XCN+S",
        399001,
        ESTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"=",
        ESTATE.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"=", ELT, 0.0, OK, ctx)?;

    testutil::TCASE(b"Perform an independent test to see if \"apparent\" positions in non-inertial frames are properly computed. Frame center is not target or observer.", ctx)?;

    spicelib::SPKEZ(
        401001,
        ET,
        b"IAU_EARTH",
        b"CN+S",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        401001,
        ET,
        b"J2000",
        b"CN+S",
        301001,
        ISTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKSSB(301001, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        399,
        ET,
        b"J2000",
        b"CN+S",
        STOBS.as_slice(),
        TSTATE.as_slice_mut(),
        &mut CLT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"J2000",
        b"IAU_EARTH",
        (ET - CLT),
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Scale the derivative block to account for the rate of change
    // of light time.
    //
    for J in 4..=6 {
        for I in 1..=3 {
            XFORM[[J, I]] = ((1.0 - DLT) * XFORM[[J, I]]);
        }
    }

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
        6,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000000000001, OK, ctx)?;

    testutil::TCASE(b"Repeat the previous test for the transmission case.", ctx)?;

    spicelib::SPKEZ(
        401001,
        ET,
        b"IAU_EARTH",
        b"XCN+S",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        401001,
        ET,
        b"J2000",
        b"XCN+S",
        301001,
        ISTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKSSB(301001, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        399,
        ET,
        b"J2000",
        b"XCN+S",
        STOBS.as_slice(),
        TSTATE.as_slice_mut(),
        &mut CLT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"J2000",
        b"IAU_EARTH",
        (ET + CLT),
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Scale the derivative block to account for the rate of change
    // of light time.
    //
    for J in 4..=6 {
        for I in 1..=3 {
            XFORM[[J, I]] = ((1.0 + DLT) * XFORM[[J, I]]);
        }
    }

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
        6,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000000000001, OK, ctx)?;

    testutil::TCASE(b"Perform an independent test to see if \"apparent\" positions in non-inertial frames are properly computed. Frame center is target.", ctx)?;

    spicelib::SPKEZ(
        399,
        ET,
        b"IAU_EARTH",
        b"CN+S",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        399,
        ET,
        b"J2000",
        b"CN+S",
        301001,
        ISTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"J2000",
        b"IAU_EARTH",
        (ET - ELT),
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Scale the derivative block to account for the rate of change
    // of light time.
    //
    for J in 4..=6 {
        for I in 1..=3 {
            XFORM[[J, I]] = ((1.0 - DLT) * XFORM[[J, I]]);
        }
    }

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
        6,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000000000001, OK, ctx)?;

    testutil::TCASE(b"Repeat the previous test for the transmission case.", ctx)?;

    spicelib::SPKEZ(
        399,
        ET,
        b"IAU_EARTH",
        b"XCN+S",
        301001,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        399,
        ET,
        b"J2000",
        b"XCN+S",
        301001,
        ISTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"J2000",
        b"IAU_EARTH",
        (ET + ELT),
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Scale the derivative block to account for the rate of change
    // of light time.
    //
    for J in 4..=6 {
        for I in 1..=3 {
            XFORM[[J, I]] = ((1.0 + DLT) * XFORM[[J, I]]);
        }
    }

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
        6,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000000000001, OK, ctx)?;

    testutil::TCASE(b"Perform an independent test to see if \"apparent\" positions in non-inertial frames are properly computed. Frame center is observer.", ctx)?;

    spicelib::SPKEZ(
        301001,
        ET,
        b"IAU_EARTH",
        b"CN+S",
        399,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        301001,
        ET,
        b"J2000",
        b"CN+S",
        399,
        ISTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(b"J2000", b"IAU_EARTH", ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
        6,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000000000001, OK, ctx)?;

    testutil::TCASE(b"Repeat the previous test for the transmission case.", ctx)?;

    spicelib::SPKEZ(
        301001,
        ET,
        b"IAU_EARTH",
        b"XCN+S",
        399,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKACS(
        301001,
        ET,
        b"J2000",
        b"XCN+S",
        399,
        ISTATE.as_slice_mut(),
        &mut ELT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(b"J2000", b"IAU_EARTH", ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
        6,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000000000001, OK, ctx)?;

    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::KILFIL(b"phoenix2.bsp", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
