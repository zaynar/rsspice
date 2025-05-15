//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Shell sort a character array
///
/// Sort an array of character strings according to the ASCII
/// collating sequence using the Shell Sort algorithm.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NDIM       I   Dimension of the array.
///  ARRAY     I-O  The array.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NDIM     is the number of elements in the array to be sorted.
///
///  ARRAY    on input, is the array to be sorted.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ARRAY    on output, contains the same elements, sorted
///           according to the ASCII collating sequence.
///           The actual sorting is done in place in ARRAY.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If NDIM < 2, this routine does not modify the array.
/// ```
///
/// # Particulars
///
/// ```text
///  The Shell Sort Algorithm is well known.
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
///  1) Given a list of words, sort it according to the ASCII
///     collating sequence using the Shell Sort algorithm.
///
///
///     Example code begins here.
///
///
///           PROGRAM SHELLC_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NDIM
///           PARAMETER           ( NDIM  = 6 )
///
///           INTEGER               WRDSZ
///           PARAMETER           ( WRDSZ = 8 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(WRDSZ)     ARRAY  ( NDIM )
///           INTEGER               I
///
///     C
///     C     Let ARRAY contain the following elements:
///     C
///           ARRAY(1) = 'FEYNMAN'
///           ARRAY(2) = 'NEWTON'
///           ARRAY(3) = 'EINSTEIN'
///           ARRAY(4) = 'GALILEO'
///           ARRAY(5) = 'EUCLID'
///           ARRAY(6) = 'Galileo'
///
///     C
///     C     Print ARRAY before calling SHELLC.
///     C
///           WRITE(*,*) 'Array before calling SHELLC:'
///           WRITE(*,*)
///           DO I = 1, NDIM
///              WRITE(*,*) '   ', ARRAY(I)
///           END DO
///           WRITE(*,*)
///
///     C
///     C     Call SHELLC and print ARRAY again.
///     C
///           CALL SHELLC ( NDIM, ARRAY )
///
///           WRITE(*,*) 'Array after calling SHELLC:'
///           WRITE(*,*)
///           DO I = 1, NDIM
///              WRITE(*,*) '   ', ARRAY(I)
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Array before calling SHELLC:
///
///         FEYNMAN
///         NEWTON
///         EINSTEIN
///         GALILEO
///         EUCLID
///         Galileo
///
///      Array after calling SHELLC:
///
///         EINSTEIN
///         EUCLID
///         FEYNMAN
///         GALILEO
///         Galileo
///         NEWTON
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
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
///         code example based on existing example. Extended $Exceptions
///         section to explain what happens if NDIM < 2.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn shellc(ndim: i32, array: CharArrayMut) {
    SHELLC(ndim, array);
}

//$Procedure SHELLC ( Shell sort a character array )
pub fn SHELLC(NDIM: i32, ARRAY: CharArrayMut) {
    let mut ARRAY = DummyCharArrayMut::new(ARRAY, None, 1..);
    let mut GAP: i32 = 0;
    let mut J: i32 = 0;
    let mut JG: i32 = 0;

    //
    // Local variables
    //

    //
    // This is a straightforward implementation of the Shell Sort
    // algorithm.
    //
    GAP = (NDIM / 2);

    while (GAP > 0) {
        for I in (GAP + 1)..=NDIM {
            J = (I - GAP);
            while (J > 0) {
                JG = (J + GAP);

                if fstr::le(&ARRAY[J], &ARRAY[JG]) {
                    J = 0;
                } else {
                    SWAPC_ARRAY(ARRAY.subscript(J), ARRAY.subscript(JG), ARRAY.as_arg_mut());
                }

                J = (J - GAP);
            }
        }

        GAP = (GAP / 2);
    }
}
