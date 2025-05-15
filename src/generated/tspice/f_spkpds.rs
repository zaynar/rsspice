//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 6;

//$Procedure      F_SPKPDS ( Test routine for SPKPDS )
pub fn F_SPKPDS(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BODY: i32 = 0;
    let mut CENTER: i32 = 0;
    let mut FRAME = [b' '; 8 as usize];
    let mut REFCOD: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut FIRST: f64 = 0.0;
    let mut LAST: f64 = 0.0;
    let mut DESCR = StackArray::<f64, 5>::new(1..=5);
    let mut DPS = StackArray::<f64, 2>::new(1..=2);
    let mut NUMS = StackArray::<i32, 6>::new(1..=6);
    let mut CHTMP = [b' '; 8 as usize];
    let mut DPTMP1: f64 = 0.0;
    let mut DPTMP2: f64 = 0.0;
    let mut INTMP: i32 = 0;

    testutil::TOPEN(b"F_SPKPDS", ctx)?;

    //
    // Set up a legitimate set of inputs and then corrupt them
    // one at a time to trigger errors.
    //
    BODY = 399;
    CENTER = 3;
    fstr::assign(&mut FRAME, b"J2000");
    REFCOD = 1;
    TYPE = 3;
    FIRST = -1000000.0;
    LAST = 1000000.0;

    //
    // Now we modify one of each value to trigger an exception.  We
    // always set the value back to the original value when we get
    // done with a test case so that we only have to modify a single
    // term in the next test case.
    //
    testutil::TCASE(b"Exception: Ephemeris for Solar System Barycenter", ctx)?;

    INTMP = BODY;
    BODY = 0;

    spicelib::SPKPDS(
        BODY,
        CENTER,
        &FRAME,
        TYPE,
        FIRST,
        LAST,
        DESCR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BARYCENTEREPHEM)", OK, ctx)?;

    BODY = INTMP;

    testutil::TCASE(b"Exception: Body and Center the same.", ctx)?;

    INTMP = CENTER;
    CENTER = BODY;

    spicelib::SPKPDS(
        BODY,
        CENTER,
        &FRAME,
        TYPE,
        FIRST,
        LAST,
        DESCR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODYANDCENTERSAME)", OK, ctx)?;

    CENTER = INTMP;

    testutil::TCASE(b"Exception: Invalid Reference Frame", ctx)?;

    fstr::assign(&mut CHTMP, &FRAME);
    fstr::assign(&mut FRAME, b"J3000");

    spicelib::SPKPDS(
        BODY,
        CENTER,
        &FRAME,
        TYPE,
        FIRST,
        LAST,
        DESCR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDREFFRAME)", OK, ctx)?;

    fstr::assign(&mut FRAME, &CHTMP);

    testutil::TCASE(b"Exception: Start and Stop times out of order", ctx)?;

    DPTMP1 = FIRST;
    DPTMP2 = LAST;

    FIRST = 1000000.0;
    LAST = -1000000.0;

    spicelib::SPKPDS(
        BODY,
        CENTER,
        &FRAME,
        TYPE,
        FIRST,
        LAST,
        DESCR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    FIRST = DPTMP1;
    LAST = DPTMP2;

    testutil::TCASE(b"Exception: Unknown SPK data type 1001.", ctx)?;

    INTMP = TYPE;
    TYPE = 1001;

    spicelib::SPKPDS(
        BODY,
        CENTER,
        &FRAME,
        TYPE,
        FIRST,
        LAST,
        DESCR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(UNKNOWNSPKTYPE)", OK, ctx)?;

    TYPE = INTMP;

    testutil::TCASE(b"Exception: Unknown SPK data type 0.", ctx)?;

    INTMP = TYPE;
    TYPE = 0;

    spicelib::SPKPDS(
        BODY,
        CENTER,
        &FRAME,
        TYPE,
        FIRST,
        LAST,
        DESCR.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(UNKNOWNSPKTYPE)", OK, ctx)?;

    TYPE = INTMP;

    testutil::TCASE(b"Check Contents of descriptor", ctx)?;

    spicelib::SPKPDS(
        BODY,
        CENTER,
        &FRAME,
        TYPE,
        FIRST,
        LAST,
        DESCR.as_slice_mut(),
        ctx,
    )?;
    spicelib::DAFUS(
        DESCR.as_slice(),
        ND,
        NI,
        DPS.as_slice_mut(),
        NUMS.as_slice_mut(),
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"FIRST", FIRST, b"=", DPS[1], 0.0, OK, ctx)?;
    testutil::CHCKSD(b"LAST", LAST, b"=", DPS[2], 0.0, OK, ctx)?;

    testutil::CHCKSI(b"BODY", BODY, b"=", NUMS[1], 0, OK, ctx)?;
    testutil::CHCKSI(b"CENTER", CENTER, b"=", NUMS[2], 0, OK, ctx)?;
    testutil::CHCKSI(b"REFCOD", REFCOD, b"=", NUMS[3], 0, OK, ctx)?;
    testutil::CHCKSI(b"TYPE", TYPE, b"=", NUMS[4], 0, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
