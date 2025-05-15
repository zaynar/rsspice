//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ZZGET: i32 = -1;
const ZZPUT: i32 = -2;
const ZZRESET: i32 = -3;
const ZZNOP: i32 = 3;
const GEN: i32 = 1;
const GF_REF: i32 = 2;
const GF_TOL: i32 = 3;
const GF_DT: i32 = 4;
const NID: i32 = 4;

/// GF, set a tolerance value for GF
///
/// Override the default GF convergence value used in the high
/// level GF routines.
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
///  ZZPUT      P   ZZHOLDD stores a DP value.
///  GF_TOL     P   ZZHOLDD acts on the GF subsystem tolerance.
///  VALUE      I   Double precision value returned or to store.
/// ```
///
/// # Detailed Input
///
/// ```text
///  VALUE    is the scalar double precision value to use as the GF
///           subsystem convergence tolerance. This value will override
///           the default tolerance, CNVTOL, defined in gf.inc. Units
///           are TDB seconds.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If VALUE is not strictly greater-than-zero, the error
///      SPICE(INVALIDTOLERANCE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The high level GF routines (see gf.req for a listing) use a
///  default value for the convergence tolerance, CNVTOL, defined in
///  gf.inc. It may occur that a GF search run needs a different
///  convergence tolerance. GFSTOL programmatically changes the
///  tolerance used by those routines.
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
///  1) Perform a search for occultation events of the sun by earth as
///     observed from the Moon center. Search during the interval from
///     14 A.D. SEP 1 to 14 A.D. SEP 30 (Julian).
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: gfstol_ex1.tm
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
///           pck00009.tpc                  Planet orientation and
///                                         radii
///           naif0009.tls                  Leapseconds
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'pck00009.tpc',
///                               'naif0009.tls'  )
///
///        \begintext
///
///        End of meta-kernel.
///
///
///     Use the SPK kernel below to load the required ephemeris,
///     covering year 14 AD.
///
///        de408.bsp
///
///
///     Example code begins here.
///
///
///           PROGRAM GFSTOL_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         TIMFMT
///           PARAMETER           ( TIMFMT =
///          .             'YYYY ERA MON DD HR:MN:SC.#### ::JCAL' )
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
///           DOUBLE PRECISION      CNFINE ( LBCELL : 2      )
///           DOUBLE PRECISION      RESULT ( LBCELL : MAXWIN )
///           DOUBLE PRECISION      ET0
///           DOUBLE PRECISION      ET1
///           DOUBLE PRECISION      LEFT
///           DOUBLE PRECISION      RIGHT
///           DOUBLE PRECISION      STEP
///
///           INTEGER               I
///
///           LOGICAL               OK
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
///           CALL FURNSH ( 'gfstol_ex1.tm' )
///
///     C
///     C     Use an SPK covering year 14 AD.
///     C
///           CALL FURNSH ( 'de408.bsp' )
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
///           WIN0 = '14 A.D. SEP 1  00:00:00'
///           WIN1 = '14 A.D. SEP 30 00:00:00'
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
///     C     Select a 3-minute step. We'll ignore any occultations
///     C     lasting less than 3 minutes.
///     C
///           STEP = 180.D0
///
///     C
///     C     Perform the search. ET0 and ET1 have values ~-6*10^10,
///     C     CNVTOL has value 10^-6, so double precision addition or
///     C     subtraction of ET0 and ET1 with CNVTOL returns a result
///     C     indistinguishable from ET0 and ET1.
///     C
///     C     Reduce the GF convergence tolerance by an order of
///     C     magnitude to resolve this condition.
///     C
///           CALL GFSTOL ( 1D-5 )
///
///           CALL GFOCLT ( 'ANY',
///          .              'EARTH', 'ellipsoid', 'IAU_EARTH',
///          .              'SUN',   'ellipsoid', 'IAU_SUN',
///          .              'LT',    'MOON',       STEP,
///          .               CNFINE,  RESULT  )
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
///      Interval            1
///         Start time:   14 A.D. SEP 27 05:02:02.8250
///         Stop time:    14 A.D. SEP 27 09:33:31.6995
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 06-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
///         Added SAVE statements for CNFINE and RESULT variables in code
///         example.
///
/// -    SPICELIB Version 1.0.0, 18-APR-2014 (EDW)
/// ```
pub fn gfstol(ctx: &mut SpiceContext, value: f64) -> crate::Result<()> {
    GFSTOL(value, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GFSTOL ( GF, set a tolerance value for GF )
pub fn GFSTOL(VALUE: f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut OK: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // This routine wraps a call to ZZHOLDD with the appropriate ID and
    // OP value.
    //

    //
    // Check tolerance value.
    //

    if (VALUE <= 0.0) {
        CHKIN(b"GFSTOL", ctx)?;
        SETMSG(
            b"Convergence tolerance must be greater-than zero. Input VALUE = #.",
            ctx,
        );
        ERRDP(b"#", VALUE, ctx);
        SIGERR(b"SPICE(INVALIDTOLERANCE)", ctx)?;
        CHKOUT(b"GFSTOL", ctx)?;
        return Ok(());
    } else {
        ZZHOLDD_ZZPUT(ZZPUT, GF_TOL, &mut OK, VALUE, ctx)?;
    }

    Ok(())
}
