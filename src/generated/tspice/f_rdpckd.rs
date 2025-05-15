//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MEDIUM: f64 = 0.0000000002;
const TIGHT: f64 = 0.000000000001;
const VTIGHT: f64 = 0.00000000000001;
const LBCELL: i32 = -5;
const FILSIZ: i32 = 255;
const KVNMLN: i32 = 32;
const LNSIZE: i32 = 200;
const MAXSET: i32 = 1000;
const NUMLEN: i32 = KVNMLN;
const MAXANG: i32 = 200;
const MAXDEG: i32 = 3;
const MAXBUF: i32 = (MAXANG * (MAXDEG + 1));

struct SaveVars {
    DTYPE: Vec<u8>,
    LINE: Vec<u8>,
    IDSTR: Vec<u8>,
    KVDEG: Vec<u8>,
    KVNAME: Vec<u8>,
    KVPHAS: Vec<u8>,
    KVNAMS: ActualCharArray,
    PCK: Vec<u8>,
    QNAME: Vec<u8>,
    TITLE: Vec<u8>,
    XNAME: Vec<u8>,
    PHSBUF: ActualArray<f64>,
    B2J: StackArray2D<f64, 9>,
    CMAT: StackArray2D<f64, 4>,
    DEC: f64,
    DELTA: f64,
    DETM: f64,
    ET: f64,
    EUL0: StackArray<f64, 3>,
    EUL2: StackArray<f64, 3>,
    EULSTA: StackArray<f64, 6>,
    J2B: StackArray2D<f64, 9>,
    LAMBDA: f64,
    MI: StackArray2D<f64, 4>,
    RA: f64,
    RADII: StackArray<f64, 3>,
    RJ2E: StackArray2D<f64, 9>,
    RVEC: StackArray<f64, 2>,
    SAVBUF: ActualArray<f64>,
    T: f64,
    TC: f64,
    TIPM: StackArray2D<f64, 9>,
    TIPM2: StackArray2D<f64, 9>,
    TJ2E: StackArray2D<f64, 36>,
    TMPMAT: StackArray2D<f64, 9>,
    TMPXF: StackArray2D<f64, 36>,
    TSIPM: StackArray2D<f64, 36>,
    TSIPM2: StackArray2D<f64, 36>,
    W: f64,
    XDEC: f64,
    XDTIPM: StackArray2D<f64, 9>,
    XLMBDA: f64,
    XRA: f64,
    XRAD: StackArray<f64, 3>,
    XTIPM: StackArray2D<f64, 9>,
    XTSIPM: StackArray2D<f64, 36>,
    XW: f64,
    ZMAT: StackArray2D<f64, 9>,
    ANGSET: ActualArray<i32>,
    CODE: i32,
    H: i32,
    I: i32,
    J: i32,
    K: i32,
    N: i32,
    NCOEFF: i32,
    RADSET: ActualArray<i32>,
    REFID: i32,
    XSET: ActualArray<i32>,
    FOUND: bool,
    MODOK: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DTYPE = vec![b' '; 1 as usize];
        let mut LINE = vec![b' '; LNSIZE as usize];
        let mut IDSTR = vec![b' '; NUMLEN as usize];
        let mut KVDEG = vec![b' '; KVNMLN as usize];
        let mut KVNAME = vec![b' '; KVNMLN as usize];
        let mut KVPHAS = vec![b' '; KVNMLN as usize];
        let mut KVNAMS = ActualCharArray::new(KVNMLN, LBCELL..=MAXSET);
        let mut PCK = vec![b' '; FILSIZ as usize];
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut XNAME = vec![b' '; KVNMLN as usize];
        let mut PHSBUF = ActualArray::<f64>::new(1..=MAXBUF);
        let mut B2J = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut CMAT = StackArray2D::<f64, 4>::new(1..=2, 1..=2);
        let mut DEC: f64 = 0.0;
        let mut DELTA: f64 = 0.0;
        let mut DETM: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut EUL0 = StackArray::<f64, 3>::new(1..=3);
        let mut EUL2 = StackArray::<f64, 3>::new(1..=3);
        let mut EULSTA = StackArray::<f64, 6>::new(1..=6);
        let mut J2B = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut LAMBDA: f64 = 0.0;
        let mut MI = StackArray2D::<f64, 4>::new(1..=2, 1..=2);
        let mut RA: f64 = 0.0;
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RJ2E = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut RVEC = StackArray::<f64, 2>::new(1..=2);
        let mut SAVBUF = ActualArray::<f64>::new(1..=MAXBUF);
        let mut T: f64 = 0.0;
        let mut TC: f64 = 0.0;
        let mut TIPM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut TIPM2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut TJ2E = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut TMPMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut TMPXF = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut TSIPM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut TSIPM2 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut W: f64 = 0.0;
        let mut XDEC: f64 = 0.0;
        let mut XDTIPM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XLMBDA: f64 = 0.0;
        let mut XRA: f64 = 0.0;
        let mut XRAD = StackArray::<f64, 3>::new(1..=3);
        let mut XTIPM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XTSIPM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut XW: f64 = 0.0;
        let mut ZMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut ANGSET = ActualArray::<i32>::new(LBCELL..=MAXSET);
        let mut CODE: i32 = 0;
        let mut H: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut N: i32 = 0;
        let mut NCOEFF: i32 = 0;
        let mut RADSET = ActualArray::<i32>::new(LBCELL..=MAXSET);
        let mut REFID: i32 = 0;
        let mut XSET = ActualArray::<i32>::new(LBCELL..=MAXSET);
        let mut FOUND: bool = false;
        let mut MODOK: bool = false;

        Self {
            DTYPE,
            LINE,
            IDSTR,
            KVDEG,
            KVNAME,
            KVPHAS,
            KVNAMS,
            PCK,
            QNAME,
            TITLE,
            XNAME,
            PHSBUF,
            B2J,
            CMAT,
            DEC,
            DELTA,
            DETM,
            ET,
            EUL0,
            EUL2,
            EULSTA,
            J2B,
            LAMBDA,
            MI,
            RA,
            RADII,
            RJ2E,
            RVEC,
            SAVBUF,
            T,
            TC,
            TIPM,
            TIPM2,
            TJ2E,
            TMPMAT,
            TMPXF,
            TSIPM,
            TSIPM2,
            W,
            XDEC,
            XDTIPM,
            XLMBDA,
            XRA,
            XRAD,
            XTIPM,
            XTSIPM,
            XW,
            ZMAT,
            ANGSET,
            CODE,
            H,
            I,
            J,
            K,
            N,
            NCOEFF,
            RADSET,
            REFID,
            XSET,
            FOUND,
            MODOK,
        }
    }
}

//$Procedure      F_RDPCKD ( Text PCK reader tests )
pub fn F_RDPCKD(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Maximum number of phase angles
    //

    //
    // Maximum phase angle degree
    //

    //
    // Size of phase angle coefficient buffer
    //

    //
    // Local variables
    //

    //
    // Save all variables.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_RDPCKD", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup:  create full text PCK file.", ctx)?;

    fstr::assign(&mut save.PCK, b"test_0011d.tpc");

    if spicelib::EXISTS(&save.PCK, ctx)? {
        spicelib::DELFIL(&save.PCK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the PCK file, load it, and delete it.
    //
    testutil::T_PCK11D(&save.PCK, true, true, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(&save.PCK, ctx)?;

    // ***************************************************************
    //
    //     Rotation tests
    //
    // ***************************************************************

    //
    //     Check the set of bodies for which rotation data are provided.
    //     Find the set of kernel variables for prime meridian coefficients;
    //     check the corresponding set of ID codes against the set
    //     from the test utility T_PCKASTD.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check the set of bodies for which rotation data are provided.",
        ctx,
    )?;

    spicelib::SSIZEI(MAXSET, save.XSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXSET, save.ANGSET.as_slice_mut(), ctx)?;

    testutil::T_PCKASTD(save.XSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GNPOOL(
        b"BODY*_PM",
        1,
        MAXSET,
        &mut save.N,
        save.KVNAMS.subarray_mut(1),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VALIDC(MAXSET, save.N, save.KVNAMS.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Remove all of the nutation precession PM names.
    //
    save.I = 1;

    while (save.I <= spicelib::CARDC(save.KVNAMS.as_arg(), ctx)?) {
        if (intrinsics::INDEX(&save.KVNAMS[save.I], b"NUT_PREC") > 0) {
            spicelib::REMOVC(&save.KVNAMS[save.I].to_vec(), save.KVNAMS.as_arg_mut(), ctx)?;
        } else {
            save.I = (save.I + 1);
        }
    }

    testutil::CHCKSL(b"PM names found", save.FOUND, true, OK, ctx)?;

    save.N = spicelib::CARDC(save.KVNAMS.as_arg(), ctx)?;

    testutil::CHCKSI(
        b"Number of PM names",
        save.N,
        b"=",
        spicelib::CARDI(save.XSET.as_slice(), ctx)?,
        0,
        OK,
        ctx,
    )?;

    //
    //     Check each name in the expected set for presence in the
    //     actual name set.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check bodies in set expected to have rotation data against those present in the kernel.",
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDI(save.XSET.as_slice(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.XNAME, b"BODY#_PM");

            spicelib::REPMI(
                &save.XNAME.to_vec(),
                b"#",
                save.XSET[save.I],
                &mut save.XNAME,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Make sure the expected name is in the kernel
            //    variable list.
            //
            save.J = spicelib::BSRCHC(&save.XNAME, save.N, save.KVNAMS.subarray(1));

            fstr::assign(&mut save.QNAME, b"# is present in kernel?");
            spicelib::REPMC(&save.QNAME.to_vec(), b"#", &save.XNAME, &mut save.QNAME);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.QNAME, (save.J > 0), true, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check bodies having rotation data in kernel against those in the expected set.",
        ctx,
    )?;

    //
    // Check each name in the actual set for presence in the
    // expected name set.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDC(save.KVNAMS.as_arg(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Get the ID code of the current name.
            //
            fstr::assign(&mut save.IDSTR, fstr::substr(save.KVNAMS.get(save.I), 5..));

            save.J = intrinsics::INDEX(&save.IDSTR, b"_");

            spicelib::PRSINT(
                fstr::substr(&save.IDSTR, 1..=(save.J - 1)),
                &mut save.CODE,
                ctx,
            )?;

            fstr::assign(&mut save.QNAME, b"# is present in expected set?");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.CODE, &mut save.QNAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(
                &save.QNAME,
                spicelib::ELEMI(save.CODE, save.XSET.as_slice(), ctx)?,
                true,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Since we've checked the set of bodies, let the actual
    // set be the expected set.
    //
    spicelib::COPYI(save.XSET.as_slice(), save.ANGSET.as_slice_mut(), ctx)?;

    //
    // Delete body -10 from the set used for the normal test cases.
    //
    spicelib::REMOVI(-10, save.ANGSET.as_slice_mut(), ctx)?;

    //
    // Perform BODEUL tests.
    //
    // A note concerning the looser tolerance value MEDIUM used
    // in these tests:  converting Euler angles to rotation matrices
    // involves applying the SIN and COS functions to the angles.
    // In the case of the prime meridian angle W, applying trig
    // functions increases the relative error, since W may start
    // out with magnitude much larger than 2*pi.  The error in W
    // is the largest source of error in the matrix TIPM.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDI(save.ANGSET.as_slice(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for XT in -5..=5 {
                //
                // --- Case: ------------------------------------------------------
                //
                save.ET = (((XT * 10) as f64) * spicelib::JYEAR());

                fstr::assign(
                    &mut save.LINE,
                    b"BODEUL test: Check Euler angles for # at ET #.",
                );

                spicelib::REPMI(
                    &save.LINE.to_vec(),
                    b"#",
                    save.ANGSET[save.I],
                    &mut save.LINE,
                    ctx,
                );
                spicelib::REPMD(&save.LINE.to_vec(), b"#", save.ET, 14, &mut save.LINE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&save.LINE, ctx)?;

                testutil::T_PCKANGD(
                    save.ANGSET[save.I],
                    save.ET,
                    &mut save.XRA,
                    &mut save.XDEC,
                    &mut save.XW,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.XW = intrinsics::DMOD(save.XW, spicelib::TWOPI(ctx));

                if (save.XW < 0 as f64) {
                    save.XW = (save.XW + spicelib::TWOPI(ctx));
                }

                save.XRA = intrinsics::DMOD(save.XRA, spicelib::TWOPI(ctx));

                if (save.XRA < 0 as f64) {
                    save.XRA = (save.XRA + spicelib::TWOPI(ctx));
                }

                spicelib::BODEUL(
                    save.ANGSET[save.I],
                    save.ET,
                    &mut save.RA,
                    &mut save.DEC,
                    &mut save.W,
                    &mut save.LAMBDA,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSD(b"RA", save.RA, b"~", save.XRA, TIGHT, OK, ctx)?;
                testutil::CHCKSD(b"DEC", save.DEC, b"~", save.XDEC, TIGHT, OK, ctx)?;
                testutil::CHCKSD(b"W", save.W, b"~", save.XW, MEDIUM, OK, ctx)?;
            }

            save.I += m3__;
        }
    }

    //
    // Perform analogous tests on both BODMAT and TIPBOD. In order to
    // compare results against those from T_PCKANGD, we represent results
    // from the latter routine as rotation matrices.
    //
    // Obtain the position and state transformations from the J2000
    // to the ECLIPJ2000 frame.  These will be used later.
    //
    spicelib::PXFORM(b"J2000", b"ECLIPJ2000", 0.0, save.RJ2E.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(b"J2000", b"ECLIPJ2000", 0.0, save.TJ2E.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDI(save.ANGSET.as_slice(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for XT in -5..=5 {
                //
                // --- Case: ------------------------------------------------------
                //
                save.ET = (((XT * 10) as f64) * spicelib::JYEAR());

                fstr::assign(
                    &mut save.LINE,
                    b"BODMAT test: Check attitude matrix for # at ET #.",
                );

                spicelib::REPMI(
                    &save.LINE.to_vec(),
                    b"#",
                    save.ANGSET[save.I],
                    &mut save.LINE,
                    ctx,
                );
                spicelib::REPMD(&save.LINE.to_vec(), b"#", save.ET, 14, &mut save.LINE, ctx);
                testutil::TCASE(&save.LINE, ctx)?;

                testutil::T_PCKANGD(
                    save.ANGSET[save.I],
                    save.ET,
                    &mut save.XRA,
                    &mut save.XDEC,
                    &mut save.XW,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::EUL2M(
                    save.XW,
                    (spicelib::HALFPI(ctx) - save.XDEC),
                    (spicelib::HALFPI(ctx) + save.XRA),
                    3,
                    1,
                    3,
                    save.XTIPM.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Test BODMAT.
                //
                spicelib::BODMAT(save.ANGSET[save.I], save.ET, save.TIPM.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"TIPM",
                    save.TIPM.as_slice(),
                    b"~",
                    save.XTIPM.as_slice(),
                    9,
                    MEDIUM,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: ------------------------------------------------------
                //
                //
                //              Test TIPBOD.
                //
                fstr::assign(
                    &mut save.LINE,
                    b"TIPBOD test: Check attitude matrix for # at ET #.",
                );
                spicelib::REPMI(
                    &save.LINE.to_vec(),
                    b"#",
                    save.ANGSET[save.I],
                    &mut save.LINE,
                    ctx,
                );
                spicelib::REPMD(&save.LINE.to_vec(), b"#", save.ET, 14, &mut save.LINE, ctx);
                testutil::TCASE(&save.LINE, ctx)?;

                spicelib::TIPBOD(
                    b"J2000",
                    save.ANGSET[save.I],
                    save.ET,
                    save.TIPM.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"TIPM",
                    save.TIPM.as_slice(),
                    b"~",
                    save.XTIPM.as_slice(),
                    9,
                    MEDIUM,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: ------------------------------------------------------
                //
                //
                //              Repeat the TIPBOD test using a non-native inertial
                //              frame. Right-multiply the matrix obtained from TIPBOD by
                //              the mapping from J2000 to the non-native frame; this
                //              should give us the same transformation we'd have had if
                //              we had looked up the mapping from J2000 to the
                //              body-fixed frame directly.
                //
                spicelib::TIPBOD(
                    b"ECLIPJ2000",
                    save.ANGSET[save.I],
                    save.ET,
                    save.TIPM.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXM(
                    save.TIPM.as_slice(),
                    save.RJ2E.as_slice(),
                    save.TMPMAT.as_slice_mut(),
                );
                spicelib::MOVED(save.TMPMAT.as_slice(), 9, save.TIPM.as_slice_mut());

                testutil::CHCKAD(
                    b"TIPM",
                    save.TIPM.as_slice(),
                    b"~",
                    save.XTIPM.as_slice(),
                    9,
                    MEDIUM,
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    // Perform analogous tests on TISBOD.  In order to compare
    // results against those from T_PCKANGD, we represent results
    // from the latter routine as rotation matrices.
    //
    // We use discrete derivatives of the Euler angles to examine the
    // derivative portion of the state transformation matrix.
    //
    spicelib::CLEARD(9, save.ZMAT.as_slice_mut());

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDI(save.ANGSET.as_slice(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            for XT in -5..=5 {
                //
                // --- Case: ------------------------------------------------------
                //
                save.ET = (((XT * 10) as f64) * spicelib::JYEAR());

                fstr::assign(
                    &mut save.LINE,
                    b"TISBOD: check state transformation matrix for # at ET #.",
                );

                spicelib::REPMI(
                    &save.LINE.to_vec(),
                    b"#",
                    save.ANGSET[save.I],
                    &mut save.LINE,
                    ctx,
                );
                spicelib::REPMD(&save.LINE.to_vec(), b"#", save.ET, 14, &mut save.LINE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&save.LINE, ctx)?;

                testutil::T_PCKANGD(
                    save.ANGSET[save.I],
                    save.ET,
                    &mut save.XRA,
                    &mut save.XDEC,
                    &mut save.XW,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::EUL2M(
                    save.XW,
                    (spicelib::HALFPI(ctx) - save.XDEC),
                    (spicelib::HALFPI(ctx) + save.XRA),
                    3,
                    1,
                    3,
                    save.XTIPM.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::TISBOD(
                    b"J2000",
                    save.ANGSET[save.I],
                    save.ET,
                    save.TSIPM.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // We test the blocks of the state transformation matrix
                // separately, since we don't use identical tolerances for
                // all blocks.
                //
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = 3;
                    let m3__: i32 = 1;
                    save.J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        {
                            let m1__: i32 = 1;
                            let m2__: i32 = 3;
                            let m3__: i32 = 1;
                            save.K = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                save.TIPM[[save.J, save.K]] = save.TSIPM[[save.J, save.K]];
                                save.K += m3__;
                            }
                        }
                        save.J += m3__;
                    }
                }

                testutil::CHCKAD(
                    b"TSIPM (upper left block)",
                    save.TIPM.as_slice(),
                    b"~",
                    save.XTIPM.as_slice(),
                    9,
                    MEDIUM,
                    OK,
                    ctx,
                )?;

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = 3;
                    let m3__: i32 = 1;
                    save.J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        {
                            let m1__: i32 = 1;
                            let m2__: i32 = 3;
                            let m3__: i32 = 1;
                            save.K = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                save.TIPM[[save.J, save.K]] =
                                    save.TSIPM[[(save.J + 3), (save.K + 3)]];
                                save.K += m3__;
                            }
                        }
                        save.J += m3__;
                    }
                }

                testutil::CHCKAD(
                    b"TSIPM (lower right block)",
                    save.TIPM.as_slice(),
                    b"~",
                    save.XTIPM.as_slice(),
                    9,
                    MEDIUM,
                    OK,
                    ctx,
                )?;

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = 3;
                    let m3__: i32 = 1;
                    save.J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        {
                            let m1__: i32 = 1;
                            let m2__: i32 = 3;
                            let m3__: i32 = 1;
                            save.K = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                save.TMPMAT[[save.J, save.K]] = save.TSIPM[[save.J, (save.K + 3)]];
                                save.K += m3__;
                            }
                        }
                        save.J += m3__;
                    }
                }

                testutil::CHCKAD(
                    b"TSIPM (upper right block)",
                    save.TMPMAT.as_slice(),
                    b"~",
                    save.ZMAT.as_slice(),
                    9,
                    MEDIUM,
                    OK,
                    ctx,
                )?;

                //
                // To test the derivative block, we'll first estimate the
                // derivatives of the Euler angles using a quadratic
                // approximation.  We'll sample the angles at +/- 1 second
                // from ET.
                //
                save.DELTA = 1.0;
                //
                // Get the left side angles...
                //
                testutil::T_PCKANGD(
                    save.ANGSET[save.I],
                    (save.ET - save.DELTA),
                    &mut save.XRA,
                    &mut save.XDEC,
                    &mut save.XW,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.EUL0[1] = save.XW;
                save.EUL0[2] = (spicelib::HALFPI(ctx) - save.XDEC);
                save.EUL0[3] = (spicelib::HALFPI(ctx) + save.XRA);

                //
                // The right side angles...
                //
                testutil::T_PCKANGD(
                    save.ANGSET[save.I],
                    (save.ET + save.DELTA),
                    &mut save.XRA,
                    &mut save.XDEC,
                    &mut save.XW,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.EUL2[1] = save.XW;
                save.EUL2[2] = (spicelib::HALFPI(ctx) - save.XDEC);
                save.EUL2[3] = (spicelib::HALFPI(ctx) + save.XRA);

                //
                // Find the derivatives.
                //
                spicelib::QDERIV(
                    3,
                    save.EUL0.as_slice(),
                    save.EUL2.as_slice(),
                    save.DELTA,
                    save.EULSTA.subarray_mut(4),
                    ctx,
                )?;

                //
                // Complete the Euler angle state vector by filling in the
                // first three components with the angles at ET.
                //
                testutil::T_PCKANGD(
                    save.ANGSET[save.I],
                    save.ET,
                    &mut save.XRA,
                    &mut save.XDEC,
                    &mut save.XW,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::VPACK(
                    save.XW,
                    (spicelib::HALFPI(ctx) - save.XDEC),
                    (spicelib::HALFPI(ctx) + save.XRA),
                    save.EULSTA.as_slice_mut(),
                );

                //
                // Now find the expected state transformation at ET.
                //
                spicelib::EUL2XF(
                    save.EULSTA.as_slice(),
                    3,
                    1,
                    3,
                    save.XTSIPM.as_slice_mut(),
                    ctx,
                )?;

                //
                // Extract the derivative block from the expected state
                // transformation.
                //
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = 3;
                    let m3__: i32 = 1;
                    save.J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        {
                            let m1__: i32 = 1;
                            let m2__: i32 = 3;
                            let m3__: i32 = 1;
                            save.K = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                save.XDTIPM[[save.J, save.K]] = save.XTSIPM[[(save.J + 3), save.K]];
                                save.K += m3__;
                            }
                        }
                        save.J += m3__;
                    }
                }

                //
                // Do the same for the transformation returned from TISBOD.
                //
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = 3;
                    let m3__: i32 = 1;
                    save.J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        {
                            let m1__: i32 = 1;
                            let m2__: i32 = 3;
                            let m3__: i32 = 1;
                            save.K = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                save.TMPMAT[[save.J, save.K]] = save.TSIPM[[(save.J + 3), save.K]];
                                save.K += m3__;
                            }
                        }
                        save.J += m3__;
                    }
                }

                testutil::CHCKAD(
                    b"TSIPM derivative (lower left block)",
                    save.TMPMAT.as_slice(),
                    b"~",
                    save.XDTIPM.as_slice(),
                    9,
                    MEDIUM,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: ------------------------------------------------------
                //
                //
                //              Repeat the TISBOD test using a non-native input frame.
                //
                fstr::assign(&mut save.LINE, b"TISBOD: check state transformation matrix for # at ET #.  Use ECLIPJ2000 as input frame.");

                spicelib::REPMI(
                    &save.LINE.to_vec(),
                    b"#",
                    save.ANGSET[save.I],
                    &mut save.LINE,
                    ctx,
                );
                spicelib::REPMD(&save.LINE.to_vec(), b"#", save.ET, 14, &mut save.LINE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&save.LINE, ctx)?;
                spicelib::TISBOD(
                    b"ECLIPJ2000",
                    save.ANGSET[save.I],
                    save.ET,
                    save.TSIPM2.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXMG(
                    save.TSIPM2.as_slice(),
                    save.TJ2E.as_slice(),
                    6,
                    6,
                    6,
                    save.TMPXF.as_slice_mut(),
                );
                spicelib::MOVED(save.TMPXF.as_slice(), 36, save.TSIPM2.as_slice_mut());

                //
                // Compare against TSIPM from the previous test.
                //
                testutil::CHCKAD(
                    b"TSIPM2",
                    save.TSIPM2.as_slice(),
                    b"~",
                    save.TSIPM.as_slice(),
                    36,
                    MEDIUM,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: ------------------------------------------------------
                //
                //
                //              Repeat the TISBOD test using the max phase angle
                //              keyword. This test exercises the logic for recognition
                //              of the keyword, but not that for higher-order phase
                //              angle expressions.
                //
                fstr::assign(&mut save.LINE, b"TISBOD: check state transformation matrix for # at ET #.  Use ECLIPJ2000 as input frame. Set max phase angle degree via keyword.");

                spicelib::REPMI(
                    &save.LINE.to_vec(),
                    b"#",
                    save.ANGSET[save.I],
                    &mut save.LINE,
                    ctx,
                );
                spicelib::REPMD(&save.LINE.to_vec(), b"#", save.ET, 14, &mut save.LINE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&save.LINE, ctx)?;

                //
                // Insert the keyword for BODY#_MAX_PHASE_DEGREE for this
                // body into the kernel pool.
                //
                spicelib::REPMI(
                    b"BODY#_MAX_PHASE_DEGREE",
                    b"#",
                    save.ANGSET[save.I],
                    &mut save.KVNAME,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Set the maximum degree to the default, which is 1.
                //
                spicelib::PIPOOL(&save.KVNAME, 1, &[1], ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::TISBOD(
                    b"ECLIPJ2000",
                    save.ANGSET[save.I],
                    save.ET,
                    save.TSIPM2.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXMG(
                    save.TSIPM2.as_slice(),
                    save.TJ2E.as_slice(),
                    6,
                    6,
                    6,
                    save.TMPXF.as_slice_mut(),
                );
                spicelib::MOVED(save.TMPXF.as_slice(), 36, save.TSIPM2.as_slice_mut());

                //
                // Compare against TSIPM from the previous test.
                //
                testutil::CHCKAD(
                    b"TSIPM2",
                    save.TSIPM2.as_slice(),
                    b"~",
                    save.TSIPM.as_slice(),
                    36,
                    MEDIUM,
                    OK,
                    ctx,
                )?;

                //
                // Delete the phase angle degree keyword.
                //
                spicelib::DVPOOL(&save.KVNAME, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // --- Case: ------------------------------------------------------
                //
                //
                //              Repeat the TISBOD test using the max phase angle
                //              keyword. In this test, we'll modify the phase angle
                //              coefficients so as to produce cubic polynomials that
                //              yield the same angles and angular rates as the originals
                //              for the specific, known input time.
                //
                //              We'll do this test only for bodies for which the phase
                //              angle degree is 1.
                //
                //              We're going to go to some trouble to create the
                //              polynomial coefficients, so in this test case we'll
                //              test BODEUL using them as well.
                //
                fstr::assign(&mut save.LINE, b"TISBOD and BODEUL: check state transformation matrix and Euler angles for # at ET #. Use J2000 as input frame. Generate cubic phase angle polynomials and use those instead of the originals.");

                spicelib::REPMI(
                    &save.LINE.to_vec(),
                    b"#",
                    save.ANGSET[save.I],
                    &mut save.LINE,
                    ctx,
                );
                spicelib::REPMD(&save.LINE.to_vec(), b"#", save.ET, 14, &mut save.LINE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&save.LINE, ctx)?;

                //
                // Get the expected results.
                //
                spicelib::TISBOD(
                    b"J2000",
                    save.ANGSET[save.I],
                    save.ET,
                    save.TSIPM.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::TIPBOD(
                    b"J2000",
                    save.ANGSET[save.I],
                    save.ET,
                    save.TIPM.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::BODEUL(
                    save.ANGSET[save.I],
                    save.ET,
                    &mut save.XRA,
                    &mut save.XDEC,
                    &mut save.XW,
                    &mut save.XLMBDA,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Perform this modification only for angle sets that
                // don't have the degree keyword.
                //
                // Get the body ID associated with the phase angles, if
                // there is one.
                //
                save.REFID = spicelib::ZZBODBRY(save.ANGSET[save.I]);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.MODOK = (spicelib::BODFND(save.REFID, b"NUT_PREC_ANGLES", ctx)?
                    && !spicelib::BODFND(save.REFID, b"MAX_PHASE_DEGREE", ctx)?);

                if save.MODOK {
                    //
                    // Fetch the original phase angle coefficients.
                    //
                    fstr::assign(&mut save.KVPHAS, b"BODY#_NUT_PREC_ANGLES");
                    spicelib::REPMI(
                        &save.KVPHAS.to_vec(),
                        b"#",
                        save.REFID,
                        &mut save.KVPHAS,
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::GDPOOL(
                        &save.KVPHAS,
                        1,
                        (MAXBUF / 2),
                        &mut save.N,
                        save.SAVBUF.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
                    //
                    // Insert the keyword for BODY#_MAX_PHASE_DEGREE for
                    // this body into the kernel pool, setting the phase
                    // angle polynomial degree to 3.

                    spicelib::REPMI(
                        b"BODY#_MAX_PHASE_DEGREE",
                        b"#",
                        save.REFID,
                        &mut save.KVDEG,
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::PIPOOL(&save.KVDEG, 1, &[3], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Each original phase angle polynomial has the form
                    //
                    //    A  +  TC*B
                    //
                    // where TC is ET in units of Julian centuries.
                    // Given TC, we can create coefficients that define
                    // a cubic polynomial that has the same value and
                    // derivative with respect to ET, at TC. We'll choose
                    // the coefficients
                    //
                    //    A2, B2, C2, D2
                    //
                    // where if TC is 0,
                    //
                    //    A2 = A
                    //    B2 = B
                    //    C2 = 0
                    //    D2 = 0
                    //
                    // and otherwise
                    //
                    //    A2 = A
                    //    B2 = B/2
                    //
                    //    C2, D2 are chosen so that the polynomial value
                    //    and derivative wrt. ET match, i.e.
                    //
                    //       C2 * TC**2    + D2 * TC**3         =  TC * B/2
                    //       C2 * 2 * TC/T + D2 * 3 * (TC/T)**2 = (B/2) / T
                    //
                    //   where TC = ET / T.
                    //
                    //   We solve for C2 and D2 by treating this as a matrix
                    //   equation:
                    //
                    //     ._         _.
                    //     |  M11  M12 |       | C2 |     | R1 |
                    //     |           |   *   |    |  =  |    |
                    //     |  M21  M22 |       | D2 |     | R2 |
                    //     `-         -'
                    //
                    //   where
                    //
                    //      M11 = TC**2
                    //      M12 = TC**3
                    //      M21 = 2 * TC/T
                    //      M22 = 3 * (TC/T)**2
                    //
                    //      R1  = TC * B/2
                    //      R2  = (B/2) / T
                    //
                    //   Then multiplying both sides of the equation by
                    //   the matrix inverse (provided it exists) yields
                    //   the solution.
                    //
                    // We'll store the new coefficients in PHSBUF.
                    //
                    save.T = ((100 as f64) * spicelib::JYEAR());
                    save.TC = (save.ET / save.T);
                    save.NCOEFF = (save.N / 2);

                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = save.NCOEFF;
                        let m3__: i32 = 1;
                        save.J = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            save.H = ((2 * (save.J - 1)) + 1);
                            save.K = ((4 * (save.J - 1)) + 1);
                            //
                            // Set A2.
                            //
                            save.PHSBUF[save.K] = save.SAVBUF[save.H];
                            //
                            // Set B2, C2 and D2.
                            //
                            if (save.TC == 0.0) {
                                save.PHSBUF[(save.K + 1)] = save.SAVBUF[(save.H + 1)];
                                save.PHSBUF[(save.K + 2)] = 0.0;
                                save.PHSBUF[(save.K + 3)] = 0.0;
                            } else {
                                save.PHSBUF[(save.K + 1)] = (save.SAVBUF[(save.H + 1)] * 0.5);

                                save.CMAT[[1, 1]] = f64::powi(save.TC, 2);
                                save.CMAT[[1, 2]] = f64::powi(save.TC, 3);
                                save.CMAT[[2, 1]] = (((2 as f64) * save.TC) / save.T);
                                save.CMAT[[2, 2]] = ((3 as f64) * f64::powi((save.TC / save.T), 2));

                                save.RVEC[1] = ((save.TC * save.SAVBUF[(save.H + 1)]) / 2 as f64);
                                save.RVEC[2] = ((save.SAVBUF[(save.H + 1)] / 2 as f64) / save.T);

                                save.DETM = ((save.CMAT[[1, 1]] * save.CMAT[[2, 2]])
                                    - (save.CMAT[[1, 2]] * save.CMAT[[2, 1]]));

                                if (save.DETM == 0.0) {
                                    //
                                    // It's very unlikely we arrive here, but we
                                    // can't continue if we do. Force the test
                                    // system to catch the error.
                                    //
                                    spicelib::SETMSG(
                                        b"Cubic polynomial solution matrix is singular.",
                                        ctx,
                                    );
                                    spicelib::SIGERR(b"SPICE(TESTERROR)", ctx)?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                                } else {
                                    //
                                    // Let MI be the inverse of the matrix M above.
                                    // Use the 2x2 matrix inverse formula.
                                    //
                                    save.MI[[1, 1]] = (save.CMAT[[2, 2]] / save.DETM);
                                    save.MI[[1, 2]] = -(save.CMAT[[1, 2]] / save.DETM);
                                    save.MI[[2, 1]] = -(save.CMAT[[2, 1]] / save.DETM);
                                    save.MI[[2, 2]] = (save.CMAT[[1, 1]] / save.DETM);

                                    spicelib::MXVG(
                                        save.MI.as_slice(),
                                        save.RVEC.as_slice(),
                                        2,
                                        2,
                                        save.PHSBUF.subarray_mut((save.K + 2)),
                                    );
                                }
                            }

                            save.J += m3__;
                        }
                    }

                    //
                    // Insert the new coefficients into the kernel pool.
                    //
                    spicelib::PDPOOL(&save.KVPHAS, (4 * save.NCOEFF), save.PHSBUF.as_slice(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::TISBOD(
                        b"J2000",
                        save.ANGSET[save.I],
                        save.ET,
                        save.TSIPM2.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::TIPBOD(
                        b"J2000",
                        save.ANGSET[save.I],
                        save.ET,
                        save.TIPM2.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Compare against TIPM and TSIPM computed using the
                    // original phase angle coefficients.
                    //
                    testutil::CHCKAD(
                        b"TIPM2",
                        save.TIPM2.as_slice(),
                        b"~",
                        save.TIPM.as_slice(),
                        9,
                        VTIGHT,
                        OK,
                        ctx,
                    )?;

                    testutil::CHCKAD(
                        b"TSIPM2",
                        save.TSIPM2.as_slice(),
                        b"~",
                        save.TSIPM.as_slice(),
                        36,
                        TIGHT,
                        OK,
                        ctx,
                    )?;

                    //
                    // Check angles from BODEUL against expected values.
                    //
                    spicelib::BODEUL(
                        save.ANGSET[save.I],
                        save.ET,
                        &mut save.RA,
                        &mut save.DEC,
                        &mut save.W,
                        &mut save.LAMBDA,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSD(b"RA", save.RA, b"~", save.XRA, VTIGHT, OK, ctx)?;
                    testutil::CHCKSD(b"DEC", save.DEC, b"~", save.XDEC, VTIGHT, OK, ctx)?;
                    testutil::CHCKSD(b"W", save.W, b"~", save.XW, VTIGHT, OK, ctx)?;

                    //
                    // LAMBDA is not affected by the phase angles. We
                    // should get an exact match.
                    //
                    testutil::CHCKSD(b"LAMBDA", save.LAMBDA, b"~", save.XLMBDA, 0.0, OK, ctx)?;

                    //
                    // Restore the original phase angle coefficients.
                    //
                    spicelib::PDPOOL(&save.KVPHAS, (2 * save.NCOEFF), save.SAVBUF.as_slice(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Delete the phase angle degree keyword. Note that
                    // this step is valid only if the keyword wasn't
                    // present to begin with.
                    //
                    spicelib::DVPOOL(&save.KVDEG, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }
            }
            //
            //    We've tested TISBOD for the current time value.
            //
            //
            // We've tested TISBOD for the current body.
            //
            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"TISBOD error case: invalid phase degree", ctx)?;

    //
    // Insert the keyword for BODY#_MAX_PHASE_DEG for this
    // body into the kernel pool.
    //
    // For this test to work, the barycenter must be one for which
    // there are associated phase angles.
    //
    save.REFID = 3;

    spicelib::REPMI(
        b"BODY#_MAX_PHASE_DEGREE",
        b"#",
        save.REFID,
        &mut save.KVDEG,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(&save.KVDEG, 1, &[4], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TISBOD(b"J2000", 399, 0.0, save.TSIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(DEGREEOUTOFRANGE)", OK, ctx)?;

    spicelib::PIPOOL(&save.KVDEG, 1, &[0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TISBOD(b"J2000", 399, 0.0, save.TSIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(DEGREEOUTOFRANGE)", OK, ctx)?;

    spicelib::DVPOOL(&save.KVDEG, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Now check TIPBOD'S ability to deal with non-standard epochs and
    //     frames.  Look up data for body -10 (these data are created by the
    //     test utility T_PCK10) at the B1950 epoch in the B1950 frame.
    //     Object -10 uses the same constants as those for the sun, but for
    //     body -10, the native frame is B1950 and the reference epoch is
    //     B1950.  The output should match the standard numbers for the sun.
    //
    //     NOTE: the current implementation of TIPBOD (as of N0067) relies
    //     on TISBOD, so these checks are not strictly necessary at this
    //     time. They may become necessary later.
    //
    //
    testutil::TCASE(b"TIPBOD lookup of matrix for body -10", ctx)?;

    save.ET = ((spicelib::B1950() - spicelib::J2000()) * spicelib::SPD());

    spicelib::BODMAT(10, 0.0, save.XTIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TIPBOD(b"B1950", -10, save.ET, save.TIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"TIPM",
        save.TIPM.as_slice(),
        b"~",
        save.XTIPM.as_slice(),
        9,
        MEDIUM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"TIPBOD error case: lookup of matrix for body -10 when both *CONSTANTS_JED_EPOCH and *CONSTS_JED_EPOCH keywords are present.", ctx)?;

    spicelib::DTPOOL(
        b"BODY-10_CONSTANTS_JED_EPOCH",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTYPE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::PDPOOL(b"BODY-10_CONSTS_JED_EPOCH", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TIPBOD(b"B1950", -10, save.ET, save.TIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGEPOCHSPEC)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"TIPBOD error case: lookup of matrix for body -10 when both *_CONSTANTS_REF_FRAME and *CONSTS_REF_FRAME keywords are present.", ctx)?;

    //
    // Delete competing epoch spec first.
    //
    spicelib::DVPOOL(b"BODY-10_CONSTS_JED_EPOCH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DTPOOL(
        b"BODY-10_CONSTANTS_REF_FRAME",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTYPE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::PDPOOL(b"BODY-10_CONSTS_REF_FRAME", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TIPBOD(b"B1950", -10, save.ET, save.TIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGFRAMESPEC)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //     Perform a normal TIPBOD computation using the _CONSTS_ forms of
    //     the epoch and base frame keywords.
    //
    testutil::TCASE(b"TIPBOD: lookup B1950 to body-fixed transformation matrix for body -10 using both *CONSTS_JED_EPOCH and *CONSTS_REF_FRAME keywords.", ctx)?;

    //
    // Delete the competing keywords.
    //
    spicelib::DVPOOL(b"BODY-10_CONSTANTS_JED_EPOCH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY-10_CONSTANTS_REF_FRAME", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add the new keywords.
    //
    spicelib::PDPOOL(b"BODY-10_CONSTS_JED_EPOCH", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"BODY-10_CONSTS_REF_FRAME", 1, &[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the transformation from J2000 to IAU_SUN for body 10
    // at the J2000 epoch. This should match the transformation from
    // the B1950 frame to IAU_SUN for body -10 at the B1950 epoch,
    // since in both cases the input time is the epoch of the constants,
    // and both bodies have identical rotational elements.
    //
    save.ET = ((spicelib::B1950() - spicelib::J2000()) * spicelib::SPD());

    spicelib::BODMAT(10, 0.0, save.XTIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TIPBOD(b"B1950", -10, save.ET, save.TIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"TIPM",
        save.TIPM.as_slice(),
        b"~",
        save.XTIPM.as_slice(),
        9,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Repeat the above sequence of tests for TISBOD.
    //
    testutil::TCASE(b"TISBOD lookup of matrix for body -10", ctx)?;

    //
    // Delete the competing keywords.
    //
    spicelib::DVPOOL(b"BODY-10_CONSTS_JED_EPOCH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY-10_CONSTS_REF_FRAME", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore the old keywords.
    //
    spicelib::PDPOOL(b"BODY-10_CONSTANTS_JED_EPOCH", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"BODY-10_CONSTANTS_REF_FRAME", 1, &[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TISBOD(b"J2000", 10, 0.0, save.XTSIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = ((spicelib::B1950() - spicelib::J2000()) * spicelib::SPD());

    spicelib::TISBOD(b"B1950", -10, save.ET, save.TSIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"TSIPM",
        save.TSIPM.as_slice(),
        b"~",
        save.XTSIPM.as_slice(),
        36,
        TIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"TISBOD error case: lookup of matrix for body -10 when both *CONSTANTS_JED_EPOCH and *CONSTS_JED_EPOCH keywords are present.", ctx)?;

    spicelib::DTPOOL(
        b"BODY-10_CONSTANTS_JED_EPOCH",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTYPE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::PDPOOL(b"BODY-10_CONSTS_JED_EPOCH", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TISBOD(b"B1950", -10, save.ET, save.TSIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGEPOCHSPEC)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"TISBOD error case: lookup of matrix for body -10 when both *_CONSTANTS_REF_FRAME and *CONSTS_REF_FRAME keywords are present.", ctx)?;

    //
    // Delete competing epoch spec first.
    //
    spicelib::DVPOOL(b"BODY-10_CONSTS_JED_EPOCH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DTPOOL(
        b"BODY-10_CONSTANTS_REF_FRAME",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTYPE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::PDPOOL(b"BODY-10_CONSTS_REF_FRAME", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TISBOD(b"B1950", -10, save.ET, save.TSIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGFRAMESPEC)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //     Perform a normal TISBOD computation using the _CONSTS_ forms of
    //     the epoch and base frame keywords.
    //
    testutil::TCASE(b"TISBOD: lookup B1950 to body-fixed transformation matrix for body -10 using both *CONSTS_JED_EPOCH and *CONSTS_REF_FRAME keywords.", ctx)?;

    //
    // Delete the competing keywords.
    //
    spicelib::DVPOOL(b"BODY-10_CONSTANTS_JED_EPOCH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY-10_CONSTANTS_REF_FRAME", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add the new keywords.
    //
    spicelib::PDPOOL(b"BODY-10_CONSTS_JED_EPOCH", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"BODY-10_CONSTS_REF_FRAME", 1, &[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the transformation from J2000 to IAU_SUN for body 10
    // at the J2000 epoch. This should match the transformation from
    // the B1950 frame to IAU_SUN for body -10 at the B1950 epoch,
    // since in both cases the input time is the epoch of the constants,
    // and both bodies have identical rotational elements.
    //
    save.ET = ((spicelib::B1950() - spicelib::J2000()) * spicelib::SPD());

    spicelib::TISBOD(b"J2000", 10, 0.0, save.XTSIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TISBOD(b"B1950", -10, save.ET, save.TSIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"TSIPM",
        save.TSIPM.as_slice(),
        b"~",
        save.XTSIPM.as_slice(),
        36,
        MEDIUM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Repeat the above sequence of tests for BODMAT.
    //
    testutil::TCASE(b"BODMAT lookup of matrix for body -10", ctx)?;

    //
    // Delete the competing keywords.
    //
    spicelib::DVPOOL(b"BODY-10_CONSTS_JED_EPOCH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY-10_CONSTS_REF_FRAME", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore the old keywords.
    //
    spicelib::PDPOOL(b"BODY-10_CONSTANTS_JED_EPOCH", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"BODY-10_CONSTANTS_REF_FRAME", 1, &[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODMAT(10, 0.0, save.XTIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // The matrix we'll get for body -10 will be multiplied on the
    // right by the transformation from J2000 to B1950, so do the same
    // to the expected matrix.
    //
    spicelib::PXFORM(b"J2000", b"B1950", 0.0, save.J2B.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXM(
        save.XTIPM.as_slice(),
        save.J2B.as_slice(),
        save.TMPMAT.as_slice_mut(),
    );
    spicelib::MEQU(save.TMPMAT.as_slice(), save.XTIPM.as_slice_mut());

    save.ET = ((spicelib::B1950() - spicelib::J2000()) * spicelib::SPD());

    spicelib::BODMAT(-10, save.ET, save.TIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"TIPM",
        save.TIPM.as_slice(),
        b"~",
        save.XTIPM.as_slice(),
        9,
        TIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"BODMAT error case: lookup of matrix for body -10 when both *CONSTANTS_JED_EPOCH and *CONSTS_JED_EPOCH keywords are present.", ctx)?;

    spicelib::DTPOOL(
        b"BODY-10_CONSTANTS_JED_EPOCH",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTYPE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::PDPOOL(b"BODY-10_CONSTS_JED_EPOCH", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODMAT(-10, save.ET, save.TSIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGEPOCHSPEC)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"BODMAT error case: lookup of matrix for body -10 when both *_CONSTANTS_REF_FRAME and *CONSTS_REF_FRAME keywords are present.", ctx)?;

    //
    // Delete competing epoch spec first.
    //
    spicelib::DVPOOL(b"BODY-10_CONSTS_JED_EPOCH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DTPOOL(
        b"BODY-10_CONSTANTS_REF_FRAME",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTYPE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::PDPOOL(b"BODY-10_CONSTS_REF_FRAME", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODMAT(-10, save.ET, save.TSIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGFRAMESPEC)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //     Perform a normal BODMAT computation using the _CONSTS_ forms of
    //     the epoch and base frame keywords.
    //
    testutil::TCASE(b"BODMAT: lookup B1950 to body-fixed transformation matrix for body -10 using both *CONSTS_JED_EPOCH and *CONSTS_REF_FRAME keywords.", ctx)?;

    //
    // Delete the competing keywords.
    //
    spicelib::DVPOOL(b"BODY-10_CONSTANTS_JED_EPOCH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY-10_CONSTANTS_REF_FRAME", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add the new keywords.
    //
    spicelib::PDPOOL(b"BODY-10_CONSTS_JED_EPOCH", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"BODY-10_CONSTS_REF_FRAME", 1, &[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the transformation from J2000 to IAU_SUN for body 10
    // at the J2000 epoch. This should match the transformation from
    // the B1950 frame to IAU_SUN for body -10 at the B1950 epoch,
    // since in both cases the input time is the epoch of the constants,
    // and both bodies have identical rotational elements.
    //
    // The matrix we'll get for body -10 will be multiplied on the
    // right by the transformation from J2000 to B1950, so do the same
    // to the expected matrix.
    //
    spicelib::PXFORM(b"J2000", b"B1950", 0.0, save.J2B.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODMAT(10, 0.0, save.XTIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXM(
        save.XTIPM.as_slice(),
        save.J2B.as_slice(),
        save.TMPMAT.as_slice_mut(),
    );
    spicelib::MEQU(save.TMPMAT.as_slice(), save.XTIPM.as_slice_mut());

    save.ET = ((spicelib::B1950() - spicelib::J2000()) * spicelib::SPD());

    spicelib::BODMAT(-10, save.ET, save.TIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"TIPM",
        save.TIPM.as_slice(),
        b"~",
        save.XTIPM.as_slice(),
        9,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Now check BODEUL's ability to deal with non-standard epochs and
    //     frames.  Look up data for body -10 at the B1950 epoch in the
    //     B1950 frame.  Convert the resulting Euler angles to a matrix.
    //
    //     Since the numbers in the PCK match the standard
    //     numbers for the sun, applying the B1950-to-J2000 transformation
    //     to the output via right multiplication
    //     should yield the standard matrix for the sun.
    //
    testutil::TCASE(b"BODEUL lookup of angles for body -10, epoch B9150", ctx)?;

    //
    // Delete the competing keywords.
    //
    spicelib::DVPOOL(b"BODY-10_CONSTS_JED_EPOCH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY-10_CONSTS_REF_FRAME", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore the old keywords.
    //
    spicelib::PDPOOL(b"BODY-10_CONSTANTS_JED_EPOCH", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"BODY-10_CONSTANTS_REF_FRAME", 1, &[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODMAT(10, 0.0, save.XTIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = ((spicelib::B1950() - spicelib::J2000()) * spicelib::SPD());

    spicelib::BODEUL(
        -10,
        save.ET,
        &mut save.RA,
        &mut save.DEC,
        &mut save.W,
        &mut save.LAMBDA,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::EUL2M(
        save.W,
        (spicelib::HALFPI(ctx) - save.DEC),
        (spicelib::HALFPI(ctx) + save.RA),
        3,
        1,
        3,
        save.TIPM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The Euler angles we used to produce the matrix TIPM will have
    // been adjusted to produce the transformation from J2000 to the
    // body-fixed frame for body -10. Remove this factor from the
    // matrix so we obtain the matrix defined by the angles derived
    // directly from the PCK constants.
    //
    spicelib::PXFORM(b"B1950", b"J2000", 0.0, save.B2J.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXM(
        save.TIPM.as_slice(),
        save.B2J.as_slice(),
        save.TMPMAT.as_slice_mut(),
    );
    spicelib::MOVED(save.TMPMAT.as_slice(), 9, save.TIPM.as_slice_mut());

    //
    // Compare to the J2000-to-bodyfixed matrix for body 10 at ET 0.
    //
    testutil::CHCKAD(
        b"TIPM",
        save.TIPM.as_slice(),
        b"~",
        save.XTIPM.as_slice(),
        9,
        MEDIUM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    //     Repeat test for epoch J2000.
    //
    testutil::TCASE(b"BODEUL lookup of angles for body -10, epoch J2000", ctx)?;

    save.ET = ((spicelib::J2000() - spicelib::B1950()) * spicelib::SPD());

    spicelib::BODMAT(10, save.ET, save.XTIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODEUL(
        -10,
        0.0,
        &mut save.RA,
        &mut save.DEC,
        &mut save.W,
        &mut save.LAMBDA,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::EUL2M(
        save.W,
        (spicelib::HALFPI(ctx) - save.DEC),
        (spicelib::HALFPI(ctx) + save.RA),
        3,
        1,
        3,
        save.TIPM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(b"B1950", b"J2000", 0.0, save.B2J.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXM(
        save.TIPM.as_slice(),
        save.B2J.as_slice(),
        save.TMPMAT.as_slice_mut(),
    );
    spicelib::MOVED(save.TMPMAT.as_slice(), 9, save.TIPM.as_slice_mut());

    testutil::CHCKAD(
        b"TIPM",
        save.TIPM.as_slice(),
        b"~",
        save.XTIPM.as_slice(),
        9,
        MEDIUM,
        OK,
        ctx,
    )?;

    //
    // Check the LAMBDA value associated with body -10.  We
    // expect the value to be 99 degrees *RPD().  (The value
    // 459 degrees is set by T_PCK09, which writes the PCK file
    // we're using.  We expect BODEUL to mod this value by 2*pi.)
    //
    save.XLMBDA = (99.0 * spicelib::RPD(ctx));
    testutil::CHCKSD(b"LAMBDA", save.LAMBDA, b"~", save.XLMBDA, MEDIUM, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"BODEUL error case: lookup of angles for body -10 when both *CONSTANTS_JED_EPOCH and *CONSTS_JED_EPOCH keywords are present.", ctx)?;

    spicelib::DTPOOL(
        b"BODY-10_CONSTANTS_JED_EPOCH",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTYPE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::PDPOOL(b"BODY-10_CONSTS_JED_EPOCH", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODEUL(
        -10,
        0.0,
        &mut save.RA,
        &mut save.DEC,
        &mut save.W,
        &mut save.LAMBDA,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGEPOCHSPEC)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"BODEUL error case: lookup of angles for body -10 when both *_CONSTANTS_REF_FRAME and *CONSTS_REF_FRAME keywords are present.", ctx)?;

    //
    // Delete competing epoch spec first.
    //
    spicelib::DVPOOL(b"BODY-10_CONSTS_JED_EPOCH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DTPOOL(
        b"BODY-10_CONSTANTS_REF_FRAME",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTYPE,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::PDPOOL(b"BODY-10_CONSTS_REF_FRAME", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODEUL(
        -10,
        0.0,
        &mut save.RA,
        &mut save.DEC,
        &mut save.W,
        &mut save.LAMBDA,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(COMPETINGFRAMESPEC)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //     Perform a normal BODEUL computation using the _CONSTS_ forms of
    //     the epoch and base frame keywords.
    //
    testutil::TCASE(b"BODEUL: lookup B1950 to body-fixed transformation matrix for body -10 using both *CONSTS_JED_EPOCH and *CONSTS_REF_FRAME keywords. Epoch is B1950.", ctx)?;

    //
    // Delete the competing keywords.
    //

    spicelib::DVPOOL(b"BODY-10_CONSTS_JED_EPOCH", ctx)?;
    spicelib::DVPOOL(b"BODY-10_CONSTS_REF_FRAME", ctx)?;

    spicelib::DVPOOL(b"BODY-10_CONSTANTS_JED_EPOCH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY-10_CONSTANTS_REF_FRAME", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add the new keywords.
    //

    spicelib::PDPOOL(b"BODY-10_CONSTS_JED_EPOCH", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"BODY-10_CONSTS_REF_FRAME", 1, &[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODMAT(10, 0.0, save.XTIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = ((spicelib::B1950() - spicelib::J2000()) * spicelib::SPD());

    spicelib::BODEUL(
        -10,
        save.ET,
        &mut save.RA,
        &mut save.DEC,
        &mut save.W,
        &mut save.LAMBDA,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::EUL2M(
        save.W,
        (spicelib::HALFPI(ctx) - save.DEC),
        (spicelib::HALFPI(ctx) + save.RA),
        3,
        1,
        3,
        save.TIPM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The Euler angles we used to produce the matrix TIPM will have
    // been adjusted to produce the transformation from J2000 to the
    // body-fixed frame for body -10. Remove this factor from the
    // matrix so we obtain the matrix defined by the angles derived
    // directly from the PCK constants.
    //
    spicelib::PXFORM(b"B1950", b"J2000", 0.0, save.B2J.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXM(
        save.TIPM.as_slice(),
        save.B2J.as_slice(),
        save.TMPMAT.as_slice_mut(),
    );
    spicelib::MOVED(save.TMPMAT.as_slice(), 9, save.TIPM.as_slice_mut());

    //
    // Compare to the J2000-to-bodyfixed matrix for body 10 at ET 0.
    //
    testutil::CHCKAD(
        b"TIPM",
        save.TIPM.as_slice(),
        b"~",
        save.XTIPM.as_slice(),
        9,
        MEDIUM,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    //     Perform a normal BODEUL computation using the _CONSTS_ forms of
    //     the epoch and base frame keywords.
    //
    testutil::TCASE(b"BODEUL: lookup B1950 to body-fixed transformation matrix for body -10 using both *CONSTS_JED_EPOCH and *CONSTS_REF_FRAME keywords. Epoch is J2000.", ctx)?;

    //
    // Delete the competing keywords.
    //
    spicelib::DVPOOL(b"BODY-10_CONSTS_JED_EPOCH", ctx)?;
    spicelib::DVPOOL(b"BODY-10_CONSTS_REF_FRAME", ctx)?;

    spicelib::DVPOOL(b"BODY-10_CONSTANTS_JED_EPOCH", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY-10_CONSTANTS_REF_FRAME", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add the new keywords.
    //
    spicelib::PDPOOL(b"BODY-10_CONSTS_JED_EPOCH", 1, &[spicelib::B1950()], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"BODY-10_CONSTS_REF_FRAME", 1, &[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = ((spicelib::J2000() - spicelib::B1950()) * spicelib::SPD());

    spicelib::BODMAT(10, save.ET, save.XTIPM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODEUL(
        -10,
        0.0,
        &mut save.RA,
        &mut save.DEC,
        &mut save.W,
        &mut save.LAMBDA,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::EUL2M(
        save.W,
        (spicelib::HALFPI(ctx) - save.DEC),
        (spicelib::HALFPI(ctx) + save.RA),
        3,
        1,
        3,
        save.TIPM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The Euler angles we used to produce the matrix TIPM will have
    // been adjusted to produce the transformation from J2000 to the
    // body-fixed frame for body -10. Remove this factor from the
    // matrix so we obtain the matrix defined by the angles derived
    // directly from the PCK constants.
    //
    spicelib::PXFORM(b"B1950", b"J2000", 0.0, save.B2J.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXM(
        save.TIPM.as_slice(),
        save.B2J.as_slice(),
        save.TMPMAT.as_slice_mut(),
    );
    spicelib::MOVED(save.TMPMAT.as_slice(), 9, save.TIPM.as_slice_mut());

    //
    // Compare to the J2000-to-bodyfixed matrix for body 10 at ET 0.
    //
    testutil::CHCKAD(
        b"TIPM",
        save.TIPM.as_slice(),
        b"~",
        save.XTIPM.as_slice(),
        9,
        MEDIUM,
        OK,
        ctx,
    )?;

    // ***************************************************************
    //
    //     Radius tests
    //
    // ***************************************************************

    //
    //     Check the set of bodies for which radius data are provided.
    //     Find the set of kernel variables for triaxial radii;
    //     check the corresponding set of ID codes against the set
    //     from the test utility T_PCKRSTD.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check the set of bodies for which radius data are provided.",
        ctx,
    )?;

    spicelib::SSIZEI(MAXSET, save.XSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXSET, save.RADSET.as_slice_mut(), ctx)?;

    testutil::T_PCKRSTD(save.XSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GNPOOL(
        b"BODY*_RADII",
        1,
        MAXSET,
        &mut save.N,
        save.KVNAMS.subarray_mut(1),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VALIDC(MAXSET, save.N, save.KVNAMS.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"RADII names found", save.FOUND, true, OK, ctx)?;

    save.N = spicelib::CARDC(save.KVNAMS.as_arg(), ctx)?;

    testutil::CHCKSI(
        b"Number of RADII names",
        save.N,
        b"=",
        spicelib::CARDI(save.XSET.as_slice(), ctx)?,
        0,
        OK,
        ctx,
    )?;

    //
    //     Check each name in the expected set for presence in the
    //     actual name set.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check bodies in set expected to have radius data against those present in the kernel.",
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDI(save.XSET.as_slice(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.XNAME, b"BODY#_RADII");

            spicelib::REPMI(
                &save.XNAME.to_vec(),
                b"#",
                save.XSET[save.I],
                &mut save.XNAME,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Make sure the expected name is in the kernel
            //    variable list.
            //
            save.J = spicelib::BSRCHC(&save.XNAME, save.N, save.KVNAMS.subarray(1));

            fstr::assign(&mut save.QNAME, b"# is present in kernel?");
            spicelib::REPMC(&save.QNAME.to_vec(), b"#", &save.XNAME, &mut save.QNAME);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.QNAME, (save.J > 0), true, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check bodies having radius data in kernel against those in the expected set.",
        ctx,
    )?;

    //
    // Check each name in the actual set for presence in the
    // expected name set.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDC(save.KVNAMS.as_arg(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Get the ID code of the current name.
            //
            fstr::assign(&mut save.IDSTR, fstr::substr(save.KVNAMS.get(save.I), 5..));

            save.J = intrinsics::INDEX(&save.IDSTR, b"_");

            spicelib::PRSINT(
                fstr::substr(&save.IDSTR, 1..=(save.J - 1)),
                &mut save.CODE,
                ctx,
            )?;

            fstr::assign(&mut save.QNAME, b"# is present in expected set?");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.CODE, &mut save.QNAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(
                &save.QNAME,
                spicelib::ELEMI(save.CODE, save.XSET.as_slice(), ctx)?,
                true,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Since we've checked the set of bodies, let the actual
    // set be the expected set.
    //
    spicelib::COPYI(save.XSET.as_slice(), save.ANGSET.as_slice_mut(), ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Check radii of each body in the expected set.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDI(save.XSET.as_slice(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Check radii of body #");
            spicelib::REPMI(
                &save.TITLE.to_vec(),
                b"#",
                save.XSET[save.I],
                &mut save.TITLE,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // --- Case: ------------------------------------------------------
            //
            testutil::TCASE(&save.TITLE, ctx)?;

            testutil::T_PCKRADD(save.XSET[save.I], save.XRAD.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::BODVCD(
                save.XSET[save.I],
                b"RADII",
                3,
                &mut save.J,
                save.RADII.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"Number of radii of body #");
            spicelib::REPMI(
                &save.QNAME.to_vec(),
                b"#",
                save.XSET[save.I],
                &mut save.QNAME,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSI(&save.QNAME, save.J, b"=", 3, 0, OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"Radii of body #");
            spicelib::REPMI(
                &save.QNAME.to_vec(),
                b"#",
                save.XSET[save.I],
                &mut save.QNAME,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAD(
                &save.QNAME,
                save.RADII.as_slice(),
                b"=",
                save.XRAD.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if spicelib::EXISTS(&save.PCK, ctx)? {
        spicelib::DELFIL(&save.PCK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
