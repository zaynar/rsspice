//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Clear an array of strings
///
/// Fill an array of strings with blank strings.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NDIM       I   Number of elements of ARRAY to be set to blank.
///  ARRAY      O   Array of strings to be filled with blank strings.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NDIM     is the number of elements in ARRAY which are to be set to
///           blank.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ARRAY    is the array of strings with each of its NDIM elements
///           set to a blank string. If NDIM is smaller than the
///           declared dimension of ARRAY, only the first NDIM
///           elements of ARRAY are set to blank; all other elements
///           remain unchanged.
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
///  1) Initialize a one dimensional string array and then clear
///     the first two strings.
///
///
///     Example code begins here.
///
///
///           PROGRAM CLEARC_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               STRSZ
///           PARAMETER           ( STRSZ = 21 )
///
///           INTEGER               NDIM
///           PARAMETER           ( NDIM = 4   )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(STRSZ)     ARRAY(NDIM)
///           INTEGER               I
///
///     C
///     C     Initialize ARRAY.
///     C
///           ARRAY(1) = 'Element #1'
///           ARRAY(2) = 'Element #2'
///           ARRAY(3) = 'Element #3'
///           ARRAY(4) = 'Element #4'
///
///           WRITE(*,'(A)') 'Contents of ARRAY before CLEARC:'
///           WRITE(*,*)
///           DO I = 1, NDIM
///              WRITE(*,'(A,I1,2A)') 'Position #', I , ': ', ARRAY(I)
///           END DO
///
///     C
///     C     Clear the first 2 elements.
///     C
///           CALL CLEARC ( 2, ARRAY )
///
///           WRITE(*,*)
///           WRITE(*,'(A)') 'Contents of ARRAY after CLEARC:'
///           WRITE(*,*)
///           DO I = 1, NDIM
///              WRITE(*,'(A,I1,2A)') 'Position #', I , ': ', ARRAY(I)
///           END DO
///
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Contents of ARRAY before CLEARC:
///
///     Position #1: Element #1
///     Position #2: Element #2
///     Position #3: Element #3
///     Position #4: Element #4
///
///     Contents of ARRAY after CLEARC:
///
///     Position #1:
///     Position #2:
///     Position #3: Element #3
///     Position #4: Element #4
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.M. Owen          (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 19-MAY-2021 (JDR) (NJB)
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
pub fn clearc(ndim: i32, array: CharArrayMut) {
    CLEARC(ndim, array);
}

//$Procedure CLEARC ( Clear an array of strings )
pub fn CLEARC(NDIM: i32, ARRAY: CharArrayMut) {
    let mut ARRAY = DummyCharArrayMut::new(ARRAY, None, 1..);

    //
    // Local variables
    //

    for I in 1..=NDIM {
        fstr::assign(ARRAY.get_mut(I), b" ");
    }
}
