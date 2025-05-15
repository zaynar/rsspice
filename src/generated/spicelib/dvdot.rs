//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Derivative of Vector Dot Product, 3-D
///
/// Compute the derivative of the dot product of two double
/// precision position vectors.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  S1         I   First state vector in the dot product.
///  S2         I   Second state vector in the dot product.
///
///  The function returns the derivative of the dot product <S1,S2>
/// ```
///
/// # Detailed Input
///
/// ```text
///  S1       is any state vector. The components are in order
///           (x, y, z, dx/dt, dy/dt, dz/dt )
///
///  S2       is any state vector.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the derivative of the dot product of the
///  position portions of the two state vectors S1 and S2.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  Given two state vectors S1 and S2 made up of position and
///  velocity components (P1,V1) and (P2,V2) respectively,
///  DVDOT calculates the derivative of the dot product of P1 and P2,
///  i.e. the time derivative
///
///        d
///        -- < P1, P2 > = < V1, P2 > + < P1, V2 >
///        dt
///
///  where <,> denotes the dot product operation.
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
///  1) Suppose that given two state vectors whose position components
///     are unit vectors, and that we need to compute the rate of
///     change of the angle between the two vectors.
///
///     Example code begins here.
///
///
///           PROGRAM DVDOT_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DVDOT
///           DOUBLE PRECISION      VDOT
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      DTHETA
///           DOUBLE PRECISION      S1     (6)
///           DOUBLE PRECISION      S2     (6)
///
///     C
///     C     Define the two state vectors whose position
///     C     components are unit vectors.
///     C
///           DATA                  S1 /
///          .         7.2459D-01,  6.6274D-01, 1.8910D-01,
///          .        -1.5990D-06,  1.6551D-06, 7.4873D-07 /
///
///           DATA                  S2 /
///          .         8.4841D-01, -4.7790D-01, -2.2764D-01,
///          .         1.0951D-07,  1.0695D-07,  4.8468D-08 /
///
///     C
///     C     We know that the Cosine of the angle THETA between them
///     C     is given by
///     C
///     C        cos(THETA) = VDOT(S1,S2)
///     C
///     C     Thus by the chain rule, the derivative of the angle is
///     C     given by:
///     C
///     C        sin(THETA) dTHETA/dt = DVDOT(S1,S2)
///     C
///     C     Thus for values of THETA away from zero we can compute
///     C     dTHETA/dt as:
///     C
///           DTHETA = DVDOT(S1,S2) / SQRT( 1 - VDOT(S1,S2)**2 )
///
///           WRITE(*,'(A,F18.12)') 'Rate of change of angle '
///          . //                   'between S1 and S2:', DTHETA
///
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Rate of change of angle between S1 and S2:   -0.000002232415
///
///
///     Note that if the position components of S1 and S2 are parallel,
///     the derivative of the  angle between the positions does not
///     exist. Any code that computes the derivative of the angle
///     between two position vectors should account for the case
///     when the position components are parallel.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The user is responsible for determining that the states S1 and
///      S2 are not so large as to cause numeric overflow. In most
///      cases this won't present a problem.
///
///  2)  An implicit assumption exists that S1 and S2 are specified in
///      the same reference frame. If this is not the case, the
///      numerical result has no meaning.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code examples. Added entry #2 to $Restrictions.
///
/// -    SPICELIB Version 1.0.0, 18-MAY-1995 (WLT)
/// ```
pub fn dvdot(s1: &[f64; 6], s2: &[f64; 6]) -> f64 {
    let ret = DVDOT(s1, s2);
    ret
}

//$Procedure DVDOT  ( Derivative of Vector Dot Product, 3-D )
pub fn DVDOT(S1: &[f64], S2: &[f64]) -> f64 {
    let S1 = DummyArray::new(S1, 1..=6);
    let S2 = DummyArray::new(S2, 1..=6);
    let mut DVDOT: f64 = 0.0;

    DVDOT = ((((((S1[1] * S2[4]) + (S1[2] * S2[5])) + (S1[3] * S2[6])) + (S1[4] * S2[1]))
        + (S1[5] * S2[2]))
        + (S1[6] * S2[3]));

    DVDOT
}
