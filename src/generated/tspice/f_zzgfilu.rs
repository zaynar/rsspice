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
const PCK: &[u8] = b"nat.tpc";
const PCK2: &[u8] = b"generic.tpc";
const OCTSPK: &[u8] = b"octl_test.bsp";
const OCTL: &[u8] = b"OCTL";
const SPK1: &[u8] = b"generic.bsp";
const ANGTOL: f64 = 0.0000000000001;
const TIMLEN: i32 = 50;
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const LNSIZE: i32 = 80;
const NCASE: i32 = 100;
const TYPLEN: i32 = 25;
const TXTSIZ: i32 = 25;

//$Procedure      F_ZZGFILU ( ZZGFILU family tests )
pub fn F_ZZGFILU(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ABCORR = [b' '; CORLEN as usize];
    let mut ANGTYP = [b' '; TYPLEN as usize];
    let mut FIXREF = [b' '; FRNMLN as usize];
    let mut ILLUM = [b' '; BDNMLN as usize];
    let mut METHOD = [b' '; LNSIZE as usize];
    let mut NAME = [b' '; LNSIZE as usize];
    let mut OBSRVR = [b' '; BDNMLN as usize];
    let mut TARGET = [b' '; BDNMLN as usize];
    let mut TIME0 = [b' '; TIMLEN as usize];
    let mut TXT = ActualCharArray::new(LNSIZE, 1..=TXTSIZ);
    let mut ANGLE: f64 = 0.0;
    let mut EMISSN: f64 = 0.0;
    let mut EMISTA = StackArray::<f64, 2>::new(1..=2);
    let mut ET: f64 = 0.0;
    let mut ET0: f64 = 0.0;
    let mut FIRST: f64 = 0.0;
    let mut INCDNC: f64 = 0.0;
    let mut INCSTA = StackArray::<f64, 2>::new(1..=2);
    let mut LAT: f64 = 0.0;
    let mut LAST: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut PHASE: f64 = 0.0;
    let mut PHSSTA = StackArray::<f64, 2>::new(1..=2);
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
    let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
    let mut STATES = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut STEP: f64 = 0.0;
    let mut TRGEPC: f64 = 0.0;
    let mut HANDLE: i32 = 0;
    let mut HAN1: i32 = 0;
    let mut N: i32 = 0;
    let mut OCTID: i32 = 0;
    let mut FOUND: bool = false;
    let mut ISDEC: bool = false;
    let mut XDEC: bool = false;

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
    //---- Case -------------------------------------------------------------
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFILU", ctx)?;

    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    testutil::TSTSPK(SPK1, true, &mut HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATPCK(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTPCK(PCK2, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up a confinement window. Initialize this and
    // the result window.
    //
    fstr::assign(&mut TIME0, b"2000 JAN 1  00:00:00 TDB");

    spicelib::STR2ET(&TIME0, &mut ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
    fstr::assign(&mut FIXREF, b"IAU_EARTH");

    //
    // Prepare a frame kernel in a string array.
    //
    fstr::assign(TXT.get_mut(1), b"FRAME_OCTL_TOPO            =  1398962");
    fstr::assign(
        TXT.get_mut(2),
        b"FRAME_1398962_NAME         =  \'OCTL_TOPO\' ",
    );
    fstr::assign(TXT.get_mut(3), b"FRAME_1398962_CLASS        =  4");
    fstr::assign(TXT.get_mut(4), b"FRAME_1398962_CLASS_ID     =  1398962");
    fstr::assign(TXT.get_mut(5), b"FRAME_1398962_CENTER       =  398962");

    fstr::assign(
        TXT.get_mut(6),
        b"OBJECT_398962_FRAME        =  \'OCTL_TOPO\' ",
    );

    fstr::assign(
        TXT.get_mut(7),
        b"TKFRAME_1398962_RELATIVE   =  \'IAU_EARTH\' ",
    );
    fstr::assign(TXT.get_mut(8), b"TKFRAME_1398962_SPEC       =  \'ANGLES\' ");
    fstr::assign(
        TXT.get_mut(9),
        b"TKFRAME_1398962_UNITS      =  \'DEGREES\' ",
    );
    fstr::assign(
        TXT.get_mut(10),
        b"TKFRAME_1398962_AXES       =  ( 3, 2, 3 )",
    );
    fstr::assign(
        TXT.get_mut(11),
        b"TKFRAME_1398962_ANGLES     =  ( -242.3171620000000,",
    );
    fstr::assign(
        TXT.get_mut(12),
        b"                                 -55.6182509000000,",
    );
    fstr::assign(
        TXT.get_mut(13),
        b"                                 180.0000000000000  )",
    );
    fstr::assign(TXT.get_mut(14), b"NAIF_BODY_NAME            +=  \'OCTL\' ");
    fstr::assign(TXT.get_mut(15), b"NAIF_BODY_CODE            +=  398962");

    //
    // It will be convenient to have a version of this frame that
    // has the +Z axis pointed down instead of up.
    //
    fstr::assign(TXT.get_mut(16), b"FRAME_OCTL_FLIP            =  2398962");
    fstr::assign(
        TXT.get_mut(17),
        b"FRAME_2398962_NAME         =  \'OCTL_FLIP\' ",
    );
    fstr::assign(TXT.get_mut(18), b"FRAME_2398962_CLASS        =  4");
    fstr::assign(TXT.get_mut(19), b"FRAME_2398962_CLASS_ID     =  2398962");
    fstr::assign(TXT.get_mut(20), b"FRAME_2398962_CENTER       =  398962");

    fstr::assign(
        TXT.get_mut(21),
        b"TKFRAME_2398962_RELATIVE   =  \'OCTL_TOPO\' ",
    );
    fstr::assign(
        TXT.get_mut(22),
        b"TKFRAME_2398962_SPEC       =  \'ANGLES\' ",
    );
    fstr::assign(
        TXT.get_mut(23),
        b"TKFRAME_2398962_UNITS      =  \'DEGREES\' ",
    );
    fstr::assign(
        TXT.get_mut(24),
        b"TKFRAME_2398962_AXES       =  ( 3, 2, 3 )",
    );
    fstr::assign(
        TXT.get_mut(25),
        b"TKFRAME_2398962_ANGLES     =  ( 0, 180.0, 0 ) ",
    );

    spicelib::LMPOOL(TXT.as_arg(), TXTSIZ, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now create an SPK file containing a type 8 segment for OCTL.
    //
    spicelib::SPKOPN(OCTSPK, OCTSPK, 0, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize both states to zero.
    //
    spicelib::CLEARD(12, STATES.as_slice_mut());

    //
    // The first position component:
    //
    SPOINT[1] = -2448.937761729;
    SPOINT[2] = -4667.935793438;
    SPOINT[3] = 3582.74849943;

    spicelib::VEQU(SPOINT.as_slice(), STATES.subarray_mut([1, 1]));

    //
    // The second position matches the first: we don't model
    // plate motion.
    //
    spicelib::VEQU(SPOINT.as_slice(), STATES.subarray_mut([1, 2]));

    //
    // Time bounds for the segment:
    //

    FIRST = (((-50 as f64) * spicelib::SPD()) * 365.25);
    STEP = (((100 as f64) * spicelib::SPD()) * 365.25);

    LAST = ((FIRST + STEP) - 0.000001);

    //
    // Get the OCTL ID code from the kernel we just
    // loaded.
    //
    spicelib::BODN2C(OCTL, &mut OCTID, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Write the segment.
    //
    spicelib::SPKW08(
        HANDLE,
        OCTID,
        399,
        &FIXREF,
        FIRST,
        LAST,
        b"octl",
        1,
        2,
        STATES.as_slice(),
        FIRST,
        STEP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now load the OCTL SPK file.
    //
    spicelib::FURNSH(OCTSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //*********************************************************************
    //*
    //*    Error cases for ZZGFILIN
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Unrecognized value of FIXREF.", ctx)?;

    fstr::assign(&mut METHOD, b"Ellipsoid");
    fstr::assign(&mut ANGTYP, b"INCIDENCE");
    fstr::assign(&mut TARGET, b"EARTH");
    fstr::assign(&mut ILLUM, b"SUN");
    fstr::assign(&mut FIXREF, b"IAU_EARTH");
    fstr::assign(&mut ABCORR, b"CN+S");
    fstr::assign(&mut OBSRVR, b"MOON");

    LON = (100.0 * spicelib::RPD(ctx));
    LAT = (30.0 * spicelib::RPD(ctx));

    spicelib::SRFREC(399, LON, LAT, SPOINT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        b"XYZ",
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNFRAME)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"FIXREF is not centered on the target body.", ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        b"IAU_MARS",
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad computation method.", ctx)?;

    fstr::assign(&mut ANGTYP, b"EMISSION");

    fstr::assign(&mut METHOD, b"DSK");

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDMETHOD)", OK, ctx)?;

    fstr::assign(&mut METHOD, b"ELLIPSOID");

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad illumination angle type", ctx)?;

    fstr::assign(&mut ANGTYP, b"XYZ");

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad aberration correction values", ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        b"S",
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        b"XS",
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        b"RLT",
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        b"XRLT",
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        b"Z",
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Transmission aberration correction values.", ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        b"XLT",
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        b"XCN",
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        b"XLT+S",
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        b"XCN+S",
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Unrecognized target, observer, or illumination source.",
        ctx,
    )?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        b"X",
        &ILLUM,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        b"X",
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        &ABCORR,
        b"X",
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Target and observer are identical.", ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        &ABCORR,
        &TARGET,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Target and illumination source are identical.", ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &TARGET,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases for ZZGFILDC
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFILDC: No SPK data for observer", ctx)?;

    fstr::assign(&mut ANGTYP, b"PHASE");

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        &ABCORR,
        b"GASPRA",
        SPOINT.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFILDC(spicelib::ZZGFILGQ, &mut 0.0.clone(), &mut ISDEC, ctx)?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFILDC: No SPK data for target", ctx)?;

    fstr::assign(&mut FIXREF, b"IAU_GASPRA");

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        b"GASPRA",
        &ILLUM,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFILDC(spicelib::ZZGFILGQ, &mut 0.0.clone(), &mut ISDEC, ctx)?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFILDC: No SPK data for illumination source", ctx)?;

    fstr::assign(&mut FIXREF, b"IAU_EARTH");

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        b"GASPRA",
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFILDC(spicelib::ZZGFILGQ, &mut 0.0.clone(), &mut ISDEC, ctx)?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGILDC: No PCK orientation data for target.", ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        b"ITRF93",
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFILDC(spicelib::ZZGFILGQ, &mut 0.0.clone(), &mut ISDEC, ctx)?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases for ZZGFILGQ
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFILGQ: No SPK data for observer", ctx)?;

    fstr::assign(&mut ANGTYP, b"PHASE");

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        &ABCORR,
        b"GASPRA",
        SPOINT.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFILGQ(&mut 0.0.clone(), &mut ANGLE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFILGQ: No SPK data for target", ctx)?;

    fstr::assign(&mut FIXREF, b"IAU_GASPRA");

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        b"GASPRA",
        &ILLUM,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFILGQ(&mut 0.0.clone(), &mut ANGLE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFILGQ: No SPK data for illumination source", ctx)?;

    fstr::assign(&mut FIXREF, b"IAU_EARTH");

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        b"GASPRA",
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFILGQ(&mut 0.0.clone(), &mut ANGLE, ctx)?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGILGQ: No PCK orientation data for target.", ctx)?;

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        b"ITRF93",
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFILGQ(&mut 0.0.clone(), &mut ANGLE, ctx)?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZGFILDC: check slope of phase angle angles for a list of times.",
        ctx,
    )?;

    fstr::assign(&mut METHOD, b"Ellipsoid");
    fstr::assign(&mut TARGET, b"EARTH");
    fstr::assign(&mut ILLUM, b"SUN");
    fstr::assign(&mut FIXREF, b"IAU_EARTH");
    fstr::assign(&mut ABCORR, b"CN+S");
    fstr::assign(&mut OBSRVR, b"MOON");

    spicelib::SPKPOS(
        b"OCTL",
        0.0,
        &FIXREF,
        b"NONE",
        &TARGET,
        SPOINT.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Phase angle:
    //
    fstr::assign(&mut ANGTYP, b"PHASE");

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the normal vector required by ZZILUSTA:
    //
    spicelib::BODVRD(&TARGET, b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SURFNM(
        RADII[1],
        RADII[2],
        RADII[3],
        SPOINT.as_slice(),
        NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    ET0 = 0.0;

    for I in 1..=NCASE {
        ET = (ET0 + (((I - 1) as f64) * spicelib::SPD()));

        spicelib::ZZGFILDC(spicelib::ZZGFILGQ, &mut ET, &mut ISDEC, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ZZILUSTA(
            &METHOD,
            &TARGET,
            &ILLUM,
            ET,
            &FIXREF,
            &ABCORR,
            &OBSRVR,
            SPOINT.as_slice(),
            NORMAL.as_slice(),
            PHSSTA.as_slice_mut(),
            INCSTA.as_slice_mut(),
            EMISTA.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        XDEC = (PHSSTA[2] < 0.0);
        //
        // Maintenance note: verify that both true and false
        // values of XDEC are generated by this test case.
        //
        // WRITE (*,*) XDEC

        fstr::assign(&mut NAME, b"Phase angle decreasing #*");
        spicelib::REPMI(&NAME.clone(), b"*", I, &mut NAME, ctx);

        testutil::CHCKSL(&NAME, ISDEC, XDEC, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZGFILDC: check slope of emission angle angles for a list of times.",
        ctx,
    )?;

    //
    // Emission angle:
    //
    fstr::assign(&mut ANGTYP, b"EMISSION");

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the normal vector required by ZZILUSTA:
    //
    spicelib::BODVRD(&TARGET, b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SURFNM(
        RADII[1],
        RADII[2],
        RADII[3],
        SPOINT.as_slice(),
        NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    ET0 = 0.0;

    for I in 1..=NCASE {
        ET = (ET0 + (((I - 1) as f64) * spicelib::SPD()));

        spicelib::ZZGFILDC(spicelib::ZZGFILGQ, &mut ET, &mut ISDEC, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ZZILUSTA(
            &METHOD,
            &TARGET,
            &ILLUM,
            ET,
            &FIXREF,
            &ABCORR,
            &OBSRVR,
            SPOINT.as_slice(),
            NORMAL.as_slice(),
            PHSSTA.as_slice_mut(),
            INCSTA.as_slice_mut(),
            EMISTA.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        XDEC = (EMISTA[2] < 0.0);
        //
        // Maintenance note: verify that both true and false
        // values of XDEC are generated by this test case.
        //
        // WRITE (*,*) XDEC

        fstr::assign(&mut NAME, b"Emission angle decreasing #*");
        spicelib::REPMI(&NAME.clone(), b"*", I, &mut NAME, ctx);

        testutil::CHCKSL(&NAME, ISDEC, XDEC, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZGFILDC: check slope of incidence angle angles for a list of times.",
        ctx,
    )?;

    //
    // Emission angle:
    //
    fstr::assign(&mut ANGTYP, b"INCIDENCE");

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the normal vector required by ZZILUSTA:
    //
    spicelib::BODVRD(&TARGET, b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SURFNM(
        RADII[1],
        RADII[2],
        RADII[3],
        SPOINT.as_slice(),
        NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    ET0 = 0.0;

    for I in 1..=NCASE {
        ET = (ET0 + (((I - 1) as f64) * spicelib::SPD()));

        spicelib::ZZGFILDC(spicelib::ZZGFILGQ, &mut ET, &mut ISDEC, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ZZILUSTA(
            &METHOD,
            &TARGET,
            &ILLUM,
            ET,
            &FIXREF,
            &ABCORR,
            &OBSRVR,
            SPOINT.as_slice(),
            NORMAL.as_slice(),
            PHSSTA.as_slice_mut(),
            INCSTA.as_slice_mut(),
            EMISTA.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        XDEC = (INCSTA[2] < 0.0);
        //
        // Maintenance note: verify that both true and false
        // values of XDEC are generated by this test case.
        //
        // WRITE (*,*) XDEC

        fstr::assign(&mut NAME, b"Incidence angle decreasing #*");
        spicelib::REPMI(&NAME.clone(), b"*", I, &mut NAME, ctx);

        testutil::CHCKSL(&NAME, ISDEC, XDEC, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZGFILGQ: get illumination incidence angles for a list of times.",
        ctx,
    )?;

    fstr::assign(&mut METHOD, b"Ellipsoid");
    fstr::assign(&mut TARGET, b"EARTH");
    fstr::assign(&mut ILLUM, b"SUN");
    fstr::assign(&mut FIXREF, b"IAU_EARTH");
    fstr::assign(&mut ABCORR, b"CN+S");
    fstr::assign(&mut OBSRVR, b"MOON");

    spicelib::SPKPOS(
        b"OCTL",
        0.0,
        &FIXREF,
        b"NONE",
        &TARGET,
        SPOINT.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Illumination incidence angle:
    //
    fstr::assign(&mut ANGTYP, b"INCIDENCE");

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    ET0 = 0.0;

    for I in 1..=NCASE {
        ET = (ET0 + (((I - 1) as f64) * spicelib::SPD()));

        spicelib::ZZGFILGQ(&mut ET, &mut ANGLE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ILLUMG(
            &METHOD,
            &TARGET,
            &ILLUM,
            ET,
            &FIXREF,
            &ABCORR,
            &OBSRVR,
            SPOINT.as_slice(),
            &mut TRGEPC,
            SRFVEC.as_slice_mut(),
            &mut PHASE,
            &mut INCDNC,
            &mut EMISSN,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut NAME, b"Illum incidence (*)");
        spicelib::REPMI(&NAME.clone(), b"*", I, &mut NAME, ctx);

        testutil::CHCKSD(&NAME, ANGLE, b"~/", INCDNC, ANGTOL, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFILGQ: get emission angles for a list of times.", ctx)?;

    //
    // Illumination incidence angle:
    //
    fstr::assign(&mut ANGTYP, b"EMISSION");

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    ET0 = 0.0;

    for I in 1..=NCASE {
        ET = (ET0 + (((I - 1) as f64) * spicelib::SPD()));

        spicelib::ZZGFILGQ(&mut ET, &mut ANGLE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ILLUMG(
            &METHOD,
            &TARGET,
            &ILLUM,
            ET,
            &FIXREF,
            &ABCORR,
            &OBSRVR,
            SPOINT.as_slice(),
            &mut TRGEPC,
            SRFVEC.as_slice_mut(),
            &mut PHASE,
            &mut INCDNC,
            &mut EMISSN,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut NAME, b"Emission angle #*");
        spicelib::REPMI(&NAME.clone(), b"*", I, &mut NAME, ctx);

        testutil::CHCKSD(&NAME, ANGLE, b"~/", EMISSN, ANGTOL, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"ZZGFILGQ: get phase angles for a list of times.", ctx)?;

    //
    // Phase angle:
    //
    fstr::assign(&mut ANGTYP, b"PHASE");

    spicelib::ZZGFILIN(
        &METHOD,
        &ANGTYP,
        &TARGET,
        &ILLUM,
        &FIXREF,
        &ABCORR,
        &OBSRVR,
        SPOINT.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    ET0 = 0.0;

    for I in 1..=NCASE {
        ET = (ET0 + (((I - 1) as f64) * spicelib::SPD()));

        spicelib::ZZGFILGQ(&mut ET, &mut ANGLE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ILLUMG(
            &METHOD,
            &TARGET,
            &ILLUM,
            ET,
            &FIXREF,
            &ABCORR,
            &OBSRVR,
            SPOINT.as_slice(),
            &mut TRGEPC,
            SRFVEC.as_slice_mut(),
            &mut PHASE,
            &mut INCDNC,
            &mut EMISSN,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut NAME, b"Phase angle #*");
        spicelib::REPMI(&NAME.clone(), b"*", I, &mut NAME, ctx);

        testutil::CHCKSD(&NAME, ANGLE, b"~/", PHASE, ANGTOL, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Clean up. Unload and delete kernels.", ctx)?;

    //
    // Clean up SPK files.
    //
    spicelib::SPKUEF(HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(OCTSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(OCTSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
