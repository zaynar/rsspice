//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const RSVUNT: i32 = 2;
const SCRUNT: i32 = 1;
const UTSIZE: i32 = ((20 + SCRUNT) + RSVUNT);
const READ: i32 = 1;
const WRITE: i32 = 2;
const SCRTCH: i32 = 3;
const NEW: i32 = 4;
const NUMAMH: i32 = 4;
const BIGI3E: i32 = 1;
const LTLI3E: i32 = 2;
const VAXGFL: i32 = 3;
const VAXDFL: i32 = 4;
const NUMBFF: i32 = 4;
const STRSIZ: i32 = 8;
const STRLEN: i32 = ((STRSIZ + 1) * NUMBFF);
const DAF: i32 = 1;
const DAS: i32 = 2;
const NUMARC: i32 = 2;
const RECL: i32 = 1024;
const FILEN: i32 = 255;
const CBFSIZ: i32 = 1024;
const TIGHT: f64 = 0.00000000000001;
const WDSIZE: i32 = 32;
const LNSIZE: i32 = 80;

//$Procedure F_DDHFTSIZE ( Handle Manager File Table Size )
pub fn F_DDHFTSIZE(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TCNAME = [b' '; LNSIZE as usize];
    let mut MKTXT = ActualCharArray::new(WDSIZE, 1..=(FTSIZE + 100));
    let mut FNMPAT = [b' '; WDSIZE as usize];
    let mut FNAME = ActualCharArray::new(WDSIZE, 1..=(FTSIZE + 1));
    let mut MKNAM = [b' '; WDSIZE as usize];
    let mut I: i32 = 0;
    let mut EPOCHS = StackArray::<f64, 2>::new(1..=2);
    let mut STATES = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut STATE1 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE2 = StackArray::<f64, 6>::new(1..=6);
    let mut QUATS = StackArray2D::<f64, 8>::new(1..=4, 1..=2);
    let mut AVS = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut LT: f64 = 0.0;
    let mut MAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut QUAT1 = StackArray::<f64, 4>::new(1..=4);
    let mut QUAT2 = StackArray::<f64, 4>::new(1..=4);
    let mut HANDLE: i32 = 0;
    let mut CLKOUT: f64 = 0.0;
    let mut FOUND: bool = false;
    let mut MAXCNT: i32 = 0;
    let mut MAXRLD: i32 = 0;
    let mut MAKKER: bool = false;
    let mut DELKER: bool = false;
    let mut NMKTXT: i32 = 0;
    let mut UNIT: i32 = 0;
    let mut COEFFS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
    let mut REPORT: bool = false;

    //
    // Local Parameters
    //

    //
    // SPICELIB functions
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
    // Open the test family.
    //
    testutil::TOPEN(b"F_DDHFTSIZE", ctx)?;

    //
    // Set the number of files we will test.
    //
    MAXCNT = FTSIZE;

    //
    // Set the number of files we will load second time.
    //
    MAXRLD = intrinsics::MIN0(&[200, MAXCNT]);

    //
    // Set flags indicating whether we try to make kernels and whether we
    // try delete kernels.
    //
    MAKKER = true;
    DELKER = true;

    //
    // Set flag to report elapsed time (or not.)
    //
    //  REPORT = .TRUE.
    REPORT = false;

    T_ELAPSD(false, b"no report on first call", b"total", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //     Test MAXCNT CKs.
    //
    //     Make and load CKs.
    //
    fstr::assign(&mut TCNAME, b"Handle Manager: # CKs");
    spicelib::REPMI(&TCNAME.clone(), b"#", MAXCNT, &mut TCNAME, ctx);
    testutil::TCASE(&TCNAME, ctx)?;

    //
    // Make CK names.
    //
    fstr::assign(&mut FNMPAT, b"file_ck#.bc");
    {
        let m1__: i32 = 1;
        let m2__: i32 = (MAXCNT + 1);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::REPMI(&FNMPAT, b"#", I, &mut FNAME[I], ctx);
            I += m3__;
        }
    }

    //
    // Make CK files.
    //
    if MAKKER {
        spicelib::CLEARD(8, QUATS.as_slice_mut());
        spicelib::CLEARD(6, AVS.as_slice_mut());

        {
            let m1__: i32 = 1;
            let m2__: i32 = MAXCNT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if spicelib::EXISTS(&FNAME[I], ctx)? {
                    spicelib::DELFIL(&FNAME[I], ctx)?;
                }

                spicelib::CKOPN(&FNAME[I], b" ", 0, &mut HANDLE, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                EPOCHS[1] = ((I as f64) - 0.5);
                EPOCHS[2] = ((I as f64) + 0.5);

                spicelib::ROTATE(
                    ((((I as f64) * 0.001) - 0.0005) as f64),
                    1,
                    MAT.as_slice_mut(),
                    ctx,
                );
                spicelib::M2Q(MAT.as_slice(), QUATS.subarray_mut([1, 1]), ctx)?;

                spicelib::ROTATE(
                    ((((I as f64) * 0.001) + 0.0005) as f64),
                    1,
                    MAT.as_slice_mut(),
                    ctx,
                );
                spicelib::M2Q(MAT.as_slice(), QUATS.subarray_mut([1, 2]), ctx)?;

                spicelib::CKW03(
                    HANDLE,
                    EPOCHS[1],
                    EPOCHS[2],
                    -999,
                    b"J2000",
                    false,
                    &FNAME[I],
                    2,
                    EPOCHS.as_slice(),
                    QUATS.as_slice(),
                    AVS.as_slice(),
                    1,
                    EPOCHS.subarray(1),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::CKCLS(HANDLE, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                I += m3__;
            }
        }
    }

    T_ELAPSD(REPORT, b"Making CKs", b"running", ctx)?;

    //
    // Load all CKs. Then load the first MAXRLD CKs again to check that
    // they are recongized as known and don't overflow the file table.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXCNT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::FURNSH(&FNAME[I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXRLD;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::FURNSH(&FNAME[I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            I += m3__;
        }
    }

    T_ELAPSD(REPORT, b"Loading CKs", b"running", ctx)?;

    //
    // Read CKs. Check rotations.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXCNT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::CKGP(
                -999,
                (I as f64),
                0.0,
                b"J2000",
                MAT.as_slice_mut(),
                &mut CLKOUT,
                &mut FOUND,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"CKGP FOUND", FOUND, true, OK, ctx)?;

            spicelib::M2Q(MAT.as_slice(), QUAT1.as_slice_mut(), ctx)?;

            spicelib::ROTATE((((I as f64) * 0.001) as f64), 1, MAT.as_slice_mut(), ctx);
            spicelib::M2Q(MAT.as_slice(), QUAT2.as_slice_mut(), ctx)?;

            testutil::CHCKAD(
                b"CKGP quaternion",
                QUAT1.as_slice(),
                b"~",
                QUAT2.as_slice(),
                4,
                TIGHT,
                OK,
                ctx,
            )?;

            I += m3__;
        }
    }

    T_ELAPSD(REPORT, b"Reading CKs", b"running", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //     Cause error by loading one more CK.
    //
    if (MAXCNT == FTSIZE) {
        fstr::assign(&mut TCNAME, b"Handle Manager: #+1 CKs");
        spicelib::REPMI(&TCNAME.clone(), b"#", MAXCNT, &mut TCNAME, ctx);
        testutil::TCASE(&TCNAME, ctx)?;

        I = (MAXCNT + 1);

        spicelib::CKOPN(&FNAME[I], &FNAME[I], 0, &mut HANDLE, ctx)?;
        testutil::CHCKXC(true, b"SPICE(DAFFTFULL)", OK, ctx)?;
    }

    //
    // Unload all CKs. Doing this one-by-one is EXTREMELY slow!
    //
    //  DO I = 1, MAXCNT
    //
    //     WRITE (*,*) 'unloading CK ', I
    //
    //     CALL UNLOAD ( FNAME(I) )
    //     CALL CHCKXC ( .FALSE., ' ', OK )
    //
    //  END DO

    spicelib::KCLEAR(ctx)?;

    if DELKER {
        {
            let m1__: i32 = 1;
            let m2__: i32 = MAXCNT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::DELFIL(&FNAME[I], ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                I += m3__;
            }
        }
    }

    T_ELAPSD(REPORT, b"Deleting CKs", b"running", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //     Test MAXCNT SPKs.
    //
    //     Make and load SPKs.
    //
    fstr::assign(&mut TCNAME, b"Handle Manager: # SPKs");
    spicelib::REPMI(&TCNAME.clone(), b"#", MAXCNT, &mut TCNAME, ctx);
    testutil::TCASE(&TCNAME, ctx)?;

    //
    // Make SPK names. Set MK name.
    //
    fstr::assign(&mut FNMPAT, b"file_spk#.bsp");

    {
        let m1__: i32 = 1;
        let m2__: i32 = (MAXCNT + 1);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::REPMI(&FNMPAT, b"#", I, &mut FNAME[I], ctx);
            I += m3__;
        }
    }

    fstr::assign(&mut MKNAM, b"file_spks.tm");

    //
    // Make SPK files.
    //
    if MAKKER {
        //
        // Set initial MK lines.
        //
        fstr::assign(MKTXT.get_mut(1), b"KPL/MK");
        testutil::BEGDAT(&mut MKTXT[2]);
        fstr::assign(MKTXT.get_mut(3), b"PATH_SYMBOLS    = (");
        fstr::assign(MKTXT.get_mut(4), b"\'FILE\'");
        fstr::assign(MKTXT.get_mut(5), b"\'SPK\'");
        fstr::assign(MKTXT.get_mut(6), b"\'BSP\'");
        fstr::assign(MKTXT.get_mut(7), b")");
        fstr::assign(MKTXT.get_mut(8), b"PATH_VALUES    = (");
        fstr::assign(MKTXT.get_mut(9), b"\'file\'");
        fstr::assign(MKTXT.get_mut(10), b"\'spk\'");
        fstr::assign(MKTXT.get_mut(11), b"\'bsp\'");
        fstr::assign(MKTXT.get_mut(12), b")");
        fstr::assign(MKTXT.get_mut(13), b"KERNELS_TO_LOAD = (");
        NMKTXT = 13;

        spicelib::CLEARD(12, STATES.as_slice_mut());

        {
            let m1__: i32 = 1;
            let m2__: i32 = MAXCNT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Make next SPK.
                //
                if spicelib::EXISTS(&FNAME[I], ctx)? {
                    spicelib::DELFIL(&FNAME[I], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                spicelib::SPKOPN(&FNAME[I], b" ", 0, &mut HANDLE, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                EPOCHS[1] = ((I as f64) - 0.5);
                EPOCHS[2] = ((I as f64) + 0.5);

                STATES[[1, 1]] = EPOCHS[1];
                STATES[[1, 2]] = EPOCHS[2];

                spicelib::SPKW09(
                    HANDLE,
                    -999,
                    3,
                    b"J2000",
                    EPOCHS[1],
                    EPOCHS[2],
                    &FNAME[I],
                    1,
                    2,
                    STATES.as_slice(),
                    EPOCHS.as_slice(),
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKCLS(HANDLE, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Make MK lines for this SPK.
                //
                NMKTXT = (NMKTXT + 1);
                fstr::assign(
                    MKTXT.get_mut(NMKTXT),
                    &fstr::concat(
                        &fstr::concat(
                            b"\'",
                            fstr::substr(FNAME.get(I), 1..=spicelib::RTRIM(&FNAME[I])),
                        ),
                        b"\'",
                    ),
                );
                spicelib::UCASE(&MKTXT[NMKTXT].to_vec(), &mut MKTXT[NMKTXT], ctx);
                spicelib::REPMC(&MKTXT[NMKTXT].to_vec(), b"F", b"$F", &mut MKTXT[NMKTXT]);
                spicelib::REPMC(&MKTXT[NMKTXT].to_vec(), b"_", b"_$", &mut MKTXT[NMKTXT]);
                spicelib::REPMC(&MKTXT[NMKTXT].to_vec(), b".", b".$", &mut MKTXT[NMKTXT]);

                I += m3__;
            }
        }

        //
        // Make finishing MK lines and write MK.
        //
        fstr::assign(MKTXT.get_mut((NMKTXT + 1)), b")");
        testutil::BEGTXT(&mut MKTXT[(NMKTXT + 2)]);
        fstr::assign(MKTXT.get_mut((NMKTXT + 3)), b"End of MK file.");
        NMKTXT = (NMKTXT + 3);

        if spicelib::EXISTS(&MKNAM, ctx)? {
            spicelib::DELFIL(&MKNAM, ctx)?;
        }

        spicelib::TXTOPN(&MKNAM, &mut UNIT, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::WRITLA(NMKTXT, MKTXT.as_arg(), UNIT, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(UNIT),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
    }

    T_ELAPSD(REPORT, b"Making SPKs", b"running", ctx)?;

    //
    // Load all SPKs. Then load the first MAXRLD SPKs again to check that
    // they are recongized as known and don't overflow the file table.
    //
    spicelib::FURNSH(&MKNAM, ctx)?;

    // DO I = 1, MAXCNT
    //
    //    CALL FURNSH ( FNAME(I) )
    //    CALL CHCKXC ( .FALSE., ' ', OK )
    //
    // END DO

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXRLD;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::FURNSH(&FNAME[I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            I += m3__;
        }
    }

    T_ELAPSD(REPORT, b"Loading SPKs", b"running", ctx)?;

    //
    // Read SPKs. Check states.
    //
    spicelib::CLEARD(6, STATE2.as_slice_mut());

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXCNT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SPKGEO(
                -999,
                (I as f64),
                b"J2000",
                3,
                STATE1.as_slice_mut(),
                &mut LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            STATE2[1] = (I as f64);

            testutil::CHCKAD(
                b"SPKGEO state",
                STATE1.as_slice(),
                b"~",
                STATE2.as_slice(),
                6,
                TIGHT,
                OK,
                ctx,
            )?;

            I += m3__;
        }
    }

    T_ELAPSD(REPORT, b"Reading SPKs", b"running", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //     Cause error by trying to make one more SPK.
    //
    if (MAXCNT == FTSIZE) {
        fstr::assign(&mut TCNAME, b"Handle Manager: #+1 SPKs");
        spicelib::REPMI(&TCNAME.clone(), b"#", MAXCNT, &mut TCNAME, ctx);
        testutil::TCASE(&TCNAME, ctx)?;

        I = (MAXCNT + 1);

        spicelib::SPKOPN(&FNAME[I], &FNAME[I], 0, &mut HANDLE, ctx)?;
        testutil::CHCKXC(true, b"SPICE(DAFFTFULL)", OK, ctx)?;
    }

    //
    // Unload all SPKs. Doing this one-by-one is EXTREMELY slow!
    //
    //  DO I = 1, MAXCNT
    //
    //     WRITE (*,*) 'unloading SPK ', I
    //
    //     CALL UNLOAD ( FNAME(I) )
    //     CALL CHCKXC ( .FALSE., ' ', OK )
    //
    //  END DO

    spicelib::KCLEAR(ctx)?;

    if DELKER {
        {
            let m1__: i32 = 1;
            let m2__: i32 = MAXCNT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::DELFIL(&FNAME[I], ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                I += m3__;
            }
        }

        spicelib::DELFIL(&MKNAM, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    T_ELAPSD(REPORT, b"Deleting SPKs", b"running", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //     Test MAXCNT PCKs.
    //
    //     Make and load PCKs.
    //
    fstr::assign(&mut TCNAME, b"Handle Manager: # PCKs");
    spicelib::REPMI(&TCNAME.clone(), b"#", MAXCNT, &mut TCNAME, ctx);
    testutil::TCASE(&TCNAME, ctx)?;

    //
    // Make PCK names.
    //
    fstr::assign(&mut FNMPAT, b"file_pck#.bpc");
    {
        let m1__: i32 = 1;
        let m2__: i32 = (MAXCNT + 1);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::REPMI(&FNMPAT, b"#", I, &mut FNAME[I], ctx);
            I += m3__;
        }
    }

    //
    // Make PCK files.
    //
    if MAKKER {
        spicelib::CLEARD(8, QUATS.as_slice_mut());
        spicelib::CLEARD(6, AVS.as_slice_mut());

        {
            let m1__: i32 = 1;
            let m2__: i32 = MAXCNT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if spicelib::EXISTS(&FNAME[I], ctx)? {
                    spicelib::DELFIL(&FNAME[I], ctx)?;
                }

                spicelib::PCKOPN(&FNAME[I], b" ", 0, &mut HANDLE, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                EPOCHS[1] = ((I as f64) - 0.5);
                EPOCHS[2] = ((I as f64) + 0.5);

                COEFFS[[1, 1]] = ((I as f64) * 0.001);
                COEFFS[[2, 1]] = 0.0;

                COEFFS[[1, 2]] = ((I as f64) * 0.002);
                COEFFS[[2, 2]] = 0.0;

                COEFFS[[1, 3]] = ((I as f64) * 0.003);
                COEFFS[[2, 3]] = 0.0;

                spicelib::PCKW02(
                    HANDLE,
                    13000,
                    b"J2000",
                    EPOCHS[1],
                    EPOCHS[2],
                    &FNAME[I],
                    (EPOCHS[2] - EPOCHS[1]),
                    1,
                    1,
                    COEFFS.as_slice(),
                    EPOCHS[1],
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::PCKCLS(HANDLE, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                I += m3__;
            }
        }
    }

    T_ELAPSD(REPORT, b"Making PCKs", b"running", ctx)?;

    //
    // Load all PCKs. Then load the first MAXRLD PCKs again to check that
    // they are recongized as known and don't overflow the file table.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXCNT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::FURNSH(&FNAME[I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXRLD;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::FURNSH(&FNAME[I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            I += m3__;
        }
    }

    T_ELAPSD(REPORT, b"Loading PCKs", b"running", ctx)?;

    //
    // Read PCKs. Check rotations.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXCNT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::BODMAT(13000, (I as f64), MAT.as_slice_mut(), ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"BODMAT FOUND", FOUND, true, OK, ctx)?;

            spicelib::M2Q(MAT.as_slice(), QUAT1.as_slice_mut(), ctx)?;

            spicelib::EUL2M(
                ((I as f64) * 0.003),
                ((I as f64) * 0.002),
                ((I as f64) * 0.001),
                3,
                1,
                3,
                MAT.as_slice_mut(),
                ctx,
            )?;
            spicelib::M2Q(MAT.as_slice(), QUAT2.as_slice_mut(), ctx)?;

            testutil::CHCKAD(
                b"BODMAT quaternion",
                QUAT1.as_slice(),
                b"~",
                QUAT2.as_slice(),
                4,
                TIGHT,
                OK,
                ctx,
            )?;

            I += m3__;
        }
    }

    T_ELAPSD(REPORT, b"Reading PCKs", b"running", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //     Cause error by loading one more PCK.
    //
    if (MAXCNT == FTSIZE) {
        fstr::assign(&mut TCNAME, b"Handle Manager: #+1 PCKs");
        spicelib::REPMI(&TCNAME.clone(), b"#", MAXCNT, &mut TCNAME, ctx);
        testutil::TCASE(&TCNAME, ctx)?;

        I = (MAXCNT + 1);

        spicelib::PCKOPN(&FNAME[I], &FNAME[I], 0, &mut HANDLE, ctx)?;
        testutil::CHCKXC(true, b"SPICE(DAFFTFULL)", OK, ctx)?;
    }

    //
    // Unload all PCKs. Doing this one-by-one is EXTREMELY slow!
    //
    //  DO I = 1, MAXCNT
    //
    //     WRITE (*,*) 'unloading PCK ', I
    //
    //     CALL UNLOAD ( FNAME(I) )
    //     CALL CHCKXC ( .FALSE., ' ', OK )
    //
    //  END DO

    spicelib::KCLEAR(ctx)?;

    if DELKER {
        {
            let m1__: i32 = 1;
            let m2__: i32 = MAXCNT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::DELFIL(&FNAME[I], ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                I += m3__;
            }
        }
    }

    T_ELAPSD(REPORT, b"Deleting PCKs", b"running", ctx)?;

    //
    // Close out the test family.
    //
    T_ELAPSD(REPORT, b"Total", b"TOTAL", ctx)?;
    spicelib::TOSTDO(b" ", ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
