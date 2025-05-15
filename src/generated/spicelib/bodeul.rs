//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXANG: i32 = 200;
const MXPHAS: i32 = 4;
const MXPOLY: i32 = 3;
const NAMLEN: i32 = 32;

struct SaveVars {
    BREF: Vec<u8>,
    DTYPE: Vec<u8>,
    ITEM: Vec<u8>,
    ITEM2: Vec<u8>,
    AC: StackArray<f64, 200>,
    CONEPC: f64,
    COSTH: StackArray<f64, 200>,
    D: f64,
    DC: StackArray<f64, 200>,
    DCOEF: StackArray<f64, 3>,
    DELTA: f64,
    DPVAL: f64,
    EPOCH: f64,
    EULANG: StackArray<f64, 6>,
    J2BFX: StackArray2D<f64, 9>,
    J2REF: StackArray2D<f64, 9>,
    PHI: f64,
    RCOEF: StackArray<f64, 3>,
    RF2BFX: StackArray2D<f64, 9>,
    SINTH: StackArray<f64, 200>,
    T: f64,
    TCOEF: ActualArray<f64>,
    THETA: f64,
    WC: StackArray<f64, 200>,
    WCOEF: StackArray<f64, 3>,
    DEG: i32,
    DIM: i32,
    J2CODE: i32,
    K: i32,
    M: i32,
    NA: i32,
    ND: i32,
    NL: i32,
    NPHASE: i32,
    NPHSCO: i32,
    NTHETA: i32,
    NW: i32,
    REF: i32,
    REFID: i32,
    FIRST: bool,
    FOUND: bool,
    FOUND2: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BREF = vec![b' '; NAMLEN as usize];
        let mut DTYPE = vec![b' '; 1 as usize];
        let mut ITEM = vec![b' '; NAMLEN as usize];
        let mut ITEM2 = vec![b' '; NAMLEN as usize];
        let mut AC = StackArray::<f64, 200>::new(1..=MAXANG);
        let mut CONEPC: f64 = 0.0;
        let mut COSTH = StackArray::<f64, 200>::new(1..=MAXANG);
        let mut D: f64 = 0.0;
        let mut DC = StackArray::<f64, 200>::new(1..=MAXANG);
        let mut DCOEF = StackArray::<f64, 3>::new(0..=(MXPOLY - 1));
        let mut DELTA: f64 = 0.0;
        let mut DPVAL: f64 = 0.0;
        let mut EPOCH: f64 = 0.0;
        let mut EULANG = StackArray::<f64, 6>::new(1..=6);
        let mut J2BFX = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut J2REF = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut PHI: f64 = 0.0;
        let mut RCOEF = StackArray::<f64, 3>::new(0..=(MXPOLY - 1));
        let mut RF2BFX = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut SINTH = StackArray::<f64, 200>::new(1..=MAXANG);
        let mut T: f64 = 0.0;
        let mut TCOEF = ActualArray::<f64>::new(1..=(MXPHAS * MAXANG));
        let mut THETA: f64 = 0.0;
        let mut WC = StackArray::<f64, 200>::new(1..=MAXANG);
        let mut WCOEF = StackArray::<f64, 3>::new(0..=(MXPOLY - 1));
        let mut DEG: i32 = 0;
        let mut DIM: i32 = 0;
        let mut J2CODE: i32 = 0;
        let mut K: i32 = 0;
        let mut M: i32 = 0;
        let mut NA: i32 = 0;
        let mut ND: i32 = 0;
        let mut NL: i32 = 0;
        let mut NPHASE: i32 = 0;
        let mut NPHSCO: i32 = 0;
        let mut NTHETA: i32 = 0;
        let mut NW: i32 = 0;
        let mut REF: i32 = 0;
        let mut REFID: i32 = 0;
        let mut FIRST: bool = false;
        let mut FOUND: bool = false;
        let mut FOUND2: bool = false;

        FIRST = true;
        FOUND = false;

        Self {
            BREF,
            DTYPE,
            ITEM,
            ITEM2,
            AC,
            CONEPC,
            COSTH,
            D,
            DC,
            DCOEF,
            DELTA,
            DPVAL,
            EPOCH,
            EULANG,
            J2BFX,
            J2REF,
            PHI,
            RCOEF,
            RF2BFX,
            SINTH,
            T,
            TCOEF,
            THETA,
            WC,
            WCOEF,
            DEG,
            DIM,
            J2CODE,
            K,
            M,
            NA,
            ND,
            NL,
            NPHASE,
            NPHSCO,
            NTHETA,
            NW,
            REF,
            REFID,
            FIRST,
            FOUND,
            FOUND2,
        }
    }
}

/// Return Euler angles for a body
///
/// Return the Euler angles needed to compute the transformation from
/// inertial to body-fixed coordinates for any body in the kernel
/// pool.
///
/// # Required Reading
///
/// * [PCK](crate::required_reading::pck)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  BODY       I   ID code of body.
///  ET         I   Epoch of transformation.
///  RA         O   Right ascension of the (IAU) north pole.
///  DEC        O   Declination of the (IAU) north pole of the body.
///  W          O   Prime meridian rotation angle.
///  LAMBDA     O   Angle between the prime meridian and longitude of
///                 longest axis.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BODY     is the integer ID code of the body for which the
///           transformation is requested. Bodies are numbered
///           according to the standard NAIF numbering scheme.
///
///  ET       is the epoch at which the transformation is
///           requested.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RA,
///  DEC      are the right ascension and declination of the
///           (IAU) north pole of the body at the epoch of
///           transformation. RA and DEC are given in radians.
///
///  W        is the angle between the ascending node of the
///           body-fixed equatorial plane on the inertial
///           equatorial plane and the prime meridian of the body.
///           The node is the cross product of the inertial
///           frame's Z-axis with the Z-axis of the body-fixed
///           frame. The angle is measured in the positive
///           (counterclockwise) sense about the body-fixed
///           Z-axis, from the node to the prime meridian. W is
///           given in radians.
///
///  LAMBDA   is the angle between the prime meridian and the
///           longest axis of the tri-axial ellipsoid which
///           models the body. LAMBDA is given in radians.
///           See the $Particulars section below for further
///           discussion.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If any of the PCK keywords required to compute the angles are
///      not available in the kernel pool, an error is signaled by a
///      routine in the call tree of this routine.
///
///  2)  If the number of phase terms is insufficient, the error
///      SPICE(INSUFFICIENTANGLES) is signaled.
///
///  3)  If, for a given body, both forms of the kernel variable names
///
///         BODY<body ID>_CONSTANTS_JED_EPOCH
///         BODY<body ID>_CONSTS_JED_EPOCH
///
///      are found in the kernel pool, the error
///      SPICE(COMPETINGEPOCHSPEC) is signaled. This is done
///      regardless of whether the values assigned to the kernel
///      variable names match.
///
///  4)  If, for a given body, both forms of the kernel variable names
///
///         BODY<body ID>_CONSTANTS_REF_FRAME
///         BODY<body ID>_CONSTS_REF_FRAME
///
///      are found in the kernel pool, the error
///      SPICE(COMPETINGFRAMESPEC) is signaled. This is done
///      regardless of whether the values assigned to the kernel
///      variable names match.
///
///  5)  If the central body associated with the input BODY, whether
///      a system barycenter or BODY itself, has associated phase
///      angles (aka nutation precession angles), and the kernel
///      variable BODY<body ID>_MAX_PHASE_DEGREE for the central
///      body is present but has a value outside the range 1:3,
///      the error SPICE(DEGREEOUTOFRANGE) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  A text or binary PCK containing orientation data for the
///  body designated by BODY must be loaded at the time this
///  routine is called.
///
///  Normally PCK files are loaded during program initialization;
///  they need not be re-loaded prior to each call to this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  Applications that need to compute the transformation between
///  body-fixed and inertial frames usually can call the higher-level
///  routine PXFORM instead of this routine.
///
///
///  If there exists high-precision binary PCK kernel information for
///  the body at the requested time, the angles, W, DELTA and PHI are
///  computed directly from that file. These angles are then used to
///  compute RA, DEC and W. The most recently loaded binary PCK file
///  has first priority followed by previously loaded binary PCK files
///  in backward time order. If no binary PCK file has been loaded,
///  the text P_constants kernel file (PCK) is used.
///
///  If there is only text PCK kernel information, it is expressed in
///  terms of RA, DEC and W (same W as above), where
///
///     RA    = PHI - HALFPI()
///     DEC   = HALFPI() - DELTA
///
///  RA, DEC, and W are defined as follows in the text PCK file:
///
///     RA  = RA0  + RA1*T  + RA2*T*T   + a  sin theta
///                                        i          i
///
///     DEC = DEC0 + DEC1*T + DEC2*T*T  + d  cos theta
///                                        i          i
///
///     W   = W0   + W1*d   + W2*d*d    + w  sin theta
///                                        i          i
///
///  where:
///
///     d = days past J2000.
///
///     T = Julian centuries past J2000.
///
///     a , d , and w  arrays apply to satellites only.
///      i   i       i
///
///     theta  = THETA0 * THETA1*T are specific to each planet.
///          i
///
///    These angles -- typically nodal rates -- vary in number and
///    definition from one planetary system to the next.
///
///
///  The prime meridian offset LAMBDA
///  ================================
///
///  The offset LAMBDA is the value specified by the kernel variable
///
///     BODYnnn_LONG_AXIS
///
///  if such a variable is defined.
///
///  The offset LAMBDA is a constant for a given body. LAMBDA serves
///  to distinguish between the planetocentric prime meridian, which
///  is provided in the PCK file, and the meridian that passes through
///  the +X axis of a reference frame aligned with the axes of the
///  body's reference ellipsoid.
///
///  However, SPICE Toolkit makes no use of LAMBDA. In order to
///  perform geometry computations using a reference ellipsoid not
///  aligned with a body's planetocentric reference frame, a
///  fixed-offset (aka "TK") reference frame aligned with the
///  ellipsoid's axes should be specified in a frames kernel. Note
///  that a fixed-offset frame may be rotated from the planetocentric
///  frame about an arbitrary axis, not just the polar axis.
///
///  See the Frames Required Reading frames.req for details on
///  constructing a fixed-offset frame specification.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, BODEUL is used to get the unit
///  vector (POLE) parallel to the north pole of a target body (BODY)
///  at a specific epoch (ET).
///
///     CALL BODEUL ( BODY, ET, RA, DEC, W, LAMBDA )
///     CALL RADREC ( 1.D0, RA,  DEC, POLE )
///
///  Note that the items necessary to compute the Euler angles
///  must have been loaded into the kernel pool (by one or more
///  previous calls to LDPOOL).
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  K.S. Zukor         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.0.0, 14-APR-2021 (NJB) (JDR)
///
///         The routine was updated to support user-defined maximum phase
///         angle degrees. The additional text kernel kernel variable name
///         BODYnnn_MAX_PHASE_DEGREE must be used when the phase angle
///         polynomials have degree higher than 1. The maximum allowed
///         degree is 3.
///
///         The kernel variable names
///
///            BODY#_CONSTS_REF_FRAME
///            BODY#_CONSTS_JED_EPOCH
///
///         are now recognized.
///
///         Error handling was upgraded to check FAILED() between kernel
///         data lookups and computations.
///
///         Now SAVEs all local variables.
///
///         Edited the header to comply with NAIF standard. Moved NAIF_IDS
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 4.2.0, 02-MAR-2016 (BVS)
///
///         BUG FIX: changed available room in the BODVCD call
///         fetching 'NUT_PREC_ANGLES' from MAXANG to MAXANG*2.
///
///         Fixed indention in some header sections.
///
///         Removed BODEUL: prefix from the text of the long
///         error for insufficient angles.
///
///      Last update was 24-APR-2014 (NJB)
///
///         Corrected the brief and detailed descriptions of W.
///
/// -    SPICELIB Version 4.1.0, 24-OCT-2005 (NJB)
///
///         Calls to ZZBODVCD have been replaced with calls to
///         BODVCD.
///
/// -    SPICELIB Version 4.0.0, 13-FEB-2004 (NJB)
///
///         Code has been updated to support satellite ID codes in the
///         range 10000 to 99999 and to allow nutation precession angles
///         to be associated with any object.
///
///         Implementation changes were made to improve robustness
///         of the code.
///
/// -    SPICELIB Version 3.1.0, 21-MAR-1995 (KSZ)
///
///         REF frame is now passed correctly as a character string.
///
/// -    SPICELIB Version 3.0.0, 10-MAR-1994 (KSZ)
///
///         Ability to get Euler angles from binary PCK file added.
///         This uses the new routine PCKEUL.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 04-SEP-1991 (NJB)
///
///         Updated to handle P_constants referenced to different epochs
///         and inertial reference frames.
///
/// -    SPICELIB Version 1.1.0, 02-NOV-1990 (NJB)
///
///         Allowed number of nutation precession angles increased to
///         100.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 4.1.0, 24-OCT-2005 (NJB)
///
///         Calls to ZZBODVCD have been replaced with calls to
///         BODVCD.
///
/// -    SPICELIB Version 4.0.0, 13-FEB-2004 (NJB)
///
///         Code has been updated to support satellite ID codes in the
///         range 10000 to 99999 and to allow nutation precession angles
///         to be associated with any object.
///
///         Calls to deprecated kernel pool access routine RTPOOL
///         were replaced by calls to GDPOOL.
///
///         Calls to BODVAR have been replaced with calls to
///         ZZBODVCD.
///
/// -    SPICELIB Version 3.1.0, 21-MAR-1995 (KSZ)
///
///         REF frame is now passed correctly as a character string.
///
/// -    SPICELIB Version 3.0.0, 10-MAR-1994 (KSZ)
///
///         BODEUL now uses new software to check for the
///         existence of binary PCK files, search the for
///         data corresponding to the requested body and time,
///         and return the appropriate Euler angles, using the
///         new routine PCKEUL. Otherwise the code calculates
///         the Euler angles from the P_constants kernel file.
///
/// -    SPICELIB Version 2.0.0, 04-SEP-1991 (NJB)
///
///         Updated to handle P_constants referenced to different epochs
///         and inertial reference frames.
///
///         Updated to handle P_constants referenced to different epochs
///         and inertial reference frames.
///
///         BODEUL now checks the kernel pool for presence of the
///         variables
///
///            BODY#_CONSTANTS_REF_FRAME
///
///         and
///
///            BODY#_CONSTANTS_JED_EPOCH
///
///         where # is the NAIF integer code of the barycenter of a
///         planetary system or of a body other than a planet or
///         satellite. If either or both of these variables are
///         present, the P_constants for BODY are presumed to be
///         referenced to the specified inertial frame or epoch.
///         If the epoch of the constants is not J2000, the input
///         time ET is converted to seconds past the reference epoch.
///         If the frame of the constants is not J2000, the Euler angles
///         defining the rotation from the P_constants' frame to
///         body-fixed coordinates are transformed so that they define
///         the rotation from J2000 coordinates to body-fixed
///         coordinates.
///
/// -    SPICELIB Version 1.1.0, 02-NOV-1990  (NJB)
///
///         Allowed number of nutation precession angles increased to
///         100.
///
/// -    Beta Version 2.0.0, 23-JUN-1989 (HAN)
///
///         Mod angles by two pi. Check to see that right ascension and
///         prime meridian angles are within the range 0 to two pi.
///
///         LAMBDA used to be returned in degrees. It has been corrected
///         to return LAMBDA in radians.
///
/// -    Beta Version 1.1.0, 16-FEB-1989 (IMU) (NJB)
///
///         $Examples section completed. Declarations of unused variables
///         HALFPI and N removed.
/// ```
pub fn bodeul(
    ctx: &mut SpiceContext,
    body: i32,
    et: f64,
    ra: &mut f64,
    dec: &mut f64,
    w: &mut f64,
    lambda: &mut f64,
) -> crate::Result<()> {
    BODEUL(body, et, ra, dec, w, lambda, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure BODEUL ( Return Euler angles for a body )
pub fn BODEUL(
    BODY: i32,
    ET: f64,
    RA: &mut f64,
    DEC: &mut f64,
    W: &mut f64,
    LAMBDA: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Maximum number of coefficients per phase angle polynomial.
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

    CHKIN(b"BODEUL", ctx)?;

    //
    // Get the code for the J2000 frame, if we don't have it yet.
    //
    if save.FIRST {
        IRFNUM(b"J2000", &mut save.J2CODE, ctx);
        save.FIRST = false;
    }

    //
    // Get Euler angles from high precision data file.
    //
    PCKEUL(
        BODY,
        ET,
        &mut save.FOUND,
        &mut save.BREF,
        save.EULANG.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"BODEUL", ctx)?;
        return Ok(());
    }

    if save.FOUND {
        save.PHI = save.EULANG[1];
        save.DELTA = save.EULANG[2];
        *W = save.EULANG[3];

        IRFNUM(&save.BREF, &mut save.REF, ctx);

        if FAILED(ctx) {
            CHKOUT(b"BODEUL", ctx)?;
            return Ok(());
        }

        //
        // The offset of the prime meridian is optional.
        //
        fstr::assign(&mut save.ITEM, b"LONG_AXIS");

        if BODFND(BODY, &save.ITEM, ctx)? {
            BODVCD(
                BODY,
                &save.ITEM,
                1,
                &mut save.NL,
                std::slice::from_mut(LAMBDA),
                ctx,
            )?;
            *LAMBDA = (*LAMBDA * RPD(ctx));
            *LAMBDA = intrinsics::DMOD(*LAMBDA, TWOPI(ctx));
        } else {
            *LAMBDA = 0 as f64;
        }
    } else {
        //
        // Find the body code used to label the reference frame and epoch
        // specifiers for the orientation constants for BODY.
        //
        // For planetary systems, the reference frame and epoch for the
        // orientation constants is associated with the system
        // barycenter, not with individual bodies in the system.  For any
        // other bodies, (the Sun or asteroids, for example) the body's
        // own code is used as the label.
        //
        save.REFID = ZZBODBRY(BODY);

        //
        // Look up the epoch of the constants. The epoch is specified
        // as a Julian ephemeris date. The epoch defaults to J2000.
        //
        // Look for both forms of the JED epoch kernel variable. At
        // most one is allowed to be present.
        //
        fstr::assign(&mut save.ITEM, b"BODY#_CONSTANTS_JED_EPOCH");
        REPMI(&save.ITEM.to_vec(), b"#", save.REFID, &mut save.ITEM, ctx);

        fstr::assign(&mut save.ITEM2, b"BODY#_CONSTS_JED_EPOCH");
        REPMI(&save.ITEM2.to_vec(), b"#", save.REFID, &mut save.ITEM2, ctx);

        GDPOOL(
            &save.ITEM,
            1,
            1,
            &mut save.DIM,
            std::slice::from_mut(&mut save.CONEPC),
            &mut save.FOUND,
            ctx,
        )?;

        if !save.FOUND {
            GDPOOL(
                &save.ITEM2,
                1,
                1,
                &mut save.DIM,
                std::slice::from_mut(&mut save.CONEPC),
                &mut save.FOUND2,
                ctx,
            )?;

            if !save.FOUND2 {
                save.CONEPC = J2000();
            }
        } else {
            //
            // Check for presence of both forms of the variable names.
            //
            DTPOOL(
                &save.ITEM2,
                &mut save.FOUND2,
                &mut save.DIM,
                &mut save.DTYPE,
                ctx,
            )?;

            if save.FOUND2 {
                SETMSG(b"Both kernel variables # and # are present in the kernel pool. At most one form of the kernel variable name may be present.", ctx);
                ERRCH(b"#", &save.ITEM, ctx);
                ERRCH(b"#", &save.ITEM2, ctx);
                SIGERR(b"SPICE(COMPETINGEPOCHSPEC)", ctx)?;
                CHKOUT(b"BODEUL", ctx)?;
                return Ok(());
            }
        }

        if FAILED(ctx) {
            CHKOUT(b"BODEUL", ctx)?;
            return Ok(());
        }

        if (save.FOUND || save.FOUND2) {
            //
            // The reference epoch is returned as a JED.  Convert to
            // ephemeris seconds past J2000.  Then convert the input ET to
            // seconds past the reference epoch.
            //
            save.CONEPC = (SPD() * (save.CONEPC - J2000()));
            save.EPOCH = (ET - save.CONEPC);
        } else {
            save.EPOCH = ET;
        }

        //
        // Look up the reference frame of the constants.  The reference
        // frame is specified by a code recognized by CHGIRF.  The
        // default frame is J2000, symbolized by the code J2CODE.
        //
        IRFNUM(b"J2000", &mut save.J2CODE, ctx);

        fstr::assign(&mut save.ITEM, b"BODY#_CONSTANTS_REF_FRAME");
        REPMI(&save.ITEM.to_vec(), b"#", save.REFID, &mut save.ITEM, ctx);

        fstr::assign(&mut save.ITEM2, b"BODY#_CONSTS_REF_FRAME");
        REPMI(&save.ITEM2.to_vec(), b"#", save.REFID, &mut save.ITEM2, ctx);

        REPMI(&save.ITEM.to_vec(), b"#", save.REFID, &mut save.ITEM, ctx);
        GIPOOL(
            &save.ITEM,
            1,
            1,
            &mut save.DIM,
            std::slice::from_mut(&mut save.REF),
            &mut save.FOUND,
            ctx,
        )?;

        if !save.FOUND {
            GIPOOL(
                &save.ITEM2,
                1,
                1,
                &mut save.DIM,
                std::slice::from_mut(&mut save.REF),
                &mut save.FOUND2,
                ctx,
            )?;

            if !save.FOUND2 {
                save.REF = save.J2CODE;
            }
        } else {
            //
            // Check for presence of both forms of the variable names.
            //
            DTPOOL(
                &save.ITEM2,
                &mut save.FOUND2,
                &mut save.DIM,
                &mut save.DTYPE,
                ctx,
            )?;

            if save.FOUND2 {
                SETMSG(b"Both kernel variables # and # are present in the kernel pool. At most one form of the kernel variable name may be present.", ctx);
                ERRCH(b"#", &save.ITEM, ctx);
                ERRCH(b"#", &save.ITEM2, ctx);
                SIGERR(b"SPICE(COMPETINGFRAMESPEC)", ctx)?;
                CHKOUT(b"BODEUL", ctx)?;
                return Ok(());
            }
        }

        //
        // Whatever the body, it has quadratic time polynomials for
        // the RA and Dec of the pole, and for the rotation of the
        // Prime Meridian.
        //
        fstr::assign(&mut save.ITEM, b"POLE_RA");
        CLEARD(MXPOLY, save.RCOEF.as_slice_mut());
        BODVCD(
            BODY,
            &save.ITEM,
            MXPOLY,
            &mut save.NA,
            save.RCOEF.as_slice_mut(),
            ctx,
        )?;

        fstr::assign(&mut save.ITEM, b"POLE_DEC");
        CLEARD(MXPOLY, save.DCOEF.as_slice_mut());
        BODVCD(
            BODY,
            &save.ITEM,
            MXPOLY,
            &mut save.ND,
            save.DCOEF.as_slice_mut(),
            ctx,
        )?;

        fstr::assign(&mut save.ITEM, b"PM");
        CLEARD(MXPOLY, save.WCOEF.as_slice_mut());
        BODVCD(
            BODY,
            &save.ITEM,
            MXPOLY,
            &mut save.NW,
            save.WCOEF.as_slice_mut(),
            ctx,
        )?;

        //
        // The offset of the prime meridian is optional.
        //
        fstr::assign(&mut save.ITEM, b"LONG_AXIS");

        if BODFND(BODY, &save.ITEM, ctx)? {
            BODVCD(
                BODY,
                &save.ITEM,
                1,
                &mut save.NL,
                std::slice::from_mut(LAMBDA),
                ctx,
            )?;
        } else {
            *LAMBDA = 0 as f64;
        }

        if FAILED(ctx) {
            CHKOUT(b"BODEUL", ctx)?;
            return Ok(());
        }

        //
        // There may be additional nutation and libration (THETA) terms.
        //
        save.NTHETA = 0;
        save.NA = 0;
        save.ND = 0;
        save.NW = 0;

        fstr::assign(&mut save.ITEM, b"NUT_PREC_ANGLES");

        if BODFND(save.REFID, &save.ITEM, ctx)? {
            //
            // Find out whether the maximum phase angle degree has been
            // explicitly set.
            //
            fstr::assign(&mut save.ITEM2, b"MAX_PHASE_DEGREE");

            if BODFND(save.REFID, &save.ITEM2, ctx)? {
                BODVCD(
                    save.REFID,
                    &save.ITEM2,
                    1,
                    &mut save.DIM,
                    std::slice::from_mut(&mut save.DPVAL),
                    ctx,
                )?;

                save.DEG = intrinsics::IDNINT(save.DPVAL);

                if ((save.DEG < 1) || (save.DEG > (MXPHAS - 1))) {
                    SETMSG(b"Maximum phase angle degree for body # must be in the range 1:# but was #.", ctx);
                    ERRINT(b"#", save.REFID, ctx);
                    ERRINT(b"#", (MXPHAS - 1), ctx);
                    ERRINT(b"#", save.DEG, ctx);
                    SIGERR(b"SPICE(DEGREEOUTOFRANGE)", ctx)?;
                    CHKOUT(b"BODEUL", ctx)?;
                    return Ok(());
                }

                save.NPHSCO = (save.DEG + 1);
            } else {
                //
                // The default degree is 1, yielding two coefficients.
                //
                save.NPHSCO = 2;
            }
            //
            // There is something a bit obscure going on below. BODVCD
            // loads the array TCOEF in the following order
            //
            //    TCOEF(1,1), TCOEF(2,1), ... TCOEF(NPHSCO),
            //    TCOEF(1,2), TCOEF(2,2), ...
            //
            // The NTHETA that comes back is the total number of items
            // loaded, but we will need the actual limit on the second
            // dimension. That is --- NTHETA / NPHSCO.
            //
            BODVCD(
                save.REFID,
                &save.ITEM,
                (MAXANG * MXPHAS),
                &mut save.NTHETA,
                save.TCOEF.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"BODEUL", ctx)?;
                return Ok(());
            }
            //
            // NPHSCO is at least 1; this division is safe.
            //
            save.NPHASE = (save.NTHETA / save.NPHSCO);
        }

        fstr::assign(&mut save.ITEM, b"NUT_PREC_RA");

        if BODFND(BODY, &save.ITEM, ctx)? {
            BODVCD(
                BODY,
                &save.ITEM,
                MAXANG,
                &mut save.NA,
                save.AC.as_slice_mut(),
                ctx,
            )?;
        }

        fstr::assign(&mut save.ITEM, b"NUT_PREC_DEC");

        if BODFND(BODY, &save.ITEM, ctx)? {
            BODVCD(
                BODY,
                &save.ITEM,
                MAXANG,
                &mut save.ND,
                save.DC.as_slice_mut(),
                ctx,
            )?;
        }

        fstr::assign(&mut save.ITEM, b"NUT_PREC_PM");

        if BODFND(BODY, &save.ITEM, ctx)? {
            BODVCD(
                BODY,
                &save.ITEM,
                MAXANG,
                &mut save.NW,
                save.WC.as_slice_mut(),
                ctx,
            )?;
        }

        if FAILED(ctx) {
            CHKOUT(b"BODEUL", ctx)?;
            return Ok(());
        }

        if (intrinsics::MAX0(&[save.NA, save.ND, save.NW]) > save.NTHETA) {
            SETMSG(
                b"Insufficient number of nutation/precession angles for body * at time #.",
                ctx,
            );
            ERRINT(b"*", BODY, ctx);
            ERRDP(b"#", ET, ctx);
            SIGERR(b"SPICE(INSUFFICIENTANGLES)", ctx)?;
            CHKOUT(b"BODEUL", ctx)?;
            return Ok(());
        }

        //
        // Evaluate the time polynomials at EPOCH.
        //
        save.D = (save.EPOCH / SPD());
        save.T = (save.D / 36525.0);

        *RA = (save.RCOEF[0] + (save.T * (save.RCOEF[1] + (save.T * save.RCOEF[2]))));
        *DEC = (save.DCOEF[0] + (save.T * (save.DCOEF[1] + (save.T * save.DCOEF[2]))));
        *W = (save.WCOEF[0] + (save.D * (save.WCOEF[1] + (save.D * save.WCOEF[2]))));

        //
        // Add nutation and libration as appropriate.
        //
        for I in 1..=save.NPHASE {
            if (save.NPHSCO == 2) {
                //
                // This case is so common that we'll deal with it
                // separately. We'll avoid unnecessary arithmetic
                // operations.
                //
                save.K = (1 + (2 * (I - 1)));
                save.M = (1 + save.K);

                save.THETA = ((save.TCOEF[save.K] + (save.T * save.TCOEF[save.M])) * RPD(ctx));
            } else {
                //
                // THETA and DTHETA have higher-order terms; there are
                // NPHSCO coefficients for each angle.
                //
                save.THETA = 0.0;

                for J in 1..=save.NPHSCO {
                    //
                    // K is the start index for the coefficients of the
                    // Ith angle.
                    //
                    save.K = (J + (save.NPHSCO * (I - 1)));

                    save.THETA = (save.THETA + (f64::powi(save.T, (J - 1)) * save.TCOEF[save.K]));
                }

                save.THETA = (save.THETA * RPD(ctx));
            }

            save.SINTH[I] = f64::sin(save.THETA);
            save.COSTH[I] = f64::cos(save.THETA);
        }

        //
        // Adjust RA, DEC, W by their librations and nutations.
        //
        *RA = (*RA + VDOTG(save.AC.as_slice(), save.SINTH.as_slice(), save.NA));
        *DEC = (*DEC + VDOTG(save.DC.as_slice(), save.COSTH.as_slice(), save.ND));
        *W = (*W + VDOTG(save.WC.as_slice(), save.SINTH.as_slice(), save.NW));

        //
        // Convert from degrees to radians and mod by two pi.
        //
        *RA = (*RA * RPD(ctx));
        *DEC = (*DEC * RPD(ctx));
        *W = (*W * RPD(ctx));
        *LAMBDA = (*LAMBDA * RPD(ctx));

        *RA = intrinsics::DMOD(*RA, TWOPI(ctx));
        *DEC = intrinsics::DMOD(*DEC, TWOPI(ctx));
        *W = intrinsics::DMOD(*W, TWOPI(ctx));
        *LAMBDA = intrinsics::DMOD(*LAMBDA, TWOPI(ctx));

        //
        // Convert to Euler angles.
        //
        save.PHI = (*RA + HALFPI(ctx));
        save.DELTA = (HALFPI(ctx) - *DEC);
    }

    //
    // Convert the angles to the J2000 frame if they are not already
    // referenced to J2000.
    //
    if (save.REF != save.J2CODE) {
        //
        // Find the transformation from the J2000 frame to the frame
        // designated by REF.  Form the transformation from `REF'
        // coordinates to body-fixed coordinates, using our Euler angles.
        // Compose the transformations to obtain the J2000-to-body-fixed
        // transformation.  Decompose this transformation into Euler
        // angles.
        //
        IRFROT(save.J2CODE, save.REF, save.J2REF.as_slice_mut(), ctx)?;

        EUL2M(
            *W,
            save.DELTA,
            save.PHI,
            3,
            1,
            3,
            save.RF2BFX.as_slice_mut(),
            ctx,
        )?;

        MXM(
            save.RF2BFX.as_slice(),
            save.J2REF.as_slice(),
            save.J2BFX.as_slice_mut(),
        );

        M2EUL(
            save.J2BFX.as_slice(),
            3,
            1,
            3,
            W,
            &mut save.DELTA,
            &mut save.PHI,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"BODEUL", ctx)?;
            return Ok(());
        }
    }

    //
    // The Euler angles now give the transformation from J2000 to
    // body-fixed coordinates at epoch ET seconds past J2000,
    // regardless of the epoch and frame of the orientation constants
    // for the specified body.
    //
    *RA = (save.PHI - HALFPI(ctx));
    *DEC = (HALFPI(ctx) - save.DELTA);

    //
    // Make sure that the prime meridian and right ascension are in
    // the correct range.
    //
    if (*W < 0 as f64) {
        *W = (*W + TWOPI(ctx));
    }

    if (*RA < 0 as f64) {
        *RA = (*RA + TWOPI(ctx));
    }

    CHKOUT(b"BODEUL", ctx)?;
    Ok(())
}
