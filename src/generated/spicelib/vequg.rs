//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector equality, general dimension
///
/// Make one double precision vector of arbitrary dimension equal
/// to another.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  VIN        I   Double precision n-dimensional vector.
///  NDIM       I   Dimension of VIN (and also VOUT).
///  VOUT       O   Double precision n-dimensional vector set equal
///                 to VIN.
/// ```
///
/// # Detailed Input
///
/// ```text
///  VIN      is an arbitrary, double precision n-dimensional vector.
///
///  NDIM     is the dimension of VIN and VOUT.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is a double precision n-dimensional vector set equal
///           to VIN.
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
///  The code simply sets each component of VOUT equal to the
///  corresponding component of VIN.
///
///  Note that this routine may be used in place of MOVED, which
///  sets each output array element equal to the corresponding
///  input array element.
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
///  1) Lets assume we have a pointing record that contains the
///     start time of an interpolation interval, the components of
///     the quaternion that represents the C-matrix associated with
///     the start time of the interval, and the angular velocity vector
///     of the interval. The following example demonstrates how to
///     extract the time, the quaternion and the angular velocity
///     vector into separate variables for their processing.
///
///
///     Example code begins here.
///
///
///           PROGRAM VEQUG_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      AV     ( 3 )
///           DOUBLE PRECISION      QUAT   ( 4 )
///           DOUBLE PRECISION      RECORD ( 8 )
///           DOUBLE PRECISION      TIME
///
///           INTEGER               I
///
///     C
///     C     Define the pointing record. We would normally obtain it
///     C     from, e.g. CK readers or other non SPICE data files.
///     C
///           DATA                  RECORD  /
///          .      283480.753D0,   0.99999622D0,  0.0D0,  0.0D0,
///          .     -0.0027499965D0, 0.0D0,         0.0D0,  0.01D0 /
///
///     C
///     C     Get the time, quaternion and angular velocity vector
///     C     into separate variables.
///     C
///           TIME = RECORD(1)
///
///           CALL VEQUG  ( RECORD(2), 4, QUAT )
///           CALL VEQU   ( RECORD(6),    AV   )
///
///     C
///     C     Display the contents of the variables.
///     C
///           WRITE(*,'(A,F11.3)') 'Time            :', TIME
///
///           WRITE(*,'(A)')       'Quaternion      :'
///           WRITE(*,'(4F15.10)')  QUAT
///           WRITE(*,'(A)')       'Angular velocity:'
///           WRITE(*,'(3F15.10)')  AV
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Time            : 283480.753
///     Quaternion      :
///        0.9999962200   0.0000000000   0.0000000000  -0.0027499965
///     Angular velocity:
///        0.0000000000   0.0000000000   0.0100000000
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.M. Owen          (JPL)
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
///         code example based on existing example.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn vequg(vin: &[f64], ndim: i32, vout: &mut [f64]) {
    VEQUG(vin, ndim, vout);
}

//$Procedure VEQUG ( Vector equality, general dimension )
pub fn VEQUG(VIN: &[f64], NDIM: i32, VOUT: &mut [f64]) {
    let VIN = DummyArray::new(VIN, 1..=NDIM);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=NDIM);

    //
    // Local variables
    //

    for I in 1..=NDIM {
        VOUT[I] = VIN[I];
    }
}
