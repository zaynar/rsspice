//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Stellar aberration, transmission case
///
/// Correct the position of a target for the stellar aberration
/// effect on radiation transmitted from a specified observer to
/// the target.
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
///  CORPOS     O   Corrected position of the object.
/// ```
///
/// # Detailed Input
///
/// ```text
///  POBJ     is the cartesian position vector of an object with
///           respect to the observer, possibly corrected for
///           light time. Units are km.
///
///  VOBS     is the cartesian velocity vector of the observer
///           with respect to the Solar System barycenter. Units
///           are km/s.
/// ```
///
/// # Detailed Output
///
/// ```text
///  CORPOS   is the  position of the object relative to the
///           observer, corrected for the stellar aberration
///           effect on radiation directed toward the target. This
///           correction is the inverse of the usual stellar
///           aberration correction: the corrected vector
///           indicates the direction in which radiation must be
///           emitted from the observer, as seen in an inertial
///           reference frame having velocity equal to that of the
///           observer, in order to reach the position indicated by
///           the input vector POBJ.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the velocity of the observer is greater than or equal to
///      the speed of light, an error is signaled by a routine in the
///      call tree of this routine. The outputs are undefined.
/// ```
///
/// # Particulars
///
/// ```text
///  In order to transmit radiation from an observer to a specified
///  target, the emission direction must be corrected for one way
///  light time and for the motion of the observer relative to the
///  solar system barycenter. The correction for the observer's
///  motion when transmitting to a target is the inverse of the
///  usual stellar aberration correction applied to the light-time
///  corrected position of the target as seen by the observer.
///
///  Below is the description of the stellar aberration correction
///  used in the SPICELIB routine STELAB (with the notation changed
///  slightly):
///
///     Let R be the vector from the observer to the object, and V be
///     the velocity of the observer with respect to the Solar System
///     barycenter. Let W be the angle between them. The aberration
///     angle PHI is given by
///
///        sin(PHI) = V * sin(W) / C
///
///     Let H be the vector given by the cross product
///
///        H = R x V
///
///     Rotate R by PHI radians about H to obtain the apparent position
///     of the object.
///
///  This routine applies the inverse correction, so here the rotation
///  about H is by -PHI radians.
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
///     Earth, corrected for one way light-time and stellar aberration
///     effect on radiation transmitted from the Earth to the Moon,
///     given the geometric state of the Earth relative to the Solar
///     System Barycenter, and the difference between the stellar
///     aberration corrected and uncorrected position vectors, taking
///     several steps.
///
///     First, compute the light-time corrected state of the Moon body
///     as seen by the Earth, using its geometric state. Then apply
///     the correction for stellar aberration to the light-time
///     corrected state of the target body, both for the transmission
///     case.
///
///     The code in this example could be replaced by a single call
///     to SPKPOS:
///
///         CALL SPKPOS ( 'MOON', ET, 'J2000', 'XLT+S', 'EARTH',
///        .               POS,   LT                            )
///
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: stlabx_ex1.tm
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
///           PROGRAM STLABX_EX1
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
///           CALL FURNSH ( 'stlabx_ex1.tm' )
///
///     C
///     C     Convert the time string to ephemeris time.
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
///     C     body IDTARG as seen by the observer. Normally we would
///     C     call SPKPOS to obtain this vector, but we already have
///     C     the state of the observer relative to the solar system
///     C     barycenter, so we can avoid looking up that state twice
///     C     by calling SPKAPO.
///     C
///           CALL SPKAPO ( IDTARG, ET, REFFRM, SOBS, 'XLT', POS, LT )
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
///           CALL STLABX ( POS, SOBS(4), PCORR )
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
///              201809.933536     -260878.049826     -147716.077987
///
///      Corrected position vector
///              201782.730972     -260894.375627     -147724.405897
///
///      Apparent difference
///                  27.202563          16.325802           8.327911
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
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.2, 13-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added example's
///         meta-kernel and problem statement. Created complete code
///         example from existing code fragments.
///
/// -    SPICELIB Version 1.0.1, 08-JAN-2008 (NJB)
///
///         The header example was updated to remove references
///         to SPKAPP.
///
/// -    SPICELIB Version 1.0.0, 02-JAN-2002 (IMU) (WLT) (NJB)
/// ```
pub fn stlabx(
    ctx: &mut SpiceContext,
    pobj: &[f64; 3],
    vobs: &[f64; 3],
    corpos: &mut [f64; 3],
) -> crate::Result<()> {
    STLABX(pobj, vobs, corpos, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure STLABX ( Stellar aberration, transmission case )
pub fn STLABX(
    POBJ: &[f64],
    VOBS: &[f64],
    CORPOS: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let POBJ = DummyArray::new(POBJ, 1..=3);
    let VOBS = DummyArray::new(VOBS, 1..=3);
    let mut CORPOS = DummyArrayMut::new(CORPOS, 1..=3);
    let mut NEGVEL = StackArray::<f64, 3>::new(1..=3);

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
        CHKIN(b"STLABX", ctx)?;
    }

    //
    // Obtain the negative of the observer's velocity.  This
    // velocity, combined with the target's position, will yield
    // the inverse of the usual stellar aberration correction,
    // which is exactly what we seek.
    //
    VMINUS(VOBS.as_slice(), NEGVEL.as_slice_mut());

    STELAB(
        POBJ.as_slice(),
        NEGVEL.as_slice(),
        CORPOS.as_slice_mut(),
        ctx,
    )?;

    CHKOUT(b"STLABX", ctx)?;
    Ok(())
}
