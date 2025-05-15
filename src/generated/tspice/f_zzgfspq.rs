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
const LSK: &[u8] = b"gfsep.tls";
const SPK1: &[u8] = b"nat.bsp";
const PCK1: &[u8] = b"nat.pck";
const TIGHT: f64 = 0.000000000001;
const LNSIZE: i32 = 80;
const TIMLEN: i32 = 50;
const NCORR: i32 = 9;
const ATOL: f64 = 0.000000000001;

struct SaveVars {
    ABCORR: Vec<u8>,
    REF: Vec<u8>,
    CORR: ActualCharArray,
    TITLE: Vec<u8>,
    TIME0: Vec<u8>,
    XR: StackArray2D<f64, 8>,
    PV1: StackArray<f64, 3>,
    PV2: StackArray<f64, 3>,
    RADA: StackArray<f64, 3>,
    RADB: StackArray<f64, 3>,
    ET0: f64,
    ET: f64,
    VALUE: f64,
    R1: f64,
    R2: f64,
    LT: f64,
    THETA: f64,
    ANG1: f64,
    ANG2: f64,
    RANGE1: f64,
    RANGE2: f64,
    HAN1: i32,
    DIM: i32,
    TARG1: i32,
    TARG2: i32,
    OBS: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut REF = vec![b' '; LNSIZE as usize];
        let mut CORR = ActualCharArray::new(LNSIZE, 1..=NCORR);
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut TIME0 = vec![b' '; TIMLEN as usize];
        let mut XR = StackArray2D::<f64, 8>::new(1..=2, 1..=4);
        let mut PV1 = StackArray::<f64, 3>::new(1..=3);
        let mut PV2 = StackArray::<f64, 3>::new(1..=3);
        let mut RADA = StackArray::<f64, 3>::new(1..=3);
        let mut RADB = StackArray::<f64, 3>::new(1..=3);
        let mut ET0: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut VALUE: f64 = 0.0;
        let mut R1: f64 = 0.0;
        let mut R2: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut THETA: f64 = 0.0;
        let mut ANG1: f64 = 0.0;
        let mut ANG2: f64 = 0.0;
        let mut RANGE1: f64 = 0.0;
        let mut RANGE2: f64 = 0.0;
        let mut HAN1: i32 = 0;
        let mut DIM: i32 = 0;
        let mut TARG1: i32 = 0;
        let mut TARG2: i32 = 0;
        let mut OBS: i32 = 0;

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

        Self {
            ABCORR,
            REF,
            CORR,
            TITLE,
            TIME0,
            XR,
            PV1,
            PV2,
            RADA,
            RADB,
            ET0,
            ET,
            VALUE,
            R1,
            R2,
            LT,
            THETA,
            ANG1,
            ANG2,
            RANGE1,
            RANGE2,
            HAN1,
            DIM,
            TARG1,
            TARG2,
            OBS,
        }
    }
}

//$Procedure F_ZZGFSPQ ( ZZGFSPQ family tests )
pub fn F_ZZGFSPQ(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // ATOL is a tolerance value for computing arc sine.
    //

    //
    // Local variables
    //

    //
    // Save all.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFSPQ", ctx)?;

    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    //
    // Create an LSK, load using FURNSH.
    //
    testutil::ZZTSTLSK(LSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(LSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the PCK for Nat's Solar System.
    //
    testutil::NATPCK(PCK1, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the SPK for Nat's Solar System.
    //
    testutil::NATSPK(SPK1, true, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a confinement window from ET0 and ET1.
    //
    fstr::assign(&mut save.TIME0, b"2000 JAN 1  12:00:00 TDB");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Case 1
    //
    testutil::TCASE(b"Ephemeris data unavailable", ctx)?;

    save.TARG1 = -203;
    save.TARG2 = 499;
    save.OBS = 10;
    save.R1 = 0.0;
    save.R2 = 0.0;
    fstr::assign(&mut save.REF, b"J2000");
    fstr::assign(&mut save.ABCORR, b"NONE");

    spicelib::ZZGFSPQ(
        save.ET0,
        save.TARG1,
        save.TARG2,
        save.R1,
        save.R2,
        save.OBS,
        &save.ABCORR,
        &save.REF,
        &mut save.VALUE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    spicelib::BODVRD(
        b"ALPHA",
        b"RADII",
        3,
        &mut save.DIM,
        save.RADA.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKSI(b"BODVRD ALPHA", save.DIM, b"=", 3, 0, OK, ctx)?;

    spicelib::BODVRD(
        b"BETA",
        b"RADII",
        3,
        &mut save.DIM,
        save.RADB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKSI(b"BODVRD ALPHA", save.DIM, b"=", 3, 0, OK, ctx)?;

    //
    // Case 2
    //
    testutil::TCASE(b"Negative body radius R1 and R2", ctx)?;

    spicelib::BODVRD(
        b"ALPHA",
        b"RADII",
        3,
        &mut save.DIM,
        save.RADA.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKSI(b"BODVRD ALPHA", save.DIM, b"=", 3, 0, OK, ctx)?;

    spicelib::BODVRD(
        b"BETA",
        b"RADII",
        3,
        &mut save.DIM,
        save.RADB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKSI(b"BODVRD ALPHA", save.DIM, b"=", 3, 0, OK, ctx)?;

    save.TARG1 = 1000;
    save.TARG2 = 2000;
    save.OBS = 10;
    save.R1 = -save.RADA[1];
    save.R2 = 0.0;
    fstr::assign(&mut save.REF, b"J2000");
    fstr::assign(&mut save.ABCORR, b"NONE");

    spicelib::ZZGFSPQ(
        save.ET0,
        save.TARG1,
        save.TARG2,
        save.R1,
        save.R2,
        save.OBS,
        &save.ABCORR,
        &save.REF,
        &mut save.VALUE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADRADIUS)", OK, ctx)?;

    save.R1 = 0.0;
    save.R2 = -save.RADB[1];

    spicelib::ZZGFSPQ(
        save.ET0,
        save.TARG1,
        save.TARG2,
        save.R1,
        save.R2,
        save.OBS,
        &save.ABCORR,
        &save.REF,
        &mut save.VALUE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADRADIUS)", OK, ctx)?;

    //
    // Case 2
    //

    //
    // Check a simple calculation for all CORR.
    //
    save.TARG1 = 1000;
    save.TARG2 = 2000;
    save.OBS = 10;
    save.R1 = 0.0;
    save.R2 = 0.0;
    fstr::assign(&mut save.REF, b"J2000");
    fstr::assign(&mut save.ABCORR, b"NONE");

    for I in 1..=NCORR {
        testutil::TCASE(&save.CORR[I], ctx)?;

        spicelib::ZZGFSPQ(
            save.ET0,
            save.TARG1,
            save.TARG2,
            save.R1,
            save.R2,
            save.OBS,
            &save.CORR[I],
            &save.REF,
            &mut save.VALUE,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Check a simple calculation for invalid CORR.
    //
    for I in 1..=NCORR {
        fstr::assign(
            &mut save.ABCORR,
            &fstr::concat(b"Z", fstr::substr(save.CORR.get(I), 1..=(LNSIZE - 1))),
        );

        testutil::TCASE(&save.ABCORR, ctx)?;

        spicelib::ZZGFSPQ(
            save.ET0,
            save.TARG1,
            save.TARG2,
            save.R1,
            save.R2,
            save.OBS,
            &save.ABCORR,
            &save.REF,
            &mut save.VALUE,
            ctx,
        )?;

        testutil::CHCKXC(true, b"SPICE(SPKINVALIDOPTION)", OK, ctx)?;
    }

    //
    // Case 3
    //

    //
    // Run a loop over one orbit of ALPHA at 50 TDB second
    // intervals with each of the aborration corrections.
    // Any occultation event exists for longer than 50 seconds.
    // Compare the value returned from ZZGFSPQ with a direct
    // calculation.
    //
    save.TARG1 = 1000;
    save.TARG2 = 2000;
    save.OBS = 10;
    fstr::assign(&mut save.REF, b"J2000");

    save.XR[[1, 1]] = 0.0;
    save.XR[[1, 2]] = save.RADA[1];
    save.XR[[1, 3]] = 0.0;
    save.XR[[1, 4]] = save.RADA[1];
    save.XR[[2, 1]] = 0.0;
    save.XR[[2, 2]] = 0.0;
    save.XR[[2, 3]] = save.RADB[1];
    save.XR[[2, 4]] = save.RADB[1];

    for I in intrinsics::range(0, 86400, 50) {
        for J in 1..=NCORR {
            fstr::assign(&mut save.ABCORR, save.CORR.get(J));

            for K in 1..=4 {
                save.R1 = save.XR[[1, K]];
                save.R2 = save.XR[[2, K]];

                fstr::assign(&mut save.TITLE, b"ET = #1, R1 = #2, R2 = #3, CORR = #4");
                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#1",
                    save.ET,
                    6,
                    &mut save.TITLE,
                    ctx,
                );
                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#2",
                    save.R1,
                    6,
                    &mut save.TITLE,
                    ctx,
                );
                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#3",
                    save.R2,
                    6,
                    &mut save.TITLE,
                    ctx,
                );
                spicelib::REPMI(&save.TITLE.to_vec(), b"#4", J, &mut save.TITLE, ctx);
                testutil::TCASE(&save.TITLE, ctx)?;

                save.ET = (save.ET0 + (I as f64));

                spicelib::ZZGFSPQ(
                    save.ET,
                    save.TARG1,
                    save.TARG2,
                    save.R1,
                    save.R2,
                    save.OBS,
                    &save.ABCORR,
                    &save.REF,
                    &mut save.VALUE,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKEZP(
                    save.TARG1,
                    save.ET,
                    &save.REF,
                    &save.ABCORR,
                    save.OBS,
                    save.PV1.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                spicelib::SPKEZP(
                    save.TARG2,
                    save.ET,
                    &save.REF,
                    &save.ABCORR,
                    save.OBS,
                    save.PV2.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.RANGE1 = spicelib::VNORM(save.PV1.as_slice());
                save.RANGE2 = spicelib::VNORM(save.PV2.as_slice());
                save.ANG1 = spicelib::DASINE((save.R1 / save.RANGE1), ATOL, ctx)?;
                save.ANG2 = spicelib::DASINE((save.R2 / save.RANGE2), ATOL, ctx)?;

                save.THETA = ((spicelib::VSEP(save.PV1.as_slice(), save.PV2.as_slice(), ctx)
                    - save.ANG1)
                    - save.ANG2);
                //
                // The results should match to roundoff.
                //
                testutil::CHCKSD(&save.TITLE, save.VALUE, b"=", save.THETA, TIGHT, OK, ctx)?;
            }
        }
    }

    //
    // Case n
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::DELFIL(LSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
