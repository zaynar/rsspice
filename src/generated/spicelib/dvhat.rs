//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const POS: i32 = 1;
const VEL: i32 = 4;

/// Derivative and unit vector "V-hat" of a state
///
/// Find the unit vector corresponding to a state vector and the
/// derivative of the unit vector.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  S1         I   State to be normalized.
///  SOUT       O   Unit vector S1 / |S1|, and its time derivative.
/// ```
///
/// # Detailed Input
///
/// ```text
///  S1       is any double precision state. If the position
///           component of the state is the zero vector, this
///           routine will detect it and will not attempt to divide
///           by zero.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SOUT     is a state containing the unit vector pointing in
///           the direction of position component of S1 and the
///           derivative of the unit vector with respect to time.
///
///           SOUT may overwrite S1.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If S1 represents the zero vector, then the position
///      component of SOUT will also be the zero vector. The
///      velocity component will be the velocity component
///      of S1.
/// ```
///
/// # Particulars
///
/// ```text
///  Let S1 be a state vector with position and velocity components P
///  and V respectively. From these components one can compute the
///  unit vector parallel to P, call it U and the derivative of U
///  with respect to time, DU. This pair (U,DU) is the state returned
///  by this routine in SOUT.
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
///  1) Suppose that STATE gives the apparent state of a body with
///     respect to an observer. This routine can be used to compute
///     the instantaneous angular rate of the object across the sky as
///     seen from the observers vantage.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: dvhat_ex1.tm
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
///           PROGRAM DVHAT_EX1
///           IMPLICIT NONE
///
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      OMEGA
///           DOUBLE PRECISION      STATE  (6)
///           DOUBLE PRECISION      USTATE (6)
///
///           DOUBLE PRECISION      VNORM
///
///           CHARACTER*(32)        EPOCH
///           CHARACTER*(32)        TARGET
///           CHARACTER*(32)        FRAME
///           CHARACTER*(32)        ABCORR
///           CHARACTER*(32)        OBSRVR
///
///     C
///     C     Load SPK, PCK, and LSK kernels, use a meta kernel for
///     C     convenience.
///     C
///           CALL FURNSH ( 'dvhat_ex1.tm' )
///
///     C
///     C     Define an arbitrary epoch, convert the epoch to
///     C     ephemeris time.
///     C
///           EPOCH = 'Jan 1 2009'
///           CALL STR2ET ( EPOCH, ET )
///
///     C
///     C     Calculate the state of the moon with respect to the
///     C     earth-moon barycenter in J2000, corrected for light time
///     C     and stellar aberration at ET.
///     C
///           TARGET   = 'MOON'
///           FRAME    = 'J2000'
///           ABCORR   = 'LT+S'
///           OBSRVR   = 'EARTH BARYCENTER'
///
///           CALL SPKEZR ( TARGET, ET,    FRAME, ABCORR,
///          .              OBSRVR, STATE, LT            )
///
///     C
///     C     Calculate the unit vector of STATE and the derivative
///     C     of the unit vector.
///     C
///           CALL DVHAT ( STATE, USTATE )
///
///     C
///     C     Calculate the instantaneous angular velocity from the
///     C     magnitude of the derivative of the unit vector.
///     C
///     C          v = r x omega
///     C
///     C          ||omega|| = ||v||  for  r . v = 0
///     C                      -----
///     C                      ||r||
///     C
///     C          ||omega|| = ||v||  for  ||r|| = 1
///     C
///           OMEGA = VNORM( USTATE(4) )
///
///           WRITE(*,'(A,E18.12)') 'Instantaneous angular velocity'
///          . //                   ' (rad/s): ', OMEGA
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Instantaneous angular velocity (rad/s): 0.248106659269E-05
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header comments to comply with NAIF standard. Added
///         meta-kernel to the example.
///
/// -    SPICELIB Version 1.1.1, 06-MAY-2010 (EDW)
///
///         Expanded the code example into a complete program.
///
///         Reordered header sections to proper NAIF convention.
///         Removed Revision section, it listed a duplication of a
///         $Version section entry.
///
/// -    SPICELIB Version 1.1.0, 02-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VPERP and VSCL calls.
///
/// -    SPICELIB Version 1.0.0, 15-JUN-1995 (WLT)
/// ```
pub fn dvhat(s1: &[f64; 6], sout: &mut [f64; 6]) {
    DVHAT(s1, sout);
}

//$Procedure DVHAT ( Derivative and unit vector "V-hat" of a state)
pub fn DVHAT(S1: &[f64], SOUT: &mut [f64]) {
    let S1 = DummyArray::new(S1, 1..=6);
    let mut SOUT = DummyArrayMut::new(SOUT, 1..=6);
    let mut LENGTH: f64 = 0.0;

    //
    // Local Constants.
    //

    //
    // Local variables.
    //

    //
    // Get the position portion of the output state and the length of
    // the input position.
    //
    UNORM(S1.subarray(POS), SOUT.subarray_mut(POS), &mut LENGTH);

    if (LENGTH == 0.0) {
        //
        // If the length of the input position is zero, just copy
        // the input velocity to the output velocity.
        //
        VEQU(S1.subarray(VEL), SOUT.subarray_mut(VEL));
    } else {
        //
        // Otherwise the derivative of the unit vector is just the
        // component of the input velocity perpendicular to the input
        // position, scaled by the reciprocal of the length of the
        // input position.
        //
        VPERP(
            S1.subarray(VEL),
            &SOUT.subarray(POS).to_vec(),
            SOUT.subarray_mut(VEL),
        );
        VSCLIP((1.0 / LENGTH), SOUT.subarray_mut(VEL));
    }
}
