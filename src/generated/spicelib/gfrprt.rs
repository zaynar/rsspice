//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

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
const LBCELL: i32 = -5;
const FPRINT: i32 = 32;
const LPRINT: i32 = 126;

struct SaveVars {
    COPYB: Vec<u8>,
    COPYE: Vec<u8>,
    REMAIN: f64,
    T0: f64,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut COPYB = vec![b' '; MXBEGM as usize];
        let mut COPYE = vec![b' '; MXENDM as usize];
        let mut REMAIN: f64 = 0.0;
        let mut T0: f64 = 0.0;

        Self {
            COPYB,
            COPYE,
            REMAIN,
            T0,
        }
    }
}

/// GF, progress reporting package
///
/// The entry points contained under this routine provide users
/// information regarding the status of a GF search in progress.
///
/// # Required Reading
///
/// * [GF](crate::required_reading::gf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LBCELL     P   The SPICE cell lower bound.
///  MXBEGM     P   Maximum progress report message prefix length.
///  MXENDM     P   Maximum progress report message suffix length.
///  WINDOW     I   A window over which a job is to be performed.
///  BEGMSS     I   Beginning of the text portion of the output message
///  ENDMSS     I   End of the text portion of the output message
///  IVBEG      I   Current confinement window interval start time.
///  IVEND      I   Current confinement window interval stop time.
///  TIME       I   Input to the reporting routine.
/// ```
///
/// # Detailed Input
///
/// ```text
///  See the individual entry points.
/// ```
///
/// # Detailed Output
///
/// ```text
///  See the individual entry points.
/// ```
///
/// # Parameters
///
/// ```text
///  LBCELL   is the SPICE cell lower bound.
///
///  MXBEGM,
///  MXENDM   are, respectively, the maximum lengths of the progress
///           report message prefix and suffix.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  See the individual entry points.
/// ```
///
/// # Particulars
///
/// ```text
///  This umbrella routine contains default progress reporting entry
///  points that display a report via console I/O. These routines may
///  be used by SPICE-based applications as inputs to mid-level GF
///  search routines. These routines may be useful even when progress
///  reporting is not desired, since the mid-level search routines
///  provide some capabilities that aren't supported by the top-level
///  GF routines.
///
///  Developers wishing to use their own GF progress reporting
///  routines must design them with the same interfaces and should
///  assign them the same progress reporting roles as the entry points
///  of these routines.
///
///  The entry points contained in this routine are written to
///  make reporting of work (such as searching for a geometric event)
///  over a particular window easy. This is an important feature for
///  interactive programs that may "go away" from the user's control
///  for a considerable length of time. It allows the user to see that
///  something is still going on (although maybe not too quickly).
///
///  The three entry points contained under this module are:
///
///     GFREPI  used to set up the reporting mechanism. It lets GFRPRT
///             know that some task is about to begin that involves
///             interaction with some window of times. It is used
///             only to set up and store the constants associated with
///             the reporting of the job in progress.
///
///     GFREPU  is used to notify the reporter that work has
///             progressed to a given point with respect to the start
///             of the confinement window.
///
///     GFREPF  is used to "finish" the reporting of work (set the
///             completion value to 100%.
///
///  The progress reporting utilities are called by GF search routines
///  as follows:
///
///     1) Given a window over which some work is to be performed,
///        CALL GFREPI with the appropriate inputs, to let the routine
///        know the intervals over which some work is to be done.
///
///     2) Each time some "good" amount of work has been done, call
///        GFREPU so that the total amount of work done can be updated
///        and can be reported.
///
///     3) When work is complete call GFREPF to "clean up" the end of
///        the progress report.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) This example shows how to call a mid-level GF search API that
///     requires as input progress reporting routines.
///
///     If custom progress reporting routines are available, they
///     can replace GFREPI, GFREPU, and GFREPF in any GF API calls.
///
///     The code example below is the first example in the header of
///     GFOCCE.
///
///
///     Conduct a search using the default GF progress reporting
///     capability.
///
///     The program will use console I/O to display a simple
///     ASCII-based progress report.
///
///     The program will find occultations of the Sun by the Moon as
///     seen from the center of the Earth over the month December,
///     2001.
///
///     We use light time corrections to model apparent positions of
///     Sun and Moon. Stellar aberration corrections are not specified
///     because they don't affect occultation computations.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: gfrprt_ex1.tm
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
///           pck00008.tpc                  Planet orientation and
///                                         radii
///           naif0009.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'pck00008.tpc',
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
///           PROGRAM GFRPRT_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               WNCARD
///
///     C
///     C     SPICELIB default functions for
///     C
///     C        - Interrupt handling (no-op function):   GFBAIL
///     C        - Search refinement:                     GFREFN
///     C        - Progress report termination:           GFREPF
///     C        - Progress report initialization:        GFREPI
///     C        - Progress report update:                GFREPU
///     C        - Search step size "get" function:       GFSTEP
///     C
///           LOGICAL               GFBAIL
///           EXTERNAL              GFBAIL
///
///           EXTERNAL              GFREFN
///           EXTERNAL              GFREPI
///           EXTERNAL              GFREPU
///           EXTERNAL              GFREPF
///           EXTERNAL              GFSTEP
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         TIMFMT
///           PARAMETER           ( TIMFMT =
///          .   'YYYY MON DD HR:MN:SC.###### ::TDB (TDB)' )
///
///           DOUBLE PRECISION      CNVTOL
///           PARAMETER           ( CNVTOL = 1.D-6 )
///
///           INTEGER               MAXWIN
///           PARAMETER           ( MAXWIN = 2 * 100 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 40 )
///
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(TIMLEN)    WIN0
///           CHARACTER*(TIMLEN)    WIN1
///           CHARACTER*(TIMLEN)    BEGSTR
///           CHARACTER*(TIMLEN)    ENDSTR
///
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2 )
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      ET1
///           DOUBLE PRECISION      LEFT
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///           DOUBLE PRECISION      RIGHT
///
///           INTEGER               I
///
///           LOGICAL               BAIL
///           LOGICAL               RPT
///
///     C
///     C     Saved variables
///     C
///     C     The confinement and result windows CNFINE and RESULT are
///     C     saved because this practice helps to prevent stack
///     C     overflow.
///     C
///           SAVE                  CNFINE
///           SAVE                  RESULT
///
///     C
///     C     Load kernels.
///     C
///           CALL FURNSH ( 'gfrprt_ex1.tm' )
///
///     C
///     C     Initialize the confinement and result windows.
///     C
///           CALL SSIZED ( 2,      CNFINE )
///           CALL SSIZED ( MAXWIN, RESULT )
///
///     C
///     C     Obtain the TDB time bounds of the confinement
///     C     window, which is a single interval in this case.
///     C
///           WIN0 = '2001 DEC 01 00:00:00 TDB'
///           WIN1 = '2002 JAN 01 00:00:00 TDB'
///
///           CALL STR2ET ( WIN0, ET0 )
///           CALL STR2ET ( WIN1, ET1 )
///
///     C
///     C     Insert the time bounds into the confinement
///     C     window.
///     C
///           CALL WNINSD ( ET0, ET1, CNFINE )
///
///     C
///     C     Select a 20 second step. We'll ignore any occultations
///     C     lasting less than 20 seconds.
///     C
///           CALL GFSSTP ( 20.D0 )
///
///     C
///     C     Turn on progress reporting; turn off interrupt
///     C     handling.
///     C
///           RPT  = .TRUE.
///           BAIL = .FALSE.
///
///     C
///     C     Perform the search.
///     C
///           CALL GFOCCE ( 'ANY',
///          .              'MOON',   'ellipsoid',  'IAU_MOON',
///          .              'SUN',    'ellipsoid',  'IAU_SUN',
///          .              'LT',     'EARTH',      CNVTOL,
///          .              GFSTEP,   GFREFN,       RPT,
///          .              GFREPI,   GFREPU,       GFREPF,
///          .              BAIL,     GFBAIL,       CNFINE,  RESULT )
///
///
///           IF ( WNCARD(RESULT) .EQ. 0 ) THEN
///
///              WRITE (*,*) 'No occultation was found.'
///
///           ELSE
///
///              DO I = 1, WNCARD(RESULT)
///
///     C
///     C           Fetch and display each occultation interval.
///     C
///                 CALL WNFETD ( RESULT, I, LEFT, RIGHT )
///
///                 CALL TIMOUT ( LEFT,  TIMFMT, BEGSTR )
///                 CALL TIMOUT ( RIGHT, TIMFMT, ENDSTR )
///
///                 WRITE (*,*) 'Interval ', I
///                 WRITE (*,*) '   Start time: '//BEGSTR
///                 WRITE (*,*) '   Stop time:  '//ENDSTR
///
///              END DO
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Occultation/transit search 100.00% done.
///
///      Interval            1
///         Start time: 2001 DEC 14 20:10:14.195952  (TDB)
///         Stop time:  2001 DEC 14 21:35:50.317994  (TDB)
///
///
///     Note that the progress report has the format shown below:
///
///        Occultation/transit search   6.02% done.
///
///     The completion percentage was updated approximately once per
///     second.
///
///
///  2) The following piece of code provides a more concrete example
///     of how these routines might be used. It is part of code that
///     performs a search for the time of an occultation of one body
///     by another. It is intended only for illustration and is not
///     recommended for use in code that has to do real work.
///
///     C
///     C     Prepare the progress reporter if appropriate.
///     C
///           IF ( RPT ) THEN
///              CALL UDREPI ( CNFINE, 'Occultation/transit search ',
///          .                         'done.'                      )
///           END IF
///
///     C
///     C     Cycle over the intervals in the confining window.
///     C
///           COUNT = WNCARD(CNFINE)
///
///           DO I = 1, COUNT
///     C
///     C        Retrieve the bounds for the Ith interval of the
///     C        confinement window. Search this interval for
///     C        occultation events. Union the result with the
///     C        contents of the RESULT window.
///     C
///              CALL WNFETD ( CNFINE, I, START, FINISH  )
///
///              CALL ZZGFSOLV ( ZZGFOCST, UDSTEP, UDREFN, BAIL,
///          .                   UDBAIL,   CSTEP,  STEP,   START,
///          .                   FINISH,   TOL,    RPT,    UDREPU,
///          .                   RESULT                          )
///
///
///              IF (  FAILED()  ) THEN
///                 CALL CHKOUT ( 'GFOCCE'  )
///                 RETURN
///              END IF
///
///              IF ( BAIL ) THEN
///     C
///     C           Interrupt handling is enabled.
///     C
///                 IF ( UDBAIL () ) THEN
///     C
///     C              An interrupt has been issued. Return now
///     C              regardless of whether the search has been
///     C              completed.
///     C
///                    CALL CHKOUT ( 'GFOCCE' )
///                    RETURN
///
///                 END IF
///
///              END IF
///
///           END DO
///
///     C
///     C     End the progress report.
///     C
///           IF ( RPT ) THEN
///              CALL UDREPF
///           END IF
///
///
///  3) For more concrete examples of how these routines are used in
///     SPICELIB, please refer to the actual code of any of the GF API
///     calls.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  L.S. Elson         (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.2, 27-AUG-2021 (JDR)
///
///         Edited the header of all entry points and GFRPRT to comply with
///         NAIF standard.
///
///         Added complete example code to GFRPRT.
///
/// -    SPICELIB Version 1.0.1, 10-FEB-2014 (BVS)
///
///         Added declarations of IVBEG and IVEND to the $Declarations
///         section of the GFREPU header.
///
///         Corrected declaration of WINDOW in the $Declarations
///         section and added descriptions of LBCELL to the GFREPI
///         header.
///
/// -    SPICELIB Version 1.0.0, 06-MAR-2009 (NJB) (LSE) (WLT) (IMU)
/// ```
pub fn gfrprt(
    ctx: &mut SpiceContext,
    window: &[f64],
    begmss: &str,
    endmss: &str,
    ivbeg: f64,
    ivend: f64,
    time: f64,
) -> crate::Result<()> {
    GFRPRT(
        window,
        begmss.as_bytes(),
        endmss.as_bytes(),
        ivbeg,
        ivend,
        time,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GFRPRT ( GF, progress reporting package )
pub fn GFRPRT(
    WINDOW: &[f64],
    BEGMSS: &[u8],
    ENDMSS: &[u8],
    IVBEG: f64,
    IVEND: f64,
    TIME: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    CHKIN(b"GFRPRT", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"GFRPRT", ctx)?;
    Ok(())
}

/// GF, progress report initialization
///
/// Initialize a search progress report.
///
/// # Required Reading
///
/// * [GF](crate::required_reading::gf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LBCELL     P   The SPICE cell lower bound.
///  MXBEGM     P   Maximum progress report message prefix length.
///  MXENDM     P   Maximum progress report message suffix length.
///  WINDOW     I   A window over which a job is to be performed.
///  BEGMSS     I   Beginning of the text portion of output message.
///  ENDMSS     I   End of the text portion of output message.
/// ```
///
/// # Detailed Input
///
/// ```text
///  WINDOW   is the name of a constraint window. This is the window
///           associated with some root finding activity. It is
///           used to determine how much total time is being searched
///           in order to find the events of interest.
///
///  BEGMSS   is the beginning of the progress report message written
///           to standard output by the GF subsystem. This output
///           message has the form
///
///              BEGMSS(1:LASTNB(BEGMSS)) // ' xxx.xx% ' // ENDMSS
///
///           The total length of BEGMSS must be less than MXBEGM
///           characters. All characters of BEGMSS must be printable.
///
///           For example, the progress report message created by the
///           SPICELIB routine GFOCCE at the completion of a search is
///
///              Occultation/transit search 100.00% done.
///
///           In this message, BEGMSS is
///
///              'Occultation/transit search'
///
///  ENDMSS   is the last portion of the output message written to
///           standard output by the GF subsystem.
///
///           The total length of ENDMSS must be less than MXENDM
///           characters. All characters of ENDMSS must be printable.
///
///           In the progress report message created by GFOCCE at the
///           completion of a search, ENDMSS is
///
///              'done.'
/// ```
///
/// # Parameters
///
/// ```text
///  LBCELL   is the SPICE cell lower bound.
///
///  MXBEGM,
///  MXENDM   are, respectively, the maximum lengths of the progress
///           report message prefix and suffix. See the INCLUDE file
///           zzgf.inc for details.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If BEGMSS has length greater than MXBEGM characters, or if
///      ENDMSS has length greater than MXENDM characters, the error
///      SPICE(MESSAGETOOLONG) is signaled.
///
///  2)  If either BEGMSS or ENDMSS contains non-printing characters,
///      the error SPICE(NOTPRINTABLECHARS) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This entry point initializes the GF progress reporting system. It
///  is called by the GF root finding utilities once at the start of
///  each search pass. See the $Particulars section of the main
///  subroutine header for further details of its function.
/// ```
///
/// # Examples
///
/// ```text
///  See $Examples in GFRPRT.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  L.S. Elson         (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.2, 27-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
///         Extended description of BEGMSS and ENDMSS arguments.
///
/// -    SPICELIB Version 1.0.1, 10-FEB-2014 (BVS)
///
///         Corrected declaration of WINDOW in the $Declarations
///         section. Added description of LBCELL to the $Declarations,
///         $Brief_I/O, and $Parameters sections.
///
/// -    SPICELIB Version 1.0.0, 21-FEB-2009 (NJB) (LSE) (WLT) (IMU)
/// ```
pub fn gfrepi(
    ctx: &mut SpiceContext,
    window: &[f64],
    begmss: &str,
    endmss: &str,
) -> crate::Result<()> {
    GFREPI(
        window,
        begmss.as_bytes(),
        endmss.as_bytes(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GFREPI ( GF, progress report initialization )
pub fn GFREPI(
    WINDOW: &[f64],
    BEGMSS: &[u8],
    ENDMSS: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let WINDOW = DummyArray::new(WINDOW, LBCELL..);
    let mut AVE: f64 = 0.0;
    let mut MEASUR: f64 = 0.0;
    let mut STDDEV: f64 = 0.0;
    let mut CHRCOD: i32 = 0;
    let mut LONG: i32 = 0;
    let mut SHORT: i32 = 0;

    //
    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"GFREPI", ctx)?;

    //
    // Check to see if either the message prefix or suffix
    // is too long.
    //
    if (LASTNB(BEGMSS) > MXBEGM) {
        SETMSG(
            b"Progress report prefix message contains # characters; limit is #.",
            ctx,
        );
        ERRINT(b"#", LASTNB(BEGMSS), ctx);
        ERRINT(b"#", MXBEGM, ctx);
        SIGERR(b"SPICE(MESSAGETOOLONG)", ctx)?;
        CHKOUT(b"GFREPI", ctx)?;
        return Ok(());
    }

    if (LASTNB(ENDMSS) > MXENDM) {
        SETMSG(
            b"Progress report suffix message contains # characters; limit is #.",
            ctx,
        );
        ERRINT(b"#", LASTNB(ENDMSS), ctx);
        ERRINT(b"#", MXENDM, ctx);
        SIGERR(b"SPICE(MESSAGETOOLONG)", ctx)?;
        CHKOUT(b"GFREPI", ctx)?;
        return Ok(());
    }

    //
    // Now check that all the characters in the message prefix and
    // suffix can be printed.
    //
    for I in 1..=LASTNB(BEGMSS) {
        CHRCOD = intrinsics::ICHAR(fstr::substr(BEGMSS, I..=I));

        if ((CHRCOD < FPRINT) || (CHRCOD > LPRINT)) {
            SETMSG(b"The progress report message prefix contains a nonprintable character; ASCII code is #.", ctx);
            ERRINT(b"#", CHRCOD, ctx);
            SIGERR(b"SPICE(NONPRINTABLECHARS)", ctx)?;
            CHKOUT(b"GFREPI", ctx)?;
            return Ok(());
        }
    }

    for I in 1..=LASTNB(ENDMSS) {
        CHRCOD = intrinsics::ICHAR(fstr::substr(ENDMSS, I..=I));

        if ((CHRCOD < FPRINT) || (CHRCOD > LPRINT)) {
            SETMSG(b"The progress report message suffix contains a nonprintable character; ASCII code is #.", ctx);
            ERRINT(b"#", CHRCOD, ctx);
            SIGERR(b"SPICE(NONPRINTABLECHARS)", ctx)?;
            CHKOUT(b"GFREPI", ctx)?;
            return Ok(());
        }
    }

    fstr::assign(&mut save.COPYB, BEGMSS);
    fstr::assign(&mut save.COPYE, ENDMSS);

    //
    // Find the length of the window. Use that to initialize the work
    // reporter.
    //
    WNSUMD(
        WINDOW.as_slice(),
        &mut MEASUR,
        &mut AVE,
        &mut STDDEV,
        &mut SHORT,
        &mut LONG,
        ctx,
    )?;
    ZZGFTSWK(MEASUR, 1.0, 4, BEGMSS, ENDMSS, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"GFREPI", ctx)?;
        return Ok(());
    }

    //
    // Initialize the time to the start of the confinement window.
    // The remaining amount of work in the current interval is
    // the measure of the interval.
    //
    if (CARDD(WINDOW.as_slice(), ctx)? >= 2) {
        save.T0 = WINDOW[1];
        save.REMAIN = (WINDOW[2] - save.T0);
    } else {
        save.REMAIN = 0.0;
    }

    CHKOUT(b"GFREPI", ctx)?;
    Ok(())
}

/// GF, progress report update
///
/// Tell the progress reporting system how far a search has
/// progressed.
///
/// # Required Reading
///
/// * [GF](crate::required_reading::gf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IVBEG      I   Start time of work interval.
///  IVEND      I   End time of work interval.
///  TIME       I   Current time being examined in the search process
/// ```
///
/// # Detailed Input
///
/// ```text
///  IVBEG,
///  IVEND    are the bounds of an interval that is contained in some
///           interval belonging to the confinement window. The
///           confinement window is associated with some root finding
///           activity. It is used to determine how much total time is
///           being searched in order to find the events of interest.
///
///           In order for a meaningful progress report to be
///           displayed, IVBEG and IVEND must satisfy the following
///           constraints:
///
///              - IVBEG must be less than or equal to IVEND.
///
///              - The interval [ IVBEG, IVEND ] must be contained in
///                some interval of the confinement window. It can be
///                a proper subset of the containing interval; that
///                is, it can be smaller than the interval of the
///                confinement window that contains it.
///
///              - Over a search pass, the sum of the differences
///
///                   IVEND - IVBEG
///
///                for all calls to this routine made during the pass
///                must equal the measure of the confinement window.
///
///
///  TIME     is the current time reached in the search for an event.
///           TIME must lie in the interval
///
///              IVBEG : IVEND
///
///           inclusive. The input values of TIME for a given interval
///           need not form an increasing sequence.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. This routine does perform console I/O when progress
///  reporting is enabled.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If IVBEG and IVEND are in decreasing order, the error
///      SPICE(BADENDPOINTS) is signaled.
///
///  2)  If TIME is not in the closed interval [IVBEG, IVEND], the
///      error SPICE(VALUEOUTOFRANGE) is signaled.
///
///  3)  If an I/O error results from writing to standard output, the
///      error is signaled by a routine in the call tree of this
///      routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This entry point is used to indicate the current progress of a
///  search. Using information recorded through the initialization
///  entry point of this routine, the progress reporting system
///  determines how much work has been completed and whether or not to
///  report it on the users screen.
/// ```
///
/// # Examples
///
/// ```text
///  See $Examples in GFRPRT.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine has no way of enforcing that the input values of
///      IVBEG and IVEND are compatible with the input window passed to
///      GFREPI. Callers of this routine are responsible for ensuring
///      that this requirement is obeyed.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  L.S. Elson         (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.2, 27-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-FEB-2014 (BVS)
///
///         Added declarations of IVBEG and IVEND to the $Declarations
///         section.
///
/// -    SPICELIB Version 1.0.0, 21-FEB-2009 (NJB) (LSE) (WLT) (IMU)
/// ```
pub fn gfrepu(ctx: &mut SpiceContext, ivbeg: f64, ivend: f64, time: f64) -> crate::Result<()> {
    GFREPU(ivbeg, ivend, time, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GFREPU ( GF, progress report update )
pub fn GFREPU(IVBEG: f64, IVEND: f64, TIME: f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut INCR: f64 = 0.0;

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"GFREPU", ctx)?;

    //
    // Do a few error checks before getting started.
    //
    // We expect the endpoints of the current window to be in order.
    //
    if (IVEND < IVBEG) {
        SETMSG(
            b"Interval endpoints are #:#; endpoints must be in increasing order.",
            ctx,
        );
        ERRDP(b"#", IVBEG, ctx);
        ERRDP(b"#", IVEND, ctx);
        SIGERR(b"SPICE(BADENDPOINTS)", ctx)?;
        CHKOUT(b"GFREPU", ctx)?;
        return Ok(());
    }

    //
    // We expect TIME to be in the current interval of the confinement
    // window.
    //
    if ((TIME < IVBEG) || (TIME > IVEND)) {
        SETMSG(b"TIME should be in interval #:# but is #.", ctx);
        ERRDP(b"#", TIME, ctx);
        ERRDP(b"#", IVBEG, ctx);
        ERRDP(b"#", IVEND, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"GFREPU", ctx)?;
        return Ok(());
    }

    //
    // The amount of work done is the difference between the current
    // time and the previous time T0, presuming both times are in
    // the current interval.  Note this work amount may be negative.
    //
    if ((save.T0 >= IVBEG) && (save.T0 <= IVEND)) {
        INCR = (TIME - save.T0);
    } else {
        //
        // T0 is in the previous interval.  The amount of work
        // done to complete processing of that interval is REMAIN.
        // The amount of work done in the current interval is
        // the difference of TIME and the left endpoint of the
        // interval.
        //
        INCR = ((save.REMAIN + TIME) - IVBEG);
    }

    //
    // The remaining work is the distance from TIME to the right
    // endpoint of the current interval.
    //
    save.REMAIN = (IVEND - TIME);

    //
    // Record the current time as T0.
    //
    save.T0 = TIME;

    //
    // Report the work increment.
    //
    ZZGFWKIN(INCR, ctx)?;

    CHKOUT(b"GFREPU", ctx)?;
    Ok(())
}

/// GF, progress report finalization
///
/// Finish a progress report.
///
/// # Required Reading
///
/// * [GF](crate::required_reading::gf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  None.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. This routine does perform console I/O when progress
///  reporting is enabled.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an I/O error results from writing to standard output, the
///      error is signaled by a routine in the call tree of this
///      routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This entry point "finishes" a progress report, i.e. updates the
///  report to indicate the underlying task is 100% complete.
/// ```
///
/// # Examples
///
/// ```text
///  See $Examples in GFRPRT.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  L.S. Elson         (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 07-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 21-FEB-2009 (NJB) (LSE) (WLT) (IMU)
/// ```
pub fn gfrepf(ctx: &mut SpiceContext) -> crate::Result<()> {
    GFREPF(ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GFREPF ( GF, progress report finalization )
pub fn GFREPF(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut BEGIN = [b' '; MXBEGM as usize];
    let mut END = [b' '; MXENDM as usize];
    let mut FREQ: f64 = 0.0;
    let mut INCR: f64 = 0.0;
    let mut TOTAL: f64 = 0.0;
    let mut STDOUT: i32 = 0;
    let mut TCHECK: i32 = 0;
    let mut UNIT: i32 = 0;

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"GFREPF", ctx)?;

    ZZGFWKAD(0.0, 1, &save.COPYB, &save.COPYE, ctx);
    ZZGFWKIN(0.0, ctx)?;

    //
    // Determine whether progress report output is currently
    // being sent to standard output. Fetch the output unit.
    //
    ZZGFWKMO(
        &mut UNIT,
        &mut TOTAL,
        &mut FREQ,
        &mut TCHECK,
        &mut BEGIN,
        &mut END,
        &mut INCR,
        ctx,
    );

    STDIO(b"STDOUT", &mut STDOUT, ctx)?;

    if (UNIT != STDOUT) {
        //
        // We're not currently writing to standard output, so we're
        // done.
        //
        CHKOUT(b"GFREPF", ctx)?;
        return Ok(());
    }

    //
    // Emit a final blank line by moving the cursor down two
    // spaces.
    //
    // The set of actual arguments passed here is rather funky
    // and deserves some explanation:
    //
    //    The first argument, calling for a leading blank line, moves
    //    the cursor down so that the next blank line written won't
    //    overwrite the final status message. That blank line is
    //    followed with a cursor repositioning command that moves the
    //    cursor to the beginning of the line that was just written. The
    //    last argument, calling for another blank line, moves the
    //    cursor down again. The total cursor movement is down 2 lines.
    //    This results in one skipped line.
    //
    // We could accomplish the same results more simply if were
    // were to use I/O statements in this routine; however, in the
    // interest of minimizing the number of places where I/O is
    // performed, we rely on ZZGFDSPS to do that job.
    //
    ZZGFDSPS(1, b" ", b"A", 1, ctx)?;

    CHKOUT(b"GFREPF", ctx)?;
    Ok(())
}
