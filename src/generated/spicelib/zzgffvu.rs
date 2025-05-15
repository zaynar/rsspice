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
const TOTAL1: i32 = -3;
const ANNLR1: i32 = -2;
const PARTL1: i32 = -1;
const NOOCC: i32 = 0;
const PARTL2: i32 = 1;
const ANNLR2: i32 = 2;
const TOTAL2: i32 = 3;
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
const UBEL: i32 = 9;
const UBPL: i32 = 4;
const ATOL: f64 = 0.000001;
const FVEMAX: f64 = 0.00001;
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;

struct SaveVars {
    SVIFRM: Vec<u8>,
    SVINAM: Vec<u8>,
    SVISHP: Vec<u8>,
    SVTFRM: Vec<u8>,
    SVTNAM: Vec<u8>,
    SVTSHP: Vec<u8>,
    SVCORR: Vec<u8>,
    SVARAD: f64,
    SVBNDS: ActualArray2D<f64>,
    SVEDCT: StackArray<f64, 3>,
    SVFAXI: StackArray<f64, 3>,
    SVFOVM: StackArray2D<f64, 9>,
    SVFPOL: ActualArray2D<f64>,
    SVFSMX: StackArray2D<f64, 9>,
    SVFVCT: StackArray<f64, 3>,
    SVORIG: StackArray<f64, 3>,
    SVPLAN: StackArray<f64, 4>,
    SVRDIR: StackArray<f64, 3>,
    SVSEMI: StackArray2D<f64, 6>,
    SVTRAD: StackArray<f64, 3>,
    SVXMAG: StackArray<f64, 2>,
    SVINST: i32,
    SVNVRT: i32,
    SVOBS: i32,
    SVTARG: i32,
    SVETRG: bool,
    SVURAY: bool,
    SVUSTL: bool,
    SVXMIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVIFRM = vec![b' '; FRNMLN as usize];
        let mut SVINAM = vec![b' '; BDNMLN as usize];
        let mut SVISHP = vec![b' '; SHPLEN as usize];
        let mut SVTFRM = vec![b' '; FRNMLN as usize];
        let mut SVTNAM = vec![b' '; BDNMLN as usize];
        let mut SVTSHP = vec![b' '; SHPLEN as usize];
        let mut SVCORR = vec![b' '; CORLEN as usize];
        let mut SVARAD: f64 = 0.0;
        let mut SVBNDS = ActualArray2D::<f64>::new(1..=3, 1..=MAXVRT);
        let mut SVEDCT = StackArray::<f64, 3>::new(1..=3);
        let mut SVFAXI = StackArray::<f64, 3>::new(1..=3);
        let mut SVFOVM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut SVFPOL = ActualArray2D::<f64>::new(1..=2, 1..=MAXVRT);
        let mut SVFSMX = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut SVFVCT = StackArray::<f64, 3>::new(1..=3);
        let mut SVORIG = StackArray::<f64, 3>::new(1..=3);
        let mut SVPLAN = StackArray::<f64, 4>::new(1..=UBPL);
        let mut SVRDIR = StackArray::<f64, 3>::new(1..=3);
        let mut SVSEMI = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
        let mut SVTRAD = StackArray::<f64, 3>::new(1..=3);
        let mut SVXMAG = StackArray::<f64, 2>::new(1..=2);
        let mut SVINST: i32 = 0;
        let mut SVNVRT: i32 = 0;
        let mut SVOBS: i32 = 0;
        let mut SVTARG: i32 = 0;
        let mut SVETRG: bool = false;
        let mut SVURAY: bool = false;
        let mut SVUSTL: bool = false;
        let mut SVXMIT: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            SVORIG
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            SVIFRM,
            SVINAM,
            SVISHP,
            SVTFRM,
            SVTNAM,
            SVTSHP,
            SVCORR,
            SVARAD,
            SVBNDS,
            SVEDCT,
            SVFAXI,
            SVFOVM,
            SVFPOL,
            SVFSMX,
            SVFVCT,
            SVORIG,
            SVPLAN,
            SVRDIR,
            SVSEMI,
            SVTRAD,
            SVXMAG,
            SVINST,
            SVNVRT,
            SVOBS,
            SVTARG,
            SVETRG,
            SVURAY,
            SVUSTL,
            SVXMIT,
        }
    }
}

//$Procedure ZZGFFVU ( GF, instrument FOV utilities )
pub fn ZZGFFVU(
    INST: &[u8],
    TSHAPE: &[u8],
    RAYDIR: &[f64],
    TARGET: &[u8],
    TFRAME: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    TIME: f64,
    VISTAT: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // ATOL is a tolerance value for computing FOV angular radius.
    // The angular radius must not exceed pi/2 - ATOL radians.
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
    // Below we initialize the list of visibility types.
    //

    //
    // This routine should never be called directly.
    //
    CHKIN(b"ZZGFFVU", ctx)?;

    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    CHKOUT(b"ZZGFFVU", ctx)?;
    Ok(())
}

//$Procedure  ZZGFFVIN ( GF, visibility initialization )
pub fn ZZGFFVIN(
    INST: &[u8],
    TSHAPE: &[u8],
    RAYDIR: &[f64],
    TARGET: &[u8],
    TFRAME: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let RAYDIR = DummyArray::new(RAYDIR, 1..=3);
    let mut BSITE = StackArray::<f64, 3>::new(1..=3);
    let mut CTREXT: f64 = 0.0;
    let mut ELLRAD = StackArray::<f64, 3>::new(1..=3);
    let mut ESCALE: f64 = 0.0;
    let mut SEMIPT = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP2 = StackArray::<f64, 3>::new(1..=3);
    let mut X = StackArray::<f64, 3>::new(1..=3);
    let mut XPT = StackArray::<f64, 3>::new(1..=3);
    let mut Y = StackArray::<f64, 3>::new(1..=3);
    let mut Z = StackArray::<f64, 3>::new(1..=3);
    let mut CLSSID: i32 = 0;
    let mut FRAMID: i32 = 0;
    let mut FRCENT: i32 = 0;
    let mut FRCLSS: i32 = 0;
    let mut NXPTS: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FOUND: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFFVIN", ctx)?;

    //
    // To avoid portability problems, initialize all
    // saved variables that aren't initialized via DATA
    // statements and aren't guaranteed to be initialized
    // for all cases.
    //
    CLEARD((3 * MAXVRT), save.SVBNDS.as_slice_mut());
    CLEARD(3, save.SVEDCT.as_slice_mut());
    CLEARD(3, save.SVFAXI.as_slice_mut());
    CLEARD((2 * MAXVRT), save.SVFPOL.as_slice_mut());
    CLEARD(9, save.SVFSMX.as_slice_mut());
    CLEARD(UBPL, save.SVPLAN.as_slice_mut());
    CLEARD(3, save.SVRDIR.as_slice_mut());
    save.SVTARG = 0;
    fstr::assign(&mut save.SVTFRM, b" ");
    fstr::assign(&mut save.SVTNAM, b" ");
    CLEARD(3, save.SVTRAD.as_slice_mut());
    save.SVUSTL = false;
    save.SVXMIT = false;

    //
    // Find the NAIF ID for OBSRVR.
    //
    BODS2C(OBSRVR, &mut save.SVOBS, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"The observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFFVIN", ctx)?;
        return Ok(());
    }

    //
    // Process the target shape specifier here.
    //
    // Save a left-justified, upper case version of the target shape
    // specifier.
    //
    LJUST(TSHAPE, &mut save.SVTSHP);
    UCASE(&save.SVTSHP.to_vec(), &mut save.SVTSHP, ctx);

    //
    // Note for maintenance programmer: these checks will
    // require modification to handle DSK-based shapes.
    //
    if ((fstr::ne(&save.SVTSHP, PTSHAP) && fstr::ne(&save.SVTSHP, EDSHAP))
        && fstr::ne(&save.SVTSHP, RYSHAP))
    {
        SETMSG(
            b"The target shape specification, \'#\', is not recognized.",
            ctx,
        );
        ERRCH(b"#", TSHAPE, ctx);
        SIGERR(b"SPICE(INVALIDSHAPE)", ctx)?;
        CHKOUT(b"ZZGFFVIN", ctx)?;
        return Ok(());
    }

    //
    // We'll use the logical variable USERAY to indicate that the
    // target is modeled as ray.
    //
    save.SVURAY = fstr::eq(&save.SVTSHP, RYSHAP);

    //
    // Indicate whether we have an ellipsoidal target. SVETRG is .TRUE.
    // if and only we have one.
    //
    save.SVETRG = fstr::eq(&save.SVTSHP, EDSHAP);

    //
    // If the target is an ephemeris object, obtain its ID code.
    // Save the target object's name, if applicable.
    //
    if !save.SVURAY {
        BODS2C(TARGET, &mut save.SVTARG, &mut FOUND, ctx)?;

        if !FOUND {
            SETMSG(b"The target object, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
            ERRCH(b"#", TARGET, ctx);
            SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
            CHKOUT(b"ZZGFFVIN", ctx)?;
            return Ok(());
        }

        //
        // Save the target's name.
        //
        fstr::assign(&mut save.SVTNAM, TARGET);

        //
        // Make sure the observer and target are distinct.
        //
        if (save.SVTARG == save.SVOBS) {
            SETMSG(b"The observer and target must be distinct objects, but are not: OBSRVR = #; TARGET = #;", ctx);
            ERRCH(b"#", OBSRVR, ctx);
            ERRCH(b"#", TARGET, ctx);
            SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
            CHKOUT(b"ZZGFFVIN", ctx)?;
            return Ok(());
        }
    }

    //
    // Process the target frame. The target frame is defined except
    // when the target is an ephemeris object modeled as a point.
    //
    if (save.SVURAY || save.SVETRG) {
        //
        // We'll use the target frame argument. Look up the target
        // frame's ID code. But first, check for a blank frame name,
        // since this may be a common problem for the GF FOV system.
        //
        if fstr::eq(TFRAME, b" ") {
            SETMSG(
                b"The target is not modeled as a point, but the associated frame name is blank.",
                ctx,
            );
            SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
            CHKOUT(b"ZZGFFVIN", ctx)?;
            return Ok(());
        }

        NAMFRM(TFRAME, &mut FRAMID, ctx)?;

        if (FRAMID == 0) {
            SETMSG(b"The target frame name # is not recognized.", ctx);
            ERRCH(b"#", TFRAME, ctx);
            SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
            CHKOUT(b"ZZGFFVIN", ctx)?;
            return Ok(());
        }

        //
        // Save the target frame name.
        //
        LJUST(TFRAME, &mut save.SVTFRM);
        UCASE(&save.SVTFRM.to_vec(), &mut save.SVTFRM, ctx);

        //
        // Obtain the center of the frame. If the target is an ephemeris
        // object, we must verify the frame center is the target.
        //
        FRINFO(
            FRAMID,
            &mut FRCENT,
            &mut FRCLSS,
            &mut CLSSID,
            &mut FOUND,
            ctx,
        )?;

        if !FOUND {
            //
            // Since we mapped the frame name to an ID code, we expect to
            // find the frame info. Getting here may be a sign of an
            // invalid frame kernel.
            //
            SETMSG(b"Frame ID found for # body-fixed frame # but FRINFO couldn\'t find frame info. This may be due to a frame kernel error.", ctx);
            ERRCH(b"#", TARGET, ctx);
            SIGERR(b"SPICE(FRAMEINFONOTFOUND)", ctx)?;
            CHKOUT(b"ZZGFFVIN", ctx)?;
            return Ok(());
        }

        if save.SVETRG {
            //
            // We have an ellipsoidal target. Check the target frame's
            // center.
            //
            if (FRCENT != save.SVTARG) {
                //
                // The supposed body-fixed frame for the target isn't
                // actually centered on the target.
                //
                SETMSG(
                    b"Supposed body-fixed frame # for target # is actually centered on body #.",
                    ctx,
                );
                ERRCH(b"#", TFRAME, ctx);
                ERRCH(b"#", TARGET, ctx);
                ERRINT(b"#", FRCENT, ctx);
                SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
                CHKOUT(b"ZZGFFVIN", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // Process the aberration correction specifier.
    //
    if save.SVURAY {
        //
        // The target is represented by a ray. Check and save the
        // aberration correction.
        //
        ZZPRSCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFFVIN", ctx)?;
            return Ok(());
        }

        //
        // Reject aberration correction flags calling for any type of
        // light time correction. However, stellar aberration corrections
        // are allowed: note this is the reverse of the situation for
        // ephemeris objects. The allowed aberration correction flags are
        //
        //    'NONE', 'S', 'XS'
        //
        if ATTBLK[LTIDX] {
            SETMSG(b"Aberration correction flag # calls for light time corrections; these are not supported for targets represented by rays.", ctx);
            ERRCH(b"#", ABCORR, ctx);
            SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
            CHKOUT(b"ZZGFFVIN", ctx)?;
            return Ok(());
        }

        //
        // Save flags indicating whether to use stellar aberration
        // corrections and indicating the sense of radiation travel.
        //
        save.SVUSTL = ATTBLK[STLIDX];
        save.SVXMIT = ATTBLK[XMTIDX];
    } else {
        //
        // The target is an ephemeris object.
        //
        // Check the aberration correction. If SPKEZR can't handle it,
        // neither can we.
        //
        ZZVALCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFFVIN", ctx)?;
            return Ok(());
        }
    }

    //
    // Remove all spaces from ABCORR then convert to uppercase. Save
    // this version of the aberration correction specifier.
    //
    CMPRSS(b" ", 0, ABCORR, &mut save.SVCORR);
    UCASE(&save.SVCORR.to_vec(), &mut save.SVCORR, ctx);

    //
    // Process the target body's radii, if applicable.
    //
    if save.SVETRG {
        //
        // Fetch and check the radii.
        //
        ZZGFTREB(save.SVTARG, save.SVTRAD.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFFVIN", ctx)?;
            return Ok(());
        }

    //
    // Checks of radii have been completed.
    //
    } else {
        //
        // We don't have an ellipsoidal target body: zero out radius
        // values for this target.
        //
        CLEARD(3, save.SVTRAD.as_slice_mut());
    }

    //
    // Check the direction vector, if applicable.
    //
    if save.SVURAY {
        //
        // Make sure the direction vector is non-zero. Save a unit-length
        // copy of the vector.
        //
        if VZERO(RAYDIR.as_slice()) {
            SETMSG(
                b"Input ray direction was the zero vector; this vector must be non-zero.",
                ctx,
            );
            SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
            CHKOUT(b"ZZGFFVIN", ctx)?;
            return Ok(());
        }

        VHAT(RAYDIR.as_slice(), save.SVRDIR.as_slice_mut());
    }

    //
    // Look up the instrument's ID code.
    //
    BODS2C(INST, &mut save.SVINST, &mut FOUND, ctx)?;

    if !FOUND {
        SETMSG(b"\'#\' is not a recognized name for an instrument. The cause of this problem may be that you have not loaded a required frame kernel or instrument kernel.", ctx);
        ERRCH(b"#", INST, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"ZZGFFVIN", ctx)?;
        return Ok(());
    }

    //
    // Save the instrument's name.
    //
    LJUST(INST, &mut save.SVINAM);
    UCASE(&save.SVINAM.to_vec(), &mut save.SVINAM, ctx);

    //
    // Look up the instrument parameters.
    //
    GETFOV(
        save.SVINST,
        MAXVRT,
        &mut save.SVISHP,
        &mut save.SVIFRM,
        BSITE.as_slice_mut(),
        &mut save.SVNVRT,
        save.SVBNDS.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFFVIN", ctx)?;
        return Ok(());
    }

    //
    // Scale boresight vector to unit length.
    //
    VHATIP(BSITE.as_slice_mut());

    //
    // Make sure the instrument shape specifier is left-justified
    // and in upper case.
    //
    LJUST(&save.SVISHP.to_vec(), &mut save.SVISHP);
    UCASE(&save.SVISHP.to_vec(), &mut save.SVISHP, ctx);

    //
    // If the instrument's shape is 'RECTANGLE', map it to
    // 'POLYGON'
    //
    if fstr::eq(&save.SVISHP, RECFOV) {
        fstr::assign(&mut save.SVISHP, POLFOV);
    }

    //
    // Save an axis vector for the FOV. For circular and ellipsoidal
    // FOVs, the boresight serves as this axis. For polygonal FOVs
    // (rectangular FOVs are included), we'll generate an axis vector.
    //
    if fstr::eq(&save.SVISHP, POLFOV) {
        ZZFOVAXI(
            INST,
            save.SVNVRT,
            save.SVBNDS.as_slice(),
            save.SVFAXI.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFFVIN", ctx)?;
            return Ok(());
        }
    } else {
        VEQU(BSITE.as_slice(), save.SVFAXI.as_slice_mut());
    }

    //
    // Check the angular radius of the FOV.
    //
    // Compute the angular radius of the FOV. We'll use this to define a
    // "bounding cone" centered on the FOV axis and having its apex at
    // the observer. This cone will be used for a preliminary FOV
    // exclusion test.
    //
    save.SVARAD = 0.0;

    for I in 1..=save.SVNVRT {
        save.SVARAD = intrinsics::DMAX1(&[
            save.SVARAD,
            VSEP(save.SVBNDS.subarray([1, I]), save.SVFAXI.as_slice(), ctx),
        ]);
    }

    //
    // Our algorithms can't handle FOVs with angular radius of 90
    // degrees.
    //
    if (save.SVARAD > (HALFPI(ctx) - ATOL)) {
        SETMSG(
            b"FOV angular radius of # degrees exceeds limit of # degrees.",
            ctx,
        );
        ERRDP(b"#", (save.SVARAD * DPR(ctx)), ctx);
        ERRDP(b"#", ((HALFPI(ctx) - ATOL) * DPR(ctx)), ctx);
        SIGERR(b"SPICE(FOVTOOWIDE)", ctx)?;
        CHKOUT(b"ZZGFFVIN", ctx)?;
        return Ok(());
    }

    //
    // Convert the FOV shape specifier to a left-justified, upper
    // case form.
    //
    LJUST(&save.SVISHP.to_vec(), &mut save.SVISHP);
    UCASE(&save.SVISHP.to_vec(), &mut save.SVISHP, ctx);

    //
    // We can make the search more efficient by computing any
    // required, time-invariant quantities here in the initialization
    // routine.
    //
    // Compute the FOV plane SVPLAN, which is represented in the
    // instrument frame. The origin will be considered to be located at
    // the observer. The plane is normal to the FOV axis, at distance 1
    // unit from the observer.
    //
    NVC2PL(save.SVFAXI.as_slice(), 1.0, save.SVPLAN.as_slice_mut(), ctx)?;

    //
    // Find the point on the plane closest to the origin. This is
    // the center of the FOV.
    //
    VHAT(save.SVFAXI.as_slice(), save.SVFVCT.as_slice_mut());

    //
    // If applicable, perform the computations required for an
    // elliptical FOV, where the target representation is arbitrary, or
    // a circular FOV when the target is an ellipsoid. Note that
    // these computations are not needed for the combination of a
    // circular FOV and a point or ray target.
    //
    if (fstr::eq(&save.SVISHP, ELLFOV) || (fstr::eq(&save.SVISHP, CIRFOV) && save.SVETRG)) {
        //
        // Also compute the center, semi-axis vectors, and semi-axis
        // lengths of the FOV. If the FOV is circular, we create an
        // artificial, second semi-axis vector.
        //
        if fstr::eq(&save.SVISHP, CIRFOV) {
            //
            // We have a circular FOV. We'll create an artificial, second
            // boundary vector, which will give rise to a second
            // semi-axis.
            //
            VROTV(
                &save.SVBNDS.subarray([1, 1]).to_vec(),
                save.SVFAXI.as_slice(),
                HALFPI(ctx),
                save.SVBNDS.subarray_mut([1, 2]),
            );
        }

        //
        // Now find the endpoints of the semi-axes in this plane.
        //
        for I in 1..=2 {
            INRYPL(
                save.SVORIG.as_slice(),
                save.SVBNDS.subarray([1, I]),
                save.SVPLAN.as_slice(),
                &mut NXPTS,
                SEMIPT.subarray_mut([1, I]),
                ctx,
            )?;

            if (NXPTS != 1) {
                SETMSG(b"Error creating FOV semi-axis vectors, NXPTS = #. This may indicate an error in the IK parameters for #.", ctx);
                ERRINT(b"#", NXPTS, ctx);
                ERRCH(b"#", INST, ctx);
                SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
                CHKOUT(b"ZZGFFVIN", ctx)?;
                return Ok(());
            }

            //
            // Compute and find the length of each semi-axis vector.
            //
            VSUB(
                SEMIPT.subarray([1, I]),
                save.SVFVCT.as_slice(),
                save.SVSEMI.subarray_mut([1, I]),
            );

            save.SVXMAG[I] = VNORM(save.SVSEMI.subarray([1, I]));

            if (save.SVXMAG[I] == 0.0) {
                SETMSG(b"FOV semi-axis #* for @ has zero length.", ctx);
                ERRINT(b"*", I, ctx);
                ERRCH(b"@", INST, ctx);
                SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
                CHKOUT(b"ZZGFFVIN", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // If we have an ellipsoidal target, and the FOV is circular or
    // elliptical, we'll create an ellipsoid whose limb coincides with
    // the FOV. This allows use to later use ZZOCCED to determine the
    // target's visibility.
    //
    if ((fstr::eq(&save.SVISHP, CIRFOV) || fstr::eq(&save.SVISHP, ELLFOV)) && save.SVETRG) {
        //
        //
        // Create an ellipsoid whose semi-axes are consistent with the
        // ellipse in SVPLAN defined by SEMIPT. Caution: after we
        // create this ellipsoid, we'll scale and shift it so it doesn't
        // extend too far from the observer.
        //
        // Recall the origin is that of the instrument frame. The plane
        // SVPLAN is normal to the FOV axis and has distance 1 km from
        // the origin.
        //
        // To start out, select the center of the ellipsoid. We place the
        // center along the direction defined by the FOV axis, at a
        // distance beyond SVPLAN (that is, on the side of the plane
        // opposite the observer), such that a sphere centered at this
        // point would have a limb consisting of a circle of radius
        // SVXMAG(1). If CTREXT is the distance of the ellipsoid center
        // from SVFVCT, then the limb geometry requires
        //
        //    CTREXT / SVXMAG(1) = SVXMAG(1) / 1
        //
        //
        CTREXT = f64::powi(save.SVXMAG[1], 2);

        //
        // The ellipsoid's center is SVEDCT.
        //
        VSCL(
            (1.0 + CTREXT),
            save.SVFVCT.as_slice(),
            save.SVEDCT.as_slice_mut(),
        );

        //
        // NOTE: in the code and discussion that follow, there are
        // references to both the FOV center SVFVCT and the ellipsoid
        // center SVEDCT. Note that the directions of the ellipsoid's
        // semi-axes point from the FOV center, NOT the ellipsoid center,
        // toward the intercepts of the FOV boundary vectors on the
        // FOV plane.
        //
        // Compute the radius of the sphere centered at SVEDCT. The
        // ellipsoid's semi-axes pointing in the FOV axis direction and
        // in the direction from SVFVCT toward SEMIPT(*,1) will have this
        // length.
        //
        ELLRAD[3] = (save.SVXMAG[1] * f64::sqrt((1.0 + f64::powf(save.SVXMAG[1], 2.0))));
        ELLRAD[1] = ELLRAD[3];

        //
        // Compute the corresponding columns of the FOV semi-axis matrix.
        //
        // The ellipsoid's third axis points along the FOV axis. Note
        // that SVFVCT is a unit vector pointing in the desired
        // direction.
        //
        VSCL(
            ELLRAD[3],
            save.SVFVCT.as_slice(),
            save.SVFSMX.subarray_mut([1, 3]),
        );

        //
        // The first ellipsoid semi-axis is associated with SEMIPT(*,1)
        // and also has length ELLRAD(3):
        //
        VHAT(save.SVSEMI.subarray([1, 1]), VTEMP.as_slice_mut());
        VSCL(
            ELLRAD[1],
            VTEMP.as_slice(),
            save.SVFSMX.subarray_mut([1, 1]),
        );

        //
        // The ellipsoid's second semi-axis points from SVFVCT toward
        // SEMIPT(*,2). The ratio of its length to that of the other
        // semi-axis is the ratio of the length of the FOV's second
        // semi-axis to that of its first. Note that we've already ruled
        // out divide-by-zero errors here.
        //
        ELLRAD[2] = ((save.SVXMAG[2] / save.SVXMAG[1]) * ELLRAD[3]);

        //
        // We define the third axis using a cross product to
        // ensure we produce a matrix with positive determinant.
        //
        UCRSS(
            save.SVFSMX.subarray([1, 3]),
            save.SVFSMX.subarray([1, 1]),
            VTEMP.as_slice_mut(),
        );
        VSCL(
            ELLRAD[2],
            VTEMP.as_slice(),
            save.SVFSMX.subarray_mut([1, 2]),
        );

        //
        // Scale the ellipsoid and the distance of its center from
        // the observer so that the ellipsoid is of reasonable size
        // and doesn't extend too far from the observer. Caution: this
        // modification means the ellipsoid no longer intersects the FOV
        // plane at the FOV boundary. The ellipsoid is still usable with
        // ZZOCCED, which is the ellipsoid's raison d'etre.
        //
        ESCALE =
            (FVEMAX / (VNORM(save.SVEDCT.as_slice()) + intrinsics::DMAX1(&[ELLRAD[1], ELLRAD[2]])));

        for I in 1..=3 {
            VSCLIP(ESCALE, save.SVFSMX.subarray_mut([1, I]));
        }

        VSCLIP(ESCALE, save.SVEDCT.as_slice_mut());
    }

    if (fstr::eq(&save.SVISHP, CIRFOV) && !save.SVETRG) {
        //
        // We have a circular FOV and a point or ray target model.
        // In this case, our FOV inclusion test is simple as can
        // be: we just compare the angular separation of the
        // target and FOV axis against the angular radius of the
        // FOV. Compute and save this angular radius.
        //
        save.SVARAD = VSEP(save.SVFAXI.as_slice(), save.SVBNDS.subarray([1, 1]), ctx);
    } else if ((fstr::eq(&save.SVISHP, RECFOV) || fstr::eq(&save.SVISHP, POLFOV)) && !save.SVETRG) {
        //
        // We have a rectangular or polygonal FOV and a ray or point
        // target.
        //
        // We're going to represent the FOV boundary by a polygon
        // in the FOV plane SVPLAN. We want to be able to use a
        // 2-dimensional winding number computation to decide whether
        // the target is within the FOV. We'll need a reference
        // frame with the Z-axis parallel to the FOV axis vector;
        // we'll represent the intersections of the boundary vectors
        // with the FOV plane in this frame. Then our 2D polygon
        // will have vertices given by the (X,Y) components of each
        // intersection.
        //
        VEQU(save.SVFAXI.as_slice(), Z.as_slice_mut());

        FRAME(Z.as_slice_mut(), X.as_slice_mut(), Y.as_slice_mut());

        for I in 1..=3 {
            save.SVFOVM[[1, I]] = X[I];
            save.SVFOVM[[2, I]] = Y[I];
            save.SVFOVM[[3, I]] = Z[I];
        }

        //
        // Compute the intersections of the FOV boundary vectors with the
        // FOV plane. For each intercept, find the vector pointing from
        // the FOV center to that intercept. Transform each such
        // difference vector into the FOV frame. Save the projection onto
        // the FOV frame's X-Y plane.
        //
        for I in 1..=save.SVNVRT {
            INRYPL(
                save.SVORIG.as_slice(),
                save.SVBNDS.subarray([1, I]),
                save.SVPLAN.as_slice(),
                &mut NXPTS,
                XPT.as_slice_mut(),
                ctx,
            )?;

            if (NXPTS != 1) {
                SETMSG(b"Error finding FOV plane intercept of FOV boundary vector #, NXPTS = #. This may indicate an error in the IK parameters for #.", ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", NXPTS, ctx);
                ERRCH(b"#", INST, ctx);
                SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
                CHKOUT(b"ZZGFFVIN", ctx)?;
                return Ok(());
            }

            VSUB(XPT.as_slice(), save.SVFVCT.as_slice(), VTEMP.as_slice_mut());
            MXV(
                save.SVFOVM.as_slice(),
                VTEMP.as_slice(),
                VTEMP2.as_slice_mut(),
            );

            save.SVFPOL[[1, I]] = VTEMP2[1];
            save.SVFPOL[[2, I]] = VTEMP2[2];
        }
    }

    CHKOUT(b"ZZGFFVIN", ctx)?;
    Ok(())
}

//$Procedure ZZGFFVST ( GF, "is target in FOV?"  )
pub fn ZZGFFVST(TIME: f64, VISTAT: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut COORD = StackArray::<f64, 2>::new(1..=2);
    let mut DIR = StackArray::<f64, 3>::new(1..=3);
    let mut ETTARG: f64 = 0.0;
    let mut FOVPT = StackArray::<f64, 3>::new(1..=3);
    let mut FVLIMB = StackArray::<f64, 9>::new(1..=UBEL);
    let mut INSMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut L: f64 = 0.0;
    let mut LIMB = StackArray::<f64, 9>::new(1..=UBEL);
    let mut LT: f64 = 0.0;
    let mut M1 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut M2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut OBSPOS = StackArray::<f64, 3>::new(1..=3);
    let mut PNT2D = StackArray::<f64, 3>::new(1..=3);
    let mut POS = StackArray::<f64, 3>::new(1..=3);
    let mut SEP: f64 = 0.0;
    let mut STOBS = StackArray::<f64, 6>::new(1..=6);
    let mut TRGCTR = StackArray::<f64, 3>::new(1..=3);
    let mut TRGSMX = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP2 = StackArray::<f64, 3>::new(1..=3);
    let mut XPT = StackArray::<f64, 3>::new(1..=3);
    let mut NXPTS: i32 = 0;
    let mut OCSTAT: i32 = 0;
    let mut W: i32 = 0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFFVST", ctx)?;

    //
    // Initialize the state output.
    //
    *VISTAT = false;

    //
    // The algorithm for the state determination depends on the
    // target model and the FOV shape.
    //
    if save.SVETRG {
        //
        // The target is an ephemeris object modeled as an ellipsoid.
        // There are two branches here: one for a rectangular/
        // polygonal FOV and one for a circular/elliptical FOV.
        //
        // Start by finding the observer-target position vector in the
        // target body-fixed frame.
        //
        SPKEZP(
            save.SVTARG,
            TIME,
            &save.SVTFRM,
            &save.SVCORR,
            save.SVOBS,
            POS.as_slice_mut(),
            &mut LT,
            ctx,
        )?;

        //
        // Compute the target epoch.
        //
        ZZCOREPC(&save.SVCORR, TIME, LT, &mut ETTARG, ctx)?;

        //
        // Find the transformation from the target frame at ETTARG to the
        // instrument frame at TIME. We'll need to use J2000 as an
        // intermediate frame.
        //
        PXFORM(&save.SVTFRM, b"J2000", ETTARG, M1.as_slice_mut(), ctx)?;
        PXFORM(b"J2000", &save.SVIFRM, TIME, M2.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFFVST", ctx)?;
            return Ok(());
        }

        MXM(M2.as_slice(), M1.as_slice(), INSMAT.as_slice_mut());

        if (fstr::eq(&save.SVISHP, RECFOV) || fstr::eq(&save.SVISHP, POLFOV)) {
            //
            // The FOV is a rectangle or other polygon; we treat both
            // cases the same way.
            //
            // Negate POS to obtain the position of the observer with
            // respect to the target.
            //
            VMINUS(POS.as_slice(), OBSPOS.as_slice_mut());

            //
            // Find the limb in the target body-fixed frame.
            //
            EDLIMB(
                save.SVTRAD[1],
                save.SVTRAD[2],
                save.SVTRAD[3],
                OBSPOS.as_slice(),
                LIMB.as_slice_mut(),
                ctx,
            )?;

            //
            // Transform the limb from the target frame at ETTARG
            // to the instrument frame at TIME. The matrix INSMAT
            // effects just this transformation. We unpack the center
            // and semi-axis vectors of LIMB, transform them, and
            // pack the results into FVLIMB. Below, M1 and M2 are
            // simply temporary 3x3 matrices.
            //
            let [arg1, arg2, arg3] = M1
                .get_disjoint_slices_mut([[1, 1], [1, 2], [1, 3]])
                .unwrap();
            EL2CGV(LIMB.as_slice(), arg1, arg2, arg3);

            //
            // Before performing the frame transformation on the
            // limb's center, translate the center so that the
            // observer is at the origin. Since POS is expressed
            // in the target body-fixed frame, this is a convenient
            // place for the translation.
            //
            VADD(POS.as_slice(), M1.subarray([1, 1]), VTEMP.as_slice_mut());
            VEQU(VTEMP.as_slice(), M1.subarray_mut([1, 1]));

            for I in 1..=3 {
                MXV(
                    INSMAT.as_slice(),
                    M1.subarray([1, I]),
                    M2.subarray_mut([1, I]),
                );
            }

            CGV2EL(
                M2.subarray([1, 1]),
                M2.subarray([1, 2]),
                M2.subarray([1, 3]),
                FVLIMB.as_slice_mut(),
                ctx,
            )?;

            //
            // All geometric objects in the following call are expressed
            // in the instrument reference frame.
            //
            // The target is in the FOV if and only if ZZELVUPY finds an
            // intersection, so we use VISTAT as the "found" flag.
            //
            ZZELVUPY(
                FVLIMB.as_slice(),
                save.SVORIG.as_slice(),
                save.SVFAXI.as_slice(),
                save.SVNVRT,
                save.SVBNDS.as_slice(),
                VISTAT,
                ctx,
            )?;
        } else if (fstr::eq(&save.SVISHP, CIRFOV) || fstr::eq(&save.SVISHP, ELLFOV)) {
            //
            // The FOV is a circle or ellipse. For both FOV shapes,
            // we represent the FOV by an ellipsoid in the FOV
            // frame. We can then use ZZOCCED to determine whether
            // there's any overlap of this ellipsoid and the target.
            //
            // We'll perform the occultation test in the instrument frame,
            // so we'll need to represent the observer-target position
            // and target semi-axes in that frame.
            //
            // Transform the target position to the instrument frame.
            //
            MXV(INSMAT.as_slice(), POS.as_slice(), TRGCTR.as_slice_mut());

            //
            // The columns of INSMAT are the target body's semi-axis
            // direction vectors; we scale these by the target radii
            // to obtain the semi-axis matrix for the target.
            //
            for I in 1..=3 {
                VSCL(
                    save.SVTRAD[I],
                    INSMAT.subarray([1, I]),
                    TRGSMX.subarray_mut([1, I]),
                );
            }

            OCSTAT = ZZOCCED(
                save.SVORIG.as_slice(),
                save.SVEDCT.as_slice(),
                save.SVFSMX.as_slice(),
                TRGCTR.as_slice(),
                TRGSMX.as_slice(),
                ctx,
            )?;

            //
            // A return code of zero indicates no occultation. Any other
            // return code indicates a non-empty intersection of the
            // target and FOV.
            //
            *VISTAT = (OCSTAT != 0);
        } else {
            //
            // This is an unexpected FOV shape. We should have prevented
            // this problem in the initialization step, but we repeat the
            // check here for safety.
            //
            SETMSG(b"The target body # has shape #; the only supported shapes are ELLIPSOID, POINT, and RAY.", ctx);
            ERRCH(b"#", &save.SVTNAM, ctx);
            ERRCH(b"#", &save.SVISHP, ctx);
            SIGERR(b"SPICE(INVALIDSHAPE)", ctx)?;
            CHKOUT(b"ZZGFFVST", ctx)?;
            return Ok(());
        }

    //
    // This is the end of the ellipsoidal target case. At this
    // point, VISTAT is set.
    //
    } else {
        //
        // The target is a ray or an ephemeris object modeled as a point.
        // In either case, we want to obtain the aberration-corrected
        // observer-target vector.
        //
        if save.SVURAY {
            //
            // The target is represented by a ray expressed in the
            // frame SVTFRM.
            //
            // Normally we'd need to correct the orientation of SVTFRM
            // for light time between the center of that frame and the
            // observer. But since light time corrections are not allowed
            // for targets represented by rays, we evaluate SVTFRM
            // at the current epoch TIME.
            //
            PXFORM(&save.SVTFRM, &save.SVIFRM, TIME, INSMAT.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZGFFVST", ctx)?;
                return Ok(());
            }

            //
            // Transform the ray's direction vector to the instrument
            // frame.
            //
            MXV(
                INSMAT.as_slice(),
                save.SVRDIR.as_slice(),
                DIR.as_slice_mut(),
            );

            //
            // If we need to correct the ray's direction for stellar
            // aberration, do it now.
            //
            if save.SVUSTL {
                //
                // Find the state of the observer relative to the
                // solar system barycenter in the J2000 frame.
                //
                SPKSSB(save.SVOBS, TIME, b"J2000", STOBS.as_slice_mut(), ctx)?;

                //
                // Convert the direction vector to the J2000 frame.
                //
                PXFORM(&save.SVIFRM, b"J2000", TIME, M1.as_slice_mut(), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"ZZGFFVST", ctx)?;
                    return Ok(());
                }

                MXV(M1.as_slice(), DIR.as_slice(), VTEMP.as_slice_mut());

                //
                // Apply the stellar aberration correction.
                //
                if save.SVXMIT {
                    //
                    // Use the transmission correction.
                    //
                    STLABX(
                        VTEMP.as_slice(),
                        STOBS.subarray(4),
                        VTEMP2.as_slice_mut(),
                        ctx,
                    )?;
                } else {
                    STELAB(
                        VTEMP.as_slice(),
                        STOBS.subarray(4),
                        VTEMP2.as_slice_mut(),
                        ctx,
                    )?;
                }

                //
                // Map the direction vector back to the instrument
                // frame.
                //
                MTXV(M1.as_slice(), VTEMP2.as_slice(), DIR.as_slice_mut());
            }
        //
        // The target direction in the instrument frame DIR has
        // been computed.
        //
        } else {
            //
            // The target is an ephemeris object. Look up the
            // target's position relative to the observer.
            //
            // Note for the maintenance programmer: don't think of
            // changing this call to look up the position in the
            // instrument frame. :) Since we don't have a guarantee that
            // the instrument frame is centered on the observer (the frame
            // could be J2000, for example), and since we don't want to
            // correct the orientation of the instrument frame for light
            // time, we look up the direction vector in the J2000 frame
            // and then map it to the instrument frame.
            //
            SPKEZP(
                save.SVTARG,
                TIME,
                b"J2000",
                &save.SVCORR,
                save.SVOBS,
                VTEMP.as_slice_mut(),
                &mut LT,
                ctx,
            )?;

            PXFORM(b"J2000", &save.SVIFRM, TIME, M1.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZGFFVST", ctx)?;
                return Ok(());
            }

            MXV(M1.as_slice(), VTEMP.as_slice(), DIR.as_slice_mut());
        }

        if FAILED(ctx) {
            CHKOUT(b"ZZGFFVST", ctx)?;
            return Ok(());
        }

        //
        // The observer-target direction vector DIR is set.
        //
        // The determination of whether the ray is in the FOV depends
        // on the FOV shape.
        //
        SEP = VSEP(DIR.as_slice(), save.SVFAXI.as_slice(), ctx);

        if fstr::eq(&save.SVISHP, CIRFOV) {
            //
            // Just compare the angular separation of POS with the
            // FOV axis direction against the FOV angular radius SVARAD.
            //
            *VISTAT = (SEP <= save.SVARAD);
        } else if (SEP > save.SVARAD) {
            //
            // The FOV is an ellipse or polygon.
            //
            // The angular separation of target and FOV axis is
            // greater than the angular radius of the exclusion
            // cone. The target can't be seen.
            //
            *VISTAT = false;
        } else {
            //
            // The FOV is an ellipse or polygon.
            //
            // The angular separation of target and FOV axis is
            // less than or equal to than the angular radius of the
            // exclusion code, so the target may be visible.
            //
            // Find the intersection of the ray emanating from the
            // observer, and having direction vector POS, with the FOV
            // plane.
            //
            INRYPL(
                save.SVORIG.as_slice(),
                DIR.as_slice(),
                save.SVPLAN.as_slice(),
                &mut NXPTS,
                XPT.as_slice_mut(),
                ctx,
            )?;

            //
            // If there's no intersection, the target isn't visible.
            //
            if (NXPTS == 0) {
                *VISTAT = false;
            } else if (NXPTS != 1) {
                //
                // "This can't happen." :)
                //
                SETMSG(b"By construction, the vertex of the observer-target ray can\'t lie in the FOV plane. If somehow it does, we have a serious problem.", ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZGFFVST", ctx)?;
                return Ok(());
            } else {
                //
                // NXPTS is 1.
                //
                // Find the vector from the center of the FOV to XPT.
                // Call this vector FOVPT.
                //
                VSUB(XPT.as_slice(), save.SVFVCT.as_slice(), FOVPT.as_slice_mut());

                if fstr::eq(&save.SVISHP, ELLFOV) {
                    //
                    // The FOV shape is elliptical. To decide whether FOVPT
                    // is within the FOV, compute the level surface
                    // parameter
                    //
                    //                   2              2
                    //    L  =  ( x / a )   +  ( y / b )
                    //
                    // and compare L to 1. We'll use the variable COORD
                    // to represent the coordinates (x,y).
                    //
                    // We've already eliminated zero divisors in the
                    // initialization routine.
                    //
                    for I in 1..=2 {
                        COORD[I] =
                            (VDOT(FOVPT.as_slice(), save.SVSEMI.subarray([1, I])) / save.SVXMAG[I]);
                    }

                    L = (f64::powf((COORD[1] / save.SVXMAG[1]), 2.0)
                        + f64::powf((COORD[2] / save.SVXMAG[2]), 2.0));

                    //
                    // The target is visible if FOVPT is inside the FOV
                    // ellipse; this condition is indicated by L <= 1.
                    //
                    *VISTAT = (L <= 1.0);
                } else if fstr::eq(&save.SVISHP, POLFOV) {
                    //
                    // The FOV is a polygon. Convert FOVPT to the FOV frame,
                    // then find the winding number of the FOV about the X-Y
                    // projection of FOVPT.
                    //
                    MXV(
                        save.SVFOVM.as_slice(),
                        FOVPT.as_slice(),
                        VTEMP.as_slice_mut(),
                    );

                    PNT2D[1] = VTEMP[1];
                    PNT2D[2] = VTEMP[2];

                    W = ZZWIND2D(save.SVNVRT, save.SVFPOL.as_slice(), PNT2D.as_slice(), ctx)?;

                    //
                    // Any non-zero winding number indicates that the
                    // FOV polygon wraps around the point representing
                    // the intercept of the target direction with the
                    // FOV plane.
                    //
                    *VISTAT = (W != 0);
                } else {
                    //
                    // This is an unexpected FOV shape. We should have
                    // prevented this problem in the initialization step,
                    // but we repeat the check here for safety.
                    //
                    SETMSG(b"Instrument #\'s FOV has shape #; the only supported shapes are ELLIPSE, CIRCLE, and POLYGON.", ctx);
                    ERRCH(b"#", &save.SVINAM, ctx);
                    ERRCH(b"#", &save.SVISHP, ctx);
                    SIGERR(b"SPICE(INVALIDSHAPE)", ctx)?;
                    CHKOUT(b"ZZGFFVST", ctx)?;
                    return Ok(());
                }
                //
                // We've performed visibility tests for elliptical or
                // polygonal FOVs. VISTAT is set.
                //
            }
            //
            // We've processed the intercept found by the INRYPL call,
            // or, if the intercept count was not 1, indicated that the
            // target is not visible. VISTAT is set.
            //
        }
        //
        // We've processed both the ray and point ephemeris object
        // cases. VISTAT is set.
        //
    }
    //
    // We've processed all target representation/FOV shape cases.
    //

    CHKOUT(b"ZZGFFVST", ctx)?;
    Ok(())
}
