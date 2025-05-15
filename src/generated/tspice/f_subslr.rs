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
const DSK0: &[u8] = b"subslr_dsk0.bds";
const DSK1: &[u8] = b"subslr_dsk1.bds";
const DSK2: &[u8] = b"subslr_dsk2.bds";
const DSK3: &[u8] = b"subslr_dsk3.bds";
const PCK: &[u8] = b"test_0008.tpc";
const SPK1: &[u8] = b"subslr_spk.bsp";
const SPK2: &[u8] = b"orbiter.bsp";
const VTIGHT: f64 = 0.0000000001;
const TIGHT: f64 = 0.000000001;
const MTIGHT: f64 = 0.00000001;
const LOOSE: f64 = 0.000005;
const LNSIZE: i32 = 240;
const NAMLEN: i32 = 32;
const NREF: i32 = 1;
const NABC: i32 = 5;
const ABCLEN: i32 = 10;
const TIMLEN: i32 = 50;
const NOBS: i32 = 2;
const NTARG: i32 = 2;
const NTIMES: i32 = 3;
const SCID: i32 = -499;
const NMAP: i32 = 4;
const NMETH: i32 = 8;

struct SaveVars {
    ABCS: ActualCharArray,
    ABCORR: Vec<u8>,
    FIXREF: Vec<u8>,
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
    ALT: f64,
    BADRAD: StackArray<f64, 3>,
    ELTS: StackArray<f64, 8>,
    ET: f64,
    ET0: f64,
    ESUBPT: StackArray<f64, 3>,
    LT: f64,
    OBSPOS: StackArray<f64, 3>,
    RADII: StackArray<f64, 3>,
    SPOINT: StackArray<f64, 3>,
    SRFVEC: StackArray<f64, 3>,
    STATE: StackArray<f64, 6>,
    STATE0: StackArray<f64, 6>,
    SUBDIR: StackArray<f64, 3>,
    SUNLT: f64,
    SUNPOS: StackArray<f64, 3>,
    SUNSTA: StackArray<f64, 6>,
    SUNTRG: StackArray<f64, 3>,
    TDELTA: f64,
    TOL: f64,
    TRGEPC: f64,
    XEPOCH: f64,
    XSUBPT: StackArray<f64, 3>,
    XSRFVC: StackArray<f64, 3>,
    XTE: f64,
    BODYID: i32,
    HANDLE: StackArray<i32, 2>,
    N: i32,
    NLAT: i32,
    NLON: i32,
    OBSCDE: i32,
    SRFBOD: StackArray<i32, 4>,
    SRFIDS: StackArray<i32, 4>,
    SURFID: i32,
    SRFLST: StackArray<i32, 100>,
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
        let mut ALT: f64 = 0.0;
        let mut BADRAD = StackArray::<f64, 3>::new(1..=3);
        let mut ELTS = StackArray::<f64, 8>::new(1..=8);
        let mut ET: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ESUBPT = StackArray::<f64, 3>::new(1..=3);
        let mut LT: f64 = 0.0;
        let mut OBSPOS = StackArray::<f64, 3>::new(1..=3);
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
        let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut STATE0 = StackArray::<f64, 6>::new(1..=6);
        let mut SUBDIR = StackArray::<f64, 3>::new(1..=3);
        let mut SUNLT: f64 = 0.0;
        let mut SUNPOS = StackArray::<f64, 3>::new(1..=3);
        let mut SUNSTA = StackArray::<f64, 6>::new(1..=6);
        let mut SUNTRG = StackArray::<f64, 3>::new(1..=3);
        let mut TDELTA: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut TRGEPC: f64 = 0.0;
        let mut XEPOCH: f64 = 0.0;
        let mut XSUBPT = StackArray::<f64, 3>::new(1..=3);
        let mut XSRFVC = StackArray::<f64, 3>::new(1..=3);
        let mut XTE: f64 = 0.0;
        let mut BODYID: i32 = 0;
        let mut HANDLE = StackArray::<i32, 2>::new(1..=2);
        let mut N: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut OBSCDE: i32 = 0;
        let mut SRFBOD = StackArray::<i32, 4>::new(1..=NMAP);
        let mut SRFIDS = StackArray::<i32, 4>::new(1..=NMAP);
        let mut SURFID: i32 = 0;
        let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
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
                Val::C(b"Cn"),
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
                Val::C(b"ELLIPSOID / intercept"),
                Val::C(b"intercept: ellipsoid"),
                Val::C(b"near point: ellipsoid"),
                Val::C(b"near point /ellipsoid"),
                Val::C(b"nadir/dsk/unprioritized/surfaces=\"high-res\""),
                Val::C(b"dsk/ nadir /unprioritized/surfaces=\"high-res\""),
                Val::C(b"intercept/ UNPRIORITIZED/ dsk /SURFACES =\"LOW-RES\""),
                Val::C(b"intercept/UNPRIORITIZED/ dsk /SURFACES =\"LOW-RES\""),
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
            ALT,
            BADRAD,
            ELTS,
            ET,
            ET0,
            ESUBPT,
            LT,
            OBSPOS,
            RADII,
            SPOINT,
            SRFVEC,
            STATE,
            STATE0,
            SUBDIR,
            SUNLT,
            SUNPOS,
            SUNSTA,
            SUNTRG,
            TDELTA,
            TOL,
            TRGEPC,
            XEPOCH,
            XSUBPT,
            XSRFVC,
            XTE,
            BODYID,
            HANDLE,
            N,
            NLAT,
            NLON,
            OBSCDE,
            SRFBOD,
            SRFIDS,
            SURFID,
            SRFLST,
            TRGCDE,
            FOUND,
            USECN,
            USELT,
        }
    }
}

//$Procedure      F_SUBSLR ( SUBSLR family tests )
pub fn F_SUBSLR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Corrections are reception-only for the sub-solar point.
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
    testutil::TOPEN(b"F_SUBSLR", ctx)?;

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
    save.TDELTA = spicelib::JYEAR();

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
    // Surface 2 for Mars is very low-res.
    //
    save.BODYID = save.TRGCDE;
    save.SURFID = 2;
    save.NLON = 40;
    save.NLAT = 20;

    if spicelib::EXISTS(DSK1, ctx)? {
        spicelib::DELFIL(DSK1, ctx)?;
    }

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
            // Loop over the time sequence.
            //
            for TIMIDX in 1..=NTIMES {
                save.ET = (save.ET0 + (((TIMIDX - 1) as f64) * save.TDELTA));

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

                    save.USECN = (fstr::eq(fstr::substr(&save.ABCORR, 1..=2), b"Cn")
                        || fstr::eq(fstr::substr(&save.ABCORR, 1..=3), b"Xcn"));

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

                            fstr::assign(&mut save.TITLE, b"Observer = #; Target = #; ABCORR = #; TRGFRM = #; METHOD = #; ET = #.");
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
                                save.ET,
                                14,
                                &mut save.TITLE,
                                ctx,
                            );

                            testutil::TCASE(&save.TITLE, ctx)?;

                            //
                            // Start off by computing the sub-solar point.
                            // We'll then check the results.
                            //
                            spicelib::SUBSLR(
                                &save.METHOD,
                                &save.TARGET,
                                save.ET,
                                &save.TRGFRM,
                                &save.ABCORR,
                                &save.OBSRVR,
                                save.SPOINT.as_slice_mut(),
                                &mut save.TRGEPC,
                                save.SRFVEC.as_slice_mut(),
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // We'll treat the computed sub-solar point as
                            // an ephemeris object and find its position
                            // relative to the observer.
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
                            // If SPOINT is correct, then the position of
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
                            // anything to show that SPOINT is a sub-solar
                            // point. Do that now.
                            //
                            // Start by getting the position of the sun with
                            // respect to the target at TRGEPC. Use the target
                            // body-fixed frame. This is a non-standard
                            // computation: we want to perform aberration
                            // corrections for the sun that apply when the
                            // sub-solar point is the observer.
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

                            spicelib::VADD(
                                save.SPOINT.as_slice(),
                                save.SUNSTA.as_slice(),
                                save.SUNPOS.as_slice_mut(),
                            );
                            spicelib::VMINUS(save.SUNPOS.as_slice(), save.SUNTRG.as_slice_mut());

                            //
                            // Compute the position of the observer relative to
                            // the target center. We'll use this later.
                            //
                            spicelib::VSUB(
                                save.SPOINT.as_slice(),
                                save.STATE.as_slice(),
                                save.OBSPOS.as_slice_mut(),
                            );

                            //
                            // Find the sub-solar point on the target body.
                            //
                            save.FOUND = false;

                            if spicelib::MATCHI(&save.METHOD, b"*INTERCEPT*", b"*", b"?", ctx) {
                                if spicelib::MATCHI(&save.METHOD, b"*ELLIPSOID*", b"*", b"?", ctx) {
                                    spicelib::SURFPT(
                                        save.SUNPOS.as_slice(),
                                        save.SUNTRG.as_slice(),
                                        save.RADII[1],
                                        save.RADII[2],
                                        save.RADII[3],
                                        save.XSUBPT.as_slice_mut(),
                                        &mut save.FOUND,
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                                } else {
                                    //
                                    // This is the DSK case. Note that we're
                                    // working with the target epoch (although
                                    // in this case, it's only used for DSK
                                    // segment selection).
                                    //
                                    // Set the surface list according to the
                                    // value of the surface list in METHOD.
                                    //
                                    if spicelib::MATCHI(
                                        &save.METHOD,
                                        b"*HIGH-RES*",
                                        b"*",
                                        b"?",
                                        ctx,
                                    ) {
                                        save.SRFLST[1] = 1;
                                    } else {
                                        save.SRFLST[1] = 2;
                                    }

                                    spicelib::DSKXV(
                                        false,
                                        &save.TARGET,
                                        1,
                                        save.SRFLST.as_slice(),
                                        save.TRGEPC,
                                        &save.TRGFRM,
                                        1,
                                        save.SUNPOS.as_slice(),
                                        save.SUNTRG.as_slice(),
                                        save.XSUBPT.as_slice_mut(),
                                        std::slice::from_mut(&mut save.FOUND),
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                                }

                                testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                                //
                                // Use the tolerance we set for the
                                // SRFVEC test.
                                //
                                // To simplify the comparison, we'll convert
                                // the expected sub-solar point to an
                                // expected observer-surface vector. This will
                                // give us a vector having the scale on which
                                // we based our tolerance magnitude.
                                //
                                spicelib::VSUB(
                                    save.XSUBPT.as_slice(),
                                    save.OBSPOS.as_slice(),
                                    save.XSRFVC.as_slice_mut(),
                                );

                                testutil::CHCKAD(
                                    b"SRFVEC (2)",
                                    save.SRFVEC.as_slice(),
                                    b"~~/",
                                    save.XSRFVC.as_slice(),
                                    3,
                                    save.TOL,
                                    OK,
                                    ctx,
                                )?;
                            } else {
                                //
                                // This is the "near point" or "nadir" case.
                                //
                                // We need the ellipsoid near point for both
                                // the ellipsoid and DSK target shape cases.
                                //
                                spicelib::NEARPT(
                                    save.SUNPOS.as_slice(),
                                    save.RADII[1],
                                    save.RADII[2],
                                    save.RADII[3],
                                    save.ESUBPT.as_slice_mut(),
                                    &mut save.ALT,
                                    ctx,
                                )?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                //
                                // Get the direction vector from the sun to
                                // the ellipsoid near point.
                                //
                                spicelib::VSUB(
                                    save.ESUBPT.as_slice(),
                                    save.SUNPOS.as_slice(),
                                    save.SUBDIR.as_slice_mut(),
                                );

                                if spicelib::MATCHI(&save.METHOD, b"*ELLIPSOID*", b"*", b"?", ctx) {
                                    //
                                    // This is the ellipsoid case. The near
                                    // point we've already found is our expected
                                    // sub-solar point.
                                    //
                                    spicelib::VEQU(
                                        save.ESUBPT.as_slice(),
                                        save.XSUBPT.as_slice_mut(),
                                    );
                                } else {
                                    //
                                    // This is the DSK case.
                                    //
                                    // Set the surface list according to the
                                    // value of the surface list in METHOD.
                                    //
                                    if spicelib::MATCHI(
                                        &save.METHOD,
                                        b"*HIGH-RES*",
                                        b"*",
                                        b"?",
                                        ctx,
                                    ) {
                                        save.SRFLST[1] = 1;
                                    } else {
                                        save.SRFLST[1] = 2;
                                    }

                                    //
                                    // Update XSUBPT to be the DSK surface
                                    // intercept of the ray from the sun to
                                    // the ellipsoid near point.
                                    //
                                    spicelib::DSKXV(
                                        false,
                                        &save.TARGET,
                                        1,
                                        save.SRFLST.as_slice(),
                                        save.TRGEPC,
                                        &save.TRGFRM,
                                        1,
                                        save.SUNPOS.as_slice(),
                                        save.SUBDIR.as_slice(),
                                        save.XSUBPT.as_slice_mut(),
                                        std::slice::from_mut(&mut save.FOUND),
                                        ctx,
                                    )?;
                                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
                                }
                                //
                                // Update XSRFVC based on the expected near
                                // point or nadir point.
                                //
                                spicelib::VSUB(
                                    save.XSUBPT.as_slice(),
                                    save.OBSPOS.as_slice(),
                                    save.XSRFVC.as_slice_mut(),
                                );

                                testutil::CHCKAD(
                                    b"SRFVEC (2)",
                                    save.SRFVEC.as_slice(),
                                    b"~~/",
                                    save.XSRFVC.as_slice(),
                                    3,
                                    save.TOL,
                                    OK,
                                    ctx,
                                )?;
                            }
                            //
                            // We're finished with the sub-solar point
                            // consistency checks for both the intercept and
                            // near point/nadir cases.
                            //
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
            // End of the time loop.
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
    fstr::assign(&mut save.METHOD, b"ellipsoid/intercept");

    spicelib::SUBSLR(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.XSUBPT.as_slice_mut(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SUBSLR(
        &save.METHOD,
        b"499",
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        b"399",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"SPOINT",
        save.SPOINT.as_slice(),
        b"=",
        save.XSUBPT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
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

    spicelib::SUBSLR(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.XSUBPT.as_slice_mut(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BODDEF(b"JUPITER", 499, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SUBSLR(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"SPOINT",
        save.SPOINT.as_slice(),
        b"=",
        save.XSUBPT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

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

    spicelib::SUBSLR(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"SPOINT",
        save.SPOINT.as_slice(),
        b"=",
        save.XSUBPT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

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

    fstr::assign(
        &mut save.METHOD,
        b"intercept/dsk/unprioritized/surfaces = 1",
    );

    spicelib::SUBSLR(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.XSUBPT.as_slice_mut(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(save.SRFNMS.get_mut(1), b"AAAbbb");

    spicelib::PCPOOL(b"NAIF_SURFACE_NAME", NMAP, save.SRFNMS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(
        &mut save.METHOD,
        b"intercept/dsk/unprioritized/surfaces = AAAbbb",
    );

    spicelib::SUBSLR(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"SPOINT",
        save.SPOINT.as_slice(),
        b"=",
        save.XSUBPT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

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

    fstr::assign(
        &mut save.METHOD,
        b"nadir/dsk/unprioritized/surfaces = low-res",
    );

    spicelib::SUBSLR(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.XSUBPT.as_slice_mut(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Unload the high-res DSK; set METHOD to remove
    // surface specification.
    //
    spicelib::UNLOAD(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"nadir/dsk/unprioritized");

    spicelib::SUBSLR(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"SPOINT",
        save.SPOINT.as_slice(),
        b"=",
        save.XSUBPT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

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

    fstr::assign(&mut save.METHOD, b"nadir/dsk/unprioritized");

    spicelib::SUBSLR(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure the result matches that obtained with the
    // high-res DSK specified.
    //
    fstr::assign(
        &mut save.METHOD,
        b"nadir/dsk/unprioritized/ SURFACES = \"HIGH-RES\" ",
    );

    spicelib::SUBSLR(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.OBSRVR,
        save.XSUBPT.as_slice_mut(),
        &mut save.XEPOCH,
        save.XSRFVC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect an exact match here.
    //
    testutil::CHCKAD(
        b"SPOINT",
        save.SPOINT.as_slice(),
        b"=",
        save.XSUBPT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //***********************************************************************
    //
    //     Error handling tests follow.
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid method.", ctx)?;

    spicelib::SUBSLR(
        b"INTERCPT/ ELLIPSOID",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"NONE",
        b"SUN",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOIDD",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"NONE",
        b"SUN",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    spicelib::SUBSLR(
        b"ellipsoid / INTERCEPT /",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"NONE",
        b"SUN",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    spicelib::SUBSLR(
        b"ELLIPSOID",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"NONE",
        b"SUN",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSUBTYPE)", OK, ctx)?;

    spicelib::SUBSLR(
        b"INTERCEPT ELLIPSOID",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"NONE",
        b"SUN",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid target name.", ctx)?;

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"ERTH",
        save.ET,
        b"IAU_EARTH",
        b"NONE",
        b"SUN",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid observer name.", ctx)?;

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"NONE",
        b"SN",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // This is a non-error case for SUBSLR.
    //
    testutil::TCASE(b"Observer is target.", ctx)?;

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"NONE",
        b"earth",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid reference frame center", ctx)?;

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"EARTH",
        save.ET,
        b"IAU_MOON",
        b"NONE",
        b"SUN",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Reference frame not found", ctx)?;

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"EARTH",
        save.ET,
        b"IAU_EART",
        b"NONE",
        b"sun",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid aberration correction", ctx)?;

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"LTT",
        b"sun",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // Test SAVE logic by repeating the call.
    //
    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"LTT",
        b"sun",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Relativistic aberration correction", ctx)?;

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"RL",
        b"sun",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Stellar aberration correction w/o light time", ctx)?;

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"S",
        b"sun",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
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

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"LT+S",
        b"sun",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
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

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"EARTH",
        save.ET,
        b"IAU_EARTH",
        b"LT+S",
        b"1000",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No ephemeris data for target", ctx)?;

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"gaspra",
        save.ET,
        b"IAU_GASPRA",
        b"LT+S",
        b"10",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"No orientation data for target", ctx)?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"earth",
        save.ET,
        b"IAU_EARTH",
        b"LT+S",
        b"10",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
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

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"earth",
        save.ET,
        b"IAU_EARTH",
        b"LT+S",
        b"10",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
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

    spicelib::SUBSLR(
        b"INTERCEPT: ELLIPSOID",
        b"earth",
        save.ET,
        b"IAU_EARTH",
        b"LT+S",
        b"10",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
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

    fstr::assign(&mut save.METHOD, b"nadir/dsk/unprioritized");

    spicelib::SUBSLR(
        &save.METHOD,
        b"499",
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        b"399",
        save.SPOINT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
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
