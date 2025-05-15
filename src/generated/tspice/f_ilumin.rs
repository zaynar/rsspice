//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXSRF: i32 = 100;
const DSKSHP: i32 = 2;
const ELLSHP: i32 = 1;
const MTHLEN: i32 = 500;
const SUBLEN: i32 = 20;
const CVTLEN: i32 = 20;
const TANGNT: i32 = 1;
const GUIDED: i32 = 2;
const TMTLEN: i32 = 20;
const LMBCRV: i32 = 0;
const UMBRAL: i32 = 1;
const PNMBRL: i32 = 2;
const ACLLEN: i32 = 25;
const CTRCOR: i32 = 1;
const ELLCOR: i32 = 2;
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const DSK0: &[u8] = b"ilumin_dsk0.bds";
const DSK1: &[u8] = b"ilumin_dsk1.bds";
const DSK2: &[u8] = b"ilumin_dsk2.bds";
const DSK3: &[u8] = b"ilumin_dsk3.bds";
const PCK: &[u8] = b"test_0008.tpc";
const SPK1: &[u8] = b"ilumin_spk.bsp";
const SPK2: &[u8] = b"orbiter.bsp";
const VTIGHT: f64 = 0.00000000000001;
const TIGHT: f64 = 0.000000000001;
const MTIGHT: f64 = 0.0000000001;
const LOOSE: f64 = 0.000005;
const LNSIZE: i32 = 240;
const NAMLEN: i32 = 32;
const NREF: i32 = 1;
const NABC: i32 = 5;
const ABCLEN: i32 = 10;
const TIMLEN: i32 = 50;
const NOBS: i32 = 2;
const NTARG: i32 = 2;
const SCID: i32 = -499;
const MAXPTS: i32 = 100;
const NMAP: i32 = 4;
const NMETH: i32 = 4;

struct SaveVars {
    ABCS: ActualCharArray,
    ABCORR: Vec<u8>,
    FIXREF: Vec<u8>,
    KVNAME: Vec<u8>,
    METHOD: Vec<u8>,
    METHDS: ActualCharArray,
    OBSRVR: Vec<u8>,
    OBSNMS: ActualCharArray,
    REFS: ActualCharArray2D,
    SRFNMS: ActualCharArray,
    TARGET: Vec<u8>,
    TRGFRM: Vec<u8>,
    TRGNMS: ActualCharArray,
    TITLE: Vec<u8>,
    UTC: Vec<u8>,
    ALTRAD: StackArray<f64, 3>,
    BADRAD: StackArray<f64, 3>,
    DLAT: f64,
    DLON: f64,
    ELTS: StackArray<f64, 8>,
    EMISSN: f64,
    ET: f64,
    ET0: f64,
    INCDNC: f64,
    LAT: f64,
    LON: f64,
    LONLAT: StackArray2D<f64, 200>,
    LT: f64,
    NORMAL: StackArray<f64, 3>,
    OBSPOS: StackArray<f64, 3>,
    PHASE: f64,
    RADII: StackArray<f64, 3>,
    SPOINT: StackArray<f64, 3>,
    SRFVEC: StackArray<f64, 3>,
    STATE: StackArray<f64, 6>,
    STATE0: StackArray<f64, 6>,
    SUNLT: f64,
    SUNSTA: StackArray<f64, 6>,
    TOL: f64,
    TRGEPC: f64,
    XEMISN: f64,
    XEPOCH: f64,
    XINCDC: f64,
    XPHASE: f64,
    XSRFVC: StackArray<f64, 3>,
    XTE: f64,
    BODYID: i32,
    HANDLE: StackArray<i32, 2>,
    K: i32,
    N: i32,
    NLAT: i32,
    NLON: i32,
    NPTS: i32,
    NSFLAT: i32,
    NSFLON: i32,
    OBSCDE: i32,
    SRFBOD: StackArray<i32, 4>,
    SRFIDS: StackArray<i32, 4>,
    SURFID: i32,
    TRGCDE: i32,
    FOUND: bool,
    USECN: bool,
    USELT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCS = ActualCharArray::new(ABCLEN, 1..=NABC);
        let mut ABCORR = vec![b' '; ABCLEN as usize];
        let mut FIXREF = vec![b' '; NAMLEN as usize];
        let mut KVNAME = vec![b' '; NAMLEN as usize];
        let mut METHOD = vec![b' '; MTHLEN as usize];
        let mut METHDS = ActualCharArray::new(MTHLEN, 1..=NMETH);
        let mut OBSRVR = vec![b' '; NAMLEN as usize];
        let mut OBSNMS = ActualCharArray::new(NAMLEN, 1..=NOBS);
        let mut REFS = ActualCharArray2D::new(NAMLEN, 1..=NREF, 1..=NTARG);
        let mut SRFNMS = ActualCharArray::new(NAMLEN, 1..=NMAP);
        let mut TARGET = vec![b' '; NAMLEN as usize];
        let mut TRGFRM = vec![b' '; NAMLEN as usize];
        let mut TRGNMS = ActualCharArray::new(NAMLEN, 1..=NTARG);
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut UTC = vec![b' '; TIMLEN as usize];
        let mut ALTRAD = StackArray::<f64, 3>::new(1..=3);
        let mut BADRAD = StackArray::<f64, 3>::new(1..=3);
        let mut DLAT: f64 = 0.0;
        let mut DLON: f64 = 0.0;
        let mut ELTS = StackArray::<f64, 8>::new(1..=8);
        let mut EMISSN: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut INCDNC: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut LONLAT = StackArray2D::<f64, 200>::new(1..=2, 1..=MAXPTS);
        let mut LT: f64 = 0.0;
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut OBSPOS = StackArray::<f64, 3>::new(1..=3);
        let mut PHASE: f64 = 0.0;
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
        let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut STATE0 = StackArray::<f64, 6>::new(1..=6);
        let mut SUNLT: f64 = 0.0;
        let mut SUNSTA = StackArray::<f64, 6>::new(1..=6);
        let mut TOL: f64 = 0.0;
        let mut TRGEPC: f64 = 0.0;
        let mut XEMISN: f64 = 0.0;
        let mut XEPOCH: f64 = 0.0;
        let mut XINCDC: f64 = 0.0;
        let mut XPHASE: f64 = 0.0;
        let mut XSRFVC = StackArray::<f64, 3>::new(1..=3);
        let mut XTE: f64 = 0.0;
        let mut BODYID: i32 = 0;
        let mut HANDLE = StackArray::<i32, 2>::new(1..=2);
        let mut K: i32 = 0;
        let mut N: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NPTS: i32 = 0;
        let mut NSFLAT: i32 = 0;
        let mut NSFLON: i32 = 0;
        let mut OBSCDE: i32 = 0;
        let mut SRFBOD = StackArray::<i32, 4>::new(1..=NMAP);
        let mut SRFIDS = StackArray::<i32, 4>::new(1..=NMAP);
        let mut SURFID: i32 = 0;
        let mut TRGCDE: i32 = 0;
        let mut FOUND: bool = false;
        let mut USECN: bool = false;
        let mut USELT: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"None"),
                Val::C(b"Lt"),
                Val::C(b"Lt+s"),
                Val::C(b"XCn"),
                Val::C(b"Cn+s"),
            ]
            .into_iter();
            ABCS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"IAU_MARS"), Val::C(b"IAU_PHOBOS")].into_iter();
            REFS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"Earth"), Val::C(b"MARS_ORBITER")].into_iter();
            OBSNMS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"Mars"), Val::C(b"PHOBOS")].into_iter();
            TRGNMS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"ELLIPSOID"),
                Val::C(b"dsk/unprioritized/surfaces=\"high-res\""),
                Val::C(b"UNPRIORITIZED/ dsk /SURFACES =\"LOW-RES\""),
                Val::C(b"UNPRIORITIZED/ dsk /SURFACES =\"LOW-RES\""),
            ]
            .into_iter();
            METHDS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCS,
            ABCORR,
            FIXREF,
            KVNAME,
            METHOD,
            METHDS,
            OBSRVR,
            OBSNMS,
            REFS,
            SRFNMS,
            TARGET,
            TRGFRM,
            TRGNMS,
            TITLE,
            UTC,
            ALTRAD,
            BADRAD,
            DLAT,
            DLON,
            ELTS,
            EMISSN,
            ET,
            ET0,
            INCDNC,
            LAT,
            LON,
            LONLAT,
            LT,
            NORMAL,
            OBSPOS,
            PHASE,
            RADII,
            SPOINT,
            SRFVEC,
            STATE,
            STATE0,
            SUNLT,
            SUNSTA,
            TOL,
            TRGEPC,
            XEMISN,
            XEPOCH,
            XINCDC,
            XPHASE,
            XSRFVC,
            XTE,
            BODYID,
            HANDLE,
            K,
            N,
            NLAT,
            NLON,
            NPTS,
            NSFLAT,
            NSFLON,
            OBSCDE,
            SRFBOD,
            SRFIDS,
            SURFID,
            TRGCDE,
            FOUND,
            USECN,
            USELT,
        }
    }
}

//$Procedure      F_ILUMIN ( ILUMIN family tests )
pub fn F_ILUMIN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // REFS is a two-dimensional array. There's a set of
    // ray reference  frames for each target. Currently
    // there are only two targets: Mars and Phobos.
    //

    //
    // Note that the last two method strings are identical. This
    // is done to test the logic that uses saved values obtained
    // by parsing method string.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ILUMIN", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup:  create SPK, PCK file.", ctx)?;

    testutil::TSTSPK(SPK1, true, &mut save.HANDLE[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the PCK file, and load it. Do not delete it.
    //
    testutil::T_PCK08(PCK, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create LSK, load it, and delete it.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set initial time.
    //
    fstr::assign(&mut save.UTC, b"2004 FEB 17");
    spicelib::STR2ET(&save.UTC, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = save.ET0;
    //
    // Create a Mars orbiter SPK file.
    //
    spicelib::SPKOPN(SPK2, SPK2, 0, &mut save.HANDLE[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up elements defining a state.  The elements expected
    // by CONICS are:
    //
    //    RP      Perifocal distance.
    //    ECC     Eccentricity.
    //    INC     Inclination.
    //    LNODE   Longitude of the ascending node.
    //    ARGP    Argument of periapse.
    //    M0      Mean anomaly at epoch.
    //    T0      Epoch.
    //    MU      Gravitational parameter.
    //
    save.ELTS[1] = 3800.0;
    save.ELTS[2] = 0.1;
    save.ELTS[3] = (80.0 * spicelib::RPD(ctx));
    save.ELTS[4] = 0.0;
    save.ELTS[5] = (90.0 * spicelib::RPD(ctx));
    save.ELTS[6] = 0.0;
    save.ELTS[7] = save.ET;
    save.ELTS[8] = 42828.314;

    spicelib::CONICS(
        save.ELTS.as_slice(),
        save.ET,
        save.STATE0.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKW05(
        save.HANDLE[2],
        SCID,
        499,
        b"MARSIAU",
        -((10 as f64) * spicelib::JYEAR()),
        ((10 as f64) * spicelib::JYEAR()),
        b"Mars orbiter",
        save.ELTS[8],
        1,
        save.STATE0.as_slice(),
        &[save.ET],
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(save.HANDLE[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the new SPK file.
    //
    spicelib::SPKLEF(SPK2, &mut save.HANDLE[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add the orbiter's name/ID mapping to the kernel pool.
    //
    spicelib::PCPOOL(b"NAIF_BODY_NAME", 1, save.OBSNMS.subarray(2), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PIPOOL(b"NAIF_BODY_CODE", 1, &[SCID], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add an incomplete frame definition to the kernel pool;
    // we'll need this later.
    //
    spicelib::PIPOOL(b"FRAME_BAD_NAME", 1, &[-666], ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create DSK files.", ctx)?;

    //
    // For Mars, surface 1 is the "main" surface.
    //
    fstr::assign(&mut save.TARGET, b"MARS");
    save.TRGCDE = 499;
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");

    save.BODYID = save.TRGCDE;
    save.SURFID = 1;
    save.NLON = 200;
    save.NLAT = 100;

    if spicelib::EXISTS(DSK0, ctx)? {
        spicelib::DELFIL(DSK0, ctx)?;
    }

    testutil::T_ELDS2Z(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
        save.NLON,
        save.NLAT,
        DSK0,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load main Mars DSK.
    //
    spicelib::FURNSH(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Surface 2 for Mars is very low-res. We also use a
    // different scale for the Mars radii used to create
    // the tessellated shape model.
    //
    save.BODYID = save.TRGCDE;
    save.SURFID = 2;
    save.NLON = 40;
    save.NLAT = 20;

    if spicelib::EXISTS(DSK1, ctx)? {
        spicelib::DELFIL(DSK1, ctx)?;
    }

    fstr::assign(&mut save.KVNAME, b"BODY499_RADII");
    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"499 radii FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::VSCL(2.0, save.RADII.as_slice(), save.ALTRAD.as_slice_mut());

    spicelib::PDPOOL(&save.KVNAME, 3, save.ALTRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create and load the second DSK.
    //
    testutil::T_ELDS2Z(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
        save.NLON,
        save.NLAT,
        DSK1,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore normal Mars radii.
    //
    spicelib::PDPOOL(&save.KVNAME, 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Surface 1 for Phobos is low-res.
    //
    save.BODYID = 401;
    save.SURFID = 1;
    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");

    save.NLON = 200;
    save.NLAT = 100;

    if spicelib::EXISTS(DSK2, ctx)? {
        spicelib::DELFIL(DSK2, ctx)?;
    }

    //
    // Create and load the first Phobos DSK.
    //
    testutil::T_ELDS2Z(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
        save.NLON,
        save.NLAT,
        DSK2,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Surface 2 for Phobos is lower-res.
    //
    save.BODYID = 401;
    save.SURFID = 2;
    fstr::assign(&mut save.FIXREF, b"IAU_PHOBOS");

    save.NLON = 80;
    save.NLAT = 40;

    fstr::assign(&mut save.KVNAME, b"BODY401_RADII");
    spicelib::GDPOOL(
        &save.KVNAME,
        1,
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"401 radii FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::VSCL(3.0, save.RADII.as_slice(), save.ALTRAD.as_slice_mut());

    spicelib::PDPOOL(&save.KVNAME, 3, save.ALTRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if spicelib::EXISTS(DSK3, ctx)? {
        spicelib::DELFIL(DSK3, ctx)?;
    }

    //
    // Create and load the second Phobos DSK.
    //
    testutil::T_ELDS2Z(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
        save.NLON,
        save.NLAT,
        DSK3,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(DSK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore normal Phobos radii.
    //
    spicelib::PDPOOL(&save.KVNAME, 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up a surface name-ID map.
    //
    save.SRFBOD[1] = 499;
    save.SRFIDS[1] = 1;
    fstr::assign(save.SRFNMS.get_mut(1), b"high-res");

    save.SRFBOD[2] = 499;
    save.SRFIDS[2] = 2;
    fstr::assign(save.SRFNMS.get_mut(2), b"low-res");

    save.SRFBOD[3] = 401;
    save.SRFIDS[3] = 1;
    fstr::assign(save.SRFNMS.get_mut(3), b"high-res");

    save.SRFBOD[4] = 401;
    save.SRFIDS[4] = 2;
    fstr::assign(save.SRFNMS.get_mut(4), b"low-res");

    spicelib::PCPOOL(b"NAIF_SURFACE_NAME", NMAP, save.SRFNMS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"NAIF_SURFACE_CODE", NMAP, save.SRFIDS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"NAIF_SURFACE_BODY", NMAP, save.SRFBOD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate a grid of planetocentric longitude/latitude
    // coordinate pairs. These, combined with surface models,
    // will yield a grid of surface points at which to
    // compute illumination angles.
    //
    save.NSFLON = 4;
    save.NSFLAT = 5;
    save.NPTS = (save.NSFLON * save.NSFLAT);

    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NSFLON as f64);
    save.DLAT = (spicelib::PI(ctx) / (save.NSFLAT - 1) as f64);

    save.K = 0;

    for I in 1..=save.NSFLON {
        //
        // We shift the coordinates away from possible plate
        // edges because we can't expect the normal vectors
        // to match our computed values at those locations.
        //
        save.LON = ((((I - 1) as f64) * save.DLON) + 0.001);

        for J in 1..=save.NSFLAT {
            save.LAT = spicelib::BRCKTD(
                (spicelib::HALFPI(ctx) - (((J - 1) as f64) * save.DLAT)),
                -spicelib::HALFPI(ctx),
                spicelib::HALFPI(ctx),
            );

            if (J == 1) {
                save.LAT = (save.LAT - 0.001);
            } else if (J == save.NSFLAT) {
                save.LAT = (save.LAT + 0.001);
            }

            save.K = (save.K + 1);

            save.LONLAT[[1, save.K]] = save.LON;
            save.LONLAT[[2, save.K]] = save.LAT;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     Main test loop follows.
    //

    //
    // Loop over every choice of observer.
    //
    for OBSIDX in 1..=NOBS {
        fstr::assign(&mut save.OBSRVR, save.OBSNMS.get(OBSIDX));
        //
        // Set the observer ID code.
        //
        spicelib::BODN2C(&save.OBSRVR, &mut save.OBSCDE, &mut save.FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Loop over every choice of target.
        //
        for TRGIDX in 1..=NTARG {
            fstr::assign(&mut save.TARGET, save.TRGNMS.get(TRGIDX));

            //
            // Set the target ID code.
            //
            spicelib::BODN2C(&save.TARGET, &mut save.TRGCDE, &mut save.FOUND, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Get target radii.
            //
            spicelib::BODVAR(
                save.TRGCDE,
                b"RADII",
                &mut save.N,
                save.RADII.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Loop over the surface point sequence.
            //
            for PTIDX in 1..=save.NPTS {
                save.LON = save.LONLAT[[1, PTIDX]];
                save.LAT = save.LONLAT[[2, PTIDX]];

                //
                // Loop over every aberration correction choice.
                //
                for ABCIDX in 1..=NABC {
                    fstr::assign(&mut save.ABCORR, save.ABCS.get(ABCIDX));
                    //
                    // Set up some logical variables describing the
                    // attributes of the selected correction.
                    //
                    save.USELT = fstr::ne(&save.ABCORR, b"None");

                    save.USECN = fstr::eq(fstr::substr(&save.ABCORR, 1..=2), b"Cn");

                    //
                    // Loop over every target body-fixed frame choice.
                    //
                    for REFIDX in 1..=NREF {
                        fstr::assign(&mut save.TRGFRM, save.REFS.get([REFIDX, TRGIDX]));

                        //
                        // Loop over all method choices.
                        //

                        for MIX in 1..=NMETH {
                            fstr::assign(&mut save.METHOD, save.METHDS.get(MIX));

                            //
                            // --- Case: ------------------------------------------------------
                            //
                            fstr::assign(&mut save.TITLE, b"Observer = #; Target = #; ABCORR = #; TRGFRM = #; METHOD = #; Longitude (deg) = #; Latitude (deg) = #; ET = #.");
                            spicelib::REPMC(
                                &save.TITLE.to_vec(),
                                b"#",
                                &save.OBSRVR,
                                &mut save.TITLE,
                            );
                            spicelib::REPMC(
                                &save.TITLE.to_vec(),
                                b"#",
                                &save.TARGET,
                                &mut save.TITLE,
                            );
                            spicelib::REPMC(
                                &save.TITLE.to_vec(),
                                b"#",
                                &save.ABCORR,
                                &mut save.TITLE,
                            );
                            spicelib::REPMC(
                                &save.TITLE.to_vec(),
                                b"#",
                                &save.TRGFRM,
                                &mut save.TITLE,
                            );
                            spicelib::REPMC(
                                &save.TITLE.to_vec(),
                                b"#",
                                &save.METHOD,
                                &mut save.TITLE,
                            );
                            spicelib::REPMD(
                                &save.TITLE.to_vec(),
                                b"#",
                                (save.LON * spicelib::DPR(ctx)),
                                14,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMD(
                                &save.TITLE.to_vec(),
                                b"#",
                                (save.LAT * spicelib::DPR(ctx)),
                                14,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMD(
                                &save.TITLE.to_vec(),
                                b"#",
                                save.ET,
                                14,
                                &mut save.TITLE,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::TCASE(&save.TITLE, ctx)?;

                            //
                            // Generate the surface point we're going to work
                            // with. We do this here because we need the
                            // current method in order to generate a point on
                            // the surface.
                            //
                            spicelib::LATSRF(
                                &save.METHOD,
                                &save.TARGET,
                                save.ET,
                                &save.TRGFRM,
                                1,
                                save.LONLAT.subarray([1, PTIDX]),
                                save.SPOINT.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            // Get the outward surface normal vector at
                            // SPOINT.
                            //
                            spicelib::SRFNRM(
                                &save.METHOD,
                                &save.TARGET,
                                save.ET,
                                &save.TRGFRM,
                                1,
                                save.SPOINT.as_slice(),
                                save.NORMAL.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            if !*OK {
                                ctx.stop()?;
                            }

                            //
                            // Start off by computing the illumination angles.
                            // We'll then check the results.
                            //
                            spicelib::ILUMIN(
                                &save.METHOD,
                                &save.TARGET,
                                save.ET,
                                &save.TRGFRM,
                                &save.ABCORR,
                                &save.OBSRVR,
                                save.SPOINT.as_slice(),
                                &mut save.TRGEPC,
                                save.SRFVEC.as_slice_mut(),
                                &mut save.PHASE,
                                &mut save.INCDNC,
                                &mut save.EMISSN,
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            if !*OK {
                                ctx.stop()?;
                            }

                            //
                            // We'll treat the input surface point an
                            // ephemeris object and find its position relative
                            // to the observer.
                            //
                            spicelib::SPKCPT(
                                save.SPOINT.as_slice(),
                                &save.TARGET,
                                &save.TRGFRM,
                                save.ET,
                                &save.TRGFRM,
                                b"TARGET",
                                &save.ABCORR,
                                &save.OBSRVR,
                                save.STATE.as_slice_mut(),
                                &mut save.LT,
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // If SRFVEC is correct, then the position of
                            // SPOINT relative to the observer should be equal
                            // to SRFVEC. The light time obtained from SPKCPT
                            // should match that implied by TRGEPC.
                            //
                            save.TOL = TIGHT;

                            spicelib::ZZCOREPC(&save.ABCORR, save.ET, save.LT, &mut save.XTE, ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKSD(
                                b"TRGEPC",
                                save.TRGEPC,
                                b"~/",
                                save.XTE,
                                save.TOL,
                                OK,
                                ctx,
                            )?;

                            if save.USELT {
                                if save.USECN {
                                    save.TOL = MTIGHT;
                                } else {
                                    save.TOL = LOOSE;
                                }
                            } else {
                                save.TOL = VTIGHT;
                            }

                            testutil::CHCKAD(
                                b"SRFVEC",
                                save.SRFVEC.as_slice(),
                                b"~~/",
                                save.STATE.as_slice(),
                                3,
                                save.TOL,
                                OK,
                                ctx,
                            )?;

                            //
                            // We've checked the consistency of SPOINT,
                            // SRFVEC, and TRGEPC, but we haven't done
                            // anything to test the illumination angles.
                            // Do that now.
                            //
                            // We need the vectors used to define the
                            // illumination angles.
                            //

                            // Negate the observer-surface point vector.
                            //
                            spicelib::VMINUS(save.SRFVEC.as_slice(), save.OBSPOS.as_slice_mut());

                            //
                            // Get the apparent position of the sun as seen
                            // from the surface point at TRGEPC.
                            //
                            spicelib::SPKCPO(
                                b"SUN",
                                save.TRGEPC,
                                &save.TRGFRM,
                                b"OBSERVER",
                                &save.ABCORR,
                                save.SPOINT.as_slice(),
                                &save.TARGET,
                                &save.TRGFRM,
                                save.SUNSTA.as_slice_mut(),
                                &mut save.SUNLT,
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // Compute the expected illumination angles.
                            //
                            save.XPHASE =
                                spicelib::VSEP(save.SUNSTA.as_slice(), save.OBSPOS.as_slice(), ctx);
                            save.XINCDC =
                                spicelib::VSEP(save.SUNSTA.as_slice(), save.NORMAL.as_slice(), ctx);
                            save.XEMISN =
                                spicelib::VSEP(save.OBSPOS.as_slice(), save.NORMAL.as_slice(), ctx);

                            //
                            // Since we're doing a consistency check, we
                            // expect to get very close agreement with
                            // ILUMIN.
                            //
                            save.TOL = VTIGHT;

                            testutil::CHCKSD(
                                b"PHASE",
                                save.PHASE,
                                b"~/",
                                save.XPHASE,
                                save.TOL,
                                OK,
                                ctx,
                            )?;

                            testutil::CHCKSD(
                                b"INCDNC",
                                save.INCDNC,
                                b"~/",
                                save.XINCDC,
                                save.TOL,
                                OK,
                                ctx,
                            )?;

                            testutil::CHCKSD(
                                b"EMISSN",
                                save.EMISSN,
                                b"~/",
                                save.XEMISN,
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        }
                        //
                        // End of the method loop.
                        //
                    }
                    //
                    // End of the reference frame loop.
                    //
                }
                //
                // End of the aberration correction loop.
                //
            }
            //
            // End of the surface point loop.
            //
        }
        //
        // End of the target loop.
        //
    }
    //
    // End of the observer loop.
    //

    //***********************************************************************
    //
    //     Normal case: input handling
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Input handling tests:  make sure target and observer
    // can be identified using integer "names."
    //
    testutil::TCASE(b"Use integer observer and target names.", ctx)?;

    //
    // Set target and target-fixed frame.
    //
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.METHOD, b"ellipsoid");

    spicelib::LATSRF(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.LONLAT.subarray([1, (save.NPTS / 2)]),
        save.SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ILUMIN(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        &mut save.XPHASE,
        &mut save.XINCDC,
        &mut save.XEMISN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ILUMIN(
        &save.METHOD,
        b"499",
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        b"399",
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"SRFVEC",
        save.SRFVEC.as_slice(),
        b"=",
        save.XSRFVC.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"TRGEPC", save.TRGEPC, b"=", save.XEPOCH, 0.0, OK, ctx)?;

    testutil::CHCKSD(b"PHASE", save.PHASE, b"=", save.XPHASE, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"INCDNC", save.INCDNC, b"=", save.XINCDC, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"EMISSN", save.EMISSN, b"=", save.XEMISN, save.TOL, OK, ctx)?;

    //***********************************************************************
    //
    //     Normal case: state change detection
    //
    //***********************************************************************

    //
    // Certain subsystem state changes must be detected and responded to
    // by SINCPT. The subsystems (or structures) having states that must
    // be monitored are:
    //
    //    - Target name-ID mapping
    //
    //    - Observer name-ID mapping
    //
    //    - Surface name-ID mapping
    //
    //    - Target body-fixed frame definition
    //
    //    - ZZDSKBSR state
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Target name changed to JUPITER for ID code 499.", ctx)?;

    //
    // First, get expected intercept.
    //
    save.ET = ((8 as f64) * spicelib::JYEAR());

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ABCORR, b"NONE");

    spicelib::ILUMIN(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        &mut save.XPHASE,
        &mut save.XINCDC,
        &mut save.XEMISN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODDEF(b"JUPITER", 499, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact matches here.
    //
    testutil::CHCKAD(
        b"SRFVEC",
        save.SRFVEC.as_slice(),
        b"=",
        save.XSRFVC.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"TRGEPC", save.TRGEPC, b"=", save.XEPOCH, 0.0, OK, ctx)?;

    testutil::CHCKSD(b"PHASE", save.PHASE, b"=", save.XPHASE, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"INCDNC", save.INCDNC, b"=", save.XINCDC, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"EMISSN", save.EMISSN, b"=", save.XEMISN, save.TOL, OK, ctx)?;

    // Restore original mapping.
    //
    spicelib::BODDEF(b"JUPITER", 599, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Observer name changed to SUN for ID code 399.", ctx)?;

    spicelib::BODDEF(b"SUN", 399, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // We expect exact matches here.
    //
    testutil::CHCKAD(
        b"SRFVEC",
        save.SRFVEC.as_slice(),
        b"=",
        save.XSRFVC.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"TRGEPC", save.TRGEPC, b"=", save.XEPOCH, 0.0, OK, ctx)?;

    testutil::CHCKSD(b"PHASE", save.PHASE, b"=", save.XPHASE, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"INCDNC", save.INCDNC, b"=", save.XINCDC, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"EMISSN", save.EMISSN, b"=", save.XEMISN, save.TOL, OK, ctx)?;
    //
    // Restore original mapping.
    //
    spicelib::BODDEF(b"SUN", 10, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars high-res surface name changed to AAAbbb.", ctx)?;

    //
    // Get expected results first.
    //
    save.ET = ((8 as f64) * spicelib::JYEAR());

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ABCORR, b"NONE");

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized/surfaces = 1");

    spicelib::LATSRF(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.LONLAT.subarray([1, (save.NPTS / 2)]),
        save.SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ILUMIN(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        &mut save.XPHASE,
        &mut save.XINCDC,
        &mut save.XEMISN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(save.SRFNMS.get_mut(1), b"AAAbbb");

    spicelib::PCPOOL(b"NAIF_SURFACE_NAME", NMAP, save.SRFNMS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized/surfaces = AAAbbb");

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
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact matches here.
    //
    testutil::CHCKAD(
        b"SRFVEC",
        save.SRFVEC.as_slice(),
        b"=",
        save.XSRFVC.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"TRGEPC", save.TRGEPC, b"=", save.XEPOCH, 0.0, OK, ctx)?;

    testutil::CHCKSD(b"PHASE", save.PHASE, b"=", save.XPHASE, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"INCDNC", save.INCDNC, b"=", save.XINCDC, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"EMISSN", save.EMISSN, b"=", save.XEMISN, save.TOL, OK, ctx)?;
    //
    // Restore original mapping.
    //
    spicelib::BODDEF(b"SUN", 10, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore original mapping.
    //
    fstr::assign(save.SRFNMS.get_mut(1), b"high-res");

    spicelib::PCPOOL(b"NAIF_SURFACE_NAME", NMAP, save.SRFNMS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Unload Mars high-res DSK.", ctx)?;

    //
    // Get reference result using low-res Mars DSK.
    //
    save.ET = ((8 as f64) * spicelib::JYEAR());

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ABCORR, b"NONE");

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized/surfaces = low-res");

    spicelib::LATSRF(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.LONLAT.subarray([1, (save.NPTS / 2)]),
        save.SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ILUMIN(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        &mut save.XPHASE,
        &mut save.XINCDC,
        &mut save.XEMISN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Unload the high-res DSK; set METHOD to remove
    // surface specification.
    //
    spicelib::UNLOAD(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized");

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
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact matches here.
    //
    testutil::CHCKAD(
        b"SRFVEC",
        save.SRFVEC.as_slice(),
        b"=",
        save.XSRFVC.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"TRGEPC", save.TRGEPC, b"=", save.XEPOCH, 0.0, OK, ctx)?;

    testutil::CHCKSD(b"PHASE", save.PHASE, b"=", save.XPHASE, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"INCDNC", save.INCDNC, b"=", save.XINCDC, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"EMISSN", save.EMISSN, b"=", save.XEMISN, save.TOL, OK, ctx)?;

    //
    // Restore original mapping.
    //
    spicelib::BODDEF(b"SUN", 10, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Unload Mars low-res DSK; reload Mars high-res DSK.", ctx)?;

    //
    // Restore DSK, unload low-res DSK, and repeat computation.
    //
    spicelib::FURNSH(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized");

    spicelib::LATSRF(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.LONLAT.subarray([1, (save.NPTS / 2)]),
        save.SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ILUMIN(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        &mut save.XPHASE,
        &mut save.XINCDC,
        &mut save.XEMISN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure the result matches that obtained with the
    // high-res DSK specified.
    //
    fstr::assign(
        &mut save.METHOD,
        b"dsk/unprioritized/ SURFACES = \"HIGH-RES\" ",
    );

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
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect exact matches here.
    //
    testutil::CHCKAD(
        b"SRFVEC",
        save.SRFVEC.as_slice(),
        b"=",
        save.XSRFVC.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"TRGEPC", save.TRGEPC, b"=", save.XEPOCH, 0.0, OK, ctx)?;

    testutil::CHCKSD(b"PHASE", save.PHASE, b"=", save.XPHASE, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"INCDNC", save.INCDNC, b"=", save.XINCDC, save.TOL, OK, ctx)?;

    testutil::CHCKSD(b"EMISSN", save.EMISSN, b"=", save.XEMISN, save.TOL, OK, ctx)?;
    //
    // Restore original mapping.
    //
    spicelib::BODDEF(b"SUN", 10, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //***********************************************************************
    //
    //     Error handling tests follow.
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid method.", ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");

    spicelib::LATSRF(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        1,
        save.LONLAT.subarray([1, (save.NPTS / 2)]),
        save.SPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ILUMIN(
        b"ELLIPSID",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"NONE",
        b"SUN",
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    spicelib::ILUMIN(
        b"INTERCEPT ELLIPSOID",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"NONE",
        b"SUN",
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    spicelib::ILUMIN(
        b"/DSK",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"NONE",
        b"SUN",
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    spicelib::ILUMIN(
        b"/DSK/UPRIORTIZED",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"NONE",
        b"SUN",
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    spicelib::ILUMIN(
        b"DSK",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"NONE",
        b"SUN",
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADPRIORITYSPEC)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid observer name.", ctx)?;

    spicelib::ILUMIN(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        b"erth",
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid target name.", ctx)?;

    spicelib::ILUMIN(
        &save.METHOD,
        b"su",
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Observer is target.", ctx)?;

    spicelib::ILUMIN(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.TARGET,
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid reference frame center", ctx)?;

    spicelib::ILUMIN(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        b"IAU_MOON",
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid aberration correction", ctx)?;

    spicelib::ILUMIN(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        b"LTT",
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // Test SAVE logic by repeating the call.
    //

    spicelib::ILUMIN(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        b"LTT",
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Relativistic aberration correction", ctx)?;

    spicelib::ILUMIN(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        b"RL",
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Stellar aberration correction w/o light time", ctx)?;

    spicelib::ILUMIN(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        b"S",
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No loaded SPK files", ctx)?;

    spicelib::SPKUEF(save.HANDLE[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HANDLE[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOLOADEDFILES)", OK, ctx)?;

    spicelib::SPKLEF(SPK1, &mut save.HANDLE[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKLEF(SPK2, &mut save.HANDLE[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No ephemeris data for observer", ctx)?;

    spicelib::ILUMIN(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        b"1000",
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No ephemeris data for target", ctx)?;

    spicelib::ILUMIN(
        &save.METHOD,
        b"gaspra",
        save.ET,
        b"IAU_GASPRA",
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No orientation data for target", ctx)?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LDPOOL(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No radius data for target", ctx)?;

    spicelib::DVPOOL(b"BODY399_RADII", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ILUMIN(
        b"ELLIPSOID",
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        &mut save.PHASE,
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    spicelib::LDPOOL(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad radius data for target", ctx)?;

    //
    // Fetch original radii.
    //
    spicelib::BODVCD(
        399,
        b"RADII",
        3,
        &mut save.N,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Overwrite good radii with bad in the kernel pool.
    //
    spicelib::VPACK(-1.0, 0.0, 3.0, save.BADRAD.as_slice_mut());

    spicelib::PDPOOL(b"BODY399_RADII", 3, save.BADRAD.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

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
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // Replace original radii.
    //
    spicelib::PDPOOL(b"BODY399_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"No loaded DSKs.", ctx)?;

    spicelib::UNLOAD(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::UNLOAD(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::UNLOAD(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::UNLOAD(DSK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET = ((8 as f64) * spicelib::JYEAR());

    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.ABCORR, b"NONE");

    fstr::assign(&mut save.METHOD, b"dsk/unprioritized");

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
        &mut save.INCDNC,
        &mut save.EMISSN,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOLOADEDDSKFILES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Clean up.
    //
    spicelib::DELFIL(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HANDLE[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HANDLE[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(DSK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
