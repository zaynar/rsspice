//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 6;
const NS: i32 = 5;
const LNSIZE: i32 = 80;

//$Procedure      F_SPKF15 ( Family of tests for SPKW15 and SPKR15 )
pub fn F_SPKF15(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    let mut NUMS = StackArray::<i32, 6>::new(1..=NI);
    let mut HANDLE: i32 = 0;
    let mut BODY: i32 = 0;
    let mut CENTER: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut SPKHAN: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Spicelib Functions
    //

    //
    // Local Variables.
    //

    testutil::TOPEN(b"F_SPKF15", ctx)?;

    //
    // delete any existing test SPK file.
    //
    testutil::KILFIL(b"t_spkw15.bsp", ctx)?;
    spicelib::SPCOPN(b"t_spkw15.bsp", b"testSPK", &mut HANDLE, ctx)?;

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

    for I in 1..=20 {
        RECORD[I] = 1.0;
    }

    //
    // The semi-latus rectum is supposed to be positive.  Start
    // out at zero and then set it to something reasonable.
    //
    testutil::TCASE(b"Semi-latus rectum exception", ctx)?;

    P = 0.0;

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
    testutil::CHCKXC(true, b"SPICE(BADLATUSRECTUM)", OK, ctx)?;

    P = 10000.0;

    //
    // Negative eccentricities should produce exceptions.  After
    // checking that this is so set the eccentricity to something
    // yielding a periodic orbit.
    //
    testutil::TCASE(b"Eccentricity Exception", ctx)?;

    ECC = -1.0;

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
    testutil::CHCKXC(true, b"SPICE(BADECCENTRICITY)", OK, ctx)?;

    ECC = 0.1;

    //
    // The central mass must be positive.  Zero or less should
    // trigger an exception. Try zero and -1.  After that we
    // use the mass of the earth.
    //
    testutil::TCASE(b"Central Mass Exception --- mass 0", ctx)?;

    GM = 0.0;

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
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEMASS)", OK, ctx)?;

    testutil::TCASE(b"Central Mass Exception --- mass -1", ctx)?;

    GM = -1.0;

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
    testutil::CHCKXC(true, b"SPICE(NONPOSITIVEMASS)", OK, ctx)?;

    GM = 398600.44770326116;

    //
    // Only a zero trajectory pole can produce a problem.  By
    // construction we already have one.
    //
    testutil::TCASE(b"Trajectory Pole Exception", ctx)?;

    TP[1] = 0.0;
    TP[2] = 0.0;
    TP[3] = 0.0;

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
    testutil::CHCKXC(true, b"SPICE(BADVECTOR)", OK, ctx)?;

    //
    // Set the trajectory pole to 45 degree inclination
    //
    TP[1] = 0.0;
    TP[2] = f64::cos((spicelib::PI(ctx) / 6.0));
    TP[3] = f64::sin((spicelib::PI(ctx) / 6.0));

    //
    // Only a zero periapsis vector yields an exception.  We
    // already have this by construction.  After testing make
    // a periapsis vector that is orthogonal to the trajectory
    // pole vector.
    //
    testutil::TCASE(b"Periapsis Vector Exception", ctx)?;

    PA[1] = 0.0;
    PA[2] = 0.0;
    PA[3] = 0.0;

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
    testutil::CHCKXC(true, b"SPICE(BADVECTOR)", OK, ctx)?;

    PA[1] = 0.0;
    PA[2] = f64::sin((spicelib::PI(ctx) / 6.0));
    PA[3] = -f64::cos((spicelib::PI(ctx) / 6.0));

    //
    // Only a zero central body pole vector can yield an exception.
    // We have such a situation by construction.  After checking
    // this, align the pole with the Z axis.
    //
    testutil::TCASE(b"Pole Vector Exception", ctx)?;

    PV[1] = 0.0;
    PV[2] = 0.0;
    PV[3] = 0.0;

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
    testutil::CHCKXC(true, b"SPICE(BADVECTOR)", OK, ctx)?;

    PV[3] = 1.0;

    //
    // Anything less than zero should trigger an exception.  After
    // checking, set the equatorial radius to that of the earth.
    //
    testutil::TCASE(b"Equatorial Radius Exception", ctx)?;

    RADIUS = -1.0;

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
    testutil::CHCKXC(true, b"SPICE(BADRADIUS)", OK, ctx)?;

    RADIUS = 6378.184;

    //
    // If the periapse is not nearly perpepndicular to the
    // trajectory pole, we should get an exception.  Create
    // a vector that isn't perpendicular to the trajectory pole
    // by messing up the sign on the z-component.
    //
    testutil::TCASE(b"Bad Initial Conditions", ctx)?;

    PA[1] = 0.0;
    PA[2] = 1.0;
    PA[3] = 0.0;

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
    testutil::CHCKXC(true, b"SPICE(BADINITSTATE)", OK, ctx)?;

    PA[1] = 0.0;
    PA[2] = f64::sin((spicelib::PI(ctx) / 6.0));
    PA[3] = -f64::cos((spicelib::PI(ctx) / 6.0));

    testutil::TCASE(b"Segment Identifier too long", ctx)?;

    fstr::assign(
        &mut SEGID,
        b"This is a very, very, very long segment identifier ",
    );

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
    testutil::CHCKXC(true, b"SPICE(SEGIDTOOLONG)", OK, ctx)?;

    testutil::TCASE(b"Non-Printing Characters Exception", ctx)?;

    fstr::assign(
        &mut SEGID,
        &fstr::concat(
            &fstr::concat(b"This is a ", &intrinsics::CHAR(9)),
            b"test segment",
        ),
    );

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
    testutil::CHCKXC(true, b"SPICE(NONPRINTABLECHARS)", OK, ctx)?;

    fstr::assign(&mut SEGID, b"Test segment");

    //
    // That takes care of all noted excpetions in  SPKW15.
    // Write a legitimate segment and close the SPK file.
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

    testutil::TCASE(b"Writing a segment", ctx)?;

    FIRST = 1000.0;
    LAST = 100000.0;

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

    //
    // In addition we write a bogus segment with the wrong amount
    // of data in it and call it type 15.
    //
    FIRST = -100000.0;
    LAST = -1000.0;
    TYPE = 15;

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

    spicelib::DAFBNA(HANDLE, DESCR.as_slice(), b"Bogus Segment", ctx)?;
    spicelib::DAFADA(RECORD.as_slice(), 17, ctx)?;
    spicelib::DAFENA(ctx)?;
    spicelib::DAFCLS(HANDLE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Corrupt the descriptor in the type component of the segment
    // and make sure that SPKR15 properly diagnoses the problem.
    //
    testutil::TCASE(b"SPKR15 bad type exception.", ctx)?;

    ET = 2000.0;

    spicelib::SPKLEF(b"t_spkw15.bsp", &mut SPKHAN, ctx)?;
    spicelib::SPKSFS(
        BODY,
        ET,
        &mut HANDLE,
        DESCR.as_slice_mut(),
        &mut SEGID,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    if *OK {
        spicelib::DAFUS(
            DESCR.as_slice(),
            ND,
            NI,
            DPS.as_slice_mut(),
            NUMS.as_slice_mut(),
        );
        NUMS[4] = 14;
        spicelib::DAFPS(
            ND,
            NI,
            DPS.as_slice(),
            NUMS.as_slice(),
            DESCR.as_slice_mut(),
        );

        spicelib::SPKR15(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, b"SPICE(WRONGSPKTYPE)", OK, ctx)?;
    }
    //
    // Recall that the second segment we wrote had too much data.
    // and had time bounds from -100000 to -1000.  We find that
    // segment next and make sure that the badly formed segment
    // is handled properly.
    //

    testutil::TCASE(b"SPKR15 bad segment exception.", ctx)?;

    ET = -2000.0;

    spicelib::SPKSFS(
        BODY,
        ET,
        &mut HANDLE,
        DESCR.as_slice_mut(),
        &mut SEGID,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    if *OK {
        spicelib::SPKR15(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        testutil::CHCKXC(true, b"SPICE(MALFORMEDSEGMENT)", OK, ctx)?;
    }

    testutil::TCASE(b"SPKR15 checking segment values.", ctx)?;

    ET = 2000.0;
    spicelib::SPKSFS(
        BODY,
        ET,
        &mut HANDLE,
        DESCR.as_slice_mut(),
        &mut SEGID,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    if *OK {
        spicelib::SPKR15(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKAD(
            b"RECORD",
            RECORD.as_slice(),
            b"=",
            MYREC.as_slice(),
            16,
            0.00000000000001,
            OK,
            ctx,
        )?;
    }

    //
    // When we've finished all tests, delete the .bsp file we created.
    //
    spicelib::SPKUEF(SPKHAN, ctx)?;
    testutil::KILFIL(b"t_spkw15.bsp", ctx)?;
    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
