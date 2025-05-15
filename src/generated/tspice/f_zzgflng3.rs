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
const TIMTOL: f64 = ((1 as f64) * CNVTOL);
const PCK: &[u8] = b"zzgflong.tpc";
const NPCK: &[u8] = b"nat.tpc";
const NSPK: &[u8] = b"nat.bsp";
const SPK: &[u8] = b"zzgflong.bsp";
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 200;
const FRNMLN: i32 = 32;
const TIMLEN: i32 = 35;
const SYSLEN: i32 = 32;
const CRDLEN: i32 = 32;
const VDFLEN: i32 = 32;
const MTHLEN: i32 = 80;
const MAXWIN: i32 = (2 * 20000);
const MW: i32 = MAXWIN;
const NW: i32 = NWMAX;
const LBCELL: i32 = -5;
const OPLEN: i32 = 40;
const NLNSYS: i32 = 7;
const FRBFSZ: i32 = 100;
const SNSLEN: i32 = 4;

struct SaveVars {
    ABCORR: Vec<u8>,
    CNAM: ActualCharArray,
    CRDNAM: Vec<u8>,
    CRDSYS: Vec<u8>,
    CSYS: ActualCharArray,
    DREF: Vec<u8>,
    FRMTXT: ActualCharArray,
    LONSNS: Vec<u8>,
    REF: Vec<u8>,
    RELATE: Vec<u8>,
    METHOD: Vec<u8>,
    OBSRVR: Vec<u8>,
    QNAME: Vec<u8>,
    TARGET: Vec<u8>,
    TIMSTR: Vec<u8>,
    TITLE: Vec<u8>,
    VECDEF: Vec<u8>,
    ADJUST: f64,
    ALPHRD: StackArray<f64, 3>,
    CNFINE: ActualArray<f64>,
    COORD: f64,
    COORD0: f64,
    COORD2: f64,
    DVEC: StackArray<f64, 3>,
    ET0: f64,
    FINISH: f64,
    REFVAL: f64,
    RESULT: ActualArray<f64>,
    START: f64,
    T1: f64,
    T2: f64,
    TMPRAD: StackArray<f64, 3>,
    TDELTA: f64,
    TOL: f64,
    WORK: ActualArray2D<f64>,
    XTIME: f64,
    XTIME2: f64,
    HAN2: i32,
    HANDLE: i32,
    N: i32,
    NLINES: i32,
    BAIL: bool,
    RPT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut CNAM = ActualCharArray::new(CRDLEN, 1..=NLNSYS);
        let mut CRDNAM = vec![b' '; CRDLEN as usize];
        let mut CRDSYS = vec![b' '; SYSLEN as usize];
        let mut CSYS = ActualCharArray::new(SYSLEN, 1..=NLNSYS);
        let mut DREF = vec![b' '; FRNMLN as usize];
        let mut FRMTXT = ActualCharArray::new(LNSIZE, 1..=FRBFSZ);
        let mut LONSNS = vec![b' '; SNSLEN as usize];
        let mut REF = vec![b' '; FRNMLN as usize];
        let mut RELATE = vec![b' '; OPLEN as usize];
        let mut METHOD = vec![b' '; MTHLEN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TIMSTR = vec![b' '; TIMLEN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut VECDEF = vec![b' '; VDFLEN as usize];
        let mut ADJUST: f64 = 0.0;
        let mut ALPHRD = StackArray::<f64, 3>::new(1..=3);
        let mut CNFINE = ActualArray::<f64>::new(LBCELL..=MW);
        let mut COORD: f64 = 0.0;
        let mut COORD0: f64 = 0.0;
        let mut COORD2: f64 = 0.0;
        let mut DVEC = StackArray::<f64, 3>::new(1..=3);
        let mut ET0: f64 = 0.0;
        let mut FINISH: f64 = 0.0;
        let mut REFVAL: f64 = 0.0;
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MW);
        let mut START: f64 = 0.0;
        let mut T1: f64 = 0.0;
        let mut T2: f64 = 0.0;
        let mut TMPRAD = StackArray::<f64, 3>::new(1..=3);
        let mut TDELTA: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MW, 1..=NW);
        let mut XTIME: f64 = 0.0;
        let mut XTIME2: f64 = 0.0;
        let mut HAN2: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut N: i32 = 0;
        let mut NLINES: i32 = 0;
        let mut BAIL: bool = false;
        let mut RPT: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"Longitude"),
                Val::C(b"Right  ascension"),
                Val::C(b"Longitude"),
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
                Val::C(b"Planetographic_2"),
                Val::C(b"Spherical"),
            ]
            .into_iter();
            CSYS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            CNAM,
            CRDNAM,
            CRDSYS,
            CSYS,
            DREF,
            FRMTXT,
            LONSNS,
            REF,
            RELATE,
            METHOD,
            OBSRVR,
            QNAME,
            TARGET,
            TIMSTR,
            TITLE,
            VECDEF,
            ADJUST,
            ALPHRD,
            CNFINE,
            COORD,
            COORD0,
            COORD2,
            DVEC,
            ET0,
            FINISH,
            REFVAL,
            RESULT,
            START,
            T1,
            T2,
            TMPRAD,
            TDELTA,
            TOL,
            WORK,
            XTIME,
            XTIME2,
            HAN2,
            HANDLE,
            N,
            NLINES,
            BAIL,
            RPT,
        }
    }
}

//$Procedure      F_ZZGFLNG3 ( Test GF longitude solver, part 3 )
pub fn F_ZZGFLNG3(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Number of systems that support longitude:
    //

    //
    // Frame definition buffer size:
    //

    //
    // Longitude sense string length:
    //

    //
    // Local variables
    //

    // INTEGER               J

    //
    // Save everything.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFLNG3", ctx)?;

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
    //*    Absolute maximum tests
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     We're going to find absolute maxima of the alphafixed longitude
    //     of the Alpha-gamma vector.
    //
    //
    spicelib::STR2ET(b"2000 JAN 1 18:00 TDB ", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MW, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MW, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We'll loop over the various coordinate systems.
    //
    for CC in 1..=NLNSYS {
        //
        // Set the coordinate system and coordinate.
        //
        fstr::assign(&mut save.CRDSYS, save.CSYS.get(CC));

        if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
            spicelib::PCPOOL(
                b"BODY1000_PGR_POSITIVE_LON",
                1,
                CharArray::from_ref(b"WEST"),
                ctx,
            )?;

            fstr::assign(&mut save.LONSNS, b"WEST");
        } else if spicelib::EQSTR(&save.CRDSYS, b"Planetographic_2") {
            spicelib::PCPOOL(
                b"BODY1000_PGR_POSITIVE_LON",
                1,
                CharArray::from_ref(b"EAST"),
                ctx,
            )?;

            fstr::assign(&mut save.LONSNS, b"EAST");
            fstr::assign(&mut save.CRDSYS, PGRSYS);
        } else {
            fstr::assign(&mut save.LONSNS, b"EAST");
        }

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

        fstr::assign(&mut save.CRDNAM, save.CNAM.get(CC));

        //
        // Loop over different confinement windows.
        // We pick the window end times so that the
        // maxima will be multiples of pi/8.
        //
        for SI in 1..=16 {
            if fstr::eq(&save.LONSNS, b"WEST") {
                save.T1 = (save.ET0 + ((((SI - 1) as f64) * 1.5) * 3600.0));
                save.T2 = (save.ET0 + spicelib::SPD());
                //
                // Set the expected time of the maximum.
                //
                save.XTIME = save.T1;
            } else {
                if (SI <= 9) {
                    save.T1 = save.ET0;
                } else {
                    save.T1 = (save.ET0 + (spicelib::SPD() / 2 as f64));
                }

                save.T2 = (save.ET0 + ((((SI - 1) as f64) * 1.5) * 3600.0));

                //
                // Set the expected time of the maximum.
                //
                save.XTIME = save.T2;
            }

            spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
            spicelib::WNINSD(save.T1, save.T2, save.CNFINE.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.REFVAL = ((((SI - 1) as f64) * spicelib::PI(ctx)) / 8.0);

            //
            // --- Case: ------------------------------------------------------
            //
            fstr::assign(&mut save.TITLE, b"Alpha-gamma maximum; #; #; ref val #");

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDNAM, &mut save.TITLE);
            spicelib::REPMF(
                &save.TITLE.to_vec(),
                b"#",
                (spicelib::DPR(ctx) * save.REFVAL),
                6,
                b"F",
                &mut save.TITLE,
                ctx,
            );
            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Set parameters required for the search.
            //
            fstr::assign(&mut save.VECDEF, POSDEF);
            fstr::assign(&mut save.METHOD, b" ");
            fstr::assign(&mut save.TARGET, b"GAMMA");
            fstr::assign(&mut save.OBSRVR, b"ALPHA");
            fstr::assign(&mut save.ABCORR, b"NONE");
            fstr::assign(&mut save.REF, b"ALPHAFIXED");

            fstr::assign(&mut save.DREF, b" ");
            spicelib::CLEARD(3, save.DVEC.as_slice_mut());

            fstr::assign(&mut save.RELATE, b"ABSMAX");
            save.REFVAL = 0.0;

            save.TOL = CNVTOL;
            save.ADJUST = 0.0;

            spicelib::GFSSTP(10000.0, ctx)?;

            save.RPT = false;
            save.BAIL = false;

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

            save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (SI == 1) {
                if fstr::eq(&save.LONSNS, b"WEST") {
                    //
                    // There's a branch cut at 0: however, we've
                    // made the confinement window the whole unit
                    // circle. So we expect a result.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
                } else if ((spicelib::EQSTR(&save.CRDSYS, RADSYS)
                    || spicelib::EQSTR(&save.CRDSYS, PGRSYS))
                    || spicelib::EQSTR(&save.CRDSYS, CYLSYS))
                {
                    //
                    // There's a branch cut at 0: the search window is
                    // bounded away from the branch cut, while the
                    // confinement window contains nothing but the branch
                    // cut. The result window should be empty.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;
                } else {
                    //
                    // The confinement window should be a singleton set
                    // at which the absolute maximum is found.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
                }
            } else {
                //
                // We expect the result window to contain one
                // solution interval.
                //
                testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
            }

            if (save.N == 1) {
                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    1,
                    &mut save.START,
                    &mut save.FINISH,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                // Check that START is equal to XTIME.
                //
                fstr::assign(&mut save.QNAME, b"START vs XTIME");
                testutil::CHCKSD(&save.QNAME, save.START, b"~", save.XTIME, TIMTOL, OK, ctx)?;

                //
                // Check that START and FINISH match.
                //
                fstr::assign(&mut save.QNAME, b"START vs FINISH");
                testutil::CHCKSD(&save.QNAME, save.START, b"~", save.FINISH, TIMTOL, OK, ctx)?;
            }
        }
        //
        // End of extreme value check loop.

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Restore Alpha's radii.
            //
            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.ALPHRD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //*********************************************************************
    //*
    //*    Adjusted absolute maximum tests
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     We're going to find adjusted absolute maxima of the alphafixed
    //     longitude of the Alpha-gamma vector.
    //
    //
    spicelib::STR2ET(b"2000 JAN 1 18:00 TDB ", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MW, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MW, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = ((15.0 * spicelib::RPD(ctx)) / 3600.0);

    //
    // We'll loop over the various coordinate systems.
    //
    for CC in 1..=NLNSYS {
        //
        // Set the coordinate system and coordinate.
        //
        fstr::assign(&mut save.CRDSYS, save.CSYS.get(CC));
        fstr::assign(&mut save.CRDNAM, save.CNAM.get(CC));

        if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
            spicelib::PCPOOL(
                b"BODY1000_PGR_POSITIVE_LON",
                1,
                CharArray::from_ref(b"WEST"),
                ctx,
            )?;

            fstr::assign(&mut save.LONSNS, b"WEST");
        } else if spicelib::EQSTR(&save.CRDSYS, b"Planetographic_2") {
            spicelib::PCPOOL(
                b"BODY1000_PGR_POSITIVE_LON",
                1,
                CharArray::from_ref(b"EAST"),
                ctx,
            )?;

            fstr::assign(&mut save.LONSNS, b"EAST");
            fstr::assign(&mut save.CRDSYS, PGRSYS);
        } else {
            fstr::assign(&mut save.LONSNS, b"EAST");
        }

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

        //
        // Loop over different confinement windows.
        // We pick the window end times so that the
        // maxima will be multiples of pi/8.
        //
        for SI in 1..=16 {
            if fstr::eq(&save.LONSNS, b"WEST") {
                save.T1 = (save.ET0 + ((((SI - 1) as f64) * 1.5) * 3600.0));

                save.T2 = (save.ET0 + spicelib::SPD());
                //
                // Set the expected time of the maximum and the
                // adjusted maximum. XTIME is the expected
                // start time; XTIME2 is the expected stop time.
                //
                save.XTIME = save.T1;
                save.XTIME2 = intrinsics::DMIN1(&[(save.T1 + 1.0), save.T2]);
            } else {
                if (SI <= 9) {
                    save.T1 = save.ET0;
                } else {
                    save.T1 = (save.ET0 + (spicelib::SPD() / 2 as f64));
                }

                save.T2 = (save.ET0 + ((((SI - 1) as f64) * 1.5) * 3600.0));
                //
                // Set the expected time of the maximum and the
                // adjusted maximum. XTIME is the expected

                // start time; XTIME2 is the expected stop time.

                save.XTIME = intrinsics::DMAX1(&[(save.T2 - 1.0), save.ET0]);
                save.XTIME2 = save.T2;
            }

            spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
            spicelib::WNINSD(save.T1, save.T2, save.CNFINE.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Contract the confinement window a bit to avoid
            // spurious maxima. However, don't wipe out the
            // confinement interval altogether.
            //
            save.REFVAL = ((((SI - 1) as f64) * spicelib::PI(ctx)) / 8.0);

            spicelib::WNFETD(
                save.CNFINE.as_slice(),
                1,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // --- Case: ------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Alpha-gamma adjusted maximum; #; #; max lon #; ADJUST = #",
            );

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDNAM, &mut save.TITLE);
            spicelib::REPMF(
                &save.TITLE.to_vec(),
                b"#",
                (spicelib::DPR(ctx) * save.REFVAL),
                6,
                b"F",
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMF(
                &save.TITLE.to_vec(),
                b"#",
                (spicelib::DPR(ctx) * save.ADJUST),
                6,
                b"F",
                &mut save.TITLE,
                ctx,
            );
            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Set parameters required for the search.
            //
            fstr::assign(&mut save.VECDEF, POSDEF);
            fstr::assign(&mut save.METHOD, b" ");
            fstr::assign(&mut save.TARGET, b"GAMMA");
            fstr::assign(&mut save.OBSRVR, b"ALPHA");
            fstr::assign(&mut save.ABCORR, b"NONE");
            fstr::assign(&mut save.REF, b"ALPHAFIXED");

            fstr::assign(&mut save.DREF, b" ");
            spicelib::CLEARD(3, save.DVEC.as_slice_mut());

            fstr::assign(&mut save.RELATE, b"ABSMAX");
            save.REFVAL = 0.0;

            save.TOL = CNVTOL;

            spicelib::GFSSTP(10000.0, ctx)?;

            save.RPT = false;
            save.BAIL = false;

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

            save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (SI == 1) {
                if fstr::eq(&save.LONSNS, b"WEST") {
                    //
                    // We expect the result window to contain one
                    // solution interval.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
                } else {
                    if ((spicelib::EQSTR(&save.CRDSYS, PGRSYS)
                        || spicelib::EQSTR(&save.CRDSYS, CYLSYS))
                        || spicelib::EQSTR(&save.CRDSYS, RADSYS))
                    {
                        //
                        // These systems have a singularity at
                        // 0 longitude/RA, so the result window
                        // should be empty.
                        //
                        testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;
                    } else {
                        //
                        // The result should be a singleton set.
                        //
                        testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
                    }
                }
            } else {
                //
                // We expect the result window to contain one
                // solution interval.
                //
                testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;

                //
                // IF ( .NOT. OK ) THEN
                //
                //    DO J = 1, WNCARD(CNFINE)
                //       CALL WNFETD ( CNFINE, J, START, FINISH )
                //       WRITE (*,'(2(5PE24.12))') START, FINISH
                //    END DO
                //
                //    DO J = 1, WNCARD(RESULT)
                //       CALL WNFETD ( RESULT, J, START, FINISH )
                //       WRITE (*,'(2(5PE24.12))') START, FINISH
                //    END DO
                //
                // END IF
                //
            }

            if (save.N == 1) {
                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    1,
                    &mut save.START,
                    &mut save.FINISH,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                // Check that START is equal to XTIME.
                //
                fstr::assign(&mut save.QNAME, b"START vs XTIME");
                testutil::CHCKSD(&save.QNAME, save.START, b"~", save.XTIME, TIMTOL, OK, ctx)?;

                //
                // Check that FINISH is equal to XTIME2.
                //
                fstr::assign(&mut save.QNAME, b"FINISH vs XTIME2");
                testutil::CHCKSD(&save.QNAME, save.FINISH, b"~", save.XTIME2, TIMTOL, OK, ctx)?;
            }
        }
        //
        // End of extreme value check loop.

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Restore Alpha's radii.
            //
            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.ALPHRD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //*********************************************************************
    //*
    //*    Absolute minimum tests
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     We're going to find absolute minima of the alphafixed longitude
    //     of the Alpha-gamma vector.
    //
    //
    spicelib::STR2ET(b"2000 JAN 1 18:00 TDB ", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MW, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MW, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We'll loop over the various coordinate systems.
    //
    for CC in 1..=NLNSYS {
        //
        // Set the coordinate system and coordinate.
        //
        fstr::assign(&mut save.CRDSYS, save.CSYS.get(CC));
        fstr::assign(&mut save.CRDNAM, save.CNAM.get(CC));

        if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
            spicelib::PCPOOL(
                b"BODY1000_PGR_POSITIVE_LON",
                1,
                CharArray::from_ref(b"WEST"),
                ctx,
            )?;

            fstr::assign(&mut save.LONSNS, b"WEST");
        } else if spicelib::EQSTR(&save.CRDSYS, b"Planetographic_2") {
            spicelib::PCPOOL(
                b"BODY1000_PGR_POSITIVE_LON",
                1,
                CharArray::from_ref(b"EAST"),
                ctx,
            )?;

            fstr::assign(&mut save.LONSNS, b"EAST");
            fstr::assign(&mut save.CRDSYS, PGRSYS);
        } else {
            fstr::assign(&mut save.LONSNS, b"EAST");
        }

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

        //
        // Loop over different confinement windows.
        // We pick the window end times so that the
        // minima will be multiples of pi/8.
        //
        for SI in 1..=16 {
            if fstr::eq(&save.LONSNS, b"WEST") {
                save.T1 = save.ET0;

                save.T2 = (save.ET0 + ((((SI - 1) as f64) * 1.5) * 3600.0));
                //
                // Set the expected time of the minimum.
                //
                save.XTIME = save.T2;
            } else {
                save.T1 = (save.ET0 + ((((SI - 1) as f64) * 1.5) * 3600.0));

                if (SI <= 8) {
                    save.T2 = (save.ET0 + (spicelib::SPD() / 2 as f64));
                } else {
                    save.T2 = (save.ET0 + spicelib::SPD());
                }

                //
                // Set the expected time of the minimum.
                //
                save.XTIME = save.T1;
            }

            spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
            spicelib::WNINSD(save.T1, save.T2, save.CNFINE.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.REFVAL = ((((SI - 1) as f64) * spicelib::PI(ctx)) / 8.0);

            //
            // --- Case: ------------------------------------------------------
            //
            fstr::assign(&mut save.TITLE, b"Alpha-gamma abs minimum; #; #; ref val #");

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDNAM, &mut save.TITLE);
            spicelib::REPMF(
                &save.TITLE.to_vec(),
                b"#",
                (spicelib::DPR(ctx) * save.REFVAL),
                6,
                b"F",
                &mut save.TITLE,
                ctx,
            );
            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Set parameters required for the search.
            //
            fstr::assign(&mut save.VECDEF, POSDEF);
            fstr::assign(&mut save.METHOD, b" ");
            fstr::assign(&mut save.TARGET, b"GAMMA");
            fstr::assign(&mut save.OBSRVR, b"ALPHA");
            fstr::assign(&mut save.ABCORR, b"NONE");
            fstr::assign(&mut save.REF, b"ALPHAFIXED");

            fstr::assign(&mut save.DREF, b" ");
            spicelib::CLEARD(3, save.DVEC.as_slice_mut());

            fstr::assign(&mut save.RELATE, b"ABSMIN");
            save.REFVAL = 0.0;

            save.TOL = CNVTOL;
            save.ADJUST = 0.0;

            spicelib::GFSSTP(10000.0, ctx)?;

            save.RPT = false;
            save.BAIL = false;

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

            save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (SI == 1) {
                if fstr::eq(&save.LONSNS, b"WEST") {
                    //
                    // There's a branch cut at 0: the confinement window
                    // lies in the deleted sector that encloses the branch
                    // cut. The result window should be empty.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;
                } else if ((spicelib::EQSTR(&save.CRDSYS, RADSYS)
                    || spicelib::EQSTR(&save.CRDSYS, PGRSYS))
                    || spicelib::EQSTR(&save.CRDSYS, CYLSYS))
                {
                    //
                    // There's a branch cut at 0: however we've made
                    // the confinement window cover the upper half
                    // circle. The result window should be non-empty.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
                } else {
                    //
                    // The confinement window should be a singleton set
                    // at which the absolute minimum is found.
                    //
                    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
                }
            } else {
                //
                // We expect the result window to contain one
                // solution interval.
                //
                testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
            }

            if (save.N == 1) {
                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    1,
                    &mut save.START,
                    &mut save.FINISH,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                // Check that START is equal to XTIME.
                //
                fstr::assign(&mut save.QNAME, b"START vs XTIME");
                testutil::CHCKSD(&save.QNAME, save.START, b"~", save.XTIME, TIMTOL, OK, ctx)?;

                //
                // Check that START and FINISH match.
                //
                fstr::assign(&mut save.QNAME, b"START vs FINISH");
                testutil::CHCKSD(&save.QNAME, save.START, b"~", save.FINISH, TIMTOL, OK, ctx)?;
            }
        }
        //
        // End of extreme value check loop.

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Restore Alpha's radii.
            //
            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.ALPHRD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //*********************************************************************
    //*
    //*    Adjusted absolute minimum tests
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     We're going to find adjusted absolute minima of the alphafixed
    //     longitude of the Alpha-gamma vector. In each case we'll use
    //     an adjustment value of 15 arcseconds, which should shift the
    //     minima by 1 second.
    //
    //
    spicelib::STR2ET(b"2000 JAN 1 18:00 TDB ", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MW, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MW, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ADJUST = ((15.0 * spicelib::RPD(ctx)) / 3600.0);

    //
    // We'll loop over the various coordinate systems.
    //

    for CC in 1..=NLNSYS {
        //
        // Set the coordinate system and coordinate.
        //
        fstr::assign(&mut save.CRDSYS, save.CSYS.get(CC));
        fstr::assign(&mut save.CRDNAM, save.CNAM.get(CC));

        if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
            spicelib::PCPOOL(
                b"BODY1000_PGR_POSITIVE_LON",
                1,
                CharArray::from_ref(b"WEST"),
                ctx,
            )?;

            fstr::assign(&mut save.LONSNS, b"WEST");
        } else if spicelib::EQSTR(&save.CRDSYS, b"Planetographic_2") {
            spicelib::PCPOOL(
                b"BODY1000_PGR_POSITIVE_LON",
                1,
                CharArray::from_ref(b"EAST"),
                ctx,
            )?;

            fstr::assign(&mut save.LONSNS, b"EAST");
            fstr::assign(&mut save.CRDSYS, PGRSYS);
        } else {
            fstr::assign(&mut save.LONSNS, b"EAST");
        }

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

        //
        // Loop over different confinement windows.
        // We pick the window end times so that the
        // minima will be multiples of pi/4.
        //
        for SI in 1..=16 {
            if fstr::eq(&save.LONSNS, b"WEST") {
                save.T1 = save.ET0;
                save.T2 = (save.ET0 + ((((SI - 1) as f64) * 1.5) * 3600.0));
                //
                // Set the expected time of the minimum
                // and the adjusted minimum.
                //
                save.XTIME = intrinsics::DMAX1(&[(save.T2 - 1.0), save.T1]);
                save.XTIME2 = save.T2;
            } else {
                save.T1 = (save.ET0 + ((((SI - 1) as f64) * 1.5) * 3600.0));

                if (SI <= 8) {
                    save.T2 = (save.ET0 + (spicelib::SPD() / 2 as f64));
                } else {
                    save.T2 = (save.ET0 + spicelib::SPD());
                }
                //
                // Set the expected time of the minimum
                // and the adjusted minimum.
                //
                save.XTIME = save.T1;
                save.XTIME2 = intrinsics::DMIN1(&[(save.T1 + 1.0), save.T2]);
            }

            spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

            spicelib::WNINSD(save.T1, save.T2, save.CNFINE.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.REFVAL = ((((SI - 1) as f64) * spicelib::PI(ctx)) / 8.0);

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
                b"Adjusted alpha-gamma minimum; #; #; #; min lon #",
            );

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDNAM, &mut save.TITLE);
            spicelib::REPMF(
                &save.TITLE.to_vec(),
                b"#",
                (spicelib::DPR(ctx) * save.ADJUST),
                6,
                b"F",
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMF(
                &save.TITLE.to_vec(),
                b"#",
                (spicelib::DPR(ctx) * save.REFVAL),
                6,
                b"F",
                &mut save.TITLE,
                ctx,
            );
            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Set parameters required for the search.
            //
            fstr::assign(&mut save.VECDEF, POSDEF);
            fstr::assign(&mut save.METHOD, b" ");
            fstr::assign(&mut save.TARGET, b"GAMMA");
            fstr::assign(&mut save.OBSRVR, b"ALPHA");
            fstr::assign(&mut save.ABCORR, b"NONE");
            fstr::assign(&mut save.REF, b"ALPHAFIXED");

            fstr::assign(&mut save.DREF, b" ");
            spicelib::CLEARD(3, save.DVEC.as_slice_mut());

            fstr::assign(&mut save.RELATE, b"ABSMIN");
            save.REFVAL = 0.0;

            save.TOL = CNVTOL;

            spicelib::GFSSTP(10000.0, ctx)?;

            save.RPT = false;
            save.BAIL = false;

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

            save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // We expect the result window to contain one
            // solution interval.
            //
            if fstr::eq(&save.LONSNS, b"WEST") {
                if (SI > 1) {
                    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
                } else {
                    testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;
                }
            } else {
                testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
            }

            if (save.N == 1) {
                spicelib::WNFETD(
                    save.RESULT.as_slice(),
                    1,
                    &mut save.START,
                    &mut save.FINISH,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Check that START is equal to XTIME.
                //
                fstr::assign(&mut save.QNAME, b"START vs XTIME");
                testutil::CHCKSD(&save.QNAME, save.START, b"~", save.XTIME, TIMTOL, OK, ctx)?;

                //
                // Check that FINISH is equal to XTIME2.
                //
                fstr::assign(&mut save.QNAME, b"FINISH vs XTIME2");
                testutil::CHCKSD(&save.QNAME, save.FINISH, b"~", save.XTIME2, TIMTOL, OK, ctx)?;
            }
        }
        //
        // End of extreme value check loop.

        if (spicelib::EQSTR(&save.CRDSYS, GEOSYS) || spicelib::EQSTR(&save.CRDSYS, PGRSYS)) {
            //
            // Restore Alpha's radii.
            //
            spicelib::PDPOOL(b"BODY1000_RADII", 3, save.ALPHRD.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    //*********************************************************************
    //*
    //*    Local maximum/minimum tests
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Local max/min tests:
    //
    //     We're going to find local extrema of the J2000 longitude
    //     of the Sun-IO vector. Jupiter's orbital angular rate is
    //     slow enough so that Io exhibits periods of retrograde
    //     motion.
    //
    //     In order to be able to use geodetic and planetographic
    //     coordinate systems, we need a reference frame aligned with
    //     the J2000 frame, but whose center is the Sun. We create
    //     such a frame specification below and insert it into the
    //     kernel pool via LMPOOL.
    //
    fstr::assign(
        save.FRMTXT.get_mut(1),
        b"FRAME_SUNJ2000             = 1400000",
    );
    fstr::assign(
        save.FRMTXT.get_mut(2),
        b"FRAME_1400000_NAME         = \'SUNJ2000\' ",
    );
    fstr::assign(save.FRMTXT.get_mut(3), b"FRAME_1400000_CLASS        = 4");
    fstr::assign(
        save.FRMTXT.get_mut(4),
        b"FRAME_1400000_CLASS_ID     = 1400000",
    );
    fstr::assign(save.FRMTXT.get_mut(5), b"FRAME_1400000_CENTER       = 10");
    fstr::assign(
        save.FRMTXT.get_mut(6),
        b"TKFRAME_1400000_RELATIVE   = \'J2000\' ",
    );
    fstr::assign(
        save.FRMTXT.get_mut(7),
        b"TKFRAME_1400000_SPEC       = \'MATRIX\' ",
    );
    fstr::assign(
        save.FRMTXT.get_mut(8),
        b"TKFRAME_1400000_MATRIX     = ( 1,0,0,0,1,0,0,0,1 )",
    );
    fstr::assign(
        save.FRMTXT.get_mut(9),
        b"OBJECT_SUN_FRAME           = \'SUNJ2000\' ",
    );

    save.NLINES = 9;

    spicelib::LMPOOL(save.FRMTXT.as_arg(), save.NLINES, ctx)?;

    //
    // Our confinement window will sample from Jupiter's orbit
    // in such a way that all branches of ZZGFLONG's local
    // extremum algorithm will be exercised. We'll use the
    // first few days of each year out of a 12 year period
    // as our confinement window.
    //
    spicelib::STR2ET(b"2000 JAN 1 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MW, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MW, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=12 {
        save.T1 = (save.ET0 + (((I - 1) as f64) * spicelib::JYEAR()));
        save.T2 = (save.T1 + (3.5 * spicelib::SPD()));

        spicelib::WNINSD(save.T1, save.T2, save.CNFINE.as_slice_mut(), ctx)?;
    }

    //
    // We'll loop over the various coordinate systems.
    //
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

        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut save.TITLE, b"Sun-Io local maximum #; #");

        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDNAM, &mut save.TITLE);

        testutil::TCASE(&save.TITLE, ctx)?;

        if spicelib::EQSTR(&save.CRDSYS, PGRSYS) {
            spicelib::PCPOOL(
                b"BODY10_PGR_POSITIVE_LON",
                1,
                CharArray::from_ref(b"WEST"),
                ctx,
            )?;

            fstr::assign(&mut save.LONSNS, b"WEST");
        } else if spicelib::EQSTR(&save.CRDSYS, b"Planetographic_2") {
            spicelib::PCPOOL(
                b"BODY10_PGR_POSITIVE_LON",
                1,
                CharArray::from_ref(b"EAST"),
                ctx,
            )?;

            fstr::assign(&mut save.LONSNS, b"EAST");
            fstr::assign(&mut save.CRDSYS, PGRSYS);
        } else {
            fstr::assign(&mut save.LONSNS, b"EAST");
        }

        //
        // Set parameters required for the search.
        //
        fstr::assign(&mut save.VECDEF, POSDEF);
        fstr::assign(&mut save.METHOD, b" ");
        fstr::assign(&mut save.TARGET, b"IO");
        fstr::assign(&mut save.OBSRVR, b"SUN");
        fstr::assign(&mut save.ABCORR, b"NONE");
        fstr::assign(&mut save.REF, b"SUNJ2000");

        fstr::assign(&mut save.DREF, b" ");
        spicelib::CLEARD(3, save.DVEC.as_slice_mut());

        fstr::assign(&mut save.RELATE, b"LOCMAX");
        save.REFVAL = 0.0;

        //
        // Use a loose tolerance to speed things up a bit.
        //
        save.TOL = 0.01;
        save.ADJUST = 0.0;

        spicelib::GFSSTP(((5 as f64) * 3600.0), ctx)?;

        save.RPT = false;
        save.BAIL = false;

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

        save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

        //
        // We expect each confinement interval to contain two
        // solutions, so we expect 24 solutions in all.
        //
        testutil::CHCKSI(b"N (0)", save.N, b"=", 24, 0, OK, ctx)?;

        for I in 1..=save.N {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;

            spicelib::ETCAL(save.FINISH, &mut save.TIMSTR, ctx);

            //
            // Check the coordinate at the event time +/- TDELTA
            // seconds.
            //
            save.TDELTA = 3.0;

            spicelib::ZZGFCOG(
                &mut (save.FINISH - save.TDELTA).clone(),
                &mut save.COORD0,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::ZZGFCOG(&mut save.FINISH, &mut save.COORD, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::ZZGFCOG(
                &mut (save.FINISH + save.TDELTA).clone(),
                &mut save.COORD2,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"#: COORD vs COORD0");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

            if fstr::eq(&save.LONSNS, b"WEST") {
                testutil::CHCKSD(&save.QNAME, save.COORD, b"<", save.COORD0, 0.0, OK, ctx)?;
            } else {
                testutil::CHCKSD(&save.QNAME, save.COORD, b">", save.COORD0, 0.0, OK, ctx)?;
            }

            fstr::assign(&mut save.QNAME, b"#: COORD2 vs COORD");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

            if fstr::eq(&save.LONSNS, b"WEST") {
                testutil::CHCKSD(&save.QNAME, save.COORD, b"<", save.COORD2, 0.0, OK, ctx)?;
            } else {
                testutil::CHCKSD(&save.QNAME, save.COORD, b">", save.COORD2, 0.0, OK, ctx)?;
            }
        }
        //
        // End of extreme value check loop.
        //

        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut save.TITLE, b"Sun-Io local minimum #; #");

        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDNAM, &mut save.TITLE);

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Set parameters required for the search.
        //
        fstr::assign(&mut save.VECDEF, POSDEF);
        fstr::assign(&mut save.METHOD, b" ");
        fstr::assign(&mut save.TARGET, b"IO");
        fstr::assign(&mut save.OBSRVR, b"SUN");
        fstr::assign(&mut save.ABCORR, b"NONE");
        fstr::assign(&mut save.REF, b"SUNJ2000");

        fstr::assign(&mut save.DREF, b" ");
        spicelib::CLEARD(3, save.DVEC.as_slice_mut());

        fstr::assign(&mut save.RELATE, b"LOCMIN");
        save.REFVAL = 0.0;

        //
        // Use a loose tolerance to speed things up a bit.
        //
        save.TOL = 0.01;
        save.ADJUST = 0.0;

        spicelib::GFSSTP(((5 as f64) * 3600.0), ctx)?;

        save.RPT = false;
        save.BAIL = false;

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

        save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

        //
        // We expect each confinement interval to contain two
        // solutions, so we expect 24 solutions in all.
        //
        testutil::CHCKSI(b"N (0)", save.N, b"=", 24, 0, OK, ctx)?;

        for I in 1..=save.N {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                I,
                &mut save.START,
                &mut save.FINISH,
                ctx,
            )?;

            spicelib::ETCAL(save.FINISH, &mut save.TIMSTR, ctx);

            //
            // Check the coordinate at the event time +/- TDELTA
            // seconds.
            //
            save.TDELTA = 3.0;

            spicelib::ZZGFCOG(
                &mut (save.FINISH - save.TDELTA).clone(),
                &mut save.COORD0,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::ZZGFCOG(&mut save.FINISH, &mut save.COORD, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::ZZGFCOG(
                &mut (save.FINISH + save.TDELTA).clone(),
                &mut save.COORD2,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.QNAME, b"#: COORD vs COORD0");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

            if fstr::eq(&save.LONSNS, b"WEST") {
                testutil::CHCKSD(&save.QNAME, save.COORD, b">", save.COORD0, 0.0, OK, ctx)?;
            } else {
                testutil::CHCKSD(&save.QNAME, save.COORD, b"<", save.COORD0, 0.0, OK, ctx)?;
            }

            fstr::assign(&mut save.QNAME, b"#: COORD2 vs COORD");
            spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

            if fstr::eq(&save.LONSNS, b"WEST") {
                testutil::CHCKSD(&save.QNAME, save.COORD, b">", save.COORD2, 0.0, OK, ctx)?;
            } else {
                testutil::CHCKSD(&save.QNAME, save.COORD, b"<", save.COORD2, 0.0, OK, ctx)?;
            }
        }
        //
        // End of extreme value check loop.
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
