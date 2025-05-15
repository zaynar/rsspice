//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Clear a double precision array
///
/// Fill a double precision array with zeros.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NDIM       I   The number of elements of ARRAY which are to be
///                 set to zero.
///  ARRAY      O   Double precision array to be filled.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NDIM     is the number of elements in ARRAY which are to be
///           set to zero.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ARRAY    is the double precision array which is to be filled
///           with zeros.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If NDIM < 1, the array is not modified.
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
///  1) Initialize all members of a double precision array to the
///     same value and clear it afterwards.
///
///
///     Example code begins here.
///
///
///           PROGRAM CLEARD_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NDIM
///           PARAMETER           ( NDIM = 4 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      ARRAY ( NDIM )
///
///           INTEGER               I
///
///     C
///     C     Initialize all member of the array ARRAY to 11.5, and
///     C     print out its contents.
///     C
///           CALL FILLD ( 11.5D0, NDIM, ARRAY )
///
///           WRITE(*,'(A)') 'Contents of ARRAY before CLEARD:'
///           WRITE(*,'(4F6.1)') ( ARRAY(I), I=1, NDIM )
///
///     C
///     C     Clear the contents of ARRAY and print it.
///     C
///           CALL CLEARD ( NDIM, ARRAY )
///
///           WRITE(*,*)
///           WRITE(*,'(A)') 'Contents of ARRAY after CLEARD:'
///           WRITE(*,'(4F6.1)') ( ARRAY(I), I=1, NDIM )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Contents of ARRAY before CLEARD:
///       11.5  11.5  11.5  11.5
///
///     Contents of ARRAY after CLEARD:
///        0.0   0.0   0.0   0.0
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.M. Owen          (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 29-MAY-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Updated the header to comply with NAIF standard. Added
///         full code example.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn cleard(ndim: i32, array: &mut [f64]) {
    CLEARD(ndim, array);
}

//$Procedure CLEARD ( Clear a double precision array )
pub fn CLEARD(NDIM: i32, ARRAY: &mut [f64]) {
    let mut ARRAY = DummyArrayMut::new(ARRAY, 1..);

    //
    // Local variables
    //

    for I in 1..=NDIM {
        ARRAY[I] = 0.0;
    }
}
