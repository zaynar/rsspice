//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const C05TP0: i32 = 0;
const C05TP1: i32 = (C05TP0 + 1);
const C05TP2: i32 = (C05TP1 + 1);
const C05TP3: i32 = (C05TP2 + 1);
const C05PS0: i32 = 8;
const C05PS1: i32 = 4;
const C05PS2: i32 = 14;
const C05PS3: i32 = 7;
const MAXDEG: i32 = 23;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const C06TP0: i32 = 0;
const C06TP1: i32 = (C06TP0 + 1);
const C06TP2: i32 = (C06TP1 + 1);
const C06TP3: i32 = (C06TP2 + 1);
const C06NST: i32 = 4;
const C06PS0: i32 = 8;
const C06PS1: i32 = 4;
const C06PS2: i32 = 14;
const C06PS3: i32 = 7;
const C06MXZ: i32 = C06PS2;
const C06MNZ: i32 = C06PS1;
const MAXRSZ: i32 = (4 + ((MAXDEG + 1) * (C06PS3 + 1)));
const QSIZ: i32 = 4;
const QAVSIZ: i32 = 7;
const CK1DTP: i32 = 1;
const CK1RSZ: i32 = 8;
const CK2DTP: i32 = 2;
const CK2RSZ: i32 = 10;
const CK3DTP: i32 = 3;
const CK3RSZ: i32 = 17;
const CK4DTP: i32 = 4;
const CK4PCD: f64 = 128.0;
const CK4MXD: i32 = 18;
const CK4SFT: i32 = 10;
const CK4RSZ: i32 = (((CK4MXD + 1) * QAVSIZ) + CK4SFT);
const CK5DTP: i32 = 5;
const CK5MXD: i32 = 23;
const CK5MET: i32 = 4;
const CK5MXP: i32 = 14;
const CK5RSZ: i32 = (((CK5MXD + 1) * CK5MXP) + CK5MET);
const CK6DTP: i32 = 6;
const CK6MXD: i32 = 23;
const CK6MET: i32 = 4;
const CK6PS3: i32 = 7;
const CK6RSZ: i32 = (((CK6MXD + 1) * (CK6PS3 + 1)) + CK6MET);
const CKMRSZ: i32 = CK5RSZ;
const CK: &[u8] = b"ckcov.bc";
const EK: &[u8] = b"ckcov.bes";
const SPK: &[u8] = b"ckcov.bsp";
const XFRCK: &[u8] = b"ckcov.xc";
const LBCELL: i32 = -5;
const LONGLN: i32 = 240;
const LNSIZE: i32 = 80;
const MAXDEF: i32 = 15;
const NINS: i32 = 6;
const MAXCOV: i32 = 10000;
const MAXREC: i32 = 10000;
const MXMINI: i32 = MAXREC;
const SIDLEN: i32 = 40;
const MAXPKT: i32 = C05PS2;

struct SaveVars {
    CVSTAT: Vec<u8>,
    DEFTXT: ActualCharArray,
    SEGID: Vec<u8>,
    TITLE: Vec<u8>,
    ANGLE: f64,
    AVVS: ActualArray2D<f64>,
    CMAT: StackArray2D<f64, 9>,
    COVER: ActualArray<f64>,
    CLEND: ActualArray<f64>,
    CLSTRT: ActualArray<f64>,
    DC: StackArray<f64, 2>,
    DLTANG: f64,
    ENDS: ActualArray<f64>,
    EPOCHS: ActualArray<f64>,
    ET: f64,
    FIRST: f64,
    INSETS: StackArray<f64, 2>,
    IVLBDS: ActualArray<f64>,
    LAST: f64,
    PACKET: StackArray<f64, 14>,
    PKTS: ActualArray<f64>,
    QUATS: ActualArray2D<f64>,
    RATE: f64,
    RATES: ActualArray<f64>,
    STARTS: ActualArray<f64>,
    T3END: f64,
    TMPWIN: ActualArray<f64>,
    TOL: f64,
    XAVINT: ActualArray2D<f64>,
    XAVSEG: ActualArray2D<f64>,
    XCVINT: ActualArray2D<f64>,
    XCVSEG: ActualArray2D<f64>,
    Z: StackArray<f64, 3>,
    CLKID: StackArray<i32, 6>,
    DEFSIZ: i32,
    DEGREE: i32,
    DEGRES: ActualArray<i32>,
    INST: StackArray<i32, 6>,
    HANDLE: i32,
    IDS: StackArray<i32, 13>,
    IVLN: StackArray<i32, 6>,
    L: i32,
    M: i32,
    NIVREC: i32,
    NMINI: i32,
    NPKTS: ActualArray<i32>,
    NSEG: StackArray<i32, 6>,
    NR: StackArray<i32, 6>,
    NREC: i32,
    NSTART: i32,
    PKTSIZ: i32,
    SUBTPS: ActualArray<i32>,
    SUBTYP: i32,
    TIKPER: StackArray<i32, 6>,
    XIDS: StackArray<i32, 13>,
    XUNIT: i32,
    SELLST: bool,
    USEAV: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CVSTAT = vec![b' '; LNSIZE as usize];
        let mut DEFTXT = ActualCharArray::new(LNSIZE, 1..=MAXDEF);
        let mut SEGID = vec![b' '; SIDLEN as usize];
        let mut TITLE = vec![b' '; LONGLN as usize];
        let mut ANGLE: f64 = 0.0;
        let mut AVVS = ActualArray2D::<f64>::new(1..=3, 1..=MAXREC);
        let mut CMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut COVER = ActualArray::<f64>::new(LBCELL..=MAXCOV);
        let mut CLEND = ActualArray::<f64>::new(1..=MAXREC);
        let mut CLSTRT = ActualArray::<f64>::new(1..=MAXREC);
        let mut DC = StackArray::<f64, 2>::new(1..=2);
        let mut DLTANG: f64 = 0.0;
        let mut ENDS = ActualArray::<f64>::new(1..=MAXREC);
        let mut EPOCHS = ActualArray::<f64>::new(1..=MAXREC);
        let mut ET: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut INSETS = StackArray::<f64, 2>::new(1..=2);
        let mut IVLBDS = ActualArray::<f64>::new(1..=(MXMINI + 1));
        let mut LAST: f64 = 0.0;
        let mut PACKET = StackArray::<f64, 14>::new(1..=MAXPKT);
        let mut PKTS = ActualArray::<f64>::new(1..=(MAXREC * MAXPKT));
        let mut QUATS = ActualArray2D::<f64>::new(0..=3, 1..=MAXREC);
        let mut RATE: f64 = 0.0;
        let mut RATES = ActualArray::<f64>::new(1..=MXMINI);
        let mut STARTS = ActualArray::<f64>::new(1..=MAXREC);
        let mut T3END: f64 = 0.0;
        let mut TMPWIN = ActualArray::<f64>::new(LBCELL..=MAXCOV);
        let mut TOL: f64 = 0.0;
        let mut XAVINT = ActualArray2D::<f64>::new(LBCELL..=MAXCOV, 1..=NINS);
        let mut XAVSEG = ActualArray2D::<f64>::new(LBCELL..=MAXCOV, 1..=NINS);
        let mut XCVINT = ActualArray2D::<f64>::new(LBCELL..=MAXCOV, 1..=NINS);
        let mut XCVSEG = ActualArray2D::<f64>::new(LBCELL..=MAXCOV, 1..=NINS);
        let mut Z = StackArray::<f64, 3>::new(1..=3);
        let mut CLKID = StackArray::<i32, 6>::new(1..=NINS);
        let mut DEFSIZ: i32 = 0;
        let mut DEGREE: i32 = 0;
        let mut DEGRES = ActualArray::<i32>::new(1..=MXMINI);
        let mut INST = StackArray::<i32, 6>::new(1..=NINS);
        let mut HANDLE: i32 = 0;
        let mut IDS = StackArray::<i32, 13>::new(LBCELL..=(NINS + 1));
        let mut IVLN = StackArray::<i32, 6>::new(1..=NINS);
        let mut L: i32 = 0;
        let mut M: i32 = 0;
        let mut NIVREC: i32 = 0;
        let mut NMINI: i32 = 0;
        let mut NPKTS = ActualArray::<i32>::new(1..=MXMINI);
        let mut NSEG = StackArray::<i32, 6>::new(1..=NINS);
        let mut NR = StackArray::<i32, 6>::new(1..=NINS);
        let mut NREC: i32 = 0;
        let mut NSTART: i32 = 0;
        let mut PKTSIZ: i32 = 0;
        let mut SUBTPS = ActualArray::<i32>::new(1..=MXMINI);
        let mut SUBTYP: i32 = 0;
        let mut TIKPER = StackArray::<i32, 6>::new(1..=NINS);
        let mut XIDS = StackArray::<i32, 13>::new(LBCELL..=(NINS + 1));
        let mut XUNIT: i32 = 0;
        let mut SELLST: bool = false;
        let mut USEAV: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(-1000),
                Val::I(-2000),
                Val::I(-3000),
                Val::I(-4000),
                Val::I(-5000),
                Val::I(-6000),
            ]
            .into_iter();
            INST.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(3),
                Val::I(3),
                Val::I(4),
                Val::I(4),
                Val::I(4),
                Val::I(4),
            ]
            .into_iter();
            NSEG.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(4),
                Val::I(99),
                Val::I(199),
                Val::I(2399),
                Val::I(2399),
                Val::I(2399),
            ]
            .into_iter();
            NR.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(4),
                Val::I(3),
                Val::I(7),
                Val::I(20),
                Val::I(20),
                Val::I(20),
            ]
            .into_iter();
            IVLN.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(2),
                Val::I(4),
                Val::I(8),
                Val::I(16),
                Val::I(32),
                Val::I(32),
            ]
            .into_iter();
            TIKPER
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            Z.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            CVSTAT,
            DEFTXT,
            SEGID,
            TITLE,
            ANGLE,
            AVVS,
            CMAT,
            COVER,
            CLEND,
            CLSTRT,
            DC,
            DLTANG,
            ENDS,
            EPOCHS,
            ET,
            FIRST,
            INSETS,
            IVLBDS,
            LAST,
            PACKET,
            PKTS,
            QUATS,
            RATE,
            RATES,
            STARTS,
            T3END,
            TMPWIN,
            TOL,
            XAVINT,
            XAVSEG,
            XCVINT,
            XCVSEG,
            Z,
            CLKID,
            DEFSIZ,
            DEGREE,
            DEGRES,
            INST,
            HANDLE,
            IDS,
            IVLN,
            L,
            M,
            NIVREC,
            NMINI,
            NPKTS,
            NSEG,
            NR,
            NREC,
            NSTART,
            PKTSIZ,
            SUBTPS,
            SUBTYP,
            TIKPER,
            XIDS,
            XUNIT,
            SELLST,
            USEAV,
        }
    }
}

//$Procedure F_CKCOV ( CKCOV tests )
pub fn F_CKCOV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // MAXPKT is the size of the largest CK type 5 subtype's packet.
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
    testutil::TOPEN(b"F_CKCOV", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Setup:  create and load SCLK definitions for each instrument.",
        ctx,
    )?;

    for I in 1..=NINS {
        save.CLKID[I] = (save.INST[I] / 1000);

        fstr::assign(
            save.DEFTXT.get_mut(1),
            b"SCLK_KERNEL_ID         = ( @03-JAN-2005/02:03 )",
        );
        fstr::assign(save.DEFTXT.get_mut(2), b"SCLK_DATA_TYPE_#       = ( 1 )");
        fstr::assign(save.DEFTXT.get_mut(3), b"SCLK01_TIME_SYSTEM_#   = ( 2 )");
        fstr::assign(save.DEFTXT.get_mut(4), b"SCLK01_N_FIELDS_#      = ( 2 )");
        fstr::assign(
            save.DEFTXT.get_mut(5),
            b"SCLK01_MODULI_#        = ( 4294967296 **  )",
        );
        fstr::assign(
            save.DEFTXT.get_mut(6),
            b"SCLK01_OFFSETS_#       = ( 0          0   )",
        );
        fstr::assign(save.DEFTXT.get_mut(7), b"SCLK01_OUTPUT_DELIM_#  = ( 1 )");
        fstr::assign(save.DEFTXT.get_mut(8), b"SCLK_PARTITION_START_# = ( 0 )");
        fstr::assign(
            save.DEFTXT.get_mut(9),
            b"SCLK_PARTITION_END_#   = ( 1.0995116277750E+12 )",
        );
        fstr::assign(
            save.DEFTXT.get_mut(10),
            b"SCLK01_COEFFICIENTS_#  = ( 0  0  1 )",
        );

        save.DEFSIZ = 10;

        for J in 1..=save.DEFSIZ {
            spicelib::REPMI(
                &save.DEFTXT[J].to_vec(),
                b"#",
                -save.CLKID[I],
                &mut save.DEFTXT[J],
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Insert the modulus for the second SCLK field for the
        // current instrument.
        //
        spicelib::REPMI(
            &save.DEFTXT[5].to_vec(),
            b"**",
            save.TIKPER[I],
            &mut save.DEFTXT[5],
            ctx,
        );

        spicelib::LMPOOL(save.DEFTXT.as_arg(), save.DEFSIZ, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // We'll need a leapseconds kernel too.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup:  create CK file.", ctx)?;

    //
    // Create a CK file with data for five objects.
    //
    if spicelib::EXISTS(CK, ctx)? {
        spicelib::DELFIL(CK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(CK, CK, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize the expected coverage windows.
    //
    for I in 1..=NINS {
        spicelib::SSIZED(MAXCOV, save.XAVSEG.subarray_mut([LBCELL, I]), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SSIZED(MAXCOV, save.XAVINT.subarray_mut([LBCELL, I]), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SSIZED(MAXCOV, save.XCVSEG.subarray_mut([LBCELL, I]), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SSIZED(MAXCOV, save.XCVINT.subarray_mut([LBCELL, I]), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SSIZED(MAXCOV, save.TMPWIN.as_slice_mut(), ctx)?;

    //
    // Initializations to make compilers happy.
    //
    save.FIRST = 0.0;
    save.LAST = 0.0;
    save.T3END = 0.0;

    //
    // For each instrument, we'll create a sequence of segments. Because
    // we have CKCOV code (in some cases, the code resides in supporting
    // utilities) unique to each data type, we'll create segments of all
    // data types:  all of the segments for the Ith instrument will of
    // data type I.  Characteristics of the segments such as presence of
    // angular velocity, spacing of epochs and interpolation intervals,
    // spacing of segments, and time ordering of segments relative to
    // each other will vary.
    //
    for I in 1..=NINS {
        for J in 1..=save.NSEG[I] {
            //
            // Create segments for instrument I.  All segments for
            // instrument I will use data type I.
            //
            // Set the number of records in the Jth segment for instrument
            // I.
            //
            save.NREC = save.NR[J];

            //
            // Set the number of pointing records per interpolation
            // interval for instrument I.
            //
            save.NIVREC = save.IVLN[J];

            //
            // The flag USEAV indicates how the angular velocity flag
            // will be set.  Odd-indexed segments get angular velocity.
            //
            save.USEAV = spicelib::ODD(J);
            //
            // Proceed to create the Jth segment for instrument I.
            // The following code is data-type dependent.
            //
            if (I == 1) {
                //
                // This is the CK type 1 case.
                //
                // The segments we create will be separated by a 3 tick gap.
                // Records will be 3*J ticks apart.
                //
                // Set segment start and epochs.
                //
                if (J == 1) {
                    save.FIRST = 0.0;
                } else {
                    //
                    // LAST is left over from the previous J-loop iteration.
                    //
                    save.FIRST = (save.LAST + 3.0);
                }

                //
                // Set EPOCHS, QUATS, and AVVS.
                //
                // Pointing data are not relevant for these tests,
                // but having distinct entries could be helpful for
                // debugging.  The Kth entry will be a frame rotation
                // by K milliradians about the Z-axis.
                //
                for K in 1..=save.NREC {
                    //
                    // As stated above, records will be 3*J ticks apart.
                    //
                    save.EPOCHS[K] = (save.FIRST + (((3 * J) * (K - 1)) as f64));

                    // The angle required by AXISAR is the negative of
                    // the frame rotation angle.
                    //
                    save.DLTANG = 0.001;
                    save.ANGLE = -((K as f64) * save.DLTANG);

                    spicelib::AXISAR(save.Z.as_slice(), save.ANGLE, save.CMAT.as_slice_mut());
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::M2Q(save.CMAT.as_slice(), save.QUATS.subarray_mut([0, K]), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Set angular velocity to be consistent with
                    // the rotation data.  Remember angular velocity
                    // units are radians/sec, so we must multiply
                    // radians/tick by ticks/second for instrument I.
                    //
                    spicelib::VSCL(
                        (((save.TIKPER[I] as f64) * save.DLTANG) / (3 * J) as f64),
                        save.Z.as_slice(),
                        save.AVVS.subarray_mut([1, K]),
                    );
                }

                //
                // Set segment end time.
                //
                save.LAST = save.EPOCHS[save.NREC];

                //
                // Add the segment's coverage interval to our segment-level
                // expected coverage window for the Ith instrument.
                //
                spicelib::WNINSD(
                    save.FIRST,
                    save.LAST,
                    save.XCVSEG.subarray_mut([LBCELL, I]),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // If we're providing angular velocity for this segment,
                // then this segment contributes to the coverage window
                // for the angular-velocity only segments at the interval
                // level.
                //
                if save.USEAV {
                    spicelib::WNINSD(
                        save.FIRST,
                        save.LAST,
                        save.XAVSEG.subarray_mut([LBCELL, I]),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // The singleton intervals defined by the pointing epochs
                // act as interpolation intervals for type 1 segments.
                // Add the interpolation intervals to our interval-level
                // expected coverage window for the Ith instrument.
                //
                for K in 1..=save.NREC {
                    spicelib::WNINSD(
                        save.EPOCHS[K],
                        save.EPOCHS[K],
                        save.XCVINT.subarray_mut([LBCELL, I]),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // If we're providing angular velocity for this segment,
                // then this segment contributes to the coverage window
                // for the angular-velocity only segments at the interval
                // level.
                //
                if save.USEAV {
                    for K in 1..=save.NREC {
                        spicelib::WNINSD(
                            save.EPOCHS[K],
                            save.EPOCHS[K],
                            save.XAVINT.subarray_mut([LBCELL, I]),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }
                }

                //
                // Create segment ID.
                //
                fstr::assign(&mut save.SEGID, b"Segment # for instrument #.");

                spicelib::REPMI(&save.SEGID.to_vec(), b"#", J, &mut save.SEGID, ctx);
                spicelib::REPMI(&save.SEGID.to_vec(), b"#", I, &mut save.SEGID, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Write the current segment to our CK.
                //
                spicelib::CKW01(
                    save.HANDLE,
                    save.FIRST,
                    save.LAST,
                    save.INST[I],
                    b"J2000",
                    save.USEAV,
                    &save.SEGID,
                    save.NREC,
                    save.EPOCHS.as_slice(),
                    save.QUATS.as_slice(),
                    save.AVVS.as_slice(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            } else if (I == 2) {
                //
                // This is the CK type 2 case.
                //
                // For type 2, angular velocity is present by definition.
                //
                save.USEAV = true;

                //
                // We're going to copy the data for the type 1 case, but
                // here, the segments we create will abut each other.
                // Records will be 2*J ticks apart.
                //
                // Set segment start and epochs.
                //
                if (J == 1) {
                    save.FIRST = 0.0;
                } else {
                    //
                    // LAST is left over from the previous J-loop iteration.
                    //
                    save.FIRST = save.LAST;
                }

                //
                // Set EPOCHS, QUATS, and AVVS.
                //
                // Pointing data are not relevant for these tests,
                // but having distinct entries could be helpful for
                // debugging.  The Kth entry will be a frame rotation
                // by K milliradians about the Z-axis.
                //
                for K in 1..=save.NREC {
                    //
                    // As stated above, records will be 2*J ticks apart.
                    //
                    save.EPOCHS[K] = (save.FIRST + ((((K - 1) * J) * 2) as f64));

                    //
                    // Each interpolation interval will be 1 tick long.
                    //
                    save.ENDS[K] = (save.EPOCHS[K] + 1.0);

                    // The angle required by AXISAR is the negative of
                    // the frame rotation angle.
                    //
                    save.DLTANG = 0.001;
                    save.ANGLE = -((K as f64) * save.DLTANG);

                    spicelib::AXISAR(save.Z.as_slice(), save.ANGLE, save.CMAT.as_slice_mut());
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::M2Q(save.CMAT.as_slice(), save.QUATS.subarray_mut([0, K]), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Set angular velocity to be consistent with
                    // the rotation data.  Remember angular velocity
                    // units are radians/sec, so we must multiply
                    // radians/tick by ticks/second for instrument I.
                    //
                    spicelib::VSCL(
                        (((save.TIKPER[I] as f64) * save.DLTANG) / (2 * J) as f64),
                        save.Z.as_slice(),
                        save.AVVS.subarray_mut([1, K]),
                    );

                    //
                    // Set the clock rate in seconds per tick for the
                    // Kth interpolation interval.
                    //
                    save.RATES[K] = (1.0 / save.TIKPER[I] as f64);
                }

                //
                // Set segment end time.  Note that this is the end of
                // the last interpolation interval.
                //
                save.LAST = save.ENDS[save.NREC];

                //
                // Add the segment's coverage interval to our segment-level
                // expected coverage window for the Ith instrument.
                //
                spicelib::WNINSD(
                    save.FIRST,
                    save.LAST,
                    save.XCVSEG.subarray_mut([LBCELL, I]),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Since we're providing angular velocity for this segment,
                // then this segment contributes to the coverage window
                // for the angular-velocity only segments at the interval
                // level.
                //
                spicelib::WNINSD(
                    save.FIRST,
                    save.LAST,
                    save.XAVSEG.subarray_mut([LBCELL, I]),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Add the interpolation intervals to our interval-level
                // expected coverage window for the Ith instrument.
                //
                for K in 1..=save.NREC {
                    spicelib::WNINSD(
                        save.EPOCHS[K],
                        save.ENDS[K],
                        save.XCVINT.subarray_mut([LBCELL, I]),
                        ctx,
                    )?;
                    spicelib::WNINSD(
                        save.EPOCHS[K],
                        save.ENDS[K],
                        save.XAVINT.subarray_mut([LBCELL, I]),
                        ctx,
                    )?;
                }

                //
                // Create segment ID.
                //
                fstr::assign(&mut save.SEGID, b"Segment # for instrument #.");

                spicelib::REPMI(&save.SEGID.to_vec(), b"#", J, &mut save.SEGID, ctx);
                spicelib::REPMI(&save.SEGID.to_vec(), b"#", I, &mut save.SEGID, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Write the current segment to our CK.
                //
                spicelib::CKW02(
                    save.HANDLE,
                    save.FIRST,
                    save.LAST,
                    save.INST[I],
                    b"J2000",
                    &save.SEGID,
                    save.NREC,
                    save.EPOCHS.as_slice(),
                    save.ENDS.as_slice(),
                    save.QUATS.as_slice(),
                    save.AVVS.as_slice(),
                    save.RATES.as_slice(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;
            } else if (I == 3) {
                //
                // This is the CK type 3 case.
                //
                // The segments we create will be separated by a 3 tick gap.
                // Records will be J ticks apart.
                //
                // Set segment start and epochs.
                //
                if (J == 1) {
                    save.FIRST = 0.0;
                } else {
                    //
                    // LAST is left over from the previous J-loop iteration.
                    //
                    save.FIRST = (save.LAST + 3.0);
                }

                //
                // Set EPOCHS, QUATS, and AVVS.
                //
                // Pointing data are not relevant for these tests,
                // but having distinct entries could be helpful for
                // debugging.  The Kth entry will be a frame rotation
                // by K milliradians about the Z-axis.
                //
                for K in 1..=save.NREC {
                    //
                    // As stated above, records will be J ticks apart.
                    //
                    save.EPOCHS[K] = (save.FIRST + (((K - 1) * J) as f64));

                    // The angle required by AXISAR is the negative of
                    // the frame rotation angle.
                    //
                    save.DLTANG = 0.001;
                    save.ANGLE = -((K as f64) * save.DLTANG);

                    spicelib::AXISAR(save.Z.as_slice(), save.ANGLE, save.CMAT.as_slice_mut());
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::M2Q(save.CMAT.as_slice(), save.QUATS.subarray_mut([0, K]), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Set angular velocity to be consistent with
                    // the rotation data.  Remember angular velocity
                    // units are radians/sec, so we must multiply
                    // radians/tick by ticks/second for instrument I.
                    //
                    spicelib::VSCL(
                        (((save.TIKPER[I] as f64) * save.DLTANG) / J as f64),
                        save.Z.as_slice(),
                        save.AVVS.subarray_mut([1, K]),
                    );
                }

                //
                // Set segment end time.
                //
                save.LAST = save.EPOCHS[save.NREC];

                //
                // Add the segment's coverage interval to our segment-level
                // expected coverage window for the Ith instrument.
                //
                spicelib::WNINSD(
                    save.FIRST,
                    save.LAST,
                    save.XCVSEG.subarray_mut([LBCELL, I]),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // If we're providing angular velocity for this segment,
                // then this segment contributes to the coverage window
                // for the angular-velocity only segments at the interval
                // level.
                //
                if save.USEAV {
                    spicelib::WNINSD(
                        save.FIRST,
                        save.LAST,
                        save.XAVSEG.subarray_mut([LBCELL, I]),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // Set the interval start times.  The first epoch
                // is always the start of an interpolation interval
                // in these tests.  Each interval has length NIVREC
                // records.
                //
                save.L = 0;

                for K in intrinsics::range(1, save.NREC, save.NIVREC) {
                    //
                    // Increment the interpolation interval; set the
                    // start time.
                    //
                    save.L = (save.L + 1);

                    save.STARTS[save.L] = save.EPOCHS[K];

                    //
                    // Keep track of the interval end times.
                    //
                    if (save.L > 1) {
                        //
                        // Record the end time of the previous interval.
                        //
                        save.ENDS[(save.L - 1)] = save.EPOCHS[(K - 1)];
                    }
                }

                //
                // Set the interpolation interval count.
                //
                save.NSTART = save.L;

                //
                // The end time of the last interval is (in this test)
                // always the last epoch.
                //
                save.ENDS[save.NSTART] = save.EPOCHS[save.NREC];

                //
                // Add the interpolation intervals to our interval-level
                // expected coverage window for the Ith instrument.
                //
                for K in 1..=save.NSTART {
                    spicelib::WNINSD(
                        save.STARTS[K],
                        save.ENDS[K],
                        save.XCVINT.subarray_mut([LBCELL, I]),
                        ctx,
                    )?;
                }

                //
                // If we're providing angular velocity for this segment,
                // then this segment contributes to the coverage window
                // for the angular-velocity only segments at the interval
                // level.
                //
                if save.USEAV {
                    for K in 1..=save.NSTART {
                        spicelib::WNINSD(
                            save.STARTS[K],
                            save.ENDS[K],
                            save.XAVINT.subarray_mut([LBCELL, I]),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }
                }

                //
                // Create segment ID.
                //
                fstr::assign(&mut save.SEGID, b"Segment # for instrument #.");

                spicelib::REPMI(&save.SEGID.to_vec(), b"#", J, &mut save.SEGID, ctx);
                spicelib::REPMI(&save.SEGID.to_vec(), b"#", I, &mut save.SEGID, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Write the current segment to our CK.
                //
                spicelib::CKW03(
                    save.HANDLE,
                    save.FIRST,
                    save.LAST,
                    save.INST[I],
                    b"J2000",
                    save.USEAV,
                    &save.SEGID,
                    save.NREC,
                    save.EPOCHS.as_slice(),
                    save.QUATS.as_slice(),
                    save.AVVS.as_slice(),
                    save.NSTART,
                    save.STARTS.as_slice(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // If this is the last type 3 segment, save the end
                // time of the segment.
                //
                if (J == save.NSEG[3]) {
                    save.T3END = save.LAST;
                }
            //
            //
            //
            } else if (I == 4) {
                //
                // This is the CK type 4 case.
                //
                // The segments we create will be separated by a 3 tick gap.
                // Records will be J ticks apart.
                //
                // Set segment start and epochs.
                //
                if (J == 1) {
                    save.FIRST = 0.0;
                } else {
                    //
                    // LAST is left over from the previous J-loop iteration.
                    //
                    save.FIRST = (save.LAST + 3.0);
                }

                //
                // Create segment ID.
                //
                fstr::assign(&mut save.SEGID, b"Segment # for instrument #.");

                spicelib::REPMI(&save.SEGID.to_vec(), b"#", J, &mut save.SEGID, ctx);
                spicelib::REPMI(&save.SEGID.to_vec(), b"#", I, &mut save.SEGID, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Begin the segment.
                //
                spicelib::CKW04B(
                    save.HANDLE,
                    save.FIRST,
                    save.INST[I],
                    b"J2000",
                    save.USEAV,
                    &save.SEGID,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Define the start epochs for the packets.
                //
                for K in 1..=save.NREC {
                    //
                    // Packet starts will be 1000*J ticks apart.
                    //
                    save.EPOCHS[K] = (save.FIRST + ((((K - 1) * J) * 1000) as f64));
                }

                //
                // The segment end time matches the end time of the last
                // packet.
                //
                save.LAST = save.EPOCHS[save.NREC];

                //
                // Define the data packets for the current segment;
                // add each one to the segment.
                //
                for K in 1..=save.NREC {
                    //
                    // Fill in the current packet.  The packet structure
                    // is as follows:
                    //
                    //   ----------------------------------------------------
                    //   | The midpoint of the approximation interval       |
                    //   ----------------------------------------------------
                    //   | The radius of the approximation interval         |
                    //   ----------------------------------------------------
                    //   | Number of coefficients for q0                    |
                    //   ----------------------------------------------------
                    //   | Number of coefficients for q1                    |
                    //   ----------------------------------------------------
                    //   | Number of coefficients for q2                    |
                    //   ----------------------------------------------------
                    //   | Number of coefficients for q3                    |
                    //   ----------------------------------------------------
                    //   | Number of coefficients for AV1                   |
                    //   ----------------------------------------------------
                    //   | Number of coefficients for AV2                   |
                    //   ----------------------------------------------------
                    //   | Number of coefficients for AV3                   |
                    //   ----------------------------------------------------
                    //   | q0 Cheby coefficients                            |
                    //   ----------------------------------------------------
                    //   | q1 Cheby coefficients                            |
                    //   ----------------------------------------------------
                    //   | q2 Cheby coefficients                            |
                    //   ----------------------------------------------------
                    //   | q3 Cheby coefficients                            |
                    //   ----------------------------------------------------
                    //   | AV1 Cheby coefficients (optional)                |
                    //   ----------------------------------------------------
                    //   | AV2 Cheby coefficients (optional)                |
                    //   ----------------------------------------------------
                    //   | AV3 Cheby coefficients (optional)                |
                    //   ----------------------------------------------------
                    //
                    // The interval radius will be 499 ticks.  This will
                    // put the intervals two ticks apart.
                    //
                    // The interval midpoint will be at the start time
                    // plus 499 ticks.
                    //
                    save.PKTS[1] = (499.0 + save.EPOCHS[K]);
                    save.PKTS[2] = 499.0;
                    //
                    // Our quaternions will be constant.
                    //
                    save.PKTS[3] = 1 as f64;
                    save.PKTS[4] = 1 as f64;
                    save.PKTS[5] = 1 as f64;
                    save.PKTS[6] = 1 as f64;
                    //
                    // Angular velocity will be constant at 0.
                    //
                    save.PKTS[7] = 1 as f64;
                    save.PKTS[8] = 1 as f64;
                    save.PKTS[9] = 1 as f64;
                    //
                    // Cheby coefficients for the quaternion elements:
                    //
                    save.PKTS[10] = 1.0;
                    save.PKTS[11] = 2.0;
                    save.PKTS[12] = 3.0;
                    save.PKTS[13] = 4.0;
                    //
                    // Cheby coefficients for the angular velocity elements:
                    //
                    save.PKTS[14] = 0.0;
                    save.PKTS[15] = 0.0;
                    save.PKTS[16] = 0.0;

                    //
                    // The packet size depends on whether we're using
                    // angular velocity in this segment.
                    //
                    if save.USEAV {
                        save.PKTSIZ = 16;
                    } else {
                        save.PKTSIZ = 13;
                    }

                    //
                    // Add the current packet.
                    //
                    spicelib::CKW04A(
                        save.HANDLE,
                        1,
                        std::slice::from_mut(&mut save.PKTSIZ),
                        save.PKTS.as_slice_mut(),
                        save.EPOCHS.subarray(K),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Add the interpolation interval to our interval-level
                    // expected coverage window for the Ith instrument.
                    //
                    spicelib::WNINSD(
                        save.EPOCHS[K],
                        (save.EPOCHS[K] + ((2 as f64) * save.PKTS[2])),
                        save.XCVINT.subarray_mut([LBCELL, I]),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // If we're providing angular velocity for this segment,
                    // then this segment contributes to the coverage window
                    // for the angular-velocity only segments at the interval
                    // level.
                    //
                    if save.USEAV {
                        spicelib::WNINSD(
                            save.EPOCHS[K],
                            (save.EPOCHS[K] + ((2 as f64) * save.PKTS[2])),
                            save.XAVINT.subarray_mut([LBCELL, I]),
                            ctx,
                        )?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }
                }

                //
                // End the segment.
                //
                spicelib::CKW04E(save.HANDLE, save.LAST, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Add the segment's coverage interval to our segment-level
                // expected coverage window for the Ith instrument.
                //
                spicelib::WNINSD(
                    save.FIRST,
                    save.LAST,
                    save.XCVSEG.subarray_mut([LBCELL, I]),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // If we're providing angular velocity for this segment,
                // then this segment contributes to the coverage window
                // for the angular-velocity only segments at the interval
                // level.
                //
                if save.USEAV {
                    spicelib::WNINSD(
                        save.FIRST,
                        save.LAST,
                        save.XAVSEG.subarray_mut([LBCELL, I]),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }
            } else if (I == 5) {
                //
                // This is the CK type 5 case.
                //
                // Set type 5 subtype.  We expect NSEG(5) == 4.
                //
                if (save.NSEG[5] != 4) {
                    spicelib::SETMSG(b"Test cases for CK type 5 segments use a different type 5 subtype for each segment.  The Ith segment is mapped to subtype I-1.  Subtype numbers range from 0 to 3. NSEG(5) was expected to be 4 but was #.", ctx);
                    spicelib::ERRINT(b"#", save.NSEG[5], ctx);
                    spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                save.SUBTYP = (J - 1);

                //
                // Set packet size.
                //
                if (save.SUBTYP == 0) {
                    save.PKTSIZ = C05PS0;
                } else if (save.SUBTYP == 1) {
                    save.PKTSIZ = C05PS1;
                } else if (save.SUBTYP == 2) {
                    save.PKTSIZ = C05PS2;
                } else if (save.SUBTYP == 3) {
                    save.PKTSIZ = C05PS3;
                }

                // We'll mimic the construction of the type 3 segments,
                // but we'll put the segments in reverse time order
                // relative to each other.
                //
                // T3END is supposed to have been initialized before
                // we get here.
                //
                //
                // We'll use M as a complementary index with respect to
                // J and NSEG(I):
                //
                save.M = ((save.NSEG[I] + 1) - J);

                //
                // We must set NREC and USEAV specially for this
                // "backward" segment order.
                //
                save.NREC = save.NR[save.M];
                save.USEAV = spicelib::ODD(save.M);

                //
                // Set segment bounds insets:  except for the short
                // segment, the segment bounds will be *inside* the
                // coverage of the interpolation intervals.
                //
                if (save.NREC < 99) {
                    save.INSETS[1] = 0.0;
                    save.INSETS[2] = 0.0;
                } else {
                    save.INSETS[1] = (3 * save.M) as f64;
                    save.INSETS[2] = (5 * save.M) as f64;
                }

                //
                // So M will start at NSEG(I) and count down to 1.
                //
                // The segments we create will be separated by a 3 tick gap.
                // Records will be M ticks apart.
                //
                // Set segment end and epochs.
                //
                if (save.M == save.NSEG[I]) {
                    save.LAST = save.T3END;
                } else {
                    //
                    // FIRST is left over from the previous M-loop iteration.
                    //
                    save.LAST = (save.FIRST - 3.0);
                }

                //
                // Set EPOCHS, QUATS, and AVVS.
                //
                // Pointing data are not relevant for these tests,
                // but having distinct entries could be helpful for
                // debugging.  The Kth entry will be a frame rotation
                // by K milliradians about the Z-axis.
                //
                for K in intrinsics::range(save.NREC, 1, -1) {
                    //
                    // As stated above, records will be M ticks apart.
                    //
                    save.EPOCHS[K] = (save.LAST - ((save.M * (save.NREC - K)) as f64));

                    // The angle required by AXISAR is the negative of
                    // the frame rotation angle.
                    //
                    save.DLTANG = 0.001;
                    save.ANGLE = -((K as f64) * save.DLTANG);

                    spicelib::AXISAR(save.Z.as_slice(), save.ANGLE, save.CMAT.as_slice_mut());
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::M2Q(save.CMAT.as_slice(), save.QUATS.subarray_mut([0, K]), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Set angular velocity to be consistent with
                    // the rotation data.  Remember angular velocity
                    // units are radians/sec, so we must multiply
                    // radians/tick by ticks/second for instrument I.
                    //
                    spicelib::VSCL(
                        (((save.TIKPER[I] as f64) * save.DLTANG) / save.M as f64),
                        save.Z.as_slice(),
                        save.AVVS.subarray_mut([1, K]),
                    );

                    //
                    // Set packet contents.
                    //
                    spicelib::CLEARD(MAXPKT, save.PACKET.as_slice_mut());

                    if (save.SUBTYP == 0) {
                        //
                        // Packets contain quaternions and quaternion
                        // derivatives.  We'll set the derivatives to zero.
                        //
                        spicelib::MOVED(save.QUATS.subarray([0, K]), 4, save.PACKET.as_slice_mut());
                    } else if (save.SUBTYP == 1) {
                        //
                        // Packets contain quaternions only.
                        //
                        spicelib::MOVED(save.QUATS.subarray([0, K]), 4, save.PACKET.as_slice_mut());
                    } else if (save.SUBTYP == 2) {
                        //
                        // Packets contain quaternions, quaternion
                        // derivatives, angular velocity, and angular
                        // velocity derivatives.  We'll set the derivatives
                        // to zero (even though this makes the angular
                        // velocity and quaternion derivatives
                        // incompatible---subtype 2 is meant to handle
                        // this).
                        //
                        spicelib::MOVED(save.QUATS.subarray([0, K]), 4, save.PACKET.as_slice_mut());
                        spicelib::MOVED(save.AVVS.subarray([1, K]), 3, save.PACKET.subarray_mut(9));
                    } else if (save.SUBTYP == 3) {
                        //
                        // Packets contain quaternions and angular velocity.
                        //
                        spicelib::MOVED(save.QUATS.subarray([0, K]), 4, save.PACKET.as_slice_mut());
                        spicelib::MOVED(save.AVVS.subarray([1, K]), 3, save.PACKET.subarray_mut(5));
                    }

                    //
                    // Insert packet into packet array.
                    //
                    save.L = (1 + ((K - 1) * save.PKTSIZ));

                    spicelib::MOVED(
                        save.PACKET.as_slice(),
                        save.PKTSIZ,
                        save.PKTS.subarray_mut(save.L),
                    );
                }

                //
                // Set segment start time.
                //
                save.FIRST = save.EPOCHS[1];

                //
                // Add the segment's coverage interval to our segment-level
                // expected coverage window for the Ith instrument.
                //
                save.DC[1] = (save.FIRST + save.INSETS[1]);
                save.DC[2] = (save.LAST - save.INSETS[2]);

                spicelib::WNINSD(
                    save.DC[1],
                    save.DC[2],
                    save.XCVSEG.subarray_mut([LBCELL, I]),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // If we're providing angular velocity for this segment,
                // then this segment contributes to the coverage window
                // for the angular-velocity only segments at the interval
                // level.
                //
                if save.USEAV {
                    spicelib::WNINSD(
                        save.DC[1],
                        save.DC[2],
                        save.XAVSEG.subarray_mut([LBCELL, I]),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // Set the interval start times.  The first epoch
                // is always the start of an interpolation interval
                // in these tests.  Each interval has length NIVREC
                // records.
                //
                save.L = 0;

                for K in intrinsics::range(1, save.NREC, save.NIVREC) {
                    //
                    // Increment the interpolation interval; set the
                    // start time.
                    //
                    save.L = (save.L + 1);

                    save.STARTS[save.L] = save.EPOCHS[K];

                    //
                    // Keep track of the interval end times.
                    //
                    if (save.L > 1) {
                        //
                        // Record the end time of the previous interval.
                        //
                        save.ENDS[(save.L - 1)] = save.EPOCHS[(K - 1)];
                    }
                }

                //
                // Set the interpolation interval count.
                //
                save.NSTART = save.L;

                //
                // The end time of the last interval is (in this test)
                // always the last epoch.
                //
                save.ENDS[save.NSTART] = save.EPOCHS[save.NREC];

                //
                // Add the interpolation intervals to our interval-level
                // expected coverage window for the Ith instrument.
                //
                for K in 1..=save.NSTART {
                    //
                    // Adjust the interpolation intervals to account for
                    // the segment boundaries.
                    //
                    save.CLSTRT[K] = intrinsics::DMAX1(&[save.STARTS[K], save.DC[1]]);
                    save.CLEND[K] = intrinsics::DMIN1(&[save.ENDS[K], save.DC[2]]);

                    if (save.CLSTRT[K] <= save.CLEND[K]) {
                        spicelib::WNINSD(
                            save.CLSTRT[K],
                            save.CLEND[K],
                            save.XCVINT.subarray_mut([LBCELL, I]),
                            ctx,
                        )?;
                    }
                }

                //
                // If we're providing angular velocity for this segment,
                // then this segment contributes to the coverage window
                // for the angular-velocity only segments at the interval
                // level.
                //
                if save.USEAV {
                    for K in 1..=save.NSTART {
                        //
                        // The interpolation intervals that have been
                        // adjusted to account for the segment boundaries
                        // are contained in CLSTRT and CLEND.
                        //
                        if (save.CLSTRT[K] <= save.CLEND[K]) {
                            spicelib::WNINSD(
                                save.CLSTRT[K],
                                save.CLEND[K],
                                save.XAVINT.subarray_mut([LBCELL, I]),
                                ctx,
                            )?;

                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        }
                    }
                }

                //
                // Create segment ID.
                //
                fstr::assign(&mut save.SEGID, b"Segment # for instrument #.");

                spicelib::REPMI(&save.SEGID.to_vec(), b"#", J, &mut save.SEGID, ctx);
                spicelib::REPMI(&save.SEGID.to_vec(), b"#", I, &mut save.SEGID, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Write the current segment to our CK.  All interpolating
                // polynomials will be cubic.
                //
                save.DEGREE = 3;
                save.RATE = (1.0 / save.TIKPER[I] as f64);

                spicelib::CKW05(
                    save.HANDLE,
                    save.SUBTYP,
                    save.DEGREE,
                    save.DC[1],
                    save.DC[2],
                    save.INST[I],
                    b"J2000",
                    save.USEAV,
                    &save.SEGID,
                    save.NREC,
                    save.EPOCHS.as_slice(),
                    save.PKTS.as_slice(),
                    save.RATE,
                    save.NSTART,
                    save.STARTS.as_slice(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;
            } else if (I == 6) {
                //
                // This is the CK type 6 case.
                //
                // We'll use the "select last" option.
                //
                save.SELLST = true;
                //
                // Set type 6 subtype.  We expect NSEG(6) == 4.
                //
                if (save.NSEG[6] != 4) {
                    spicelib::SETMSG(b"Test cases for CK type 6 segments use a different type 6 subtype for each segment.  The Ith segment is mapped to subtype I-1.  Subtype numbers range from 0 to 3. NSEG(6) was expected to be 4 but was #.", ctx);
                    spicelib::ERRINT(b"#", save.NSEG[6], ctx);
                    spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                save.SUBTYP = (J - 1);

                //
                // Set packet size.
                //
                if (save.SUBTYP == 0) {
                    save.PKTSIZ = C06PS0;
                } else if (save.SUBTYP == 1) {
                    save.PKTSIZ = C06PS1;
                } else if (save.SUBTYP == 2) {
                    save.PKTSIZ = C06PS2;
                } else if (save.SUBTYP == 3) {
                    save.PKTSIZ = C06PS3;
                }

                //
                // We'll mimic the construction of the type 3 segments,
                // but we'll put the segments in reverse time order
                // relative to each other.
                //
                // T3END is supposed to have been initialized before
                // we get here.
                //
                // We'll use M as a complementary index with respect to
                // J and NSEG(I):
                //
                save.M = ((save.NSEG[I] + 1) - J);

                //
                // We must set NREC and USEAV specially for this
                // "backward" segment order.
                //
                save.NREC = save.NR[save.M];

                save.USEAV = spicelib::ODD(save.M);

                //
                // Set segment bounds insets:  except for the short
                // segment, the segment bounds will be *inside* the
                // coverage of the interpolation intervals.
                //
                if (save.NREC < 99) {
                    save.INSETS[1] = 0.0;
                    save.INSETS[2] = 0.0;
                } else {
                    save.INSETS[1] = (3 * save.M) as f64;
                    save.INSETS[2] = (5 * save.M) as f64;
                }

                //
                // So M will start at NSEG(I) and count down to 1.
                //
                // The segments we create will be separated by a 3 tick gap.
                // Records will be M ticks apart.
                //
                // Set segment end and epochs.
                //
                if (save.M == save.NSEG[I]) {
                    save.LAST = save.T3END;
                } else {
                    //
                    // FIRST is left over from the previous M-loop iteration.
                    //
                    save.LAST = (save.FIRST - 3.0);
                }

                //
                // Set EPOCHS, QUATS, and AVVS.
                //
                // Pointing data are not relevant for these tests,
                // but having distinct entries could be helpful for
                // debugging.  The Kth entry will be a frame rotation
                // by K milliradians about the Z-axis.
                //
                for K in intrinsics::range(save.NREC, 1, -1) {
                    //
                    // As stated above, records will be M ticks apart.
                    //
                    save.EPOCHS[K] = (save.LAST - ((save.M * (save.NREC - K)) as f64));

                    // The angle required by AXISAR is the negative of
                    // the frame rotation angle.
                    //
                    save.DLTANG = 0.001;
                    save.ANGLE = -((K as f64) * save.DLTANG);

                    spicelib::AXISAR(save.Z.as_slice(), save.ANGLE, save.CMAT.as_slice_mut());
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::M2Q(save.CMAT.as_slice(), save.QUATS.subarray_mut([0, K]), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Set angular velocity to be consistent with
                    // the rotation data.  Remember angular velocity
                    // units are radians/sec, so we must multiply
                    // radians/tick by ticks/second for instrument I.
                    //
                    spicelib::VSCL(
                        (((save.TIKPER[I] as f64) * save.DLTANG) / save.M as f64),
                        save.Z.as_slice(),
                        save.AVVS.subarray_mut([1, K]),
                    );

                    //
                    // Set packet contents.
                    //
                    spicelib::CLEARD(MAXPKT, save.PACKET.as_slice_mut());

                    if (save.SUBTYP == 0) {
                        //
                        // Packets contain quaternions and quaternion
                        // derivatives.  We'll set the derivatives to zero.
                        //
                        spicelib::MOVED(save.QUATS.subarray([0, K]), 4, save.PACKET.as_slice_mut());
                    } else if (save.SUBTYP == 1) {
                        //
                        // Packets contain quaternions only.
                        //
                        spicelib::MOVED(save.QUATS.subarray([0, K]), 4, save.PACKET.as_slice_mut());
                    } else if (save.SUBTYP == 2) {
                        //
                        // Packets contain quaternions, quaternion
                        // derivatives, angular velocity, and angular
                        // velocity derivatives.  We'll set the derivatives
                        // to zero (even though this makes the angular
                        // velocity and quaternion derivatives
                        // incompatible---subtype 2 is meant to handle
                        // this).
                        //
                        spicelib::MOVED(save.QUATS.subarray([0, K]), 4, save.PACKET.as_slice_mut());
                        spicelib::MOVED(save.AVVS.subarray([1, K]), 3, save.PACKET.subarray_mut(9));
                    } else if (save.SUBTYP == 3) {
                        //
                        // Packets contain quaternions and angular velocity.
                        //
                        spicelib::MOVED(save.QUATS.subarray([0, K]), 4, save.PACKET.as_slice_mut());
                        spicelib::MOVED(save.AVVS.subarray([1, K]), 3, save.PACKET.subarray_mut(5));
                    }

                    //
                    // Insert packet into packet array.
                    //
                    save.L = (1 + ((K - 1) * save.PKTSIZ));

                    spicelib::MOVED(
                        save.PACKET.as_slice(),
                        save.PKTSIZ,
                        save.PKTS.subarray_mut(save.L),
                    );
                }

                //
                // Set segment start time.
                //
                save.FIRST = save.EPOCHS[1];

                //
                // Add the segment's coverage interval to our segment-level
                // expected coverage window for the Ith instrument.
                //
                save.DC[1] = (save.FIRST + save.INSETS[1]);
                save.DC[2] = (save.LAST - save.INSETS[2]);

                spicelib::WNINSD(
                    save.DC[1],
                    save.DC[2],
                    save.XCVSEG.subarray_mut([LBCELL, I]),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // If we're providing angular velocity for this segment,
                // then this segment contributes to the coverage window
                // for the angular-velocity only segments at the interval
                // level.
                //
                if save.USEAV {
                    spicelib::WNINSD(
                        save.DC[1],
                        save.DC[2],
                        save.XAVSEG.subarray_mut([LBCELL, I]),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // Set the interval start times.  The first epoch
                // is always the start of an interpolation interval
                // in these tests.  Each interval but the last
                // one in the segment has length NIVREC records.
                // NMINI counts the number of interpolation intervals
                // (equal to the number of mini-segments) in the
                // segment.
                //
                // Each interval has a gap at the end. The gap duration
                // is the (constant) separation between epochs, which
                // has been set to M ticks.
                //
                save.L = 0;

                //
                // Terminate this loop before the last record, so we don't
                // start an interpolation interval at that record.
                //
                for K in intrinsics::range(1, (save.NREC - 1), save.NIVREC) {
                    //
                    // Increment the interpolation interval; set the
                    // start time.
                    //
                    save.L = (save.L + 1);

                    save.IVLBDS[save.L] = save.EPOCHS[K];
                    save.STARTS[save.L] = save.IVLBDS[save.L];

                    //
                    // Keep track of the interval end times.
                    //
                    if (save.L > 1) {
                        //
                        // Record the "end" time of the previous interval.
                        // The end time in this context is the last epoch,
                        // not the right hand interval boundary. The end time
                        // represents the upper bound of the interval over
                        // which data are available. The interval upper bound
                        // coincides with the upper end of a coverage gap.
                        //
                        save.ENDS[(save.L - 1)] = save.EPOCHS[(K - 1)];
                    }
                    //
                    // Set the other interval-related parameters.
                    //
                    // All interpolating polynomials will be cubic.
                    //
                    save.DEGREE = 3;
                    save.RATE = (1.0 / save.TIKPER[I] as f64);

                    save.NPKTS[save.L] = intrinsics::MIN0(&[save.NIVREC, ((save.NREC - K) + 1)]);
                    save.RATES[save.L] = save.RATE;
                    save.SUBTPS[save.L] = save.SUBTYP;
                    save.DEGRES[save.L] = save.DEGREE;
                }

                //
                // Set the interpolation interval count.
                //
                save.NMINI = save.L;
                save.NSTART = save.NMINI;
                //
                // The last epoch of the last interval is (in this test)
                // the last epoch; the interval upper bound is beyond that
                // epoch, since the interval has a gap.
                //
                save.ENDS[save.NMINI] = save.EPOCHS[save.NREC];

                save.IVLBDS[(save.NMINI + 1)] = (save.EPOCHS[save.NREC] + save.M as f64);

                //
                // Add the interpolation intervals to our interval-level
                // expected coverage window for the Ith instrument.
                //
                for K in 1..=save.NSTART {
                    //
                    // Adjust the interpolation intervals to account for
                    // the segment boundaries.
                    //
                    save.CLSTRT[K] = intrinsics::DMAX1(&[save.STARTS[K], save.DC[1]]);
                    save.CLEND[K] = intrinsics::DMIN1(&[save.ENDS[K], save.DC[2]]);

                    if (save.CLSTRT[K] <= save.CLEND[K]) {
                        spicelib::WNINSD(
                            save.CLSTRT[K],
                            save.CLEND[K],
                            save.XCVINT.subarray_mut([LBCELL, I]),
                            ctx,
                        )?;
                    }
                }

                //
                // If we're providing angular velocity for this segment,
                // then this segment contributes to the coverage window
                // for the angular-velocity only segments at the interval
                // level.
                //
                if save.USEAV {
                    for K in 1..=save.NSTART {
                        //
                        // The interpolation intervals that have been
                        // adjusted to account for the segment boundaries
                        // are contained in CLSTRT and CLEND.
                        //
                        if (save.CLSTRT[K] <= save.CLEND[K]) {
                            spicelib::WNINSD(
                                save.CLSTRT[K],
                                save.CLEND[K],
                                save.XAVINT.subarray_mut([LBCELL, I]),
                                ctx,
                            )?;

                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        }
                    }
                }

                //
                // Create segment ID.
                //
                fstr::assign(&mut save.SEGID, b"Segment # for instrument #.");

                spicelib::REPMI(&save.SEGID.to_vec(), b"#", J, &mut save.SEGID, ctx);
                spicelib::REPMI(&save.SEGID.to_vec(), b"#", I, &mut save.SEGID, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Write the current segment to our CK.
                //
                spicelib::CKW06(
                    save.HANDLE,
                    save.INST[I],
                    b"J2000",
                    save.USEAV,
                    save.DC[1],
                    save.DC[2],
                    &save.SEGID,
                    save.NMINI,
                    save.NPKTS.as_slice(),
                    save.SUBTPS.as_slice(),
                    save.DEGRES.as_slice(),
                    save.PKTS.as_slice(),
                    save.RATES.as_slice(),
                    save.EPOCHS.as_slice(),
                    save.IVLBDS.as_slice(),
                    save.SELLST,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            } else {
                //
                // Oops.
                //
                spicelib::SETMSG(b"Unexpected data type: #", ctx);
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }
        }
    }

    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Call DAFCLS as a last resort, since CKCLS may fail to
    // close the file.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // ******************************************************
    // ******************************************************
    // ******************************************************
    //     CKCOV tests
    // ******************************************************
    // ******************************************************
    // ******************************************************
    //
    //     We've written the CK.  It's time to check out CKCOV.
    //
    //
    //     Check actual vs expected coverage as we vary the input
    //     arguments to CKCOV.
    //
    //
    //     Each test we do will be performed with both an empty
    //     and non-empty input coverage window.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 2;
        let m3__: i32 = 1;
        save.L = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // We'll start out by testing the coverage summary at the
            // segment level.
            //
            if (save.L == 1) {
                //
                // We'll set COVER to be empty on input to CKCOV.
                //
                spicelib::SCARDD(0, save.COVER.as_slice_mut(), ctx)?;

                fstr::assign(&mut save.CVSTAT, b"empty");
            } else {
                fstr::assign(&mut save.CVSTAT, b"non-empty");
            }

            for I in 1..=NINS {
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut save.TITLE, b"Check segment-level coverage for instrument #; COVER starts out #. Angular velocity not needed. TOL = 0.D0.");

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CVSTAT, &mut save.TITLE);
                testutil::TCASE(&save.TITLE, ctx)?;

                //
                // Initialize COVER.
                //
                spicelib::SSIZED(MAXCOV, save.COVER.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Make a copy of the expected window.
                //
                spicelib::COPYD(
                    save.XCVSEG.subarray([LBCELL, I]),
                    save.TMPWIN.as_slice_mut(),
                    ctx,
                )?;

                if (save.L == 2) {
                    //
                    // Insert an interval into COVER.  This same interval
                    // must be added to each window containing expected
                    // coverage.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.COVER.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // The same interval is expected to appear in the output.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                spicelib::CKCOV(
                    CK,
                    save.INST[I],
                    false,
                    b"SEGMENT",
                    0.0,
                    b"SCLK",
                    save.COVER.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Check cardinality of coverage window.
                //
                testutil::CHCKSI(
                    b"CARDD(COVER)",
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    b"=",
                    spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                    0,
                    OK,
                    ctx,
                )?;

                //
                // Check coverage window.
                //
                testutil::CHCKAD(
                    b"COVER",
                    save.COVER.subarray(1),
                    b"=",
                    save.TMPWIN.subarray(1),
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    0.0,
                    OK,
                    ctx,
                )?;

                if (I == 6) {
                    spicelib::SCARDD(0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if (save.L == 2) {
                        //
                        // The same interval is expected to appear in the output.
                        //
                        spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Compare CKCOV results against those from T_CKCOV.
                    //
                    T_CKCOV(
                        CK,
                        save.INST[I],
                        false,
                        b"SEGMENT",
                        0.0,
                        b"SCLK",
                        save.TMPWIN.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check cardinality of coverage window.
                    //
                    testutil::CHCKSI(
                        b"CARDD(COVER)",
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        b"=",
                        spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                        0,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check coverage window.
                    //
                    testutil::CHCKAD(
                        b"COVER",
                        save.COVER.subarray(1),
                        b"=",
                        save.TMPWIN.subarray(1),
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        0.0,
                        OK,
                        ctx,
                    )?;
                }
            }

            //
            // --- Case: ------------------------------------------------------
            //
            for I in 1..=NINS {
                fstr::assign(&mut save.TITLE, b"INST: #;  LEVEL: SEGMENT;  NEEDAV: TRUE; TIMSYS: SCLK; TOL: 0.D0; COVER starts out #.");

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CVSTAT, &mut save.TITLE);
                testutil::TCASE(&save.TITLE, ctx)?;

                //
                // Initialize COVER.
                //
                spicelib::SSIZED(MAXCOV, save.COVER.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Make a copy of the expected window.
                //
                spicelib::COPYD(
                    save.XAVSEG.subarray([LBCELL, I]),
                    save.TMPWIN.as_slice_mut(),
                    ctx,
                )?;

                if (save.L == 2) {
                    //
                    // Insert an interval into COVER.  This same interval
                    // must be added to each window containing expected
                    // coverage.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.COVER.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // The same interval is expected to appear in the output.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                spicelib::CKCOV(
                    CK,
                    save.INST[I],
                    true,
                    b"SEGMENT",
                    0.0,
                    b"SCLK",
                    save.COVER.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Check cardinality of coverage window.
                //
                testutil::CHCKSI(
                    b"CARDD(COVER)",
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    b"=",
                    spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                    0,
                    OK,
                    ctx,
                )?;

                //
                // Check coverage window.
                //
                testutil::CHCKAD(
                    b"COVER",
                    save.COVER.subarray(1),
                    b"=",
                    save.TMPWIN.subarray(1),
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    0.0,
                    OK,
                    ctx,
                )?;

                if (I == 6) {
                    spicelib::SCARDD(0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if (save.L == 2) {
                        //
                        // The same interval is expected to appear in the output.
                        //
                        spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Compare CKCOV results against those from T_CKCOV.
                    //
                    T_CKCOV(
                        CK,
                        save.INST[I],
                        true,
                        b"SEGMENT",
                        0.0,
                        b"SCLK",
                        save.TMPWIN.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check cardinality of coverage window.
                    //
                    testutil::CHCKSI(
                        b"CARDD(COVER)",
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        b"=",
                        spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                        0,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check coverage window.
                    //
                    testutil::CHCKAD(
                        b"COVER",
                        save.COVER.subarray(1),
                        b"=",
                        save.TMPWIN.subarray(1),
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        0.0,
                        OK,
                        ctx,
                    )?;
                }
            }

            //
            // --- Case: ------------------------------------------------------
            //
            for I in 1..=NINS {
                fstr::assign(&mut save.TITLE, b"INST: #;  LEVEL: SEGMENT;  NEEDAV: FALSE; TIMSYS: SCLK; TOL: 1.D0; COVER starts out #.");

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CVSTAT, &mut save.TITLE);
                testutil::TCASE(&save.TITLE, ctx)?;

                //
                // Initialize COVER.
                //
                spicelib::SSIZED(MAXCOV, save.COVER.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust our expected result window by TOL.
                //
                spicelib::COPYD(
                    save.XCVSEG.subarray([LBCELL, I]),
                    save.TMPWIN.as_slice_mut(),
                    ctx,
                )?;

                save.TOL = 1.0;

                spicelib::WNEXPD(save.TOL, save.TOL, save.TMPWIN.as_slice_mut(), ctx)?;
                //
                // Make sure the window doesn't start with a negative tick
                // value.
                //
                save.TMPWIN[1] = intrinsics::DMAX1(&[save.TMPWIN[1], 0.0]);

                if (save.L == 2) {
                    //
                    // Insert an interval into COVER.  This same interval
                    // must be added to each window containing expected
                    // coverage.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.COVER.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // The same interval is expected to appear in the output.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                spicelib::CKCOV(
                    CK,
                    save.INST[I],
                    false,
                    b"SEGMENT",
                    save.TOL,
                    b"SCLK",
                    save.COVER.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Check cardinality of coverage window.
                //
                testutil::CHCKSI(
                    b"CARDD(COVER)",
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    b"=",
                    spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                    0,
                    OK,
                    ctx,
                )?;

                //
                // Check coverage window.
                //
                testutil::CHCKAD(
                    b"COVER",
                    save.COVER.subarray(1),
                    b"=",
                    save.TMPWIN.subarray(1),
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    0.0,
                    OK,
                    ctx,
                )?;

                if (I == 6) {
                    spicelib::SCARDD(0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if (save.L == 2) {
                        //
                        // The same interval is expected to appear in the output.
                        //
                        spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Compare CKCOV results against those from T_CKCOV.
                    //
                    T_CKCOV(
                        CK,
                        save.INST[I],
                        false,
                        b"SEGMENT",
                        save.TOL,
                        b"SCLK",
                        save.TMPWIN.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check cardinality of coverage window.
                    //
                    testutil::CHCKSI(
                        b"CARDD(COVER)",
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        b"=",
                        spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                        0,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check coverage window.
                    //
                    testutil::CHCKAD(
                        b"COVER",
                        save.COVER.subarray(1),
                        b"=",
                        save.TMPWIN.subarray(1),
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        0.0,
                        OK,
                        ctx,
                    )?;
                }
            }

            for I in 1..=NINS {
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut save.TITLE,
                    b"INST: #;  LEVEL: SEGMENT;  NEEDAV: FALSE; TIMSYS: TDB; TOL: 0.D0; COVER: #.",
                );

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CVSTAT, &mut save.TITLE);
                testutil::TCASE(&save.TITLE, ctx)?;

                //
                // Initialize COVER.
                //
                spicelib::SSIZED(MAXCOV, save.COVER.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Make a copy of the expected window.
                //
                spicelib::COPYD(
                    save.XCVSEG.subarray([LBCELL, I]),
                    save.TMPWIN.as_slice_mut(),
                    ctx,
                )?;

                //
                // Convert the expected window to TDB.
                //
                for J in 1..=spicelib::CARDD(save.TMPWIN.as_slice(), ctx)? {
                    spicelib::SCT2E(save.CLKID[I], save.TMPWIN[J], &mut save.ET, ctx)?;
                    save.TMPWIN[J] = save.ET;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                if (save.L == 2) {
                    //
                    // Insert an interval into COVER.  This same interval
                    // must be added to each window containing expected
                    // coverage.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.COVER.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // The same interval is expected to appear in the output.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                spicelib::CKCOV(
                    CK,
                    save.INST[I],
                    false,
                    b"SEGMENT",
                    0.0,
                    b"TDB",
                    save.COVER.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check cardinality of coverage window.
                //
                testutil::CHCKSI(
                    b"CARDD(COVER)",
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    b"=",
                    spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                    0,
                    OK,
                    ctx,
                )?;

                //
                // Check coverage window.
                //
                testutil::CHCKAD(
                    b"COVER",
                    save.COVER.subarray(1),
                    b"=",
                    save.TMPWIN.subarray(1),
                    spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                    0.0,
                    OK,
                    ctx,
                )?;

                if (I == 6) {
                    spicelib::SCARDD(0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if (save.L == 2) {
                        //
                        // The same interval is expected to appear in the output.
                        //
                        spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Compare CKCOV results against those from T_CKCOV.
                    //
                    T_CKCOV(
                        CK,
                        save.INST[I],
                        false,
                        b"SEGMENT",
                        0.0,
                        b"TDB",
                        save.TMPWIN.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check cardinality of coverage window.
                    //
                    testutil::CHCKSI(
                        b"CARDD(COVER)",
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        b"=",
                        spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                        0,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check coverage window.
                    //
                    testutil::CHCKAD(
                        b"COVER",
                        save.COVER.subarray(1),
                        b"=",
                        save.TMPWIN.subarray(1),
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        0.0,
                        OK,
                        ctx,
                    )?;
                }
            }

            for I in 1..=NINS {
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut save.TITLE,
                    b"INST: #;  LEVEL: SEGMENT;  NEEDAV: FALSE; TIMSYS: TDB; TOL: 1.D0; COVER: #.",
                );

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CVSTAT, &mut save.TITLE);
                testutil::TCASE(&save.TITLE, ctx)?;

                //
                // Initialize COVER.
                //
                spicelib::SSIZED(MAXCOV, save.COVER.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust our expected result window by TOL.
                //
                spicelib::COPYD(
                    save.XCVSEG.subarray([LBCELL, I]),
                    save.TMPWIN.as_slice_mut(),
                    ctx,
                )?;

                save.TOL = 1.0;

                spicelib::WNEXPD(save.TOL, save.TOL, save.TMPWIN.as_slice_mut(), ctx)?;

                //
                // Make sure the window doesn't start with a negative tick
                // value.
                //
                save.TMPWIN[1] = intrinsics::DMAX1(&[save.TMPWIN[1], 0.0]);

                //
                // Convert the expected window to TDB.
                //
                for J in 1..=spicelib::CARDD(save.TMPWIN.as_slice(), ctx)? {
                    spicelib::SCT2E(save.CLKID[I], save.TMPWIN[J], &mut save.ET, ctx)?;
                    save.TMPWIN[J] = save.ET;
                }

                if (save.L == 2) {
                    //
                    // Insert an interval into COVER.  This same interval
                    // must be added to each window containing expected
                    // coverage.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.COVER.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // The same interval is expected to appear in the output.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                spicelib::CKCOV(
                    CK,
                    save.INST[I],
                    false,
                    b"SEGMENT",
                    1.0,
                    b"TDB",
                    save.COVER.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check cardinality of coverage window.
                //
                testutil::CHCKSI(
                    b"CARDD(COVER)",
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    b"=",
                    spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                    0,
                    OK,
                    ctx,
                )?;

                //
                // Check coverage window.
                //
                testutil::CHCKAD(
                    b"COVER",
                    save.COVER.subarray(1),
                    b"=",
                    save.TMPWIN.subarray(1),
                    spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                    0.0,
                    OK,
                    ctx,
                )?;

                if (I == 6) {
                    spicelib::SCARDD(0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if (save.L == 2) {
                        //
                        // The same interval is expected to appear in the output.
                        //
                        spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Compare CKCOV results against those from T_CKCOV.
                    //
                    T_CKCOV(
                        CK,
                        save.INST[I],
                        false,
                        b"SEGMENT",
                        1.0,
                        b"TDB",
                        save.TMPWIN.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check cardinality of coverage window.
                    //
                    testutil::CHCKSI(
                        b"CARDD(COVER)",
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        b"=",
                        spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                        0,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check coverage window.
                    //
                    testutil::CHCKAD(
                        b"COVER",
                        save.COVER.subarray(1),
                        b"=",
                        save.TMPWIN.subarray(1),
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        0.0,
                        OK,
                        ctx,
                    )?;
                }
            }

            //
            // INTERVAL level tests:
            //
            //
            // Now we'll repeat the previous tests, but this time the
            // coverage will be summarized at the interval level.
            //

            for I in 1..=NINS {
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut save.TITLE,
                    b"INST: #;  LEVEL: INTERVAL;  NEEDAV: FALSE; TIMSYS: SCLK TOL: 0.D0; COVER: #.",
                );

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CVSTAT, &mut save.TITLE);
                testutil::TCASE(&save.TITLE, ctx)?;

                //
                // Initialize COVER.
                //
                spicelib::SSIZED(MAXCOV, save.COVER.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Make a copy of the expected window.
                //
                spicelib::COPYD(
                    save.XCVINT.subarray([LBCELL, I]),
                    save.TMPWIN.as_slice_mut(),
                    ctx,
                )?;

                if (save.L == 2) {
                    //
                    // Insert an interval into COVER.  This same interval
                    // must be added to each window containing expected
                    // coverage.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.COVER.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // The same interval is expected to appear in the output.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                spicelib::CKCOV(
                    CK,
                    save.INST[I],
                    false,
                    b"INTERVAL",
                    0.0,
                    b"SCLK",
                    save.COVER.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check cardinality of coverage window.
                //
                testutil::CHCKSI(
                    b"CARDD(COVER)",
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    b"=",
                    spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                    0,
                    OK,
                    ctx,
                )?;

                //
                // Check coverage window.
                //
                testutil::CHCKAD(
                    b"COVER",
                    save.COVER.subarray(1),
                    b"=",
                    save.TMPWIN.subarray(1),
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    0.0,
                    OK,
                    ctx,
                )?;

                if (I == 6) {
                    spicelib::SCARDD(0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if (save.L == 2) {
                        //
                        // The same interval is expected to appear in the output.
                        //
                        spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Compare CKCOV results against those from T_CKCOV.
                    //
                    T_CKCOV(
                        CK,
                        save.INST[I],
                        false,
                        b"INTERVAL",
                        0.0,
                        b"SCLK",
                        save.TMPWIN.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check cardinality of coverage window.
                    //
                    testutil::CHCKSI(
                        b"CARDD(COVER)",
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        b"=",
                        spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                        0,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check coverage window.
                    //
                    testutil::CHCKAD(
                        b"COVER",
                        save.COVER.subarray(1),
                        b"=",
                        save.TMPWIN.subarray(1),
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        0.0,
                        OK,
                        ctx,
                    )?;
                }
            }

            //
            // --- Case: ------------------------------------------------------
            //
            for I in 1..=NINS {
                fstr::assign(
                    &mut save.TITLE,
                    b"INST: #;  LEVEL: INTERVAL;  NEEDAV: TRUE; TIMSYS: SCLK TOL: 0.D0; COVER: #.",
                );

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CVSTAT, &mut save.TITLE);
                testutil::TCASE(&save.TITLE, ctx)?;

                //
                // Initialize COVER.
                //
                spicelib::SSIZED(MAXCOV, save.COVER.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Make a copy of the expected window.
                //
                spicelib::COPYD(
                    save.XAVINT.subarray([LBCELL, I]),
                    save.TMPWIN.as_slice_mut(),
                    ctx,
                )?;

                if (save.L == 2) {
                    //
                    // Insert an interval into COVER.  This same interval
                    // must be added to each window containing expected
                    // coverage.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.COVER.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // The same interval is expected to appear in the output.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                spicelib::CKCOV(
                    CK,
                    save.INST[I],
                    true,
                    b"INTERVAL",
                    0.0,
                    b"SCLK",
                    save.COVER.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Check cardinality of coverage window.
                //
                testutil::CHCKSI(
                    b"CARDD(COVER)",
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    b"=",
                    spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                    0,
                    OK,
                    ctx,
                )?;

                //
                // Check coverage window.
                //
                testutil::CHCKAD(
                    b"COVER",
                    save.COVER.subarray(1),
                    b"=",
                    save.TMPWIN.subarray(1),
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    0.0,
                    OK,
                    ctx,
                )?;

                if (I == 6) {
                    spicelib::SCARDD(0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if (save.L == 2) {
                        //
                        // The same interval is expected to appear in the output.
                        //
                        spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Compare CKCOV results against those from T_CKCOV.
                    //
                    T_CKCOV(
                        CK,
                        save.INST[I],
                        true,
                        b"INTERVAL",
                        0.0,
                        b"SCLK",
                        save.TMPWIN.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check cardinality of coverage window.
                    //
                    testutil::CHCKSI(
                        b"CARDD(COVER)",
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        b"=",
                        spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                        0,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check coverage window.
                    //
                    testutil::CHCKAD(
                        b"COVER",
                        save.COVER.subarray(1),
                        b"=",
                        save.TMPWIN.subarray(1),
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        0.0,
                        OK,
                        ctx,
                    )?;
                }
            }

            //
            // --- Case: ------------------------------------------------------
            //
            for I in 1..=NINS {
                fstr::assign(
                    &mut save.TITLE,
                    b"INST: #;  LEVEL: INTERVAL;  NEEDAV: FALSE; TIMSYS: SCLK TOL: 1.D0; COVER: #.",
                );

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CVSTAT, &mut save.TITLE);
                testutil::TCASE(&save.TITLE, ctx)?;

                //
                // Initialize COVER.
                //
                spicelib::SSIZED(MAXCOV, save.COVER.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust our expected result window by TOL.
                //
                spicelib::COPYD(
                    save.XCVINT.subarray([LBCELL, I]),
                    save.TMPWIN.as_slice_mut(),
                    ctx,
                )?;

                save.TOL = 1.0;

                spicelib::WNEXPD(save.TOL, save.TOL, save.TMPWIN.as_slice_mut(), ctx)?;

                //
                // Make sure the window doesn't start with a negative tick
                // value.
                //
                save.TMPWIN[1] = intrinsics::DMAX1(&[save.TMPWIN[1], 0.0]);

                if (save.L == 2) {
                    //
                    // Insert an interval into COVER.  This same interval
                    // must be added to each window containing expected
                    // coverage.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.COVER.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // The same interval is expected to appear in the output.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                spicelib::CKCOV(
                    CK,
                    save.INST[I],
                    false,
                    b"INTERVAL",
                    save.TOL,
                    b"SCLK",
                    save.COVER.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Check cardinality of coverage window.
                //
                testutil::CHCKSI(
                    b"CARDD(COVER)",
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    b"=",
                    spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                    0,
                    OK,
                    ctx,
                )?;

                //
                // Check coverage window.
                //
                testutil::CHCKAD(
                    b"COVER",
                    save.COVER.subarray(1),
                    b"=",
                    save.TMPWIN.subarray(1),
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    0.0,
                    OK,
                    ctx,
                )?;

                if (I == 6) {
                    spicelib::SCARDD(0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if (save.L == 2) {
                        //
                        // The same interval is expected to appear in the output.
                        //
                        spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Compare CKCOV results against those from T_CKCOV.
                    //
                    T_CKCOV(
                        CK,
                        save.INST[I],
                        false,
                        b"INTERVAL",
                        save.TOL,
                        b"SCLK",
                        save.TMPWIN.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check cardinality of coverage window.
                    //
                    testutil::CHCKSI(
                        b"CARDD(COVER)",
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        b"=",
                        spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                        0,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check coverage window.
                    //
                    testutil::CHCKAD(
                        b"COVER",
                        save.COVER.subarray(1),
                        b"=",
                        save.TMPWIN.subarray(1),
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        0.0,
                        OK,
                        ctx,
                    )?;
                }
            }

            for I in 1..=NINS {
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut save.TITLE,
                    b"INST: #;  LEVEL: INTERVAL  NEEDAV: FALSE; TIMSYS: TDB; TOL: 0.D0; COVER: #.",
                );

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CVSTAT, &mut save.TITLE);
                testutil::TCASE(&save.TITLE, ctx)?;

                //
                // Initialize COVER.
                //
                spicelib::SSIZED(MAXCOV, save.COVER.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Make a copy of the expected window.
                //
                spicelib::COPYD(
                    save.XCVINT.subarray([LBCELL, I]),
                    save.TMPWIN.as_slice_mut(),
                    ctx,
                )?;

                //
                // Convert the expected window to TDB.
                //
                for J in 1..=spicelib::CARDD(save.TMPWIN.as_slice(), ctx)? {
                    spicelib::SCT2E(save.CLKID[I], save.TMPWIN[J], &mut save.ET, ctx)?;
                    save.TMPWIN[J] = save.ET;
                }

                if (save.L == 2) {
                    //
                    // Insert an interval into COVER.  This same interval
                    // must be added to each window containing expected
                    // coverage.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.COVER.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // The same interval is expected to appear in the output.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                spicelib::CKCOV(
                    CK,
                    save.INST[I],
                    false,
                    b"INTERVAL",
                    0.0,
                    b"TDB",
                    save.COVER.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check cardinality of coverage window.
                //
                testutil::CHCKSI(
                    b"CARDD(COVER)",
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    b"=",
                    spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                    0,
                    OK,
                    ctx,
                )?;

                //
                // Check coverage window.
                //
                testutil::CHCKAD(
                    b"COVER",
                    save.COVER.subarray(1),
                    b"=",
                    save.TMPWIN.subarray(1),
                    spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                    0.0,
                    OK,
                    ctx,
                )?;

                if (I == 6) {
                    spicelib::SCARDD(0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if (save.L == 2) {
                        //
                        // The same interval is expected to appear in the output.
                        //
                        spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Compare CKCOV results against those from T_CKCOV.
                    //
                    T_CKCOV(
                        CK,
                        save.INST[I],
                        false,
                        b"INTERVAL",
                        0.0,
                        b"TDB",
                        save.TMPWIN.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check cardinality of coverage window.
                    //
                    testutil::CHCKSI(
                        b"CARDD(COVER)",
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        b"=",
                        spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                        0,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check coverage window.
                    //
                    testutil::CHCKAD(
                        b"COVER",
                        save.COVER.subarray(1),
                        b"=",
                        save.TMPWIN.subarray(1),
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        0.0,
                        OK,
                        ctx,
                    )?;
                }
            }

            for I in 1..=NINS {
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut save.TITLE,
                    b"INST: #;  LEVEL: INTERVAL  NEEDAV: FALSE; TIMSYS: TDB; TOL: 1.D0; COVER: #.",
                );

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CVSTAT, &mut save.TITLE);
                testutil::TCASE(&save.TITLE, ctx)?;

                //
                // Initialize COVER.
                //
                spicelib::SSIZED(MAXCOV, save.COVER.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Adjust our expected result window by TOL.
                //
                spicelib::COPYD(
                    save.XCVINT.subarray([LBCELL, I]),
                    save.TMPWIN.as_slice_mut(),
                    ctx,
                )?;

                save.TOL = 1.0;

                spicelib::WNEXPD(save.TOL, save.TOL, save.TMPWIN.as_slice_mut(), ctx)?;

                //
                // Make sure the window doesn't start with a negative tick
                // value.
                //
                save.TMPWIN[1] = intrinsics::DMAX1(&[save.TMPWIN[1], 0.0]);

                //
                // Convert the expected window to TDB.
                //
                for J in 1..=spicelib::CARDD(save.TMPWIN.as_slice(), ctx)? {
                    spicelib::SCT2E(save.CLKID[I], save.TMPWIN[J], &mut save.ET, ctx)?;
                    save.TMPWIN[J] = save.ET;
                }

                if (save.L == 2) {
                    //
                    // Insert an interval into COVER.  This same interval
                    // must be added to each window containing expected
                    // coverage.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.COVER.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // The same interval is expected to appear in the output.
                    //
                    spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                spicelib::CKCOV(
                    CK,
                    save.INST[I],
                    false,
                    b"INTERVAL",
                    1.0,
                    b"TDB",
                    save.COVER.as_slice_mut(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check cardinality of coverage window.
                //
                testutil::CHCKSI(
                    b"CARDD(COVER)",
                    spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                    b"=",
                    spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                    0,
                    OK,
                    ctx,
                )?;

                //
                // Check coverage window.
                //
                testutil::CHCKAD(
                    b"COVER",
                    save.COVER.subarray(1),
                    b"=",
                    save.TMPWIN.subarray(1),
                    spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                    0.0,
                    OK,
                    ctx,
                )?;

                if (I == 6) {
                    spicelib::SCARDD(0, save.TMPWIN.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if (save.L == 2) {
                        //
                        // The same interval is expected to appear in the output.
                        //
                        spicelib::WNINSD(1000000.0, 10000000.0, save.TMPWIN.as_slice_mut(), ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Compare CKCOV results against those from T_CKCOV.
                    //
                    T_CKCOV(
                        CK,
                        save.INST[I],
                        false,
                        b"INTERVAL",
                        1.0,
                        b"TDB",
                        save.TMPWIN.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check cardinality of coverage window.
                    //
                    testutil::CHCKSI(
                        b"CARDD(COVER)",
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        b"=",
                        spicelib::CARDD(save.TMPWIN.as_slice(), ctx)?,
                        0,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check coverage window.
                    //
                    testutil::CHCKAD(
                        b"COVER",
                        save.COVER.subarray(1),
                        b"=",
                        save.TMPWIN.subarray(1),
                        spicelib::CARDD(save.COVER.as_slice(), ctx)?,
                        0.0,
                        OK,
                        ctx,
                    )?;
                }
            }

            save.L += m3__;
        }
    }

    //
    // Error cases:
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find coverage using time system UTC.", ctx)?;

    spicelib::CKCOV(
        CK,
        save.INST[2],
        false,
        b"SEGMENT",
        0.0,
        b"UTC",
        save.COVER.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find coverage using negative tolerance.", ctx)?;

    spicelib::CKCOV(
        CK,
        save.INST[2],
        false,
        b"SEGMENT",
        -1.0,
        b"TDB",
        save.COVER.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find coverage using level \"file\".", ctx)?;

    spicelib::CKCOV(
        CK,
        save.INST[2],
        false,
        b"FILE",
        0.0,
        b"TDB",
        save.COVER.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find coverage for a transfer CK.", ctx)?;

    spicelib::TXTOPN(XFRCK, &mut save.XUNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBT(CK, save.XUNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(save.XUNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    spicelib::CKCOV(
        XFRCK,
        save.INST[1],
        false,
        b"SEGMENT",
        0.0,
        b"SCLK",
        save.COVER.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFORMAT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find coverage for an SPK.", ctx)?;

    testutil::TSTSPK(SPK, false, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKCOV(
        SPK,
        save.INST[1],
        false,
        b"SEGMENT",
        0.0,
        b"SCLK",
        save.COVER.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFILETYPE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find coverage for an EK.", ctx)?;

    testutil::TSTEK(EK, 1, 20, false, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKCOV(
        EK,
        save.INST[1],
        false,
        b"SEGMENT",
        0.0,
        b"SCLK",
        save.COVER.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDARCHTYPE)", OK, ctx)?;

    // ******************************************************
    // ******************************************************
    // ******************************************************
    //     CKOBJ tests
    // ******************************************************
    // ******************************************************
    // ******************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find objects in our test CK.", ctx)?;

    spicelib::SSIZEI(NINS, save.IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI(NINS, save.XIDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NINS {
        spicelib::INSRTI(save.INST[I], save.XIDS.as_slice_mut(), ctx)?;
    }

    spicelib::CKOBJ(CK, save.IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check cardinality of object set.
    //
    testutil::CHCKSI(
        b"CARDI(IDS)",
        spicelib::CARDI(save.IDS.as_slice(), ctx)?,
        b"=",
        spicelib::CARDI(save.XIDS.as_slice(), ctx)?,
        0,
        OK,
        ctx,
    )?;

    //
    // Check object set.
    //
    testutil::CHCKAI(
        b"IDS",
        save.IDS.subarray(1),
        b"=",
        save.XIDS.subarray(1),
        spicelib::CARDI(save.XIDS.as_slice(), ctx)?,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Find objects in our test CK.  Start with non-empty ID set.",
        ctx,
    )?;

    spicelib::SSIZEI((NINS + 1), save.IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI((NINS + 1), save.XIDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::INSRTI(-1000000, save.XIDS.as_slice_mut(), ctx)?;

    for I in 1..=NINS {
        spicelib::INSRTI(save.INST[I], save.XIDS.as_slice_mut(), ctx)?;
    }

    spicelib::INSRTI(-1000000, save.IDS.as_slice_mut(), ctx)?;

    spicelib::CKOBJ(CK, save.IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check cardinality of object set.
    //
    testutil::CHCKSI(
        b"CARDI(IDS)",
        spicelib::CARDI(save.IDS.as_slice(), ctx)?,
        b"=",
        spicelib::CARDI(save.XIDS.as_slice(), ctx)?,
        0,
        OK,
        ctx,
    )?;

    //
    // Check object set.
    //
    testutil::CHCKAI(
        b"IDS",
        save.IDS.subarray(1),
        b"=",
        save.XIDS.subarray(1),
        spicelib::CARDI(save.XIDS.as_slice(), ctx)?,
        OK,
        ctx,
    )?;
    //
    // Error cases:
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find objects for a transfer CK.", ctx)?;

    //
    // Initialize the IDS set.
    //
    spicelib::SSIZEI(NINS, save.IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKOBJ(XFRCK, save.IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFORMAT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find objects for an SPK.", ctx)?;

    spicelib::CKOBJ(SPK, save.IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFILETYPE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Try to find objects for an EK.", ctx)?;

    spicelib::CKOBJ(EK, save.IDS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDARCHTYPE)", OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::DELFIL(CK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(XFRCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(EK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
