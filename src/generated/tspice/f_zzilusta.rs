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
const PCK2: &[u8] = b"generic.tpc";
const SPK2: &[u8] = b"generic.bsp";
const OCTSPK: &[u8] = b"octl_test.bsp";
const OCTL: &[u8] = b"OCTL";
const IREF: &[u8] = b"J2000";
const ANGTOL: f64 = 0.00000000001;
const SINGLE: f64 = 0.000005;
const DPHTOL: f64 = 0.000001;
const DINTOL: f64 = 0.0000005;
const DEMTOL: f64 = 0.0000005;
const ALTTOL: f64 = 0.0000000001;
const MEDTOL: f64 = 0.0000001;
const LNSIZE: i32 = 80;
const NTIMES: i32 = 100;
const TIMLEN: i32 = 50;
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const NCORR: i32 = 5;
const TXTSIZ: i32 = 25;
const MSGLEN: i32 = 800;

struct SaveVars {
    ABCORR: Vec<u8>,
    CORRS: ActualCharArray,
    FIXREF: Vec<u8>,
    ILLUM: Vec<u8>,
    METHOD: Vec<u8>,
    OBSRVR: Vec<u8>,
    SRFREF: Vec<u8>,
    TARGET: Vec<u8>,
    TITLE: Vec<u8>,
    TIMSTR: Vec<u8>,
    TIME0: Vec<u8>,
    TXT: ActualCharArray,
    XMTCOR: Vec<u8>,
    DEMSSN: f64,
    DFDT: StackArray<f64, 3>,
    DINCDN: f64,
    DLT: f64,
    DPHASE: f64,
    EMISSN: f64,
    EMISTA: StackArray<f64, 2>,
    ET: f64,
    ET0: f64,
    ET1: f64,
    ETSURF: f64,
    FIRST: f64,
    LAT: f64,
    LAST: f64,
    LON: f64,
    LT: f64,
    LT2: f64,
    LTSRC: f64,
    NORMAL: StackArray<f64, 3>,
    NRMSTA: StackArray<f64, 6>,
    OBSST2: StackArray<f64, 6>,
    OBSSTA: StackArray<f64, 6>,
    PHASE: f64,
    PHSSTA: StackArray<f64, 2>,
    R: StackArray2D<f64, 9>,
    RADII: StackArray<f64, 3>,
    SOLAR: f64,
    SOLSTA: StackArray<f64, 2>,
    SPOINT: StackArray<f64, 3>,
    SRCST2: StackArray<f64, 6>,
    SRCSTA: StackArray<f64, 6>,
    SRFVEC: StackArray<f64, 3>,
    SSTATE: StackArray<f64, 6>,
    STATES: StackArray2D<f64, 12>,
    STEP: f64,
    TMPSTA: StackArray<f64, 6>,
    TOL: f64,
    TRGEPC: f64,
    XDEMIT: f64,
    XDSOLR: f64,
    XDPHAS: f64,
    HANDLE: i32,
    HAN2: i32,
    N: i32,
    OBSID: i32,
    OCTID: i32,
    ATTBLK: StackArray<bool, 6>,
    FOUND: bool,
    GEOM: bool,
    USECN: bool,
    USELT: bool,
    USESTL: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; CORLEN as usize];
        let mut CORRS = ActualCharArray::new(CORLEN, 1..=NCORR);
        let mut FIXREF = vec![b' '; FRNMLN as usize];
        let mut ILLUM = vec![b' '; BDNMLN as usize];
        let mut METHOD = vec![b' '; LNSIZE as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut SRFREF = vec![b' '; FRNMLN as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TITLE = vec![b' '; MSGLEN as usize];
        let mut TIMSTR = vec![b' '; TIMLEN as usize];
        let mut TIME0 = vec![b' '; TIMLEN as usize];
        let mut TXT = ActualCharArray::new(LNSIZE, 1..=TXTSIZ);
        let mut XMTCOR = vec![b' '; CORLEN as usize];
        let mut DEMSSN: f64 = 0.0;
        let mut DFDT = StackArray::<f64, 3>::new(1..=3);
        let mut DINCDN: f64 = 0.0;
        let mut DLT: f64 = 0.0;
        let mut DPHASE: f64 = 0.0;
        let mut EMISSN: f64 = 0.0;
        let mut EMISTA = StackArray::<f64, 2>::new(1..=2);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut ETSURF: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LAST: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut LT2: f64 = 0.0;
        let mut LTSRC: f64 = 0.0;
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut NRMSTA = StackArray::<f64, 6>::new(1..=6);
        let mut OBSST2 = StackArray::<f64, 6>::new(1..=6);
        let mut OBSSTA = StackArray::<f64, 6>::new(1..=6);
        let mut PHASE: f64 = 0.0;
        let mut PHSSTA = StackArray::<f64, 2>::new(1..=2);
        let mut R = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut SOLAR: f64 = 0.0;
        let mut SOLSTA = StackArray::<f64, 2>::new(1..=2);
        let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
        let mut SRCST2 = StackArray::<f64, 6>::new(1..=6);
        let mut SRCSTA = StackArray::<f64, 6>::new(1..=6);
        let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut SSTATE = StackArray::<f64, 6>::new(1..=6);
        let mut STATES = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
        let mut STEP: f64 = 0.0;
        let mut TMPSTA = StackArray::<f64, 6>::new(1..=6);
        let mut TOL: f64 = 0.0;
        let mut TRGEPC: f64 = 0.0;
        let mut XDEMIT: f64 = 0.0;
        let mut XDSOLR: f64 = 0.0;
        let mut XDPHAS: f64 = 0.0;
        let mut HANDLE: i32 = 0;
        let mut HAN2: i32 = 0;
        let mut N: i32 = 0;
        let mut OBSID: i32 = 0;
        let mut OCTID: i32 = 0;
        let mut ATTBLK = StackArray::<bool, 6>::new(1..=ABATSZ);
        let mut FOUND: bool = false;
        let mut GEOM: bool = false;
        let mut USECN: bool = false;
        let mut USELT: bool = false;
        let mut USESTL: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NONE"),
                Val::C(b"CN"),
                Val::C(b"CN+S"),
                Val::C(b"LT"),
                Val::C(b"LT+S"),
            ]
            .into_iter();
            CORRS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            CORRS,
            FIXREF,
            ILLUM,
            METHOD,
            OBSRVR,
            SRFREF,
            TARGET,
            TITLE,
            TIMSTR,
            TIME0,
            TXT,
            XMTCOR,
            DEMSSN,
            DFDT,
            DINCDN,
            DLT,
            DPHASE,
            EMISSN,
            EMISTA,
            ET,
            ET0,
            ET1,
            ETSURF,
            FIRST,
            LAT,
            LAST,
            LON,
            LT,
            LT2,
            LTSRC,
            NORMAL,
            NRMSTA,
            OBSST2,
            OBSSTA,
            PHASE,
            PHSSTA,
            R,
            RADII,
            SOLAR,
            SOLSTA,
            SPOINT,
            SRCST2,
            SRCSTA,
            SRFVEC,
            SSTATE,
            STATES,
            STEP,
            TMPSTA,
            TOL,
            TRGEPC,
            XDEMIT,
            XDSOLR,
            XDPHAS,
            HANDLE,
            HAN2,
            N,
            OBSID,
            OCTID,
            ATTBLK,
            FOUND,
            GEOM,
            USECN,
            USELT,
            USESTL,
        }
    }
}

//$Procedure      F_ZZILUSTA ( ZZILUSTA tests )
pub fn F_ZZILUSTA(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    //
    //

    //
    // Tolerances for tests involving discrete derivatives
    // of illumination angle rate. These are applicable
    // when aberration corrections are turned off, or when
    // a "smooth" planetary ephemeris such as DE421.bsp is
    // used.
    //

    //
    // Tolerance for illumination angle tests using
    // alternate computation methods:
    //

    //
    // Number of test epochs:
    //

    //
    // Local variables
    //

    //
    // The workspace array has to handle the largest workspace
    // we'll use, which is that required by GFPOSC.
    //

    //
    // Saved everything.
    //

    //
    // Initial values
    //
    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZILUSTA", ctx)?;

    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    testutil::TSTSPK(SPK2, true, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use a JPL DE for smoother planetary ephemerides. This
    // option is not available for a deliverable version of
    // this test family.
    //
    // CALL FURNSH ( 'de421.bsp' )
    // CALL CHCKXC ( .FALSE., ' ',   OK )

    testutil::TSTPCK(PCK2, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up a confinement window. Initialize this and
    // the result window.
    //
    fstr::assign(&mut save.TIME0, b"2000 JAN 1  00:00:00 TDB");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Invalid METHOD.", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"Elipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.ILLUM, b"SUN");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.OBSRVR, b"MOON");

    save.LON = (100.0 * spicelib::RPD(ctx));
    save.LAT = (30.0 * spicelib::RPD(ctx));

    spicelib::SRFREC(399, save.LON, save.LAT, save.SPOINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.NORMAL.as_slice_mut());

    spicelib::ZZILUSTA(
        &save.METHOD,
        &save.TARGET,
        &save.ILLUM,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice(),
        save.PHSSTA.as_slice_mut(),
        save.SOLSTA.as_slice_mut(),
        save.EMISTA.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDMETHOD)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Zero normal vector.", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.ILLUM, b"SUN");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.OBSRVR, b"MOON");

    save.LON = (100.0 * spicelib::RPD(ctx));
    save.LAT = (30.0 * spicelib::RPD(ctx));

    spicelib::SRFREC(399, save.LON, save.LAT, save.SPOINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(0.0, 0.0, 0.0, save.NORMAL.as_slice_mut());

    spicelib::ZZILUSTA(
        &save.METHOD,
        &save.TARGET,
        &save.ILLUM,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice(),
        save.PHSSTA.as_slice_mut(),
        save.SOLSTA.as_slice_mut(),
        save.EMISTA.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.NORMAL.as_slice_mut());

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Unrecognized target, observer, or illumination source.",
        ctx,
    )?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");

    spicelib::ZZILUSTA(
        &save.METHOD,
        b"X",
        &save.ILLUM,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice(),
        save.PHSSTA.as_slice_mut(),
        save.SOLSTA.as_slice_mut(),
        save.EMISTA.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    spicelib::ZZILUSTA(
        &save.METHOD,
        &save.TARGET,
        b"X",
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice(),
        save.PHSSTA.as_slice_mut(),
        save.SOLSTA.as_slice_mut(),
        save.EMISTA.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    spicelib::ZZILUSTA(
        &save.METHOD,
        &save.TARGET,
        &save.ILLUM,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        b"X",
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice(),
        save.PHSSTA.as_slice_mut(),
        save.SOLSTA.as_slice_mut(),
        save.EMISTA.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"No SPK data for observer", ctx)?;

    spicelib::ZZILUSTA(
        &save.METHOD,
        &save.TARGET,
        &save.ILLUM,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        b"GASPRA",
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice(),
        save.PHSSTA.as_slice_mut(),
        save.SOLSTA.as_slice_mut(),
        save.EMISTA.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"No SPK data for target", ctx)?;

    spicelib::ZZILUSTA(
        &save.METHOD,
        b"GASPRA",
        &save.ILLUM,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice(),
        save.PHSSTA.as_slice_mut(),
        save.SOLSTA.as_slice_mut(),
        save.EMISTA.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"No SPK data for illumination source", ctx)?;

    spicelib::ZZILUSTA(
        &save.METHOD,
        &save.TARGET,
        b"GASPRA",
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice(),
        save.PHSSTA.as_slice_mut(),
        save.SOLSTA.as_slice_mut(),
        save.EMISTA.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"No PCK orientation data for target.", ctx)?;

    fstr::assign(&mut save.FIXREF, b"ITRF93");

    spicelib::ZZILUSTA(
        &save.METHOD,
        &save.TARGET,
        &save.ILLUM,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice(),
        save.PHSSTA.as_slice_mut(),
        save.SOLSTA.as_slice_mut(),
        save.EMISTA.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Aberration correction is for transmission.", ctx)?;

    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");

    spicelib::ZZILUSTA(
        &save.METHOD,
        &save.TARGET,
        &save.ILLUM,
        save.ET,
        &save.FIXREF,
        b"XCN",
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice(),
        save.PHSSTA.as_slice_mut(),
        save.SOLSTA.as_slice_mut(),
        save.EMISTA.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //*********************************************************************
    //*
    //*    Comprehensive cases using comparisons against alternate
    //*    computations
    //*
    //*********************************************************************

    //*********************************************************************
    //*
    //*    PHASE angle tests
    //*
    //*********************************************************************

    //
    // We'll use the earth, moon, and sun as the three participating
    // bodies. The surface point will be the OCTL telescope; we'll
    // create an SPK file for this point. We also need an FK for a
    // topocentric frame centered at the point.
    //
    // Though it's not strictly necessary, we'll use real data for
    // these kernels, with one exception: we'll use the terrestrial
    // reference frame IAU_EARTH rather than ITRF93.
    //
    // The original reference frame specifications follow:
    //
    //
    //    Topocentric frame OCTL_TOPO
    //
    //       The Z axis of this frame points toward the zenith.
    //       The X axis of this frame points North.
    //
    //       Topocentric frame OCTL_TOPO is centered at the site OCTL
    //       which has Cartesian coordinates
    //
    //          X (km):                 -0.2448937761729E+04
    //          Y (km):                 -0.4667935793438E+04
    //          Z (km):                  0.3582748499430E+04
    //
    //       and planetodetic coordinates
    //
    //          Longitude (deg):      -117.6828380000000
    //          Latitude  (deg):        34.3817491000000
    //          Altitude   (km):         0.2259489999999E+01
    //
    //       These planetodetic coordinates are expressed relative to
    //       a reference spheroid having the dimensions
    //
    //          Equatorial radius (km):  6.3781400000000E+03
    //          Polar radius      (km):  6.3567523100000E+03
    //
    //       All of the above coordinates are relative to the frame
    //       ITRF93.
    //

    //
    //---- Case -------------------------------------------------------------
    //

    //
    // This isn't a test, but we'll call it that so we'll have
    // a meaningful label in any error messages that arise.
    //
    testutil::TCASE(b"Create OCTL kernels.", ctx)?;

    //
    // As mentioned, we go with a frame that's more convenient than
    // ITRF93:
    //
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");

    //
    // Prepare a frame kernel in a string array.
    //
    fstr::assign(
        save.TXT.get_mut(1),
        b"FRAME_OCTL_TOPO            =  1398962",
    );
    fstr::assign(
        save.TXT.get_mut(2),
        b"FRAME_1398962_NAME         =  \'OCTL_TOPO\' ",
    );
    fstr::assign(save.TXT.get_mut(3), b"FRAME_1398962_CLASS        =  4");
    fstr::assign(
        save.TXT.get_mut(4),
        b"FRAME_1398962_CLASS_ID     =  1398962",
    );
    fstr::assign(save.TXT.get_mut(5), b"FRAME_1398962_CENTER       =  398962");

    fstr::assign(
        save.TXT.get_mut(6),
        b"OBJECT_398962_FRAME        =  \'OCTL_TOPO\' ",
    );

    fstr::assign(
        save.TXT.get_mut(7),
        b"TKFRAME_1398962_RELATIVE   =  \'IAU_EARTH\' ",
    );
    fstr::assign(
        save.TXT.get_mut(8),
        b"TKFRAME_1398962_SPEC       =  \'ANGLES\' ",
    );
    fstr::assign(
        save.TXT.get_mut(9),
        b"TKFRAME_1398962_UNITS      =  \'DEGREES\' ",
    );
    fstr::assign(
        save.TXT.get_mut(10),
        b"TKFRAME_1398962_AXES       =  ( 3, 2, 3 )",
    );
    fstr::assign(
        save.TXT.get_mut(11),
        b"TKFRAME_1398962_ANGLES     =  ( -242.3171620000000,",
    );
    fstr::assign(
        save.TXT.get_mut(12),
        b"                                 -55.6182509000000,",
    );
    fstr::assign(
        save.TXT.get_mut(13),
        b"                                 180.0000000000000  )",
    );
    fstr::assign(
        save.TXT.get_mut(14),
        b"NAIF_BODY_NAME            +=  \'OCTL\' ",
    );
    fstr::assign(
        save.TXT.get_mut(15),
        b"NAIF_BODY_CODE            +=  398962",
    );

    //
    // It will be convenient to have a version of this frame that
    // has the +Z axis pointed down instead of up.
    //
    fstr::assign(
        save.TXT.get_mut(16),
        b"FRAME_OCTL_FLIP            =  2398962",
    );
    fstr::assign(
        save.TXT.get_mut(17),
        b"FRAME_2398962_NAME         =  \'OCTL_FLIP\' ",
    );
    fstr::assign(save.TXT.get_mut(18), b"FRAME_2398962_CLASS        =  4");
    fstr::assign(
        save.TXT.get_mut(19),
        b"FRAME_2398962_CLASS_ID     =  2398962",
    );
    fstr::assign(
        save.TXT.get_mut(20),
        b"FRAME_2398962_CENTER       =  398962",
    );

    fstr::assign(
        save.TXT.get_mut(21),
        b"TKFRAME_2398962_RELATIVE   =  \'OCTL_TOPO\' ",
    );
    fstr::assign(
        save.TXT.get_mut(22),
        b"TKFRAME_2398962_SPEC       =  \'ANGLES\' ",
    );
    fstr::assign(
        save.TXT.get_mut(23),
        b"TKFRAME_2398962_UNITS      =  \'DEGREES\' ",
    );
    fstr::assign(
        save.TXT.get_mut(24),
        b"TKFRAME_2398962_AXES       =  ( 3, 2, 3 )",
    );
    fstr::assign(
        save.TXT.get_mut(25),
        b"TKFRAME_2398962_ANGLES     =  ( 0, 180.0, 0 ) ",
    );

    spicelib::LMPOOL(save.TXT.as_arg(), TXTSIZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now create an SPK file containing a type 8 segment for OCTL.
    //
    spicelib::SPKOPN(OCTSPK, OCTSPK, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize both states to zero.
    //
    spicelib::CLEARD(12, save.STATES.as_slice_mut());

    //
    // The first position component:
    //
    save.SPOINT[1] = -2448.937761729;
    save.SPOINT[2] = -4667.935793438;
    save.SPOINT[3] = 3582.74849943;

    spicelib::VEQU(save.SPOINT.as_slice(), save.STATES.subarray_mut([1, 1]));

    //
    // The second position matches the first: we don't model
    // plate motion.
    //
    spicelib::VEQU(save.SPOINT.as_slice(), save.STATES.subarray_mut([1, 2]));

    //
    // Time bounds for the segment:
    //

    save.FIRST = (((-50 as f64) * spicelib::SPD()) * 365.25);
    save.STEP = (((100 as f64) * spicelib::SPD()) * 365.25);

    save.LAST = ((save.FIRST + save.STEP) - 0.000001);

    //
    // Get the OCTL ID code from the kernel we just
    // loaded.
    //
    spicelib::BODN2C(OCTL, &mut save.OCTID, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Write the segment.
    //
    spicelib::SPKW08(
        save.HANDLE,
        save.OCTID,
        399,
        &save.FIXREF,
        save.FIRST,
        save.LAST,
        b"octl",
        1,
        2,
        save.STATES.as_slice(),
        save.FIRST,
        save.STEP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now load the OCTL SPK file.
    //
    spicelib::FURNSH(OCTSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create an outward normal vector at SPOINT.
    //
    spicelib::BODVRD(
        b"EARTH",
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SURFNM(
        save.RADII[1],
        save.RADII[2],
        save.RADII[3],
        save.SPOINT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Phase angle tests: we'll compare results from GFILUM to those
    // obtained using GFPA. Note that the surface point must be
    // an ephemeris object in order to carry out these tests.
    //
    fstr::assign(&mut save.METHOD, b"Ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.ILLUM, b"SUN");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");

    //
    // Set up the sequence of test times.
    //
    fstr::assign(&mut save.TIME0, b"2011 JAN 1");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Search over approximately two months. Take NTIMES samples.
    //
    save.ET1 = (save.ET0 + ((60 as f64) * spicelib::SPD()));

    save.STEP = ((save.ET1 - save.ET0) / (NTIMES - 1) as f64);

    //
    // Loop over all aberration corrections and all
    // observation epochs.
    //
    for CORIDX in 1..=NCORR {
        fstr::assign(&mut save.ABCORR, save.CORRS.get(CORIDX));

        //
        // Parse the aberration correction specifier.
        //
        spicelib::ZZPRSCOR(&save.ABCORR, save.ATTBLK.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.USELT = save.ATTBLK[LTIDX];
        save.USESTL = save.ATTBLK[STLIDX];
        save.USECN = save.ATTBLK[CNVIDX];
        save.GEOM = save.ATTBLK[GEOIDX];

        for EPIDX in 1..=NTIMES {
            //
            //---- Case -------------------------------------------------------------
            //
            save.ET = (save.ET0 + (((EPIDX - 1) as f64) * save.STEP));

            spicelib::TIMOUT(
                save.ET,
                b"YYYY MON DD HR:MN:SC.######::TDB TDB",
                &mut save.TIMSTR,
                ctx,
            )?;

            fstr::assign(&mut save.TITLE, b"ZZILUSTA test: ABCORR = #; observer = #; target = #; illum source = #; FIXREF = #; SPOINT = ( # # # ); ET = # (#); EPIDX = #.");

            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ILLUM, &mut save.TITLE);
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.FIXREF, &mut save.TITLE);
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.SPOINT[1],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.SPOINT[2],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.SPOINT[3],
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TIMSTR, &mut save.TITLE);
            spicelib::REPMD(
                &save.TITLE.to_vec(),
                b"#",
                save.ET,
                14,
                &mut save.TITLE,
                ctx,
            );
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", EPIDX, &mut save.TITLE, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            // CALL TOSTDO ( TITLE )

            //
            // Look up illumination angle states.
            //
            spicelib::ZZILUSTA(
                &save.METHOD,
                &save.TARGET,
                &save.ILLUM,
                save.ET,
                &save.FIXREF,
                &save.ABCORR,
                &save.OBSRVR,
                save.SPOINT.as_slice(),
                save.NORMAL.as_slice(),
                save.PHSSTA.as_slice_mut(),
                save.SOLSTA.as_slice_mut(),
                save.EMISTA.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // First compare angles against those from ILUMIN.
            // The algorithms used should be quite similar,
            // so we can use a tight tolerance.
            //
            spicelib::ILUMIN(
                &save.METHOD,
                &save.TARGET,
                save.ET,
                &save.FIXREF,
                &save.ABCORR,
                &save.OBSRVR,
                save.SPOINT.as_slice(),
                &mut save.TRGEPC,
                save.SRFVEC.as_slice_mut(),
                &mut save.PHASE,
                &mut save.SOLAR,
                &mut save.EMISSN,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check the phase angle.
            //
            testutil::CHCKSD(
                b"PHASE",
                *save.PHSSTA.first(),
                b"~/",
                save.PHASE,
                ANGTOL,
                OK,
                ctx,
            )?;

            //
            // Check the solar incidence angle.
            //
            testutil::CHCKSD(
                b"SOLAR",
                *save.SOLSTA.first(),
                b"~/",
                save.SOLAR,
                ANGTOL,
                OK,
                ctx,
            )?;

            //
            // Check the emission angle.
            //
            testutil::CHCKSD(
                b"EMISTA",
                *save.EMISTA.first(),
                b"~/",
                save.EMISSN,
                ANGTOL,
                OK,
                ctx,
            )?;

            //
            // Now compute illumination angles via an independent
            // algorithm.
            //
            // This algorithm computes aberration corrections for
            // the illumination source less accurately, so adjust
            // the tolerance when aberration corrections are used.
            //
            if save.ATTBLK[GEOIDX] {
                save.TOL = ANGTOL;
            } else {
                save.TOL = SINGLE;
            }

            testutil::T_ILUMIN(
                &save.METHOD,
                &save.TARGET,
                save.ET,
                &save.FIXREF,
                &save.ABCORR,
                &save.OBSRVR,
                save.SPOINT.as_slice(),
                &mut save.TRGEPC,
                save.SRFVEC.as_slice_mut(),
                &mut save.PHASE,
                &mut save.SOLAR,
                &mut save.EMISSN,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check the phase angle.
            //
            testutil::CHCKSD(
                b"PHASE (2)",
                *save.PHSSTA.first(),
                b"~",
                save.PHASE,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Check the solar incidence angle.
            //
            testutil::CHCKSD(
                b"SOLAR (2)",
                *save.SOLSTA.first(),
                b"~",
                save.SOLAR,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Check the emission angle.
            //
            testutil::CHCKSD(
                b"EMISTA (2)",
                *save.EMISTA.first(),
                b"~",
                save.EMISSN,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Now for the interesting part: check the angular rates.
            // We'll compute discrete derivatives of each one and
            // compare these to the rates produced by ZZILUSTA.
            //
            // Fit Cheby expansions to the illumination angles
            // and find derivatives of the expansion wrt time at ET.
            //
            T_DZZILU(
                &save.METHOD,
                &save.TARGET,
                &save.ILLUM,
                save.ET,
                &save.FIXREF,
                &save.ABCORR,
                &save.OBSRVR,
                save.SPOINT.as_slice(),
                save.NORMAL.as_slice(),
                &mut save.DPHASE,
                &mut save.DINCDN,
                &mut save.DEMSSN,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.DFDT[1] = save.DPHASE;
            save.DFDT[2] = save.DINCDN;
            save.DFDT[3] = save.DEMSSN;

            spicelib::ZZILUSTA(
                &save.METHOD,
                &save.TARGET,
                &save.ILLUM,
                save.ET,
                &save.FIXREF,
                &save.ABCORR,
                &save.OBSRVR,
                save.SPOINT.as_slice(),
                save.NORMAL.as_slice(),
                save.PHSSTA.as_slice_mut(),
                save.SOLSTA.as_slice_mut(),
                save.EMISTA.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //        WRITE (*,*) 'Cheby Phase rate:          ', DPHASE
            //        WRITE (*,*) 'ZZILUSTA Phase rate:  ', PHSSTA(2)
            //        WRITE (*,*) 'Difference:           ', DPHASE-PHSSTA(2)
            //        WRITE (*,*) 'Phase rate rel error:      ',
            // .                                  (DPHASE-PHSSTA(2))/DPHASE
            //        WRITE (*,*) 'Cheby Incidence rate:      ', DINCDN
            //        WRITE (*,*) 'Incidence rate rel error:  ',
            // .                                  (DINCDN-SOLSTA(2))/DINCDN
            //        WRITE (*,*) 'Cheby Emission rate:       ', DEMSSN
            //        WRITE (*,*) 'Emission rate rel error:   ',
            // .                                  (DEMSSN-EMISTA(2))/DEMSSN
            //
            //        WRITE (*,*) ' '

            //
            // Check the phase angle rate.
            //
            if save.GEOM {
                save.TOL = DPHTOL;
            } else {
                save.TOL = (DPHTOL * 1.5);
            }

            testutil::CHCKSD(
                b"d PHASE/dt",
                save.PHSSTA[2],
                b"~/",
                save.DFDT[1],
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Check the solar incidence angle rate.
            //
            save.TOL = DINTOL;

            testutil::CHCKSD(
                b"d SOLAR/dt",
                save.SOLSTA[2],
                b"~/",
                save.DFDT[2],
                save.TOL,
                OK,
                ctx,
            )?;
            //
            // Check the emission angle rate.
            //
            save.TOL = DEMTOL;

            testutil::CHCKSD(
                b"d EMISSN/dt",
                save.EMISTA[2],
                b"~/",
                save.DFDT[3],
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Now check the angular rates using an alternate
            // analytic method. We'll do the computation inline.
            //
            // We'll use a body-fixed frame to compute the
            // angles and rates. Life will be easier if the
            // frame is centered at the surface point.
            //
            fstr::assign(&mut save.SRFREF, b"OCTL_TOPO");

            //
            // Get the observer-surface point state at ET in the surface
            // point centered, body-fixed frame. Negate this state to
            // obtain the surface point-observer state.
            //
            spicelib::SPKEZR(
                b"OCTL",
                save.ET,
                &save.SRFREF,
                &save.ABCORR,
                &save.OBSRVR,
                save.SSTATE.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VMINUG(save.SSTATE.as_slice(), 6, save.OBSSTA.as_slice_mut());

            //
            // Compute the epoch associated with the surface point.
            //
            // Also compute the rate of change of light time.
            //
            spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"OBSID FOUND", save.FOUND, true, OK, ctx)?;

            spicelib::SPKACS(
                save.OCTID,
                save.ET,
                IREF,
                &save.ABCORR,
                save.OBSID,
                save.TMPSTA.as_slice_mut(),
                &mut save.LT2,
                &mut save.DLT,
                ctx,
            )?;

            if save.ATTBLK[GEOIDX] {
                save.ETSURF = save.ET;
                save.DLT = 0.0;
            } else {
                save.ETSURF = (save.ET - save.LT);
            }

            //
            // Compute the surface point-illumination source state
            // at ETSURF in the surface point body-fixed frame.
            //
            spicelib::SPKEZR(
                &save.ILLUM,
                save.ETSURF,
                &save.SRFREF,
                &save.ABCORR,
                b"OCTL",
                save.SRCSTA.as_slice_mut(),
                &mut save.LTSRC,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Adjust the surface point-illumination source velocity
            // for the rate of change of ETSURF with respect to ET.
            //
            spicelib::VSCLIP((1.0 - save.DLT), save.SRCSTA.subarray_mut(4));

            //
            // Create a state for the normal vector. Transform
            // the normal vector to frame SRFREF.
            //
            spicelib::PXFORM(
                &save.FIXREF,
                &save.SRFREF,
                save.ETSURF,
                save.R.as_slice_mut(),
                ctx,
            )?;
            spicelib::MXV(
                save.R.as_slice(),
                save.NORMAL.as_slice(),
                save.NRMSTA.as_slice_mut(),
            );
            spicelib::CLEARD(3, save.NRMSTA.subarray_mut(4));

            //
            // Compute and check expected rates.
            //
            // Phase angle rate:
            //
            save.XDPHAS = spicelib::DVSEP(save.OBSSTA.as_slice(), save.SRCSTA.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(
                b"d PHASE/dt (2)",
                save.PHSSTA[2],
                b"~/",
                save.XDPHAS,
                ALTTOL,
                OK,
                ctx,
            )?;
            //
            // Emission angle rate:
            //
            save.XDEMIT = spicelib::DVSEP(save.OBSSTA.as_slice(), save.NRMSTA.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(
                b"d EMISSN/dt (2)",
                save.EMISTA[2],
                b"~/",
                save.XDEMIT,
                ALTTOL,
                OK,
                ctx,
            )?;

            //
            // Solar incidence angle rate:
            //
            save.XDSOLR = spicelib::DVSEP(save.SRCSTA.as_slice(), save.NRMSTA.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(
                b"d SOLAR/dt (2)",
                save.SOLSTA[2],
                b"~/",
                save.XDSOLR,
                ALTTOL,
                OK,
                ctx,
            )?;

            //
            // Now compute the rates using a second, alternate,
            // analytic method.
            //
            // This time we compute rates using ETSURF as the
            // observation epoch. After computing the rates,
            // we adjust them so they express the rates of change
            // of the illumination angles with respect to ET.
            //
            // Start out by creating a transmission aberration
            // correction specifier, if corrections are being
            // used.
            //
            if save.USELT {
                spicelib::LJUST(&save.ABCORR, &mut save.XMTCOR);
                spicelib::PREFIX(b"X", 0, &mut save.XMTCOR);
            } else {
                save.DLT = 0.0;
                fstr::assign(&mut save.XMTCOR, b"NONE");
            }

            //
            // Compute the surface point-observer state at ETSURF.
            //
            if save.USESTL {
                //
                // Due to the asymmetry of stellar aberration
                // corrections, it's easiest to compute the
                // surface point-observer vector in the usual
                // way.
                //
                spicelib::SPKEZR(
                    b"OCTL",
                    save.ET,
                    &save.SRFREF,
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.SSTATE.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::VMINUG(save.SSTATE.as_slice(), 6, save.OBSST2.as_slice_mut());

                //
                // Adjust the velocity portion of the state so
                // that it is the derivative of position with
                // respect to ETSURF rather than ET.
                //
                spicelib::VSCLIP((1.0 / (1.0 - save.DLT)), save.OBSST2.subarray_mut(4));
            } else {
                //
                // We can compute the surface point-observer state
                // directly using transmission corrections and
                // an event time of ETSURF.
                //
                spicelib::SPKEZR(
                    &save.OBSRVR,
                    save.ETSURF,
                    &save.SRFREF,
                    &save.XMTCOR,
                    b"OCTL",
                    save.OBSST2.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }

            //
            // Compute the surface point-illumination source
            // state at ETSURF.
            //
            spicelib::SPKEZR(
                &save.ILLUM,
                save.ETSURF,
                &save.SRFREF,
                &save.ABCORR,
                b"OCTL",
                save.SRCST2.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MOVED(save.OBSST2.as_slice(), 6, save.TMPSTA.as_slice_mut());

            spicelib::VSCLIP((1.0 - save.DLT), save.TMPSTA.subarray_mut(4));

            spicelib::MOVED(save.SRCST2.as_slice(), 6, save.TMPSTA.as_slice_mut());

            spicelib::VSCLIP((1.0 - save.DLT), save.TMPSTA.subarray_mut(4));
            //
            // We already have the state of the normal vector.
            //
            //
            // Compute the phase angle rates; scale each one
            // by the rate of change of ETSURF with respect to
            // ET.
            //
            if (save.GEOM || save.USECN) {
                save.TOL = ALTTOL;
            } else {
                save.TOL = MEDTOL;
            }
            //
            // Phase angle rate:
            //
            save.XDPHAS = (spicelib::DVSEP(save.OBSST2.as_slice(), save.SRCST2.as_slice(), ctx)?
                * (1.0 - save.DLT));
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(
                b"d PHASE/dt (3)",
                save.PHSSTA[2],
                b"~/",
                save.XDPHAS,
                save.TOL,
                OK,
                ctx,
            )?;
            //
            // Emission angle rate:
            //
            save.XDEMIT = (spicelib::DVSEP(save.OBSST2.as_slice(), save.NRMSTA.as_slice(), ctx)?
                * (1.0 - save.DLT));
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(
                b"d EMISSN/dt (3)",
                save.EMISTA[2],
                b"~/",
                save.XDEMIT,
                save.TOL,
                OK,
                ctx,
            )?;

            //
            // Solar incidence angle rate:
            //
            save.XDSOLR = (spicelib::DVSEP(save.SRCST2.as_slice(), save.NRMSTA.as_slice(), ctx)?
                * (1.0 - save.DLT));
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(
                b"d SOLAR/dt (3)",
                save.SOLSTA[2],
                b"~/",
                save.XDSOLR,
                save.TOL,
                OK,
                ctx,
            )?;
        }
        //
        // End of epoch loop.
        //
    }
    //
    // End of aberration correction loop.
    //

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Clean up. Unload and delete kernels.", ctx)?;

    //
    // Clean up SPK files.
    //
    spicelib::SPKUEF(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(OCTSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(OCTSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
