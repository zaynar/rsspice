//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const LNSIZE: i32 = 80;

//$Procedure      F_SPKPVN ( Family of routine tests for SPKPVN )
pub fn F_SPKPVN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IDENT = [b' '; 40 as usize];
    let mut BUFFER = ActualCharArray::new(LNSIZE, 1..=20);
    let mut THSFRM = ActualCharArray::new(WDSIZE, 1..=5);
    let mut DESCR = StackArray::<f64, 5>::new(1..=5);
    let mut ET: f64 = 0.0;
    let mut GM: f64 = 0.0;
    let mut CSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut ESTATE = StackArray::<f64, 6>::new(1..=6);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut BODY = StackArray::<i32, 14>::new(1..=14);
    let mut ECENTR: i32 = 0;
    let mut CENTER: i32 = 0;
    let mut EFRAME: i32 = 0;
    let mut FRAME: i32 = 0;
    let mut CKHAN: i32 = 0;
    let mut NFRAMES: i32 = 0;
    let mut OFRAME: i32 = 0;
    let mut SPKHAN: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut NLINES: i32 = 0;
    let mut FOUND: bool = false;

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
    testutil::TOPEN(b"F_SPKPVN", ctx)?;
    //
    // Create a test kernels for use with this test family.
    //
    fstr::assign(BUFFER.get_mut(1), b" ");
    fstr::assign(
        BUFFER.get_mut(2),
        b"This is a test frame kernel for use in testing various ",
    );
    fstr::assign(
        BUFFER.get_mut(3),
        b"aspects of the SPK system.  It is intended for testing ",
    );
    fstr::assign(BUFFER.get_mut(4), b"purposes only. ");
    fstr::assign(BUFFER.get_mut(5), b" ");
    testutil::BEGDAT(&mut BUFFER[6]);
    fstr::assign(BUFFER.get_mut(7), b" ");
    fstr::assign(BUFFER.get_mut(8), b"FRAME_-9999_CLASS      =  3 ");
    fstr::assign(BUFFER.get_mut(9), b"FRAME_-9999_CENTER     = -9 ");
    fstr::assign(BUFFER.get_mut(10), b"FRAME_-9999_CLASS_ID   = -9999 ");
    fstr::assign(BUFFER.get_mut(11), b"FRAME_-9999_NAME       = \'PHOENIX\' ");
    fstr::assign(BUFFER.get_mut(12), b"FRAME_PHOENIX          = -9999 ");
    fstr::assign(BUFFER.get_mut(13), b" ");
    NLINES = 13;

    spicelib::CLPOOL(ctx)?;
    testutil::KILFIL(b"test_pck.ker", ctx)?;
    testutil::TSTTXT(b"frames.ker", BUFFER.as_arg(), NLINES, true, false, ctx)?;
    testutil::TSTSPK(b"test_spk.bsp", true, &mut SPKHAN, ctx)?;
    testutil::TSTPCK(b"test_pck.ker", true, false, ctx)?;
    testutil::TSTCK3(
        b"test_ck.bc",
        b"test_sclk.ker",
        false,
        true,
        false,
        &mut CKHAN,
        ctx,
    )?;
    spicelib::CKLPF(b"test_ck.bc", &mut CKHAN, ctx)?;

    testutil::TCASE(
        b"Make sure that we can get the recorded states that are written from TSTSPK. ",
        ctx,
    )?;

    BODY[1] = 1;
    BODY[2] = 2;
    BODY[3] = 3;
    BODY[4] = 4;
    BODY[5] = 5;
    BODY[6] = 6;
    BODY[7] = 7;
    BODY[8] = 8;
    BODY[9] = 9;
    BODY[10] = 301;
    BODY[11] = 401;
    BODY[12] = 501;
    BODY[13] = -9;
    BODY[14] = 399001;

    ET = -10000000.0;

    for I in 1..=14 {
        //
        // Fetch a predicted state from the test SPK file generator.
        //
        testutil::TSTST(
            BODY[I],
            ET,
            &mut IDENT,
            &mut EFRAME,
            ESTATE.as_slice_mut(),
            &mut ECENTR,
            &mut GM,
            ctx,
        )?;
        spicelib::SPKSFS(
            BODY[I],
            ET,
            &mut HANDLE,
            DESCR.as_slice_mut(),
            &mut IDENT,
            &mut FOUND,
            ctx,
        )?;

        if !FOUND {
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
        } else {
            spicelib::SPKPVN(
                HANDLE,
                DESCR.as_slice(),
                ET,
                &mut FRAME,
                STATE.as_slice_mut(),
                &mut CENTER,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSI(b"CENTER", CENTER, b"=", ECENTR, 0, OK, ctx)?;
            testutil::CHCKSI(b"FRAME", FRAME, b"=", EFRAME, 0, OK, ctx)?;
            testutil::CHCKAD(
                b"STATE",
                STATE.as_slice(),
                b"~",
                ESTATE.as_slice(),
                6,
                0.0000000000001,
                OK,
                ctx,
            )?;
        }
    }

    testutil::TCASE(b"Examine the results of SPKPV to make sure that they are compatible with what is expected using the states returned by TSTST ", ctx)?;

    fstr::assign(THSFRM.get_mut(1), b"J2000");
    fstr::assign(THSFRM.get_mut(2), b"B1950");
    fstr::assign(THSFRM.get_mut(3), b"J2000");
    fstr::assign(THSFRM.get_mut(4), b"IAU_EARTH");
    fstr::assign(THSFRM.get_mut(5), b"PHOENIX");

    NFRAMES = 5;

    testutil::TSTST(
        301,
        ET,
        &mut IDENT,
        &mut EFRAME,
        ESTATE.as_slice_mut(),
        &mut ECENTR,
        &mut GM,
        ctx,
    )?;
    spicelib::SPKSFS(
        301,
        ET,
        &mut HANDLE,
        DESCR.as_slice_mut(),
        &mut IDENT,
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    } else {
        for I in 1..=NFRAMES {
            spicelib::NAMFRM(&THSFRM[I], &mut OFRAME, ctx)?;

            spicelib::FRMCHG(EFRAME, OFRAME, ET, XFORM.as_slice_mut(), ctx)?;
            spicelib::MXVG(
                XFORM.as_slice(),
                ESTATE.as_slice(),
                6,
                6,
                CSTATE.as_slice_mut(),
            );

            spicelib::SPKPV(
                HANDLE,
                DESCR.as_slice(),
                ET,
                &THSFRM[I],
                STATE.as_slice_mut(),
                &mut CENTER,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSI(b"CENTER", CENTER, b"=", ECENTR, 0, OK, ctx)?;
            testutil::CHCKAD(
                b"STATE",
                STATE.as_slice(),
                b"~",
                CSTATE.as_slice(),
                6,
                0.0000000000001,
                OK,
                ctx,
            )?;
        }
    }
    //
    // That's it.  Unload the SPK and CK files.
    //
    spicelib::SPKUEF(SPKHAN, ctx)?;
    spicelib::CKUPF(CKHAN, ctx)?;

    testutil::KILFIL(b"test_spk.bsp", ctx)?;
    testutil::KILFIL(b"test_ck.bc", ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
