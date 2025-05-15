//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 6;
const NS: i32 = 5;
const LNSIZE: i32 = 80;

//$Procedure F_SPKSPV ( Family of tests for SPKS15 )
pub fn F_SPKSPV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FRAME = [b' '; 8 as usize];
    let mut SEGID = [b' '; LNSIZE as usize];
    let mut ET: f64 = 0.0;
    let mut FIRST: f64 = 0.0;
    let mut LAST: f64 = 0.0;
    let mut J2FLG: f64 = 0.0;
    let mut EPOCH: f64 = 0.0;
    let mut TP = StackArray::<f64, 3>::new(1..=3);
    let mut PA = StackArray::<f64, 3>::new(1..=3);
    let mut PV = StackArray::<f64, 3>::new(1..=3);
    let mut GM: f64 = 0.0;
    let mut P: f64 = 0.0;
    let mut ECC: f64 = 0.0;
    let mut J2: f64 = 0.0;
    let mut RADIUS: f64 = 0.0;
    let mut RECORD = StackArray::<f64, 20>::new(1..=20);
    let mut MYREC = StackArray::<f64, 16>::new(1..=16);
    let mut DESCR = StackArray::<f64, 5>::new(1..=NS);
    let mut DPS = StackArray::<f64, 2>::new(1..=ND);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STATE2 = StackArray::<f64, 6>::new(1..=6);
    let mut NUMS = StackArray::<i32, 6>::new(1..=NI);
    let mut HANDLE: i32 = 0;
    let mut OLDH: i32 = 0;
    let mut NEWH: i32 = 0;
    let mut BODY: i32 = 0;
    let mut CENTER: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Spicelib Functions
    //
    //
    // Local Variables.
    //

    testutil::TOPEN(b"F_SPKSPV", ctx)?;

    //
    // Set up a bunch of initial values.
    //

    BODY = -1000;
    CENTER = 399;
    fstr::assign(&mut FRAME, b"J2000");
    FIRST = 0.0;
    LAST = 100000.0;
    fstr::assign(&mut SEGID, b" ");
    EPOCH = 1000.0;
    J2FLG = 0.0;
    J2 = 0.001082616;

    for I in 1..=20 {
        RECORD[I] = 1.0;
    }

    //
    // Set up the data needed for creating a legitimate segment.
    //
    BODY = -1000;
    CENTER = 399;

    fstr::assign(&mut FRAME, b"J2000");

    FIRST = 0.0;
    LAST = 100000.0;

    fstr::assign(&mut SEGID, b" ");

    EPOCH = 1000.0;
    J2FLG = 0.0;
    J2 = 0.001082616;

    P = 10000.0;
    ECC = 0.1;
    GM = 398600.44770326116;

    TP[1] = 0.0;
    TP[2] = f64::cos((spicelib::PI(ctx) / 6.0));
    TP[3] = f64::sin((spicelib::PI(ctx) / 6.0));

    PA[1] = 0.0;
    PA[2] = f64::sin((spicelib::PI(ctx) / 6.0));
    PA[3] = -f64::cos((spicelib::PI(ctx) / 6.0));

    PV[1] = 0.0;
    PV[2] = 0.0;
    PV[3] = 1.0;
    RADIUS = 6378.184;

    fstr::assign(&mut SEGID, b"Test segment");

    //
    // Copy the record we expect to be in the SPK file.
    //
    MYREC[1] = EPOCH;

    MYREC[2] = TP[1];
    MYREC[3] = TP[2];
    MYREC[4] = TP[3];

    MYREC[5] = PA[1];
    MYREC[6] = PA[2];
    MYREC[7] = PA[3];
    MYREC[8] = P;
    MYREC[9] = ECC;

    MYREC[10] = J2FLG;

    MYREC[11] = PV[1];
    MYREC[12] = PV[2];
    MYREC[13] = PV[3];
    MYREC[14] = GM;
    MYREC[15] = J2;
    MYREC[16] = RADIUS;

    testutil::TCASE(b"Bad Time range exception.", ctx)?;

    //
    // delete any existing test SPK file.
    //
    testutil::KILFIL(b"t_spks15.bsp", ctx)?;
    spicelib::SPCOPN(b"t_spks15.bsp", b"testSPK", &mut HANDLE, ctx)?;

    FIRST = 1000.0;
    LAST = 100000.0;
    ET = ((FIRST + LAST) / 2.0);

    spicelib::SPKW15(
        HANDLE,
        BODY,
        CENTER,
        &FRAME,
        FIRST,
        LAST,
        &SEGID,
        EPOCH,
        TP.as_slice(),
        PA.as_slice(),
        P,
        ECC,
        J2FLG,
        PV.as_slice(),
        GM,
        J2,
        RADIUS,
        ctx,
    )?;

    spicelib::DAFCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKLEF(b"t_spks15.bsp", &mut OLDH, ctx)?;
    spicelib::SPKSFS(
        BODY,
        ET,
        &mut OLDH,
        DESCR.as_slice_mut(),
        &mut SEGID,
        &mut FOUND,
        ctx,
    )?;

    testutil::KILFIL(b"spksub15.bsp", ctx)?;
    spicelib::SPCOPN(b"spksub15.bsp", b"testSPK", &mut NEWH, ctx)?;

    FIRST = (FIRST - 1000.0);
    LAST = (LAST + 1000.0);

    spicelib::SPKSUB(OLDH, DESCR.as_slice(), &SEGID, FIRST, LAST, NEWH, ctx)?;
    testutil::CHCKXC(true, b"SPICE(SPKNOTASUBSET)", OK, ctx)?;

    testutil::TCASE(b"Extracting a subsegment", ctx)?;

    FIRST = (FIRST + 2000.0);
    LAST = (LAST - 2000.0);

    spicelib::SPKSUB(OLDH, DESCR.as_slice(), &SEGID, FIRST, LAST, NEWH, ctx)?;
    spicelib::DAFCLS(NEWH, ctx)?;
    //
    // No errors should have been signaled so far.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Load the subsetted ephemeris.
    //
    spicelib::SPKUEF(OLDH, ctx)?;
    spicelib::SPKLEF(b"spksub15.bsp", &mut NEWH, ctx)?;
    //
    // Look for data for this item.
    //

    spicelib::SPKSFS(
        BODY,
        ET,
        &mut HANDLE,
        DESCR.as_slice_mut(),
        &mut SEGID,
        &mut FOUND,
        ctx,
    )?;
    spicelib::SPKR15(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
    //
    // The data in the record should be identical to the one we
    // stored earlier.
    //
    testutil::CHCKAD(
        b"RECORD",
        RECORD.as_slice(),
        b"=",
        MYREC.as_slice(),
        16,
        0.0,
        OK,
        ctx,
    )?;

    //
    // The stop and end times should be what we set them to earlier.
    //
    spicelib::DAFUS(
        DESCR.as_slice(),
        ND,
        NI,
        DPS.as_slice_mut(),
        NUMS.as_slice_mut(),
    );

    testutil::CHCKSD(b"FIRST", FIRST, b"=", DPS[1], 0.0, OK, ctx)?;
    testutil::CHCKSD(b"LAST", LAST, b"=", DPS[2], 0.0, OK, ctx)?;

    testutil::TCASE(b"State Evaluation using SPKPV", ctx)?;

    spicelib::SPKPV(
        HANDLE,
        DESCR.as_slice(),
        ET,
        b"J2000",
        STATE.as_slice_mut(),
        &mut CENTER,
        ctx,
    )?;
    spicelib::SPKE15(ET, RECORD.as_slice(), STATE2.as_slice_mut(), ctx)?;

    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"=",
        STATE2.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    //
    // When we've finished all tests, delete the .bsp files we created.
    //
    spicelib::SPKUEF(NEWH, ctx)?;
    testutil::KILFIL(b"t_spks15.bsp", ctx)?;
    testutil::KILFIL(b"spksub15.bsp", ctx)?;
    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
