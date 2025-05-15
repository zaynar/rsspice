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
const RECSYS: &[u8] = b"RECTANGULAR";
const LATSYS: &[u8] = b"LATITUDINAL";
const SPHSYS: &[u8] = b"SPHERICAL";
const RADSYS: &[u8] = b"RA/DEC";
const CYLSYS: &[u8] = b"CYLINDRICAL";
const GEOSYS: &[u8] = b"GEODETIC";
const PGRSYS: &[u8] = b"PLANETOGRAPHIC";
const XCRD: &[u8] = b"X";
const YCRD: &[u8] = b"Y";
const ZCRD: &[u8] = b"Z";
const RADCRD: &[u8] = b"RADIUS";
const LONCRD: &[u8] = b"LONGITUDE";
const LATCRD: &[u8] = b"LATITUDE";
const RACRD: &[u8] = b"RIGHT ASCENSION";
const DECCRD: &[u8] = b"DECLINATION";
const RNGCRD: &[u8] = b"RANGE";
const CLTCRD: &[u8] = b"COLATITUDE";
const ALTCRD: &[u8] = b"ALTITUDE";
const POSDEF: &[u8] = b"POSITION";
const SOBDEF: &[u8] = b"SUB-OBSERVER POINT";
const SINDEF: &[u8] = b"SURFACE INTERCEPT POINT";
const NWREL: i32 = 5;
const NWLONG: i32 = 7;
const EXWIDX: i32 = ((NWREL + NWLONG) + 1);
const MXBEGM: i32 = 55;
const MXENDM: i32 = 13;
const MXMSG: i32 = ((MXBEGM + MXENDM) + 10);
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const PCK: &[u8] = b"zzgflong.tpc";
const NPCK: &[u8] = b"nat.tpc";
const NSPK: &[u8] = b"nat.bsp";
const SPK: &[u8] = b"zzgflong.bsp";
const TIMTOL: f64 = CNVTOL;
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 200;
const FRNMLN: i32 = 32;
const SYSLEN: i32 = 32;
const CRDLEN: i32 = 32;
const VDFLEN: i32 = 32;
const MTHLEN: i32 = 80;
const MAXWIN: i32 = (2 * 20000);
const MW: i32 = MAXWIN;
const NW: i32 = NWMAX;
const LBCELL: i32 = -5;
const OPLEN: i32 = 40;
const NCORR: i32 = 9;
const NLNSYS: i32 = 6;
const NVDEF: i32 = 3;

struct SaveVars {
    ABCORR: Vec<u8>,
    CNAM: ActualCharArray,
    CORR: ActualCharArray,
    CRDNAM: Vec<u8>,
    CRDSYS: Vec<u8>,
    CSYS: ActualCharArray,
    DREF: Vec<u8>,
    DSCSTR: Vec<u8>,
    REF: Vec<u8>,
    RELATE: Vec<u8>,
    METHOD: Vec<u8>,
    OBSRVR: Vec<u8>,
    QNAME: Vec<u8>,
    TARGET: Vec<u8>,
    TITLE: Vec<u8>,
    VDEF: ActualCharArray,
    VECDEF: Vec<u8>,
    ADJUST: f64,
    ALPHRD: StackArray<f64, 3>,
    ALT: f64,
    CNFINE: ActualArray<f64>,
    DVEC: StackArray<f64, 3>,
    ET: f64,
    ET0: f64,
    ET1: f64,
    FINISH: f64,
    LON: f64,
    LT: f64,
    LTOFF: f64,
    QTOL: f64,
    POS: StackArray<f64, 3>,
    REFVAL: f64,
    RESULT: ActualArray<f64>,
    START: f64,
    STATE: StackArray<f64, 6>,
    T1: f64,
    T2: f64,
    TMPRAD: StackArray<f64, 3>,
    TOL: f64,
    LAT: f64,
    WORK: ActualArray2D<f64>,
    R: f64,
    XET: f64,
    XLON: f64,
    XTIME: f64,
    HAN2: i32,
    HANDLE: i32,
    N: i32,
    ATTBLK: StackArray<bool, 6>,
    BAIL: bool,
    RPT: bool,
    USELT: bool,
    XMIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut CNAM = ActualCharArray::new(CRDLEN, 1..=NLNSYS);
        let mut CORR = ActualCharArray::new(LNSIZE, 1..=NCORR);
        let mut CRDNAM = vec![b' '; CRDLEN as usize];
        let mut CRDSYS = vec![b' '; SYSLEN as usize];
        let mut CSYS = ActualCharArray::new(SYSLEN, 1..=NLNSYS);
        let mut DREF = vec![b' '; FRNMLN as usize];
        let mut DSCSTR = vec![b' '; LNSIZE as usize];
        let mut REF = vec![b' '; FRNMLN as usize];
        let mut RELATE = vec![b' '; OPLEN as usize];
        let mut METHOD = vec![b' '; MTHLEN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut VDEF = ActualCharArray::new(VDFLEN, 1..=NVDEF);
        let mut VECDEF = vec![b' '; VDFLEN as usize];
        let mut ADJUST: f64 = 0.0;
        let mut ALPHRD = StackArray::<f64, 3>::new(1..=3);
        let mut ALT: f64 = 0.0;
        let mut CNFINE = ActualArray::<f64>::new(LBCELL..=MW);
        let mut DVEC = StackArray::<f64, 3>::new(1..=3);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut FINISH: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut LTOFF: f64 = 0.0;
        let mut QTOL: f64 = 0.0;
        let mut POS = StackArray::<f64, 3>::new(1..=3);
        let mut REFVAL: f64 = 0.0;
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MW);
        let mut START: f64 = 0.0;
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut T1: f64 = 0.0;
        let mut T2: f64 = 0.0;
        let mut TMPRAD = StackArray::<f64, 3>::new(1..=3);
        let mut TOL: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MW, 1..=NW);
        let mut R: f64 = 0.0;
        let mut XET: f64 = 0.0;
        let mut XLON: f64 = 0.0;
        let mut XTIME: f64 = 0.0;
        let mut HAN2: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut N: i32 = 0;
        let mut ATTBLK = StackArray::<bool, 6>::new(1..=ABATSZ);
        let mut BAIL: bool = false;
        let mut RPT: bool = false;
        let mut USELT: bool = false;
        let mut XMIT: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"None"),
                Val::C(b"Lt"),
                Val::C(b"cN"),
                Val::C(b"XlT"),
                Val::C(b"xCn"),
                Val::C(b"LT + s"),
                Val::C(b"xLt+ S"),
                Val::C(b"xCn"),
                Val::C(b" XcN +s"),
            ]
            .into_iter();
            CORR.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"Longitude"),
                Val::C(b"Right  ascension"),
                Val::C(b"Longitude"),
                Val::C(b"Longitude"),
                Val::C(b"Longitude"),
                Val::C(b"Longitude"),
            ]
            .into_iter();
            CNAM.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"Latitudinal"),
                Val::C(b"Ra / Dec"),
                Val::C(b"Cylindrical"),
                Val::C(b"Geodetic"),
                Val::C(b"Planetographic"),
                Val::C(b"Spherical"),
            ]
            .into_iter();
            CSYS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(POSDEF), Val::C(SOBDEF), Val::C(SINDEF)].into_iter();
            VDEF.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            CNAM,
            CORR,
            CRDNAM,
            CRDSYS,
            CSYS,
            DREF,
            DSCSTR,
            REF,
            RELATE,
            METHOD,
            OBSRVR,
            QNAME,
            TARGET,
            TITLE,
            VDEF,
            VECDEF,
            ADJUST,
            ALPHRD,
            ALT,
            CNFINE,
            DVEC,
            ET,
            ET0,
            ET1,
            FINISH,
            LON,
            LT,
            LTOFF,
            QTOL,
            POS,
            REFVAL,
            RESULT,
            START,
            STATE,
            T1,
            T2,
            TMPRAD,
            TOL,
            LAT,
            WORK,
            R,
            XET,
            XLON,
            XTIME,
            HAN2,
            HANDLE,
            N,
            ATTBLK,
            BAIL,
            RPT,
            USELT,
            XMIT,
        }
    }
}

//$Procedure      F_ZZGFLNG2 ( Test GF longitude solver, part 2 )
pub fn F_ZZGFLNG2(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // EXTERNAL declarations
    //

    //
    // Local parameters
    //

    //
    // Maximum length of a coordinate system name.
    //

    //
    // Maximum length of a coordinate name.
    //

    //
    // Number of aberration correction choices:
    //

    //
    // Number of systems that support longitude:
    //

    //
    // Number of vector definitions:
    //

    //
    // Local variables
    //

    //
    // Save everything.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFLNG2", ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    // Leapseconds:  Note that the LSK is deleted after loading, so we
    // don't have to clean it up later.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load a PCK.
    //
    testutil::TSTPCK(PCK, true, false, ctx)?;

    //
    // Load an SPK file as well.
    //
    testutil::TSTSPK(SPK, true, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create and load Nat's solar system SPK and PCK/FK
    // kernels.
    //
    testutil::NATPCK(NPCK, true, false, ctx)?;
    testutil::NATSPK(NSPK, true, &mut save.HAN2, ctx)?;

    //
    // Actual DE-based ephemerides yield better comparisons,
    // since these ephemerides have less noise than do those
    // produced by TSTSPK.
    //
    //  CALL FURNSH ( 'de421.bsp' )
    //  CALL FURNSH ( 'jup230.bsp' )
    //

    //*********************************************************************
    //*
    //*    Normal cases --- Nat's solar system
    //*
    //*********************************************************************

    //*********************************************************************
    //*
    //*    Equality cases
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Equality tests:
    //
    //     We want at least one case that exercises every combination of
    //     aberration correction and coordinate system. This set of tests
    //     does that. Later on, we'll test a more complete set of equality
    //     constraints.
    //
    //     We're going to find times when the Sun-Beta vector crosses the
    //     ALPHA_VIEW_XY prime meridian. This should occur every 24 hours,
    //     starting at the J2000 epoch.
    //
    //     Loop over the aberration corrections.
    //
    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 12:04 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 12:06 TDB", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.VECDEF, POSDEF);
    fstr::assign(&mut save.METHOD, b" ");
    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.REF, b"ALPHA_VIEW_XY");
    fstr::assign(&mut save.DREF, b" ");

    for CC in 1..=NLNSYS {
        fstr::assign(&mut save.CRDSYS, save.CSYS.get(CC));
        fstr::assign(&mut save.CRDNAM, save.CNAM.get(CC));

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Set the longer equatorial radius of Alpha equal to
            // the shorter: unequal equatorial radii are not supported
            // for these systems.
            //
            spicelib::BODVRD(
                b"ALPHA",
                b"RADII",
                3,
                &mut save.N,
                save.ALPHRD.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Note: use of the subscript "2" in the first argument
            // below is intentional.
            //
            spicelib::VPACK(
                save.ALPHRD[2],
                save.ALPHRD[2],
                save.ALPHRD[3],
                save.TMPRAD.as_slice_mut(),
            );

            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.TMPRAD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        for AC in 1..=NCORR {
            //
            // --- Case: ------------------------------------------------------
            //
            fstr::assign(&mut save.ABCORR, save.CORR.get(AC));

            fstr::assign(
                &mut save.TITLE,
                b"Sun-Beta vector crosses the ALPHA_VIEW_XY prime meridian. #; #; #",
            );

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDNAM, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

            testutil::TCASE(&save.TITLE, ctx)?;

            spicelib::ZZPRSCOR(&save.ABCORR, save.ATTBLK.as_slice_mut(), ctx)?;

            save.XMIT = save.ATTBLK[XMTIDX];
            save.USELT = save.ATTBLK[LTIDX];

            spicelib::CLEARD(3, save.DVEC.as_slice_mut());

            fstr::assign(&mut save.RELATE, b"=");
            save.REFVAL = 0.0;
            save.TOL = CNVTOL;
            save.ADJUST = 0.0;

            save.RPT = false;
            save.BAIL = false;

            //
            // Set the search step size.
            //
            spicelib::GFSSTP(10000.0, ctx)?;

            //
            // Perform the search.
            //
            spicelib::ZZGFLONG(
                &save.VECDEF,
                &save.METHOD,
                &save.TARGET,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                &save.DREF,
                save.DVEC.as_slice(),
                &save.CRDSYS,
                &save.CRDNAM,
                &save.RELATE,
                save.REFVAL,
                save.TOL,
                save.ADJUST,
                spicelib::GFSTEP,
                spicelib::GFREFN,
                save.RPT,
                spicelib::GFREPI,
                spicelib::GFREPU,
                spicelib::GFREPF,
                save.BAIL,
                spicelib::GFBAIL,
                MW,
                NW,
                save.WORK.as_slice_mut(),
                save.CNFINE.as_slice(),
                save.RESULT.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Make sure we found 1 root.
            //
            save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
            testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

            //
            // Make sure that the start and finish times are
            // those we expect. The crossing should occur at
            // the midpoint of the occultation of ALPHA by BETA,
            // which is 5 minutes after noon TDB, plus or minus
            // the light time offset.
            //
            if save.USELT {
                if save.XMIT {
                    save.LTOFF = -1.0;
                } else {
                    save.LTOFF = 1.0;
                }
            } else {
                save.LTOFF = 0.0;
            }

            for I in 1..=save.N {
                save.XTIME = (((((I - 1) as f64) * spicelib::SPD()) + 300.0) + save.LTOFF);

                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    I,
                    &mut save.START,
                    &mut save.FINISH,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut save.QNAME, b"Interval start # (0)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

                testutil::CHCKSD(&save.QNAME, save.START, b"~", save.XTIME, TIMTOL, OK, ctx)?;

                fstr::assign(&mut save.QNAME, b"Interval stop  # (0)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

                testutil::CHCKSD(&save.QNAME, save.FINISH, b"~", save.XTIME, TIMTOL, OK, ctx)?;
            }

            //
            // Check the longitude at the crossing time.
            //
            // In order to decide how accurate the coordinate should be,
            // we'll use the angular rate of Beta together with the time
            // tolerance.
            //
            save.QTOL = ((spicelib::TWOPI(ctx) / spicelib::SPD()) * CNVTOL);

            for I in 1..=save.N {
                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    I,
                    &mut save.START,
                    &mut save.FINISH,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKEZR(
                    &save.TARGET,
                    save.START,
                    &save.REF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::RECLAT(
                    save.STATE.as_slice(),
                    &mut save.R,
                    &mut save.LON,
                    &mut save.LAT,
                );

                fstr::assign(&mut save.QNAME, b"Longitude # (0)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

                testutil::CHCKSD(&save.QNAME, save.LON, b"~", 0.0, save.QTOL, OK, ctx)?;
            }
        }
        //
        // End of aberration correction loop.
        //

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Restore Alpha's radii.
            //
            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.ALPHRD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }
    //
    // End of coordinate system loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     This set of sets exercises every combination of vector definition
    //     and coordinate system; we don't vary the aberration correction.
    //     For sub-observer points, we vary the method but don't test every
    //     combination of method and coordinate system.
    //
    //     We're going to find times when the each of the following vectors
    //     crosses the
    //     ALPHAFIXED prime meridian:
    //
    //        1) Position of Gamma with respect to Alpha
    //
    //        2) Sub-Gamma point on Alpha
    //
    //        3) Intercept of Gamma-to-Alpha ray on surface of Alpha
    //
    //     This should occur every 24 hours, starting at 6 hours past the
    //     J2000 epoch.
    //
    //
    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 17:59 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 2 18:01 TDB", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ABCORR, b"NONE");

    for CC in 1..=NLNSYS {
        //
        // Set the coordinate system and coordinate.
        //
        fstr::assign(&mut save.CRDSYS, save.CSYS.get(CC));
        fstr::assign(&mut save.CRDNAM, save.CNAM.get(CC));

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Set the longer equatorial radius of Alpha equal to
            // the shorter: unequal equatorial radii are not supported
            // for these systems.
            //
            spicelib::BODVRD(
                b"ALPHA",
                b"RADII",
                3,
                &mut save.N,
                save.ALPHRD.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Note: use of the subscript "2" in the first argument
            // below is intentional.
            //
            spicelib::VPACK(
                save.ALPHRD[2],
                save.ALPHRD[2],
                save.ALPHRD[3],
                save.TMPRAD.as_slice_mut(),
            );

            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.TMPRAD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        for VI in 1..=NVDEF {
            //
            // Set the vector definition.
            //
            fstr::assign(&mut save.VECDEF, save.VDEF.get(VI));

            //
            // Set the definition method and ray's
            // direction.
            //
            if fstr::eq(&save.VECDEF, POSDEF) {
                //
                // Note that for positions, the vector
                // is the *opposite* of the sub-observer
                // or position vector. It's easiest to
                // switch observer and target for this case.
                //
                fstr::assign(&mut save.DSCSTR, b"Alpha-Gamma position");
                fstr::assign(&mut save.OBSRVR, b"ALPHA");
                fstr::assign(&mut save.TARGET, b"GAMMA");
                fstr::assign(&mut save.REF, b"ALPHAFIXED");

                fstr::assign(&mut save.METHOD, b" ");
                fstr::assign(&mut save.DREF, b" ");
                spicelib::CLEARD(3, save.DVEC.as_slice_mut());
            } else if fstr::eq(&save.VECDEF, SOBDEF) {
                fstr::assign(&mut save.DSCSTR, b"Sub-Gamma point");
                fstr::assign(&mut save.TARGET, b"ALPHA");
                fstr::assign(&mut save.OBSRVR, b"GAMMA");
                fstr::assign(&mut save.REF, b"ALPHAFIXED");
                //
                // Vary the sub-observer point definition.
                //
                if spicelib::ODD(CC) {
                    fstr::assign(&mut save.METHOD, b"Near point: ellipsoid");
                } else {
                    fstr::assign(&mut save.METHOD, b"Intercept: ellipsoid");
                }

                fstr::assign(&mut save.DREF, b" ");
                spicelib::CLEARD(3, save.DVEC.as_slice_mut());
            } else if fstr::eq(&save.VECDEF, SINDEF) {
                fstr::assign(&mut save.DSCSTR, b"Gamma-Alpha ray intercept");
                fstr::assign(&mut save.TARGET, b"ALPHA");
                fstr::assign(&mut save.OBSRVR, b"GAMMA");
                fstr::assign(&mut save.REF, b"ALPHAFIXED");

                fstr::assign(&mut save.METHOD, b"ellipsoid");
                fstr::assign(&mut save.DREF, b"gammafixed");

                //
                // The pointing direction is along the gammafixed
                // frame's +X axis.
                //
                spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());
            } else {
                //
                // This is a backstop case.
                //
                spicelib::SETMSG(b"VECDEF = #", ctx);
                spicelib::ERRCH(b"#", &save.VECDEF, ctx);
                spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }

            //
            // --- Case: ------------------------------------------------------
            //

            fstr::assign(&mut save.TITLE, b"# on Alpha 0 meridian. #; #; #; #; #");

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.DSCSTR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDNAM, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.VECDEF, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.METHOD, &mut save.TITLE);

            testutil::TCASE(&save.TITLE, ctx)?;

            fstr::assign(&mut save.RELATE, b"=");
            save.REFVAL = 0.0;
            save.TOL = CNVTOL;
            save.ADJUST = 0.0;

            save.RPT = false;
            save.BAIL = false;

            //
            // Set the search step size.
            //
            spicelib::GFSSTP(10000.0, ctx)?;

            //
            // Perform the search.
            //
            spicelib::ZZGFLONG(
                &save.VECDEF,
                &save.METHOD,
                &save.TARGET,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                &save.DREF,
                save.DVEC.as_slice(),
                &save.CRDSYS,
                &save.CRDNAM,
                &save.RELATE,
                save.REFVAL,
                save.TOL,
                save.ADJUST,
                spicelib::GFSTEP,
                spicelib::GFREFN,
                save.RPT,
                spicelib::GFREPI,
                spicelib::GFREPU,
                spicelib::GFREPF,
                save.BAIL,
                spicelib::GFBAIL,
                MW,
                NW,
                save.WORK.as_slice_mut(),
                save.CNFINE.as_slice(),
                save.RESULT.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Make sure we found 2 roots.
            //
            save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
            testutil::CHCKSI(b"N", save.N, b"=", 2, 0, OK, ctx)?;

            for I in 1..=save.N {
                //
                // We expect to cross the prime meridian at 6 pm TDB.
                //
                save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + ((6 as f64) * 3600.0));

                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    I,
                    &mut save.START,
                    &mut save.FINISH,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut save.QNAME, b"Interval start # (0)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

                testutil::CHCKSD(&save.QNAME, save.START, b"~", save.XTIME, TIMTOL, OK, ctx)?;

                fstr::assign(&mut save.QNAME, b"Interval stop  # (0)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

                testutil::CHCKSD(&save.QNAME, save.FINISH, b"~", save.XTIME, TIMTOL, OK, ctx)?;
            }

            //
            // Check the longitude at the crossing time.
            //
            // In order to decide how accurate the coordinate should be,
            // we'll use the angular rate of Gamma together with the time
            // tolerance.
            //
            save.QTOL = ((spicelib::TWOPI(ctx) / spicelib::SPD()) * CNVTOL);

            for I in 1..=save.N {
                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    I,
                    &mut save.START,
                    &mut save.FINISH,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Compute the longitude of the vector we want.
                // Because of the special geometry we've set up,
                // we can use the observer-target state for all
                // cases; just negate the position for the
                // sub-observer and intercept definitions.
                //
                spicelib::SPKEZR(
                    &save.TARGET,
                    save.START,
                    &save.REF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if (fstr::eq(&save.VECDEF, SOBDEF) || fstr::eq(&save.VECDEF, SINDEF)) {
                    spicelib::VMINUS(save.STATE.as_slice(), save.POS.as_slice_mut());
                } else {
                    spicelib::VEQU(save.STATE.as_slice(), save.POS.as_slice_mut());
                }

                spicelib::RECLAT(
                    save.POS.as_slice(),
                    &mut save.R,
                    &mut save.LON,
                    &mut save.LAT,
                );

                fstr::assign(&mut save.QNAME, b"Longitude # (0)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

                testutil::CHCKSD(&save.QNAME, save.LON, b"~", 0.0, save.QTOL, OK, ctx)?;
            }
        }
        //
        // End of vector definition loop.
        //

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Restore Alpha's radii.
            //
            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.ALPHRD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }
    //
    // End of coordinate system loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     More '=' cases follow.
    //
    //     At this point we've tested ZZGFLONG's ability to handle
    //     vector definitions and aberration corrections in combination
    //     with different coordinate systems. Now we want to perform
    //     more robust tests on the root finding aspects of ZZGFLONG.
    //     We'll stick with geometric position vectors from now on,
    //     but we'll test each coordinate system.
    //
    //     Now we want to find the times when the Alpha-Gamma vector's
    //     longitude hits each integer multiple of pi/8.
    //     This should occur every 90 minutes.
    //
    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 17:59 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.REF, b"alphafixed");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"Alpha");
    fstr::assign(&mut save.TARGET, b"Gamma");
    fstr::assign(&mut save.DSCSTR, b"Alpha-Gamma position");
    fstr::assign(&mut save.VECDEF, POSDEF);
    fstr::assign(&mut save.DREF, b" ");
    spicelib::CLEARD(3, save.DVEC.as_slice_mut());

    for CC in 1..=NLNSYS {
        //
        // Set the coordinate system and coordinate.
        //
        fstr::assign(&mut save.CRDSYS, save.CSYS.get(CC));
        fstr::assign(&mut save.CRDNAM, save.CNAM.get(CC));

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Set the longer equatorial radius of Alpha equal to
            // the shorter: unequal equatorial radii are not supported
            // for these systems.
            //
            spicelib::BODVRD(
                b"ALPHA",
                b"RADII",
                3,
                &mut save.N,
                save.ALPHRD.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Note: use of the subscript "2" in the first argument
            // below is intentional.
            //
            spicelib::VPACK(
                save.ALPHRD[2],
                save.ALPHRD[2],
                save.ALPHRD[3],
                save.TMPRAD.as_slice_mut(),
            );

            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.TMPRAD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        for VI in 1..=18 {
            //
            // Loop over each reference value.
            //
            save.REFVAL = ((((VI - 1) as f64) * spicelib::PI(ctx)) / 8.0);

            //
            // For planetographic coordinates, set the reference
            // value to indicate the same position on the unit
            // circle as that used for the other systems.
            //
            if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                save.REFVAL = (spicelib::TWOPI(ctx) - save.REFVAL);
            }

            //
            // --- Case: ------------------------------------------------------
            //

            fstr::assign(&mut save.TITLE, b"# on Alpha # (deg) meridian. #; #");

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.DSCSTR, &mut save.TITLE);
            spicelib::REPMF(
                &save.TITLE.to_vec(),
                b"#",
                (save.REFVAL * spicelib::DPR(ctx)),
                6,
                b"F",
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDNAM, &mut save.TITLE);

            testutil::TCASE(&save.TITLE, ctx)?;

            fstr::assign(&mut save.RELATE, b"=");
            save.TOL = CNVTOL;
            save.ADJUST = 0.0;

            save.RPT = false;
            save.BAIL = false;

            //
            // Set the search step size.
            //
            spicelib::GFSSTP(10000.0, ctx)?;

            //
            // We're going to set the confinement window for each
            // search so we find two roots.
            //
            spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

            save.T1 = (save.ET0 + (((VI - 1) as f64) * 5400.0));
            save.T2 = (save.T1 + ((32 as f64) * 5400.0));

            spicelib::WNINSD(save.T1, save.T2, save.CNFINE.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Perform the search.
            //
            spicelib::ZZGFLONG(
                &save.VECDEF,
                &save.METHOD,
                &save.TARGET,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                &save.DREF,
                save.DVEC.as_slice(),
                &save.CRDSYS,
                &save.CRDNAM,
                &save.RELATE,
                save.REFVAL,
                save.TOL,
                save.ADJUST,
                spicelib::GFSTEP,
                spicelib::GFREFN,
                save.RPT,
                spicelib::GFREPI,
                spicelib::GFREPU,
                spicelib::GFREPF,
                save.BAIL,
                spicelib::GFBAIL,
                MW,
                NW,
                save.WORK.as_slice_mut(),
                save.CNFINE.as_slice(),
                save.RESULT.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Make sure we found 2 roots.
            //
            save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
            testutil::CHCKSI(b"N", save.N, b"=", 2, 0, OK, ctx)?;

            for I in 1..=save.N {
                //
                // We expect to cross the VIth reference value at
                //
                //    24*(I-1) + 4.5 + 1.5*VI hours
                //
                // past noon TDB.
                //
                save.XTIME = (((((24 * (I - 1)) as f64) + (1.5 * VI as f64)) + 4.5) * 3600.0);

                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    I,
                    &mut save.START,
                    &mut save.FINISH,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut save.QNAME, b"Interval start # (0)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

                testutil::CHCKSD(&save.QNAME, save.START, b"~", save.XTIME, TIMTOL, OK, ctx)?;

                fstr::assign(&mut save.QNAME, b"Interval stop  # (0)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

                testutil::CHCKSD(&save.QNAME, save.FINISH, b"~", save.XTIME, TIMTOL, OK, ctx)?;
            }

            //
            // Check the longitude at the crossing time.
            //
            // In order to decide how accurate the coordinate should be,
            // we'll use the angular rate of Gamma together with the time
            // tolerance. Adjust the tolerance for the units we're using
            // in the CHCKSD call.
            //
            save.QTOL = ((spicelib::DPR(ctx) * (spicelib::TWOPI(ctx) / spicelib::SPD())) * CNVTOL);

            for I in 1..=save.N {
                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    I,
                    &mut save.START,
                    &mut save.FINISH,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Compute the longitude of the vector we want.
                //
                spicelib::SPKEZR(
                    &save.TARGET,
                    save.START,
                    &save.REF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::VEQU(save.STATE.as_slice(), save.POS.as_slice_mut());

                if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                    spicelib::RECPGR(
                        &save.OBSRVR,
                        save.POS.as_slice(),
                        1.0,
                        0.0,
                        &mut save.LON,
                        &mut save.LAT,
                        &mut save.ALT,
                        ctx,
                    )?;
                } else {
                    spicelib::RECLAT(
                        save.POS.as_slice(),
                        &mut save.R,
                        &mut save.LON,
                        &mut save.LAT,
                    );
                }

                fstr::assign(&mut save.QNAME, b"Longitude # (deg)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

                //
                // Adjust the longitude by 2*pi if it's too far
                // from the expected value.
                //
                if (save.LON > (save.REFVAL + 1.0)) {
                    save.LON = (save.LON - spicelib::TWOPI(ctx));
                } else if (save.LON < (save.REFVAL - 1.0)) {
                    save.LON = (save.LON + spicelib::TWOPI(ctx));
                }

                testutil::CHCKSD(
                    &save.QNAME,
                    (save.LON * spicelib::DPR(ctx)),
                    b"~",
                    (save.REFVAL * spicelib::DPR(ctx)),
                    save.QTOL,
                    OK,
                    ctx,
                )?;
            }
        }
        //
        // End of vector definition loop.
        //

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Restore Alpha's radii.
            //
            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.ALPHRD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }
    //
    // End of coordinate system loop.
    //

    //*********************************************************************
    //*
    //*    "Less than" cases
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Now we're going to work with inequality constraints.
    //
    //     We're going to start out with some very simple cases. We want to
    //     find the times when the Alpha-Gamma vector's longitude is less
    //     than each integer multiple of pi/8, where the confinement window
    //     extends 1 second before and after the crossing time. Crossings of
    //     these meridians should occur every 90 minutes.
    //
    //
    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 18:00:00 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.REF, b"alphafixed");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"Alpha");
    fstr::assign(&mut save.TARGET, b"Gamma");
    fstr::assign(&mut save.DSCSTR, b"Alpha-Gamma position longitude");
    fstr::assign(&mut save.VECDEF, POSDEF);
    fstr::assign(&mut save.DREF, b" ");
    spicelib::CLEARD(3, save.DVEC.as_slice_mut());

    for CC in 1..=NLNSYS {
        //
        // Set the coordinate system and coordinate.
        //
        fstr::assign(&mut save.CRDSYS, save.CSYS.get(CC));
        fstr::assign(&mut save.CRDNAM, save.CNAM.get(CC));

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Set the longer equatorial radius of Alpha equal to
            // the shorter: unequal equatorial radii are not supported
            // for these systems.
            //
            spicelib::BODVRD(
                b"ALPHA",
                b"RADII",
                3,
                &mut save.N,
                save.ALPHRD.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Note: use of the subscript "2" in the first argument
            // below is intentional.
            //
            spicelib::VPACK(
                save.ALPHRD[2],
                save.ALPHRD[2],
                save.ALPHRD[3],
                save.TMPRAD.as_slice_mut(),
            );

            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.TMPRAD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        for VI in 1..=17 {
            //
            // Loop over each reference value. Include both 0 and 2*pi.
            //
            save.REFVAL = ((((VI - 1) as f64) * spicelib::PI(ctx)) / 8.0);

            //
            // For planetographic coordinates, set the reference
            // value to indicate the same position on the unit
            // circle as that used for the other systems.
            //
            if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                save.REFVAL = (spicelib::TWOPI(ctx) - save.REFVAL);
            }

            if (save.REFVAL == spicelib::TWOPI(ctx)) {
                save.REFVAL = 0.0;
            }

            if (save.REFVAL < 0.0) {
                save.REFVAL = (save.REFVAL + spicelib::TWOPI(ctx));
            }

            //
            // --- Case: ------------------------------------------------------
            //

            fstr::assign(&mut save.TITLE, b"# less than # (deg) #; #; 2-sec CONFINE");

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.DSCSTR, &mut save.TITLE);
            spicelib::REPMF(
                &save.TITLE.to_vec(),
                b"#",
                (save.REFVAL * spicelib::DPR(ctx)),
                6,
                b"F",
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDNAM, &mut save.TITLE);

            testutil::TCASE(&save.TITLE, ctx)?;

            fstr::assign(&mut save.RELATE, b"<");
            save.TOL = CNVTOL;
            save.ADJUST = 0.0;

            save.RPT = false;
            save.BAIL = false;

            //
            // Set the search step size.
            //
            spicelib::GFSSTP(10000.0, ctx)?;

            //
            // We're going to set the confinement window for each
            // search so we cover one reference value crossing.
            //
            spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

            save.T1 = ((save.ET0 + (((VI - 1) as f64) * 5400.0)) - 1.0);
            save.T2 = (save.T1 + 2.0);

            spicelib::WNINSD(save.T1, save.T2, save.CNFINE.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Perform the search.
            //
            spicelib::ZZGFLONG(
                &save.VECDEF,
                &save.METHOD,
                &save.TARGET,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                &save.DREF,
                save.DVEC.as_slice(),
                &save.CRDSYS,
                &save.CRDNAM,
                &save.RELATE,
                save.REFVAL,
                save.TOL,
                save.ADJUST,
                spicelib::GFSTEP,
                spicelib::GFREFN,
                save.RPT,
                spicelib::GFREPI,
                spicelib::GFREPU,
                spicelib::GFREPF,
                save.BAIL,
                spicelib::GFBAIL,
                MW,
                NW,
                save.WORK.as_slice_mut(),
                save.CNFINE.as_slice(),
                save.RESULT.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Filter the result window to remove spurious
            // intervals that may arise due to convergence
            // error.
            //
            spicelib::WNFLTD(save.TOL, save.RESULT.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // We expect to cross the VIth reference value at
            //
            //    4.5 + 1.5D0*VI hours
            //
            // past noon TDB.
            //
            //
            save.XTIME = (((1.5 * VI as f64) + 4.5) * 3600.0);

            //
            // Normally we expect to find one solution interval, but
            // the result window should be empty if the constraint
            // can't be satisfied.
            //
            save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            // WRITE (*,'(A,I15,E25.16)') 'N, REFVAL = ', N, REFVAL

            if ((spicelib::EQSTR(&save.CRDSYS, CYLSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS))
                || spicelib::EQSTR(&save.CRDSYS, RADSYS))
            {
                if (f64::cos(save.REFVAL) == 1.0) {
                    //
                    // The condition
                    //
                    //    longitude < REFVAL
                    //
                    // is never satisfied, so the window should be empty.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;
                } else {
                    //
                    // Make sure we found 1 interval.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
                }
            } else {
                //
                // Make sure we found 1 interval.
                //
                testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
            }

            //
            // Note: in the code below, we don't expect to find
            // result windows containing multiple intervals.
            // But we format the code to handle multiple intervals
            // so we can easily re-use the code in cases where
            // multiple intervals are expected.
            //
            //
            // Now check the solution intervals.
            //
            for I in 1..=save.N {
                //
                // Look up the Ith solution interval.
                //
                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    I,
                    &mut save.START,
                    &mut save.FINISH,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if spicelib::EQSTR(&save.CRDSYS, b"PLANETOGRAPHIC") {
                    if (I == 1) {
                        //
                        // This is the normal case.
                        //
                        // WRITE (*,*) 1, START, FINISH
                        //
                        // The stop time should be T2; the start time should
                        // be the time of the reference meridian crossing.
                        //
                        fstr::assign(&mut save.QNAME, b"Interval start  # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        testutil::CHCKSD(
                            &save.QNAME,
                            save.START,
                            b"~",
                            save.XTIME,
                            TIMTOL,
                            OK,
                            ctx,
                        )?;

                        fstr::assign(&mut save.QNAME, b"Interval stop # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        testutil::CHCKSD(&save.QNAME, save.FINISH, b"~", save.T2, TIMTOL, OK, ctx)?;

                        save.XET = save.XTIME;
                        save.ET = save.START;
                    }
                } else if (spicelib::EQSTR(&save.CRDSYS, b"CYLINDRICAL")
                    || spicelib::EQSTR(&save.CRDSYS, b"RA/DEC"))
                {
                    //
                    // Longitude is positive East. The branch cut is at 0.
                    //
                    if (I == 1) {
                        //
                        // This is the normal case.
                        //
                        // The start time should be T1. If the reference
                        // value is at the branch cut, the stop time should
                        // be T2. Otherwise, the stop time should be the time
                        // of the reference meridian crossing.
                        //
                        //
                        fstr::assign(&mut save.QNAME, b"Interval start # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        testutil::CHCKSD(&save.QNAME, save.START, b"~", save.T1, TIMTOL, OK, ctx)?;

                        fstr::assign(&mut save.QNAME, b"Interval stop  # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        if (f64::cos(save.REFVAL) == 1.0) {
                            save.XET = save.T2;
                            save.ET = save.FINISH;
                        } else {
                            save.XET = save.XTIME;
                            save.ET = save.FINISH;
                        }

                        testutil::CHCKSD(
                            &save.QNAME,
                            save.FINISH,
                            b"~",
                            save.XET,
                            TIMTOL,
                            OK,
                            ctx,
                        )?;
                    }
                } else if ((spicelib::EQSTR(&save.CRDSYS, b"LATITUDINAL")
                    || spicelib::EQSTR(&save.CRDSYS, b"GEODETIC"))
                    || spicelib::EQSTR(&save.CRDSYS, b"SPHERICAL"))
                {
                    //
                    // Longitude is positive East. The branch cut is at Pi.
                    //
                    if (I == 1) {
                        //
                        // This is the normal case.
                        //
                        // The start time should be T1. If the reference
                        // value is at the branch cut, the stop time should
                        // be T2. Otherwise, the stop time should be the time
                        // of the reference meridian crossing.
                        //
                        //
                        fstr::assign(&mut save.QNAME, b"Interval start # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        testutil::CHCKSD(&save.QNAME, save.START, b"~", save.T1, TIMTOL, OK, ctx)?;

                        fstr::assign(&mut save.QNAME, b"Interval stop  # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        if (f64::cos(save.REFVAL) == -1.0) {
                            save.XET = save.T2;
                        } else {
                            save.XET = save.XTIME;
                        }

                        save.ET = save.FINISH;

                        testutil::CHCKSD(&save.QNAME, save.ET, b"~", save.XET, TIMTOL, OK, ctx)?;
                    }
                } else {
                    spicelib::SETMSG(b"Unexpected system: #", ctx);
                    spicelib::ERRCH(b"#", &save.CRDSYS, ctx);
                    spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // Check the longitude at the crossing time ET.
                //
                // In order to decide how accurate the coordinate should
                // be, we'll use the angular rate of Gamma together with
                // the time tolerance. Adjust the tolerance for the units
                // we're using in the CHCKSD call.
                //
                save.QTOL =
                    ((spicelib::DPR(ctx) * (spicelib::TWOPI(ctx) / spicelib::SPD())) * CNVTOL);

                //
                // Compute the longitude of the vector we expect and
                // the longitude at FINISH.
                //
                spicelib::SPKEZR(
                    &save.TARGET,
                    save.XET,
                    &save.REF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::VEQU(save.STATE.as_slice(), save.POS.as_slice_mut());

                if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                    spicelib::RECPGR(
                        &save.OBSRVR,
                        save.POS.as_slice(),
                        1.0,
                        0.0,
                        &mut save.XLON,
                        &mut save.LAT,
                        &mut save.ALT,
                        ctx,
                    )?;
                } else {
                    spicelib::RECLAT(
                        save.POS.as_slice(),
                        &mut save.R,
                        &mut save.XLON,
                        &mut save.LAT,
                    );
                }

                spicelib::SPKEZR(
                    &save.TARGET,
                    save.ET,
                    &save.REF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::VEQU(save.STATE.as_slice(), save.POS.as_slice_mut());

                if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                    spicelib::RECPGR(
                        &save.OBSRVR,
                        save.POS.as_slice(),
                        1.0,
                        0.0,
                        &mut save.LON,
                        &mut save.LAT,
                        &mut save.ALT,
                        ctx,
                    )?;
                } else {
                    spicelib::RECLAT(
                        save.POS.as_slice(),
                        &mut save.R,
                        &mut save.LON,
                        &mut save.LAT,
                    );
                }

                fstr::assign(&mut save.QNAME, b"Longitude # (deg)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

                //
                // Adjust the longitude by 2*pi if it's too far
                // from the expected value.
                //
                if (save.LON > (save.XLON + 1.0)) {
                    save.LON = (save.LON - spicelib::TWOPI(ctx));
                } else if (save.LON < (save.XLON - 1.0)) {
                    save.LON = (save.LON + spicelib::TWOPI(ctx));
                }

                testutil::CHCKSD(
                    &save.QNAME,
                    (save.LON * spicelib::DPR(ctx)),
                    b"~",
                    (save.XLON * spicelib::DPR(ctx)),
                    save.QTOL,
                    OK,
                    ctx,
                )?;
            }
            //
            // We're done with the Ith interval.
            //
        }
        //
        // End of vector definition loop.
        //

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Restore Alpha's radii.
            //
            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.ALPHRD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }
    //
    // End of coordinate system loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    //     Repeat the previous case, but this time keep the confinement
    //     window start time fixed at 18:00 Jan 1 2000 TDB. We want to find
    //     the times when the Alpha-Gamma vector's longitude is less than
    //     each integer multiple of pi/8, where the confinement window
    //     extends 1 second before and after the crossing time. Crossings of
    //     these meridians should occur every 90 minutes.
    //
    //
    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 18:00:00 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.REF, b"alphafixed");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"Alpha");
    fstr::assign(&mut save.TARGET, b"Gamma");
    fstr::assign(&mut save.DSCSTR, b"Alpha-Gamma position longitude");
    fstr::assign(&mut save.VECDEF, POSDEF);
    fstr::assign(&mut save.DREF, b" ");
    spicelib::CLEARD(3, save.DVEC.as_slice_mut());

    for CC in 1..=NLNSYS {
        //
        // Set the coordinate system and coordinate.
        //
        fstr::assign(&mut save.CRDSYS, save.CSYS.get(CC));
        fstr::assign(&mut save.CRDNAM, save.CNAM.get(CC));

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Set the longer equatorial radius of Alpha equal to
            // the shorter: unequal equatorial radii are not supported
            // for these systems.
            //
            spicelib::BODVRD(
                b"ALPHA",
                b"RADII",
                3,
                &mut save.N,
                save.ALPHRD.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Note: use of the subscript "2" in the first argument
            // below is intentional.
            //
            spicelib::VPACK(
                save.ALPHRD[2],
                save.ALPHRD[2],
                save.ALPHRD[3],
                save.TMPRAD.as_slice_mut(),
            );

            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.TMPRAD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        for VI in 1..=17 {
            //
            // Loop over each reference value. Include both 0 and 2*pi.
            //
            save.REFVAL = ((((VI - 1) as f64) * spicelib::PI(ctx)) / 8.0);

            //
            // For planetographic coordinates, set the reference
            // value to indicate the same position on the unit
            // circle as that used for the other systems.
            //
            if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                save.REFVAL = (spicelib::TWOPI(ctx) - save.REFVAL);
            }

            if (save.REFVAL == spicelib::TWOPI(ctx)) {
                save.REFVAL = 0.0;
            }

            if (save.REFVAL < 0.0) {
                save.REFVAL = (save.REFVAL + spicelib::TWOPI(ctx));
            }

            //
            // --- Case: ------------------------------------------------------
            //

            fstr::assign(
                &mut save.TITLE,
                b"# less than # (deg) #; #; fixed CNFINE start",
            );

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.DSCSTR, &mut save.TITLE);
            spicelib::REPMF(
                &save.TITLE.to_vec(),
                b"#",
                (save.REFVAL * spicelib::DPR(ctx)),
                6,
                b"F",
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDNAM, &mut save.TITLE);

            testutil::TCASE(&save.TITLE, ctx)?;

            fstr::assign(&mut save.RELATE, b"<");
            save.TOL = CNVTOL;
            save.ADJUST = 0.0;

            save.RPT = false;
            save.BAIL = false;

            //
            // Set the search step size.
            //
            spicelib::GFSSTP(10000.0, ctx)?;

            //
            // We're going to set the confinement window for each
            // search so we cover one reference value crossing.
            //
            spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

            save.T1 = (save.ET0 + (CNVTOL / 10.0));
            save.T2 = ((save.ET0 + (((VI - 1) as f64) * 5400.0)) + 1.0);

            spicelib::WNINSD(save.T1, save.T2, save.CNFINE.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Perform the search.
            //
            spicelib::ZZGFLONG(
                &save.VECDEF,
                &save.METHOD,
                &save.TARGET,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                &save.DREF,
                save.DVEC.as_slice(),
                &save.CRDSYS,
                &save.CRDNAM,
                &save.RELATE,
                save.REFVAL,
                save.TOL,
                save.ADJUST,
                spicelib::GFSTEP,
                spicelib::GFREFN,
                save.RPT,
                spicelib::GFREPI,
                spicelib::GFREPU,
                spicelib::GFREPF,
                save.BAIL,
                spicelib::GFBAIL,
                MW,
                NW,
                save.WORK.as_slice_mut(),
                save.CNFINE.as_slice(),
                save.RESULT.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Filter the result window to remove spurious
            // intervals that may arise due to convergence
            // error.
            //
            spicelib::WNFLTD(save.TOL, save.RESULT.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // We expect to cross the VIth reference value at
            //
            //    4.5 + 1.5D0*VI hours
            //
            //    past noon TDB.
            //
            //
            save.XTIME = (((1.5 * VI as f64) + 4.5) * 3600.0);

            //
            // Normally we expect to find one solution interval, but
            // the result window should be empty if the constraint
            // can't be satisfied.
            //
            save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

            // WRITE (*,'(A,I15,E25.16)') 'N, REFVAL = ', N, REFVAL

            if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                if (f64::cos(save.REFVAL) == 1.0) {
                    //
                    // The condition
                    //
                    //    longitude < REFVAL
                    //
                    // is never satisfied, so the window should be empty.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;
                } else {
                    //
                    // Make sure we found 1 interval.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
                }
            } else if (spicelib::EQSTR(&save.CRDSYS, CYLSYS)
                || spicelib::EQSTR(&save.CRDSYS, RADSYS))
            {
                if (f64::cos(save.REFVAL) == 1.0) {
                    //
                    // The condition
                    //
                    //    longitude < REFVAL
                    //
                    // is never satisfied, so the window should be empty.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;
                } else {
                    //
                    // Make sure we found 1 interval.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
                }
            } else if (spicelib::EQSTR(&save.CRDSYS, CYLSYS)
                || spicelib::EQSTR(&save.CRDSYS, RADSYS))
            {
                //
                // Make sure we found 1 interval.
                //
                testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
            } else if ((spicelib::EQSTR(&save.CRDSYS, LATSYS)
                || spicelib::EQSTR(&save.CRDSYS, GEOSYS))
                || spicelib::EQSTR(&save.CRDSYS, SPHSYS))
            {
                //
                // The number of expected intervals depends on the
                // case index.
                //
                if (VI == 1) {
                    //
                    // The condition
                    //
                    //    longitude < REFVAL
                    //
                    // is never satisfied, so the window should be empty.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;
                //
                } else {
                    //
                    // Make sure we found 1 interval.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
                }
            }

            //
            // Now check the solution intervals.
            //
            for I in 1..=save.N {
                //
                // Look up the Ith solution interval.
                //
                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    I,
                    &mut save.START,
                    &mut save.FINISH,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if spicelib::EQSTR(&save.CRDSYS, b"PLANETOGRAPHIC") {
                    if (I == 1) {
                        //
                        // This is the normal case.
                        //
                        // WRITE (*,*) 1, START, FINISH
                        //
                        // The stop time should be T2; the start time should
                        // be the time of the reference meridian crossing.
                        //
                        fstr::assign(&mut save.QNAME, b"Interval start  # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        testutil::CHCKSD(
                            &save.QNAME,
                            save.START,
                            b"~",
                            save.XTIME,
                            TIMTOL,
                            OK,
                            ctx,
                        )?;

                        fstr::assign(&mut save.QNAME, b"Interval stop # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        testutil::CHCKSD(&save.QNAME, save.FINISH, b"~", save.T2, TIMTOL, OK, ctx)?;

                        save.XET = save.XTIME;
                        save.ET = save.START;
                    } else {
                        //
                        // We don't expect to have more than one
                        // interval for this case.
                        //
                        spicelib::SETMSG(b"Unexpected interval count: #; VI = #", ctx);
                        spicelib::ERRINT(b"#", save.N, ctx);
                        spicelib::ERRINT(b"#", VI, ctx);
                        spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }
                } else if (spicelib::EQSTR(&save.CRDSYS, b"CYLINDRICAL")
                    || spicelib::EQSTR(&save.CRDSYS, b"RA/DEC"))
                {
                    //
                    // Longitude is positive East. The branch cut is at 0.
                    //
                    if (I == 1) {
                        //
                        // This is the normal case.
                        //
                        // The start time should be T1. If the reference
                        // value is at the branch cut, the stop time should
                        // be T2. Otherwise, the stop time should be the time
                        // of the reference meridian crossing.
                        //
                        //
                        fstr::assign(&mut save.QNAME, b"Interval start # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        testutil::CHCKSD(&save.QNAME, save.START, b"~", save.T1, TIMTOL, OK, ctx)?;

                        fstr::assign(&mut save.QNAME, b"Interval stop  # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        if (f64::cos(save.REFVAL) == 1.0) {
                            save.XET = save.T2;
                            save.ET = save.FINISH;
                        } else {
                            save.XET = save.XTIME;
                            save.ET = save.FINISH;
                        }

                        testutil::CHCKSD(
                            &save.QNAME,
                            save.FINISH,
                            b"~",
                            save.XET,
                            TIMTOL,
                            OK,
                            ctx,
                        )?;
                    }
                } else if ((spicelib::EQSTR(&save.CRDSYS, b"LATITUDINAL")
                    || spicelib::EQSTR(&save.CRDSYS, b"GEODETIC"))
                    || spicelib::EQSTR(&save.CRDSYS, b"SPHERICAL"))
                {
                    //
                    // Longitude is positive East. The branch cut is at Pi.
                    //
                    if (I == 1) {
                        //
                        // This is the normal case.
                        //
                        // If REFVAL is in the open upper half of the unit
                        // circle, the start time should be ET0 and the
                        // stop time should be the time of the reference
                        // meridian crossing.
                        //
                        // If REFVAL is pi, the solution set includes the
                        // interval from XTIME to T2. We expect this
                        // interval to be merged with the interval for the
                        // top half of the circle, so the expected end
                        // time becomes T2.
                        //
                        // Otherwise, the first interval should be the
                        // singleton [ET0, ET0].
                        //
                        fstr::assign(&mut save.QNAME, b"Interval start # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        if (f64::sin(save.REFVAL) > 0.0) {
                            testutil::CHCKSD(
                                &save.QNAME,
                                save.START,
                                b"~",
                                save.ET0,
                                TIMTOL,
                                OK,
                                ctx,
                            )?;
                        } else {
                            testutil::CHCKSD(
                                &save.QNAME,
                                save.START,
                                b"~",
                                (save.ET0 + (spicelib::SPD() / 2 as f64)),
                                TIMTOL,
                                OK,
                                ctx,
                            )?;
                        }

                        // IF ( .NOT. OK ) THEN
                        //    WRITE (*,*) 'VI = ', VI
                        // END IF

                        fstr::assign(&mut save.QNAME, b"Interval stop  # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        if (f64::cos(save.REFVAL) == -1.0) {
                            save.XET = save.T2;
                        } else {
                            save.XET = save.XTIME;
                        }

                        save.ET = save.FINISH;

                        testutil::CHCKSD(&save.QNAME, save.ET, b"~", save.XET, TIMTOL, OK, ctx)?;
                    } else {
                        //
                        // We don't expect to have more than one
                        // interval for this case.
                        //
                        spicelib::SETMSG(b"Unexpected interval count: #; VI = #", ctx);
                        spicelib::ERRINT(b"#", save.N, ctx);
                        spicelib::ERRINT(b"#", VI, ctx);
                        spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }
                } else {
                    spicelib::SETMSG(b"Unexpected system: #", ctx);
                    spicelib::ERRCH(b"#", &save.CRDSYS, ctx);
                    spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // Check the longitude at the crossing time ET.
                //
                // In order to decide how accurate the coordinate should
                // be, we'll use the angular rate of Gamma together with
                // the time tolerance. Adjust the tolerance for the units
                // we're using in the CHCKSD call.
                //
                save.QTOL =
                    ((spicelib::DPR(ctx) * (spicelib::TWOPI(ctx) / spicelib::SPD())) * CNVTOL);

                //
                // Compute the longitude of the vector we expect and
                // the longitude at FINISH.
                //
                spicelib::SPKEZR(
                    &save.TARGET,
                    save.XET,
                    &save.REF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::VEQU(save.STATE.as_slice(), save.POS.as_slice_mut());

                if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                    spicelib::RECPGR(
                        &save.OBSRVR,
                        save.POS.as_slice(),
                        1.0,
                        0.0,
                        &mut save.XLON,
                        &mut save.LAT,
                        &mut save.ALT,
                        ctx,
                    )?;
                } else {
                    spicelib::RECLAT(
                        save.POS.as_slice(),
                        &mut save.R,
                        &mut save.XLON,
                        &mut save.LAT,
                    );
                }

                spicelib::SPKEZR(
                    &save.TARGET,
                    save.ET,
                    &save.REF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::VEQU(save.STATE.as_slice(), save.POS.as_slice_mut());

                if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                    spicelib::RECPGR(
                        &save.OBSRVR,
                        save.POS.as_slice(),
                        1.0,
                        0.0,
                        &mut save.LON,
                        &mut save.LAT,
                        &mut save.ALT,
                        ctx,
                    )?;
                } else {
                    spicelib::RECLAT(
                        save.POS.as_slice(),
                        &mut save.R,
                        &mut save.LON,
                        &mut save.LAT,
                    );
                }

                fstr::assign(&mut save.QNAME, b"Longitude # (deg)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

                //
                // Adjust the longitude by 2*pi if it's too far
                // from the expected value.
                //
                if (save.LON > (save.XLON + 1.0)) {
                    save.LON = (save.LON - spicelib::TWOPI(ctx));
                } else if (save.LON < (save.XLON - 1.0)) {
                    save.LON = (save.LON + spicelib::TWOPI(ctx));
                }

                testutil::CHCKSD(
                    &save.QNAME,
                    (save.LON * spicelib::DPR(ctx)),
                    b"~",
                    (save.XLON * spicelib::DPR(ctx)),
                    save.QTOL,
                    OK,
                    ctx,
                )?;
            }
            //
            // We're done with the Ith interval.
            //
        }
        //
        // End of vector definition loop.
        //

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Restore Alpha's radii.
            //
            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.ALPHRD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }
    //
    // End of coordinate system loop.
    //

    //*********************************************************************
    //*
    //*    "Greater than" cases
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // '>' Tests
    //
    // We're going to try some very simple '>' cases. We want to
    // find the times when the Alpha-Gamma vector's longitude is greater
    // than each integer multiple of pi/8, where the confinement window
    // extends 1 second before and after the crossing time. Crossings of
    // these meridians should occur every 90 minutes.
    //
    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 18:00:00 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.REF, b"alphafixed");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"Alpha");
    fstr::assign(&mut save.TARGET, b"Gamma");
    fstr::assign(&mut save.DSCSTR, b"Alpha-Gamma position longitude");
    fstr::assign(&mut save.VECDEF, POSDEF);
    fstr::assign(&mut save.DREF, b" ");
    spicelib::CLEARD(3, save.DVEC.as_slice_mut());

    for CC in 1..=NLNSYS {
        //
        // Set the coordinate system and coordinate.
        //
        fstr::assign(&mut save.CRDSYS, save.CSYS.get(CC));
        fstr::assign(&mut save.CRDNAM, save.CNAM.get(CC));

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Set the longer equatorial radius of Alpha equal to
            // the shorter: unequal equatorial radii are not supported
            // for these systems.
            //
            spicelib::BODVRD(
                b"ALPHA",
                b"RADII",
                3,
                &mut save.N,
                save.ALPHRD.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Note: use of the subscript "2" in the first argument
            // below is intentional.
            //
            spicelib::VPACK(
                save.ALPHRD[2],
                save.ALPHRD[2],
                save.ALPHRD[3],
                save.TMPRAD.as_slice_mut(),
            );

            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.TMPRAD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        for VI in 1..=17 {
            //
            // Loop over each reference value. Include both 0 and 2*pi.
            //
            save.REFVAL = ((((VI - 1) as f64) * spicelib::PI(ctx)) / 8.0);

            //
            // For planetographic coordinates, set the reference
            // value to indicate the same position on the unit
            // circle as that used for the other systems.
            //
            if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                save.REFVAL = (spicelib::TWOPI(ctx) - save.REFVAL);
            }

            if (save.REFVAL == spicelib::TWOPI(ctx)) {
                save.REFVAL = 0.0;
            }

            if (save.REFVAL < 0.0) {
                save.REFVAL = (save.REFVAL + spicelib::TWOPI(ctx));
            }

            //
            // --- Case: ------------------------------------------------------
            //

            fstr::assign(
                &mut save.TITLE,
                b"# greater than # (deg) #; #; 2-sec CONFINE",
            );

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.DSCSTR, &mut save.TITLE);
            spicelib::REPMF(
                &save.TITLE.to_vec(),
                b"#",
                (save.REFVAL * spicelib::DPR(ctx)),
                6,
                b"F",
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDNAM, &mut save.TITLE);

            testutil::TCASE(&save.TITLE, ctx)?;

            fstr::assign(&mut save.RELATE, b">");
            save.TOL = CNVTOL;
            save.ADJUST = 0.0;

            save.RPT = false;
            save.BAIL = false;

            //
            // Set the search step size.
            //
            spicelib::GFSSTP(10000.0, ctx)?;

            //
            // We're going to set the confinement window for each
            // search so we cover one reference value crossing.
            //
            spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

            save.T1 = ((save.ET0 + (((VI - 1) as f64) * 5400.0)) - 1.0);
            save.T2 = (save.T1 + 2.0);

            spicelib::WNINSD(save.T1, save.T2, save.CNFINE.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Perform the search.
            //
            spicelib::ZZGFLONG(
                &save.VECDEF,
                &save.METHOD,
                &save.TARGET,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                &save.DREF,
                save.DVEC.as_slice(),
                &save.CRDSYS,
                &save.CRDNAM,
                &save.RELATE,
                save.REFVAL,
                save.TOL,
                save.ADJUST,
                spicelib::GFSTEP,
                spicelib::GFREFN,
                save.RPT,
                spicelib::GFREPI,
                spicelib::GFREPU,
                spicelib::GFREPF,
                save.BAIL,
                spicelib::GFBAIL,
                MW,
                NW,
                save.WORK.as_slice_mut(),
                save.CNFINE.as_slice(),
                save.RESULT.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // We expect to cross the VIth reference value at
            //
            //    24*(I-1) + 4.5 + 1.5D0*VI hours
            //
            //    past noon TDB.
            //
            //
            save.XTIME = (((1.5 * VI as f64) + 4.5) * 3600.0);

            //
            // Filter the result window to remove spurious
            // intervals that may arise due to convergence
            // error.
            //
            spicelib::WNFLTD(save.TOL, save.RESULT.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Normally we expect to find one solution interval, but
            // the result window should be empty if the constraint
            // can't be satisfied.
            //
            save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

            // WRITE (*,'(A,I15,E25.16)') 'N, REFVAL = ', N, REFVAL

            if ((spicelib::EQSTR(&save.CRDSYS, CYLSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS))
                || spicelib::EQSTR(&save.CRDSYS, RADSYS))
            {
                //
                // Make sure we found 1 interval.
                //
                testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
            } else {
                if (f64::cos(save.REFVAL) == -1 as f64) {
                    //
                    // We don't expect to find any intervals.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;
                } else {
                    //
                    // Make sure we found 1 interval.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
                }
            }

            //
            // Note: in the code below, we don't expect to find
            // result windows containing multiple intervals.
            // But we format the code to handle multiple intervals
            // so we can easily re-use the code in cases where
            // multiple intervals are expected.
            //
            //
            // Now check the solution intervals.
            //
            for I in 1..=save.N {
                //
                // Look up the Ith solution interval.
                //
                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    I,
                    &mut save.START,
                    &mut save.FINISH,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                if spicelib::EQSTR(&save.CRDSYS, b"PLANETOGRAPHIC") {
                    if (I == 1) {
                        //
                        // This is the normal case.
                        //
                        // WRITE (*,*) 1, START, FINISH
                        //
                        if (f64::cos(save.REFVAL) == 1 as f64) {
                            //
                            // The stop time actually should be T2, since the
                            // interval from T1 to XTIME satisfies the
                            // inequality.
                            //
                            save.XET = save.T2;
                        } else {
                            //
                            // The start time should be XTIME. The stop time
                            // should be T2.
                            //
                            save.XET = save.XTIME;
                        }

                        // The start time should be T1; the stop time should
                        // be the time of the reference meridian crossing.
                        //
                        fstr::assign(&mut save.QNAME, b"Interval start  # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        testutil::CHCKSD(&save.QNAME, save.START, b"~", save.T1, TIMTOL, OK, ctx)?;

                        fstr::assign(&mut save.QNAME, b"Interval stop # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        save.ET = save.FINISH;

                        testutil::CHCKSD(&save.QNAME, save.ET, b"~", save.XET, TIMTOL, OK, ctx)?;
                    }
                } else if (spicelib::EQSTR(&save.CRDSYS, b"CYLINDRICAL")
                    || spicelib::EQSTR(&save.CRDSYS, b"RA/DEC"))
                {
                    //
                    // Longitude is positive East. The branch cut is at 0.
                    //
                    if (I == 1) {
                        //
                        // This is the normal case.
                        //
                        if (f64::cos(save.REFVAL) == 1 as f64) {
                            //
                            // The start time actually should be T1, since the
                            // interval from T1 to XTIME satisfies the
                            // inequality.
                            //
                            save.XET = save.T1;
                        } else {
                            //
                            // The start time should be XTIME. The stop time
                            // should be T2.
                            //
                            save.XET = save.XTIME;
                        }

                        fstr::assign(&mut save.QNAME, b"Interval start # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        save.ET = save.START;

                        testutil::CHCKSD(&save.QNAME, save.ET, b"~", save.XET, TIMTOL, OK, ctx)?;

                        fstr::assign(&mut save.QNAME, b"Interval stop  # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        testutil::CHCKSD(&save.QNAME, save.FINISH, b"~", save.T2, TIMTOL, OK, ctx)?;
                    }
                } else if ((spicelib::EQSTR(&save.CRDSYS, b"LATITUDINAL")
                    || spicelib::EQSTR(&save.CRDSYS, b"GEODETIC"))
                    || spicelib::EQSTR(&save.CRDSYS, b"SPHERICAL"))
                {
                    //
                    // Longitude is positive East. The branch cut is at Pi.
                    //
                    if (I == 1) {
                        //
                        // This is the normal case.
                        //
                        // The start time should be XTIME. The stop time
                        // should be T2.
                        //
                        fstr::assign(&mut save.QNAME, b"Interval start # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        save.ET = save.START;
                        save.XET = save.XTIME;

                        testutil::CHCKSD(&save.QNAME, save.ET, b"~", save.XET, TIMTOL, OK, ctx)?;

                        fstr::assign(&mut save.QNAME, b"Interval stop  # (0)");
                        spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                        testutil::CHCKSD(&save.QNAME, save.FINISH, b"~", save.T2, TIMTOL, OK, ctx)?;
                    }
                } else {
                    spicelib::SETMSG(b"Unexpected system: #", ctx);
                    spicelib::ERRCH(b"#", &save.CRDSYS, ctx);
                    spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // Check the longitude at the crossing time ET.
                //
                // In order to decide how accurate the coordinate should
                // be, we'll use the angular rate of Gamma together with
                // the time tolerance. Adjust the tolerance for the units
                // we're using in the CHCKSD call.
                //
                save.QTOL =
                    ((spicelib::DPR(ctx) * (spicelib::TWOPI(ctx) / spicelib::SPD())) * CNVTOL);

                //
                // Compute the longitude of the vector we expect and
                // the longitude at FINISH.
                //
                spicelib::SPKEZR(
                    &save.TARGET,
                    save.XET,
                    &save.REF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::VEQU(save.STATE.as_slice(), save.POS.as_slice_mut());

                if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                    spicelib::RECPGR(
                        &save.OBSRVR,
                        save.POS.as_slice(),
                        1.0,
                        0.0,
                        &mut save.XLON,
                        &mut save.LAT,
                        &mut save.ALT,
                        ctx,
                    )?;
                } else {
                    spicelib::RECLAT(
                        save.POS.as_slice(),
                        &mut save.R,
                        &mut save.XLON,
                        &mut save.LAT,
                    );
                }

                spicelib::SPKEZR(
                    &save.TARGET,
                    save.ET,
                    &save.REF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::VEQU(save.STATE.as_slice(), save.POS.as_slice_mut());

                if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                    spicelib::RECPGR(
                        &save.OBSRVR,
                        save.POS.as_slice(),
                        1.0,
                        0.0,
                        &mut save.LON,
                        &mut save.LAT,
                        &mut save.ALT,
                        ctx,
                    )?;
                } else {
                    spicelib::RECLAT(
                        save.POS.as_slice(),
                        &mut save.R,
                        &mut save.LON,
                        &mut save.LAT,
                    );
                }

                fstr::assign(&mut save.QNAME, b"Longitude # (deg)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

                //
                // Adjust the longitude by 2*pi if it's too far
                // from the expected value.
                //
                if (save.LON > (save.XLON + 1.0)) {
                    save.LON = (save.LON - spicelib::TWOPI(ctx));
                } else if (save.LON < (save.XLON - 1.0)) {
                    save.LON = (save.LON + spicelib::TWOPI(ctx));
                }

                testutil::CHCKSD(
                    &save.QNAME,
                    (save.LON * spicelib::DPR(ctx)),
                    b"~",
                    (save.XLON * spicelib::DPR(ctx)),
                    save.QTOL,
                    OK,
                    ctx,
                )?;
            }
            //
            // We're done with the Ith interval.
            //
        }
        //
        // End of vector definition loop.
        //

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Restore Alpha's radii.
            //
            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.ALPHRD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }
    //
    // End of coordinate system loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    //     Repeat the previous case, but this time keep the confinement
    //     window stop time fixed at 18:00 Jan 2 2000 TDB. We want to
    //     find the times when the Alpha-Gamma vector's longitude is greater
    //     than each integer multiple of pi/8. Crossings of
    //     these meridians should occur every 90 minutes.
    //
    //
    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 18:00:00 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET1 = (save.ET0 + spicelib::SPD());

    fstr::assign(&mut save.REF, b"alphafixed");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"Alpha");
    fstr::assign(&mut save.TARGET, b"Gamma");
    fstr::assign(&mut save.DSCSTR, b"Alpha-Gamma position longitude");
    fstr::assign(&mut save.VECDEF, POSDEF);
    fstr::assign(&mut save.DREF, b" ");
    spicelib::CLEARD(3, save.DVEC.as_slice_mut());

    for CC in 1..=NLNSYS {
        //
        // Set the coordinate system and coordinate.
        //
        fstr::assign(&mut save.CRDSYS, save.CSYS.get(CC));
        fstr::assign(&mut save.CRDNAM, save.CNAM.get(CC));

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Set the longer equatorial radius of Alpha equal to
            // the shorter: unequal equatorial radii are not supported
            // for these systems.
            //
            spicelib::BODVRD(
                b"ALPHA",
                b"RADII",
                3,
                &mut save.N,
                save.ALPHRD.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Note: use of the subscript "2" in the first argument
            // below is intentional.
            //
            spicelib::VPACK(
                save.ALPHRD[2],
                save.ALPHRD[2],
                save.ALPHRD[3],
                save.TMPRAD.as_slice_mut(),
            );

            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.TMPRAD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        for VI in 1..=17 {
            //
            // Loop over each reference value. Include both 0 and 2*pi.
            //
            save.REFVAL = ((((VI - 1) as f64) * spicelib::PI(ctx)) / 8.0);

            //
            // For planetographic coordinates, set the reference
            // value to indicate the same position on the unit
            // circle as that used for the other systems.
            //
            if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                save.REFVAL = (spicelib::TWOPI(ctx) - save.REFVAL);
            }

            if (save.REFVAL < 0.0) {
                save.REFVAL = (save.REFVAL + spicelib::TWOPI(ctx));
            }

            //
            // --- Case: ------------------------------------------------------
            //

            fstr::assign(
                &mut save.TITLE,
                b"# greater than # (deg) #; #; fixed CNFINE stop",
            );

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.DSCSTR, &mut save.TITLE);
            spicelib::REPMF(
                &save.TITLE.to_vec(),
                b"#",
                (save.REFVAL * spicelib::DPR(ctx)),
                6,
                b"F",
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDNAM, &mut save.TITLE);

            testutil::TCASE(&save.TITLE, ctx)?;

            fstr::assign(&mut save.RELATE, b">");
            save.TOL = CNVTOL;
            save.ADJUST = 0.0;

            save.RPT = false;
            save.BAIL = false;

            //
            // Set the search step size.
            //
            spicelib::GFSSTP(10000.0, ctx)?;

            //
            // We're going to set the confinement window for each
            // search so we cover one reference value crossing.
            //
            spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

            save.T1 = ((save.ET0 + (((VI - 1) as f64) * 5400.0)) - 1.0);
            save.T2 = save.ET1;

            spicelib::WNINSD(save.T1, save.T2, save.CNFINE.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Perform the search.
            //
            spicelib::ZZGFLONG(
                &save.VECDEF,
                &save.METHOD,
                &save.TARGET,
                &save.REF,
                &save.ABCORR,
                &save.OBSRVR,
                &save.DREF,
                save.DVEC.as_slice(),
                &save.CRDSYS,
                &save.CRDNAM,
                &save.RELATE,
                save.REFVAL,
                save.TOL,
                save.ADJUST,
                spicelib::GFSTEP,
                spicelib::GFREFN,
                save.RPT,
                spicelib::GFREPI,
                spicelib::GFREPU,
                spicelib::GFREPF,
                save.BAIL,
                spicelib::GFBAIL,
                MW,
                NW,
                save.WORK.as_slice_mut(),
                save.CNFINE.as_slice(),
                save.RESULT.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // We expect to cross the VIth reference value at
            //
            //    24*(I-1) + 4.5 + 1.5D0*VI hours
            //
            //    past noon TDB.
            //
            //
            save.XTIME = (((1.5 * VI as f64) + 4.5) * 3600.0);

            //
            // Filter the result window to remove spurious
            // intervals that may arise due to convergence
            // error.
            //
            spicelib::WNFLTD(save.TOL, save.RESULT.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Normally we expect to find one solution interval, but
            // the result window should be empty if the constraint
            // can't be satisfied.
            //
            save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

            // WRITE (*,'(A,I15,E25.16)') 'N, REFVAL = ', N, REFVAL

            if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                //
                // Make sure we found 1 interval.
                //
                testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
            } else if (spicelib::EQSTR(&save.CRDSYS, CYLSYS)
                || spicelib::EQSTR(&save.CRDSYS, RADSYS))
            {
                //
                // Make sure we found 1 interval.
                //
                testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
            } else if ((spicelib::EQSTR(&save.CRDSYS, LATSYS)
                || spicelib::EQSTR(&save.CRDSYS, GEOSYS))
                || spicelib::EQSTR(&save.CRDSYS, SPHSYS))
            {
                //
                // The number of expected intervals depends on the
                // case index.
                //
                if (f64::cos(save.REFVAL) == -1.0) {
                    //
                    // Make sure we found no intervals.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;
                } else if (VI == 17) {
                    //
                    // Make sure we found no intervals.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;
                } else {
                    //
                    // We should have 1 interval.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
                }
            }

            //
            // Now check the solution intervals.
            //
            for I in 1..=save.N {
                //
                // Look up the Ith solution interval.
                //
                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    I,
                    &mut save.START,
                    &mut save.FINISH,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                // WRITE (*,*) START, FINISH

                if spicelib::EQSTR(&save.CRDSYS, b"PLANETOGRAPHIC") {
                    if (I == 1) {
                        //
                        // This is the normal case.
                        //
                        // WRITE (*,*) 1, START, FINISH
                        //
                        if (f64::cos(save.REFVAL) == 1.0) {
                            //
                            // This is a special case: the < condition
                            // is considered to be met on the entire
                            // confinement window.
                            //
                            fstr::assign(&mut save.QNAME, b"Interval start  # (0)");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                            testutil::CHCKSD(
                                &save.QNAME,
                                save.START,
                                b"~",
                                save.T1,
                                TIMTOL,
                                OK,
                                ctx,
                            )?;

                            save.XET = save.T2;
                            save.ET = save.FINISH;

                            fstr::assign(&mut save.QNAME, b"Interval stop # (0)");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                            testutil::CHCKSD(
                                &save.QNAME,
                                save.ET,
                                b"~",
                                save.XET,
                                TIMTOL,
                                OK,
                                ctx,
                            )?;
                        } else {
                            //
                            // The start time should be T1. The stop time
                            // should be the time of the reference meridian
                            // crossing.

                            fstr::assign(&mut save.QNAME, b"Interval start  # (0)");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                            testutil::CHCKSD(
                                &save.QNAME,
                                save.START,
                                b"~",
                                save.T1,
                                TIMTOL,
                                OK,
                                ctx,
                            )?;

                            save.XET = save.XTIME;
                            save.ET = save.FINISH;

                            fstr::assign(&mut save.QNAME, b"Interval stop # (0)");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                            testutil::CHCKSD(
                                &save.QNAME,
                                save.ET,
                                b"~",
                                save.XET,
                                TIMTOL,
                                OK,
                                ctx,
                            )?;
                        }
                    } else {
                        //
                        // We don't expect to have more than one
                        // interval for this case.
                        //
                        spicelib::SETMSG(b"Unexpected interval count: #; VI = #", ctx);
                        spicelib::ERRINT(b"#", save.N, ctx);
                        spicelib::ERRINT(b"#", VI, ctx);
                        spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }
                } else if (spicelib::EQSTR(&save.CRDSYS, b"CYLINDRICAL")
                    || spicelib::EQSTR(&save.CRDSYS, b"RA/DEC"))
                {
                    //
                    // Longitude is positive East. The branch cut is at 0.
                    //
                    if (I == 1) {
                        //
                        // This is the normal case.
                        //
                        if (f64::cos(save.REFVAL) == 1.0) {
                            //
                            // This is a special case: the > condition
                            // is considered to be met on the whole
                            // confinement window.
                            //
                            save.XET = save.T1;
                            save.ET = save.START;

                            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                            testutil::CHCKSD(
                                &save.QNAME,
                                save.ET,
                                b"~",
                                save.XET,
                                TIMTOL,
                                OK,
                                ctx,
                            )?;

                            fstr::assign(&mut save.QNAME, b"Interval stop  # (0)");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                            testutil::CHCKSD(
                                &save.QNAME,
                                save.FINISH,
                                b"~",
                                save.T2,
                                TIMTOL,
                                OK,
                                ctx,
                            )?;
                        } else {
                            //
                            // The start time should be the REFVAL meridian
                            // crossing time; the end time should be T2.
                            //
                            save.XET = save.XTIME;
                            save.ET = save.START;

                            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                            testutil::CHCKSD(
                                &save.QNAME,
                                save.ET,
                                b"~",
                                save.XET,
                                TIMTOL,
                                OK,
                                ctx,
                            )?;

                            fstr::assign(&mut save.QNAME, b"Interval stop  # (0)");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                            testutil::CHCKSD(
                                &save.QNAME,
                                save.FINISH,
                                b"~",
                                save.T2,
                                TIMTOL,
                                OK,
                                ctx,
                            )?;
                        }
                    }
                } else if ((spicelib::EQSTR(&save.CRDSYS, b"LATITUDINAL")
                    || spicelib::EQSTR(&save.CRDSYS, b"GEODETIC"))
                    || spicelib::EQSTR(&save.CRDSYS, b"SPHERICAL"))
                {
                    //
                    // Longitude is positive East. The branch cut is at Pi.
                    //
                    if (VI < 17) {
                        if (I == 1) {
                            //
                            // This is the normal case.
                            //
                            // If REFVAL is in the upper half of the unit
                            // circle, including 0 and pi, the start time
                            // should be the time of the reference meridian
                            // crossing and the stop time should be ET0 +
                            // 12 h.
                            //
                            // If REFVAL is in the lower half of the unit
                            // circle, the start time should be the time of
                            // the reference meridian crossing and the stop
                            // time should be T2 (the end of the confinement
                            // window).
                            //
                            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                            save.XET = save.XTIME;
                            save.ET = save.START;

                            testutil::CHCKSD(
                                &save.QNAME,
                                save.START,
                                b"~",
                                save.XET,
                                TIMTOL,
                                OK,
                                ctx,
                            )?;

                            fstr::assign(&mut save.QNAME, b"Interval stop  # (0)");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                            if (f64::sin(save.REFVAL) >= 0.0) {
                                testutil::CHCKSD(
                                    &save.QNAME,
                                    save.FINISH,
                                    b"~",
                                    (save.ET0 + (spicelib::SPD() / 2 as f64)),
                                    TIMTOL,
                                    OK,
                                    ctx,
                                )?;
                            } else {
                                testutil::CHCKSD(
                                    &save.QNAME,
                                    save.FINISH,
                                    b"~",
                                    save.T2,
                                    TIMTOL,
                                    OK,
                                    ctx,
                                )?;
                            }
                        } else {
                            //
                            // We don't expect to have more than one
                            // interval for this case.
                            //
                            spicelib::SETMSG(b"Unexpected interval count: #; VI = #", ctx);
                            spicelib::ERRINT(b"#", save.N, ctx);
                            spicelib::ERRINT(b"#", VI, ctx);
                            spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        }
                    } else {
                        //
                        // This case corresponds to the first interval
                        // when REFVAL is 0 and VI is 17. The first interval
                        // should be the singleton [ET0, ET0].
                        //
                        if (I == 1) {
                            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                            testutil::CHCKSD(
                                &save.QNAME,
                                save.START,
                                b"~",
                                save.ET0,
                                TIMTOL,
                                OK,
                                ctx,
                            )?;

                            fstr::assign(&mut save.QNAME, b"Interval stop  # (0)");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                            save.XET = save.ET0;

                            save.ET = save.FINISH;

                            testutil::CHCKSD(
                                &save.QNAME,
                                save.ET,
                                b"~",
                                save.XET,
                                TIMTOL,
                                OK,
                                ctx,
                            )?;
                        } else if (I == 2) {
                            //
                            // This case corresponds to the second interval
                            // when REFVAL is 0 and VI is 17. The second
                            // interval should cover from ET0+12h to ET0+24h.
                            //
                            fstr::assign(&mut save.QNAME, b"Interval start # (0)");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                            testutil::CHCKSD(
                                &save.QNAME,
                                save.START,
                                b"~",
                                (save.ET0 + (spicelib::SPD() / 2 as f64)),
                                TIMTOL,
                                OK,
                                ctx,
                            )?;

                            fstr::assign(&mut save.QNAME, b"Interval stop  # (0)");
                            spicelib::REPMI(&save.QNAME.to_vec(), b"#", 1, &mut save.QNAME, ctx);

                            save.XET = (save.ET0 + spicelib::SPD());

                            save.ET = save.FINISH;

                            testutil::CHCKSD(
                                &save.QNAME,
                                save.ET,
                                b"~",
                                save.XET,
                                TIMTOL,
                                OK,
                                ctx,
                            )?;
                        } else {
                            //
                            // We don't expect to have more than two
                            // intervals for this case.
                            //
                            spicelib::SETMSG(b"Unexpected interval count: #; VI = #", ctx);
                            spicelib::ERRINT(b"#", save.N, ctx);
                            spicelib::ERRINT(b"#", VI, ctx);
                            spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        }
                    }
                } else {
                    spicelib::SETMSG(b"Unexpected system: #", ctx);
                    spicelib::ERRCH(b"#", &save.CRDSYS, ctx);
                    spicelib::SIGERR(b"SPICE(TSPICEBUG)", ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                }

                //
                // Check the longitude at the crossing time ET.
                //
                // In order to decide how accurate the coordinate should
                // be, we'll use the angular rate of Gamma together with
                // the time tolerance. Adjust the tolerance for the units
                // we're using in the CHCKSD call.
                //
                save.QTOL =
                    ((spicelib::DPR(ctx) * (spicelib::TWOPI(ctx) / spicelib::SPD())) * CNVTOL);

                //
                // Compute the longitude of the vector we expect and
                // the longitude at FINISH.
                //
                spicelib::SPKEZR(
                    &save.TARGET,
                    save.XET,
                    &save.REF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::VEQU(save.STATE.as_slice(), save.POS.as_slice_mut());

                if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                    spicelib::RECPGR(
                        &save.OBSRVR,
                        save.POS.as_slice(),
                        1.0,
                        0.0,
                        &mut save.XLON,
                        &mut save.LAT,
                        &mut save.ALT,
                        ctx,
                    )?;
                } else {
                    spicelib::RECLAT(
                        save.POS.as_slice(),
                        &mut save.R,
                        &mut save.XLON,
                        &mut save.LAT,
                    );
                }

                spicelib::SPKEZR(
                    &save.TARGET,
                    save.ET,
                    &save.REF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.STATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::VEQU(save.STATE.as_slice(), save.POS.as_slice_mut());

                if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
                    spicelib::RECPGR(
                        &save.OBSRVR,
                        save.POS.as_slice(),
                        1.0,
                        0.0,
                        &mut save.LON,
                        &mut save.LAT,
                        &mut save.ALT,
                        ctx,
                    )?;
                } else {
                    spicelib::RECLAT(
                        save.POS.as_slice(),
                        &mut save.R,
                        &mut save.LON,
                        &mut save.LAT,
                    );
                }

                fstr::assign(&mut save.QNAME, b"Longitude # (deg)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

                //
                // Adjust the longitude by 2*pi if it's too far
                // from the expected value.
                //
                if (save.LON > (save.XLON + 1.0)) {
                    save.LON = (save.LON - spicelib::TWOPI(ctx));
                } else if (save.LON < (save.XLON - 1.0)) {
                    save.LON = (save.LON + spicelib::TWOPI(ctx));
                }

                testutil::CHCKSD(
                    &save.QNAME,
                    (save.LON * spicelib::DPR(ctx)),
                    b"~",
                    (save.XLON * spicelib::DPR(ctx)),
                    save.QTOL,
                    OK,
                    ctx,
                )?;
            }
            //
            // We're done with the Ith interval.
            //
        }
        //
        // End of vector definition loop.
        //

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Restore Alpha's radii.
            //
            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.ALPHRD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }
    //
    // End of coordinate system loop.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(NSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
