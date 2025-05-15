//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Unit Normalized Cross Product and Derivative
///
/// Compute the unit vector parallel to the cross product of
/// two 3-dimensional vectors and the derivative of this unit vector.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  S1         I   Left hand state for cross product and derivative.
///  S2         I   Right hand state for cross product and derivative.
///  SOUT       O   Unit vector and derivative of the cross product.
/// ```
///
/// # Detailed Input
///
/// ```text
///  S1       is any state vector. Typically, this might represent the
///           apparent state of a planet or the Sun, which defines the
///           orientation of axes of some coordinate system.
///
///  S2       is any state vector.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SOUT     is the unit vector parallel to the cross product of the
///           position components of S1 and S2 and the derivative of
///           the unit vector.
///
///           If the cross product of the position components is
///           the zero vector, then the position component of the
///           output will be the zero vector. The velocity component
///           of the output will simply be the derivative of the
///           cross product of the position components of S1 and S2.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the position components of S1 and S2 cross together to
///      give a zero vector, the position component of the output
///      will be the zero vector. The velocity component of the
///      output will simply be the derivative of the cross product
///      of the position vectors.
///
///  2)  If S1 and S2 are large in magnitude (taken together,
///      their magnitude surpasses the limit allowed by the
///      computer) then it may be possible to generate a
///      floating point overflow from an intermediate
///      computation even though the actual cross product and
///      derivative may be well within the range of double
///      precision numbers.
/// ```
///
/// # Particulars
///
/// ```text
///  DUCRSS calculates the unit vector parallel to the cross product
///  of two vectors and the derivative of that unit vector.
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
///  1) One can construct non-inertial coordinate frames from apparent
///     positions of objects or defined directions. However, if one
///     wants to convert states in this non-inertial frame to states
///     in an inertial reference frame, the derivatives of the axes of
///     the non-inertial frame are required.
///
///     Define a reference frame with the apparent direction of the
///     Sun as seen from Earth as the primary axis X. Use the Earth
///     pole vector to define with the primary axis the XY plane of
///     the frame, with the primary axis Y pointing in the direction
///     of the pole.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: ducrss_ex1.tm
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
///           PROGRAM DUCRSS_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      STATE  ( 6    )
///           DOUBLE PRECISION      TRANS  ( 6, 6 )
///           DOUBLE PRECISION      X_NEW  ( 6    )
///           DOUBLE PRECISION      Y_NEW  ( 6    )
///           DOUBLE PRECISION      Z      ( 6    )
///           DOUBLE PRECISION      Z_NEW  ( 6    )
///           DOUBLE PRECISION      ZINERT ( 6    )
///
///           INTEGER               I
///
///
///     C
///     C     Define the earth body-fixed pole vector (Z). The pole
///     C     has no velocity in the Earth fixed frame IAU_EARTH.
///     C
///           DATA                  Z  / 0.D0, 0.D0, 1.D0,
///          .                           0.D0, 0.D0, 0.D0  /
///
///     C
///     C     Load SPK, PCK, and LSK kernels, use a meta kernel for
///     C     convenience.
///     C
///           CALL FURNSH ( 'ducrss_ex1.tm' )
///
///     C
///     C     Calculate the state transformation between IAU_EARTH and
///     C     J2000 at an arbitrary epoch.
///     C
///           CALL STR2ET ( 'Jan 1, 2009', ET )
///           CALL SXFORM ( 'IAU_EARTH', 'J2000', ET, TRANS )
///
///     C
///     C     Transform the earth pole vector from the IAU_EARTH frame
///     C     to J2000.
///     C
///           CALL MXVG ( TRANS, Z, 6, 6, ZINERT )
///
///     C
///     C     Calculate the apparent state of the Sun from Earth at
///     C     the epoch ET in the J2000 frame.
///     C
///           CALL SPKEZR ( 'Sun',   ET,   'J2000', 'LT+S',
///          .              'Earth', STATE, LT              )
///
///     C
///     C     Define the z axis of the new frame as the cross product
///     C     between the apparent direction of the Sun and the Earth
///     C     pole. Z_NEW cross X_NEW defines the Y axis of the
///     C     derived frame.
///     C
///           CALL DVHAT  ( STATE, X_NEW         )
///           CALL DUCRSS ( STATE, ZINERT, Z_NEW )
///           CALL DUCRSS ( Z_NEW, STATE,  Y_NEW )
///
///     C
///     C     Display the results.
///     C
///           WRITE(*,'(A)') 'New X-axis:'
///           WRITE(*,'(A,3F16.12)') '   position:', (X_NEW(I), I=1,3)
///           WRITE(*,'(A,3F16.12)') '   velocity:', (X_NEW(I), I=4,6)
///           WRITE(*,'(A)') 'New Y-axis:'
///           WRITE(*,'(A,3F16.12)') '   position:', (Y_NEW(I), I=1,3)
///           WRITE(*,'(A,3F16.12)') '   velocity:', (Y_NEW(I), I=4,6)
///           WRITE(*,'(A)') 'New Z-axis:'
///           WRITE(*,'(A,3F16.12)') '   position:', (Z_NEW(I), I=1,3)
///           WRITE(*,'(A,3F16.12)') '   velocity:', (Z_NEW(I), I=4,6)
///
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     New X-axis:
///        position:  0.183446637633 -0.901919663328 -0.391009273602
///        velocity:  0.000000202450  0.000000034660  0.000000015033
///     New Y-axis:
///        position:  0.078846540163 -0.382978080242  0.920386339077
///        velocity:  0.000000082384  0.000000032309  0.000000006387
///     New Z-axis:
///        position: -0.979862518033 -0.199671507623  0.000857203851
///        velocity:  0.000000044531 -0.000000218531 -0.000000000036
///
///
///     Note that these vectors define the transformation between the
///     new frame and J2000 at the given ET:
///
///            .-            -.
///            |       :      |
///            |   R   :  0   |
///        M = | ......:......|
///            |       :      |
///            | dRdt  :  R   |
///            |       :      |
///            `-            -'
///
///     with
///
///        DATA         R     / X_NEW(1:3), Y_NEW(1:3), Z_NEW(1:3)  /
///
///        DATA         dRdt  / X_NEW(4:6), Y_NEW(4:6), Z_NEW(4:6)  /
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  No checking of S1 or S2 is done to prevent floating point
///      overflow. The user is required to determine that the magnitude
///      of each component of the states is within an appropriate range
///      so as not to cause floating point overflow. In almost every
///      case there will be no problem and no checking actually needs
///      to be done.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
///         Added complete code example.
///
/// -    SPICELIB Version 1.2.0, 08-APR-2014 (NJB)
///
///         Now scales inputs to reduce chance of numeric
///         overflow.
///
/// -    SPICELIB Version 1.1.1, 22-APR-2010 (NJB)
///
///         Header correction: assertions that the output
///         can overwrite the input have been removed.
///
/// -    SPICELIB Version 1.1.0, 30-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in DVHAT call.
///
/// -    SPICELIB Version 1.0.0, 15-JUN-1995 (WLT)
/// ```
pub fn ducrss(s1: &[f64; 6], s2: &[f64; 6], sout: &mut [f64; 6]) {
    DUCRSS(s1, s2, sout);
}

//$Procedure DUCRSS ( Unit Normalized Cross Product and Derivative )
pub fn DUCRSS(S1: &[f64], S2: &[f64], SOUT: &mut [f64]) {
    let S1 = DummyArray::new(S1, 1..=6);
    let S2 = DummyArray::new(S2, 1..=6);
    let mut SOUT = DummyArrayMut::new(SOUT, 1..=6);
    let mut F1: f64 = 0.0;
    let mut F2: f64 = 0.0;
    let mut SCLS1 = StackArray::<f64, 6>::new(1..=6);
    let mut SCLS2 = StackArray::<f64, 6>::new(1..=6);
    let mut TMPSTA = StackArray::<f64, 6>::new(1..=6);

    //
    // Local variables
    //

    //
    // Scale the components of the input states so the states have the
    // same direction and angular rates, but their largest position
    // components have absolute value equal to 1. Do not modify states
    // that have all position components equal to zero.
    //
    F1 = intrinsics::DMAX1(&[f64::abs(S1[1]), f64::abs(S1[2]), f64::abs(S1[3])]);
    F2 = intrinsics::DMAX1(&[f64::abs(S2[1]), f64::abs(S2[2]), f64::abs(S2[3])]);

    if (F1 > 0.0) {
        VSCLG((1.0 / F1), S1.as_slice(), 6, SCLS1.as_slice_mut());
    } else {
        MOVED(S1.as_slice(), 6, SCLS1.as_slice_mut());
    }

    if (F2 > 0.0) {
        VSCLG((1.0 / F2), S2.as_slice(), 6, SCLS2.as_slice_mut());
    } else {
        MOVED(S2.as_slice(), 6, SCLS2.as_slice_mut());
    }

    //
    // Not much to this.  Just get the cross product and its derivative.
    // Using that, get the associated unit vector and its derivative.
    //
    DVCRSS(SCLS1.as_slice(), SCLS2.as_slice(), TMPSTA.as_slice_mut());
    DVHAT(TMPSTA.as_slice(), SOUT.as_slice_mut());
}
