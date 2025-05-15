//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TOL0: f64 = 0.0;
const FTSIZE: i32 = 500;
const STSIZE: i32 = 1000;
const ITSIZE: i32 = 100;
const DSCSIZ: i32 = 5;
const ND: i32 = 2;
const NI: i32 = 6;
const SIDLEN: i32 = 40;
const NINST: i32 = 4;
const MAXSEG: i32 = (NINST * STSIZE);
const LNSIZE: i32 = 80;
const NCK: i32 = 15;
const NSEG1: i32 = 1;
const NSEG2: i32 = (STSIZE / 2);
const NSEG3: i32 = (STSIZE / 2);
const NSEG4: i32 = 10;
const NSEG5: i32 = STSIZE;
const NSEG6: i32 = (STSIZE + 10);
const NSEG7: i32 = ITSIZE;
const NSEG8: i32 = 23;
const NSEG9: i32 = NSEG8;
const NSEG10: i32 = (NINST * STSIZE);
const NSEG11: i32 = (NINST * (STSIZE - 3));
const NSEG12: i32 = STSIZE;
const NSEG13: i32 = 10;
const NSEG14: i32 = ((STSIZE / 2) + 10);
const NSEG15: i32 = (STSIZE / 2);
const FILSIZ: i32 = 255;
const SCALE: i32 = 10000;
const SMSLEN: i32 = 25;
const TIMLEN: i32 = 50;

struct SaveVars {
    SEGID: Vec<u8>,
    SMSG: Vec<u8>,
    CKCPY: ActualCharArray,
    CKS: ActualCharArray,
    XSEGID: ActualCharArray,
    DESCR: StackArray<f64, 5>,
    T: f64,
    TBEGS: ActualArray<f64>,
    TENDS: ActualArray<f64>,
    TOL: f64,
    XDESCR: ActualArray2D<f64>,
    CKNO: i32,
    CPYHAN: ActualArray<i32>,
    HANDLE: i32,
    HNDLES: StackArray<i32, 15>,
    I: i32,
    IDS: ActualArray<i32>,
    INST: i32,
    J: i32,
    NSEG: StackArray<i32, 15>,
    SEGNO: i32,
    AVFLAG: bool,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SEGID = vec![b' '; SIDLEN as usize];
        let mut SMSG = vec![b' '; SMSLEN as usize];
        let mut CKCPY = ActualCharArray::new(FILSIZ, 1..=FTSIZE);
        let mut CKS = ActualCharArray::new(FILSIZ, 1..=NCK);
        let mut XSEGID = ActualCharArray::new(SIDLEN, 1..=MAXSEG);
        let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
        let mut T: f64 = 0.0;
        let mut TBEGS = ActualArray::<f64>::new(1..=MAXSEG);
        let mut TENDS = ActualArray::<f64>::new(1..=MAXSEG);
        let mut TOL: f64 = 0.0;
        let mut XDESCR = ActualArray2D::<f64>::new(1..=DSCSIZ, 1..=MAXSEG);
        let mut CKNO: i32 = 0;
        let mut CPYHAN = ActualArray::<i32>::new(1..=FTSIZE);
        let mut HANDLE: i32 = 0;
        let mut HNDLES = StackArray::<i32, 15>::new(1..=NCK);
        let mut I: i32 = 0;
        let mut IDS = ActualArray::<i32>::new(1..=MAXSEG);
        let mut INST: i32 = 0;
        let mut J: i32 = 0;
        let mut NSEG = StackArray::<i32, 15>::new(1..=NCK);
        let mut SEGNO: i32 = 0;
        let mut AVFLAG: bool = false;
        let mut FOUND: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"bsr1.bc"),
                Val::C(b"bsr2.bc"),
                Val::C(b"bsr3.bc"),
                Val::C(b"bsr4.bc"),
                Val::C(b"bsr5.bc"),
                Val::C(b"bsr6.bc"),
                Val::C(b"bsr7.bc"),
                Val::C(b"bsr8.bc"),
                Val::C(b"bsr9.bc"),
                Val::C(b"bsr10.bc"),
                Val::C(b"bsr11.bc"),
                Val::C(b"bsr12.bc"),
                Val::C(b"bsr13.bc"),
                Val::C(b"bsr14.bc"),
                Val::C(b"bsr15.bc"),
            ]
            .into_iter();
            CKS.iter_mut()
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
                Val::I(NSEG11),
                Val::I(NSEG12),
                Val::I(NSEG13),
                Val::I(NSEG14),
                Val::I(NSEG15),
            ]
            .into_iter();
            NSEG.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            SEGID,
            SMSG,
            CKCPY,
            CKS,
            XSEGID,
            DESCR,
            T,
            TBEGS,
            TENDS,
            TOL,
            XDESCR,
            CKNO,
            CPYHAN,
            HANDLE,
            HNDLES,
            I,
            IDS,
            INST,
            J,
            NSEG,
            SEGNO,
            AVFLAG,
            FOUND,
        }
    }
}

//$Procedure F_CKBSR ( Family of tests for T_CBR )
pub fn F_CKBSR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // MAXSEG should be set to MAX ( NINST * STSIZE, FTSIZE ).
    //

    //
    // The number of segments in the respective CK files:
    //

    //
    // Other parameters:
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_CKBSR", ctx)?;

    testutil::TCASE(b"The first CK file contains 1 segment for instrument 1. Make sure we can look up data from this file.", ctx)?;

    //
    // Create the first CK file.
    //
    save.INST = 1;
    save.TBEGS[1] = ((SCALE as f64) + 0.0);
    save.TENDS[1] = ((SCALE as f64) + 1.0);
    save.CKNO = 1;

    fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[1].to_vec(),
        b"#",
        &save.CKS[1],
        &mut save.XSEGID[1],
    );
    spicelib::REPMI(&save.XSEGID[1].to_vec(), b"#", 1, &mut save.XSEGID[1], ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CRDAF(
        b"CK",
        &save.CKS[1],
        save.NSEG[1],
        &[save.INST],
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CLF(&save.CKS[1], &mut save.HNDLES[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CBS(save.INST, (save.TBEGS[1] + 0.5f32 as f64), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
    T_CRCKDS(
        1,
        save.INST,
        save.TBEGS[1],
        save.TENDS[1],
        save.XDESCR.subarray_mut([1, 1]),
        true,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 1]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Try to look up data for a different instrumentin CK 1.  Also look up data for instrument 1 for a time which is not covered.", ctx)?;

    T_CBS(2, (save.TBEGS[1] + 0.5f32 as f64), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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

    T_CBS(1, (save.TBEGS[1] + 10.0), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // In this case, the segment should not be found.
    //
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    testutil::TCASE(b"Create a second CK containing data for instrument 1 and instrument 2.  Load this CK, then look up a state covered by the new file.", ctx)?;

    save.INST = 1;
    save.CKNO = 2;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (save.I <= (save.NSEG[save.CKNO] / 2)) {
                save.IDS[save.I] = 2;
            } else {
                save.IDS[save.I] = 1;
            }

            save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
            save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.CKS[save.CKNO],
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
        b"CK",
        &save.CKS[save.CKNO],
        save.NSEG[save.CKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CLF(&save.CKS[save.CKNO], &mut save.HNDLES[save.CKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = save.NSEG[save.CKNO];

    T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
        save.HNDLES[save.CKNO],
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
    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        true,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Look up data for instrument 2.  This should cause an OLD FILES search.",
        ctx,
    )?;

    save.INST = 2;
    save.CKNO = 2;
    save.SEGNO = 1;

    T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
        save.HNDLES[save.CKNO],
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
    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        true,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Create a third CK containing data for instrument 3. Load this CK, then look up a state covered by the new file. This should cause the segment list for instrument 1 to get dumped.", ctx)?;

    save.INST = 3;
    save.CKNO = 3;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = save.INST;

            save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
            save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.CKS[save.CKNO],
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
        b"CK",
        &save.CKS[save.CKNO],
        save.NSEG[save.CKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CLF(&save.CKS[save.CKNO], &mut save.HNDLES[save.CKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = save.NSEG[save.CKNO];

    T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
        save.HNDLES[save.CKNO],
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
    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        true,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Create another CK for instrument 2 and load it. Make another read request for instrument 2 that will be satisfied by a segment in file 3. This should result in another OLD FILES search.", ctx)?;

    save.INST = 2;
    save.CKNO = 4;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = save.INST;

            save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
            save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.CKS[save.CKNO],
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
        b"CK",
        &save.CKS[save.CKNO],
        save.NSEG[save.CKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(b"Create another CK for instrument 1 and load it. The segment count in this file is such that all other instrument lists must be dumped to make room. Then make a request that is satisfied by CK 1.  The segment in CK 1 cannot be added to the segment table.", ctx)?;

    save.INST = 1;
    save.CKNO = 5;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = save.INST;

            save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
            save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.CKS[save.CKNO],
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
        b"CK",
        &save.CKS[save.CKNO],
        save.NSEG[save.CKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CLF(&save.CKS[save.CKNO], &mut save.HNDLES[save.CKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CKNO = 1;
    save.SEGNO = 1;

    save.TBEGS[save.SEGNO] = (((save.CKNO * SCALE) + save.SEGNO) - 1) as f64;
    save.TENDS[save.SEGNO] = ((save.CKNO * SCALE) + save.SEGNO) as f64;

    T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
        &save.CKS[save.CKNO],
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
        save.HNDLES[save.CKNO],
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
    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        true,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Start a segment list for instrument 1 by making a request that is satisfied by CK 1.  Then build a file (CK 6) with too many segments for instrument 1 to be buffered.  Make a request that is satisfied by CK 6. This tests the logic for searching the subset of a segment list that must be dumped due to lack of room.", ctx)?;

    //
    // Set up by making a request that will be satisfied by the segment
    // in CK 1.  This builds up the segment list for instrument 1.
    //
    save.INST = 1;
    save.TBEGS[1] = ((SCALE as f64) + 0.0);
    save.TENDS[1] = ((SCALE as f64) + 1.0);
    save.CKNO = 1;
    save.SEGNO = 1;

    fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[1].to_vec(),
        b"#",
        &save.CKS[1],
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

    T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
    save.INST = 1;
    save.CKNO = 6;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = save.INST;

            if ((save.I == 10) || (save.I == (STSIZE + 1))) {
                //
                // We want the lower bound of the re-use interval to
                // match the right endpoint of the segment's coverage
                // interval.
                //
                save.TBEGS[(save.I - 1)] = ((save.CKNO * SCALE) + save.I) as f64;
                save.TENDS[(save.I - 1)] = (save.TBEGS[(save.I - 1)] + 1.0);

                save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

                save.TBEGS[(save.I + 1)] = save.TBEGS[save.I];
                save.TENDS[(save.I + 1)] = save.TENDS[save.I];

                save.TBEGS[(save.I + 2)] = (save.TENDS[save.I] + 1 as f64);
                save.TENDS[(save.I + 2)] = (save.TBEGS[(save.I + 2)] + 1 as f64);
            } else if (save.I == (STSIZE + 6)) {
                //
                // Create a singleton segment.
                //
                save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = save.TBEGS[save.I];
            } else if (save.I == (STSIZE + 7)) {
                //
                // Create an invisible segment.
                //
                save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = (save.TBEGS[save.I] - 1 as f64);
            } else if (((save.I < 9) || ((save.I > 12) && (save.I < STSIZE)))
                || (save.I > (STSIZE + 3)))
            {
                save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);
            }

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.CKS[save.CKNO],
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
        b"CK",
        &save.CKS[save.CKNO],
        save.NSEG[save.CKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CLF(&save.CKS[save.CKNO], &mut save.HNDLES[save.CKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = 1;

    T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
        &save.CKS[save.CKNO],
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
        save.HNDLES[save.CKNO],
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
    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        true,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Create a CK containing data for ITSIZE new instruments. Look up data for each.",
        ctx,
    )?;

    //
    // Unload all CKs.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NCK;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CUF(save.HNDLES[save.I], ctx)?;
            save.I += m3__;
        }
    }

    save.CKNO = 7;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = (ITSIZE + save.I);

            save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
            save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.CKS[save.CKNO],
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
        b"CK",
        &save.CKS[save.CKNO],
        save.NSEG[save.CKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CLF(&save.CKS[save.CKNO], &mut save.HNDLES[save.CKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.INST = save.IDS[save.I];
            save.SEGNO = save.I;

            T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
                &save.CKS[save.CKNO],
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
                save.HNDLES[save.CKNO],
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
            T_CRCKDS(
                save.SEGNO,
                save.INST,
                save.TBEGS[save.SEGNO],
                save.TENDS[save.SEGNO],
                save.XDESCR.subarray_mut([1, save.SEGNO]),
                true,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, save.SEGNO]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    testutil::TCASE(b"The instrument table should be full now; the segment table should have room.  Cause an instrument list to be dumped to make room in the instrument table.", ctx)?;

    //
    // Create a list for instrument 1 more expensive than those for the
    // instruments in CK 7.  Instrument 1's list will be placed at the
    // head of the instrument table.
    //
    save.INST = 1;
    save.CKNO = 2;
    save.SEGNO = save.NSEG[save.CKNO];
    save.I = save.SEGNO;
    save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
    save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

    fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[1].to_vec(),
        b"#",
        &save.CKS[save.CKNO],
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

    T_CLF(&save.CKS[2], &mut save.HNDLES[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CBS(save.INST, (save.TBEGS[save.I] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
        save.HNDLES[save.CKNO],
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[1], OK, ctx)?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.I],
        save.TENDS[save.I],
        save.XDESCR.subarray_mut([1, 1]),
        true,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 1]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    //
    // Now do a look up for instrument 2.  This will require dumping
    // lists from CK 7.
    //
    save.INST = 2;
    save.CKNO = 2;
    save.SEGNO = 1;
    save.I = save.SEGNO;
    save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
    save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.CKS[save.CKNO],
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

    T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
        save.HNDLES[save.CKNO],
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
    T_CRCKDS(
        1,
        save.INST,
        save.TBEGS[1],
        save.TENDS[1],
        save.XDESCR.subarray_mut([1, 1]),
        true,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 1]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Look up data from a representative subset of the segments in CK 6.",
        ctx,
    )?;

    save.CKNO = 6;

    T_CLF(&save.CKS[save.CKNO], &mut save.HNDLES[save.CKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = save.INST;

            if ((save.I == 10) || (save.I == (STSIZE + 1))) {
                //
                // We want the lower bound of the re-use interval to
                // match the right endpoint of the segment's coverage
                // interval.
                //
                save.TBEGS[(save.I - 1)] = ((save.CKNO * SCALE) + save.I) as f64;
                save.TENDS[(save.I - 1)] = (save.TBEGS[(save.I - 1)] + 1.0);

                save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

                save.TBEGS[(save.I + 1)] = save.TBEGS[save.I];
                save.TENDS[(save.I + 1)] = save.TENDS[save.I];

                save.TBEGS[(save.I + 2)] = (save.TENDS[save.I] + 1 as f64);
                save.TENDS[(save.I + 2)] = (save.TBEGS[(save.I + 2)] + 1 as f64);
            } else if (save.I == (STSIZE + 6)) {
                //
                // Create a singleton segment.
                //
                save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = save.TBEGS[save.I];
            } else if (save.I == (STSIZE + 7)) {
                //
                // Create an invisible segment.
                //
                save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = (save.TBEGS[save.I] - 1 as f64);
            } else if (((save.I < 10) || ((save.I > 12) && (save.I < STSIZE)))
                || (save.I > (STSIZE + 3)))
            {
                save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
                save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);
            }

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.CKS[save.CKNO],
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

    while (save.I <= save.NSEG[save.CKNO]) {
        save.INST = 1;
        save.SEGNO = save.I;

        T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        T_CSN(
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
            &save.CKS[save.CKNO],
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
            save.HNDLES[save.CKNO],
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
        T_CRCKDS(
            save.SEGNO,
            save.INST,
            save.TBEGS[save.SEGNO],
            save.TENDS[save.SEGNO],
            save.XDESCR.subarray_mut([1, save.SEGNO]),
            true,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        T_CHDS(
            b"DESCR",
            save.DESCR.as_slice(),
            b"=",
            save.XDESCR.subarray([1, save.SEGNO]),
            (DSCSIZ - 1),
            0.0,
            OK,
            ctx,
        )?;

        //
        // Skip some tests that are unlikely to reveal bugs, as well as
        // those which would give anomalous results due to the structure
        // of CK 6.
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
    save.CKNO = 6;
    save.INST = 1;
    save.T = ((2 as f64) * save.TENDS[save.NSEG[save.CKNO]]);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
        b"Make sure all T_CBR entry points return on entry when RETURN() is .TRUE.",
        ctx,
    )?;

    //
    // Depending on whether we're calling a version of T_CBR that does
    // coverage checking, the error status may be reset.

    fstr::assign(&mut save.SMSG, b"Return on entry");

    spicelib::SIGERR(&save.SMSG, ctx)?;

    T_CBR(
        b" ",
        1,
        1,
        0.0,
        0.0,
        false,
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

    T_CLF(b" ", &mut save.HANDLE, ctx)?;

    if spicelib::RETURN(ctx) {
        testutil::CHCKXC(true, &save.SMSG, OK, ctx)?;
    } else {
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SIGERR(&save.SMSG, ctx)?;

    T_CUF(save.HANDLE, ctx)?;

    if spicelib::RETURN(ctx) {
        testutil::CHCKXC(true, &save.SMSG, OK, ctx)?;
    } else {
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SIGERR(&save.SMSG, ctx)?;

    T_CBS(1, 0.0, TOL0, true, ctx)?;

    if spicelib::RETURN(ctx) {
        testutil::CHCKXC(true, &save.SMSG, OK, ctx)?;
    } else {
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SIGERR(&save.SMSG, ctx)?;

    T_CSN(
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
        b"Make sure an error is signaled if T_CBR is called directly and RETURN() is .FALSE.",
        ctx,
    )?;

    T_CBR(
        b" ",
        1,
        1,
        0.0,
        0.0,
        false,
        save.DESCR.as_slice(),
        &save.SEGID,
        save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(CKBOGUSENTRY)", OK, ctx)?;

    //
    // Unload the CK files.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NCK;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CUF(save.HNDLES[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    //
    // Make sure an error is signaled if no CKs are loaded.
    //
    testutil::TCASE(b"Make sure an error is signaled if no CKs are loaded.", ctx)?;
    T_CBS(1, 0.0, TOL0, true, ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOLOADEDFILES)", OK, ctx)?;

    //
    // Load CK1 and look up a state from it to create a cheap list.
    // Make the cheap list the second list by looking up data from
    // it after looking up data for instrument ITSIZE+1.
    //
    testutil::TCASE(
        b"Test removal of cheap list when adding a new instrument; cheap list is 2nd.",
        ctx,
    )?;

    T_CLF(&save.CKS[1], &mut save.HNDLES[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now load the CK containing 100 instruments.  Look up data for
    // each one.  The last one will cause the list for instrument 1 to
    // be dumped.
    //
    save.CKNO = 7;
    T_CLF(&save.CKS[save.CKNO], &mut save.HNDLES[save.CKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = (ITSIZE + save.I);

            save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
            save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

            fstr::assign(save.XSEGID.get_mut(save.I), b"File: # Segno: #");

            spicelib::REPMC(
                &save.XSEGID[save.I].to_vec(),
                b"#",
                &save.CKS[save.CKNO],
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
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.INST = save.IDS[save.I];
            save.SEGNO = save.I;

            T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
                &save.CKS[save.CKNO],
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
                save.HNDLES[save.CKNO],
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
            T_CRCKDS(
                save.SEGNO,
                save.INST,
                save.TBEGS[save.SEGNO],
                save.TENDS[save.SEGNO],
                save.XDESCR.subarray_mut([1, save.SEGNO]),
                true,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, save.SEGNO]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            if (save.I == 1) {
                //
                // Create a cheap list for instrument 1.
                //
                save.INST = 1;

                T_CLF(&save.CKS[save.CKNO], &mut save.HNDLES[save.CKNO], ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                save.TBEGS[1] = SCALE as f64;

                T_CBS(save.INST, (save.TBEGS[1] + 0.5), TOL0, true, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                T_CSN(
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
        b"Test ability to make room by deleting an instrument table entry with an empty list.",
        ctx,
    )?;

    //
    // Create an example of the list in question by forcing a search
    // without buffering on instrument 1, where the highest priority file
    // contains too many segments to buffer.  However, we want this
    // list to have a high expense, so load a CK with many segments
    // for this instrument and search it first.
    //
    save.CKNO = 6;
    T_CLF(&save.CKS[save.CKNO], &mut save.HNDLES[save.CKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.INST = 1;
    save.T = (((((SCALE * save.CKNO) + (STSIZE + 1)) - 1) as f64) + 0.5);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now look up data for the first NSEG-1 instruments in CK 7.  This
    // should fill up the instrument table.
    //
    save.CKNO = 7;

    {
        let m1__: i32 = 1;
        let m2__: i32 = (save.NSEG[save.CKNO] - 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.I] = (ITSIZE + save.I);

            save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
            save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

            save.INST = save.IDS[save.I];
            save.SEGNO = save.I;

            T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
                &save.CKS[save.CKNO],
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
                save.HNDLES[save.CKNO],
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
            T_CRCKDS(
                save.SEGNO,
                save.INST,
                save.TBEGS[save.SEGNO],
                save.TENDS[save.SEGNO],
                save.XDESCR.subarray_mut([1, save.SEGNO]),
                true,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, save.SEGNO]),
                (DSCSIZ - 1),
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
    // Set up the case by unloading the currently loaded CKs.  Load
    // CK 1 and look up a state from it.  Then load CK 6.
    //

    //
    // Unload the CK files.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (NCK - 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CUF(save.HNDLES[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    //
    // Load CK 1 and look up a state from this file.
    //
    T_CLF(&save.CKS[1], &mut save.HNDLES[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.INST = 1;
    save.TBEGS[1] = ((SCALE as f64) + 0.0);
    save.TENDS[1] = ((SCALE as f64) + 1.0);
    save.CKNO = 1;
    save.SEGNO = 1;

    fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[1].to_vec(),
        b"#",
        &save.CKS[1],
        &mut save.XSEGID[1],
    );
    spicelib::REPMI(&save.XSEGID[1].to_vec(), b"#", 1, &mut save.XSEGID[1], ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // Now load CK 6.  Look up a state from segment 9, where the
    // request time is to the right of a segment whose right endpoint
    // is at the left endpoint of the re-use interval.
    //
    T_CLF(&save.CKS[6], &mut save.HNDLES[6], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CKNO = 6;
    save.INST = 1;
    save.SEGNO = 9;

    save.TBEGS[save.SEGNO] = (((save.CKNO * SCALE) + save.SEGNO) + 1) as f64;
    save.TENDS[save.SEGNO] = (save.TBEGS[save.SEGNO] + 1 as f64);

    save.T = (save.TBEGS[save.SEGNO] + 0.25);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
        &save.CKS[6],
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
        save.HNDLES[save.CKNO],
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[1], OK, ctx)?;

    //
    // Check the descriptor as well.  However, don't check the
    // segment addresses.
    //
    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        true,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    //
    // Create a situation where the segment list for instrument 1
    // contributed by CK 6 gets dumped, and where the request is
    // satisfied by a segment in CK 1.
    //
    testutil::TCASE(
        b"Dump segment list from CK 6; find segment for instrument 1 in CK 1.",
        ctx,
    )?;

    T_CLF(&save.CKS[1], &mut save.HNDLES[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CLF(&save.CKS[6], &mut save.HNDLES[6], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.INST = 1;
    save.TBEGS[1] = SCALE as f64;
    save.TENDS[1] = ((SCALE as f64) + 1.0);
    save.T = (0.5 * (save.TBEGS[1] + save.TENDS[1]));

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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

    T_CRCKDS(
        1,
        save.INST,
        save.TBEGS[1],
        save.TENDS[1],
        save.XDESCR.subarray_mut([1, 1]),
        true,
        ctx,
    )?;
    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 1]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[1].to_vec(),
        b"#",
        &save.CKS[1],
        &mut save.XSEGID[1],
    );
    spicelib::REPMI(&save.XSEGID[1].to_vec(), b"#", 1, &mut save.XSEGID[1], ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[1], OK, ctx)?;

    testutil::TCASE(b"Dump segment list from CK 6.  While searching list for segment for instrument 1, make lower bound of re-use interval match lower bound of segment descriptor.", ctx)?;

    //
    // Make CK 1 higher priority than CK 6.
    //
    T_CLF(&save.CKS[1], &mut save.HNDLES[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Place request time in the "hole" between segments STSIZE+1 and
    // STSIZE+3.
    //
    save.I = (STSIZE + 1);

    save.TBEGS[(save.I - 1)] = ((save.CKNO * SCALE) + save.I) as f64;
    save.TENDS[(save.I - 1)] = (save.TBEGS[(save.I - 1)] + 1.0);

    save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
    save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

    save.TBEGS[(save.I + 1)] = save.TBEGS[save.I];
    save.TENDS[(save.I + 1)] = save.TENDS[save.I];

    save.TBEGS[(save.I + 2)] = (save.TENDS[save.I] + 1 as f64);
    save.TENDS[(save.I + 2)] = (save.TBEGS[(save.I + 2)] + 1 as f64);

    save.T = (save.TBEGS[(save.I - 1)] + 0.5);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
        &save.CKS[6],
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
        save.HNDLES[save.CKNO],
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
    save.TBEGS[(save.I - 1)] = ((save.CKNO * SCALE) + save.I) as f64;
    save.TENDS[(save.I - 1)] = (save.TBEGS[(save.I - 1)] + 1.0);

    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        true,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check correct handling of re-use intervals.  Create a new
    // CK file that contains coverage that exemplifies the various
    // masking possibilities that may occur.
    //
    testutil::TCASE(b"Check re-use for a 1-instrument segment list.", ctx)?;

    save.CKNO = 8;

    //
    // Segment 1:
    //
    save.INST = 1;
    save.IDS[1] = save.INST;

    save.TBEGS[1] = (save.CKNO * SCALE) as f64;
    save.TENDS[1] = (save.TBEGS[1] + 1.0);

    //
    // Segments 2-3:
    //
    save.INST = 2;
    save.IDS[2] = save.INST;
    save.IDS[3] = save.INST;

    save.TBEGS[3] = (save.CKNO * SCALE) as f64;
    save.TENDS[3] = (save.TBEGS[3] + 1.0);

    save.TBEGS[2] = (save.TENDS[3] + 1.0);
    save.TENDS[2] = (save.TBEGS[2] + 1.0);

    //
    // Segments 4-6:
    //
    save.INST = 3;
    save.IDS[4] = save.INST;
    save.IDS[5] = save.INST;
    save.IDS[6] = save.INST;

    save.TBEGS[6] = (save.CKNO * SCALE) as f64;
    save.TENDS[6] = (save.TBEGS[6] + 3.0);

    save.TBEGS[5] = (save.TENDS[6] - 1.0);
    save.TENDS[5] = (save.TBEGS[5] + 3.0);

    save.TBEGS[4] = (save.TBEGS[5] + 1.0);
    save.TENDS[4] = (save.TENDS[5] - 1.0);

    //
    // Segments 7-9:
    //
    save.INST = 4;
    save.IDS[7] = save.INST;
    save.IDS[8] = save.INST;
    save.IDS[9] = save.INST;

    save.TBEGS[9] = (save.CKNO * SCALE) as f64;
    save.TENDS[9] = (save.TBEGS[9] + 3.0);

    save.TBEGS[8] = save.TBEGS[9];
    save.TENDS[8] = save.TENDS[9];

    save.TBEGS[7] = (save.TBEGS[9] - 2.0);
    save.TENDS[7] = (save.TBEGS[9] + 3.0);

    //
    // Segments 10-12:
    //
    save.INST = 5;
    save.IDS[10] = save.INST;
    save.IDS[11] = save.INST;
    save.IDS[12] = save.INST;

    save.TBEGS[12] = (save.CKNO * SCALE) as f64;
    save.TENDS[12] = (save.TBEGS[12] + 3.0);

    save.TBEGS[11] = (save.TBEGS[12] - 2.0);
    save.TENDS[11] = (save.TBEGS[11] + 3.0);

    save.TBEGS[10] = (save.TBEGS[11] - 2.0);
    save.TENDS[10] = (save.TENDS[12] + 1.0);

    //
    // Segments 13-14:
    //
    save.INST = 6;
    save.IDS[13] = save.INST;
    save.IDS[14] = save.INST;

    //
    // Singleton segment:
    //
    save.TBEGS[13] = (save.CKNO * SCALE) as f64;
    save.TENDS[13] = save.TBEGS[13];

    //
    // Invisible segment:
    //
    save.TBEGS[14] = (save.TENDS[13] + 3.0);
    save.TENDS[14] = (save.TBEGS[14] - 1.0);

    //
    // Three more segments for instrument 4:
    //
    save.IDS[15] = 4;
    save.IDS[16] = 4;
    save.IDS[17] = 4;

    save.TBEGS[15] = (((save.CKNO * SCALE) as f64) + 10.0);
    save.TENDS[15] = (save.TBEGS[15] + 3.0);

    save.TBEGS[16] = (save.TBEGS[15] + 1.0);
    save.TENDS[16] = (save.TENDS[15] - 1.0);

    save.TBEGS[17] = save.TBEGS[16];
    save.TENDS[17] = save.TENDS[16];

    //
    // Three more segments for instrument 5:
    //
    save.INST = 5;
    save.IDS[18] = save.INST;
    save.IDS[19] = save.INST;
    save.IDS[20] = save.INST;

    save.TBEGS[20] = (((save.CKNO * SCALE) as f64) + 10.0);
    save.TENDS[20] = (save.TBEGS[20] + 3.0);

    save.TBEGS[19] = (save.TBEGS[20] - 2.0);
    save.TENDS[19] = (save.TBEGS[19] + 3.0);

    save.TBEGS[18] = (save.TBEGS[19] - 2.0);
    save.TENDS[18] = (save.TENDS[20] + 1.0);

    //
    // Create a sequence of segments for instrument 6 with the
    // following topology:
    //
    //
    //          +++++++           segment 21
    //                +++++++             22
    //    +++++++                         23
    //
    //
    save.INST = 6;
    save.IDS[21] = save.INST;
    save.IDS[22] = save.INST;
    save.IDS[23] = save.INST;

    save.TBEGS[21] = (((save.CKNO * SCALE) as f64) + 10.0);
    save.TENDS[21] = (save.TBEGS[21] + 3.0);

    save.TBEGS[22] = save.TENDS[21];
    save.TENDS[22] = (save.TBEGS[21] + 3.0);

    save.TBEGS[23] = (save.TBEGS[21] - 3.0);
    save.TENDS[23] = save.TBEGS[21];

    //
    // Create the ninth CK, which is just a copy of the 8th, except
    // for descriptors and segment IDs.
    //
    save.CKNO = 9;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[9];
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CRCKDS(
                save.SEGNO,
                save.IDS[save.SEGNO],
                save.TBEGS[save.SEGNO],
                save.TENDS[save.SEGNO],
                save.XDESCR.subarray_mut([1, save.SEGNO]),
                true,
                ctx,
            )?;

            fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
            spicelib::REPMC(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                &save.CKS[save.CKNO],
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
        b"CK",
        &save.CKS[9],
        save.NSEG[9],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    //
    // Create the segment descriptors and segment identifiers for
    // this CK file.
    //
    save.CKNO = 8;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[8];
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CRCKDS(
                save.SEGNO,
                save.IDS[save.SEGNO],
                save.TBEGS[save.SEGNO],
                save.TENDS[save.SEGNO],
                save.XDESCR.subarray_mut([1, save.SEGNO]),
                true,
                ctx,
            )?;

            fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
            spicelib::REPMC(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                &save.CKS[save.CKNO],
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
    // Unload the other CK files.  Create and load the CK file.
    //
    //
    // Unload the CK files.  Again.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (save.CKNO - 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CUF(save.HNDLES[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    T_CRDAF(
        b"CK",
        &save.CKS[8],
        save.NSEG[8],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    // CALL BYEBYE ( 'SUCCESS' )

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CLF(&save.CKS[8], &mut save.HNDLES[8], ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Time for tests.
    //
    //
    // Make sure we can re-use data from the first segment for
    // instrument 1.
    //
    save.CKNO = 8;
    save.INST = save.IDS[1];
    save.T = (0.5 * (save.TBEGS[1] + save.TENDS[1]));

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(save.INST, save.T, TOL0, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
            testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 1]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[1], OK, ctx)?;

            save.I += m3__;
        }
    }

    save.T = (save.TBEGS[1] - 1.0);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    save.T = (save.TENDS[1] + 1.0);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    save.T = save.TBEGS[1];

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.T = save.TENDS[1];

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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

    save.INST = save.IDS[2];
    save.T = (0.5 * (save.TBEGS[2] + save.TENDS[2]));

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(save.INST, save.T, TOL0, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
            testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 2]),
                (DSCSIZ - 1),
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
            T_CBS(save.INST, save.T, TOL0, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
            testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 3]),
                (DSCSIZ - 1),
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

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 3]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[3], OK, ctx)?;

    save.T = save.TENDS[3];

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 3]),
        (DSCSIZ - 1),
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

    save.INST = save.IDS[5];
    save.T = (save.TENDS[6] + 0.25);

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(save.INST, save.T, TOL0, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
            testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 5]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[5], OK, ctx)?;

            save.I += m3__;
        }
    }

    save.INST = save.IDS[4];
    save.T = (save.TBEGS[6] + 0.25);

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(save.INST, save.T, TOL0, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
            testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 6]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[6], OK, ctx)?;

            save.I += m3__;
        }
    }

    save.T = (save.TBEGS[5] + 0.25);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.T = (save.TBEGS[6] - 0.25);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
    // the left endpoint of a descriptor, where T lies to the left
    // of the segment, in the CHECK LIST state:
    //
    save.INST = save.IDS[7];
    save.T = (save.TBEGS[7] + 0.25);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 7]),
        (DSCSIZ - 1),
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

    save.INST = save.IDS[10];
    save.T = (save.TENDS[12] + 0.25);

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(save.INST, save.T, TOL0, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
            testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 10]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[10], OK, ctx)?;

            save.I += m3__;
        }
    }

    save.T = (save.TENDS[10] + 1.0);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
            T_CBS(save.INST, save.T, TOL0, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
            testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 10]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[10], OK, ctx)?;

            save.I += m3__;
        }
    }

    save.T = (save.TBEGS[11] - 0.25);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 10]),
        (DSCSIZ - 1),
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
        b"T > segment upper bound.  Lower bound of re-use interval = upper bound of segment.",
        ctx,
    )?;

    save.INST = save.IDS[15];
    save.T = (save.TENDS[17] + 0.5);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 15]),
        (DSCSIZ - 1),
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
        b"T is in segment.  Lower bound of re-use interval = lower bound of segment.",
        ctx,
    )?;

    save.INST = 6;
    save.IDS[21] = save.INST;
    save.IDS[22] = save.INST;
    save.IDS[23] = save.INST;

    save.TBEGS[21] = (((save.CKNO * SCALE) as f64) + 10.0);
    save.TENDS[21] = (save.TBEGS[21] + 3.0);

    save.TBEGS[22] = save.TENDS[21];
    save.TENDS[22] = (save.TBEGS[21] + 3.0);

    save.TBEGS[23] = (save.TBEGS[21] - 3.0);
    save.TENDS[23] = save.TBEGS[21];

    save.INST = save.IDS[21];
    save.T = (save.TBEGS[21] + 0.5);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 21]),
        (DSCSIZ - 1),
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
    save.INST = save.IDS[13];

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 13]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[13], OK, ctx)?;

    //
    // Exercise the logic for handling singleton and invisible
    // segments during a NEW INST search.
    //
    testutil::TCASE(
        b"Look up data from a singleton segment, this time in a NEW SEGMENTS search.",
        ctx,
    )?;

    T_CLF(&save.CKS[9], &mut save.HNDLES[9], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = 13;
    save.CKNO = 9;

    T_CRCKDS(
        save.SEGNO,
        save.IDS[save.SEGNO],
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        true,
        ctx,
    )?;

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.CKS[save.CKNO],
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
    save.INST = save.IDS[13];

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[9], 0, OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, 13]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSC(b"SEGID", &save.SEGID, b"=", &save.XSEGID[13], OK, ctx)?;

    testutil::TCASE(b"Prepare for search w/o buffering tests: create a CK with STSIZE segments for instruments 1-NINST.", ctx)?;

    //
    // Create a CK file with STSIZE segments for instruments 1-NINST.
    //
    save.CKNO = 10;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NINST;
        let m3__: i32 = 1;
        save.INST = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = STSIZE;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.J = (((save.INST - 1) * STSIZE) + save.I);
                    save.IDS[save.J] = save.INST;

                    if (save.INST == NINST) {
                        save.TBEGS[save.J] = (((save.CKNO * SCALE) - save.I) - 1) as f64;
                    } else {
                        save.TBEGS[save.J] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
                    }

                    save.TENDS[save.J] = (save.TBEGS[save.J] + 1 as f64);

                    fstr::assign(save.XSEGID.get_mut(save.J), b"File: # Segno: #  Inst:  #");

                    spicelib::REPMC(
                        &save.XSEGID[save.J].to_vec(),
                        b"#",
                        &save.CKS[save.CKNO],
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
                        save.INST,
                        &mut save.XSEGID[save.J],
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.I += m3__;
                }
            }

            save.INST += m3__;
        }
    }

    T_CRDAF(
        b"CK",
        &save.CKS[save.CKNO],
        save.NSEG[save.CKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(b"Prepare for search w/o buffering tests: create a CK with STSIZE-3 segments for instruments 1-NINST.", ctx)?;

    //
    // Create a CK file with STSIZE-3 segments for instruments
    // 1-NINST.
    //

    save.CKNO = 11;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NINST;
        let m3__: i32 = 1;
        save.INST = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = (STSIZE - 3);
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.J = (((save.INST - 1) * (STSIZE - 3)) + save.I);
                    save.IDS[save.J] = save.INST;

                    if (save.INST == NINST) {
                        save.TBEGS[save.J] = (((save.CKNO * SCALE) - save.I) - 1) as f64;
                    } else {
                        save.TBEGS[save.J] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
                    }

                    save.TENDS[save.J] = (save.TBEGS[save.J] + 1 as f64);

                    fstr::assign(save.XSEGID.get_mut(save.J), b"File: # Segno: #  Inst:  #");

                    spicelib::REPMC(
                        &save.XSEGID[save.J].to_vec(),
                        b"#",
                        &save.CKS[save.CKNO],
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
                        save.INST,
                        &mut save.XSEGID[save.J],
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.I += m3__;
                }
            }

            save.INST += m3__;
        }
    }

    T_CRDAF(
        b"CK",
        &save.CKS[save.CKNO],
        save.NSEG[save.CKNO],
        save.IDS.as_slice(),
        save.TBEGS.as_slice(),
        save.TENDS.as_slice(),
        save.XSEGID.as_arg(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(
        b"Search w/o buffering, T < segment begin, re-use interval right endpoint < segment begin.",
        ctx,
    )?;

    //
    // Unload the CK files.  Again.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NCK;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CUF(save.HNDLES[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    //
    // Load CKs 8 and 10.
    //
    T_CLF(&save.CKS[8], &mut save.HNDLES[8], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CLF(&save.CKS[10], &mut save.HNDLES[10], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The request time should precede the coverage of segment 3 in
    // CK 8.
    //
    save.INST = 2;
    save.T = (((8 * SCALE) as f64) - 1.0);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    testutil::TCASE(
        b"Search w/o buffering, T within segment, re-use interval, left endpoint > segment begin.",
        ctx,
    )?;

    //
    // The request time should precede the coverage of segment 3 in
    // CK 8.
    //
    save.INST = 3;
    save.SEGNO = 5;
    save.CKNO = 8;

    save.TBEGS[6] = (save.CKNO * SCALE) as f64;
    save.TENDS[6] = (save.TBEGS[6] + 3.0);

    save.TBEGS[5] = (save.TENDS[6] - 1.0);
    save.TENDS[5] = (save.TBEGS[5] + 3.0);

    save.T = (((save.CKNO * SCALE) as f64) + 4.0);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
        &save.CKS[save.CKNO],
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
    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        true,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Search w/o buffering, T < segment begin, re-use interval right endpoint = segment begin.",
        ctx,
    )?;
    save.INST = 4;
    save.SEGNO = 7;

    save.IDS[7] = save.INST;
    save.IDS[8] = save.INST;
    save.IDS[9] = save.INST;

    save.TBEGS[9] = (save.CKNO * SCALE) as f64;
    save.TENDS[9] = (save.TBEGS[9] + 3.0);

    save.TBEGS[8] = save.TBEGS[9];
    save.TENDS[8] = save.TENDS[9];

    save.TBEGS[7] = (save.TBEGS[9] - 2.0);
    save.TENDS[7] = (save.TBEGS[9] + 3.0);

    save.T = (save.TBEGS[8] - 1.0);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.CKS[save.CKNO],
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
    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        true,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    //
    // Some cases where a partial list must be dumped:
    //
    testutil::TCASE(b"Dump segment list from CK 11.  While searching list for segment for instrument 4, make upper bound of re-use interval < upper bound of segment descriptor.", ctx)?;

    //
    // Unload CK 10; load CK 11.
    //
    T_CUF(save.HNDLES[10], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CLF(&save.CKS[11], &mut save.HNDLES[11], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Our request time should be in the interior of segment 15 in
    // CK 8.
    //
    save.CKNO = 8;
    save.SEGNO = 15;

    save.IDS[15] = 4;
    save.IDS[16] = 4;
    save.IDS[17] = 4;

    save.TBEGS[15] = (((save.CKNO * SCALE) as f64) + 10.0);
    save.TENDS[15] = (save.TBEGS[15] + 3.0);

    save.TBEGS[16] = (save.TBEGS[15] + 1.0);
    save.TENDS[16] = (save.TENDS[15] - 1.0);

    save.TBEGS[17] = save.TBEGS[16];
    save.TENDS[17] = save.TBEGS[17];

    save.T = (save.TBEGS[15] + 0.5);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.CKS[save.CKNO],
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
    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        true,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Dump segment list from CK 11.  While searching list for segment for instrument 4, make lower bound of re-use interval = upper bound of segment descriptor.", ctx)?;

    save.CKNO = 8;
    save.INST = 4;
    save.TBEGS[9] = (save.CKNO * SCALE) as f64;
    save.TENDS[9] = (save.TBEGS[9] + 3.0);
    save.T = (save.TENDS[9] + 0.5);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    testutil::TCASE(b"Dump segment list from CK 11.  While searching list for segment for instrument 5, make lower bound of re-use interval > lower bound of segment descriptor.", ctx)?;

    save.INST = 5;
    save.IDS[18] = save.INST;
    save.IDS[19] = save.INST;
    save.IDS[20] = save.INST;

    save.TBEGS[20] = (((save.CKNO * SCALE) as f64) + 10.0);
    save.TENDS[20] = (save.TBEGS[20] + 3.0);

    save.TBEGS[19] = (save.TBEGS[20] - 2.0);
    save.TENDS[19] = (save.TBEGS[19] + 3.0);

    save.TBEGS[18] = (save.TBEGS[19] - 2.0);
    save.TENDS[18] = (save.TENDS[20] + 1.0);

    save.T = (save.TENDS[18] - 0.5);

    T_CBS(save.INST, save.T, TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HNDLES[8], 0, OK, ctx)?;

    save.SEGNO = 18;

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.CKS[save.CKNO],
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
    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, save.SEGNO]),
        true,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    T_CHDS(
        b"DESCR",
        save.DESCR.as_slice(),
        b"=",
        save.XDESCR.subarray([1, save.SEGNO]),
        (DSCSIZ - 1),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Create a situation where room is needed in the instrument table, and the second instrument list has expense greater than the first.", ctx)?;

    //
    // Unload CKs 8 and 11.
    //
    T_CUF(save.HNDLES[8], ctx)?;
    T_CUF(save.HNDLES[11], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fill up (nearly) the segment table with a cheap list for
    // instrument 2 and an expensive list for instrument 1.
    //
    T_CLF(&save.CKS[8], &mut save.HNDLES[8], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.INST = 2;
    save.IDS[2] = save.INST;
    save.IDS[3] = save.INST;

    save.TBEGS[3] = (save.CKNO * SCALE) as f64;
    save.TENDS[3] = (save.TBEGS[3] + 1.0);

    save.INST = 2;
    save.CKNO = 8;
    save.SEGNO = 3;

    T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
        save.HNDLES[save.CKNO],
        0,
        OK,
        ctx,
    )?;

    T_CLF(&save.CKS[11], &mut save.HNDLES[11], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.INST = 1;
    save.CKNO = 11;
    save.SEGNO = 1;
    save.I = 1;
    save.TBEGS[save.I] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
    save.TENDS[save.I] = (save.TBEGS[save.I] + 1 as f64);

    T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
        save.HNDLES[save.CKNO],
        0,
        OK,
        ctx,
    )?;

    //
    // Now do a look up for instrument 3.  This should cause the segment
    // lists for instruments 2 and 1 to get dumped.
    //
    save.INST = 3;
    save.CKNO = 11;

    save.I = 1;
    save.J = (((save.INST - 1) * (STSIZE - 3)) + save.I);

    save.TBEGS[save.J] = (((save.CKNO * SCALE) + save.I) - 1) as f64;
    save.TENDS[save.J] = (save.TBEGS[save.I] + 1 as f64);

    fstr::assign(
        save.XSEGID.get_mut(save.J),
        b"File: # Segno: #  Instrument:  #",
    );

    spicelib::REPMC(
        &save.XSEGID[save.I].to_vec(),
        b"#",
        &save.CKS[save.CKNO],
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
        save.INST,
        &mut save.XSEGID[save.I],
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = save.J;

    T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
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
        save.HNDLES[save.CKNO],
        0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Try DAFOPR error handling.", ctx)?;

    T_CLF(b"ThisFileDoesNotExist", &mut save.HANDLE, ctx)?;

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
    // Unload the CK files.  The load files 1 and 2.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NCK;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CUF(save.HNDLES[save.I], ctx)?;
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
            T_CLF(&save.CKS[save.I], &mut save.HNDLES[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    //
    // Do lookups for instrument 1 that hit both files.
    //
    save.INST = 1;
    save.TBEGS[1] = ((SCALE as f64) + 0.0);

    T_CBS(save.INST, (save.TBEGS[1] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.INST = 1;
    save.CKNO = 2;

    save.SEGNO = ((save.NSEG[save.CKNO] / 2) + 1);

    save.TBEGS[save.SEGNO] = (((save.CKNO * SCALE) + save.SEGNO) - 1) as f64;

    T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Do a lookup for instrument 2 to create a segment list for that
    // instrument.
    //
    save.INST = 2;
    save.CKNO = 2;

    save.SEGNO = (save.NSEG[save.CKNO] / 2);

    save.TBEGS[save.SEGNO] = (((save.CKNO * SCALE) + save.SEGNO) - 1) as f64;

    T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Reload file 1, removing the portion of instrument 1's segment list
    // that came from file 1, as part of the unload process that
    // precedes re-loading file 1.
    //
    T_CLF(&save.CKS[1], &mut save.HNDLES[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create FTSIZE copies of CK 1 and load FTSIZE-1 of them.  We
    // should get a file table overflow error.
    //
    testutil::TCASE(b"File table overflow error.", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = FTSIZE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(save.CKCPY.get_mut(save.I), b"copy#.bc");
            spicelib::REPMI(
                &save.CKCPY[save.I].to_vec(),
                b"#",
                save.I,
                &mut save.CKCPY[save.I],
                ctx,
            );

            save.INST = 1;
            save.TBEGS[1] = ((SCALE as f64) + 0.0);
            save.TENDS[1] = ((SCALE as f64) + 1.0);
            save.CKNO = 1;
            save.SEGNO = 1;

            fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
            spicelib::REPMC(
                &save.XSEGID[1].to_vec(),
                b"#",
                &save.CKCPY[save.I],
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

            T_CRDAF(
                b"CK",
                &save.CKCPY[save.I],
                save.NSEG[1],
                &[save.INST],
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
            T_CLF(&save.CKCPY[save.I], &mut save.CPYHAN[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }

    T_CLF(
        &save.CKCPY[(FTSIZE - 1)],
        &mut save.CPYHAN[(FTSIZE - 1)],
        ctx,
    )?;
    //
    // If the error is caught by the handle manager, the short message
    // check should be:
    //
    //    CALL CHCKXC ( .TRUE., 'SPICE(FTFULL)', OK )
    //
    // Since the BSR file table is smaller than the handle manager
    // table, the error check is as shown below:
    //
    testutil::CHCKXC(true, b"SPICE(CKTOOMANYFILES)", OK, ctx)?;

    //
    // Loading, unloading, and priority checks:
    //
    testutil::TCASE(b"Load all copies of CK 1, looking up the same state from each.  Unload the files in reverse order.  Repeat 3 times.", ctx)?;

    //
    // First, make sure all files are unloaded.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NCK;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CUF(save.HNDLES[save.I], ctx)?;
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
            T_CUF(save.CPYHAN[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    save.INST = 1;

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

                    T_CLF(&save.CKCPY[save.J], &mut save.CPYHAN[save.J], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(save.XSEGID.get_mut(save.J), b"File: # Segno: #");
                    spicelib::REPMC(
                        &save.XSEGID[save.J].to_vec(),
                        b"#",
                        &save.CKCPY[save.J],
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

                    T_CBS(save.INST, (save.TBEGS[save.J] + 0.5), TOL0, true, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    T_CSN(
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
                    T_CRCKDS(
                        save.SEGNO,
                        save.INST,
                        save.TBEGS[save.J],
                        save.TENDS[save.J],
                        save.XDESCR.subarray_mut([1, save.J]),
                        true,
                        ctx,
                    )?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    T_CHDS(
                        b"DESCR",
                        save.DESCR.as_slice(),
                        b"=",
                        save.XDESCR.subarray([1, save.J]),
                        (DSCSIZ - 1),
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
                    T_CUF(save.CPYHAN[(save.J + 1)], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.TBEGS[save.J] = ((SCALE as f64) + 0.0);
                    save.TENDS[save.J] = ((SCALE as f64) + 1.0);

                    fstr::assign(save.XSEGID.get_mut(save.J), b"File: # Segno: #");
                    spicelib::REPMC(
                        &save.XSEGID[save.J].to_vec(),
                        b"#",
                        &save.CKCPY[save.J],
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

                    T_CBS(save.INST, (save.TBEGS[save.J] + 0.5), TOL0, true, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    T_CSN(
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
                    T_CRCKDS(
                        save.SEGNO,
                        save.INST,
                        save.TBEGS[save.J],
                        save.TENDS[save.J],
                        save.XDESCR.subarray_mut([1, save.J]),
                        true,
                        ctx,
                    )?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    T_CHDS(
                        b"DESCR",
                        save.DESCR.as_slice(),
                        b"=",
                        save.XDESCR.subarray([1, save.J]),
                        (DSCSIZ - 1),
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
    testutil::TCASE(b"Load the first CK file 2*FTSIZE times.", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = (FTSIZE * 2);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SEGNO = 1;
            save.TBEGS[1] = ((SCALE as f64) + 0.0);
            save.TENDS[1] = ((SCALE as f64) + 1.0);

            T_CLF(&save.CKS[1], &mut save.HNDLES[1], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(save.XSEGID.get_mut(1), b"File: # Segno: #");
            spicelib::REPMC(
                &save.XSEGID[1].to_vec(),
                b"#",
                &save.CKS[1],
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

            T_CBS(save.INST, (save.TBEGS[1] + 0.5), TOL0, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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

            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 1]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Tests using non-zero tolerance follow...
    // We'll make use of CK8.
    //
    T_CLF(&save.CKS[8], &mut save.HNDLES[8], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The topology of the coverage for segments 10-12 is as shown.
    //
    //   ++++++++++++++++++        segment 10
    //       +++++++                       11
    //           ++++++++                  12
    //

    testutil::TCASE(b"Request time is covered by segment 11, but tolerance allows segment 12 to cover request.Repeat the lookup to test the re-use interval.", ctx)?;

    save.CKNO = 8;
    save.INST = 5;
    save.SEGNO = 12;
    save.TOL = 1.5;

    save.TBEGS[12] = (save.CKNO * SCALE) as f64;
    save.TENDS[12] = (save.TBEGS[12] + 3.0);

    save.TBEGS[11] = (save.TBEGS[12] - 2.0);
    save.TENDS[11] = (save.TBEGS[11] + 3.0);

    save.TBEGS[10] = (save.TBEGS[11] - 2.0);
    save.TENDS[10] = (save.TENDS[12] + 1.0);

    save.TBEGS[save.SEGNO] = (save.CKNO * SCALE) as f64;
    save.TENDS[save.SEGNO] = (save.TBEGS[save.SEGNO] + 3.0);

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.CKS[save.CKNO],
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

    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, 1]),
        true,
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(
                save.INST,
                (save.TBEGS[save.SEGNO] - 1.0),
                save.TOL,
                true,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
                save.HNDLES[save.CKNO],
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
            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 1]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    testutil::TCASE(
        b"Repeat the test with tolerance too small to catch segment 12.  We should hit segment 11.",
        ctx,
    )?;

    save.TOL = 0.5;
    save.SEGNO = 11;

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.CKS[save.CKNO],
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

    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, 1]),
        true,
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(save.INST, (save.TBEGS[12] - 1.0), save.TOL, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
                save.HNDLES[save.CKNO],
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
            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 1]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    testutil::TCASE(b"Request time is covered by segment 10, but tolerance allows segment 10 to cover request.Repeat the lookup to test the re-use interval.", ctx)?;

    save.SEGNO = 11;
    save.TOL = 1.1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(save.INST, (save.TBEGS[11] - 1.0), save.TOL, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
                save.HNDLES[save.CKNO],
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
            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 1]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Now check handling of tolerance of the left side of the
    // request time.
    //
    testutil::TCASE(b"Request time is covered by segment 10, but tolerance allows segment 12 to cover request.Repeat the lookup to test the re-use interval.", ctx)?;

    save.SEGNO = 12;
    save.TOL = 0.5;

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.CKS[save.CKNO],
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

    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, 1]),
        true,
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(
                save.INST,
                (save.TENDS[save.SEGNO] + 0.01),
                save.TOL,
                true,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
                save.HNDLES[save.CKNO],
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
            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 1]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    testutil::TCASE(
        b"Repeat the test with tolerance too small to catch segment 12.  We should hit segment 10.",
        ctx,
    )?;

    save.SEGNO = 10;
    save.TOL = 0.001;

    fstr::assign(save.XSEGID.get_mut(save.SEGNO), b"File: # Segno: #");
    spicelib::REPMC(
        &save.XSEGID[save.SEGNO].to_vec(),
        b"#",
        &save.CKS[save.CKNO],
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

    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, 1]),
        true,
        ctx,
    )?;

    //
    // Repeat lookup to test the re-use interval.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(save.INST, (save.TENDS[12] + 0.01), save.TOL, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
                save.HNDLES[save.CKNO],
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
            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 1]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Load CKs 11 and 10, in that order.  Most segments from
    // CK 10 can't be buffered.
    //
    T_CLF(&save.CKS[11], &mut save.HNDLES[11], ctx)?;
    T_CLF(&save.CKS[10], &mut save.HNDLES[10], ctx)?;

    //
    // Do a look up that will hit the last segment for inst NINST
    // in CK 10, using non-zero tolerance.
    //
    testutil::TCASE(b"Do a look up that will hit the last segment for instrument NINST in CK 10, relying on tolerance to catch the segment.", ctx)?;

    save.CKNO = 10;
    save.INST = NINST;

    save.I = STSIZE;
    save.J = (((save.INST - 1) * STSIZE) + save.I);
    save.IDS[save.J] = save.INST;

    save.TBEGS[save.J] = (((save.CKNO * SCALE) - save.I) - 1) as f64;

    save.TENDS[save.J] = (save.TBEGS[save.J] + 1 as f64);

    fstr::assign(save.XSEGID.get_mut(save.J), b"File: # Segno: #  Inst:  #");

    spicelib::REPMC(
        &save.XSEGID[save.J].to_vec(),
        b"#",
        &save.CKS[save.CKNO],
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
        save.INST,
        &mut save.XSEGID[save.J],
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = save.J;

    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, 1]),
        true,
        ctx,
    )?;

    save.TOL = 0.1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(
                save.INST,
                (save.TBEGS[save.SEGNO] - 0.01),
                save.TOL,
                true,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
                save.HNDLES[save.CKNO],
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
            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 1]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Do a look up that will hit the last segment for inst NINST-1
    // in CK 10, using non-zero tolerance.
    //
    testutil::TCASE(b"Do a look up that will hit the last segment for instrument NINST-1 in CK 10, relying on tolerance to catch the segment.", ctx)?;

    save.CKNO = 10;
    save.INST = (NINST - 1);

    save.I = STSIZE;
    save.J = (((save.INST - 1) * STSIZE) + save.I);
    save.IDS[save.J] = save.INST;

    save.TBEGS[save.J] = (((save.CKNO * SCALE) + save.I) - 1) as f64;

    save.TENDS[save.J] = (save.TBEGS[save.J] + 1 as f64);

    fstr::assign(save.XSEGID.get_mut(save.J), b"File: # Segno: #  Inst:  #");

    spicelib::REPMC(
        &save.XSEGID[save.J].to_vec(),
        b"#",
        &save.CKS[save.CKNO],
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
        save.INST,
        &mut save.XSEGID[save.J],
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEGNO = save.J;

    T_CRCKDS(
        save.SEGNO,
        save.INST,
        save.TBEGS[save.SEGNO],
        save.TENDS[save.SEGNO],
        save.XDESCR.subarray_mut([1, 1]),
        true,
        ctx,
    )?;

    save.TOL = 0.1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(
                save.INST,
                (save.TENDS[save.SEGNO] + 0.01),
                save.TOL,
                true,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
                save.HNDLES[save.CKNO],
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
            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, 1]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    testutil::TCASE(b"Make sure presence of a.v. data is not considered for lookups for which T_CBS was supplied a \"need a.v.\" flag of .FALSE.", ctx)?;

    //
    // Now check that segments w/o angular velocity are handled
    // correctly.  Create a CK file with STSIZE segments for
    // instrument 1.  For N = 1, ... STSIZE/2, the segments
    // indexed 2N-1 have no angular velocity, while the segments
    // indexed 2N do have angular velocity.
    //
    //
    // Create a CK file with STSIZE segments for instruments 1-NINST.
    //
    save.CKNO = 12;
    save.INST = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.IDS[save.SEGNO] = save.INST;

            save.TBEGS[save.SEGNO] = (((save.CKNO * SCALE) + save.SEGNO) - 1) as f64;
            save.TENDS[save.SEGNO] = (save.TBEGS[save.SEGNO] + 1 as f64);

            fstr::assign(
                save.XSEGID.get_mut(save.SEGNO),
                b"File: # Segno: #  Inst:  #",
            );

            spicelib::REPMC(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                &save.CKS[save.CKNO],
                &mut save.XSEGID[save.SEGNO],
            );
            spicelib::REPMI(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                save.SEGNO,
                &mut save.XSEGID[save.SEGNO],
                ctx,
            );
            spicelib::REPMI(
                &save.XSEGID[save.SEGNO].to_vec(),
                b"#",
                save.INST,
                &mut save.XSEGID[save.SEGNO],
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.AVFLAG = spicelib::EVEN(save.SEGNO);

            T_CRCKDS(
                save.SEGNO,
                save.IDS[save.SEGNO],
                save.TBEGS[save.SEGNO],
                save.TENDS[save.SEGNO],
                save.XDESCR.subarray_mut([1, save.SEGNO]),
                save.AVFLAG,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.SEGNO += m3__;
        }
    }

    //
    // Create the CK directly, since we want to have control over the
    // value of the a.v. flag.
    //
    spicelib::CKOPN(
        &save.CKS[save.CKNO],
        b" ",
        0,
        &mut save.HNDLES[save.CKNO],
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::DAFBNA(
                save.HNDLES[save.CKNO],
                save.XDESCR.subarray([1, save.SEGNO]),
                &save.XSEGID[save.SEGNO],
                ctx,
            )?;
            spicelib::DAFENA(ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.SEGNO += m3__;
        }
    }

    spicelib::CKCLS(save.HNDLES[save.CKNO], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Unload all files.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NCK;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CUF(save.HNDLES[save.I], ctx)?;
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
            T_CUF(save.CPYHAN[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    // Let's look up data without requiring a.v.
    //
    T_CLF(&save.CKS[save.CKNO], &mut save.HNDLES[save.CKNO], ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, false, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
                save.HNDLES[save.CKNO],
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
            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, save.SEGNO]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            save.SEGNO += m3__;
        }
    }

    testutil::TCASE(
        b"Now make sure that segments without a.v. are not seen when a.v. is requested.",
        ctx,
    )?;

    //
    // Repeat, now requiring a.v.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
            testutil::CHCKSL(b"FOUND", save.FOUND, spicelib::EVEN(save.SEGNO), OK, ctx)?;

            if save.FOUND {
                testutil::CHCKSI(
                    b"HANDLE",
                    save.HANDLE,
                    b"=",
                    save.HNDLES[save.CKNO],
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
                T_CHDS(
                    b"DESCR",
                    save.DESCR.as_slice(),
                    b"=",
                    save.XDESCR.subarray([1, save.SEGNO]),
                    (DSCSIZ - 1),
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            save.SEGNO += m3__;
        }
    }

    testutil::TCASE(b"Repeat the previous tests after loading CK2. This requires the search to be performed on a partial segment list.", ctx)?;

    T_CLF(&save.CKS[2], &mut save.HNDLES[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, false, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
                save.HNDLES[save.CKNO],
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
            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, save.SEGNO]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            save.SEGNO += m3__;
        }
    }

    //
    // Repeat, now requiring a.v.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
            testutil::CHCKSL(b"FOUND", save.FOUND, spicelib::EVEN(save.SEGNO), OK, ctx)?;

            if save.FOUND {
                testutil::CHCKSI(
                    b"HANDLE",
                    save.HANDLE,
                    b"=",
                    save.HNDLES[save.CKNO],
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
                T_CHDS(
                    b"DESCR",
                    save.DESCR.as_slice(),
                    b"=",
                    save.XDESCR.subarray([1, save.SEGNO]),
                    (DSCSIZ - 1),
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            save.SEGNO += m3__;
        }
    }

    testutil::TCASE(b"Repeat the previous tests after loading CK10. This requires the search to be performed on an unbuffered file (CK12).", ctx)?;

    T_CLF(&save.CKS[10], &mut save.HNDLES[10], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, false, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
                save.HNDLES[save.CKNO],
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
            T_CHDS(
                b"DESCR",
                save.DESCR.as_slice(),
                b"=",
                save.XDESCR.subarray([1, save.SEGNO]),
                (DSCSIZ - 1),
                0.0,
                OK,
                ctx,
            )?;

            save.SEGNO += m3__;
        }
    }

    //
    // Repeat, now requiring a.v.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSEG[save.CKNO];
        let m3__: i32 = 1;
        save.SEGNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CBS(save.INST, (save.TBEGS[save.SEGNO] + 0.5), TOL0, true, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            T_CSN(
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
            testutil::CHCKSL(b"FOUND", save.FOUND, spicelib::EVEN(save.SEGNO), OK, ctx)?;

            if save.FOUND {
                testutil::CHCKSI(
                    b"HANDLE",
                    save.HANDLE,
                    b"=",
                    save.HNDLES[save.CKNO],
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
                T_CHDS(
                    b"DESCR",
                    save.DESCR.as_slice(),
                    b"=",
                    save.XDESCR.subarray([1, save.SEGNO]),
                    (DSCSIZ - 1),
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            save.SEGNO += m3__;
        }
    }

    testutil::TCASE(b"Test our ability to resume searches after a segment is found.  Resume searches of the normal segment list, partial list, and of an unbuffered file.", ctx)?;

    //
    // Create files to be used in continued searches.  All segments
    // in these files have identical coverage.
    //
    {
        let m1__: i32 = 13;
        let m2__: i32 = 15;
        let m3__: i32 = 1;
        save.CKNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.INST = 1;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NSEG[save.CKNO];
                let m3__: i32 = 1;
                save.SEGNO = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.IDS[save.SEGNO] = save.INST;

                    save.TBEGS[save.SEGNO] = ((13 * SCALE) - 1) as f64;
                    save.TENDS[save.SEGNO] = (save.TBEGS[save.SEGNO] + 1 as f64);

                    fstr::assign(
                        save.XSEGID.get_mut(save.SEGNO),
                        b"File: # Segno: #  Inst:  #",
                    );

                    spicelib::REPMC(
                        &save.XSEGID[save.SEGNO].to_vec(),
                        b"#",
                        &save.CKS[save.CKNO],
                        &mut save.XSEGID[save.SEGNO],
                    );
                    spicelib::REPMI(
                        &save.XSEGID[save.SEGNO].to_vec(),
                        b"#",
                        save.SEGNO,
                        &mut save.XSEGID[save.SEGNO],
                        ctx,
                    );
                    spicelib::REPMI(
                        &save.XSEGID[save.SEGNO].to_vec(),
                        b"#",
                        save.INST,
                        &mut save.XSEGID[save.SEGNO],
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.SEGNO += m3__;
                }
            }

            T_CRDAF(
                b"CK",
                &save.CKS[save.CKNO],
                save.NSEG[save.CKNO],
                save.IDS.as_slice(),
                save.TBEGS.as_slice(),
                save.TENDS.as_slice(),
                save.XSEGID.as_arg(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.CKNO += m3__;
        }
    }

    //
    // Load the files.
    //
    {
        let m1__: i32 = 13;
        let m2__: i32 = 15;
        let m3__: i32 = 1;
        save.CKNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CLF(&save.CKS[save.CKNO], &mut save.HNDLES[save.CKNO], ctx)?;
            save.CKNO += m3__;
        }
    }

    //
    // Start the search.
    //
    T_CBS(save.INST, (save.TBEGS[1] + 0.5), TOL0, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 15;
        let m2__: i32 = 13;
        let m3__: i32 = -1;
        save.CKNO = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = save.NSEG[save.CKNO];
                let m2__: i32 = 1;
                let m3__: i32 = -1;
                save.SEGNO = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    fstr::assign(
                        save.XSEGID.get_mut(save.SEGNO),
                        b"File: # Segno: #  Inst:  #",
                    );
                    spicelib::REPMC(
                        &save.XSEGID[save.SEGNO].to_vec(),
                        b"#",
                        &save.CKS[save.CKNO],
                        &mut save.XSEGID[save.SEGNO],
                    );
                    spicelib::REPMI(
                        &save.XSEGID[save.SEGNO].to_vec(),
                        b"#",
                        save.SEGNO,
                        &mut save.XSEGID[save.SEGNO],
                        ctx,
                    );
                    spicelib::REPMI(
                        &save.XSEGID[save.SEGNO].to_vec(),
                        b"#",
                        save.INST,
                        &mut save.XSEGID[save.SEGNO],
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    T_CSN(
                        &mut save.HANDLE,
                        save.DESCR.as_slice_mut(),
                        &mut save.SEGID,
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    //
                    // The segment should be found.  Make sure
                    // we get back the right handle and segment identifier.
                    //
                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                    if save.FOUND {
                        testutil::CHCKSI(
                            b"HANDLE",
                            save.HANDLE,
                            b"=",
                            save.HNDLES[save.CKNO],
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
                        T_CRCKDS(
                            save.SEGNO,
                            save.IDS[save.SEGNO],
                            save.TBEGS[save.SEGNO],
                            save.TENDS[save.SEGNO],
                            save.XDESCR.subarray_mut([1, save.SEGNO]),
                            true,
                            ctx,
                        )?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        T_CHDS(
                            b"DESCR",
                            save.DESCR.as_slice(),
                            b"=",
                            save.XDESCR.subarray([1, save.SEGNO]),
                            (DSCSIZ - 1),
                            0.0,
                            OK,
                            ctx,
                        )?;
                    }

                    save.SEGNO += m3__;
                }
            }

            save.CKNO += m3__;
        }
    }

    //
    // Finally, we should end up with FOUND = .FALSE.
    //

    T_CSN(
        &mut save.HANDLE,
        save.DESCR.as_slice_mut(),
        &mut save.SEGID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // Last step:  delete all of the CK files we created.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NCK;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_CUF(save.HNDLES[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DELFIL(&save.CKS[save.I], ctx)?;
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
            T_CUF(save.CPYHAN[save.I], ctx)?;
            spicelib::DELFIL(&save.CKCPY[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
