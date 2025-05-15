//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const UTC1: &[u8] = b"1999 JUL 1";
const REF1: &[u8] = b"J2000";
const BSP14: &[u8] = b"test14.bsp";
const BSP14E: &[u8] = b"test14err.bsp";
const BSP14S: &[u8] = b"test14sub.bsp";
const BSP14B: &[u8] = b"test14big.bsp";
const BIGSTP: f64 = 10.0;
const TIGHT: f64 = 0.00000000001;
const LOOSE: f64 = 0.001;
const SLOPPY: f64 = 0.1;
const BIGN: i32 = 10101;
const BIGID: i32 = -10000;
const BIGCTR: i32 = 5;
const CHBDEG: i32 = 2;
const DSCSIZ: i32 = 5;
const LNSIZE: i32 = 80;
const MSGLEN: i32 = 240;
const NCHBRC: i32 = 4;
const ND: i32 = 2;
const NEPOCH: i32 = (NCHBRC + 1);
const NI: i32 = 6;
const POLDEG: i32 = 3;
const SIDLEN: i32 = 60;
const FRNMLN: i32 = 32;

struct SaveVars {
    SEGID: Vec<u8>,
    SEGID2: Vec<u8>,
    XREF: Vec<u8>,
    BEPLST: ActualArray<f64>,
    CHBR14: StackArray2D<f64, 80>,
    CHBCFB: ActualArray3D<f64>,
    CHBCFF: StackArray2D<f64, 18>,
    DC: StackArray<f64, 2>,
    DESCR: StackArray<f64, 5>,
    DSCEPC: StackArray<f64, 5>,
    ET: f64,
    INTLEN: f64,
    LT: f64,
    MIDPT: f64,
    PACKET: StackArray<f64, 20>,
    RADIUS: f64,
    STATE: StackArray<f64, 6>,
    THETA: f64,
    XSTATE: StackArray<f64, 6>,
    HANDLE: i32,
    I: i32,
    IC: StackArray<i32, 6>,
    N: i32,
    NEWH: i32,
    PAKSIZ: i32,
    XBODY: i32,
    XCENTR: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SEGID = vec![b' '; SIDLEN as usize];
        let mut SEGID2 = vec![b' '; SIDLEN as usize];
        let mut XREF = vec![b' '; FRNMLN as usize];
        let mut BEPLST = ActualArray::<f64>::new(1..=(BIGN + 1));
        let mut CHBR14 = StackArray2D::<f64, 80>::new(1..=(2 + (6 * (CHBDEG + 1))), 1..=NCHBRC);
        let mut CHBCFB = ActualArray3D::<f64>::new(0..=CHBDEG, 1..=6, 1..=BIGN);
        let mut CHBCFF = StackArray2D::<f64, 18>::new(0..=CHBDEG, 1..=6);
        let mut DC = StackArray::<f64, 2>::new(1..=ND);
        let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
        let mut DSCEPC = StackArray::<f64, 5>::new(1..=NEPOCH);
        let mut ET: f64 = 0.0;
        let mut INTLEN: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut MIDPT: f64 = 0.0;
        let mut PACKET = StackArray::<f64, 20>::new(1..=(2 + (6 * (CHBDEG + 1))));
        let mut RADIUS: f64 = 0.0;
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut THETA: f64 = 0.0;
        let mut XSTATE = StackArray::<f64, 6>::new(1..=6);
        let mut HANDLE: i32 = 0;
        let mut I: i32 = 0;
        let mut IC = StackArray::<i32, 6>::new(1..=NI);
        let mut N: i32 = 0;
        let mut NEWH: i32 = 0;
        let mut PAKSIZ: i32 = 0;
        let mut XBODY: i32 = 0;
        let mut XCENTR: i32 = 0;
        let mut FOUND: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(100.0),
                Val::D(200.0),
                Val::D(300.0),
                Val::D(400.0),
                Val::D(500.0),
            ]
            .into_iter();
            DSCEPC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(150.0),
                Val::D(50.0),
                Val::D(1.0101),
                Val::D(1.0102),
                Val::D(1.0103),
                Val::D(1.0201),
                Val::D(1.0202),
                Val::D(1.0203),
                Val::D(1.0301),
                Val::D(1.0302),
                Val::D(1.0303),
                Val::D(1.0401),
                Val::D(1.0402),
                Val::D(1.0403),
                Val::D(1.0501),
                Val::D(1.0502),
                Val::D(1.0503),
                Val::D(1.0601),
                Val::D(1.0602),
                Val::D(1.0603),
            ]
            .into_iter();
            for I in intrinsics::range(1, (2 + (6 * (CHBDEG + 1))), 1) {
                CHBR14[[I, 1]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(250.0),
                Val::D(50.0),
                Val::D(2.0101),
                Val::D(2.0102),
                Val::D(2.0103),
                Val::D(2.0201),
                Val::D(2.0202),
                Val::D(2.0203),
                Val::D(2.0301),
                Val::D(2.0302),
                Val::D(2.0303),
                Val::D(2.0401),
                Val::D(2.0402),
                Val::D(2.0403),
                Val::D(2.0501),
                Val::D(2.0502),
                Val::D(2.0503),
                Val::D(2.0601),
                Val::D(2.0602),
                Val::D(2.0603),
            ]
            .into_iter();
            for I in intrinsics::range(1, (2 + (6 * (CHBDEG + 1))), 1) {
                CHBR14[[I, 2]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(350.0),
                Val::D(50.0),
                Val::D(3.0101),
                Val::D(3.0102),
                Val::D(3.0103),
                Val::D(3.0201),
                Val::D(3.0202),
                Val::D(3.0203),
                Val::D(3.0301),
                Val::D(3.0302),
                Val::D(3.0303),
                Val::D(3.0401),
                Val::D(3.0402),
                Val::D(3.0403),
                Val::D(3.0501),
                Val::D(3.0502),
                Val::D(3.0503),
                Val::D(3.0601),
                Val::D(3.0602),
                Val::D(3.0603),
            ]
            .into_iter();
            for I in intrinsics::range(1, (2 + (6 * (CHBDEG + 1))), 1) {
                CHBR14[[I, 3]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(450.0),
                Val::D(50.0),
                Val::D(4.0101),
                Val::D(4.0102),
                Val::D(4.0103),
                Val::D(4.0201),
                Val::D(4.0202),
                Val::D(4.0203),
                Val::D(4.0301),
                Val::D(4.0302),
                Val::D(4.0303),
                Val::D(4.0401),
                Val::D(4.0402),
                Val::D(4.0403),
                Val::D(4.0501),
                Val::D(4.0502),
                Val::D(4.0503),
                Val::D(4.0601),
                Val::D(4.0602),
                Val::D(4.0603),
            ]
            .into_iter();
            for I in intrinsics::range(1, (2 + (6 * (CHBDEG + 1))), 1) {
                CHBR14[[I, 4]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            SEGID,
            SEGID2,
            XREF,
            BEPLST,
            CHBR14,
            CHBCFB,
            CHBCFF,
            DC,
            DESCR,
            DSCEPC,
            ET,
            INTLEN,
            LT,
            MIDPT,
            PACKET,
            RADIUS,
            STATE,
            THETA,
            XSTATE,
            HANDLE,
            I,
            IC,
            N,
            NEWH,
            PAKSIZ,
            XBODY,
            XCENTR,
            FOUND,
        }
    }
}

fn T(N: i32, THETA: f64) -> f64 {
    f64::cos(((N as f64) * f64::acos(intrinsics::DMIN1(&[1.0, intrinsics::DMAX1(&[-1.0, THETA])]))))
}

//$Procedure F_SPK14 ( SPK type 14 tests )
pub fn F_SPK14(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
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
    // Epochs associated with coefficient sets.
    //

    //
    // Chebyshev coefficients for testing SPK type 14 routines:
    //

    //
    // Statement functions
    //
    //
    // T(n,theta) represents the Chebyshev polynomial
    //
    //   T ( theta )
    //    n
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_SPK14", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup", ctx)?;

    //
    // Create and load a leapseconds kernel.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test SPK14B:  start out with error handling.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad coefficient set count.", ctx)?;

    if spicelib::EXISTS(BSP14E, ctx)? {
        spicelib::DELFIL(BSP14E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SPKOPN(
        BSP14E,
        b"Type 14 SPK internal file name",
        4,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 301;
    save.XCENTR = 3;
    fstr::assign(&mut save.XREF, b"J2000");
    fstr::assign(&mut save.SEGID, b"SPK Type 14 test segment");

    spicelib::SPK14B(
        save.HANDLE,
        &save.SEGID,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[1],
        save.DSCEPC[(NCHBRC + 1)],
        -1,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDARGUMENT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid reference frame.", ctx)?;

    spicelib::SPK14B(
        save.HANDLE,
        &save.SEGID,
        save.XBODY,
        save.XCENTR,
        b"SPUD",
        save.DSCEPC[1],
        save.DSCEPC[(NCHBRC + 1)],
        NCHBRC,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDREFFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Descriptor times out of order.", ctx)?;

    spicelib::SPK14B(
        save.HANDLE,
        &save.SEGID,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[(NCHBRC + 1)],
        save.DSCEPC[1],
        NCHBRC,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADDESCRTIMES)", OK, ctx)?;

    //
    // Close this file.  Note that the file contains no segments,
    // so SPKCLS won't close it.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPK14a, SPK14b, SPK14c: write small segment.", ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");

    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 14 test segment");

    //
    // Open a new SPK file.
    //
    if spicelib::EXISTS(BSP14, ctx)? {
        spicelib::DELFIL(BSP14, ctx)?;
    }

    spicelib::SPKOPN(
        BSP14,
        b"Type 14 SPK internal file name",
        4,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a type 14 segment.
    //
    //
    // Begin the segment.
    //
    spicelib::SPK14B(
        save.HANDLE,
        &save.SEGID,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.DSCEPC[1],
        save.DSCEPC[(NCHBRC + 1)],
        CHBDEG,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add data.
    //
    spicelib::SPK14A(
        save.HANDLE,
        NCHBRC,
        save.CHBR14.as_slice(),
        save.DSCEPC.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // End the segment.
    //
    spicelib::SPK14E(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the SPK file.
    //
    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPKR014, SPKE014: read small segment.", ctx)?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(BSP14, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up states for each epoch in our list.  Compare.
    //
    save.INTLEN = (save.DSCEPC[2] - save.DSCEPC[1]);

    {
        let m1__: i32 = 1;
        let m2__: i32 = NCHBRC;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Generate look-up epoch ET.
            //
            save.RADIUS = (0.5 * save.INTLEN);
            save.MIDPT = (save.DSCEPC[save.I] + save.RADIUS);

            save.ET = (save.MIDPT + (0.5 * save.RADIUS));

            spicelib::SPKGEO(
                save.XBODY,
                save.ET,
                &save.XREF,
                save.XCENTR,
                save.STATE.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Capture the Ith coefficient packet in an array that is
            // more easily indexed.
            //
            save.PAKSIZ = ((CHBDEG + 1) * 6);

            spicelib::MOVED(
                save.CHBR14.subarray([3, save.I]),
                save.PAKSIZ,
                save.CHBCFF.as_slice_mut(),
            );

            //
            // Evaluate the position "manually."
            //
            save.THETA = ((save.ET - save.MIDPT) / save.RADIUS);

            for J in 1..=6 {
                save.XSTATE[J] = 0.0;

                for K in 0..=CHBDEG {
                    save.XSTATE[J] = (save.XSTATE[J] + (save.CHBCFF[[K, J]] * T(K, save.THETA)));
                }
            }

            testutil::CHCKAD(
                b"type 14 position",
                save.STATE.as_slice(),
                b"~",
                save.XSTATE.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"type 14 velocity",
                save.STATE.subarray(4),
                b"~",
                save.XSTATE.subarray(4),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Unload the SPK file.
    //
    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPKS14: write new file having a segment created by subsetting small segment from BSP14.", ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");

    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 14 test subset segment");

    //
    // Open a new SPK file.
    //
    if spicelib::EXISTS(BSP14S, ctx)? {
        spicelib::DELFIL(BSP14S, ctx)?;
    }

    spicelib::SPKOPN(
        BSP14S,
        b"Type 14 SPK internal file name",
        0,
        &mut save.NEWH,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Open SPK14 and extract segment descriptor and ID of first segment.
    //
    spicelib::DAFOPR(BSP14, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFGN(&mut save.SEGID2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a type 14 segment in new file.  Shorten the time
    // coverage by knocking off the coverage contributed by
    // the first and last packets of the source segment.
    //
    spicelib::SPKSUB(
        save.HANDLE,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.DSCEPC[2],
        save.DSCEPC[(NEPOCH - 1)],
        save.NEWH,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the new SPK file.
    //
    spicelib::SPKCLS(save.NEWH, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the old SPK file.
    //
    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Test SPKS14: check descriptor bounds on subsetted file.",
        ctx,
    )?;

    //
    // Open SPK14S and extract segment descriptor and ID of first
    // segment.
    //
    spicelib::DAFOPR(BSP14S, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
    // Check the time bounds.
    //
    save.I = (NEPOCH - 1);

    testutil::CHCKSD(
        b"Segment start",
        save.DC[1],
        b"=",
        save.DSCEPC[2],
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"Segment end",
        save.DC[2],
        b"=",
        save.DSCEPC[save.I],
        0.0,
        OK,
        ctx,
    )?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPKS14: read states from subsetted file.", ctx)?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(BSP14S, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 3;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    save.INTLEN = (save.DSCEPC[2] - save.DSCEPC[1]);

    //
    // Look up states for each epoch in our list.  Compare.
    //
    {
        let m1__: i32 = 2;
        let m2__: i32 = (NCHBRC - 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Generate look-up epoch ET.
            //
            save.RADIUS = (0.5 * save.INTLEN);
            save.MIDPT = (save.DSCEPC[save.I] + save.RADIUS);

            save.ET = (save.MIDPT + (0.5 * save.RADIUS));

            spicelib::SPKGEO(
                save.XBODY,
                save.ET,
                &save.XREF,
                save.XCENTR,
                save.STATE.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Capture the Ith coefficient packet in an array that is
            // more easily indexed.
            //
            save.PAKSIZ = ((CHBDEG + 1) * 6);

            spicelib::MOVED(
                save.CHBR14.subarray([3, save.I]),
                save.PAKSIZ,
                save.CHBCFF.as_slice_mut(),
            );

            //
            // Evaluate the position "manually."
            //
            save.THETA = ((save.ET - save.MIDPT) / save.RADIUS);

            for J in 1..=6 {
                save.XSTATE[J] = 0.0;

                for K in 0..=CHBDEG {
                    save.XSTATE[J] = (save.XSTATE[J] + (save.CHBCFF[[K, J]] * T(K, save.THETA)));
                }
            }

            testutil::CHCKAD(
                b"type 14 position",
                save.STATE.as_slice(),
                b"~",
                save.XSTATE.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"type 14 velocity",
                save.STATE.subarray(4),
                b"~",
                save.XSTATE.subarray(4),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Unload the SPK file.
    //
    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SPKR14/SPKE14 test:  create a large segment with multiple directories.",
        ctx,
    )?;

    //
    // Create the coefficient and epoch values we'll use. We're going to
    // follow a pattern similar to that used for the smaller segments
    // created so far: each coefficient will have the value
    //
    //    I + J*10**-4 + K*10**-8
    //
    // where I is the coefficient set index, J is the component (X,Y, or
    // Z) index, and K-1 is the associated degree.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = BIGN;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for J in 1..=6 {
                for K in 0..=CHBDEG {
                    save.CHBCFB[[K, J, save.I]] =
                        (((save.I as f64) + ((J as f64) * 0.0001)) + ((K as f64) * 0.00000001));
                }
            }

            save.I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = (BIGN + 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Initialize the Ith epoch.
            //
            save.BEPLST[save.I] = (100 * save.I) as f64;

            save.I += m3__;
        }
    }

    save.XBODY = 14;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");

    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"SPK type 14 big test segment");

    //
    // Open a new SPK file.
    //
    if spicelib::EXISTS(BSP14B, ctx)? {
        spicelib::DELFIL(BSP14B, ctx)?;
    }

    spicelib::SPKOPN(
        BSP14B,
        b"Type 14 SPK internal file name",
        0,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // Begin the segment.
    //
    spicelib::SPK14B(
        save.HANDLE,
        &save.SEGID,
        save.XBODY,
        save.XCENTR,
        &save.XREF,
        save.BEPLST[1],
        save.BEPLST[(BIGN + 1)],
        CHBDEG,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add data.
    //
    save.INTLEN = (save.BEPLST[2] - save.BEPLST[1]);

    {
        let m1__: i32 = 1;
        let m2__: i32 = BIGN;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.RADIUS = (0.5 * save.INTLEN);
            save.MIDPT = (save.BEPLST[save.I] + save.RADIUS);

            save.PACKET[1] = save.MIDPT;
            save.PACKET[2] = save.RADIUS;

            spicelib::MOVED(
                save.CHBCFB.subarray([0, 1, save.I]),
                (6 * (CHBDEG + 1)),
                save.PACKET.subarray_mut(3),
            );

            spicelib::SPK14A(
                save.HANDLE,
                1,
                save.PACKET.as_slice(),
                save.BEPLST.subarray(save.I),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // End the segment.
    //
    spicelib::SPK14E(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the SPK file.
    //
    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test SPKR14, SPKE14: read states from large file.", ctx)?;

    //
    // Load the SPK file.
    //
    spicelib::SPKLEF(BSP14B, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XBODY = 14;
    save.XCENTR = 10;
    fstr::assign(&mut save.XREF, b"J2000");
    save.INTLEN = (save.BEPLST[2] - save.BEPLST[1]);

    //
    // Look up states for each epoch in our list.  Compare.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = BIGN;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Generate look-up epoch ET.
            //
            save.RADIUS = (0.5 * save.INTLEN);
            save.MIDPT = (save.BEPLST[save.I] + save.RADIUS);

            save.ET = (save.MIDPT + (0.5 * save.RADIUS));

            spicelib::SPKGEO(
                save.XBODY,
                save.ET,
                &save.XREF,
                save.XCENTR,
                save.STATE.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Capture the Ith coefficient packet in an array that is
            // more easily indexed.
            //
            save.PAKSIZ = ((CHBDEG + 1) * 6);

            spicelib::MOVED(
                save.CHBCFB.subarray([0, 1, save.I]),
                save.PAKSIZ,
                save.CHBCFF.as_slice_mut(),
            );

            //
            // Evaluate the position "manually."
            //
            save.THETA = ((save.ET - save.MIDPT) / save.RADIUS);

            for J in 1..=6 {
                save.XSTATE[J] = 0.0;

                for K in 0..=CHBDEG {
                    save.XSTATE[J] = (save.XSTATE[J] + (save.CHBCFF[[K, J]] * T(K, save.THETA)));
                }
            }

            testutil::CHCKAD(
                b"type 14 position",
                save.STATE.as_slice(),
                b"~",
                save.XSTATE.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"type 14 velocity",
                save.STATE.subarray(4),
                b"~",
                save.XSTATE.subarray(4),
                3,
                TIGHT,
                OK,
                ctx,
            )?;
            save.I += m3__;
        }
    }
    //
    // Unload the SPK file.
    //
    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Deleting SPK files at clean-up time.", ctx)?;

    //
    // Clean up the SPK files.
    //
    spicelib::DELFIL(BSP14, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(BSP14E, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(BSP14S, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(BSP14B, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
