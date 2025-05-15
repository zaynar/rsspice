//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const STSIZE: i32 = 5000;
const BTSIZE: i32 = 20;
const DSCSIZ: i32 = 5;
const ND: i32 = 2;
const NI: i32 = 6;
const SIDLEN: i32 = 40;
const LNSIZE: i32 = 80;
const NBODY: i32 = 4;
const NPCK: i32 = 10;
const MAXSEG: i32 = (NBODY * STSIZE);
const NSEG1: i32 = 1;
const NSEG2: i32 = (STSIZE / 2);
const NSEG3: i32 = (STSIZE / 2);
const NSEG4: i32 = STSIZE;
const NSEG5: i32 = (STSIZE + 10);
const NSEG6: i32 = BTSIZE;
const NSEG7: i32 = 23;
const NSEG8: i32 = NSEG7;
const NSEG9: i32 = (NBODY * STSIZE);
const NSEG10: i32 = (NBODY * (STSIZE - 3));
const FILSIZ: i32 = 255;
const SCALE: i32 = 10000;
const SMSLEN: i32 = 25;
const TIMLEN: i32 = 50;

struct SaveVars {
    SEGID: Vec<u8>,
    SMSG: Vec<u8>,
    PCKCPY: ActualCharArray,
    PCKS: ActualCharArray,
    XSEGID: ActualCharArray,
    DESCR: StackArray<f64, 5>,
    T: f64,
    TBEGS: ActualArray<f64>,
    TENDS: ActualArray<f64>,
    XDESCR: ActualArray2D<f64>,
    BODY: i32,
    CPYHAN: ActualArray<i32>,
    HANDLE: i32,
    HNDLES: StackArray<i32, 10>,
    I: i32,
    IDS: ActualArray<i32>,
    J: i32,
    NSEG: StackArray<i32, 10>,
    SEGNO: i32,
    PCKNO: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SEGID = vec![b' '; SIDLEN as usize];
        let mut SMSG = vec![b' '; SMSLEN as usize];
        let mut PCKCPY = ActualCharArray::new(FILSIZ, 1..=FTSIZE);
        let mut PCKS = ActualCharArray::new(FILSIZ, 1..=NPCK);
        let mut XSEGID = ActualCharArray::new(SIDLEN, 1..=MAXSEG);
        let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
        let mut T: f64 = 0.0;
        let mut TBEGS = ActualArray::<f64>::new(1..=MAXSEG);
        let mut TENDS = ActualArray::<f64>::new(1..=MAXSEG);
        let mut XDESCR = ActualArray2D::<f64>::new(1..=DSCSIZ, 1..=MAXSEG);
        let mut BODY: i32 = 0;
        let mut CPYHAN = ActualArray::<i32>::new(1..=FTSIZE);
        let mut HANDLE: i32 = 0;
        let mut HNDLES = StackArray::<i32, 10>::new(1..=NPCK);
        let mut I: i32 = 0;
        let mut IDS = ActualArray::<i32>::new(1..=MAXSEG);
        let mut J: i32 = 0;
        let mut NSEG = StackArray::<i32, 10>::new(1..=NPCK);
        let mut SEGNO: i32 = 0;
        let mut PCKNO: i32 = 0;
        let mut FOUND: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"sfs1.bpc"),
                Val::C(b"sfs2.bpc"),
                Val::C(b"sfs3.bpc"),
                Val::C(b"sfs4.bpc"),
                Val::C(b"sfs5.bpc"),
                Val::C(b"sfs6.bpc"),
                Val::C(b"sfs7.bpc"),
                Val::C(b"sfs8.bpc"),
                Val::C(b"sfs9.bpc"),
                Val::C(b"sfs10.bpc"),
            ]
            .into_iter();
            PCKS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(NSEG1),
                Val::I(NSEG2),
                Val::I(NSEG3),
                Val::I(NSEG4),
                Val::I(NSEG5),
                Val::I(NSEG6),
                Val::I(NSEG7),
                Val::I(NSEG8),
                Val::I(NSEG9),
                Val::I(NSEG10),
            ]
            .into_iter();
            NSEG.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            SEGID,
            SMSG,
            PCKCPY,
            PCKS,
            XSEGID,
            DESCR,
            T,
            TBEGS,
            TENDS,
            XDESCR,
            BODY,
            CPYHAN,
            HANDLE,
            HNDLES,
            I,
            IDS,
            J,
            NSEG,
            SEGNO,
            PCKNO,
            FOUND,
        }
    }
}

//$Procedure F_PCKBSR ( Family of tests for PCKBSR )
pub fn F_PCKBSR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // The number of segments in the respective PCK files:
    //

    //
    // Other parameters:
    //

    //
    // Local variables
    //

    //
    // Saved all.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_PCKBSR", ctx)?;

    testutil::TCASE(b"The first PCK file contains 1 segment for body 1. Make sure we can look up data from this file.", ctx)?;

    //
    // Create the first PCK file.
    //
    save.BODY = 1;
    save.TBEGS[1] = ((SCALE as f64) + 0.0);
    save.TENDS[1] = ((SCALE as f64) + 1.0);
    save.PCKNO = 1;

    fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[1].to_vec(),
        b"#",
        &save.PCKS[1],
        &mut save.XSEGID[1],
    );
    spicelib::REPMI(&save.XSEGID[1].to_vec(), b"#", 1, &mut save.XSEGID[1], ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CRDAF(
        b"PCK",
        &save.PCKS[1],
        save.NSEG[1],
        &[save.BODY],
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKLOF(&save.PCKS[1], &mut save.HNDLES[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[1] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the segment should be found.  Make sure we get
    // back the right handle and segment identifier.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[1], 0, OK, ctx)?;
    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[1], OK, ctx)?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRDESC(
        b"PCK",
        1,
        save.BODY,
        save.TBEGS[1],
        save.TENDS[1],
        save.XDESCR.subarray_mut([1, 1]),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 1]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Try to look up data for a different body in PCK 1.  Also look up data for body 1 for a time which is not covered.", ctx)?;

    spicelib::PCKSFS(
        2,
        (save.TBEGS[1] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the segment should not be found.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::PCKSFS(
        1,
        (save.TBEGS[1] + 10 as f64),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the segment should not be found.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    testutil::TCASE(b"Create a second PCK containing data for body 1 and body 2.  Load this PCK, then look up a state covered by the new file.", ctx)?;

    save.BODY = 1;
    save.PCKNO = 2;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.PCKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (save.I <= (save.NSEG[save.PCKNO] / 2)) {
                save.IDS[save.I] = 2;
            } else {
                save.IDS[save.I] = 1;
            }

            save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
            save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.PCKS[save.PCKNO],
                &mut save.XSEGID[save.I],
            );
            spicelib::REPMI(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                save.I,
                &mut save.XSEGID[save.I],
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    T_CRDAF(
        b"PCK",
        &save.PCKS[save.PCKNO],
        save.NSEG[save.PCKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKLOF(&save.PCKS[save.PCKNO], &mut save.HNDLES[save.PCKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = save.NSEG[save.PCKNO];

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[save.SEGNO] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the segment should be found.  Make sure we get
    // back the right handle and segment identifier.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(
        b"SEGID",
        &save.SEGID,
        b"=",
        &save.XSEGID[save.SEGNO],
        OK,
        ctx,
    )?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRDESC(
        b"PCK",
        save.SEGNO,
        save.BODY,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Look up data for body 2.  This should cause an OLD FILES search.",
        ctx,
    )?;

    save.BODY = 2;
    save.PCKNO = 2;
    save.SEGNO = 1;

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[1] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the segment should be found.  Make sure we get
    // back the right handle and segment identifier.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(
        b"SEGID",
        &save.SEGID,
        b"=",
        &save.XSEGID[save.SEGNO],
        OK,
        ctx,
    )?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRDESC(
        b"PCK",
        save.SEGNO,
        save.BODY,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Create a third PCK containing data for body 3. Load this PCK, then look up a state covered by the new file. This should cause the segment list for body 1 to get dumped.", ctx)?;

    save.BODY = 3;
    save.PCKNO = 3;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.PCKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = save.BODY;

            save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
            save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.PCKS[save.PCKNO],
                &mut save.XSEGID[save.I],
            );
            spicelib::REPMI(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                save.I,
                &mut save.XSEGID[save.I],
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    T_CRDAF(
        b"PCK",
        &save.PCKS[save.PCKNO],
        save.NSEG[save.PCKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKLOF(&save.PCKS[save.PCKNO], &mut save.HNDLES[save.PCKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = save.NSEG[save.PCKNO];

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[save.SEGNO] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the segment should be found.  Make sure we get
    // back the right handle and segment identifier.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(
        b"SEGID",
        &save.SEGID,
        b"=",
        &save.XSEGID[save.SEGNO],
        OK,
        ctx,
    )?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRDESC(
        b"PCK",
        save.SEGNO,
        save.BODY,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Create another PCK for body 1 and load it. The segment count in this file is such that all other body lists must be dumped to make room. Then make a request that is satisfied by PCK 1. The segment in PCK 1 cannot be added to the segment table.", ctx)?;

    save.BODY = 1;
    save.PCKNO = 4;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.PCKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = save.BODY;

            save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
            save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.PCKS[save.PCKNO],
                &mut save.XSEGID[save.I],
            );
            spicelib::REPMI(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                save.I,
                &mut save.XSEGID[save.I],
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    T_CRDAF(
        b"PCK",
        &save.PCKS[save.PCKNO],
        save.NSEG[save.PCKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKLOF(&save.PCKS[save.PCKNO], &mut save.HNDLES[save.PCKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.PCKNO = 1;
    save.SEGNO = 1;

    save.TBEGS[save.SEGNO] = (((save.PCKNO * SCALE) + save.SEGNO) - 1) as f64;
    save.TENDS[save.SEGNO] = ((save.PCKNO * SCALE) + save.SEGNO) as f64;

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[save.SEGNO] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the segment should be found.  Make sure we get
    // back the right handle and segment identifier.
    //

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.PCKS[save.PCKNO],
        &mut save.XSEGID[save.SEGNO],
    );
    spicelib::REPMI(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        1,
        &mut save.XSEGID[save.SEGNO],
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(
        b"SEGID",
        &save.SEGID,
        b"=",
        &save.XSEGID[save.SEGNO],
        OK,
        ctx,
    )?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRDESC(
        b"PCK",
        save.SEGNO,
        save.BODY,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Start a segment list for body 1 by making a request that is satisfied by PCK 1.  Then build a file (PCK 5) with too many segments for body 1 to be buffered.  Make a request that is satisfied by PCK 5. This tests the logic for searching the subset of a segment list that must be dumped due to lack of room.", ctx)?;

    //
    // Set up by making a request that will be satisfied by the segment
    // in PCK 1.  This builds up the segment list for body 1.
    //
    save.BODY = 1;
    save.TBEGS[1] = ((SCALE as f64) + 0.0);
    save.TENDS[1] = ((SCALE as f64) + 1.0);
    save.PCKNO = 1;

    fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[1].to_vec(),
        b"#",
        &save.PCKS[1],
        &mut save.XSEGID[1],
    );
    spicelib::REPMI(&save.XSEGID[1].to_vec(), b"#", 1, &mut save.XSEGID[1], ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[1] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Go ahead and make the new file.
    //
    save.BODY = 1;
    save.PCKNO = 5;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.PCKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = save.BODY;

            if ((save.I == 10) || (save.I == (STSIZE + 1))) {
                //
                // We want the lower bound of the re-use interval to
                // match the right endpoint of the segment's coverage
                // interval.
                //
                save.TBEGS[(save.I - 1)] = ((save.PCKNO * SCALE) + save.I) as f64;
                save.TENDS[(save.I - 1)] = (save.TBEGS[(save.I - 1)] + 1.0);

                save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

                save.TBEGS[(save.I + 1)] = save.TBEGS[save.I];
                save.TENDS[(save.I + 1)] = save.TENDS[save.I];

                save.TBEGS[(save.I + 2)] = (save.TENDS[save.I] + 1 as f64);
                save.TENDS[(save.I + 2)] = (save.TBEGS[(save.I + 2)] + 1 as f64);
            } else if (save.I == (STSIZE + 6)) {
                //
                // Create a singleton segment.
                //
                save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = save.TBEGS[save.I];
            } else if (save.I == (STSIZE + 7)) {
                //
                // Create an invisible segment.
                //
                save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = (save.TBEGS[save.I] - 1 as f64);
            } else if (((save.I < 9) || ((save.I > 12) && (save.I < STSIZE)))
                || (save.I > (STSIZE + 3)))
            {
                save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);
            }

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.PCKS[save.PCKNO],
                &mut save.XSEGID[save.I],
            );
            spicelib::REPMI(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                save.I,
                &mut save.XSEGID[save.I],
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    T_CRDAF(
        b"PCK",
        &save.PCKS[save.PCKNO],
        save.NSEG[save.PCKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKLOF(&save.PCKS[save.PCKNO], &mut save.HNDLES[save.PCKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = 1;

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[save.SEGNO] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the segment should be found.  Make sure we get
    // back the right handle and segment identifier.
    //

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.PCKS[save.PCKNO],
        &mut save.XSEGID[save.SEGNO],
    );
    spicelib::REPMI(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        1,
        &mut save.XSEGID[save.SEGNO],
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(
        b"SEGID",
        &save.SEGID,
        b"=",
        &save.XSEGID[save.SEGNO],
        OK,
        ctx,
    )?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRDESC(
        b"PCK",
        save.SEGNO,
        save.BODY,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Create an PCK containing data for BTSIZE new bodies. Look up data for each.",
        ctx,
    )?;

    //
    // Unload all PCKs.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NPCK;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKUOF(save.HNDLES[save.I], ctx)?;
            save.I += m3__;
        }
    }

    save.PCKNO = 6;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.PCKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = (BTSIZE + save.I);

            save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
            save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.PCKS[save.PCKNO],
                &mut save.XSEGID[save.I],
            );
            spicelib::REPMI(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                save.I,
                &mut save.XSEGID[save.I],
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    T_CRDAF(
        b"PCK",
        &save.PCKS[save.PCKNO],
        save.NSEG[save.PCKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKLOF(&save.PCKS[save.PCKNO], &mut save.HNDLES[save.PCKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.PCKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.BODY = save.IDS[save.I];
            save.SEGNO = save.I;

            spicelib::PCKSFS(
                save.BODY,
                (save.TBEGS[save.SEGNO] + 0.5),
                &mut save.HANDLE,
                save.DESCR.as_slice_mut(),
                &mut save.SEGID,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // In this case, the segment should be found.  Make sure we get
            // back the right handle and segment identifier.
            //
            fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
            spicelib::REPMC(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                &save.PCKS[save.PCKNO],
                &mut save.XSEGID[save.SEGNO],
            );
            spicelib::REPMI(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                save.SEGNO,
                &mut save.XSEGID[save.SEGNO],
                ctx,
            );

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
            testutil::CHCKSI(
                b"HANDLE",
                save.HANDLE,
                b"=",
                save.HNDLES[save.PCKNO],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSC(
                b"SEGID",
                &save.SEGID,
                b"=",
                &save.XSEGID[save.SEGNO],
                OK,
                ctx,
            )?;

            //
            // Check the descriptor as well.  However, don't check the
            // segment addresses.
            //
            T_CRDESC(
                b"PCK",
                save.SEGNO,
                save.BODY,
                save.TBEGS[save.SEGNO],
                save.TENDS[save.SEGNO],
                save.XDESCR.subarray_mut([1, save.SEGNO]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, save.SEGNO]),
                (DSCSIZ - 2),
                0.0,
                OK,
                ctx,
            )?;
            save.I += m3__;
        }
    }

    testutil::TCASE(b"The body table should be full now; the segment table should have room.  Cause a body list to be dumped to make room in the body table.", ctx)?;

    //
    // Create a list for body 1 more expensive than those for the
    // bodies in PCK 6.  Body 1's list will be placed at the head of
    // the body table.
    //
    save.BODY = 1;
    save.PCKNO = 2;
    save.SEGNO = save.NSEG[save.PCKNO];
    save.I = save.SEGNO;
    save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
    save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

    fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[1].to_vec(),
        b"#",
        &save.PCKS[save.PCKNO],
        &mut save.XSEGID[1],
    );
    spicelib::REPMI(
        &save.XSEGID[1].to_vec(),
        b"#",
        save.SEGNO,
        &mut save.XSEGID[1],
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKLOF(&save.PCKS[2], &mut save.HNDLES[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[save.I] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the segment should be found.  Make sure we get
    // back the right handle and segment identifier.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[1], OK, ctx)?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRDESC(
        b"PCK",
        save.SEGNO,
        save.BODY,
        save.TBEGS[save.I],
        save.TENDS[save.I],
        save.XDESCR.subarray_mut([1, 1]),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 1]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    //
    // Now do a look up for body 2.  This will require dumping lists
    // from PCK 6.
    //
    save.BODY = 2;
    save.PCKNO = 2;
    save.SEGNO = 1;
    save.I = save.SEGNO;
    save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
    save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.PCKS[save.PCKNO],
        &mut save.XSEGID[save.SEGNO],
    );
    spicelib::REPMI(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        save.SEGNO,
        &mut save.XSEGID[save.SEGNO],
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[save.I] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the segment should be found.  Make sure we get
    // back the right handle and segment identifier.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(
        b"SEGID",
        &save.SEGID,
        b"=",
        &save.XSEGID[save.SEGNO],
        OK,
        ctx,
    )?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRDESC(
        b"PCK",
        1,
        save.BODY,
        save.TBEGS[1],
        save.TENDS[1],
        save.XDESCR.subarray_mut([1, 1]),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 1]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Look up data from a representative subset of the segments in PCK 5.",
        ctx,
    )?;

    save.PCKNO = 5;

    spicelib::PCKLOF(&save.PCKS[save.PCKNO], &mut save.HNDLES[save.PCKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.PCKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = save.BODY;

            if ((save.I == 10) || (save.I == (STSIZE + 1))) {
                //
                // We want the lower bound of the re-use interval to
                // match the right endpoint of the segment's coverage
                // interval.
                //
                save.TBEGS[(save.I - 1)] = ((save.PCKNO * SCALE) + save.I) as f64;
                save.TENDS[(save.I - 1)] = (save.TBEGS[(save.I - 1)] + 1.0);

                save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

                save.TBEGS[(save.I + 1)] = save.TBEGS[save.I];
                save.TENDS[(save.I + 1)] = save.TENDS[save.I];

                save.TBEGS[(save.I + 2)] = (save.TENDS[save.I] + 1 as f64);
                save.TENDS[(save.I + 2)] = (save.TBEGS[(save.I + 2)] + 1 as f64);
            } else if (save.I == (STSIZE + 6)) {
                //
                // Create a singleton segment.
                //
                save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = save.TBEGS[save.I];
            } else if (save.I == (STSIZE + 7)) {
                //
                // Create an invisible segment.
                //
                save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = (save.TBEGS[save.I] - 1 as f64);
            } else if (((save.I < 10) || ((save.I > 12) && (save.I < STSIZE)))
                || (save.I > (STSIZE + 3)))
            {
                save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);
            }

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.PCKS[save.PCKNO],
                &mut save.XSEGID[save.I],
            );
            spicelib::REPMI(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                save.I,
                &mut save.XSEGID[save.I],
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    save.I = 1;

    while (save.I <= save.NSEG[save.PCKNO]) {
        save.BODY = 1;
        save.SEGNO = save.I;

        spicelib::PCKSFS(
            save.BODY,
            (save.TBEGS[save.SEGNO] + 0.5),
            &mut save.HANDLE,
            save.DESCR.as_slice_mut(),
            &mut save.SEGID,
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // In this case, the segment should be found.  Make sure we get
        // back the right handle and segment identifier.
        //
        fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
        spicelib::REPMC(
            &save.XSEGID[save.SEGNO].to_vec(),
            b"#",
            &save.PCKS[save.PCKNO],
            &mut save.XSEGID[save.SEGNO],
        );
        spicelib::REPMI(
            &save.XSEGID[save.SEGNO].to_vec(),
            b"#",
            save.SEGNO,
            &mut save.XSEGID[save.SEGNO],
            ctx,
        );

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
        testutil::CHCKSI(
            b"HANDLE",
            save.HANDLE,
            b"=",
            save.HNDLES[save.PCKNO],
            0,
            OK,
            ctx,
        )?;
        testutil::CHCKSC(
            b"SEGID",
            &save.SEGID,
            b"=",
            &save.XSEGID[save.SEGNO],
            OK,
            ctx,
        )?;

        //
        // Check the descriptor as well.  However, don't check the
        // segment addresses.
        //
        T_CRDESC(
            b"PCK",
            save.SEGNO,
            save.BODY,
            save.TBEGS[save.SEGNO],
            save.TENDS[save.SEGNO],
            save.XDESCR.subarray_mut([1, save.SEGNO]),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        T_CHDS(
            b"DESCR",
            save.DESCR.as_slice(),
            b"=",
            save.XDESCR.subarray([1, save.SEGNO]),
            (DSCSIZ - 2),
            0.0,
            OK,
            ctx,
        )?;

        //
        // Skip some tests that are unlikely to reveal bugs, as well as
        // those which would give anomalous results due to the structure
        // of PCK 6.
        //
        if (save.I == 3) {
            save.I = ((STSIZE / 2) - 2);
        } else if (save.I == ((STSIZE / 2) + 3)) {
            save.I = (STSIZE - 2);
        } else if (save.I == STSIZE) {
            save.I = (STSIZE + 5);
        } else if (save.I == (STSIZE + 5)) {
            save.I = (STSIZE + 8);
        } else {
            save.I = (save.I + 1);
        }
    }

    //
    // Try a search w/o buffering case where no segment is found.
    //
    testutil::TCASE(b"Search w/o buffering, no segment should be found.", ctx)?;
    save.PCKNO = 5;
    save.BODY = 1;
    save.T = ((2 as f64) * save.TENDS[save.NSEG[save.PCKNO]]);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // Return on entry in RETURN mode, if the error status is set.
    //
    testutil::TCASE(
        b"Make sure PCKSFS returns on entry when RETURN()is .TRUE.",
        ctx,
    )?;

    fstr::assign(&mut save.SMSG, b"Return on entry");

    spicelib::SIGERR(&save.SMSG, ctx)?;

    spicelib::PCKSFS(
        1,
        0.0,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    //
    // Depending on whether we're calling a version of PCKBSR that does
    // coverage checking, the error status may be reset.
    //
    if spicelib::RETURN(ctx) {
        testutil::CHCKXC(true, &save.SMSG, OK, ctx)?;
    } else {
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Unload the PCK files.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NPCK;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKUOF(save.HNDLES[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    //
    // Make sure an error is NOT signaled if no PCKs are loaded.
    //
    testutil::TCASE(
        b"Make sure an error is NOT signaled if no PCKs are loaded.",
        ctx,
    )?;
    spicelib::PCKSFS(
        1,
        0.0,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load PCK1 and look up a state from it to create a cheap list.
    // Make the cheap list the second list by looking up data from
    // it after looking up data for body BTSIZE+1.
    //
    testutil::TCASE(
        b"Test removal of cheap list when adding a new body; cheap list is 2nd.",
        ctx,
    )?;

    spicelib::PCKLOF(&save.PCKS[1], &mut save.HNDLES[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now load the PCK containing 100 bodies.  Look up data for
    // each one.  The last one will cause the list for body 1 to
    // be dumped.
    //
    save.PCKNO = 6;
    spicelib::PCKLOF(&save.PCKS[save.PCKNO], &mut save.HNDLES[save.PCKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.PCKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = (BTSIZE + save.I);

            save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
            save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.PCKS[save.PCKNO],
                &mut save.XSEGID[save.I],
            );
            spicelib::REPMI(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                save.I,
                &mut save.XSEGID[save.I],
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.PCKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.BODY = save.IDS[save.I];
            save.SEGNO = save.I;

            spicelib::PCKSFS(
                save.BODY,
                (save.TBEGS[save.SEGNO] + 0.5),
                &mut save.HANDLE,
                save.DESCR.as_slice_mut(),
                &mut save.SEGID,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // In this case, the segment should be found.  Make sure we get
            // back the right handle and segment identifier.
            //
            fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
            spicelib::REPMC(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                &save.PCKS[save.PCKNO],
                &mut save.XSEGID[save.SEGNO],
            );
            spicelib::REPMI(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                save.SEGNO,
                &mut save.XSEGID[save.SEGNO],
                ctx,
            );

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
            testutil::CHCKSI(
                b"HANDLE",
                save.HANDLE,
                b"=",
                save.HNDLES[save.PCKNO],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSC(
                b"SEGID",
                &save.SEGID,
                b"=",
                &save.XSEGID[save.SEGNO],
                OK,
                ctx,
            )?;

            //
            // Check the descriptor as well.  However, don't check the
            // segment addresses.
            //
            T_CRDESC(
                b"PCK",
                save.SEGNO,
                save.BODY,
                save.TBEGS[save.SEGNO],
                save.TENDS[save.SEGNO],
                save.XDESCR.subarray_mut([1, save.SEGNO]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, save.SEGNO]),
                (DSCSIZ - 2),
                0.0,
                OK,
                ctx,
            )?;

            if (save.I == 1) {
                //
                // Create a cheap list for body 1.
                //
                spicelib::PCKLOF(&save.PCKS[save.PCKNO], &mut save.HNDLES[save.PCKNO], ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                save.TBEGS[1] = SCALE as f64;
                spicelib::PCKSFS(
                    1,
                    (save.TBEGS[1] + 0.5),
                    &mut save.HANDLE,
                    save.DESCR.as_slice_mut(),
                    &mut save.SEGID,
                    &mut save.FOUND,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }

            save.I += m3__;
        }
    }

    testutil::TCASE(
        b"Test ability to make room by deleting a body table entry with an empty list.",
        ctx,
    )?;

    //
    // Create an example of the list in question by forcing a search
    // without buffering on body 1, where the highest priority file
    // contains too many segments to buffer.  However, we want this
    // list to have a high expense, so load an PCK with many segments
    // for this body and search it first.
    //
    save.PCKNO = 5;
    spicelib::PCKLOF(&save.PCKS[save.PCKNO], &mut save.HNDLES[save.PCKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BODY = 1;
    save.T = (((((SCALE * save.PCKNO) + (STSIZE + 1)) - 1) as f64) + 0.5);

    spicelib::PCKSFS(
        1,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Now look up data for the first NSEG-1 bodies in PCK 6.  This
    // should fill up the body table.
    //
    save.PCKNO = 6;

    {
        let m1__: i32 = 1;
        let m2__: i32 = (save.NSEG[save.PCKNO] - 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = (BTSIZE + save.I);

            save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
            save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

            save.BODY = save.IDS[save.I];
            save.SEGNO = save.I;

            spicelib::PCKSFS(
                save.BODY,
                (save.TBEGS[save.SEGNO] + 0.5),
                &mut save.HANDLE,
                save.DESCR.as_slice_mut(),
                &mut save.SEGID,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // In this case, the segment should be found.  Make sure we get
            // back the right handle and segment identifier.
            //
            fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
            spicelib::REPMC(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                &save.PCKS[save.PCKNO],
                &mut save.XSEGID[save.SEGNO],
            );
            spicelib::REPMI(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                save.SEGNO,
                &mut save.XSEGID[save.SEGNO],
                ctx,
            );

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
            testutil::CHCKSI(
                b"HANDLE",
                save.HANDLE,
                b"=",
                save.HNDLES[save.PCKNO],
                0,
                OK,
                ctx,
            )?;
            testutil::CHCKSC(
                b"SEGID",
                &save.SEGID,
                b"=",
                &save.XSEGID[save.SEGNO],
                OK,
                ctx,
            )?;

            //
            // Check the descriptor as well.  However, don't check the
            // segment addresses.
            //
            T_CRDESC(
                b"PCK",
                save.SEGNO,
                save.BODY,
                save.TBEGS[save.SEGNO],
                save.TENDS[save.SEGNO],
                save.XDESCR.subarray_mut([1, save.SEGNO]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, save.SEGNO]),
                (DSCSIZ - 2),
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Try some cases where the re-use interval matches the selected
    // segment's coverage interval.
    //
    testutil::TCASE(b"Search w/o buffering case, selected segment is in dumped list, coverage interval matches re-use interval, request time is in center of re-use interval.", ctx)?;

    //
    // Set up the case by unloading the currently loaded PCKs.  Load
    // PCK 1 and look up a state from it.  Then load PCK 5.
    //

    //
    // Unload the PCK files.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (NPCK - 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKUOF(save.HNDLES[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    //
    // Load PCK 1 and look up a state from this file.
    //
    spicelib::PCKLOF(&save.PCKS[1], &mut save.HNDLES[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BODY = 1;
    save.TBEGS[1] = ((SCALE as f64) + 0.0);
    save.TENDS[1] = ((SCALE as f64) + 1.0);
    save.PCKNO = 1;

    fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[1].to_vec(),
        b"#",
        &save.PCKS[1],
        &mut save.XSEGID[1],
    );
    spicelib::REPMI(&save.XSEGID[1].to_vec(), b"#", 1, &mut save.XSEGID[1], ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[1] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now load PCK 5.  Look up a state from segment 9, where the
    // request time is to the right of a segment whose right endpoint
    // is at the left endpoint of the re-use interval.
    //
    spicelib::PCKLOF(&save.PCKS[5], &mut save.HNDLES[5], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.PCKNO = 5;
    save.BODY = 1;
    save.SEGNO = 9;

    save.TBEGS[save.SEGNO] = (((save.PCKNO * SCALE) + save.SEGNO) + 1) as f64;
    save.TENDS[save.SEGNO] = (save.TBEGS[save.SEGNO] + 1 as f64);

    save.T = (save.TBEGS[save.SEGNO] + 0.25);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, segment 9 should match.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.SEGNO = 9;

    fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[1].to_vec(),
        b"#",
        &save.PCKS[5],
        &mut save.XSEGID[1],
    );
    spicelib::REPMI(
        &save.XSEGID[1].to_vec(),
        b"#",
        save.SEGNO,
        &mut save.XSEGID[1],
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[1], OK, ctx)?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRDESC(
        b"PCK",
        save.SEGNO,
        save.BODY,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    //
    // Create a situation where the segment list for body 1 contributed
    // by PCK 5 gets dumped, and where the request is satisfied by
    // a segment in PCK 1.
    //
    testutil::TCASE(
        b"Dump segment list from PCK 5; find segment for body 1 in PCK 1.",
        ctx,
    )?;

    spicelib::PCKLOF(&save.PCKS[1], &mut save.HNDLES[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKLOF(&save.PCKS[5], &mut save.HNDLES[5], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BODY = 1;
    save.TBEGS[1] = SCALE as f64;
    save.TENDS[1] = ((SCALE as f64) + 1.0);
    save.T = (0.5 * (save.TBEGS[1] + save.TENDS[1]));

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Check handle, segment descriptor and ID.
    //
    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[1], 0, OK, ctx)?;

    T_CRDESC(
        b"PCK",
        1,
        save.BODY,
        save.TBEGS[1],
        save.TENDS[1],
        save.XDESCR.subarray_mut([1, 1]),
        ctx,
    )?;
    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 1]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[1].to_vec(),
        b"#",
        &save.PCKS[1],
        &mut save.XSEGID[1],
    );
    spicelib::REPMI(&save.XSEGID[1].to_vec(), b"#", 1, &mut save.XSEGID[1], ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[1], OK, ctx)?;

    testutil::TCASE(b"Dump segment list from PCK 5.  While searching list for segment for body 1, make lower bound of re-use interval match lower bound of segment descriptor.", ctx)?;

    //
    // Make PCK 1 higher priority than PCK 5.
    //
    spicelib::PCKLOF(&save.PCKS[1], &mut save.HNDLES[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Place request time in the "hole" between segments STSIZE+1 and
    // STSIZE+3.
    //
    save.I = (STSIZE + 1);

    save.TBEGS[(save.I - 1)] = ((save.PCKNO * SCALE) + save.I) as f64;
    save.TENDS[(save.I - 1)] = (save.TBEGS[(save.I - 1)] + 1.0);

    save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
    save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

    save.TBEGS[(save.I + 1)] = save.TBEGS[save.I];
    save.TENDS[(save.I + 1)] = save.TENDS[save.I];

    save.TBEGS[(save.I + 2)] = (save.TENDS[save.I] + 1 as f64);
    save.TENDS[(save.I + 2)] = (save.TBEGS[(save.I + 2)] + 1 as f64);

    save.T = (save.TBEGS[(save.I - 1)] + 0.5);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, segment STSIZE should match.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.SEGNO = STSIZE;

    fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[1].to_vec(),
        b"#",
        &save.PCKS[5],
        &mut save.XSEGID[1],
    );
    spicelib::REPMI(
        &save.XSEGID[1].to_vec(),
        b"#",
        save.SEGNO,
        &mut save.XSEGID[1],
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[1], OK, ctx)?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    save.I = (save.SEGNO + 1);
    save.TBEGS[(save.I - 1)] = ((save.PCKNO * SCALE) + save.I) as f64;
    save.TENDS[(save.I - 1)] = (save.TBEGS[(save.I - 1)] + 1.0);

    T_CRDESC(
        b"PCK",
        save.SEGNO,
        save.BODY,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check correct handling of re-use intervals.  Create a new
    // PCK file that contains coverage that exemplifies the various
    // masking possibilities that may occur.
    //
    testutil::TCASE(b"Check re-use for a 1-body segment list.", ctx)?;

    save.PCKNO = 7;

    //
    // Segment 1:
    //
    save.BODY = 1;
    save.IDS[1] = save.BODY;

    save.TBEGS[1] = (save.PCKNO * SCALE) as f64;
    save.TENDS[1] = (save.TBEGS[1] + 1.0);

    //
    // Segments 2-3:
    //
    save.BODY = 2;
    save.IDS[2] = save.BODY;
    save.IDS[3] = save.BODY;

    save.TBEGS[3] = (save.PCKNO * SCALE) as f64;
    save.TENDS[3] = (save.TBEGS[3] + 1.0);

    save.TBEGS[2] = (save.TENDS[3] + 1.0);
    save.TENDS[2] = (save.TBEGS[2] + 1.0);

    //
    // Segments 4-6:
    //
    save.BODY = 3;
    save.IDS[4] = save.BODY;
    save.IDS[5] = save.BODY;
    save.IDS[6] = save.BODY;

    save.TBEGS[6] = (save.PCKNO * SCALE) as f64;
    save.TENDS[6] = (save.TBEGS[6] + 3.0);

    save.TBEGS[5] = (save.TENDS[6] - 1.0);
    save.TENDS[5] = (save.TBEGS[5] + 3.0);

    save.TBEGS[4] = (save.TBEGS[5] + 1.0);
    save.TENDS[4] = (save.TENDS[5] - 1.0);

    //
    // Segments 7-9:
    //
    save.BODY = 4;
    save.IDS[7] = save.BODY;
    save.IDS[8] = save.BODY;
    save.IDS[9] = save.BODY;

    save.TBEGS[9] = (save.PCKNO * SCALE) as f64;
    save.TENDS[9] = (save.TBEGS[9] + 3.0);

    save.TBEGS[8] = save.TBEGS[9];
    save.TENDS[8] = save.TENDS[9];

    save.TBEGS[7] = (save.TBEGS[9] - 2.0);
    save.TENDS[7] = (save.TBEGS[9] + 3.0);

    //
    // Segments 10-12:
    //
    save.BODY = 5;
    save.IDS[10] = save.BODY;
    save.IDS[11] = save.BODY;
    save.IDS[12] = save.BODY;

    save.TBEGS[12] = (save.PCKNO * SCALE) as f64;
    save.TENDS[12] = (save.TBEGS[12] + 3.0);

    save.TBEGS[11] = (save.TBEGS[12] - 2.0);
    save.TENDS[11] = (save.TBEGS[11] + 3.0);

    save.TBEGS[10] = (save.TBEGS[11] - 2.0);
    save.TENDS[10] = (save.TENDS[12] + 1.0);

    //
    // Segments 13-14:
    //
    save.BODY = 6;
    save.IDS[13] = save.BODY;
    save.IDS[14] = save.BODY;

    //
    // Singleton segment:
    //
    save.TBEGS[13] = (save.PCKNO * SCALE) as f64;
    save.TENDS[13] = save.TBEGS[13];

    //
    // Invisible segment:
    //
    save.TBEGS[14] = (save.TENDS[13] + 3.0);
    save.TENDS[14] = (save.TBEGS[14] - 1.0);

    //
    // Three more segments for body 4:
    //
    save.IDS[15] = 4;
    save.IDS[16] = 4;
    save.IDS[17] = 4;

    save.TBEGS[15] = (((save.PCKNO * SCALE) as f64) + 10.0);
    save.TENDS[15] = (save.TBEGS[15] + 3.0);

    save.TBEGS[16] = (save.TBEGS[15] + 1.0);
    save.TENDS[16] = (save.TENDS[15] - 1.0);

    save.TBEGS[17] = save.TBEGS[16];
    save.TENDS[17] = save.TENDS[16];

    //
    // Three more segments for body 5:
    //
    save.BODY = 5;
    save.IDS[18] = save.BODY;
    save.IDS[19] = save.BODY;
    save.IDS[20] = save.BODY;

    save.TBEGS[20] = (((save.PCKNO * SCALE) as f64) + 10.0);
    save.TENDS[20] = (save.TBEGS[20] + 3.0);

    save.TBEGS[19] = (save.TBEGS[20] - 2.0);
    save.TENDS[19] = (save.TBEGS[19] + 3.0);

    save.TBEGS[18] = (save.TBEGS[19] - 2.0);
    save.TENDS[18] = (save.TENDS[20] + 1.0);

    //
    // Create a segment sequence for body 6 with the following topology:
    //
    //
    //          +++++++           segment 21
    //                +++++++             22
    //    +++++++                         23
    //
    //
    save.BODY = 6;
    save.IDS[21] = save.BODY;
    save.IDS[22] = save.BODY;
    save.IDS[23] = save.BODY;

    save.TBEGS[21] = (((save.PCKNO * SCALE) as f64) + 10.0);
    save.TENDS[21] = (save.TBEGS[21] + 3.0);

    save.TBEGS[22] = save.TENDS[21];
    save.TENDS[22] = (save.TBEGS[21] + 3.0);

    save.TBEGS[23] = (save.TBEGS[21] - 3.0);
    save.TENDS[23] = save.TBEGS[21];

    //
    // Create the eighth PCK, which is just a copy of the 7th, except
    // for descriptors and segment IDs.
    //
    save.PCKNO = 8;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.PCKNO];
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CRDESC(
                b"PCK",
                save.SEGNO,
                save.IDS[save.SEGNO],
                save.TBEGS[save.SEGNO],
                save.TENDS[save.SEGNO],
                save.XDESCR.subarray_mut([1, save.SEGNO]),
                ctx,
            )?;

            fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
            spicelib::REPMC(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                &save.PCKS[save.PCKNO],
                &mut save.XSEGID[save.SEGNO],
            );
            spicelib::REPMI(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                save.SEGNO,
                &mut save.XSEGID[save.SEGNO],
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.SEGNO += m3__;
        }
    }

    T_CRDAF(
        b"PCK",
        &save.PCKS[save.PCKNO],
        save.NSEG[save.PCKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    //
    // Create the segment descriptors and segment identifiers for
    // this PCK file.
    //
    save.PCKNO = 7;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.PCKNO];
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CRDESC(
                b"PCK",
                save.SEGNO,
                save.IDS[save.SEGNO],
                save.TBEGS[save.SEGNO],
                save.TENDS[save.SEGNO],
                save.XDESCR.subarray_mut([1, save.SEGNO]),
                ctx,
            )?;

            fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
            spicelib::REPMC(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                &save.PCKS[save.PCKNO],
                &mut save.XSEGID[save.SEGNO],
            );
            spicelib::REPMI(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                save.SEGNO,
                &mut save.XSEGID[save.SEGNO],
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.SEGNO += m3__;
        }
    }

    //
    // Unload the other PCK files.  Create and load the PCK file.
    //
    //
    // Unload the PCK files.  Again.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (save.PCKNO - 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKUOF(save.HNDLES[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    T_CRDAF(
        b"PCK",
        &save.PCKS[save.PCKNO],
        save.NSEG[save.PCKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKLOF(&save.PCKS[save.PCKNO], &mut save.HNDLES[save.PCKNO], ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Time for tests.
    //
    //
    // Make sure we can re-use data from the first segment for body 1.
    //
    save.PCKNO = 7;
    save.BODY = save.IDS[1];
    save.T = (0.5 * (save.TBEGS[1] + save.TENDS[1]));

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKSFS(
                save.BODY,
                save.T,
                &mut save.HANDLE,
                save.DESCR.as_slice_mut(),
                &mut save.SEGID,
                &mut save.FOUND,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            //
            // Check handle, segment descriptor and ID.
            //
            testutil::CHCKSI(
                b"HANDLE",
                save.HANDLE,
                b"=",
                save.HNDLES[save.PCKNO],
                0,
                OK,
                ctx,
            )?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 1]),
                (DSCSIZ - 2),
                0.0,
                OK,
                ctx,
            )?;

            testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[1], OK, ctx)?;

            save.I += m3__;
        }
    }

    save.T = (save.TBEGS[1] - 1.0);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    save.T = (save.TENDS[1] + 1.0);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    save.T = save.TBEGS[1];

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.T = save.TENDS[1];

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Check out behavior for coverage consisting of two non-overlapping
    // segments.  The coverage topology is as follows:
    //
    //
    //                  ++++++++++    segment 2
    //    +++++++++++                         3
    //
    //
    //
    testutil::TCASE(
        b"Coverage is union of two disjoint intervals. Test re-use of each.",
        ctx,
    )?;

    save.BODY = save.IDS[2];
    save.T = (0.5 * (save.TBEGS[2] + save.TENDS[2]));

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKSFS(
                save.BODY,
                save.T,
                &mut save.HANDLE,
                save.DESCR.as_slice_mut(),
                &mut save.SEGID,
                &mut save.FOUND,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            //
            // Check handle, segment descriptor and ID.
            //
            testutil::CHCKSI(
                b"HANDLE",
                save.HANDLE,
                b"=",
                save.HNDLES[save.PCKNO],
                0,
                OK,
                ctx,
            )?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 2]),
                (DSCSIZ - 2),
                0.0,
                OK,
                ctx,
            )?;

            testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[2], OK, ctx)?;

            save.I += m3__;
        }
    }

    save.T = (0.5 * (save.TBEGS[3] + save.TENDS[3]));

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKSFS(
                save.BODY,
                save.T,
                &mut save.HANDLE,
                save.DESCR.as_slice_mut(),
                &mut save.SEGID,
                &mut save.FOUND,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            //
            // Check handle, segment descriptor and ID.
            //
            testutil::CHCKSI(
                b"HANDLE",
                save.HANDLE,
                b"=",
                save.HNDLES[save.PCKNO],
                0,
                OK,
                ctx,
            )?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 3]),
                (DSCSIZ - 2),
                0.0,
                OK,
                ctx,
            )?;

            testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[3], OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Hit the endpoints of the left interval.
    //
    save.T = save.TBEGS[3];

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Check handle, segment descriptor and ID.
    //
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 3]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[3], OK, ctx)?;

    save.T = save.TENDS[3];

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Check handle, segment descriptor and ID.
    //
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 3]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[3], OK, ctx)?;

    //
    // Segments 4-6:
    //
    //
    // Check out behavior for coverage consisting of three segments
    // whose coverage is as shown:
    //
    //
    //             +++++++          segment 4
    //          +++++++++++++               5
    //    +++++++++++                       6
    //
    //
    testutil::TCASE(b"Segments 4-6:  three-segment overlapping case #1.", ctx)?;

    save.BODY = save.IDS[5];
    save.T = (save.TENDS[6] + 0.25);

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKSFS(
                save.BODY,
                save.T,
                &mut save.HANDLE,
                save.DESCR.as_slice_mut(),
                &mut save.SEGID,
                &mut save.FOUND,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            //
            // Check handle, segment descriptor and ID.
            //
            testutil::CHCKSI(
                b"HANDLE",
                save.HANDLE,
                b"=",
                save.HNDLES[save.PCKNO],
                0,
                OK,
                ctx,
            )?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 5]),
                (DSCSIZ - 2),
                0.0,
                OK,
                ctx,
            )?;

            testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[5], OK, ctx)?;

            save.I += m3__;
        }
    }

    save.BODY = save.IDS[4];
    save.T = (save.TBEGS[6] + 0.25);

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKSFS(
                save.BODY,
                save.T,
                &mut save.HANDLE,
                save.DESCR.as_slice_mut(),
                &mut save.SEGID,
                &mut save.FOUND,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            //
            // Check handle, segment descriptor and ID.
            //
            testutil::CHCKSI(
                b"HANDLE",
                save.HANDLE,
                b"=",
                save.HNDLES[save.PCKNO],
                0,
                OK,
                ctx,
            )?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 6]),
                (DSCSIZ - 2),
                0.0,
                OK,
                ctx,
            )?;

            testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[6], OK, ctx)?;

            save.I += m3__;
        }
    }

    save.T = (save.TBEGS[5] + 0.25);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.T = (save.TBEGS[6] - 0.25);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // Segments 7-9:
    //
    //
    // Check out behavior for coverage consisting of three segments
    // whose coverage is as shown:
    //
    //    +++++++++++           segment 7
    //         +++++++++++              8
    //         +++++++++++              9
    //
    testutil::TCASE(b"Segments 7-9:  three-segment overlapping case #2.", ctx)?;

    //
    // Get the right side of the re-use interval to coincide with
    // the left endpoint of a descriptor, where ET lies to the left
    // of the segment, in the CHECK LIST state:
    //
    save.BODY = save.IDS[7];
    save.T = (save.TBEGS[7] + 0.25);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 7]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[7], OK, ctx)?;

    //
    // Check out behavior for coverage consisting of three segments
    // whose coverage is as shown:
    //
    //
    //   ++++++++++++++++++        segment 10
    //       +++++++                       11
    //           ++++++++                  12
    //
    //
    testutil::TCASE(b"Three-segment overlapping case #2.", ctx)?;

    save.BODY = save.IDS[10];
    save.T = (save.TENDS[12] + 0.25);

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKSFS(
                save.BODY,
                save.T,
                &mut save.HANDLE,
                save.DESCR.as_slice_mut(),
                &mut save.SEGID,
                &mut save.FOUND,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            //
            // Check handle, segment descriptor and ID.
            //
            testutil::CHCKSI(
                b"HANDLE",
                save.HANDLE,
                b"=",
                save.HNDLES[save.PCKNO],
                0,
                OK,
                ctx,
            )?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 10]),
                (DSCSIZ - 2),
                0.0,
                OK,
                ctx,
            )?;

            testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[10], OK, ctx)?;

            save.I += m3__;
        }
    }

    save.T = (save.TENDS[10] + 1.0);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    save.T = (save.TBEGS[10] + 0.25);

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKSFS(
                save.BODY,
                save.T,
                &mut save.HANDLE,
                save.DESCR.as_slice_mut(),
                &mut save.SEGID,
                &mut save.FOUND,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            //
            // Check handle, segment descriptor and ID.
            //
            testutil::CHCKSI(
                b"HANDLE",
                save.HANDLE,
                b"=",
                save.HNDLES[save.PCKNO],
                0,
                OK,
                ctx,
            )?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 10]),
                (DSCSIZ - 2),
                0.0,
                OK,
                ctx,
            )?;

            testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[10], OK, ctx)?;

            save.I += m3__;
        }
    }

    save.T = (save.TBEGS[11] - 0.25);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 10]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[10], OK, ctx)?;

    //
    // Check out behavior for coverage consisting of three segments
    // whose coverage is as shown:
    //
    //
    //   ++++++++++++++++++        segment 15
    //        +++++++                      16
    //        +++++++                      17
    //
    //
    testutil::TCASE(
        b"ET > segment uppper bound.  Lower bound of re-use interval = upper bound of segment.",
        ctx,
    )?;

    save.BODY = save.IDS[15];
    save.T = (save.TENDS[17] + 0.5);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Check handle, segment descriptor and ID.
    //
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 15]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[15], OK, ctx)?;

    //
    // Check out behavior for coverage consisting of three segments
    // whose coverage is as shown:
    //
    //
    //          +++++++           segment 21
    //                +++++++             22
    //    +++++++                         23
    //
    //

    testutil::TCASE(
        b"ET is in segment.  Lower bound of re-use interval = lower bound of segment.",
        ctx,
    )?;

    save.BODY = 6;
    save.IDS[21] = save.BODY;
    save.IDS[22] = save.BODY;
    save.IDS[23] = save.BODY;

    save.TBEGS[21] = (((save.PCKNO * SCALE) as f64) + 10.0);
    save.TENDS[21] = (save.TBEGS[21] + 3.0);

    save.TBEGS[22] = save.TENDS[21];
    save.TENDS[22] = (save.TBEGS[21] + 3.0);

    save.TBEGS[23] = (save.TBEGS[21] - 3.0);
    save.TENDS[23] = save.TBEGS[21];

    save.BODY = save.IDS[21];
    save.T = (save.TBEGS[21] + 0.5);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Check handle, segment descriptor and ID.
    //
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 21]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[21], OK, ctx)?;

    //
    // Check out behavior for coverage consisting singleton and
    // invisible segments.
    //
    //
    testutil::TCASE(b"Look up data from a singleton segment.", ctx)?;

    save.T = save.TBEGS[13];
    save.BODY = save.IDS[13];

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 13]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[13], OK, ctx)?;

    //
    // Exercise the logic for handling singleton and invisible
    // segments during a NEW BODY search.
    //
    testutil::TCASE(
        b"Look up data from a singleton segment, this time in a NEW SEGMENTS search.",
        ctx,
    )?;

    save.PCKNO = 8;

    spicelib::PCKLOF(&save.PCKS[save.PCKNO], &mut save.HNDLES[save.PCKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = 13;

    T_CRDESC(
        b"PCK",
        save.SEGNO,
        save.IDS[save.SEGNO],
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        ctx,
    )?;

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.PCKS[save.PCKNO],
        &mut save.XSEGID[save.SEGNO],
    );
    spicelib::REPMI(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        save.SEGNO,
        &mut save.XSEGID[save.SEGNO],
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.T = save.TBEGS[13];
    save.BODY = save.IDS[13];

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 13]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[13], OK, ctx)?;

    testutil::TCASE(b"Prepare for search w/o buffering tests: create an PCK with STSIZE segments for bodies 1-NBODY.", ctx)?;

    //
    // Create an PCK file with STSIZE segments for bodies 1-NBODY.
    //
    save.PCKNO = 9;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NBODY;
        let m3__: i32 = 1;
        save.BODY = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = STSIZE;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.J = (((save.BODY - 1) * STSIZE) + save.I);
                    save.IDS[save.J] = save.BODY;

                    save.TBEGS[save.J] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
                    save.TENDS[save.J] = (save.TBEGS[save.J] + 1 as f64);

                    fstr::assign(save.XSEGID.get_mut(save.J), b"File: # Segno: #  Body:  #");

                    spicelib::REPMC(
                        &save.XSEGID[save.J].to_vec(),
                        b"#",
                        &save.PCKS[save.PCKNO],
                        &mut save.XSEGID[save.J],
                    );
                    spicelib::REPMI(
                        &save.XSEGID[save.J].to_vec(),
                        b"#",
                        save.J,
                        &mut save.XSEGID[save.J],
                        ctx,
                    );
                    spicelib::REPMI(
                        &save.XSEGID[save.J].to_vec(),
                        b"#",
                        save.BODY,
                        &mut save.XSEGID[save.J],
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.I += m3__;
                }
            }

            save.BODY += m3__;
        }
    }

    T_CRDAF(
        b"PCK",
        &save.PCKS[save.PCKNO],
        save.NSEG[save.PCKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(b"Prepare for search w/o buffering tests: create an PCK with STSIZE segments for bodies 1-NBODY.", ctx)?;

    //
    // Create an PCK file with STSIZE segments for bodies 1-NBODY.
    //

    save.PCKNO = 10;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NBODY;
        let m3__: i32 = 1;
        save.BODY = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = (STSIZE - 3);
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.J = (((save.BODY - 1) * (STSIZE - 3)) + save.I);
                    save.IDS[save.J] = save.BODY;

                    save.TBEGS[save.J] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
                    save.TENDS[save.J] = (save.TBEGS[save.J] + 1 as f64);

                    fstr::assign(save.XSEGID.get_mut(save.J), b"File: # Segno: #  Body:  #");

                    spicelib::REPMC(
                        &save.XSEGID[save.J].to_vec(),
                        b"#",
                        &save.PCKS[save.PCKNO],
                        &mut save.XSEGID[save.J],
                    );
                    spicelib::REPMI(
                        &save.XSEGID[save.J].to_vec(),
                        b"#",
                        save.J,
                        &mut save.XSEGID[save.J],
                        ctx,
                    );
                    spicelib::REPMI(
                        &save.XSEGID[save.J].to_vec(),
                        b"#",
                        save.BODY,
                        &mut save.XSEGID[save.J],
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.I += m3__;
                }
            }

            save.BODY += m3__;
        }
    }

    T_CRDAF(
        b"PCK",
        &save.PCKS[save.PCKNO],
        save.NSEG[save.PCKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(b"Search w/o buffering, ET < segment begin, re-use interval right endpoint < segment begin.", ctx)?;

    //
    // Unload the PCK files.  Again.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NPCK;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKUOF(save.HNDLES[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    //
    // Load PCKs 7 and 9.
    //
    spicelib::PCKLOF(&save.PCKS[7], &mut save.HNDLES[7], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKLOF(&save.PCKS[9], &mut save.HNDLES[9], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The request time should precede the coverage of segment 3 in
    // PCK 7.
    //
    save.BODY = 2;
    save.T = (((7 * SCALE) as f64) - 1.0);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    testutil::TCASE(
        b"Search w/o buffering, ET within segment, re-use interval, left endpoint > segment begin.",
        ctx,
    )?;

    //
    // The request time should precede the coverage of segment 3 in
    // PCK 7.
    //
    save.BODY = 3;
    save.SEGNO = 5;
    save.PCKNO = 7;

    save.TBEGS[6] = (save.PCKNO * SCALE) as f64;
    save.TENDS[6] = (save.TBEGS[6] + 3.0);

    save.TBEGS[5] = (save.TENDS[6] - 1.0);
    save.TENDS[5] = (save.TBEGS[5] + 3.0);

    save.T = (((save.PCKNO * SCALE) as f64) + 4.0);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.PCKS[save.PCKNO],
        &mut save.XSEGID[save.SEGNO],
    );
    spicelib::REPMI(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        save.SEGNO,
        &mut save.XSEGID[save.SEGNO],
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(
        b"SEGID",
        &save.SEGID,
        b"=",
        &save.XSEGID[save.SEGNO],
        OK,
        ctx,
    )?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRDESC(
        b"PCK",
        save.SEGNO,
        save.BODY,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Search w/o buffering, ET < segment begin, re-use interval right endpoint = segment begin.", ctx)?;
    save.BODY = 4;
    save.SEGNO = 7;
    save.PCKNO = 7;

    save.IDS[7] = save.BODY;
    save.IDS[8] = save.BODY;
    save.IDS[9] = save.BODY;

    save.TBEGS[9] = (save.PCKNO * SCALE) as f64;
    save.TENDS[9] = (save.TBEGS[9] + 3.0);

    save.TBEGS[8] = save.TBEGS[9];
    save.TENDS[8] = save.TENDS[9];

    save.TBEGS[7] = (save.TBEGS[9] - 2.0);
    save.TENDS[7] = (save.TBEGS[9] + 3.0);

    save.T = (save.TBEGS[8] - 1.0);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.PCKS[save.PCKNO],
        &mut save.XSEGID[save.SEGNO],
    );
    spicelib::REPMI(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        save.SEGNO,
        &mut save.XSEGID[save.SEGNO],
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(
        b"SEGID",
        &save.SEGID,
        b"=",
        &save.XSEGID[save.SEGNO],
        OK,
        ctx,
    )?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRDESC(
        b"PCK",
        save.SEGNO,
        save.BODY,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    //
    // Some cases where a partial list must be dumped:
    //
    testutil::TCASE(b"Dump segment list from PCK 10.  While searching list for segment for body 4, make upper bound of re-use interval < upper bound of segment descriptor.", ctx)?;

    //
    // Unload PCK 9; load PCK 10.
    //
    spicelib::PCKUOF(save.HNDLES[9], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCKLOF(&save.PCKS[10], &mut save.HNDLES[10], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Our request time should be in the interior of segment 15 in
    // PCK 7.
    //
    save.PCKNO = 7;
    save.SEGNO = 15;

    save.IDS[15] = 4;
    save.IDS[16] = 4;
    save.IDS[17] = 4;

    save.TBEGS[15] = (((save.PCKNO * SCALE) as f64) + 10.0);
    save.TENDS[15] = (save.TBEGS[15] + 3.0);

    save.TBEGS[16] = (save.TBEGS[15] + 1.0);
    save.TENDS[16] = (save.TENDS[15] - 1.0);

    save.TBEGS[17] = save.TBEGS[16];
    save.TENDS[17] = save.TBEGS[17];

    save.T = (save.TBEGS[15] + 0.5);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.PCKS[save.PCKNO],
        &mut save.XSEGID[save.SEGNO],
    );
    spicelib::REPMI(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        save.SEGNO,
        &mut save.XSEGID[save.SEGNO],
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(
        b"SEGID",
        &save.SEGID,
        b"=",
        &save.XSEGID[save.SEGNO],
        OK,
        ctx,
    )?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRDESC(
        b"PCK",
        save.SEGNO,
        save.BODY,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Dump segment list from PCK 10.  While searching list for segment for body 4, make lower bound of re-use interval = upper bound of segment descriptor.", ctx)?;

    save.PCKNO = 7;
    save.BODY = 4;
    save.TBEGS[9] = (save.PCKNO * SCALE) as f64;
    save.TENDS[9] = (save.TBEGS[9] + 3.0);
    save.T = (save.TENDS[9] + 0.5);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    testutil::TCASE(b"Dump segment list from PCK 10.  While searching list for segment for body 5, make lower bound of re-use interval > lower bound of segment descriptor.", ctx)?;

    save.PCKNO = 7;
    save.BODY = 5;
    save.IDS[18] = save.BODY;
    save.IDS[19] = save.BODY;
    save.IDS[20] = save.BODY;

    save.TBEGS[20] = (((save.PCKNO * SCALE) as f64) + 10.0);
    save.TENDS[20] = (save.TBEGS[20] + 3.0);

    save.TBEGS[19] = (save.TBEGS[20] - 2.0);
    save.TENDS[19] = (save.TBEGS[19] + 3.0);

    save.TBEGS[18] = (save.TBEGS[19] - 2.0);
    save.TENDS[18] = (save.TENDS[20] + 1.0);

    save.T = (save.TENDS[18] - 0.5);

    spicelib::PCKSFS(
        save.BODY,
        save.T,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;

    save.SEGNO = 18;

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.PCKS[save.PCKNO],
        &mut save.XSEGID[save.SEGNO],
    );
    spicelib::REPMI(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        save.SEGNO,
        &mut save.XSEGID[save.SEGNO],
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(
        b"SEGID",
        &save.SEGID,
        b"=",
        &save.XSEGID[save.SEGNO],
        OK,
        ctx,
    )?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRDESC(
        b"PCK",
        save.SEGNO,
        save.BODY,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 2),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Create a situation where room is needed in the body table, and the second body list has expense greater than the first.", ctx)?;

    //
    // Unload PCKs 7 and 10.
    //
    spicelib::PCKUOF(save.HNDLES[7], ctx)?;
    spicelib::PCKUOF(save.HNDLES[10], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fill up (nearly) the segment table with a cheap list for body 2
    // and an expensive list for body 1.
    //
    save.PCKNO = 7;
    spicelib::PCKLOF(&save.PCKS[save.PCKNO], &mut save.HNDLES[save.PCKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BODY = 2;
    save.IDS[2] = save.BODY;
    save.IDS[3] = save.BODY;

    save.TBEGS[3] = (save.PCKNO * SCALE) as f64;
    save.TENDS[3] = (save.TBEGS[3] + 1.0);

    save.BODY = 2;
    save.SEGNO = 3;

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[save.SEGNO] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the segment should be found.  Make sure we get
    // back the right handle and segment identifier.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;

    spicelib::PCKLOF(&save.PCKS[10], &mut save.HNDLES[10], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BODY = 1;
    save.PCKNO = 10;
    save.SEGNO = 1;
    save.I = 1;
    save.TBEGS[save.I] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
    save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[1] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;

    //
    // Now do a look up for body 3.  This should cause the segment
    // lists for bodies 2 and 1 to get dumped.
    //
    save.BODY = 3;
    save.PCKNO = 10;

    save.I = 1;
    save.J = (((save.BODY - 1) * (STSIZE - 3)) + save.I);

    save.TBEGS[save.J] = (((save.PCKNO * SCALE) + save.I) - 1) as f64;
    save.TENDS[save.J] = (save.TBEGS[save.I] + 1 as f64);

    fstr::assign(save.XSEGID.get_mut(save.J), b"File: # Segno: #  Body:  #");

    spicelib::REPMC(
        &save.XSEGID[save.I].to_vec(),
        b"#",
        &save.PCKS[save.PCKNO],
        &mut save.XSEGID[save.I],
    );
    spicelib::REPMI(
        &save.XSEGID[save.I].to_vec(),
        b"#",
        save.J,
        &mut save.XSEGID[save.I],
        ctx,
    );
    spicelib::REPMI(
        &save.XSEGID[save.I].to_vec(),
        b"#",
        save.BODY,
        &mut save.XSEGID[save.I],
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = save.J;

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[save.J] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HNDLES[save.PCKNO],
        0,
        OK,
        ctx,
    )?;

    //
    // Return on entry in RETURN mode, if the error status is set.
    //
    testutil::TCASE(
        b"Make sure all PCKBSR entry points return on entry when RETURN() is .TRUE.",
        ctx,
    )?;

    //
    // Depending on whether we're calling a version of PCKBSR that does
    // coverage checking, the error status may be reset.

    fstr::assign(&mut save.SMSG, b"Return on entry");

    spicelib::SIGERR(&save.SMSG, ctx)?;

    spicelib::PCKBSR(
        b" ",
        1,
        1,
        0.0,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.FOUND,
        ctx,
    )?;

    if spicelib::RETURN(ctx) {
        testutil::CHCKXC(true, &save.SMSG, OK, ctx)?;
    } else {
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SIGERR(&save.SMSG, ctx)?;

    spicelib::PCKLOF(b" ", &mut save.HANDLE, ctx)?;

    if spicelib::RETURN(ctx) {
        testutil::CHCKXC(true, &save.SMSG, OK, ctx)?;
    } else {
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SIGERR(&save.SMSG, ctx)?;

    spicelib::PCKUOF(save.HANDLE, ctx)?;

    if spicelib::RETURN(ctx) {
        testutil::CHCKXC(true, &save.SMSG, OK, ctx)?;
    } else {
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SIGERR(&save.SMSG, ctx)?;

    spicelib::PCKSFS(
        1,
        0.0,
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;

    if spicelib::RETURN(ctx) {
        testutil::CHCKXC(true, &save.SMSG, OK, ctx)?;
    } else {
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::TCASE(
        b"Make sure an error is signaled if PCKBSR is called directly and RETURN() is .FALSE.",
        ctx,
    )?;

    spicelib::PCKBSR(
        b" ",
        1,
        1,
        0.0,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BOGUSENTRY)", OK, ctx)?;

    testutil::TCASE(b"Try DAFOPR error handling.", ctx)?;

    spicelib::PCKLOF(b"ThisFileDoesNotExist", &mut save.HANDLE, ctx)?;

    if spicelib::RETURN(ctx) {
        testutil::CHCKXC(true, b"SPICE(FILENOTFOUND)", OK, ctx)?;
    } else {
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::TCASE(
        b"Test partial deletion of a segment list when a file is unloaded.",
        ctx,
    )?;

    //
    // Unload the PCK files.  The load files 1 and 2.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NPCK;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKUOF(save.HNDLES[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = 2;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKLOF(&save.PCKS[save.I], &mut save.HNDLES[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    //
    // Do lookups for body 1 that hit both files.
    //
    save.BODY = 1;
    save.TBEGS[1] = ((SCALE as f64) + 0.0);

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[1] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.BODY = 1;
    save.PCKNO = 2;

    save.SEGNO = ((save.NSEG[save.PCKNO] / 2) + 1);

    save.TBEGS[save.SEGNO] = (((save.PCKNO * SCALE) + save.SEGNO) - 1) as f64;

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[save.SEGNO] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Do a lookup for body 2 to create a segment list for that
    // body.
    //
    save.BODY = 2;
    save.PCKNO = 2;

    save.SEGNO = (save.NSEG[save.PCKNO] / 2);

    save.TBEGS[save.SEGNO] = (((save.PCKNO * SCALE) + save.SEGNO) - 1) as f64;

    spicelib::PCKSFS(
        save.BODY,
        (save.TBEGS[save.SEGNO] + 0.5),
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Reload file 1, removing the portion of body 1's segment list
    // that came from file 1, as part of the unload process that
    // precedes re-loading file 1.
    //
    spicelib::PCKLOF(&save.PCKS[1], &mut save.HNDLES[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create FTSIZE copies of PCK 1 and load FTSIZE-1 of them.  We
    // should get a file table overflow error.
    //
    testutil::TCASE(b"File table overflow error.", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = FTSIZE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(save.PCKCPY.get_mut(save.I), b"copy#.bpc");
            spicelib::REPMI(
                &save.PCKCPY[save.I].to_vec(),
                b"#",
                save.I,
                &mut save.PCKCPY[save.I],
                ctx,
            );

            save.BODY = 1;
            save.TBEGS[1] = ((SCALE as f64) + 0.0);
            save.TENDS[1] = ((SCALE as f64) + 1.0);
            save.PCKNO = 1;

            fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
            spicelib::REPMC(
                &save.XSEGID[1].to_vec(),
                b"#",
                &save.PCKCPY[save.I],
                &mut save.XSEGID[1],
            );
            spicelib::REPMI(&save.XSEGID[1].to_vec(), b"#", 1, &mut save.XSEGID[1], ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            T_CRDAF2(
                b"PCK",
                &save.PCKCPY[save.I],
                &save.PCKCPY[save.I],
                save.NSEG[1],
                &[save.BODY],
                save.TBEGS.as_slice(),
                save.TENDS.as_slice(),
                save.XSEGID.as_arg(),
                ctx,
            )?;

            save.I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = (FTSIZE - 2);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKLOF(&save.PCKCPY[save.I], &mut save.CPYHAN[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    spicelib::PCKLOF(
        &save.PCKCPY[(FTSIZE - 1)],
        &mut save.CPYHAN[(FTSIZE - 1)],
        ctx,
    )?;

    //
    // Note:  if FTSIZE >= the file table size in the handle manager,
    // the appropriate check would be
    //
    testutil::CHCKXC(true, b"SPICE(FTFULL)", OK, ctx)?;

    //
    // Loading, unloading, and priority checks:
    //
    testutil::TCASE(b"Load all copies of PCK 1, looking up the same state from each.  Unload the files in reverse order.  Repeat 3 times.", ctx)?;

    //
    // First, make sure all files are unloaded.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NPCK;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKUOF(save.HNDLES[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = (FTSIZE - 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKUOF(save.CPYHAN[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    save.BODY = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = FTSIZE;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.TBEGS[save.J] = ((SCALE as f64) + 0.0);
                    save.TENDS[save.J] = ((SCALE as f64) + 1.0);

                    spicelib::PCKLOF(&save.PCKCPY[save.J], &mut save.CPYHAN[save.J], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(save.XSEGID.get_mut(save.J), b"File: # Segno: #");
                    spicelib::REPMC(
                        &save.XSEGID[save.J].to_vec(),
                        b"#",
                        &save.PCKCPY[save.J],
                        &mut save.XSEGID[save.J],
                    );
                    spicelib::REPMI(
                        &save.XSEGID[save.J].to_vec(),
                        b"#",
                        1,
                        &mut save.XSEGID[save.J],
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::PCKSFS(
                        save.BODY,
                        (save.TBEGS[save.J] + 0.5),
                        &mut save.HANDLE,
                        save.DESCR.as_slice_mut(),
                        &mut save.SEGID,
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // In this case, the segment should be found.  Make sure
                    // we get back the right handle and segment identifier.
                    //
                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
                    testutil::CHCKSI(
                        b"HANDLE",
                        save.HANDLE,
                        b"=",
                        save.CPYHAN[save.J],
                        0,
                        OK,
                        ctx,
                    )?;
                    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[save.J], OK, ctx)?;

                    //
                    // Check the descriptor as well.  However, don't check the
                    // segment addresses.
                    //
                    T_CRDESC(
                        b"PCK",
                        1,
                        save.BODY,
                        save.TBEGS[save.J],
                        save.TENDS[save.J],
                        save.XDESCR.subarray_mut([1, save.J]),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    T_CHDS(
                        b"DESCR",
                        save.DESCR.as_slice(),
                        b"=",
                        save.XDESCR.subarray([1, save.J]),
                        (DSCSIZ - 2),
                        0.0,
                        OK,
                        ctx,
                    )?;

                    save.J += m3__;
                }
            }

            //
            // Now unload files, looking up states as we go.
            //
            {
                let m1__: i32 = (FTSIZE - 1);
                let m2__: i32 = 1;
                let m3__: i32 = -1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    spicelib::PCKUOF(save.CPYHAN[(save.J + 1)], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.TBEGS[save.J] = ((SCALE as f64) + 0.0);
                    save.TENDS[save.J] = ((SCALE as f64) + 1.0);

                    fstr::assign(save.XSEGID.get_mut(save.J), b"File: # Segno: #");
                    spicelib::REPMC(
                        &save.XSEGID[save.J].to_vec(),
                        b"#",
                        &save.PCKCPY[save.J],
                        &mut save.XSEGID[save.J],
                    );
                    spicelib::REPMI(
                        &save.XSEGID[save.J].to_vec(),
                        b"#",
                        1,
                        &mut save.XSEGID[save.J],
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::PCKSFS(
                        save.BODY,
                        (save.TBEGS[save.J] + 0.5),
                        &mut save.HANDLE,
                        save.DESCR.as_slice_mut(),
                        &mut save.SEGID,
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // In this case, the segment should be found.  Make sure
                    // we get back the right handle and segment identifier.
                    //
                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
                    testutil::CHCKSI(
                        b"HANDLE",
                        save.HANDLE,
                        b"=",
                        save.CPYHAN[save.J],
                        0,
                        OK,
                        ctx,
                    )?;
                    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[save.J], OK, ctx)?;

                    //
                    // Check the descriptor as well.  However, don't check the
                    // segment addresses.
                    //
                    T_CRDESC(
                        b"PCK",
                        1,
                        save.BODY,
                        save.TBEGS[save.J],
                        save.TENDS[save.J],
                        save.XDESCR.subarray_mut([1, save.J]),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    T_CHDS(
                        b"DESCR",
                        save.DESCR.as_slice(),
                        b"=",
                        save.XDESCR.subarray([1, save.J]),
                        (DSCSIZ - 2),
                        0.0,
                        OK,
                        ctx,
                    )?;

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // Make sure we don't accumulate DAF links by re-loading a file.
    //
    testutil::TCASE(b"Load the first PCK file 2*FTSIZE times.", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = (FTSIZE * 2);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.TBEGS[1] = ((SCALE as f64) + 0.0);
            save.TENDS[1] = ((SCALE as f64) + 1.0);

            spicelib::PCKLOF(&save.PCKS[1], &mut save.HNDLES[1], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
            spicelib::REPMC(
                &save.XSEGID[1].to_vec(),
                b"#",
                &save.PCKS[1],
                &mut save.XSEGID[1],
            );
            spicelib::REPMI(&save.XSEGID[1].to_vec(), b"#", 1, &mut save.XSEGID[1], ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::PCKSFS(
                save.BODY,
                (save.TBEGS[1] + 0.5),
                &mut save.HANDLE,
                save.DESCR.as_slice_mut(),
                &mut save.SEGID,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // In this case, the segment should be found.  Make sure
            // we get back the right handle and segment identifier.
            //
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
            testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[1], 0, OK, ctx)?;
            testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[1], OK, ctx)?;

            //
            // Check the descriptor as well.  However, don't check the
            // segment addresses.
            //
            T_CRDESC(
                b"PCK",
                1,
                save.BODY,
                save.TBEGS[1],
                save.TENDS[1],
                save.XDESCR.subarray_mut([1, 1]),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 1]),
                (DSCSIZ - 2),
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Last step:  delete all of the PCK files we created.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NPCK;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKUOF(save.HNDLES[save.I], ctx)?;
            spicelib::DELFIL(&save.PCKS[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = FTSIZE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::PCKUOF(save.CPYHAN[save.I], ctx)?;
            spicelib::DELFIL(&save.PCKCPY[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
