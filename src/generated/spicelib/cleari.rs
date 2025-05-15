//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Clear an integer array
///
/// Fill an integer array with zeros.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NDIM       I   The number of elements of ARRAY which are to be
///                 set to zero.
///  ARRAY      O   Integer array to be filled.
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
///  ARRAY    is the integer array which is to be filled with
///           zeros.
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
///  1) Initialize all members of an integer array to the same value
///     and clear it afterwards.
///
///
///     Example code begins here.
///
///
///           PROGRAM CLEARI_EX1
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
///           INTEGER               ARRAY ( NDIM )
///           INTEGER               I
///
///     C
///     C     Initialize all member of the array ARRAY to 11, and
///     C     print out its contents.
///     C
///           CALL FILLI ( 11, NDIM, ARRAY )
///
///           WRITE(*,'(A)') 'Contents of ARRAY before CLEARI:'
///           WRITE(*,'(4I4)') ( ARRAY(I), I=1, NDIM )
///
///     C
///     C     Clear the contents of ARRAY and print it.
///     C
///           CALL CLEARI ( NDIM, ARRAY )
///
///           WRITE(*,*)
///           WRITE(*,'(A)') 'Contents of ARRAY after CLEARI:'
///           WRITE(*,'(4I4)') ( ARRAY(I), I=1, NDIM )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Contents of ARRAY before CLEARI:
///       11  11  11  11
///
///     Contents of ARRAY after CLEARI:
///        0   0   0   0
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
/// -    SPICELIB Version 1.1.0, 19-MAY-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Updated the header to comply with NAIF standard. Added
///         full code example.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn cleari(ndim: i32, array: &mut [i32]) {
    CLEARI(ndim, array);
}

//$Procedure CLEARI ( Clear an integer array )
pub fn CLEARI(NDIM: i32, ARRAY: &mut [i32]) {
    let mut ARRAY = DummyArrayMut::new(ARRAY, 1..);

    //
    // Local variables
    //

    for I in 1..=NDIM {
        ARRAY[I] = 0;
    }
}
