//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const FRNMLN: i32 = 32;

struct SaveVars {
    SVCTR1: StackArray<i32, 2>,
    SVFROM: Vec<u8>,
    SVFCOD: i32,
    SVCTR2: StackArray<i32, 2>,
    SVTO: Vec<u8>,
    SVTCDE: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVFROM = vec![b' '; FRNMLN as usize];
        let mut SVFCOD: i32 = 0;
        let mut SVCTR2 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVTO = vec![b' '; FRNMLN as usize];
        let mut SVTCDE: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            SVCTR1,
            SVFROM,
            SVFCOD,
            SVCTR2,
            SVTO,
            SVTCDE,
            FIRST,
        }
    }
}

/// Position Transformation Matrix
///
/// Return the matrix that transforms position vectors from one
/// specified frame to another at a specified epoch.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FROM       I   Name of the frame to transform from.
///  TO         I   Name of the frame to transform to.
///  ET         I   Epoch of the rotation matrix.
///  ROTATE     O   A rotation matrix.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FROM     is the name of a reference frame in which a position
///           vector is known.
///
///  TO       is the name of a reference frame in which it is desired
///           to represent a position vector.
///
///  ET       is the epoch in ephemeris seconds past the epoch of J2000
///           (TDB) at which the position transformation matrix ROTATE
///           should be evaluated.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ROTATE   is the matrix that transforms position vectors from
///           the reference frame FROM to the frame TO at epoch ET.
///           If (x, y, z) is a position relative to the frame FROM
///           then the vector ( x', y', z') is the same position
///           relative to the frame TO at epoch ET. Here the
///           vector ( x', y', z' ) is defined by the equation:
///
///              .-   -.     .-        -.   .-  -.
///              | x'  |     |          |   | x  |
///              | y'  |  =  |  ROTATE  |   | y  |
///              | z'  |     |          |   | z  |
///              `-   -'     `-        -'   `-  -'
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If sufficient information has not been supplied via loaded
///      SPICE kernels to compute the transformation between the
///      two frames, an error is signaled by a routine
///      in the call tree of this routine.
///
///  2)  If either frame FROM or TO is not recognized, the error
///      SPICE(UNKNOWNFRAME) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides the user level interface to computing
///  position transformations from one reference frame to another.
///
///  Note that the reference frames may be inertial or non-inertial.
///  However, the user must take care that sufficient SPICE kernel
///  information is loaded to provide a complete position
///  transformation path from the FROM frame to the TO frame.
///
///  A common type of reference frame transformation is one from one
///  time-dependent frame to another, where the orientations of the
///  frames are computed at different epochs. For example, a remote
///  sensing application may compute the transformation from a target
///  body-fixed frame, with its orientation evaluated at the epoch of
///  photon emission, to a spacecraft instrument frame, with its
///  orientation evaluated at the epoch of photon reception. The
///  SPICELIB routine PXFRM2 computes this type of frame
///  transformation.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have geodetic coordinates of a station on the
///  surface of the earth and that you need the inertial (J2000)
///  position of this station. The following code fragment
///  illustrates how to transform the position of the station to a
///  J2000 position.
///
///     CALL BODVRD ( 'EARTH', RADII, 3, N, ABC  )
///
///     EQUATR   =  ABC(1)
///     POLAR    =  ABC(3)
///     F        = (EQUATR - POLAR) / EQUATR
///
///     CALL GEOREC ( LONG, LAT, 0.0D0,  EQUATR, F, EPOS )
///
///     CALL PXFORM ( 'IAU_EARTH', 'J2000', ET,  ROTATE  )
///     CALL MXV    (  ROTATE,      EPOS,   JPOS         )
///
///  The position JPOS is the desired J2000 position of the station.
/// ```
///
/// # Author and Institution
///
/// ```text
///  C.H. Acton         (JPL)
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 09-AUG-2021 (NJB) (JDR)
///
///         Edited the header to comply with NAIF standard. Updated the
///         $Particulars header section to mention PXFRM2.
///
/// -    SPICELIB Version 1.1.0, 23-SEP-2013 (BVS)
///
///         Updated to save the input frame names and POOL state counters
///         and to do frame name-ID conversions only if the counters have
///         changed.
///
/// -    SPICELIB Version 1.0.3, 27-FEB-2008 (BVS)
///
///         Added FRAMES to the $Required_Reading section.
///
/// -    SPICELIB Version 1.0.2, 23-OCT-2005 (NJB)
///
///         Header example had invalid flattening factor computation;
///         this was corrected. Reference to BODVAR in header was
///         replaced with reference to BODVRD.
///
/// -    SPICELIB Version 1.0.1, 29-JUL-2003 (NJB) (CHA)
///
///         Various header corrections were made.
///
/// -    SPICELIB Version 1.0.0, 05-APR-1999 (WLT)
/// ```
pub fn pxform(
    ctx: &mut SpiceContext,
    from: &str,
    to: &str,
    et: f64,
    rotate: &mut [[f64; 3]; 3],
) -> crate::Result<()> {
    PXFORM(
        from.as_bytes(),
        to.as_bytes(),
        et,
        rotate.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PXFORM ( Position Transformation Matrix )
pub fn PXFORM(
    FROM: &[u8],
    TO: &[u8],
    ET: f64,
    ROTATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ROTATE = DummyArrayMut2D::new(ROTATE, 1..=3, 1..=3);
    let mut FCODE: i32 = 0;
    let mut TCODE: i32 = 0;

    //
    // Spicelib Functions
    //

    //
    // Local parameters.
    //

    //
    // Saved frame name length.
    //

    //
    // Local Variables.
    //

    //
    // Saved frame name/ID item declarations.
    //

    //
    // Saved frame name/ID items.
    //

    //
    // Initial values.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"PXFORM", ctx)?;

    //
    // Initialization.
    //
    if save.FIRST {
        //
        // Initialize counters.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR2.as_slice_mut(), ctx);

        save.FIRST = false;
    }

    ZZNAMFRM(
        save.SVCTR1.as_slice_mut(),
        &mut save.SVFROM,
        &mut save.SVFCOD,
        FROM,
        &mut FCODE,
        ctx,
    )?;
    ZZNAMFRM(
        save.SVCTR2.as_slice_mut(),
        &mut save.SVTO,
        &mut save.SVTCDE,
        TO,
        &mut TCODE,
        ctx,
    )?;

    //
    // Only non-zero id-codes are legitimate frame id-codes.  Zero
    // indicates that the frame wasn't recognized.
    //
    if ((FCODE != 0) && (TCODE != 0)) {
        REFCHG(FCODE, TCODE, ET, ROTATE.as_slice_mut(), ctx)?;
    } else if ((FCODE == 0) && (TCODE == 0)) {
        SETMSG(
            b"Neither of the frames # or # was recognized as a known reference frame. ",
            ctx,
        );
        ERRCH(b"#", FROM, ctx);
        ERRCH(b"#", TO, ctx);
        SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;
    } else if (FCODE == 0) {
        SETMSG(
            b"The frame # was not recognized as a known reference frame. ",
            ctx,
        );
        ERRCH(b"#", FROM, ctx);
        SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;
    } else if (TCODE == 0) {
        SETMSG(
            b"The frame # was not recognized as a known reference frame. ",
            ctx,
        );
        ERRCH(b"#", TO, ctx);
        SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;
    }

    CHKOUT(b"PXFORM", ctx)?;
    Ok(())
}
