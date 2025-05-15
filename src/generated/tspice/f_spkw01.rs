//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SPK: &[u8] = b"spk_test_01.bsp";
const DEFSID: &[u8] = b"spkw01 test segment";
const DLSIZE: i32 = 71;
const DLMAX: i32 = 10000;
const SIDLEN: i32 = 40;
const FRNMLN: i32 = 32;
const LNSIZE: i32 = 80;
const ND: i32 = 2;
const NI: i32 = 6;
const DSCSIZ: i32 = 5;

struct SaveVars {
    SEGID: Vec<u8>,
    FRAME: Vec<u8>,
    LABEL: Vec<u8>,
    DC: StackArray<f64, 2>,
    DESCR: StackArray<f64, 5>,
    DLBUFF: ActualArray2D<f64>,
    TBUFF: ActualArray<f64>,
    FIRST: f64,
    LAST: f64,
    ET: f64,
    T1REC: StackArray<f64, 71>,
    BODY: i32,
    CENTER: i32,
    FRCODE: i32,
    HANDLE: i32,
    IC: StackArray<i32, 6>,
    N: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SEGID = vec![b' '; LNSIZE as usize];
        let mut FRAME = vec![b' '; FRNMLN as usize];
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut DC = StackArray::<f64, 2>::new(1..=ND);
        let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
        let mut DLBUFF = ActualArray2D::<f64>::new(1..=DLSIZE, 1..=DLMAX);
        let mut TBUFF = ActualArray::<f64>::new(1..=DLMAX);
        let mut FIRST: f64 = 0.0;
        let mut LAST: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut T1REC = StackArray::<f64, 71>::new(1..=DLSIZE);
        let mut BODY: i32 = 0;
        let mut CENTER: i32 = 0;
        let mut FRCODE: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut IC = StackArray::<i32, 6>::new(1..=NI);
        let mut N: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            SEGID,
            FRAME,
            LABEL,
            DC,
            DESCR,
            DLBUFF,
            TBUFF,
            FIRST,
            LAST,
            ET,
            T1REC,
            BODY,
            CENTER,
            FRCODE,
            HANDLE,
            IC,
            N,
            FOUND,
        }
    }
}

//$Procedure      F_SPKW01 ( SPKW01 routine tests )
pub fn F_SPKW01(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_SPKW01", ctx)?;

    //
    // Open a new SPK file for writing.
    //
    testutil::TCASE(b"Setup:  open a new SPK file for writing.", ctx)?;

    spicelib::SPKOPN(SPK, b" ", 0, &mut save.HANDLE, ctx)?;

    //
    // Initialize the time and data buffers with values that are
    // recognizable but otherwise bogus.
    //
    for I in 1..=DLMAX {
        save.TBUFF[I] = ((I as f64) * 1000.0);

        for J in 1..=DLSIZE {
            save.DLBUFF[[J, I]] = (save.TBUFF[I] + J as f64);
        }
    }

    //
    // Pick body, center, and frame.
    //
    save.BODY = 3;
    save.CENTER = 10;
    fstr::assign(&mut save.FRAME, b"J2000");

    //
    // Initial difference line count.
    //
    save.N = 100;

    //
    // Pick nominal time bounds.
    //
    save.FIRST = 0.0;
    save.LAST = save.TBUFF[save.N];

    //
    // Initialize segment identifier.
    //
    fstr::assign(&mut save.SEGID, DEFSID);

    //
    // Error test cases follow.
    //
    testutil::TCASE(b"Invalid frame.", ctx)?;

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        b"XXX",
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDREFFRAME)", OK, ctx)?;

    testutil::TCASE(b"SEGID too long.", ctx)?;

    fstr::assign(
        &mut save.SEGID,
        b"1234567890123456789012345678912345678901234567890",
    );

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SEGIDTOOLONG)", OK, ctx)?;

    testutil::TCASE(b"SEGID contains non-printable characters.", ctx)?;

    fstr::assign(&mut save.SEGID, DEFSID);

    fstr::assign(
        fstr::substr_mut(&mut save.SEGID, 5..=5),
        &intrinsics::CHAR(7),
    );

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPRINTABLECHARS)", OK, ctx)?;

    testutil::TCASE(b"Invalid difference line count", ctx)?;

    fstr::assign(&mut save.SEGID, DEFSID);
    save.N = 0;

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    save.N = -1;

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    save.N = 100;

    testutil::TCASE(b"Descriptor times out of order", ctx)?;

    save.FIRST = (save.LAST + 1.0);

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    save.FIRST = 0.0;

    testutil::TCASE(b"epochs out of order", ctx)?;

    save.ET = save.TBUFF[3];
    save.TBUFF[3] = save.TBUFF[2];

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TIMESOUTOFORDER)", OK, ctx)?;

    save.TBUFF[3] = save.ET;

    testutil::TCASE(b"Gap following last epoch", ctx)?;

    save.ET = save.TBUFF[save.N];
    save.TBUFF[save.N] = (save.TBUFF[save.N] - 0.000001);

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    save.TBUFF[save.N] = save.ET;

    //
    // Normal cases:
    //

    testutil::TCASE(
        b"Make sure segment containing one difference line is readable.",
        ctx,
    )?;

    save.LAST = save.TBUFF[1];

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        1,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFOPR(SPK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Segment found", save.FOUND, true, OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the descriptor.
    //
    spicelib::NAMFRM(&save.FRAME, &mut save.FRCODE, ctx)?;

    testutil::CHCKSI(b"Body", save.IC[1], b"=", save.BODY, 0, OK, ctx)?;
    testutil::CHCKSI(b"Center", save.IC[2], b"=", save.CENTER, 0, OK, ctx)?;
    testutil::CHCKSI(b"Frame", save.IC[3], b"=", save.FRCODE, 0, OK, ctx)?;
    testutil::CHCKSI(b"Data type", save.IC[4], b"=", 1, 0, OK, ctx)?;

    testutil::CHCKSD(b"Start time", save.DC[1], b"=", save.FIRST, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"Stop time", save.DC[2], b"=", save.LAST, 0.0, OK, ctx)?;

    //
    // Look up the data and compare it to what we put in.  We
    // expect an exact match.
    //
    spicelib::SPKR01(
        save.HANDLE,
        save.DESCR.as_slice(),
        save.TBUFF[1],
        save.T1REC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKAD(
        b"Diff line",
        save.T1REC.as_slice(),
        b"=",
        save.DLBUFF.subarray([1, 1]),
        DLSIZE,
        0.0,
        OK,
        ctx,
    )?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;

    //
    // Repeat the test with DLMAX records.  The new segment will mask
    // the previous one.
    //
    spicelib::SPKOPA(SPK, &mut save.HANDLE, ctx)?;

    save.N = DLMAX;
    save.LAST = save.TBUFF[save.N];

    spicelib::SPKW01(
        save.HANDLE,
        save.BODY,
        save.CENTER,
        &save.FRAME,
        save.FIRST,
        save.LAST,
        &save.SEGID,
        save.N,
        save.DLBUFF.as_slice(),
        save.TBUFF.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Open the file for read access; find the 2nd descriptor.
    //
    spicelib::DAFOPR(SPK, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Segment found", save.FOUND, true, OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFUS(
        save.DESCR.as_slice(),
        ND,
        NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the descriptor.
    //
    spicelib::NAMFRM(&save.FRAME, &mut save.FRCODE, ctx)?;

    testutil::CHCKSI(b"Body", save.IC[1], b"=", save.BODY, 0, OK, ctx)?;
    testutil::CHCKSI(b"Center", save.IC[2], b"=", save.CENTER, 0, OK, ctx)?;
    testutil::CHCKSI(b"Frame", save.IC[3], b"=", save.FRCODE, 0, OK, ctx)?;
    testutil::CHCKSI(b"Data type", save.IC[4], b"=", 1, 0, OK, ctx)?;

    testutil::CHCKSD(b"Start time", save.DC[1], b"=", save.FIRST, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"Stop time", save.DC[2], b"=", save.LAST, 0.0, OK, ctx)?;

    //
    // Look up the data and compare it to what we put in.  We
    // expect an exact match.
    //
    for I in 1..=save.N {
        fstr::assign(&mut save.LABEL, b"Difference line number *, time *");

        spicelib::REPMI(&save.LABEL.to_vec(), b"*", I, &mut save.LABEL, ctx);
        spicelib::REPMD(
            &save.LABEL.to_vec(),
            b"*",
            save.TBUFF[I],
            14,
            &mut save.LABEL,
            ctx,
        );

        spicelib::SPKR01(
            save.HANDLE,
            save.DESCR.as_slice(),
            save.TBUFF[I],
            save.T1REC.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKAD(
            &save.LABEL,
            save.T1REC.as_slice(),
            b"=",
            save.DLBUFF.subarray([1, I]),
            DLSIZE,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // Close and delete the SPK file.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
