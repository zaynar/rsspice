//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector equality, 3 dimensions
///
/// Make one double precision 3-dimensional vector equal to
/// another.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  VIN        I   Double precision 3-dimensional vector.
///  VOUT       O   Double precision 3-dimensional vector set equal
///                 to VIN.
/// ```
///
/// # Detailed Input
///
/// ```text
///  VIN      is an arbitrary, double precision 3-dimensional vector.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is a double precision 3-dimensional vector set equal
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
///  VEQU simply sets each component of VOUT in turn equal to VIN. No
///  error checking is performed because none is needed.
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
///           PROGRAM VEQU_EX1
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
///         code example.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn vequ(vin: &[f64; 3], vout: &mut [f64; 3]) {
    VEQU(vin, vout);
}

//$Procedure VEQU ( Vector equality, 3 dimensions )
pub fn VEQU(VIN: &[f64], VOUT: &mut [f64]) {
    let VIN = DummyArray::new(VIN, 1..=3);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=3);

    VOUT[1] = VIN[1];
    VOUT[2] = VIN[2];
    VOUT[3] = VIN[3];
}
