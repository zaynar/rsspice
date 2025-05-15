//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const MAXL: i32 = 36;
const MAXP: i32 = 150;
const NPERM: i32 = 692;
const MAXE: i32 = 853;
const NROOM: i32 = 14983;
const LSK: &[u8] = b"trgsep.tls";
const SPK1: &[u8] = b"nat.bsp";
const PCK1: &[u8] = b"nat.pck";
const TIGHT: f64 = 0.000000000001;
const LNSIZE: i32 = 80;
const TIMLEN: i32 = 50;
const NCORR: i32 = 9;
const ATOL: f64 = 0.000000000001;

struct SaveVars {
    FRAME: ActualCharArray,
    SHAPE: ActualCharArray,
    ABCORR: Vec<u8>,
    CORR: ActualCharArray,
    REF: Vec<u8>,
    TITLE: Vec<u8>,
    TARG2: Vec<u8>,
    OBSRVR: Vec<u8>,
    TARG: ActualCharArray,
    TIME0: Vec<u8>,
    ANG1: f64,
    ANG2: f64,
    ET: f64,
    ET0: f64,
    LT: f64,
    PV1: StackArray<f64, 3>,
    PV2: StackArray<f64, 3>,
    R1: f64,
    R2: f64,
    RADA: StackArray<f64, 3>,
    RADB: StackArray<f64, 3>,
    RAD1: StackArray<f64, 3>,
    RANGE1: f64,
    RANGE2: f64,
    THETA: f64,
    VALCR: StackArray<f64, 9>,
    VALUE: f64,
    XR: StackArray2D<f64, 8>,
    DIM: i32,
    HAN1: i32,
    M: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FRAME = ActualCharArray::new(16, 1..=4);
        let mut SHAPE = ActualCharArray::new(16, 1..=3);
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut CORR = ActualCharArray::new(LNSIZE, 1..=(NCORR + 1));
        let mut REF = vec![b' '; LNSIZE as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut TARG2 = vec![b' '; LNSIZE as usize];
        let mut OBSRVR = vec![b' '; MAXL as usize];
        let mut TARG = ActualCharArray::new(MAXL, 1..=4);
        let mut TIME0 = vec![b' '; TIMLEN as usize];
        let mut ANG1: f64 = 0.0;
        let mut ANG2: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut PV1 = StackArray::<f64, 3>::new(1..=3);
        let mut PV2 = StackArray::<f64, 3>::new(1..=3);
        let mut R1: f64 = 0.0;
        let mut R2: f64 = 0.0;
        let mut RADA = StackArray::<f64, 3>::new(1..=3);
        let mut RADB = StackArray::<f64, 3>::new(1..=3);
        let mut RAD1 = StackArray::<f64, 3>::new(1..=3);
        let mut RANGE1: f64 = 0.0;
        let mut RANGE2: f64 = 0.0;
        let mut THETA: f64 = 0.0;
        let mut VALCR = StackArray::<f64, 9>::new(1..=NCORR);
        let mut VALUE: f64 = 0.0;
        let mut XR = StackArray2D::<f64, 8>::new(1..=2, 1..=4);
        let mut DIM: i32 = 0;
        let mut HAN1: i32 = 0;
        let mut M: i32 = 0;

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
                Val::C(b"BAD"),
            ]
            .into_iter();
            CORR.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"ALPHA_FIXED"),
                Val::C(b"BETA_FIXED"),
                Val::C(b"NULL"),
                Val::C(b"BAD"),
            ]
            .into_iter();
            FRAME
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"ALPHA"),
                Val::C(b"BETA"),
                Val::C(b"GAMMA"),
                Val::C(b"CALUFRAX"),
            ]
            .into_iter();
            TARG.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"POINT"), Val::C(b"SPHERE"), Val::C(b"BAD")].into_iter();
            SHAPE
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1000000.0), Val::D(1000000.0), Val::D(1000000.0)].into_iter();
            RAD1.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.021816615649929),
                Val::D(0.021889337702096),
                Val::D(0.021889337702096),
                Val::D(0.021889337702096),
                Val::D(0.021889337702096),
                Val::D(0.021743893597763),
                Val::D(0.021743893597763),
                Val::D(0.021743893597763),
                Val::D(0.021743893597763),
            ]
            .into_iter();
            VALCR
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            FRAME,
            SHAPE,
            ABCORR,
            CORR,
            REF,
            TITLE,
            TARG2,
            OBSRVR,
            TARG,
            TIME0,
            ANG1,
            ANG2,
            ET,
            ET0,
            LT,
            PV1,
            PV2,
            R1,
            R2,
            RADA,
            RADB,
            RAD1,
            RANGE1,
            RANGE2,
            THETA,
            VALCR,
            VALUE,
            XR,
            DIM,
            HAN1,
            M,
        }
    }
}

//$Procedure F_TRGSEP ( TRGSEP tests )
pub fn F_TRGSEP(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_TRGSEP", ctx)?;

    //
    // *****************************************************************
    //
    // Setup tests: TRGSEP
    //
    // *****************************************************************
    //
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

    fstr::assign(&mut save.TIME0, b"2000 JAN 1  12:00:00 TDB");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: TRGSEP
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Initial case to confirm functionality.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.OBSRVR, b"SUN");

    for I in 1..=NCORR {
        testutil::TCASE(
            &fstr::concat(b"Confirm basic functionality ", save.CORR.get(I)),
            ctx,
        )?;

        save.VALUE = spicelib::TRGSEP(
            save.ET0,
            &save.TARG[1],
            &save.SHAPE[1],
            &save.FRAME[1],
            &save.TARG[2],
            &save.SHAPE[1],
            &save.FRAME[2],
            &save.OBSRVR,
            &save.CORR[I],
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSD(b"VALUE", save.VALUE, b"~", save.VALCR[I], TIGHT, OK, ctx)?;
    }

    //
    // --- Case: -------------------------------------------------------
    //
    // Run a loop over one orbit of ALPHA at 50 TDB second intervals
    // with each of the aberration corrections. Any occultation event
    // exists for longer than 50 seconds. Compare the value returned
    // from TRGSEP with a direct calculation.
    //

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
    // 1 - point, point
    // 2 - sphere, point
    // 3 - point, sphere,
    // 4 - sphere, sphere.
    //
    save.XR[[1, 1]] = 0.0;
    save.XR[[1, 2]] = save.RADA[1];
    save.XR[[1, 3]] = 0.0;
    save.XR[[1, 4]] = save.RADA[1];
    save.XR[[2, 1]] = 0.0;
    save.XR[[2, 2]] = 0.0;
    save.XR[[2, 3]] = save.RADB[1];
    save.XR[[2, 4]] = save.RADB[1];

    fstr::assign(&mut save.REF, b"J2000");

    for I in intrinsics::range(0, 86400, 50) {
        for J in 1..=NCORR {
            fstr::assign(&mut save.ABCORR, save.CORR.get(J));
            save.M = 1;

            for K in 1..=2 {
                for L in 1..=2 {
                    save.R1 = save.XR[[1, save.M]];
                    save.R2 = save.XR[[2, save.M]];
                    save.M = (save.M + 1);

                    fstr::assign(&mut save.TITLE, b"SEP ET = #1, R1 = #2, R2 = #3, CORR = #4");
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

                    save.VALUE = spicelib::TRGSEP(
                        save.ET,
                        &save.TARG[1],
                        &save.SHAPE[L],
                        &save.FRAME[1],
                        &save.TARG[2],
                        &save.SHAPE[K],
                        &save.FRAME[2],
                        &save.OBSRVR,
                        &save.CORR[J],
                        ctx,
                    )?;

                    spicelib::SPKPOS(
                        &save.TARG[1],
                        save.ET,
                        &save.REF,
                        &save.CORR[J],
                        &save.OBSRVR,
                        save.PV1.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;

                    spicelib::SPKPOS(
                        &save.TARG[2],
                        save.ET,
                        &save.REF,
                        &save.CORR[J],
                        &save.OBSRVR,
                        save.PV2.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;

                    save.RANGE1 = spicelib::VNORM(save.PV1.as_slice());
                    save.RANGE2 = spicelib::VNORM(save.PV2.as_slice());

                    save.ANG1 = spicelib::DASINE((save.R1 / save.RANGE1), ATOL, ctx)?;
                    save.ANG2 = spicelib::DASINE((save.R2 / save.RANGE2), ATOL, ctx)?;

                    save.THETA = ((spicelib::VSEP(save.PV1.as_slice(), save.PV2.as_slice(), ctx)
                        - save.ANG1)
                        - save.ANG2);

                    //
                    // The results should match to roundoff, or
                    // something badly wrong.
                    //
                    testutil::CHCKSD(&save.TITLE, save.VALUE, b"=", save.THETA, TIGHT, OK, ctx)?;
                }
            }
        }
    }

    //
    // *****************************************************************
    //
    // Non-error exceptional cases: TRGSEP
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Null string as an argument for FRAME1 and FRAME2.
    //
    testutil::TCASE(b"Empty string as an argument for frame", ctx)?;

    fstr::assign(&mut save.ABCORR, b"NONE");

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[4],
        &save.TARG[2],
        &save.SHAPE[2],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.ABCORR,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[2],
        &save.FRAME[4],
        &save.OBSRVR,
        &save.ABCORR,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[4],
        &save.TARG[2],
        &save.SHAPE[2],
        &save.FRAME[4],
        &save.OBSRVR,
        &save.ABCORR,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Error cases: TRGSEP
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Three objects TARG1, TARG2 and OBSRVR  are not distinct
    //
    testutil::TCASE(b"Bodies not distinct", ctx)?;

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.CORR[1],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.TARG[2],
        &save.CORR[1],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.TARG[1],
        &save.CORR[1],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // Incorrect object names for TARG1, TARG2 and OBSRVR.
    //
    testutil::TCASE(b"Bad TARG1", ctx)?;

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[4],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.CORR[1],
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Bad TARG2", ctx)?;

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG[4],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.CORR[1],
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Bad OBSRVR", ctx)?;

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.TARG[4],
        &save.CORR[1],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // Incorrect centers for TARG1, FRAME1 and TARG2, FRAME2.
    //
    //
    // The current version of TRGSEP ignores the FRAME argument.
    // When 'ELLIPSOID' is implemented, the following Exception
    // test is to be implemented:
    //
    // If the reference frame associated with TARG1, FRAME1, is not
    // centered on TARG1, or if the reference frame associated with
    // TARG2, FRAME2, is not centered on TARG2, an error is signaled
    // by a routine in the call tree of this routine. This
    // restriction does not apply to shapes 'SPHERE' and 'POINT', for
    // which the frame input is ignored.
    //

    //
    // --- Case: -------------------------------------------------------
    //
    // Incorrect body shapes for targets.
    //
    testutil::TCASE(b"Bad SHAPE1", ctx)?;

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[1],
        &save.SHAPE[3],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.CORR[1],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    testutil::TCASE(b"Bad SHAPE2", ctx)?;

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[1],
        &save.SHAPE[2],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[3],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.CORR[1],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // Incorrect aberration correction.
    //
    testutil::TCASE(b"Bad ABCORR", ctx)?;

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[1],
        &save.SHAPE[2],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[2],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.CORR[(NCORR + 1)],
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // PCK data unavailable.
    //
    testutil::TCASE(b"PCK data unavailable", ctx)?;

    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.TARG2, b"EARTH");

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG2,
        &save.SHAPE[2],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.ABCORR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    // Ephemeris data unavailable.
    //
    testutil::TCASE(b"Ephemeris data unavailable", ctx)?;

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[1],
        &save.SHAPE[1],
        &save.FRAME[1],
        &save.TARG2,
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.OBSRVR,
        &save.ABCORR,
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
    // --- Case: -------------------------------------------------------
    //
    // OBSRVR is located within either one of the targets.
    //
    testutil::TCASE(b"OBSRVR is located within one of the targets", ctx)?;

    spicelib::PDPOOL(b"BODY1000_RADII", 3, save.RAD1.as_slice(), ctx)?;

    save.VALUE = spicelib::TRGSEP(
        save.ET0,
        &save.TARG[1],
        &save.SHAPE[2],
        &save.FRAME[1],
        &save.TARG[2],
        &save.SHAPE[1],
        &save.FRAME[2],
        &save.TARG[3],
        &save.ABCORR,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INSIDEBODY)", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Clean up tests: TRGSEP
    //
    // *****************************************************************
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
