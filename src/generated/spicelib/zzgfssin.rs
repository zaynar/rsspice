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
const TDELTA: f64 = 1.0;
const BDNMLN: i32 = 36;

struct SaveVars {
    SVOBS: Vec<u8>,
    SVTARG: Vec<u8>,
    PRVOBS: i32,
    PRVTRG: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVOBS = vec![b' '; BDNMLN as usize];
        let mut SVTARG = vec![b' '; BDNMLN as usize];
        let mut PRVOBS: i32 = 0;
        let mut PRVTRG: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;
        PRVOBS = 0;
        PRVTRG = 0;
        fstr::assign(&mut SVOBS, b" ");
        fstr::assign(&mut SVTARG, b" ");

        Self {
            SVOBS,
            SVTARG,
            PRVOBS,
            PRVTRG,
            FIRST,
        }
    }
}

//$Procedure      ZZGFSSIN ( GF, state of surface intercept point )
pub fn ZZGFSSIN(
    METHOD: &[u8],
    TRGID: i32,
    ET: f64,
    FIXREF: &[u8],
    ABCORR: &[u8],
    OBSID: i32,
    DREF: &[u8],
    DCTR: i32,
    DVEC: &[f64],
    RADII: &[f64],
    STATE: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DVEC = DummyArray::new(DVEC, 1..=3);
    let RADII = DummyArray::new(RADII, 1..=3);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut ACC = StackArray::<f64, 3>::new(1..=3);
    let mut CORXFI = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut CORXFM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut CTRSTA = StackArray::<f64, 6>::new(1..=6);
    let mut DCORXF = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut DLT: f64 = 0.0;
    let mut DLTCTR: f64 = 0.0;
    let mut DRFEPC: f64 = 0.0;
    let mut DRXFRM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut FXDSTA = StackArray::<f64, 6>::new(1..=6);
    let mut FXOSTA = StackArray::<f64, 6>::new(1..=6);
    let mut FXPSTA = StackArray::<f64, 6>::new(1..=6);
    let mut FXPVEL = StackArray::<f64, 3>::new(1..=3);
    let mut FXTSTA = StackArray::<f64, 6>::new(1..=6);
    let mut J2DSTA = StackArray::<f64, 6>::new(1..=6);
    let mut LT: f64 = 0.0;
    let mut LTCTR: f64 = 0.0;
    let mut LTSIGN: f64 = 0.0;
    let mut OBSPNT = StackArray::<f64, 6>::new(1..=6);
    let mut OBSSTA = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut OBSTRG = StackArray::<f64, 6>::new(1..=6);
    let mut PNTSTA = StackArray::<f64, 6>::new(1..=6);
    let mut SA = StackArray::<f64, 3>::new(1..=3);
    let mut SASTAT = StackArray::<f64, 6>::new(1..=6);
    let mut SAVEL = StackArray::<f64, 3>::new(1..=3);
    let mut SCALE: f64 = 0.0;
    let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
    let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
    let mut SSBOBS = StackArray::<f64, 6>::new(1..=6);
    let mut SSBTG0 = StackArray::<f64, 6>::new(1..=6);
    let mut SSBTRG = StackArray::<f64, 6>::new(1..=6);
    let mut STEMP = StackArray::<f64, 6>::new(1..=6);
    let mut T: f64 = 0.0;
    let mut TRGEPC: f64 = 0.0;
    let mut UPOS = StackArray::<f64, 3>::new(1..=3);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut CENTER: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut FRCODE: i32 = 0;
    let mut FRCLSS: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 6>::new(1..=ABATSZ);
    let mut FND: bool = false;
    let mut GEOM: bool = false;
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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFSSIN", ctx)?;

    //
    // No result has been found.
    //
    *FOUND = false;

    if (save.FIRST || (TRGID != save.PRVTRG)) {
        BODC2S(TRGID, &mut save.SVTARG, ctx)?;

        save.PRVTRG = TRGID;
    }

    if (save.FIRST || (OBSID != save.PRVOBS)) {
        BODC2S(OBSID, &mut save.SVOBS, ctx)?;

        save.PRVOBS = OBSID;
    }

    save.FIRST = false;

    //
    // Parse the aberration correction specifier.
    //
    ZZVALCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFSSIN", ctx)?;
        return Ok(());
    }

    GEOM = ATTBLK[GEOIDX];
    USELT = ATTBLK[LTIDX];
    USESTL = ATTBLK[STLIDX];
    XMIT = ATTBLK[XMTIDX];

    //
    // Set the sign associated with the light time correction.
    //
    if XMIT {
        LTSIGN = 1.0;
    } else {
        LTSIGN = -1.0;
    }

    //
    // Decide whether the surface intercept point is computed using
    // the "near point" or "surface intercept" method. Only
    // ellipsoids may be used a shape models for this computation.
    //
    if !EQSTR(METHOD, b"Ellipsoid") {
        SETMSG(
            b"Surface intercept point computation method # is not supported by this routine.",
            ctx,
        );
        ERRCH(b"#", METHOD, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZGFSSIN", ctx)?;
        return Ok(());
    }

    if GEOM {
        //
        // This is the geometric case.
        //
        // No light time correction is involved, so all frames are
        // evaluated at the observation epoch.
        //
        // Compute the state transformation from DREF to J2000.
        //
        SXFORM(DREF, b"J2000", ET, DCORXF.as_slice_mut(), ctx)?;

        //
        // Transform the ray's direction vector from DREF to the J2000
        // frame. The velocity of DVEC in frame DREF is zero.
        //
        MOVED(DVEC.as_slice(), 3, STEMP.as_slice_mut());
        CLEARD(3, STEMP.subarray_mut(4));

        MXVG(
            DCORXF.as_slice(),
            STEMP.as_slice(),
            6,
            6,
            J2DSTA.as_slice_mut(),
        );

        //
        // We need to check the body-fixed reference frame here.
        //
        NAMFRM(FIXREF, &mut FRCODE, ctx)?;
        FRINFO(FRCODE, &mut CENTER, &mut FRCLSS, &mut CLSSID, &mut FND, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFSSIN", ctx)?;
            return Ok(());
        }

        if !FND {
            SETMSG(b"Input reference frame # was not recognized.", ctx);
            ERRCH(b"#", FIXREF, ctx);
            SIGERR(b"SPICE(NOFRAME)", ctx)?;
            CHKOUT(b"ZZGFSSIN", ctx)?;
            return Ok(());
        }

        if (CENTER != TRGID) {
            SETMSG(
                b"Input reference frame # is centered on body # instead of body #.",
                ctx,
            );
            ERRCH(b"#", FIXREF, ctx);
            ERRINT(b"#", CENTER, ctx);
            ERRINT(b"#", TRGID, ctx);
            SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
            CHKOUT(b"ZZGFSSIN", ctx)?;
            return Ok(());
        }

        // Get the state of the target with respect to the observer,
        // expressed relative to the target body-fixed frame. We don't
        // need to propagate states to the solar system barycenter in
        // this case.
        //
        SPKGEO(
            TRGID,
            ET,
            FIXREF,
            OBSID,
            FXTSTA.as_slice_mut(),
            &mut LT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFSSIN", ctx)?;
            return Ok(());
        }

        //
        // Compute the state of the observer with respect to the target
        // in the body-fixed frame.
        //
        VMINUG(FXTSTA.as_slice(), 6, FXOSTA.as_slice_mut());

        //
        // Transform the state of the direction vector from the J2000
        // frame to the target body-fixed frame at TRGEPC. Since no
        // light time corrections are involved, the state transformation
        // matrix from SXFORM works just fine.
        //
        SXFORM(b"J2000", FIXREF, ET, XFORM.as_slice_mut(), ctx)?;
        MXVG(
            XFORM.as_slice(),
            J2DSTA.as_slice(),
            6,
            6,
            FXDSTA.as_slice_mut(),
        );

        //
        // Now we can obtain the surface velocity of the surface intercept
        // point.
        //
        SURFPV(
            FXOSTA.as_slice(),
            FXDSTA.as_slice(),
            RADII[1],
            RADII[2],
            RADII[3],
            FXPSTA.as_slice_mut(),
            FOUND,
            ctx,
        )?;

        //
        // It's not an error for SURFPV to be unable to compute an
        // intercept state; return now if the state was not
        // computable.
        //
        if !*FOUND {
            CHKOUT(b"ZZGFSSIN", ctx)?;
            return Ok(());
        }
    } else if USELT {
        //
        // Light time and possibly stellar aberration corrections
        // are applied.
        //
        // Compute the state transformation from DREF to J2000.
        //
        if (OBSID == DCTR) {
            //
            // DREF is centered on the observer, so there's no light time
            // correction.
            //
            SXFORM(DREF, b"J2000", ET, DCORXF.as_slice_mut(), ctx)?;
        } else {
            //
            // Find the epoch DRFEPC associated with the input direction
            // vector's reference frame DREF. We use SPK rules for
            // determining the epoch, just as in SINCPT. Let DLTCTR be the
            // rate of change of light time between the frame center and
            // the observer.
            //
            //
            // Find the light time from the observer to the center of
            // frame DREF.
            //
            SPKACS(
                DCTR,
                ET,
                b"J2000",
                ABCORR,
                OBSID,
                CTRSTA.as_slice_mut(),
                &mut LTCTR,
                &mut DLTCTR,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZGFSSIN", ctx)?;
                return Ok(());
            }

            DRFEPC = (ET + (LTSIGN * LTCTR));

            //
            // Compute the state of the input direction vector in the
            // J2000 frame at DRFEPC. Correct the state transformation for
            // the rate of change of light time.
            //
            SXFORM(DREF, b"J2000", DRFEPC, DRXFRM.as_slice_mut(), ctx)?;
            ZZCORSXF(XMIT, DLTCTR, DRXFRM.as_slice(), DCORXF.as_slice_mut());
        }

        //
        // The velocity of DVEC in frame DREF is zero.
        //
        MOVED(DVEC.as_slice(), 3, STEMP.as_slice_mut());
        CLEARD(3, STEMP.subarray_mut(4));

        MXVG(
            DCORXF.as_slice(),
            STEMP.as_slice(),
            6,
            6,
            J2DSTA.as_slice_mut(),
        );

        //
        // We'll transform J2DSTA to the target body-fixed frame at
        // the target epoch once we've computed the required
        // state transformation matrix. This occurs just before
        // we use this state vector in a call to SURFPV.
        //
        // Most our work consists of getting ready to call the SPICELIB
        // routine SURFPV. In order to make this call, we'll need the
        // velocity of the observer relative to the target body's center
        // in the target body-fixed frame. We must evaluate the rotation
        // state of the target at the correct epoch, and account for the
        // rate of change of light time, if light time corrections are
        // used. The algorithm we use depends on the algorithm used in
        // SINCPT, since we're computing the derivative with respect to
        // time of the solution found by that routine.
        //
        // In this algorithm, we must take into account the fact that
        // SINCPT performs light time and stellar aberration corrections
        // for the surface intercept point, not for the center of the
        // target body.
        //
        // If light time and stellar aberration corrections are used,
        //
        // - Find the aberration corrected surface intercept point and
        //   the light time-corrected epoch TRGEPC associated
        //   with the surface intercept point.
        //
        // - Use TRGEPC to find the position of the target relative
        //   to the solar system barycenter.
        //
        // - Use TRGEPC to find the orientation of the target relative
        //   to the J2000 reference frame.
        //
        // - Find the light-time corrected position of the
        //   surface intercept point; use this to compute the
        //   stellar aberration offset that applies to the
        //   surface intercept point, as well as the velocity of
        //   this offset.
        //
        // - Find the corrected state of the target center as seen
        //   from the observer, where the corrections are those
        //   applicable to the surface intercept point.
        //
        // - Negate the corrected target center state to obtain
        //   the state of the observer relative to the target.
        //
        // - Express the state of the observer relative to the
        //   target in the target body fixed frame at TRGEPC.
        //
        //
        // Below, we'll use the convention that vectors expressed
        // relative to the body-fixed frame have names of the form
        //
        // FX*
        //
        // Note that SINCPT will signal an error if FIXREF is not
        // actually centered on the target body.
        //
        SINCPT(
            METHOD,
            &save.SVTARG,
            ET,
            FIXREF,
            ABCORR,
            &save.SVOBS,
            DREF,
            DVEC.as_slice(),
            SPOINT.as_slice_mut(),
            &mut TRGEPC,
            SRFVEC.as_slice_mut(),
            FOUND,
            ctx,
        )?;
        //
        // It's not an error for SINCPT to be unable to compute an
        // intercept point; return now if the intercept was not found.
        //
        if !*FOUND {
            CHKOUT(b"ZZGFSSIN", ctx)?;
            return Ok(());
        }

        //
        // Get J2000-relative states of observer and target with respect
        // to the solar system barycenter at their respective epochs of
        // participation.
        //
        SPKSSB(OBSID, ET, b"J2000", SSBOBS.as_slice_mut(), ctx)?;
        SPKSSB(TRGID, TRGEPC, b"J2000", SSBTG0.as_slice_mut(), ctx)?;

        //
        // Get the uncorrected J2000 to body-fixed to state
        // transformation at TRGEPC.
        //
        SXFORM(b"J2000", FIXREF, TRGEPC, XFORM.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFSSIN", ctx)?;
            return Ok(());
        }

        //
        // Initialize the state of the surface intercept point in the
        // body-fixed frame. At this point we don't know the point's
        // velocity; set it to zero.
        //
        MOVED(SPOINT.as_slice(), 3, FXPSTA.as_slice_mut());
        CLEARD(3, FXPSTA.subarray_mut(4));

        if USESTL {
            //
            // We're going to need the acceleration of the observer
            // relative to the SSB. Compute this now.
            //
            for I in 1..=2 {
                //
                // The epoch is ET -/+ TDELTA.
                //
                T = (ET + ((((2 * I) - 3) as f64) * TDELTA));

                SPKSSB(OBSID, T, b"J2000", OBSSTA.subarray_mut([1, I]), ctx)?;
            }

            if FAILED(ctx) {
                CHKOUT(b"ZZGFSSIN", ctx)?;
                return Ok(());
            }

            //
            // Compute the observer's acceleration using a quadratic
            // approximation.
            //
            QDERIV(
                3,
                OBSSTA.subarray([4, 1]),
                OBSSTA.subarray([4, 2]),
                TDELTA,
                ACC.as_slice_mut(),
                ctx,
            )?;
        }

        //
        // The rest of the algorithm is iterative. On the first
        // iteration, we don't have a good estimate of the velocity
        // of the surface intercept point relative to the body-fixed
        // frame. Since we're using this velocity as an input
        // to the aberration velocity computations, we
        // expect that treating this velocity as zero on the first
        // pass yields a reasonable estimate. On the second pass,
        // we'll use the velocity derived on the first pass.
        //
        CLEARD(3, FXPVEL.as_slice_mut());

        //
        // We'll also estimate the rate of change of light time
        // as zero on the first pass.
        //
        DLT = 0.0;

        for I in 1..=3 {
            //
            // Correct the target's velocity for the rate of
            // change of light time.
            //
            if XMIT {
                SCALE = (1.0 + DLT);
            } else {
                SCALE = (1.0 - DLT);
            }

            //
            // Scale the velocity portion of the target state to
            // correct the velocity for the rate of change of light
            // time.
            //
            MOVED(SSBTG0.as_slice(), 3, SSBTRG.as_slice_mut());
            VSCL(SCALE, SSBTG0.subarray(4), SSBTRG.subarray_mut(4));

            //
            // Get the state of the target with respect to the observer.
            //
            VSUBG(
                SSBTRG.as_slice(),
                SSBOBS.as_slice(),
                6,
                OBSTRG.as_slice_mut(),
            );

            //
            // Correct the J2000 to body-fixed state transformation matrix
            // for the rate of change of light time.
            //
            ZZCORSXF(XMIT, DLT, XFORM.as_slice(), CORXFM.as_slice_mut());

            //
            // Invert CORXFM to obtain the corrected
            // body-fixed to J2000 state transformation.
            //
            INVSTM(CORXFM.as_slice(), CORXFI.as_slice_mut(), ctx)?;

            //
            // Convert the surface intercept point state to the J2000
            // frame.
            //
            MXVG(
                CORXFI.as_slice(),
                FXPSTA.as_slice(),
                6,
                6,
                PNTSTA.as_slice_mut(),
            );

            //
            // Find the J2000-relative state of the surface intercept
            // point with respect to the target.
            //
            VADDG(
                OBSTRG.as_slice(),
                PNTSTA.as_slice(),
                6,
                OBSPNT.as_slice_mut(),
            );

            if USESTL {
                //
                // Now compute the stellar aberration correction
                // applicable to OBSPNT. We need the velocity of
                // this correction as well.
                //
                ZZSTELAB(
                    XMIT,
                    ACC.as_slice(),
                    SSBOBS.subarray(4),
                    OBSPNT.as_slice(),
                    SA.as_slice_mut(),
                    SAVEL.as_slice_mut(),
                    ctx,
                )?;

                MOVED(SA.as_slice(), 3, SASTAT.as_slice_mut());
                MOVED(SAVEL.as_slice(), 3, SASTAT.subarray_mut(4));
                //
                // Adding the stellar aberration state to the target center
                // state gives us the state of the target center with
                // respect to the observer, corrected for the aberrations
                // applicable to the surface intercept point.
                //
                VADDG(
                    OBSTRG.as_slice(),
                    SASTAT.as_slice(),
                    6,
                    STEMP.as_slice_mut(),
                );
            } else {
                MOVED(OBSTRG.as_slice(), 6, STEMP.as_slice_mut());
            }

            //
            // Convert STEMP to the body-fixed reference frame.
            //
            MXVG(
                CORXFM.as_slice(),
                STEMP.as_slice(),
                6,
                6,
                FXTSTA.as_slice_mut(),
            );

            //
            // At long last, compute the state of the observer
            // with respect to the target in the body-fixed frame.
            //
            VMINUG(FXTSTA.as_slice(), 6, FXOSTA.as_slice_mut());

            //
            // Transform the state of the direction vector from the
            // J2000 frame to the target body-fixed frame at TRGEPC.
            //
            MXVG(
                CORXFM.as_slice(),
                J2DSTA.as_slice(),
                6,
                6,
                FXDSTA.as_slice_mut(),
            );

            //
            // Now we can obtain the surface velocity of the
            // surface intercept point.
            //
            SURFPV(
                FXOSTA.as_slice(),
                FXDSTA.as_slice(),
                RADII[1],
                RADII[2],
                RADII[3],
                FXPSTA.as_slice_mut(),
                FOUND,
                ctx,
            )?;

            //
            // It's not an error for SURFPV to be unable to compute an
            // intercept state; return now if the state was not
            // computable.
            //
            if !*FOUND {
                CHKOUT(b"ZZGFSSIN", ctx)?;
                return Ok(());
            }

            //
            // At this point we can update the surface point
            // velocity and light time derivative estimates.
            //
            // In order to compute the light time rate, we'll
            // need the J2000-relative velocity of the surface intercept
            // point with respect to the observer. First convert
            // the surface intercept state to the J2000 frame, then
            // add the result to the state of the target center
            // with respect to the observer.
            //
            MXVG(
                CORXFI.as_slice(),
                FXPSTA.as_slice(),
                6,
                6,
                PNTSTA.as_slice_mut(),
            );
            VADDG(
                OBSTRG.as_slice(),
                PNTSTA.as_slice(),
                6,
                OBSPNT.as_slice_mut(),
            );

            //
            // Now that we have an improved estimate of the
            // surface intercept state, we can estimate the rate of
            // change of light time as
            //
            //    range rate
            //    ----------
            //        c
            //
            //
            // If we're correcting for stellar aberration, *ideally* we
            // should remove that correction now, since the light time
            // rate is based on light time between the observer and the
            // light-time corrected surface intercept point. But the error
            // made by including stellar aberration is too small to make
            // it worthwhile to make this adjustment.
            //
            VHAT(OBSPNT.as_slice(), UPOS.as_slice_mut());

            DLT = (VDOT(OBSPNT.subarray(4), UPOS.as_slice()) / CLIGHT());

            //
            // With FXPVEL and DLT updated, we'll repeat our
            // computations.
            //
        }
    } else {
        //
        // We should never get here.
        //
        SETMSG(b"Aberration correction # was not recognized.", ctx);
        ERRCH(b"#", ABCORR, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZGFSSIN", ctx)?;
        return Ok(());
    }

    //
    // Copy the computed state to the output argument STATE.
    // FOUND has already been set to .TRUE. by SURFPV.
    //
    MOVED(FXPSTA.as_slice(), 6, STATE.as_slice_mut());

    CHKOUT(b"ZZGFSSIN", ctx)?;
    Ok(())
}
