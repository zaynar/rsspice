//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CNVTOL: f64 = 0.000001;
const NWMAX: i32 = 15;
const NWDIST: i32 = 5;
const NWSEP: i32 = 5;
const NWRR: i32 = 5;
const NWUDS: i32 = 5;
const NWPA: i32 = 5;
const NWILUM: i32 = 5;
const ADDWIN: f64 = 0.5;
const FRMNLN: i32 = 32;
const FOVTLN: i32 = 40;
const FTCIRC: &[u8] = b"CIRCLE";
const FTELLI: &[u8] = b"ELLIPSE";
const FTPOLY: &[u8] = b"POLYGON";
const FTRECT: &[u8] = b"RECTANGLE";
const ANNULR: &[u8] = b"ANNULAR";
const ANY: &[u8] = b"ANY";
const PARTL: &[u8] = b"PARTIAL";
const FULL: &[u8] = b"FULL";
const DSSHAP: &[u8] = b"DSK";
const EDSHAP: &[u8] = b"ELLIPSOID";
const PTSHAP: &[u8] = b"POINT";
const RYSHAP: &[u8] = b"RAY";
const SPSHAP: &[u8] = b"SPHERE";
const NOCTYP: i32 = 4;
const OCLLN: i32 = 7;
const SHPLEN: i32 = 9;
const MAXVRT: i32 = 10000;
const CIRFOV: &[u8] = b"CIRCLE";
const ELLFOV: &[u8] = b"ELLIPSE";
const POLFOV: &[u8] = b"POLYGON";
const RECFOV: &[u8] = b"RECTANGLE";
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const SPK1: &[u8] = b"nat.bsp";
const PCK1: &[u8] = b"nat.pck";
const LNSIZE: i32 = 80;
const VTIGHT: f64 = 0.000000000001;
const NCORR: i32 = 9;
const ABCLEN: i32 = 25;

struct SaveVars {
    TARGT1: ActualCharArray,
    OBSRVR: ActualCharArray,
    TARGT2: ActualCharArray,
    CORR: ActualCharArray,
    SHAPES: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TARGT1 = ActualCharArray::new(ABCLEN, 1..=6);
        let mut OBSRVR = ActualCharArray::new(ABCLEN, 1..=6);
        let mut TARGT2 = ActualCharArray::new(ABCLEN, 1..=6);
        let mut CORR = ActualCharArray::new(ABCLEN, 1..=NCORR);
        let mut SHAPES = ActualCharArray::new(LNSIZE, 1..=2);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"ALPHA"),
                Val::C(b"X"),
                Val::C(b"ALPHA"),
                Val::C(b"ALPHA"),
                Val::C(b"ALPHA"),
                Val::C(b"SUN"),
            ]
            .into_iter();
            TARGT1
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"X"),
                Val::C(b"BETA"),
                Val::C(b"BETA"),
                Val::C(b"ALPHA"),
                Val::C(b"BETA"),
                Val::C(b"BETA"),
            ]
            .into_iter();
            TARGT2
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"SUN"),
                Val::C(b"SUN"),
                Val::C(b"X"),
                Val::C(b"SUN"),
                Val::C(b"BETA"),
                Val::C(b"SUN"),
            ]
            .into_iter();
            OBSRVR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NONE"),
                Val::C(b"lt"),
                Val::C(b" lt+s"),
                Val::C(b" cn"),
                Val::C(b" cn + s"),
                Val::C(b"XLT"),
                Val::C(b"XLT + S"),
                Val::C(b"XCN"),
                Val::C(b"XCN+S"),
            ]
            .into_iter();
            CORR.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"POINT"), Val::C(b"SPHERE")].into_iter();
            SHAPES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            TARGT1,
            OBSRVR,
            TARGT2,
            CORR,
            SHAPES,
        }
    }
}

//$Procedure F_ZZGFSPU ( ZZGFSPU family tests )
pub fn F_ZZGFSPU(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut XABCR = [b' '; ABCLEN as usize];
    let mut XREF = ActualCharArray::new(ABCLEN, 1..=2);
    let mut YREF = [b' '; ABCLEN as usize];
    let mut ABCORR = [b' '; ABCLEN as usize];
    let mut FRAME = ActualCharArray::new(ABCLEN, 1..=2);
    let mut FROM = [b' '; LNSIZE as usize];
    let mut OF = ActualCharArray::new(LNSIZE, 1..=2);
    let mut SHAPE = ActualCharArray::new(LNSIZE, 1..=2);
    let mut TXT = [b' '; LNSIZE as usize];
    let mut AXESA = StackArray::<f64, 3>::new(1..=3);
    let mut AXESB = StackArray::<f64, 3>::new(1..=3);
    let mut ET: f64 = 0.0;
    let mut HANGA: f64 = 0.0;
    let mut HANGB: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut POSA = StackArray::<f64, 3>::new(1..=3);
    let mut POSB = StackArray::<f64, 3>::new(1..=3);
    let mut RADA: f64 = 0.0;
    let mut RADB: f64 = 0.0;
    let mut SEP: f64 = 0.0;
    let mut THETA: f64 = 0.0;
    let mut XRAD = StackArray::<f64, 2>::new(1..=2);
    let mut HAN1: i32 = 0;
    let mut XBOD = StackArray::<i32, 2>::new(1..=2);
    let mut XOBS: i32 = 0;
    let mut XSHP = StackArray::<i32, 2>::new(1..=2);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Indices 1:3 for Invalid body name test, 4:6 for not distinct
    // body names test.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFSPU", ctx)?;

    //
    // Case 1: Create kernels, load same.
    //
    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    // Leapseconds:  Note that the LSK is deleted after loading, so we
    // don't have to clean it up later.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a PCK, load using FURNSH.
    //

    testutil::NATPCK(PCK1, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create an SPK, load using FURNSH.
    //

    testutil::NATSPK(SPK1, true, &mut HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Assign the body radii.
    //
    spicelib::ZZGFTREB(1000, AXESA.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    RADA = intrinsics::DMAX1(&[AXESA[1], AXESA[2], AXESA[3]]);

    spicelib::ZZGFTREB(2000, AXESB.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    RADB = intrinsics::DMAX1(&[AXESB[1], AXESB[2], AXESB[3]]);

    //
    // Case 2: Invalid body names.
    //
    fstr::assign(&mut ABCORR, save.CORR.get(1));
    fstr::assign(SHAPE.get_mut(1), save.SHAPES.get(1));
    fstr::assign(SHAPE.get_mut(2), save.SHAPES.get(1));
    fstr::assign(FRAME.get_mut(1), b"ECLIPJ2000");
    fstr::assign(FRAME.get_mut(2), b"ECLIPJ2000");

    for I in 1..=3 {
        fstr::assign(OF.get_mut(1), save.TARGT1.get(I));
        fstr::assign(OF.get_mut(2), save.TARGT2.get(I));
        fstr::assign(&mut FROM, save.OBSRVR.get(I));

        fstr::assign(
            &mut TXT,
            b"Invalid body name test. OF(1) = #, OF(2) = #, FROM = #",
        );
        spicelib::REPMC(&TXT.clone(), b"#", &OF[1], &mut TXT);
        spicelib::REPMC(&TXT.clone(), b"#", &OF[2], &mut TXT);
        spicelib::REPMC(&TXT.clone(), b"#", &FROM, &mut TXT);

        testutil::TCASE(&TXT, ctx)?;

        spicelib::ZZGFSPIN(
            OF.as_arg(),
            &FROM,
            SHAPE.as_arg_mut(),
            FRAME.as_arg(),
            &ABCORR,
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;
    }

    //
    // Case 3: Not distinct body names.
    //
    fstr::assign(&mut ABCORR, save.CORR.get(1));
    fstr::assign(SHAPE.get_mut(1), save.SHAPES.get(1));
    fstr::assign(SHAPE.get_mut(2), save.SHAPES.get(1));
    fstr::assign(FRAME.get_mut(1), b"ECLIPJ2000");
    fstr::assign(FRAME.get_mut(2), b"ECLIPJ2000");

    for I in 4..=6 {
        fstr::assign(OF.get_mut(1), save.TARGT1.get(I));
        fstr::assign(OF.get_mut(2), save.TARGT2.get(I));
        fstr::assign(&mut FROM, save.OBSRVR.get(I));

        fstr::assign(
            &mut TXT,
            b"Not distinct body name test. OF(1) = #, OF(2) = #, FROM = #",
        );
        spicelib::REPMC(&TXT.clone(), b"#", &OF[1], &mut TXT);
        spicelib::REPMC(&TXT.clone(), b"#", &OF[2], &mut TXT);
        spicelib::REPMC(&TXT.clone(), b"#", &FROM, &mut TXT);

        testutil::TCASE(&TXT, ctx)?;

        spicelib::ZZGFSPIN(
            OF.as_arg(),
            &FROM,
            SHAPE.as_arg_mut(),
            FRAME.as_arg(),
            &ABCORR,
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;
    }

    //
    // Case 4:
    //
    fstr::assign(&mut ABCORR, save.CORR.get(1));
    fstr::assign(SHAPE.get_mut(1), b"X");
    fstr::assign(SHAPE.get_mut(2), save.SHAPES.get(2));
    fstr::assign(FRAME.get_mut(1), b"ECLIPJ2000");
    fstr::assign(FRAME.get_mut(2), b"ECLIPJ2000");
    fstr::assign(OF.get_mut(1), save.TARGT1.get(1));
    fstr::assign(OF.get_mut(2), save.TARGT2.get(2));
    fstr::assign(&mut FROM, save.OBSRVR.get(1));

    fstr::assign(&mut TXT, b"Invalid shape. SHAPE(1) = #, SHAPE(2) = #");
    spicelib::REPMC(&TXT.clone(), b"#", &SHAPE[1], &mut TXT);
    spicelib::REPMC(&TXT.clone(), b"#", &SHAPE[2], &mut TXT);

    testutil::TCASE(&TXT, ctx)?;

    spicelib::ZZGFSPIN(
        OF.as_arg(),
        &FROM,
        SHAPE.as_arg_mut(),
        FRAME.as_arg(),
        &ABCORR,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    fstr::assign(SHAPE.get_mut(1), save.SHAPES.get(1));
    fstr::assign(SHAPE.get_mut(2), b"X");

    fstr::assign(&mut TXT, b"Invalid shape. SHAPE(1) = #, SHAPE(2) = #");
    spicelib::REPMC(&TXT.clone(), b"#", &SHAPE[1], &mut TXT);
    spicelib::REPMC(&TXT.clone(), b"#", &SHAPE[2], &mut TXT);

    testutil::TCASE(&TXT, ctx)?;

    spicelib::ZZGFSPIN(
        OF.as_arg(),
        &FROM,
        SHAPE.as_arg_mut(),
        FRAME.as_arg(),
        &ABCORR,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    //
    // Case 5: Confirm initialized values are correctly saved.
    //
    fstr::assign(OF.get_mut(1), b"1000");
    fstr::assign(OF.get_mut(2), b"2000");
    fstr::assign(&mut FROM, b"SUN");
    fstr::assign(FRAME.get_mut(1), b"ECLIPJ2000");
    fstr::assign(FRAME.get_mut(2), b"ECLIPJ2000");

    for I in 1..=2 {
        for J in 1..=2 {
            for K in 1..=NCORR {
                fstr::assign(&mut ABCORR, save.CORR.get(K));
                fstr::assign(SHAPE.get_mut(1), save.SHAPES.get(I));
                fstr::assign(SHAPE.get_mut(2), save.SHAPES.get(J));

                spicelib::REPMC(b"ZZGFSPX #1 #2 #3", b"#1", &ABCORR, &mut TXT);
                spicelib::REPMC(&TXT.clone(), b"#2", &SHAPE[1], &mut TXT);
                spicelib::REPMC(&TXT.clone(), b"#3", &SHAPE[2], &mut TXT);

                testutil::TCASE(&TXT, ctx)?;

                spicelib::ZZGFSPIN(
                    OF.as_arg(),
                    &FROM,
                    SHAPE.as_arg_mut(),
                    FRAME.as_arg(),
                    &ABCORR,
                    ctx,
                )?;

                spicelib::ZZGFSPX(
                    &mut XABCR,
                    XBOD.as_slice_mut(),
                    &mut YREF,
                    XREF.as_arg_mut(),
                    &mut XOBS,
                    XRAD.as_slice_mut(),
                    XSHP.as_slice_mut(),
                    ctx,
                );

                //
                // Squeaky cleanse the ABCORR string.
                //
                spicelib::UCASE(&ABCORR.clone(), &mut ABCORR, ctx);
                spicelib::CMPRSS(b" ", 0, &ABCORR.clone(), &mut ABCORR);

                testutil::CHCKSC(b"XABCR", &XABCR, b"=", &ABCORR, OK, ctx)?;

                //
                // Check IDs for target bodies array.
                //
                testutil::CHCKSI(b"XBOD 1", XBOD[1], b"=", 1000, 0, OK, ctx)?;
                testutil::CHCKSI(b"XBOD 2", XBOD[2], b"=", 2000, 0, OK, ctx)?;

                //
                // All calculations use J2000 frame.
                //
                testutil::CHCKSC(b"YREF", &YREF, b"=", b"J2000", OK, ctx)?;

                testutil::CHCKSC(b"XREF(1)", &XREF[1], b"=", &FRAME[1], OK, ctx)?;
                testutil::CHCKSC(b"XREF(2)", &XREF[2], b"=", &FRAME[2], OK, ctx)?;

                //
                // Check ID for observer body.
                //
                testutil::CHCKSI(b"XOBS", XOBS, b"=", 10, 0, OK, ctx)?;

                //
                // Check Index for SHAPES array.
                //
                testutil::CHCKSI(b"XSHP 1", XSHP[1], b"=", I, 0, OK, ctx)?;
                testutil::CHCKSI(b"XSHP 2", XSHP[2], b"=", J, 0, OK, ctx)?;

                //
                // Check radii for spherical body, XSHP(I) = 2
                //
                if (XSHP[1] == 2) {
                    testutil::CHCKSD(b"RAD 1", XRAD[1], b"=", RADA, 0.0, OK, ctx)?;
                }

                if (XSHP[2] == 2) {
                    testutil::CHCKSD(b"RAD 2", XRAD[2], b"=", RADB, 0.0, OK, ctx)?;
                }
            }
        }
    }

    //
    // Test the ZZSEPQ result using ALPHA and BETA from Nat's Solar
    // System.
    //
    // Step in increments of one minute from ET 0 to ET 86400 in steps
    // of 60 TDB seconds. Test the four combinations of Sphere and Point
    // object pairs. Perform each test in a block for clarity.
    //

    //
    // Perform the four test blocks for each aberration correction value.
    //
    for J in 1..=NCORR {
        fstr::assign(&mut ABCORR, save.CORR.get(J));

        //
        // Case 6: Sphere-Sphere
        //
        ET = 0.0;

        for I in 1..=1440 {
            spicelib::REPMI(
                b"ZZSEPQ Sphere/Sphere #1, ABCORR #2",
                b"#1",
                I,
                &mut TXT,
                ctx,
            );
            spicelib::REPMC(&TXT.clone(), b"#2", &ABCORR, &mut TXT);
            testutil::TCASE(&TXT, ctx)?;

            SEP = spicelib::ZZSEPQ(ET, 1000, 2000, RADA, RADB, 10, &ABCORR, b"J2000", ctx)?;

            spicelib::SPKPOS(
                b"ALPHA",
                ET,
                b"J2000",
                &ABCORR,
                b"SUN",
                POSA.as_slice_mut(),
                &mut LT,
                ctx,
            )?;
            spicelib::SPKPOS(
                b"BETA",
                ET,
                b"J2000",
                &ABCORR,
                b"SUN",
                POSB.as_slice_mut(),
                &mut LT,
                ctx,
            )?;

            HANGA = f64::asin((RADA / spicelib::VNORM(POSA.as_slice())));
            HANGB = f64::asin((RADB / spicelib::VNORM(POSB.as_slice())));

            THETA = (spicelib::VSEP(POSA.as_slice(), POSB.as_slice(), ctx) - (HANGA + HANGB));
            testutil::CHCKSD(b"ZZSEPQ", SEP, b"~", THETA, VTIGHT, OK, ctx)?;

            ET = (ET + 60.0);
        }

        //
        // Case 7: Point-Sphere
        //
        RADA = 0.0;
        ET = 0.0;

        for I in 1..=1440 {
            spicelib::REPMI(
                b"ZZSEPQ Point/Sphere #1, ABCORR #2",
                b"#1",
                I,
                &mut TXT,
                ctx,
            );
            spicelib::REPMC(&TXT.clone(), b"#2", &ABCORR, &mut TXT);
            testutil::TCASE(&TXT, ctx)?;

            SEP = spicelib::ZZSEPQ(ET, 1000, 2000, RADA, RADB, 10, &ABCORR, b"J2000", ctx)?;

            spicelib::SPKPOS(
                b"ALPHA",
                ET,
                b"J2000",
                &ABCORR,
                b"SUN",
                POSA.as_slice_mut(),
                &mut LT,
                ctx,
            )?;
            spicelib::SPKPOS(
                b"BETA",
                ET,
                b"J2000",
                &ABCORR,
                b"SUN",
                POSB.as_slice_mut(),
                &mut LT,
                ctx,
            )?;

            HANGA = f64::asin((RADA / spicelib::VNORM(POSA.as_slice())));
            HANGB = f64::asin((RADB / spicelib::VNORM(POSB.as_slice())));

            THETA = (spicelib::VSEP(POSA.as_slice(), POSB.as_slice(), ctx) - (HANGA + HANGB));
            testutil::CHCKSD(b"ZZSEPQ", SEP, b"~", THETA, VTIGHT, OK, ctx)?;

            ET = (ET + 60.0);
        }

        //
        // Case 8: Sphere-Point
        //
        RADA = intrinsics::DMAX1(&[AXESA[1], AXESA[2], AXESA[3]]);
        RADB = 0.0;
        ET = 0.0;

        for I in 1..=1440 {
            spicelib::REPMI(
                b"ZZSEPQ Sphere/Point #1, ABCORR #2",
                b"#1",
                I,
                &mut TXT,
                ctx,
            );
            spicelib::REPMC(&TXT.clone(), b"#2", &ABCORR, &mut TXT);
            testutil::TCASE(&TXT, ctx)?;

            SEP = spicelib::ZZSEPQ(ET, 1000, 2000, RADA, RADB, 10, &ABCORR, b"J2000", ctx)?;

            spicelib::SPKPOS(
                b"ALPHA",
                ET,
                b"J2000",
                &ABCORR,
                b"SUN",
                POSA.as_slice_mut(),
                &mut LT,
                ctx,
            )?;
            spicelib::SPKPOS(
                b"BETA",
                ET,
                b"J2000",
                &ABCORR,
                b"SUN",
                POSB.as_slice_mut(),
                &mut LT,
                ctx,
            )?;

            HANGA = f64::asin((RADA / spicelib::VNORM(POSA.as_slice())));
            HANGB = f64::asin((RADB / spicelib::VNORM(POSB.as_slice())));

            THETA = (spicelib::VSEP(POSA.as_slice(), POSB.as_slice(), ctx) - (HANGA + HANGB));
            testutil::CHCKSD(b"ZZSEPQ", SEP, b"~", THETA, VTIGHT, OK, ctx)?;

            ET = (ET + 60.0);
        }

        //
        // Case 9: Point-Point
        //
        RADA = 0.0;
        RADB = 0.0;
        ET = 0.0;

        for I in 1..=1440 {
            spicelib::REPMI(b"ZZSEPQ Point/Point #1, ABCORR #2", b"#1", I, &mut TXT, ctx);
            spicelib::REPMC(&TXT.clone(), b"#2", &ABCORR, &mut TXT);
            testutil::TCASE(&TXT, ctx)?;

            SEP = spicelib::ZZSEPQ(ET, 1000, 2000, RADA, RADB, 10, &ABCORR, b"J2000", ctx)?;

            spicelib::SPKPOS(
                b"ALPHA",
                ET,
                b"J2000",
                &ABCORR,
                b"SUN",
                POSA.as_slice_mut(),
                &mut LT,
                ctx,
            )?;
            spicelib::SPKPOS(
                b"BETA",
                ET,
                b"J2000",
                &ABCORR,
                b"SUN",
                POSB.as_slice_mut(),
                &mut LT,
                ctx,
            )?;

            HANGA = f64::asin((RADA / spicelib::VNORM(POSA.as_slice())));
            HANGB = f64::asin((RADB / spicelib::VNORM(POSB.as_slice())));

            THETA = (spicelib::VSEP(POSA.as_slice(), POSB.as_slice(), ctx) - (HANGA + HANGB));
            testutil::CHCKSD(b"ZZSEPQ", SEP, b"~", THETA, VTIGHT, OK, ctx)?;

            ET = (ET + 60.0);
        }
    }

    //
    // Case N
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
