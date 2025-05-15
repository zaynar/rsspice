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

/// State Transformation Matrix
///
/// Return the state transformation matrix from one frame to
/// another at a specified epoch.
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
///  ET         I   Epoch of the state transformation matrix.
///  XFORM      O   A state transformation matrix.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FROM     is the name of a reference frame in which a state is
///           known.
///
///  TO       is the name of a reference frame in which it is
///           desired to represent the state.
///
///  ET       is the epoch in ephemeris seconds past the epoch of
///           J2000 (TDB) at which the state transformation matrix
///           should be evaluated.
/// ```
///
/// # Detailed Output
///
/// ```text
///  XFORM    is the state transformation matrix that transforms
///           states from the reference frame FROM to the frame TO
///           at epoch ET. If (x, y, z, dx, dy, dz) is a state
///           relative to the frame FROM then the vector ( x', y',
///           z', dx', dy', dz' ) is the same state relative to the
///           frame TO at epoch ET. Here the vector ( x', y', z',
///           dx', dy', dz' ) is defined by the equation:
///
///              .-   -.     .-          -.   .-  -.
///              | x'  |     |            |   | x  |
///              | y'  |     |            |   | y  |
///              | z'  |  =  |   XFORM    |   | z  |
///              | dx' |     |            |   | dx |
///              | dy' |     |            |   | dy |
///              | dz' |     |            |   | dz |
///              `-   -'     `-          -'   `-  -'
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If sufficient information has not been supplied via loaded
///      SPICE kernels to compute the transformation between the two
///      frames, an error is signaled by a routine in the call tree of
///      this routine.
///
///  2)  If either frame FROM or TO is not recognized, the error
///      SPICE(UNKNOWNFRAME) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides the user level interface to computing state
///  transformations from one reference frame to another.
///
///  Note that the reference frames may be inertial or non-inertial.
///  However, the user must take care that sufficient SPICE kernel
///  information is loaded to provide a complete state transformation
///  path from the FROM frame to the TO frame.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have geodetic coordinates of a station on
///  the surface of the earth and that you need the inertial
///  (J2000) state of this station. The following code fragment
///  illustrates how to transform the position of the station to
///  a J2000 state.
///
///     CALL BODVRD ( 'EARTH', RADII, 3, N, ABC  )
///
///     EQUATR   =  ABC(1)
///     POLAR    =  ABC(3)
///     F        = (EQUATR - POLAR) / EQUATR
///
///     CALL GEOREC ( LONG, LAT, 0.0D0,  EQUATR, F,  ESTATE )
///
///     ESTATE(4) = 0.0D0
///     ESTATE(5) = 0.0D0
///     ESTATE(6) = 0.0D0
///
///     CALL SXFORM ( 'IAU_EARTH', 'J2000',   ET, XFORM  )
///     CALL MXVG   (  XFORM,       ESTATE, 6, 6, JSTATE )
///
///  The state JSTATE is the desired J2000 state of the station.
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
/// -    SPICELIB Version 1.1.1, 05-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
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
///         Minor header changes were made to improve clarity.
///
/// -    SPICELIB Version 1.0.0, 19-SEP-1995 (WLT)
/// ```
pub fn sxform(
    ctx: &mut SpiceContext,
    from: &str,
    to: &str,
    et: f64,
    xform: &mut [[f64; 6]; 6],
) -> crate::Result<()> {
    SXFORM(
        from.as_bytes(),
        to.as_bytes(),
        et,
        xform.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SXFORM ( State Transformation Matrix )
pub fn SXFORM(
    FROM: &[u8],
    TO: &[u8],
    ET: f64,
    XFORM: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut XFORM = DummyArrayMut2D::new(XFORM, 1..=6, 1..=6);
    let mut FCODE: i32 = 0;
    let mut TCODE: i32 = 0;

    //
    // Spicelib Functions
    //

    //
    // Local parameters
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

    CHKIN(b"SXFORM", ctx)?;

    //
    // Initialization.
    //
    if save.FIRST {
        //
        // Initialize counters
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
        FRMCHG(FCODE, TCODE, ET, XFORM.as_slice_mut(), ctx)?;
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

    CHKOUT(b"SXFORM", ctx)?;
    Ok(())
}
