//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Stellar Aberration
///
/// Correct the apparent position of an object for stellar
/// aberration.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  POBJ       I   Position of an object with respect to the
///                 observer.
///  VOBS       I   Velocity of the observer with respect to the
///                 Solar System barycenter.
///  APPOBJ     O   Apparent position of the object with respect to
///                 the observer, corrected for stellar aberration.
/// ```
///
/// # Detailed Input
///
/// ```text
///  POBJ     is the position (x, y, z, km) of an object with
///           respect to the observer, possibly corrected for
///           light time.
///
///  VOBS     is the velocity (dx/dt, dy/dt, dz/dt, km/sec)
///           of the observer with respect to the Solar System
///           barycenter.
/// ```
///
/// # Detailed Output
///
/// ```text
///  APPOBJ   is the apparent position of the object relative
///           to the observer, corrected for stellar aberration.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the velocity of the observer is greater than or equal
///      to the speed of light, the error SPICE(VALUEOUTOFRANGE)
///      is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Let r be the vector from the observer to the object, and v be
///      -                                                    -
///  the velocity of the observer with respect to the Solar System
///  barycenter. Let w be the angle between them. The aberration
///  angle phi is given by
///
///       sin(phi) = v sin(w) / c
///
///  Let h be the vector given by the cross product
///      -
///
///        h = r X v
///        -   -   -
///
///  Rotate r by phi radians about h to obtain the apparent position
///         -                      -
///  of the object.
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
///  1) Compute the apparent position of the Moon relative to the
///     Earth, corrected for one light-time and stellar aberration,
///     given the geometric state of the Earth relative to the Solar
///     System Barycenter, and the difference between the stellar
///     aberration corrected and uncorrected position vectors, taking
///     several steps.
///
///     First, compute the light-time corrected state of the Moon body
///     as seen by the Earth, using its geometric state. Then apply
///     the correction for stellar aberration to the light-time
///     corrected state of the target body.
///
///     The code in this example could be replaced by a single call
///     to SPKPOS:
///
///         CALL SPKPOS ( 'MOON', ET, 'J2000', 'LT+S', 'EARTH',
///        .               POS,   LT                           )
///
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: stelab_ex1.tm
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
///           de418.bsp                     Planetary ephemeris
///           naif0009.tls                  Leapseconds
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de418.bsp',
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
///           PROGRAM STELAB_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(6)         REFFRM
///           CHARACTER*(12)        UTCSTR
///
///           DOUBLE PRECISION      APPDIF ( 3 )
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      PCORR  ( 3 )
///           DOUBLE PRECISION      POS    ( 3 )
///           DOUBLE PRECISION      SOBS   ( 6 )
///
///           INTEGER               IDOBS
///           INTEGER               IDTARG
///
///     C
///     C     Assign an observer, Earth, target, Moon, time of interest
///     C     and reference frame for returned vectors.
///     C
///           IDOBS  = 399
///           IDTARG = 301
///           UTCSTR = 'July 4 2004'
///           REFFRM = 'J2000'
///
///     C
///     C     Load the needed kernels.
///     C
///           CALL FURNSH ( 'stelab_ex1.tm' )
///
///     C
///     C     Convert the time string to ephemeris time, J2000.
///     C
///           CALL STR2ET ( UTCSTR, ET )
///
///     C
///     C     Get the state of the observer with respect to the solar
///     C     system barycenter.
///     C
///           CALL SPKSSB ( IDOBS, ET, REFFRM, SOBS )
///
///     C
///     C     Get the light-time corrected position POS of the target
///     C     body IDTARG as seen by the observer.
///     C
///           CALL SPKAPO ( IDTARG, ET, REFFRM, SOBS, 'LT', POS, LT )
///
///     C
///     C     Output the uncorrected vector.
///     C
///           WRITE(*,*) 'Uncorrected position vector'
///           WRITE(*,'(A,3F19.6)') '   ', POS(1), POS(2), POS(3)
///
///     C
///     C     Apply the correction for stellar aberration to the
///     C     light-time corrected position of the target body.
///     C
///           CALL STELAB ( POS, SOBS(4), PCORR )
///
///     C
///     C     Output the corrected position vector and the apparent
///     C     difference from the uncorrected vector.
///     C
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Corrected position vector'
///           WRITE(*,'(A,3F19.6)') '   ', PCORR(1), PCORR(2),
///          .                             PCORR(3)
///
///     C
///     C     Apparent difference.
///     C
///           CALL VSUB ( POS, PCORR, APPDIF )
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Apparent difference'
///           WRITE(*,'(A,3F19.6)') '   ', APPDIF(1), APPDIF(2),
///          .                            APPDIF(3)
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Uncorrected position vector
///              201738.725087     -260893.141602     -147722.589056
///
///      Corrected position vector
///              201765.929516     -260876.818077     -147714.262441
///
///      Apparent difference
///                 -27.204429         -16.323525          -8.326615
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  W. Owen, "The Treatment of Aberration in Optical Navigation",
///       JPL IOM #314.8-524, 8 February 1985.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 05-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added example's
///         meta-kernel and problem statement. Created complete code
///         example from existing code fragments.
///
/// -    SPICELIB Version 1.1.1, 08-JAN-2008 (NJB)
///
///         The header example was updated to remove references
///         to SPKAPP.
///
/// -    SPICELIB Version 1.1.0, 08-FEB-1999 (WLT)
///
///         The example was corrected so that SOBS(4) is passed
///         into STELAB instead of STARG(4).
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 08-AUG-1990 (HAN)
///
///         $Examples section of the header was updated to replace
///         calls to the GEF ephemeris readers by calls to the
///         new SPK ephemeris reader.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (WLT) (HAN)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 2.1.0, 9-MAR-1989 (HAN)
///
///         Declaration of the variable LIGHT was removed from the code.
///         The variable was declared but never used.
///
/// -    Beta Version 2.0.0, 28-DEC-1988 (HAN)
///
///         Error handling was added to check the velocity of the
///         observer. If the velocity of the observer is greater
///         than or equal to the speed of light, the error
///         SPICE(VALUEOUTOFRANGE) is signaled.
/// ```
pub fn stelab(
    ctx: &mut SpiceContext,
    pobj: &[f64; 3],
    vobs: &[f64; 3],
    appobj: &mut [f64; 3],
) -> crate::Result<()> {
    STELAB(pobj, vobs, appobj, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure STELAB     ( Stellar Aberration )
pub fn STELAB(
    POBJ: &[f64],
    VOBS: &[f64],
    APPOBJ: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let POBJ = DummyArray::new(POBJ, 1..=3);
    let VOBS = DummyArray::new(VOBS, 1..=3);
    let mut APPOBJ = DummyArrayMut::new(APPOBJ, 1..=3);
    let mut ONEBYC: f64 = 0.0;
    let mut U = StackArray::<f64, 3>::new(1..=3);
    let mut VBYC = StackArray::<f64, 3>::new(1..=3);
    let mut LENSQR: f64 = 0.0;
    let mut H = StackArray::<f64, 3>::new(1..=3);
    let mut SINPHI: f64 = 0.0;
    let mut PHI: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"STELAB", ctx)?;
    }

    //
    // We are not going to compute the aberrated vector in exactly the
    // way described in the particulars section.  We can combine some
    // steps and we take some precautions to prevent floating point
    // overflows.
    //
    //
    // Get a unit vector that points in the direction of the object
    // ( u_obj ).
    //
    VHAT(POBJ.as_slice(), U.as_slice_mut());

    //
    // Get the velocity vector scaled with respect to the speed of light
    // ( v/c ).
    //
    ONEBYC = (1.0 / CLIGHT());

    VSCL(ONEBYC, VOBS.as_slice(), VBYC.as_slice_mut());

    //
    // If the square of the length of the velocity vector is greater than
    // or equal to one, the speed of the observer is greater than or
    // equal to the speed of light. The observer speed is definitely out
    // of range. Signal an error and check out.
    //
    LENSQR = VDOT(VBYC.as_slice(), VBYC.as_slice());

    if (LENSQR >= 1.0) {
        SETMSG(
            b"Velocity components of observer were:  dx/dt = *, dy/dt = *, dz/dt = *.",
            ctx,
        );
        ERRDP(b"*", VOBS[1], ctx);
        ERRDP(b"*", VOBS[2], ctx);
        ERRDP(b"*", VOBS[3], ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"STELAB", ctx)?;
        return Ok(());
    }

    //
    // Compute u_obj x (v/c)
    //
    VCRSS(U.as_slice(), VBYC.as_slice(), H.as_slice_mut());

    //
    // If the magnitude of the vector H is zero, the observer is moving
    // along the line of sight to the object, and no correction is
    // required. Otherwise, rotate the position of the object by phi
    // radians about H to obtain the apparent position.
    //
    SINPHI = VNORM(H.as_slice());

    if (SINPHI != 0.0) {
        PHI = f64::asin(SINPHI);
        VROTV(POBJ.as_slice(), H.as_slice(), PHI, APPOBJ.as_slice_mut());
    } else {
        MOVED(POBJ.as_slice(), 3, APPOBJ.as_slice_mut());
    }

    CHKOUT(b"STELAB", ctx)?;
    Ok(())
}
