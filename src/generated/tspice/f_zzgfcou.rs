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
const PCK: &[u8] = b"zzgfcou.tpc";
const SPK: &[u8] = b"zzgfcou.bsp";
const TIGHT: f64 = 0.0000000000000001;
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 200;
const FRNMLN: i32 = 32;
const TIMLEN: i32 = 50;
const NCORR: i32 = 9;
const NREF: i32 = 4;
const NSAMP: i32 = 10;
const NSYS: i32 = 7;
const SYSLEN: i32 = 32;
const CRDLEN: i32 = 32;
const VDFLEN: i32 = 32;
const NVDEF: i32 = 3;
const MTHLEN: i32 = 80;

struct SaveVars {
    ABCORR: Vec<u8>,
    CENTER: Vec<u8>,
    CORR: ActualCharArray,
    CRDNMS: ActualCharArray2D,
    CRDNAM: Vec<u8>,
    CRDSYS: Vec<u8>,
    DREF: Vec<u8>,
    EFRAME: ActualCharArray,
    REF: Vec<u8>,
    METHOD: Vec<u8>,
    OBSRVR: Vec<u8>,
    QNAME: Vec<u8>,
    SYSNMS: ActualCharArray,
    TARGET: Vec<u8>,
    TIMSTR: Vec<u8>,
    TITLE: Vec<u8>,
    VCDEFS: ActualCharArray,
    VECDEF: Vec<u8>,
    BADRAD: StackArray<f64, 3>,
    DELTA: f64,
    DVEC: StackArray<f64, 3>,
    ET: f64,
    ET0: f64,
    F: f64,
    JACOBI: StackArray2D<f64, 9>,
    LT: f64,
    NEGVEC: StackArray<f64, 3>,
    RADII: StackArray<f64, 3>,
    RE: f64,
    REFVAL: f64,
    SRFVEC: StackArray<f64, 3>,
    TRGEPC: f64,
    VALUE: f64,
    XCOORD: StackArray<f64, 3>,
    XPOS: StackArray<f64, 3>,
    XSTATE: StackArray<f64, 6>,
    XDCOR: StackArray<f64, 3>,
    DCTR: i32,
    FI: i32,
    FRCLID: i32,
    FRCLSS: i32,
    FRCODE: i32,
    HANDLE: i32,
    N: i32,
    NF: i32,
    OBSID: i32,
    RCTR: i32,
    TRGID: i32,
    ATTBLK: StackArray<bool, 6>,
    CCDEC: bool,
    CCNDEC: bool,
    CDEC: bool,
    CNDEC: bool,
    CSDEC: bool,
    CSNDEC: bool,
    DECRES: bool,
    FND: bool,
    FOUND: bool,
    LSSTHN: bool,
    XDECRS: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut CENTER = vec![b' '; BDNMLN as usize];
        let mut CORR = ActualCharArray::new(LNSIZE, 1..=NCORR);
        let mut CRDNMS = ActualCharArray2D::new(CRDLEN, 1..=3, 1..=NSYS);
        let mut CRDNAM = vec![b' '; CRDLEN as usize];
        let mut CRDSYS = vec![b' '; SYSLEN as usize];
        let mut DREF = vec![b' '; FRNMLN as usize];
        let mut EFRAME = ActualCharArray::new(FRNMLN, 1..=NREF);
        let mut REF = vec![b' '; FRNMLN as usize];
        let mut METHOD = vec![b' '; MTHLEN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut SYSNMS = ActualCharArray::new(SYSLEN, 1..=NSYS);
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TIMSTR = vec![b' '; TIMLEN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut VCDEFS = ActualCharArray::new(VDFLEN, 1..=NVDEF);
        let mut VECDEF = vec![b' '; VDFLEN as usize];
        let mut BADRAD = StackArray::<f64, 3>::new(1..=3);
        let mut DELTA: f64 = 0.0;
        let mut DVEC = StackArray::<f64, 3>::new(1..=3);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut F: f64 = 0.0;
        let mut JACOBI = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut LT: f64 = 0.0;
        let mut NEGVEC = StackArray::<f64, 3>::new(1..=3);
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RE: f64 = 0.0;
        let mut REFVAL: f64 = 0.0;
        let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut TRGEPC: f64 = 0.0;
        let mut VALUE: f64 = 0.0;
        let mut XCOORD = StackArray::<f64, 3>::new(1..=3);
        let mut XPOS = StackArray::<f64, 3>::new(1..=3);
        let mut XSTATE = StackArray::<f64, 6>::new(1..=6);
        let mut XDCOR = StackArray::<f64, 3>::new(1..=3);
        let mut DCTR: i32 = 0;
        let mut FI: i32 = 0;
        let mut FRCLID: i32 = 0;
        let mut FRCLSS: i32 = 0;
        let mut FRCODE: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut N: i32 = 0;
        let mut NF: i32 = 0;
        let mut OBSID: i32 = 0;
        let mut RCTR: i32 = 0;
        let mut TRGID: i32 = 0;
        let mut ATTBLK = StackArray::<bool, 6>::new(1..=ABATSZ);
        let mut CCDEC: bool = false;
        let mut CCNDEC: bool = false;
        let mut CDEC: bool = false;
        let mut CNDEC: bool = false;
        let mut CSDEC: bool = false;
        let mut CSNDEC: bool = false;
        let mut DECRES: bool = false;
        let mut FND: bool = false;
        let mut FOUND: bool = false;
        let mut LSSTHN: bool = false;
        let mut XDECRS: bool = false;

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

            let mut clist = [
                Val::C(b"J2000"),
                Val::C(b"IAU_EARTH"),
                Val::C(b"ECLIPJ2000"),
                Val::C(b"IAU_MOON"),
            ]
            .into_iter();
            EFRAME
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(RECSYS),
                Val::C(LATSYS),
                Val::C(RADSYS),
                Val::C(SPHSYS),
                Val::C(CYLSYS),
                Val::C(GEOSYS),
                Val::C(PGRSYS),
            ]
            .into_iter();
            SYSNMS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(XCRD),
                Val::C(YCRD),
                Val::C(ZCRD),
                Val::C(RADCRD),
                Val::C(LONCRD),
                Val::C(LATCRD),
                Val::C(RNGCRD),
                Val::C(RACRD),
                Val::C(DECCRD),
                Val::C(RADCRD),
                Val::C(CLTCRD),
                Val::C(LONCRD),
                Val::C(RADCRD),
                Val::C(LONCRD),
                Val::C(ZCRD),
                Val::C(LONCRD),
                Val::C(LATCRD),
                Val::C(ALTCRD),
                Val::C(LONCRD),
                Val::C(LATCRD),
                Val::C(ALTCRD),
            ]
            .into_iter();
            CRDNMS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(POSDEF), Val::C(SOBDEF), Val::C(SINDEF)].into_iter();
            VCDEFS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            CENTER,
            CORR,
            CRDNMS,
            CRDNAM,
            CRDSYS,
            DREF,
            EFRAME,
            REF,
            METHOD,
            OBSRVR,
            QNAME,
            SYSNMS,
            TARGET,
            TIMSTR,
            TITLE,
            VCDEFS,
            VECDEF,
            BADRAD,
            DELTA,
            DVEC,
            ET,
            ET0,
            F,
            JACOBI,
            LT,
            NEGVEC,
            RADII,
            RE,
            REFVAL,
            SRFVEC,
            TRGEPC,
            VALUE,
            XCOORD,
            XPOS,
            XSTATE,
            XDCOR,
            DCTR,
            FI,
            FRCLID,
            FRCLSS,
            FRCODE,
            HANDLE,
            N,
            NF,
            OBSID,
            RCTR,
            TRGID,
            ATTBLK,
            CCDEC,
            CCNDEC,
            CDEC,
            CNDEC,
            CSDEC,
            CSNDEC,
            DECRES,
            FND,
            FOUND,
            LSSTHN,
            XDECRS,
        }
    }
}

//$Procedure F_ZZGFCOU ( Test GF coordinate quantity utilities )
pub fn F_ZZGFCOU(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Local parameters
    //

    //
    // Number of recognized coordinate systems.
    //

    //
    // Maximum length of a coordinate system name.
    //

    //
    // Maximum length of a coordinate name.
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
    // Saved Variables
    //

    //
    // Initial values
    //

    //
    // Names of supported coordinate systems.
    //
    // The Ith coordinate system in the array SYSNMS has coordinates
    // in the Ith row of the array CRDNMS. This association must be
    // preserved when this routine is updated.
    //

    //
    // Names of coordinate triples for the supported coordinate
    // systems.
    //
    // The order of the coordinate names in the Ith row of this array
    // matches the order of the outputs of the corresponding
    // SPICELIB routine REC*, which maps a Cartesian vector to
    // the Ith coordinate system in the array SYSNMS. Again, this
    // order must be preserved.
    //

    //
    // Vector definition methods:
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFCOU", ctx)?;

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
    // Actual DE-based ephemerides yield better comparisons,
    // since these ephemerides have less noise than do those
    // produced by TSTSPK.
    //
    //  CALL FURNSH ( 'de421.bsp' )
    //  CALL FURNSH ( 'jup230.bsp' )
    //

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    // The following error cases involve invalid initialization
    // values or missing data discovered at search time.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad observer or target ID name.", ctx)?;

    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ellipsoid");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, &save.REF);
    save.DCTR = 399;
    fstr::assign(&mut save.CRDSYS, b"Latitudinal");
    fstr::assign(&mut save.CRDNAM, b"Latitude");

    spicelib::VPACK(0.0, 0.0, 1.0, save.DVEC.as_slice_mut());

    fstr::assign(&mut save.TIMSTR, b"2008 MAY 1");

    spicelib::STR2ET(&save.TIMSTR, &mut save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODVRD(
        &save.TARGET,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.RE = save.RADII[1];
    save.F = ((save.RE - save.RADII[3]) / save.RE);

    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

    fstr::assign(&mut save.TARGET, b"X");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"Y");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Observer equals target", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"EARTH");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad vector definition", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.METHOD, b"Near point");

    fstr::assign(&mut save.VECDEF, b"ACCELERATION");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad computation method", ctx)?;

    //
    // This error is detected post-initialization.
    //
    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.METHOD, b"Near point");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOG(&mut 0.0.clone(), &mut save.VALUE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad aberration correction", ctx)?;

    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");

    fstr::assign(&mut save.ABCORR, b"None.");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad coordinate system", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XCN+S");

    fstr::assign(&mut save.CRDSYS, b"GAUSSIAN");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad coordinate name", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XCN+S");

    fstr::assign(&mut save.CRDSYS, b"CYLINDRICAL");
    fstr::assign(&mut save.CRDNAM, b"LATITUDE");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad reference frame REF", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XCN+S");

    fstr::assign(&mut save.CRDSYS, b"cylindrical");
    fstr::assign(&mut save.CRDNAM, b"z");

    fstr::assign(&mut save.REF, b"EME2000");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Reference frame REF not centered on target when VECDEF requires it.",
        ctx,
    )?;

    fstr::assign(&mut save.VECDEF, SOBDEF);
    fstr::assign(&mut save.METHOD, b"NEAR POINT: ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"Mars");
    fstr::assign(&mut save.REF, b"IAU_EARTH");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    fstr::assign(&mut save.VECDEF, SINDEF);

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // Make sure an error is NOT signaled for the POSDEF case.
    //
    fstr::assign(&mut save.VECDEF, POSDEF);

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-existent reference frame DREF", ctx)?;

    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XCN+S");

    fstr::assign(&mut save.CRDSYS, b"CYLINDRICAL");
    fstr::assign(&mut save.CRDNAM, b"Z");

    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b"EME2000");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOFRAME)", OK, ctx)?;

    //
    // Make sure an error is NOT signaled for the POSDEF
    // and SOBDEF cases.
    //
    fstr::assign(&mut save.VECDEF, POSDEF);

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.VECDEF, SOBDEF);
    fstr::assign(&mut save.METHOD, b"NEAR POINT: ELLIPSOID");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad vector DVEC", ctx)?;

    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"XCN+S");

    fstr::assign(&mut save.CRDSYS, b"RECTANGULAR");
    fstr::assign(&mut save.CRDNAM, b"Z");

    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b"J2000");

    spicelib::CLEARD(3, save.DVEC.as_slice_mut());

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad target radii count", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.DVEC.as_slice_mut());

    fstr::assign(&mut save.CRDSYS, b"LATITUDINAL");
    fstr::assign(&mut save.CRDNAM, b"LATITUDE");

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.DREF, b"J2000");

    spicelib::GDPOOL(
        b"BODY399_RADII",
        1,
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(b"BODY399_RADII", 2, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // Make sure an error is NOT signaled for the POSDEF
    // case. (Since the system is latitudinal, the radii
    // are not needed for the coordinate computation.)
    //
    fstr::assign(&mut save.VECDEF, POSDEF);

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore all three radii.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad target radii values", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, save.DVEC.as_slice_mut());

    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.CRDSYS, b"GEODETIC");
    fstr::assign(&mut save.CRDNAM, b"LATITUDE");

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.DREF, b"J2000");

    spicelib::GDPOOL(
        b"BODY399_RADII",
        1,
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(0.0, 1.0, 1.0, save.BADRAD.as_slice_mut());

    spicelib::PDPOOL(b"BODY399_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::VPACK(1.0, 0.0, 1.0, save.BADRAD.as_slice_mut());
    spicelib::PDPOOL(b"BODY399_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Set the coordinate system to planetographic.
    //
    fstr::assign(&mut save.CRDSYS, PGRSYS);

    spicelib::VPACK(1.0, -1.0, 1.0, save.BADRAD.as_slice_mut());
    spicelib::PDPOOL(b"BODY399_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Set the vector definition to sub-observer point.
    //
    fstr::assign(&mut save.VECDEF, SOBDEF);
    fstr::assign(&mut save.METHOD, b"Ellipsoid: near point");

    spicelib::VPACK(1.0, 1.0, 0.0, save.BADRAD.as_slice_mut());
    spicelib::PDPOOL(b"BODY399_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Set the vector definition to position. Since the coordinate
    // system is planetographic, radii for the frame center are
    // needed.
    //
    fstr::assign(&mut save.VECDEF, POSDEF);

    spicelib::VPACK(1.0, 1.0, -1.0, save.BADRAD.as_slice_mut());
    spicelib::PDPOOL(b"BODY399_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Restore all three radii for the Earth.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Repeat the above test with a frame whose center
    // is not the target.
    //
    fstr::assign(&mut save.VECDEF, POSDEF);
    fstr::assign(&mut save.REF, b"IAU_MARS");

    spicelib::GDPOOL(
        b"BODY499_RADII",
        1,
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(1.0, 1.0, -1.0, save.BADRAD.as_slice_mut());
    spicelib::PDPOOL(b"BODY499_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Restore all three radii for Mars.
    //
    spicelib::PDPOOL(b"BODY499_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Unequal ellipsoid equatorial radii values with GEODETIC coordinates.",
        ctx,
    )?;
    //
    // Surface intercept computation:
    //
    fstr::assign(&mut save.VECDEF, SINDEF);
    spicelib::VPACK(0.0, 0.0, 1.0, save.DVEC.as_slice_mut());

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.CRDSYS, b"GEODETIC");
    fstr::assign(&mut save.CRDNAM, b"LATITUDE");

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.DREF, b"J2000");

    spicelib::GDPOOL(
        b"BODY399_RADII",
        1,
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(6500.0, 6200.0, 6000.0, save.BADRAD.as_slice_mut());

    spicelib::PDPOOL(b"BODY399_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // Sub-observer computation:
    //
    fstr::assign(&mut save.VECDEF, SOBDEF);
    fstr::assign(&mut save.METHOD, b"Ellipsoid: near point");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // Restore all three radii for the Earth.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // Set the vector definition to position. Use a frame whose center
    // is not the target.
    //
    fstr::assign(&mut save.VECDEF, POSDEF);
    fstr::assign(&mut save.REF, b"IAU_MARS");

    spicelib::GDPOOL(
        b"BODY499_RADII",
        1,
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(4000.0, 5000.0, 3000.0, save.BADRAD.as_slice_mut());
    spicelib::PDPOOL(b"BODY499_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // Restore all three radii for Mars.
    //
    spicelib::PDPOOL(b"BODY499_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Unequal ellipsoid equatorial radii values with PLANETOGRAPHIC coordinates.",
        ctx,
    )?;

    //
    // Surface intercept computation:
    //
    fstr::assign(&mut save.VECDEF, SINDEF);
    spicelib::VPACK(0.0, 0.0, 1.0, save.DVEC.as_slice_mut());

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.CRDSYS, b"PLANETOGRAPHIC");
    fstr::assign(&mut save.CRDNAM, b"LATITUDE");

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.ABCORR, b"None");
    fstr::assign(&mut save.DREF, b"J2000");

    spicelib::GDPOOL(
        b"BODY399_RADII",
        1,
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(6500.0, 6200.0, 6000.0, save.BADRAD.as_slice_mut());

    spicelib::PDPOOL(b"BODY399_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // Sub-observer computation:
    //
    fstr::assign(&mut save.VECDEF, SOBDEF);
    fstr::assign(&mut save.METHOD, b"Ellipsoid: near point");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // Restore all three radii for the Earth.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // Set the vector definition to position. Use a frame whose center
    // is not the target.
    //
    fstr::assign(&mut save.VECDEF, POSDEF);
    fstr::assign(&mut save.REF, b"IAU_MARS");

    spicelib::GDPOOL(
        b"BODY499_RADII",
        1,
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(4000.0, 5000.0, 3000.0, save.BADRAD.as_slice_mut());
    spicelib::PDPOOL(b"BODY499_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // Restore all three radii for Mars.
    //
    spicelib::PDPOOL(b"BODY499_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Unrecognized frame", ctx)?;

    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ellipsoid");
    fstr::assign(&mut save.REF, b"ITRF95");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOFRAME)", OK, ctx)?;

    //
    // Restore the frame.
    //
    fstr::assign(&mut save.REF, b"ITRF93");

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No target orientation data available", ctx)?;

    //
    // This error is detected post-initialization.
    //
    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ellipsoid");
    fstr::assign(&mut save.REF, b"ITRF93");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOG(&mut 0.0.clone(), &mut save.VALUE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No target ephemeris data available", ctx)?;
    //
    // This error is detected post-initialization.
    //

    fstr::assign(&mut save.TARGET, b"GASPRA");
    save.TRGID = 9511010;
    fstr::assign(&mut save.REF, b"IAU_GASPRA");
    fstr::assign(&mut save.CRDSYS, b"LATITUDINAL");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOG(&mut 0.0.clone(), &mut save.VALUE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No observer ephemeris data available", ctx)?;
    //
    // This error is detected post-initialization.
    //

    fstr::assign(&mut save.TARGET, b"MOON");
    save.TRGID = 301;
    fstr::assign(&mut save.OBSRVR, b"GASPRA");
    save.OBSID = 9511010;
    fstr::assign(&mut save.REF, b"IAU_MOON");

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOG(&mut 0.0.clone(), &mut save.VALUE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No DREF ephemeris data available", ctx)?;
    //
    // This error is detected post-initialization.
    //

    fstr::assign(&mut save.DREF, b"IAU_GASPRA");
    save.DCTR = 9511010;
    spicelib::VPACK(0.0, 0.0, 1.0, save.DVEC.as_slice_mut());

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOG(&mut 0.0.clone(), &mut save.VALUE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No DREF orientation data available", ctx)?;
    //
    // This error is detected post-initialization.
    //

    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"EARTH");
    save.TRGID = 399;
    fstr::assign(&mut save.OBSRVR, b"MOON");
    save.OBSID = 301;
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b"ITRF93");
    save.DCTR = 399;
    spicelib::VPACK(0.0, 0.0, 1.0, save.DVEC.as_slice_mut());

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFCOG(&mut 0.0.clone(), &mut save.VALUE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFUDLT o ZZGFCOG: coordinate not computable", ctx)?;

    fstr::assign(&mut save.VECDEF, SINDEF);
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"EARTH");
    save.TRGID = 399;
    fstr::assign(&mut save.OBSRVR, b"MOON");
    save.OBSID = 301;
    fstr::assign(&mut save.REF, b"IAU_EARTH");
    fstr::assign(&mut save.DREF, b"J2000");

    spicelib::VPACK(0.0, 0.0, 1.0, save.DVEC.as_slice_mut());

    spicelib::ZZGFCOIN(
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
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFUDLT(spicelib::ZZGFCOG, &mut 0.0.clone(), &mut save.LSSTHN, ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTCOMPUTABLE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFCODC: coordinate not computable", ctx)?;
    //
    // Use the setup values from the previous case.
    //
    spicelib::ZZGFCODC(spicelib::UDF, &mut 0.0.clone(), &mut save.DECRES, ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTCOMPUTABLE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFCOCG: coordinate not computable", ctx)?;
    //
    // Use the setup values from the previous case.
    //
    spicelib::ZZGFCOCG(&mut 0.0.clone(), &mut save.VALUE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTCOMPUTABLE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFCOSG: coordinate not computable", ctx)?;
    //
    // Use the setup values from the previous case.
    //
    spicelib::ZZGFCOSG(&mut 0.0.clone(), &mut save.VALUE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTCOMPUTABLE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFUDLT o ZZGFCOCG: coordinate not computable", ctx)?;
    //
    // Use the setup values from the previous case.
    //
    spicelib::ZZGFUDLT(spicelib::ZZGFCOCG, &mut 0.0.clone(), &mut save.LSSTHN, ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTCOMPUTABLE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFUDLT o ZZGFCOSG: coordinate not computable", ctx)?;
    //
    // Use the setup values from the previous case.
    //
    spicelib::ZZGFUDLT(spicelib::ZZGFCOSG, &mut 0.0.clone(), &mut save.LSSTHN, ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTCOMPUTABLE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFCOCD: coordinate not computable", ctx)?;
    //
    // Use the setup values from the previous case.
    //
    spicelib::ZZGFCOCD(spicelib::UDF, &mut 0.0.clone(), &mut save.DECRES, ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTCOMPUTABLE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFCOSD: coordinate not computable", ctx)?;
    //
    // Use the setup values from the previous case.
    //
    spicelib::ZZGFCOSD(spicelib::UDF, &mut 0.0.clone(), &mut save.DECRES, ctx)?;

    testutil::CHCKXC(true, b"SPICE(NOTCOMPUTABLE)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    // Initialize flags indicating which cases were hit.
    //
    save.CDEC = false;
    save.CNDEC = false;
    save.CCDEC = false;
    save.CCNDEC = false;
    save.CSDEC = false;
    save.CSNDEC = false;

    //
    // We're going to loop over all vector definitions,
    // aberration corrections, coordinate systems, and
    // coordinates.
    //
    //
    for ND in 1..=NVDEF {
        fstr::assign(&mut save.VECDEF, save.VCDEFS.get(ND));

        if fstr::eq(&save.VECDEF, SINDEF) {
            fstr::assign(&mut save.METHOD, b"Ellipsoid");
        } else if fstr::eq(&save.VECDEF, SOBDEF) {
            fstr::assign(&mut save.METHOD, b"Near point: ellipsoid");
        } else {
            fstr::assign(&mut save.METHOD, b" ");
        }

        for NC in 1..=NCORR {
            fstr::assign(&mut save.ABCORR, save.CORR.get(NC));

            spicelib::ZZPRSCOR(&save.ABCORR, save.ATTBLK.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            for SI in 1..=NSYS {
                fstr::assign(&mut save.CRDSYS, save.SYSNMS.get(SI));
                //
                // Loop over the set of sample times.
                //
                fstr::assign(&mut save.TIMSTR, b"2008 MAY 1");

                spicelib::STR2ET(&save.TIMSTR, &mut save.ET0, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.DELTA = ((100 as f64) * spicelib::SPD());

                for NS in 1..=NSAMP {
                    //
                    // --- Case: ------------------------------------------------------
                    //
                    if fstr::eq(&save.VECDEF, POSDEF) {
                        //
                        // For vectors defined by positions, we don't
                        // have to use a frame centered on the target.
                        // In addition to the target-centered frames,
                        // we use body-fixed frames centered on a third body.
                        //
                        save.FI = (intrinsics::MOD(NS, 4) + 1);

                        if (save.FI == 1) {
                            fstr::assign(&mut save.TARGET, b"EARTH");
                            fstr::assign(&mut save.OBSRVR, b"MOON");
                            fstr::assign(&mut save.REF, b"IAU_EARTH");
                        } else if (save.FI == 2) {
                            fstr::assign(&mut save.TARGET, b"JUPITER");
                            fstr::assign(&mut save.OBSRVR, b"IO");
                            fstr::assign(&mut save.REF, b"IAU_JUPITER");
                        } else if (save.FI == 3) {
                            fstr::assign(&mut save.TARGET, b"JUPITER");
                            fstr::assign(&mut save.OBSRVR, b"IO");
                            fstr::assign(&mut save.REF, b"IAU_MARS");
                        } else if (save.FI == 4) {
                            fstr::assign(&mut save.TARGET, b"JUPITER");
                            fstr::assign(&mut save.OBSRVR, b"IO");
                            fstr::assign(&mut save.REF, b"IAU_MOON");
                        }
                    } else {
                        //
                        // We use two different observer-target combinations.
                        //
                        if spicelib::ODD(NS) {
                            fstr::assign(&mut save.TARGET, b"EARTH");
                            fstr::assign(&mut save.OBSRVR, b"MOON");
                            fstr::assign(&mut save.REF, b"IAU_EARTH");
                        } else {
                            fstr::assign(&mut save.TARGET, b"JUPITER");
                            fstr::assign(&mut save.OBSRVR, b"IO");
                            fstr::assign(&mut save.REF, b"IAU_JUPITER");
                        }
                    }

                    //
                    // Radii used with planetographic and geodetic
                    // systems are always associated with the center
                    // of the frame REF.
                    //
                    spicelib::NAMFRM(&save.REF, &mut save.FRCODE, ctx)?;
                    spicelib::FRINFO(
                        save.FRCODE,
                        &mut save.RCTR,
                        &mut save.FRCLSS,
                        &mut save.FRCLID,
                        &mut save.FND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    testutil::CHCKSL(b"FRINFO FND", save.FND, true, OK, ctx)?;

                    spicelib::BODVCD(
                        save.RCTR,
                        b"RADII",
                        3,
                        &mut save.N,
                        save.RADII.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.RE = save.RADII[1];
                    save.F = ((save.RADII[1] - save.RADII[3]) / save.RE);

                    //
                    // We'll later need the name of the frame center
                    // in order to call RECPGR.
                    //
                    spicelib::BODC2N(save.RCTR, &mut save.CENTER, &mut save.FOUND, ctx)?;

                    //
                    // Get ID codes of target and observer.
                    //
                    spicelib::BODN2C(&save.TARGET, &mut save.TRGID, &mut save.FOUND, ctx)?;
                    spicelib::BODN2C(&save.OBSRVR, &mut save.OBSID, &mut save.FOUND, ctx)?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Test ZZGFCOIN and ZZGFCOG: compare against
                    // results from ZZGFCOQ.
                    //
                    if fstr::eq(&save.VECDEF, POSDEF) {
                        fstr::assign(&mut save.TITLE, b"#; #-# position; #; #; #");

                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.REF, &mut save.TITLE);
                    } else if fstr::eq(&save.VECDEF, SOBDEF) {
                        fstr::assign(&mut save.TITLE, b"#; # sub # point on #; #; #");

                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.METHOD, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
                    } else if fstr::eq(&save.VECDEF, SINDEF) {
                        save.NF = (intrinsics::MOD(NS, 4) + 1);
                        fstr::assign(&mut save.DREF, save.EFRAME.get(save.NF));

                        fstr::assign(&mut save.TITLE, b"#; # #-# intercept. #; DREF = #; #");

                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.CRDSYS, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.METHOD, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.DREF, &mut save.TITLE);
                    }

                    save.ET = (save.ET0 + (((NS - 1) as f64) * save.DELTA));
                    spicelib::ETCAL(save.ET, &mut save.TIMSTR, ctx);
                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TIMSTR, &mut save.TITLE);

                    spicelib::PREFIX(b"COIN/COG:", 1, &mut save.TITLE);

                    testutil::TCASE(&save.TITLE, ctx)?;

                    if fstr::eq(&save.VECDEF, SINDEF) {
                        //
                        // Look up the direction vector.
                        //
                        spicelib::SPKPOS(
                            &save.TARGET,
                            save.ET,
                            &save.DREF,
                            &save.ABCORR,
                            &save.OBSRVR,
                            save.DVEC.as_slice_mut(),
                            &mut save.LT,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        spicelib::NAMFRM(&save.DREF, &mut save.FRCODE, ctx)?;
                        testutil::CHCKSI(b"FRCODE ", save.FRCODE, b"!=", 0, 0, OK, ctx)?;

                        spicelib::FRINFO(
                            save.FRCODE,
                            &mut save.DCTR,
                            &mut save.FRCLSS,
                            &mut save.FRCLID,
                            &mut save.FND,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                        testutil::CHCKSL(b"FRINFO FND", save.FND, true, OK, ctx)?;
                    }

                    //
                    // Look up the expected position vector.
                    //
                    if fstr::eq(&save.VECDEF, POSDEF) {
                        spicelib::SPKPOS(
                            &save.TARGET,
                            save.ET,
                            &save.REF,
                            &save.ABCORR,
                            &save.OBSRVR,
                            save.XPOS.as_slice_mut(),
                            &mut save.LT,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    } else if fstr::eq(&save.VECDEF, SOBDEF) {
                        spicelib::SUBPNT(
                            &save.METHOD,
                            &save.TARGET,
                            save.ET,
                            &save.REF,
                            &save.ABCORR,
                            &save.OBSRVR,
                            save.XPOS.as_slice_mut(),
                            &mut save.TRGEPC,
                            save.SRFVEC.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    } else {
                        spicelib::SINCPT(
                            &save.METHOD,
                            &save.TARGET,
                            save.ET,
                            &save.REF,
                            &save.ABCORR,
                            &save.OBSRVR,
                            &save.DREF,
                            save.DVEC.as_slice(),
                            save.XPOS.as_slice_mut(),
                            &mut save.TRGEPC,
                            save.SRFVEC.as_slice_mut(),
                            &mut save.FOUND,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // By construction, the intercept should be found.
                        //
                        testutil::CHCKSL(b"intercept found", save.FOUND, true, OK, ctx)?;
                    }

                    //
                    // Convert position vector to current coordinate system.
                    //
                    if fstr::eq(&save.CRDSYS, RECSYS) {
                        spicelib::MOVED(save.XPOS.as_slice(), 3, save.XCOORD.as_slice_mut());
                    } else if fstr::eq(&save.CRDSYS, LATSYS) {
                        let [arg1, arg2, arg3] = save.XCOORD.get_disjoint_mut([1, 2, 3]).expect(
                            "mutable array elements passed to function must have disjoint indexes",
                        );
                        spicelib::RECLAT(save.XPOS.as_slice(), arg1, arg2, arg3);
                    } else if fstr::eq(&save.CRDSYS, RADSYS) {
                        let [arg1, arg2, arg3] = save.XCOORD.get_disjoint_mut([1, 2, 3]).expect(
                            "mutable array elements passed to function must have disjoint indexes",
                        );
                        spicelib::RECRAD(save.XPOS.as_slice(), arg1, arg2, arg3, ctx);
                    } else if fstr::eq(&save.CRDSYS, SPHSYS) {
                        let [arg1, arg2, arg3] = save.XCOORD.get_disjoint_mut([1, 2, 3]).expect(
                            "mutable array elements passed to function must have disjoint indexes",
                        );
                        spicelib::RECSPH(save.XPOS.as_slice(), arg1, arg2, arg3);
                    } else if fstr::eq(&save.CRDSYS, CYLSYS) {
                        let [arg1, arg2, arg3] = save.XCOORD.get_disjoint_mut([1, 2, 3]).expect(
                            "mutable array elements passed to function must have disjoint indexes",
                        );
                        spicelib::RECCYL(save.XPOS.as_slice(), arg1, arg2, arg3, ctx);
                    } else if fstr::eq(&save.CRDSYS, GEOSYS) {
                        let [arg3, arg4, arg5] = save.XCOORD.get_disjoint_mut([1, 2, 3]).expect(
                            "mutable array elements passed to function must have disjoint indexes",
                        );
                        spicelib::RECGEO(
                            save.XPOS.as_slice(),
                            save.RE,
                            save.F,
                            arg3,
                            arg4,
                            arg5,
                            ctx,
                        )?;
                    } else if fstr::eq(&save.CRDSYS, PGRSYS) {
                        let [arg4, arg5, arg6] = save.XCOORD.get_disjoint_mut([1, 2, 3]).expect(
                            "mutable array elements passed to function must have disjoint indexes",
                        );
                        spicelib::RECPGR(
                            &save.CENTER,
                            save.XPOS.as_slice(),
                            save.RE,
                            save.F,
                            arg4,
                            arg5,
                            arg6,
                            ctx,
                        )?;
                    } else {
                        spicelib::SETMSG(b"Unrecognized system #.", ctx);
                        spicelib::ERRCH(b"#", &save.CRDSYS, ctx);
                        spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Get the expected state vector.
                    //
                    spicelib::ZZGFCOST(
                        &save.VECDEF,
                        &save.METHOD,
                        save.TRGID,
                        save.ET,
                        &save.REF,
                        &save.ABCORR,
                        save.OBSID,
                        &save.DREF,
                        save.DCTR,
                        save.DVEC.as_slice(),
                        save.RADII.as_slice(),
                        save.XSTATE.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                    //
                    // Find the expected coordinate rate.
                    //
                    if fstr::eq(save.SYSNMS.get(SI), RECSYS) {
                        spicelib::IDENT(save.JACOBI.as_slice_mut());
                    } else if fstr::eq(save.SYSNMS.get(SI), LATSYS) {
                        spicelib::DLATDR(
                            save.XSTATE[1],
                            save.XSTATE[2],
                            save.XSTATE[3],
                            save.JACOBI.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    } else if fstr::eq(save.SYSNMS.get(SI), RADSYS) {
                        spicelib::DLATDR(
                            save.XSTATE[1],
                            save.XSTATE[2],
                            save.XSTATE[3],
                            save.JACOBI.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    } else if fstr::eq(save.SYSNMS.get(SI), SPHSYS) {
                        spicelib::DSPHDR(
                            save.XSTATE[1],
                            save.XSTATE[2],
                            save.XSTATE[3],
                            save.JACOBI.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    } else if fstr::eq(save.SYSNMS.get(SI), CYLSYS) {
                        spicelib::DCYLDR(
                            save.XSTATE[1],
                            save.XSTATE[2],
                            save.XSTATE[3],
                            save.JACOBI.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    } else if fstr::eq(save.SYSNMS.get(SI), GEOSYS) {
                        spicelib::DGEODR(
                            save.XSTATE[1],
                            save.XSTATE[2],
                            save.XSTATE[3],
                            save.RE,
                            save.F,
                            save.JACOBI.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    } else if fstr::eq(save.SYSNMS.get(SI), PGRSYS) {
                        spicelib::DPGRDR(
                            &save.CENTER,
                            save.XSTATE[1],
                            save.XSTATE[2],
                            save.XSTATE[3],
                            save.RE,
                            save.F,
                            save.JACOBI.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    } else {
                        spicelib::SETMSG(b"Bad coordinate system #", ctx);
                        spicelib::ERRCH(b"#", &save.SYSNMS[SI], ctx);
                        spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Store the coordinate rates in XDCOR.
                    //
                    spicelib::MXV(
                        save.JACOBI.as_slice(),
                        save.XSTATE.subarray(4),
                        save.XDCOR.as_slice_mut(),
                    );

                    //
                    // Check each coordinate.
                    //
                    for CI in 1..=3 {
                        fstr::assign(&mut save.CRDNAM, save.CRDNMS.get([CI, SI]));
                        //
                        // Use the combination of ZZGFCOIN and ZZGFCOG
                        // to compute the coordinate.
                        //
                        save.REFVAL = 0.0;

                        spicelib::ZZGFCOIN(
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
                            ctx,
                        )?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // The coordinate state should always exist for
                        // the inputs we've selected.
                        //

                        spicelib::ZZGFCOEX(spicelib::UDF, &mut save.ET, &mut save.FOUND, ctx)?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                        //
                        // Fetch and check the coordinate from the ZZGFCOU
                        // package.
                        //

                        spicelib::ZZGFCOG(&mut save.ET, &mut save.VALUE, ctx)?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKSD(
                            &save.CRDNAM,
                            save.VALUE,
                            b"~/",
                            save.XCOORD[CI],
                            TIGHT,
                            OK,
                            ctx,
                        )?;

                        //
                        // Fetch and check the sine and cosine of the
                        // coordinate from the ZZGFCOU package.
                        //
                        //
                        // Check sine.
                        //
                        spicelib::ZZGFCOSG(&mut save.ET, &mut save.VALUE, ctx)?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        fstr::assign(&mut save.QNAME, b"sin(#)");
                        spicelib::REPMC(&save.QNAME.to_vec(), b"#", &save.CRDNAM, &mut save.QNAME);

                        testutil::CHCKSD(
                            &save.QNAME,
                            save.VALUE,
                            b"~/",
                            f64::sin(save.XCOORD[CI]),
                            TIGHT,
                            OK,
                            ctx,
                        )?;

                        //
                        // Check cosine.
                        //
                        spicelib::ZZGFCOCG(&mut save.ET, &mut save.VALUE, ctx)?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        fstr::assign(&mut save.QNAME, b"cos(#)");
                        spicelib::REPMC(&save.QNAME.to_vec(), b"#", &save.CRDNAM, &mut save.QNAME);

                        testutil::CHCKSD(
                            &save.QNAME,
                            save.VALUE,
                            b"~/",
                            f64::cos(save.XCOORD[CI]),
                            TIGHT,
                            OK,
                            ctx,
                        )?;

                        //
                        // Check the sign of the coordinate rate.
                        // Caution: altitudes of surface points are
                        // just noise, so don't check these.
                        //
                        if (fstr::eq(&save.VECDEF, POSDEF) || fstr::ne(&save.CRDNAM, ALTCRD)) {
                            spicelib::ZZGFCODC(spicelib::UDF, &mut save.ET, &mut save.DECRES, ctx)?;

                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            save.XDECRS = (save.XDCOR[CI] < 0.0);

                            fstr::assign(&mut save.QNAME, &save.CRDNAM);
                            spicelib::SUFFIX(b"is decreasing?", 1, &mut save.QNAME);

                            testutil::CHCKSL(&save.QNAME, save.DECRES, save.XDECRS, OK, ctx)?;

                            //
                            // Set case flags.
                            //
                            if save.DECRES {
                                save.CDEC = true;
                            } else {
                                save.CNDEC = true;
                            }

                            //
                            // For angular coordinates, fetch and check the
                            // signs of the rates of the sine and cosine of
                            // the coordinate from the ZZGFCOU package.
                            //
                            if (((((((fstr::eq(&save.CRDNAM, LATCRD)
                                || fstr::eq(&save.CRDNAM, LONCRD))
                                || fstr::eq(&save.CRDNAM, RACRD))
                                || fstr::eq(&save.CRDNAM, XCRD))
                                || fstr::eq(&save.CRDNAM, YCRD))
                                || fstr::eq(&save.CRDNAM, ZCRD))
                                || fstr::eq(&save.CRDNAM, DECCRD))
                                || fstr::eq(&save.CRDNAM, CLTCRD))
                            {
                                //
                                // Check sign of sine rate.
                                //
                                save.XDECRS = ((f64::cos(save.XCOORD[CI]) * save.XDCOR[CI]) < 0.0);

                                spicelib::ZZGFCOSD(
                                    spicelib::UDF,
                                    &mut save.ET,
                                    &mut save.DECRES,
                                    ctx,
                                )?;

                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                //
                                // Set case flags.
                                //
                                if save.DECRES {
                                    save.CSDEC = true;
                                } else {
                                    save.CSNDEC = true;
                                }

                                fstr::assign(&mut save.QNAME, b"sin(#) is decreasing?");
                                spicelib::REPMC(
                                    &save.QNAME.to_vec(),
                                    b"#",
                                    &save.CRDNAM,
                                    &mut save.QNAME,
                                );

                                testutil::CHCKSL(&save.QNAME, save.DECRES, save.XDECRS, OK, ctx)?;

                                //
                                // Check sign of cosine rate.
                                //
                                save.XDECRS = (-(f64::sin(save.XCOORD[CI]) * save.XDCOR[CI]) < 0.0);

                                spicelib::ZZGFCOCD(
                                    spicelib::UDF,
                                    &mut save.ET,
                                    &mut save.DECRES,
                                    ctx,
                                )?;

                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                if save.DECRES {
                                    save.CCDEC = true;
                                } else {
                                    save.CCNDEC = true;
                                }

                                fstr::assign(&mut save.QNAME, b"cos(#) is decreasing?");
                                spicelib::REPMC(
                                    &save.QNAME.to_vec(),
                                    b"#",
                                    &save.CRDNAM,
                                    &mut save.QNAME,
                                );

                                testutil::CHCKSL(&save.QNAME, save.DECRES, save.XDECRS, OK, ctx)?;
                            }
                            //
                            // End of angular coordinate check block.
                            //

                            //
                            // Check the "is less than?" functions and
                            // the reference value reset function.
                            //
                            // All values are less than DPMAX(), we hope.
                            //
                            save.REFVAL = spicelib::DPMAX();
                            spicelib::ZZGFREF(save.REFVAL, ctx)?;

                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            spicelib::ZZGFUDLT(
                                spicelib::ZZGFCOG,
                                &mut save.ET,
                                &mut save.LSSTHN,
                                ctx,
                            )?;

                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            testutil::CHCKSL(&save.CRDNAM, save.LSSTHN, true, OK, ctx)?;

                            spicelib::ZZGFUDLT(
                                spicelib::ZZGFCOSG,
                                &mut save.ET,
                                &mut save.LSSTHN,
                                ctx,
                            )?;

                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            fstr::assign(&mut save.QNAME, b"sin(#) is less than reference?");
                            spicelib::REPMC(
                                &save.QNAME.to_vec(),
                                b"#",
                                &save.CRDNAM,
                                &mut save.QNAME,
                            );
                            testutil::CHCKSL(&save.QNAME, save.LSSTHN, true, OK, ctx)?;

                            spicelib::ZZGFUDLT(
                                spicelib::ZZGFCOCG,
                                &mut save.ET,
                                &mut save.LSSTHN,
                                ctx,
                            )?;

                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            fstr::assign(&mut save.QNAME, b"cos(#) is less than reference?");
                            spicelib::REPMC(
                                &save.QNAME.to_vec(),
                                b"#",
                                &save.CRDNAM,
                                &mut save.QNAME,
                            );
                            testutil::CHCKSL(&save.QNAME, save.LSSTHN, true, OK, ctx)?;

                            //
                            // Now make sure the quantities are not less than
                            // DPMIN().
                            //
                            save.REFVAL = spicelib::DPMIN();
                            spicelib::ZZGFREF(save.REFVAL, ctx)?;

                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            spicelib::ZZGFUDLT(
                                spicelib::ZZGFCOG,
                                &mut save.ET,
                                &mut save.LSSTHN,
                                ctx,
                            )?;

                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            testutil::CHCKSL(&save.CRDNAM, save.LSSTHN, false, OK, ctx)?;

                            spicelib::ZZGFUDLT(
                                spicelib::ZZGFCOSG,
                                &mut save.ET,
                                &mut save.LSSTHN,
                                ctx,
                            )?;

                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            fstr::assign(&mut save.QNAME, b"sin(#) is less than reference?");
                            spicelib::REPMC(
                                &save.QNAME.to_vec(),
                                b"#",
                                &save.CRDNAM,
                                &mut save.QNAME,
                            );
                            testutil::CHCKSL(&save.QNAME, save.LSSTHN, false, OK, ctx)?;

                            spicelib::ZZGFUDLT(
                                spicelib::ZZGFCOCG,
                                &mut save.ET,
                                &mut save.LSSTHN,
                                ctx,
                            )?;

                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            fstr::assign(&mut save.QNAME, b"cos(#) is less than reference?");
                            spicelib::REPMC(
                                &save.QNAME.to_vec(),
                                b"#",
                                &save.CRDNAM,
                                &mut save.QNAME,
                            );
                            testutil::CHCKSL(&save.QNAME, save.LSSTHN, false, OK, ctx)?;
                        }
                        //
                        // End of "non-noise" block.
                        //
                    }
                    //
                    // End of coordinate loop.
                    //

                    if fstr::eq(&save.VECDEF, SINDEF) {
                        //
                        // --- Case: ------------------------------------------------------
                        //

                        // Given the inputs above, make sure we do not find
                        // the state if we reverse the ray's direction.'
                        //
                        spicelib::PREFIX(b"Earth non-intersection case:", 1, &mut save.TITLE);
                        testutil::TCASE(&save.TITLE, ctx)?;

                        spicelib::VMINUS(save.DVEC.as_slice(), save.NEGVEC.as_slice_mut());
                        spicelib::VEQU(save.NEGVEC.as_slice(), save.DVEC.as_slice_mut());

                        for CI in 1..=3 {
                            fstr::assign(&mut save.CRDNAM, save.CRDNMS.get([CI, SI]));
                            //
                            // Use the combination of ZZGFCOIN and ZZGFCOG
                            // to compute the coordinate.
                            //
                            save.REFVAL = 0.0;

                            spicelib::ZZGFCOIN(
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
                                ctx,
                            )?;

                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            spicelib::ZZGFCOEX(spicelib::UDF, &mut save.ET, &mut save.FOUND, ctx)?;

                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // By construction, the state should NOT be found.
                            //
                            testutil::CHCKSL(b"ZZGFCOEX found", save.FOUND, false, OK, ctx)?;
                        }
                    }
                }
                //
                // End of cases for the current epoch.
                //
            }
            //
            // End of cases for the current coordinate system.
            //
        }
        //
        // End of cases for the current aberration correction.
        //
    }
    //
    // End of cases for the current vector definition.
    //

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Check flags indicating whether "increasing" and
    // "not increasing" cases were hit.
    //

    testutil::TCASE(b"Check decreasing quantity coverage", ctx)?;

    testutil::CHCKSL(b"CDEC", save.CDEC, true, OK, ctx)?;
    testutil::CHCKSL(b"CNDEC", save.CNDEC, true, OK, ctx)?;
    testutil::CHCKSL(b"CCDEC", save.CCDEC, true, OK, ctx)?;
    testutil::CHCKSL(b"CCNDEC", save.CCNDEC, true, OK, ctx)?;
    testutil::CHCKSL(b"CSDEC", save.CSDEC, true, OK, ctx)?;
    testutil::CHCKSL(b"CSNDEC", save.CSNDEC, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
