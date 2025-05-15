//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const PCK: &[u8] = b"test_0008.tpc";
const SPK1: &[u8] = b"srfxpt_spk.bsp";
const SPK2: &[u8] = b"orbiter.bsp";
const VTIGHT: f64 = 0.00000000000001;
const TIGHT: f64 = 0.000000000001;
const MTIGHT: f64 = 0.0000000001;
const MEDIUM: f64 = 0.00000001;
const LOOSE: f64 = 0.00001;
const SLOPPY: f64 = 0.001;
const UBEL: i32 = 9;
const FILSIZ: i32 = 255;
const LNSIZE: i32 = 80;
const NAMLEN: i32 = 32;
const NREF: i32 = 3;
const NABC: i32 = 9;
const ABCLEN: i32 = 10;
const TIMLEN: i32 = 50;
const NOBS: i32 = 2;
const NGEOM: i32 = 4;
const SCID: i32 = -499;

struct SaveVars {
    ABCS: ActualCharArray,
    GEOMS: ActualCharArray,
    OBSNMS: ActualCharArray,
    REFS: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCS = ActualCharArray::new(ABCLEN, 1..=NABC);
        let mut GEOMS = ActualCharArray::new(LNSIZE, 1..=NGEOM);
        let mut OBSNMS = ActualCharArray::new(NAMLEN, 1..=NOBS);
        let mut REFS = ActualCharArray::new(NAMLEN, 1..=NREF);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"None"),
                Val::C(b"Lt"),
                Val::C(b"Lt+s"),
                Val::C(b"Cn"),
                Val::C(b"Cn+s"),
                Val::C(b"Xlt"),
                Val::C(b"Xlt+s"),
                Val::C(b"Xcn"),
                Val::C(b"Xcn+s"),
            ]
            .into_iter();
            ABCS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist =
                [Val::C(b"J2000"), Val::C(b"IAU_MARS"), Val::C(b"IAU_EARTH")].into_iter();
            REFS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"POINT_AT_CENTER"),
                Val::C(b"MISS_BACKWARD"),
                Val::C(b"LIMB_INSIDE_NEAR"),
                Val::C(b"MISS_LIMB_NEAR"),
            ]
            .into_iter();
            GEOMS
                .iter_mut()
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

        Self {
            ABCS,
            GEOMS,
            OBSNMS,
            REFS,
        }
    }
}

//$Procedure      F_SRFXPT ( SRFXPT family tests )
pub fn F_SRFXPT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ABCORR = [b' '; ABCLEN as usize];
    let mut DREF = [b' '; NAMLEN as usize];
    let mut GEOM = [b' '; LNSIZE as usize];
    let mut OBSRVR = [b' '; NAMLEN as usize];
    let mut TARGET = [b' '; NAMLEN as usize];
    let mut TRGFRM = [b' '; NAMLEN as usize];
    let mut TITLE = [b' '; LNSIZE as usize];
    let mut UTC = [b' '; TIMLEN as usize];
    let mut DEPOCH: f64 = 0.0;
    let mut DELTA: f64 = 0.0;
    let mut DIST: f64 = 0.0;
    let mut DJ2 = StackArray::<f64, 3>::new(1..=3);
    let mut DJ2M = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DLT: f64 = 0.0;
    let mut DVEC = StackArray::<f64, 3>::new(1..=3);
    let mut DVECFX = StackArray::<f64, 3>::new(1..=3);
    let mut DVECJ2 = StackArray::<f64, 3>::new(1..=3);
    let mut DVECST = StackArray::<f64, 3>::new(1..=3);
    let mut ELTS = StackArray::<f64, 8>::new(1..=8);
    let mut ET: f64 = 0.0;
    let mut ETOL: f64 = 0.0;
    let mut FRAC: f64 = 0.0;
    let mut J2OBS = StackArray::<f64, 3>::new(1..=3);
    let mut LCENTR = StackArray::<f64, 3>::new(1..=3);
    let mut LIMB = StackArray::<f64, 9>::new(1..=UBEL);
    let mut LT: f64 = 0.0;
    let mut NEGJ2 = StackArray::<f64, 3>::new(1..=3);
    let mut NEGVEC = StackArray::<f64, 3>::new(1..=3);
    let mut OBSJ2 = StackArray::<f64, 3>::new(1..=3);
    let mut OBSPOS = StackArray::<f64, 3>::new(1..=3);
    let mut OBSVEC = StackArray::<f64, 3>::new(1..=3);
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut REFPOS = StackArray::<f64, 3>::new(1..=3);
    let mut RLT: f64 = 0.0;
    let mut SEP: f64 = 0.0;
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut SPJ2 = StackArray::<f64, 3>::new(1..=3);
    let mut SPLT = StackArray::<f64, 3>::new(1..=3);
    let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
    let mut SPPOS = StackArray::<f64, 3>::new(1..=3);
    let mut SPVEC = StackArray::<f64, 3>::new(1..=3);
    let mut SSBOBS = StackArray::<f64, 6>::new(1..=6);
    let mut SSBTRG = StackArray::<f64, 6>::new(1..=6);
    let mut STATE0 = StackArray::<f64, 6>::new(1..=6);
    let mut TE: f64 = 0.0;
    let mut TGT = StackArray::<f64, 3>::new(1..=3);
    let mut TIPFX = StackArray::<f64, 3>::new(1..=3);
    let mut TIPJ2 = StackArray::<f64, 3>::new(1..=3);
    let mut TIPM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TLT: f64 = 0.0;
    let mut TOL: f64 = 0.0;
    let mut TRGEPC: f64 = 0.0;
    let mut TRGJ2M = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut XDIST: f64 = 0.0;
    let mut XEPOCH: f64 = 0.0;
    let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut XLT: f64 = 0.0;
    let mut XOBSPS = StackArray::<f64, 3>::new(1..=3);
    let mut XRAY = StackArray::<f64, 3>::new(1..=3);
    let mut XRTRG = StackArray::<f64, 3>::new(1..=3);
    let mut XSPNT = StackArray::<f64, 3>::new(1..=3);
    let mut XTE: f64 = 0.0;
    let mut CLS: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut DREFID: i32 = 0;
    let mut FRCODE: i32 = 0;
    let mut HANDLE = StackArray::<i32, 2>::new(1..=2);
    let mut N: i32 = 0;
    let mut NITR: i32 = 0;
    let mut OBSCDE: i32 = 0;
    let mut REFCTR: i32 = 0;
    let mut TRGCDE: i32 = 0;
    let mut FND: bool = false;
    let mut FOUND: bool = false;
    let mut USECN: bool = false;
    let mut USELT: bool = false;
    let mut USESTL: bool = false;
    let mut XMIT: bool = false;

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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_SRFXPT", ctx)?;

    testutil::TCASE(b"Setup:  create SPK, PCK file.", ctx)?;

    testutil::TSTSPK(SPK1, true, &mut HANDLE[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the PCK file, load it, and delete it.
    //
    testutil::T_PCK08(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create LSK, load it, and delete it.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set time.
    //
    fstr::assign(&mut UTC, b"2004 FEB 17");
    spicelib::STR2ET(&UTC, &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a Mars orbiter SPK file.
    //
    spicelib::SPKOPN(SPK2, SPK2, 0, &mut HANDLE[2], ctx)?;
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
    ELTS[1] = 3800.0;
    ELTS[2] = 0.1;
    ELTS[3] = (80.0 * spicelib::RPD(ctx));
    ELTS[4] = 0.0;
    ELTS[5] = (90.0 * spicelib::RPD(ctx));
    ELTS[6] = 0.0;
    ELTS[7] = ET;
    ELTS[8] = 42828.314;

    spicelib::CONICS(ELTS.as_slice(), ET, STATE0.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKW05(
        HANDLE[2],
        SCID,
        499,
        b"MARSIAU",
        -((10 as f64) * spicelib::JYEAR()),
        ((10 as f64) * spicelib::JYEAR()),
        b"Mars orbiter",
        ELTS[8],
        1,
        STATE0.as_slice(),
        &[ET],
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKCLS(HANDLE[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the new SPK file.
    //
    spicelib::SPKLEF(SPK2, &mut HANDLE[2], ctx)?;
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
    // Start out with consistency checks:  having found SPOINT,
    // find the aberration corrected location of SPOINT by
    // direct computation, and compare results to those from
    // SRFXPT.
    //
    // Test cases for a distant viewer:  ray emanates from earth's
    // center, points towards Mars' center.

    //
    // Set target.  Get target code, target body-fixed frame
    // name.
    //
    fstr::assign(&mut TARGET, b"Mars");

    spicelib::BODN2C(&TARGET, &mut TRGCDE, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CNMFRM(&TARGET, &mut FRCODE, &mut TRGFRM, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Get target radii.
    //
    spicelib::BODVAR(TRGCDE, b"RADII", &mut N, RADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Loop over every choice of observer.
    //
    for OBSIDX in 1..=NOBS {
        fstr::assign(&mut OBSRVR, save.OBSNMS.get(OBSIDX));
        //
        // Set the observer ID code.
        //
        spicelib::BODN2C(&OBSRVR, &mut OBSCDE, &mut FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Loop over every viewing geometry case.
        //
        for GEOIDX in 1..=NGEOM {
            fstr::assign(&mut GEOM, save.GEOMS.get(GEOIDX));

            //
            // Loop over every aberration correction choice.
            //
            for ABCIDX in 1..=NABC {
                fstr::assign(&mut ABCORR, save.ABCS.get(ABCIDX));

                //
                // Set up some logical variables describing the
                // attributes of the selected correction.
                //
                USELT = fstr::ne(&ABCORR, b"None");
                XMIT = fstr::eq(fstr::substr(&ABCORR, 1..=1), b"X");

                USECN = (fstr::eq(fstr::substr(&ABCORR, 1..=2), b"Cn")
                    || fstr::eq(fstr::substr(&ABCORR, 1..=3), b"Xcn"));

                USESTL = (intrinsics::INDEX(&ABCORR, b"+s") != 0);

                //
                // Loop over every direction vector frame choice.
                //
                for REFIDX in 1..=NREF {
                    fstr::assign(&mut DREF, save.REFS.get(REFIDX));
                    //
                    // Set light time RLT from observer to center of frame
                    // for the direction vector.
                    //
                    spicelib::NAMFRM(&DREF, &mut DREFID, ctx)?;
                    spicelib::FRINFO(DREFID, &mut REFCTR, &mut CLS, &mut CLSSID, &mut FND, ctx)?;

                    spicelib::SPKEZP(
                        REFCTR,
                        ET,
                        b"J2000",
                        &ABCORR,
                        OBSCDE,
                        REFPOS.as_slice_mut(),
                        &mut RLT,
                        ctx,
                    )?;

                    //
                    // We'll need the epoch DEPOCH associated
                    // with the center of DREF.  RLT is the
                    // light time from DREF's center to the observer.
                    //
                    if USELT {
                        if XMIT {
                            DEPOCH = (ET + RLT);
                        } else {
                            DEPOCH = (ET - RLT);
                        }
                    } else {
                        DEPOCH = ET;
                    }

                    //
                    // Look up the transformation from frame DREF to J2000.
                    // We don't need this right away, but we'll have
                    // occasion to use it later.
                    //
                    spicelib::PXFORM(&DREF, b"J2000", DEPOCH, DJ2M.as_slice_mut(), ctx)?;

                    fstr::assign(
                        &mut TITLE,
                        b"Observer = #.  Geometry = #. ABCORR = #; DREF = #.",
                    );
                    spicelib::REPMC(&TITLE.clone(), b"#", &OBSRVR, &mut TITLE);
                    spicelib::REPMC(&TITLE.clone(), b"#", &GEOM, &mut TITLE);
                    spicelib::REPMC(&TITLE.clone(), b"#", &ABCORR, &mut TITLE);
                    spicelib::REPMC(&TITLE.clone(), b"#", &DREF, &mut TITLE);

                    testutil::TCASE(&TITLE, ctx)?;

                    if fstr::eq(&GEOM, b"POINT_AT_CENTER") {
                        //
                        // Look up direction vector using current frame and
                        // aberration correction.  The direction vector is
                        // going to point to the target's center, so we
                        // should hit the target.
                        //
                        spicelib::SPKPOS(
                            &TARGET,
                            ET,
                            &DREF,
                            &ABCORR,
                            &OBSRVR,
                            DVEC.as_slice_mut(),
                            &mut DLT,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    } else if fstr::eq(&GEOM, b"MISS_BACKWARD") {
                        //
                        // Set the pointing direction to the inverse
                        // of that obtained in the 'POINT_AT_CENTER'
                        // case.
                        //
                        spicelib::SPKPOS(
                            &TARGET,
                            ET,
                            &DREF,
                            &ABCORR,
                            &OBSRVR,
                            NEGVEC.as_slice_mut(),
                            &mut DLT,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        spicelib::VMINUS(NEGVEC.as_slice(), DVEC.as_slice_mut());
                    } else if (fstr::eq(&GEOM, b"LIMB_INSIDE_NEAR")
                        || fstr::eq(&GEOM, b"MISS_LIMB_NEAR"))
                    {
                        //
                        // Find the limb of the target based on
                        // the aberration-corrected target center position.
                        // Select ray to hit limb along major axis, slightly
                        // inside or slightly outside the ellipse, depending
                        // on the geometry case.
                        //
                        // Note we're looking up the target state in
                        // the target's body-fixed frame, not in the
                        // DREF frame.
                        //
                        spicelib::SPKPOS(
                            &TARGET,
                            ET,
                            &TRGFRM,
                            &ABCORR,
                            &OBSRVR,
                            NEGVEC.as_slice_mut(),
                            &mut TLT,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        spicelib::VMINUS(NEGVEC.as_slice(), OBSVEC.as_slice_mut());

                        //
                        // Get the limb's center and semi-axis vectors.
                        //
                        spicelib::EDLIMB(
                            RADII[1],
                            RADII[2],
                            RADII[3],
                            OBSVEC.as_slice(),
                            LIMB.as_slice_mut(),
                            ctx,
                        )?;

                        spicelib::EL2CGV(
                            LIMB.as_slice(),
                            LCENTR.as_slice_mut(),
                            SMAJOR.as_slice_mut(),
                            SMINOR.as_slice_mut(),
                        );
                        //
                        // To get an accurate limb, we'll find the light
                        // time from observer to tip of semi-major axis
                        // and get an improved light time estimate.
                        //
                        spicelib::VADD(LCENTR.as_slice(), SMAJOR.as_slice(), TIPFX.as_slice_mut());

                        if USELT {
                            if XMIT {
                                TE = (ET + TLT);
                            } else {
                                TE = (ET - TLT);
                            }
                        } else {
                            TE = ET;
                        }

                        //
                        // Get the "tip" in the J2000 frame at epoch TE.
                        //
                        spicelib::PXFORM(b"J2000", &TRGFRM, TE, TIPM.as_slice_mut(), ctx)?;

                        spicelib::MTXV(TIPM.as_slice(), TIPFX.as_slice(), TIPJ2.as_slice_mut());

                        //
                        // Get state of observer relative to SSB at
                        // ET and state of target relative to SSB at
                        // TE.
                        //
                        spicelib::SPKSSB(OBSCDE, ET, b"J2000", SSBOBS.as_slice_mut(), ctx)?;
                        spicelib::SPKSSB(TRGCDE, TE, b"J2000", SSBTRG.as_slice_mut(), ctx)?;

                        //
                        // Get the position of the tip.  Compute a
                        // new light time value and target epoch.
                        //
                        spicelib::VADD(SSBTRG.as_slice(), TIPJ2.as_slice(), TGT.as_slice_mut());

                        TLT = (spicelib::VDIST(SSBOBS.as_slice(), TGT.as_slice())
                            / spicelib::CLIGHT());

                        if USELT {
                            if XMIT {
                                TE = (ET + TLT);
                            } else {
                                TE = (ET - TLT);
                            }
                        } else {
                            TE = ET;
                        }

                        //
                        // Re-compute TIPM.
                        //
                        spicelib::PXFORM(b"J2000", &TRGFRM, TE, TIPM.as_slice_mut(), ctx)?;

                        //
                        // Get the observer position in the target body-fixed
                        // frame at TE.
                        //
                        spicelib::VSUB(SSBOBS.as_slice(), SSBTRG.as_slice(), OBSJ2.as_slice_mut());
                        spicelib::MXV(TIPM.as_slice(), OBSJ2.as_slice(), OBSVEC.as_slice_mut());

                        //
                        // Get the limb's center and semi-axis vectors.
                        //
                        spicelib::EDLIMB(
                            RADII[1],
                            RADII[2],
                            RADII[3],
                            OBSVEC.as_slice(),
                            LIMB.as_slice_mut(),
                            ctx,
                        )?;

                        spicelib::EL2CGV(
                            LIMB.as_slice(),
                            LCENTR.as_slice_mut(),
                            SMAJOR.as_slice_mut(),
                            SMINOR.as_slice_mut(),
                        );

                        //
                        // Pick our target point near the limb.  The
                        // point is 1+/- DELTA of the semi-major axis length
                        // out from the center, along one of the semi-
                        // major axes.
                        //
                        DELTA = 0.001;

                        if fstr::eq(fstr::substr(&GEOM, 1..=4), b"MISS") {
                            FRAC = (1.0 + DELTA);
                        } else {
                            FRAC = (1.0 - DELTA);
                        }

                        spicelib::VLCOM(
                            1.0,
                            LCENTR.as_slice(),
                            FRAC,
                            SMAJOR.as_slice(),
                            TGT.as_slice_mut(),
                        );

                        //
                        // Our ray extends from the observer to the target
                        // point.
                        //
                        spicelib::VSUB(TGT.as_slice(), OBSVEC.as_slice(), DVECFX.as_slice_mut());

                        SEP = spicelib::VSEP(NEGVEC.as_slice(), DVECFX.as_slice(), ctx);

                        //
                        // Convert the ray from the target body fixed
                        // frame to J2000, then from J2000 to the DREF
                        // frame.  We need the target frame epoch TE
                        // to find the first transformation matrix TIPM.
                        //
                        if USELT {
                            if XMIT {
                                TE = (ET + TLT);
                            } else {
                                TE = (ET - TLT);
                            }
                        } else {
                            TE = ET;
                        }

                        spicelib::PXFORM(b"J2000", &TRGFRM, TE, TIPM.as_slice_mut(), ctx)?;

                        spicelib::MTXV(TIPM.as_slice(), DVECFX.as_slice(), DVECJ2.as_slice_mut());

                        //
                        // If we're using stellar aberration correction,
                        // apply the correction to our ray direction.
                        //
                        if USESTL {
                            if XMIT {
                                spicelib::STLABX(
                                    DVECJ2.as_slice(),
                                    SSBOBS.subarray(4),
                                    DVECST.as_slice_mut(),
                                    ctx,
                                )?;
                            } else {
                                spicelib::STELAB(
                                    DVECJ2.as_slice(),
                                    SSBOBS.subarray(4),
                                    DVECST.as_slice_mut(),
                                    ctx,
                                )?;
                            }

                            spicelib::VEQU(DVECST.as_slice(), DVECJ2.as_slice_mut());
                        }
                        //
                        // The matrix DJ2M maps from DREF to J2000, so
                        // apply the transpose of this matrix to obtain
                        // DVEC.
                        //
                        spicelib::MTXV(DJ2M.as_slice(), DVECJ2.as_slice(), DVEC.as_slice_mut());
                    } else {
                        //
                        // Oops!  Name mismatch.
                        //
                        spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    //
                    // Find the surface intercept point.
                    //
                    spicelib::SRFXPT(
                        b"Ellipsoid",
                        &TARGET,
                        ET,
                        &ABCORR,
                        &OBSRVR,
                        &DREF,
                        DVEC.as_slice(),
                        SPOINT.as_slice_mut(),
                        &mut DIST,
                        &mut TRGEPC,
                        OBSPOS.as_slice_mut(),
                        &mut FOUND,
                        ctx,
                    )?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check the results.
                    //
                    if !FOUND {
                        if fstr::eq(fstr::substr(&GEOM, 1..=4), b"MISS") {
                            //
                            // FOUND should be .FALSE.; the other outputs
                            // are undefined.
                            //
                            testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;
                        } else {
                            //
                            // We're supposed to have an intercept.
                            //
                            testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
                        }
                    } else {
                        //
                        // FOUND is true.
                        //
                        // Form the vector from the observer to the target
                        // intercept.
                        //
                        spicelib::VSUB(SPOINT.as_slice(), OBSPOS.as_slice(), SPVEC.as_slice_mut());

                        //
                        // The length of SPVEC had better be DIST.
                        //
                        testutil::CHCKSD(
                            b"DIST",
                            DIST,
                            b"~",
                            spicelib::VNORM(SPVEC.as_slice()),
                            VTIGHT,
                            OK,
                            ctx,
                        )?;

                        //
                        // The target epoch had better be consistent with
                        // DIST and ABCORR.
                        //
                        XLT = (DIST / spicelib::CLIGHT());

                        if USELT {
                            if XMIT {
                                XEPOCH = (ET + XLT);
                            } else {
                                XEPOCH = (ET - XLT);
                            }
                        } else {
                            XEPOCH = ET;
                        }

                        //
                        // This is a relative error check.
                        //
                        if USECN {
                            ETOL = TIGHT;
                        } else {
                            ETOL = MEDIUM;
                        }

                        testutil::CHCKSD(b"TRGEPC", TRGEPC, b"~/", XEPOCH, ETOL, OK, ctx)?;

                        //
                        // Get the transformation from the target frame
                        // to J2000.
                        //
                        spicelib::PXFORM(&TRGFRM, b"J2000", TRGEPC, TRGJ2M.as_slice_mut(), ctx)?;

                        //
                        // Now transform DVEC to the J2000 frame.
                        //
                        spicelib::MXV(DJ2M.as_slice(), DVEC.as_slice(), DJ2.as_slice_mut());

                        //
                        // The following check applies only to the case in
                        // which the pointing direction is toward the
                        // target's center.
                        //
                        if fstr::eq(&GEOM, b"POINT_AT_CENTER") {
                            //
                            // The angular separation of -OBSPOS and DVEC
                            // should be pretty small when these vectors are
                            // compared in compatible reference frames.  We
                            // don't expect these vectors to be identical
                            // (even theoretically) because they've been
                            // computed with different target epochs.
                            // Furthermore, if stellar aberration correction
                            // is used, DVEC will be corrected for stellar
                            // aberration but OBSPOS will not, leading to even
                            // greater disparity.
                            //
                            // First step:  get -OBSPOS in the J2000 frame.
                            //

                            spicelib::VMINUS(OBSPOS.as_slice(), NEGVEC.as_slice_mut());

                            spicelib::MXV(
                                TRGJ2M.as_slice(),
                                NEGVEC.as_slice(),
                                NEGJ2.as_slice_mut(),
                            );

                            //
                            // Find the angular separation and make sure it's
                            // not too large.
                            //
                            SEP = spicelib::VSEP(NEGJ2.as_slice(), DJ2.as_slice(), ctx);

                            testutil::CHCKSD(b"DJ2 vs NEG2 SEP", SEP, b"~", 0.0, SLOPPY, OK, ctx)?;
                        }
                        //
                        // End of sanity check test for the POINT_AT_CENTER
                        // case.
                        //
                        // Having made it this far, we're ready for some more
                        // rigorous tests.  In particular, we're going treat
                        // SPOINT as an ephemeris object and find its
                        // aberration-corrected position relative to the
                        // observer in J2000 coordinates. This computation
                        // will allow us to derive expected values of TRGEPC,
                        // OBSPOS, the transformation from the J2000 frame to
                        // the target body-fixed frame at TRGEPC.  We will
                        // verify that the aberration-corrected location
                        // of SPOINT, lies on the ray DVEC:  this is the
                        // the criterion we used to define SPOINT.
                        //
                        // These tests make sense only when aberration
                        // corrections are used.
                        //
                        if USELT {
                            //
                            // We're expecting to get good agreement between
                            // all of these items and their counterparts
                            // obtained from SRFXPT, especially when use use
                            // converged Newtonian corrections.
                            //
                            if spicelib::EQSTR(&OBSRVR, b"EARTH") {
                                if USECN {
                                    TOL = TIGHT;
                                } else {
                                    TOL = MEDIUM;
                                }
                            } else {
                                //
                                // Use looser tolerances for the Mars
                                // orbiter.  For the orbiter, small errors
                                // in SPOINT lead to larger relative errors
                                // in DIST and SEP.
                                //
                                if USECN {
                                    TOL = MTIGHT;
                                } else {
                                    TOL = LOOSE;
                                }
                            }

                            //
                            // Find the aberration-corrected location of
                            // SPOINT.
                            //
                            // We need the J2000 state of the observer relative
                            // to the solar system barycenter at ET.
                            //
                            spicelib::SPKSSB(OBSCDE, ET, b"J2000", SSBOBS.as_slice_mut(), ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // The SPOINT re-computation is done iteratively.
                            // Since we're starting with a geometric target
                            // position, the light time value obtained from
                            // the normal light time correction corresponds to
                            // the light time found on the *second* iteration.
                            // We increment our iteration counts by 1 for both
                            // the normal and CN cases.
                            //
                            if USECN {
                                NITR = 5;
                            } else {
                                NITR = 2;
                            }

                            //
                            // The initial target position relative to the
                            // solar system barycenter is found by summing the
                            // target center position relative to the solar
                            // system barycenter at ET with SPOINT, after
                            // SPOINT has been converted to the J2000 frame at
                            // ET.
                            //
                            spicelib::SPKSSB(TRGCDE, ET, b"J2000", SSBTRG.as_slice_mut(), ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            spicelib::PXFORM(b"J2000", &TRGFRM, ET, TIPM.as_slice_mut(), ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            spicelib::MTXV(TIPM.as_slice(), SPOINT.as_slice(), SPJ2.as_slice_mut());
                            spicelib::VADD(
                                SSBTRG.as_slice(),
                                SPJ2.as_slice(),
                                SPPOS.as_slice_mut(),
                            );

                            for I in 1..=NITR {
                                //
                                // Make a new estimate of the target epoch XTE.
                                //
                                LT = (spicelib::VDIST(SSBOBS.as_slice(), SPPOS.as_slice())
                                    / spicelib::CLIGHT());

                                if XMIT {
                                    XTE = (ET + LT);
                                } else {
                                    XTE = (ET - LT);
                                }

                                //
                                // Compute the J2000 state of SPOINT relative
                                // to the solar system barycenter at XTE.
                                //
                                spicelib::SPKSSB(
                                    TRGCDE,
                                    XTE,
                                    b"J2000",
                                    SSBTRG.as_slice_mut(),
                                    ctx,
                                )?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                spicelib::PXFORM(b"J2000", &TRGFRM, XTE, TIPM.as_slice_mut(), ctx)?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                testutil::CHCKXC(false, b" ", OK, ctx)?;

                                spicelib::MTXV(
                                    TIPM.as_slice(),
                                    SPOINT.as_slice(),
                                    SPJ2.as_slice_mut(),
                                );
                                spicelib::VADD(
                                    SSBTRG.as_slice(),
                                    SPJ2.as_slice(),
                                    SPPOS.as_slice_mut(),
                                );
                            }
                            //
                            // Compute the light-time corrected position of
                            // SPOINT as seen by the observer.
                            //
                            spicelib::VSUB(
                                SPPOS.as_slice(),
                                SSBOBS.as_slice(),
                                SPLT.as_slice_mut(),
                            );
                            //
                            // Correct SPLT for stellar aberration, if ABCORR
                            // so indicates.
                            //
                            if USESTL {
                                if XMIT {
                                    spicelib::STLABX(
                                        SPLT.as_slice(),
                                        SSBOBS.subarray(4),
                                        XRAY.as_slice_mut(),
                                        ctx,
                                    )?;
                                } else {
                                    spicelib::STELAB(
                                        SPLT.as_slice(),
                                        SSBOBS.subarray(4),
                                        XRAY.as_slice_mut(),
                                        ctx,
                                    )?;
                                }
                            } else {
                                //
                                // XRAY is our expected result.
                                //
                                spicelib::VEQU(SPLT.as_slice(), XRAY.as_slice_mut());
                            }

                            //
                            // Moment of truth:  XRAY is the J2000 vector from
                            // the observer to the aberration-corrected
                            // position of our "ephemeris object" located on
                            // the target surface at location SPOINT.  If
                            // SPOINT were correct in the first place, then
                            // XRAY should be lined up with our boresight
                            // direction DVEC, when DVEC is rotated to the
                            // J2000 frame.
                            //
                            // Actually, we computed DVEC in the J2000 frame
                            // long ago:  this vector is called DJ2.
                            //
                            testutil::CHCKSD(b"TRGEPC vs XTE", TRGEPC, b"~/", XTE, TOL, OK, ctx)?;

                            SEP = spicelib::VSEP(DJ2.as_slice(), XRAY.as_slice(), ctx);

                            testutil::CHCKSD(b"XRAY vs DJ2 sep", SEP, b"~", 0.0, TOL, OK, ctx)?;

                            //
                            // Check DIST against its predicted value.
                            //
                            testutil::CHCKSD(
                                b"DIST",
                                DIST,
                                b"~/",
                                spicelib::VNORM(XRAY.as_slice()),
                                TOL,
                                OK,
                                ctx,
                            )?;

                            //
                            // Create a predicted value for OBSPOS:  the
                            // difference of SSBOBS and SSBTRG from our last
                            // loop iteration gives us OBSPOS in the J2000
                            // frame.  Use XTE to get the transformation to
                            // target body-fixed coordinates at epoch XTE.
                            //
                            spicelib::VSUB(
                                SSBOBS.as_slice(),
                                SSBTRG.as_slice(),
                                J2OBS.as_slice_mut(),
                            );
                            spicelib::PXFORM(b"J2000", &TRGFRM, XTE, XFORM.as_slice_mut(), ctx)?;
                            spicelib::MXV(
                                XFORM.as_slice(),
                                J2OBS.as_slice(),
                                XOBSPS.as_slice_mut(),
                            );

                            testutil::CHCKAD(
                                b"OBSPOS",
                                OBSPOS.as_slice(),
                                b"~/",
                                XOBSPS.as_slice(),
                                3,
                                TOL,
                                OK,
                                ctx,
                            )?;

                            //
                            // The following test only makes sense
                            // when stellar aberration is not used.
                            // (When stellar aberration IS used, SPOINT
                            // does not lie on the ray; the image of
                            // SPOINT under the stellar aberration correction
                            // lies on the ray.
                            //
                            if !USESTL {
                                //
                                // Create a predicted value for SPOINT:  convert
                                // XRAY to target body-fixed coordinates at
                                // epoch XTE and add to XOBSPS to form XSPNT.
                                //
                                spicelib::MXV(
                                    XFORM.as_slice(),
                                    XRAY.as_slice(),
                                    XRTRG.as_slice_mut(),
                                );
                                spicelib::VADD(
                                    XRTRG.as_slice(),
                                    XOBSPS.as_slice(),
                                    XSPNT.as_slice_mut(),
                                );

                                testutil::CHCKAD(
                                    b"SPOINT",
                                    SPOINT.as_slice(),
                                    b"~/",
                                    XSPNT.as_slice(),
                                    3,
                                    MEDIUM,
                                    OK,
                                    ctx,
                                )?;
                            }
                        }
                        //
                        // We're finished with the consistency checks for
                        // the aberration correction cases.
                        //
                    }
                    //
                    // End of the checks for the intercept cases.
                    //
                }
                //
                // End of the aberration correction loop.
                //
            }
            //
            // End of the reference frame loop.
            //
        }
        //
        // End of the geometry case loop.
        //
    }
    //
    // End of the observer loop.
    //

    //
    // Input handling tests:  make sure target and observer
    // can be identified using integer "names."
    //
    testutil::TCASE(b"Use integer observer and target names.", ctx)?;
    //
    // Set up the ray first.
    //
    spicelib::SPKPOS(
        b"499",
        ET,
        &DREF,
        &ABCORR,
        b"399",
        DVEC.as_slice_mut(),
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SRFXPT(
        b"Ellipsoid",
        b"MARS",
        ET,
        &ABCORR,
        b"Earth",
        &DREF,
        DVEC.as_slice(),
        XSPNT.as_slice_mut(),
        &mut XDIST,
        &mut XEPOCH,
        XOBSPS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::SRFXPT(
        b"Ellipsoid",
        b"499",
        ET,
        &ABCORR,
        b"399",
        &DREF,
        DVEC.as_slice(),
        SPOINT.as_slice_mut(),
        &mut DIST,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    testutil::CHCKAD(
        b"SPOINT",
        SPOINT.as_slice(),
        b"=",
        XSPNT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"OBSPOS",
        OBSPOS.as_slice(),
        b"=",
        XOBSPS.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"TRGEPC", TRGEPC, b"=", XEPOCH, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"DIST", DIST, b"=", DIST, 0.0, OK, ctx)?;

    //
    // Error handling tests follow.
    //
    testutil::TCASE(b"Target name not translated", ctx)?;
    //
    // Find the surface intercept point.
    //
    spicelib::SRFXPT(
        b"Ellipsoid",
        b"xyz",
        ET,
        &ABCORR,
        &OBSRVR,
        &DREF,
        DVEC.as_slice(),
        SPOINT.as_slice_mut(),
        &mut DIST,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Observer name not translated", ctx)?;
    //
    // Find the surface intercept point.
    //
    spicelib::SRFXPT(
        b"Ellipsoid",
        &TARGET,
        ET,
        &ABCORR,
        b"xyz",
        &DREF,
        DVEC.as_slice(),
        SPOINT.as_slice_mut(),
        &mut DIST,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    testutil::TCASE(b"Observer coincides with target", ctx)?;
    //
    // Find the surface intercept point.
    //
    spicelib::SRFXPT(
        b"Ellipsoid",
        &TARGET,
        ET,
        &ABCORR,
        &TARGET,
        &DREF,
        DVEC.as_slice(),
        SPOINT.as_slice_mut(),
        &mut DIST,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    testutil::TCASE(b"Unsupported computation method", ctx)?;
    //
    // Find the surface intercept point.
    //
    spicelib::SRFXPT(
        b"xyz",
        &TARGET,
        ET,
        &ABCORR,
        &OBSRVR,
        &DREF,
        DVEC.as_slice(),
        SPOINT.as_slice_mut(),
        &mut DIST,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDMETHOD)", OK, ctx)?;

    testutil::TCASE(b"No body-fixed frame associated with target", ctx)?;
    //
    // Find the surface intercept point.
    //
    spicelib::SRFXPT(
        b"Ellipsoid",
        b"Mars_orbiter",
        ET,
        &ABCORR,
        b"EARTH",
        &DREF,
        DVEC.as_slice(),
        SPOINT.as_slice_mut(),
        &mut DIST,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOFRAME)", OK, ctx)?;

    testutil::TCASE(
        b"Frame name maps to code but FRINFO can\'t find frame info.",
        ctx,
    )?;
    //
    // Find the surface intercept point.  Use reference frame 'BAD'
    // for direction vector.
    //
    spicelib::SRFXPT(
        b"Ellipsoid",
        b"Mars_orbiter",
        ET,
        &ABCORR,
        b"EARTH",
        b"BAD",
        DVEC.as_slice(),
        SPOINT.as_slice_mut(),
        &mut DIST,
        &mut TRGEPC,
        OBSPOS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOFRAME)", OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::SPKUEF(HANDLE[1], ctx)?;
    spicelib::DELFIL(SPK1, ctx)?;

    spicelib::SPKUEF(HANDLE[2], ctx)?;
    spicelib::DELFIL(SPK2, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
