//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
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
const CTRSIZ: i32 = 2;
const RNAME: &[u8] = b"PHASEQ";
const MAXL: i32 = 36;

struct SaveVars {
    SVCTR1: StackArray<i32, 2>,
    SVTARG: Vec<u8>,
    SVTGID: i32,
    SVFND1: bool,
    SVCTR2: StackArray<i32, 2>,
    SVILMN: Vec<u8>,
    SVICDE: i32,
    SVFND2: bool,
    SVCTR3: StackArray<i32, 2>,
    SVOBSR: Vec<u8>,
    SVOBSN: i32,
    SVFND3: bool,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVTARG = vec![b' '; MAXL as usize];
        let mut SVTGID: i32 = 0;
        let mut SVFND1: bool = false;
        let mut SVCTR2 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVILMN = vec![b' '; MAXL as usize];
        let mut SVICDE: i32 = 0;
        let mut SVFND2: bool = false;
        let mut SVCTR3 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVOBSR = vec![b' '; MAXL as usize];
        let mut SVOBSN: i32 = 0;
        let mut SVFND3: bool = false;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            SVCTR1,
            SVTARG,
            SVTGID,
            SVFND1,
            SVCTR2,
            SVILMN,
            SVICDE,
            SVFND2,
            SVCTR3,
            SVOBSR,
            SVOBSN,
            SVFND3,
            FIRST,
        }
    }
}

/// Phase angle quantity between bodies centers
///
/// Compute the apparent phase angle for a target, observer,
/// illuminator set of ephemeris objects.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ET         I   Ephemeris seconds past J2000 TDB.
///  TARGET     I   Target body name.
///  ILLMN      I   Illuminating body name.
///  OBSRVR     I   Observer body.
///  ABCORR     I   Aberration correction flag.
///
///  The function returns the value of phase angle.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is the time in ephemeris seconds past J2000 TDB at which
///           to compute the phase angle.
///
///  TARGET   is the name of the target body. Optionally, you may
///           supply a string containing the integer ID code
///           for the object. For example both 'MOON' and '301'
///           are legitimate strings that indicate the Moon is the
///           target body. The TARGET string lack sensitivity to
///           case, leading and trailing blanks.
///
///  ILLMN    is the name of the illuminating body. Optionally, you may
///           supply a string containing the integer ID code
///           for the object. For example both 'SUN' and '10'
///           are legitimate strings that indicate the sun is the
///           illuminating body. The ILLMN string lack sensitivity
///           to case, leading and trailing blanks.
///
///           In most cases, ILLMN is the sun.
///
///  OBSRVR   is the name of the observer body. Optionally, you may
///           supply a string containing the integer ID code
///           for the object. For example both 'MOON' and '301'
///           are legitimate strings that indicate the Moon is the
///           observer body. The OBSRVR string lack sensitivity
///           to case, leading and trailing blanks.
///
///  ABCORR   is the string description of the aberration corrections
///           to apply to the state evaluations to account for one-way
///           light time and stellar aberration. The ABCORR string lack
///           sensitivity to case, leading and trailing blanks.
///
///           This routine accepts only reception mode aberration
///           corrections. See the header of SPKEZR for a detailed
///           description of the aberration correction options.
///           For convenience, the appropriate aberration options are
///           listed below:
///
///              'NONE'     Apply no correction. Returns the "true"
///                         geometric state.
///
///              'LT'       "Reception" case: correct for
///                         one-way light time using a Newtonian
///                         formulation.
///
///              'LT+S'     "Reception" case: correct for
///                         one-way light time and stellar
///                         aberration using a Newtonian
///                         formulation.
///
///              'CN'       "Reception" case: converged
///                         Newtonian light time correction.
///
///              'CN+S'     "Reception" case: converged
///                         Newtonian light time and stellar
///                         aberration corrections.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the optionally light-time corrected phase
///  angle between TARGET and ILLMN as observed from OBSRVR.
///
///  The range of the phase angle is [0, pi].
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the body name to SPICE ID look-up fails for any of the
///      TARGET, ILLMN, or OBSRVR names, the error
///      SPICE(IDCODENOTFOUND) is signaled.
///
///  2)  If the aberration correct, ABCORR, indicates a transmission
///      based correction, the error SPICE(INVALIDOPTION) is signaled.
///
///  3)  If the TARGET, ILLMN, and OBSRVR are not unique, the error
///      SPICE(BODIESNOTDISTINCT) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  Appropriate kernels must be loaded by the calling program before
///  this routine is called.
///
///  The following data are required:
///
///  -  SPK data: ephemeris data for the observer, illuminator, and
///     target must be loaded. If aberration corrections are used,
///     the states of the ephemeris bodies relative to the solar
///     system barycenter must be calculable from the available
///     ephemeris data. Typically ephemeris data are made
///     available by loading one or more SPK files using FURNSH.
///
///  The following data may be required:
///
///  -  Frame data: if a frame definition not built into SPICE is
///     required, that definition must be available in the kernel
///     pool. Typically frame definitions are supplied by loading a
///     frame kernel using FURNSH.
///
///  -  Orientation data: if a CK based frame is used in this routine's
///     state computation, then at least one CK and corresponding SCLK
///     kernel is required. If dynamic frames are used, additional
///     SPK, PCK, CK, or SCLK kernels may be required.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine returns the phase angle using the location of the
///  bodies (if point objects) or the center of the bodies (if finite
///  bodies).
///
///
///
///                    ILLMN     OBSRVR
///    ILLMN as seen      ^       /
///    from TARGET at     |      /
///    ET - LT.           |     /
///                      >|..../< phase angle
///                       |   /
///                     . |  /
///                   .   | /
///                  .    |v        TARGET as seen from OBSRVR
///            SEP   .  TARGET      at ET
///                   .  /
///                     /
///                    v
///
///
///
///     PI = SEP + PHASE
///
///     so
///
///     PHASE = PI - SEP
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Determine the time windows from December 1, 2006 UTC to
///     January 31, 2007 UTC for which the sun-moon-earth configuration
///     phase angle satisfies the relation conditions with respect to a
///     reference value of 0.57598845 radians (the phase angle at
///     January 1, 2007 00:00:00.000 UTC, 33.001707 degrees). Also
///     determine the time windows corresponding to the local maximum
///     and minimum phase angles, and the absolute maximum and minimum
///     phase angles during the search interval. The configuration
///     defines the Sun as the illuminator, the Moon as the target, and
///     the Earth as the observer.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: phaseq_ex1.tm
///
///        This meta-kernel is intended to support operation of SPICE
///        example programs. The kernels shown here should not be
///        assumed to contain adequate or correct versions of data
///        required by SPICE-based user applications.
///
///        In order for an application to use this meta-kernel, the
///        kernels referenced here must be present in the user's
///        current working directory.
///
///        The names and contents of the kernels referenced
///        by this meta-kernel are as follows:
///
///           File name                     Contents
///           ---------                     --------
///           de421.bsp                     Planetary ephemeris
///           pck00009.tpc                  Planet orientation and
///                                         radii
///           naif0009.tls                  Leapseconds
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'pck00009.tpc',
///                               'naif0009.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM PHASEQ_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include GF parameter declarations:
///     C
///           INCLUDE 'gf.inc'
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      SPD
///           DOUBLE PRECISION      PHASEQ
///
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///     C
///     C     Use the parameter MAXWIN for both the result window size
///     C     and the workspace size.
///     C
///           INTEGER               MAXWIN
///           PARAMETER           ( MAXWIN = 1000 )
///
///     C
///     C     Length of strings:
///     C
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 26 )
///
///           INTEGER               NLOOPS
///           PARAMETER           ( NLOOPS = 7 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(TIMLEN)    RELATE (NLOOPS)
///           CHARACTER*(6)         ABCORR
///           CHARACTER*(6)         ILLMN
///           CHARACTER*(6)         OBSRVR
///           CHARACTER*(6)         TARGET
///           CHARACTER*(TIMLEN)    TIMSTR
///
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2 )
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///           DOUBLE PRECISION      WORK   ( LBCELL : MAXWIN, NWPA )
///           DOUBLE PRECISION      ADJUST
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      ET1
///           DOUBLE PRECISION      FINISH
///           DOUBLE PRECISION      PHASE
///           DOUBLE PRECISION      REFVAL
///           DOUBLE PRECISION      START
///           DOUBLE PRECISION      STEP
///
///           INTEGER               I
///           INTEGER               J
///
///
///     C
///     C     The relation values for the search.
///     C
///           DATA                  RELATE / '=',
///          .                               '<',
///          .                               '>',
///          .                               'LOCMIN',
///          .                               'ABSMIN',
///          .                               'LOCMAX',
///          .                               'ABSMAX'  /
///
///
///     C
///     C     Load kernels.
///     C
///           CALL FURNSH ( 'phaseq_ex1.tm' )
///
///     C
///     C     Initialize windows.
///     C
///           CALL SSIZED ( MAXWIN, RESULT )
///           CALL SSIZED ( 2,      CNFINE )
///
///     C
///     C     Store the time bounds of our search interval in
///     C     the confinement window.
///     C
///           CALL STR2ET ( '2006 DEC 01', ET0 )
///           CALL STR2ET ( '2007 JAN 31', ET1 )
///
///           CALL WNINSD ( ET0, ET1, CNFINE )
///
///     C
///     C     Search using a step size of 1 day (in units of seconds).
///     C     The reference value is 0.57598845. We're not using the
///     C     adjustment feature, so we set ADJUST to zero.
///     C
///           STEP   = SPD()
///           REFVAL = 0.57598845D0
///           ADJUST = 0.D0
///
///     C
///     C     Define the values for target, observer, illuminator, and
///     C     aberration correction.
///     C
///           TARGET = 'MOON'
///           ILLMN  = 'SUN'
///           ABCORR = 'LT+S'
///           OBSRVR = 'EARTH'
///
///           DO J=1, NLOOPS
///
///              WRITE(*,*) 'Relation condition: ', RELATE(J)
///
///     C
///     C        Perform the search. The SPICE window RESULT contains
///     C        the set of times when the condition is met.
///     C
///              CALL GFPA (  TARGET,    ILLMN,  ABCORR, OBSRVR,
///          .                RELATE(J), REFVAL, ADJUST, STEP,
///          .                CNFINE,    MAXWIN, NWPA,   WORK,
///          .                RESULT )
///
///     C
///     C        Display the results.
///     C
///              IF ( WNCARD(RESULT) .EQ. 0 ) THEN
///
///                 WRITE (*, '(A)') 'Result window is empty.'
///
///              ELSE
///
///                 DO I = 1, WNCARD(RESULT)
///     C
///     C              Fetch the endpoints of the Ith interval
///     C              of the result window.
///     C
///                    CALL WNFETD ( RESULT, I, START, FINISH )
///
///                    PHASE = PHASEQ( START, TARGET, ILLMN, OBSRVR,
///          .                         ABCORR )
///                    CALL TIMOUT ( START,
///          .                       'YYYY-MON-DD HR:MN:SC.###',
///          .                       TIMSTR                          )
///
///                    WRITE (*, '(2X,A,F16.9)') 'Start time = '
///          .                               //   TIMSTR, PHASE
///
///
///                    PHASE = PHASEQ( FINISH, TARGET, ILLMN, OBSRVR,
///          .                         ABCORR )
///                    CALL TIMOUT ( FINISH,
///          .                       'YYYY-MON-DD HR:MN:SC.###',
///          .                       TIMSTR                          )
///
///                    WRITE (*, '(2X,A,F16.9)') 'Stop time  = '
///          .                                //  TIMSTR, PHASE
///
///                 END DO
///
///              END IF
///
///              WRITE(*,*) ' '
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Relation condition: =
///       Start time = 2006-DEC-02 13:31:34.414       0.575988450
///       Stop time  = 2006-DEC-02 13:31:34.414       0.575988450
///       Start time = 2006-DEC-07 14:07:55.470       0.575988450
///       Stop time  = 2006-DEC-07 14:07:55.470       0.575988450
///       Start time = 2006-DEC-31 23:59:59.997       0.575988450
///       Stop time  = 2006-DEC-31 23:59:59.997       0.575988450
///       Start time = 2007-JAN-06 08:16:25.512       0.575988450
///       Stop time  = 2007-JAN-06 08:16:25.512       0.575988450
///       Start time = 2007-JAN-30 11:41:32.557       0.575988450
///       Stop time  = 2007-JAN-30 11:41:32.557       0.575988450
///
///      Relation condition: <
///       Start time = 2006-DEC-02 13:31:34.414       0.575988450
///       Stop time  = 2006-DEC-07 14:07:55.470       0.575988450
///       Start time = 2006-DEC-31 23:59:59.997       0.575988450
///       Stop time  = 2007-JAN-06 08:16:25.512       0.575988450
///       Start time = 2007-JAN-30 11:41:32.557       0.575988450
///       Stop time  = 2007-JAN-31 00:00:00.000       0.468279091
///
///      Relation condition: >
///       Start time = 2006-DEC-01 00:00:00.000       0.940714974
///       Stop time  = 2006-DEC-02 13:31:34.414       0.575988450
///       Start time = 2006-DEC-07 14:07:55.470       0.575988450
///       Stop time  = 2006-DEC-31 23:59:59.997       0.575988450
///       Start time = 2007-JAN-06 08:16:25.512       0.575988450
///       Stop time  = 2007-JAN-30 11:41:32.557       0.575988450
///
///      Relation condition: LOCMIN
///       Start time = 2006-DEC-05 00:16:50.317       0.086121423
///       Stop time  = 2006-DEC-05 00:16:50.317       0.086121423
///       Start time = 2007-JAN-03 14:18:31.977       0.079899769
///       Stop time  = 2007-JAN-03 14:18:31.977       0.079899769
///
///      Relation condition: ABSMIN
///       Start time = 2007-JAN-03 14:18:31.977       0.079899769
///       Stop time  = 2007-JAN-03 14:18:31.977       0.079899769
///
///      Relation condition: LOCMAX
///       Start time = 2006-DEC-20 14:09:10.392       3.055062862
///       Stop time  = 2006-DEC-20 14:09:10.392       3.055062862
///       Start time = 2007-JAN-19 04:27:54.600       3.074603891
///       Stop time  = 2007-JAN-19 04:27:54.600       3.074603891
///
///      Relation condition: ABSMAX
///       Start time = 2007-JAN-19 04:27:54.600       3.074603891
///       Stop time  = 2007-JAN-19 04:27:54.600       3.074603891
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 31-AUG-2021 (JDR) (EDW)
///
///         Edited the header to comply with NAIF standards.
///
///         Fixed typos in the Header. Renamed example's meta-kernel.
///         Removed reference to private routine ZZGFPAQ from $Particulars
///         section.
///
/// -    SPICELIB Version 1.0.0, 27-MAR-2014 (EDW) (BVS)
/// ```
pub fn phaseq(
    ctx: &mut SpiceContext,
    et: f64,
    target: &str,
    illmn: &str,
    obsrvr: &str,
    abcorr: &str,
) -> crate::Result<f64> {
    let ret = PHASEQ(
        et,
        target.as_bytes(),
        illmn.as_bytes(),
        obsrvr.as_bytes(),
        abcorr.as_bytes(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure PHASEQ ( Phase angle quantity between bodies centers )
pub fn PHASEQ(
    ET: f64,
    TARGET: &[u8],
    ILLMN: &[u8],
    OBSRVR: &[u8],
    ABCORR: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut PHASEQ: f64 = 0.0;
    let mut TARG: i32 = 0;
    let mut ILLUM: i32 = 0;
    let mut OBS: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FND: bool = false;
    let mut XBCORR = [b' '; 32 as usize];

    //
    // SPICELIB functions.
    //

    //
    // Local parameters
    //

    //
    // Saved body name length.
    //

    //
    // Local Variables.
    //

    //
    // Saved name/ID item declarations.
    //

    //
    // Saved name/ID items.
    //

    //
    // Initial values.
    //

    PHASEQ = 0.0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(PHASEQ);
    }

    CHKIN(RNAME, ctx)?;

    //
    // Initialization.
    //
    if save.FIRST {
        //
        // Initialize counters.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR2.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR3.as_slice_mut(), ctx);

        save.FIRST = false;
    }

    //
    // Obtain integer codes for the target, illuminator, and observer.
    //
    ZZBODS2C(
        save.SVCTR1.as_slice_mut(),
        &mut save.SVTARG,
        &mut save.SVTGID,
        &mut save.SVFND1,
        TARGET,
        &mut TARG,
        &mut FND,
        ctx,
    )?;

    if !FND {
        SETMSG(b"The target, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(PHASEQ);
    }

    ZZBODS2C(
        save.SVCTR2.as_slice_mut(),
        &mut save.SVILMN,
        &mut save.SVICDE,
        &mut save.SVFND2,
        ILLMN,
        &mut ILLUM,
        &mut FND,
        ctx,
    )?;

    if !FND {
        SETMSG(b"The illuminator, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        ERRCH(b"#", ILLMN, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(PHASEQ);
    }

    ZZBODS2C(
        save.SVCTR3.as_slice_mut(),
        &mut save.SVOBSR,
        &mut save.SVOBSN,
        &mut save.SVFND3,
        OBSRVR,
        &mut OBS,
        &mut FND,
        ctx,
    )?;

    if !FND {
        SETMSG(b"The observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit. ", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(PHASEQ);
    }

    //
    // Squeeze all blanks out of the aberration correction
    // string; ensure the string is in upper case.
    //
    LJUCRS(0, ABCORR, &mut XBCORR, ctx);

    //
    // Check the aberration correction. If SPKEZR can't handle it,
    // neither can we.
    //
    ZZVALCOR(&XBCORR, ATTBLK.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(PHASEQ);
    }

    //
    // Restrict correction to reception cases. The attribute block ID
    // for transmit corrections is XMTIDX.
    //
    if ATTBLK[XMTIDX] {
        SETMSG(b"Invalid aberration correction \'#\'. Phase angle geometry calculations currently restricted to reception cases.", ctx);
        ERRCH(b"#", ABCORR, ctx);
        SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(PHASEQ);
    }

    //
    // Make sure the observer, illuminator, and target are distinct.
    //
    if (((TARG == OBS) || (TARG == ILLUM)) || (OBS == ILLUM)) {
        SETMSG(b"The observer, illuminator, and target must be distinct objects, but are not: OBSRVR = #, TARGET = #, are not: ILLMN= #.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        ERRCH(b"#", TARGET, ctx);
        ERRCH(b"#", ILLMN, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(PHASEQ);
    }

    //
    // Call the routine to calculate the phase angle
    //
    ZZGFPAQ(ET, TARG, ILLUM, OBS, &XBCORR, &mut PHASEQ, ctx)?;

    //
    // All done.
    //
    CHKOUT(RNAME, ctx)?;
    Ok(PHASEQ)
}
