//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Order of a character array
///
/// Determine the order of elements in an array of character strings.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ARRAY      I    Input array.
///  NDIM       I    Dimension of ARRAY.
///  IORDER     O    Order vector for ARRAY.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ARRAY    is the input array.
///
///  NDIM     is the number of elements in the input array.
/// ```
///
/// # Detailed Output
///
/// ```text
///  IORDER   is the order vector for the input array.
///           IORDER(1) is the index of the smallest element
///           of ARRAY; IORDER(2) is the index of the next
///           smallest; and so on. Strings are ordered according
///           to the ASCII collating sequence.
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
///  ORDERC finds the index of the smallest element of the input
///  array. This becomes the first element of the order vector.
///  The process is repeated for the rest of the elements.
///
///  The order vector returned by ORDERC may be used by any of
///  the REORD routines to sort sets of related arrays, as shown
///  in the example below.
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
///  1) Sort four related arrays (containing the names, masses,
///     integer ID codes, and flags indicating whether they have
///     a ring system, for a group of planets).
///
///
///     Example code begins here.
///
///
///           PROGRAM ORDERC_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local constants.
///     C
///           INTEGER                 NDIM
///           PARAMETER             ( NDIM   = 8  )
///
///           INTEGER                 STRLEN
///           PARAMETER             ( STRLEN = 7  )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(STRLEN)      NAMES  ( NDIM )
///
///           DOUBLE PRECISION        MASSES ( NDIM )
///
///           INTEGER                 CODES  ( NDIM )
///           INTEGER                 I
///           INTEGER                 IORDER ( NDIM )
///
///           LOGICAL                 RINGS  ( NDIM )
///
///     C
///     C     Set the arrays containing the names, masses (given as
///     C     ratios to of Solar GM to barycenter GM), integer ID
///     C     codes, and flags indicating whether they have a ring
///     C     system.
///     C
///           DATA                    NAMES  /
///          .            'MERCURY', 'VENUS',  'EARTH',  'MARS',
///          .            'JUPITER', 'SATURN', 'URANUS', 'NEPTUNE' /
///
///           DATA                    MASSES /
///          .                       22032.080D0,   324858.599D0,
///          .                      398600.436D0,    42828.314D0,
///          .                   126712767.881D0, 37940626.068D0,
///          .                     5794559.128D0,  6836534.065D0 /
///
///           DATA                    CODES  / 199, 299, 399, 499,
///          .                                 599, 699, 799, 899 /
///
///           DATA                    RINGS  /
///          .                   .FALSE., .FALSE., .FALSE., .FALSE.,
///          .                   .TRUE.,  .TRUE.,  .TRUE., .TRUE.   /
///
///     C
///     C     Sort the object arrays by name.
///     C
///           CALL ORDERC ( NAMES,  NDIM, IORDER )
///
///           CALL REORDC ( IORDER, NDIM, NAMES  )
///           CALL REORDD ( IORDER, NDIM, MASSES )
///           CALL REORDI ( IORDER, NDIM, CODES  )
///           CALL REORDL ( IORDER, NDIM, RINGS  )
///
///     C
///     C     Output the resulting table.
///     C
///           WRITE(*,'(A)') ' Planet   Mass(GMS/GM)  ID Code  Rings?'
///           WRITE(*,'(A)') '-------  -------------  -------  ------'
///
///           DO I = 1, NDIM
///
///              WRITE(*,'(A,F15.3,I9,L5)') NAMES(I), MASSES(I),
///          .                              CODES(I), RINGS(I)
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Planet   Mass(GMS/GM)  ID Code  Rings?
///     -------  -------------  -------  ------
///     EARTH       398600.436      399    F
///     JUPITER  126712767.881      599    T
///     MARS         42828.314      499    F
///     MERCURY      22032.080      199    F
///     NEPTUNE    6836534.065      899    T
///     SATURN    37940626.068      699    T
///     URANUS     5794559.128      799    T
///     VENUS       324858.599      299    F
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
/// -    SPICELIB Version 1.1.0, 04-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn orderc(array: CharArray, ndim: i32, iorder: &mut [i32]) {
    ORDERC(array, ndim, iorder);
}

//$Procedure ORDERC ( Order of a character array )
pub fn ORDERC(ARRAY: CharArray, NDIM: i32, IORDER: &mut [i32]) {
    let ARRAY = DummyCharArray::new(ARRAY, None, 1..);
    let mut IORDER = DummyArrayMut::new(IORDER, 1..);
    let mut GAP: i32 = 0;
    let mut J: i32 = 0;
    let mut JG: i32 = 0;

    //
    // Local variables
    //

    //
    // Begin with the initial ordering.
    //
    for I in 1..=NDIM {
        IORDER[I] = I;
    }

    //
    // Find the smallest element, then the next smallest, and so on.
    // This uses the Shell Sort algorithm, but swaps the elements of
    // the order vector instead of the array itself.
    //
    GAP = (NDIM / 2);

    while (GAP > 0) {
        for I in (GAP + 1)..=NDIM {
            J = (I - GAP);
            while (J > 0) {
                JG = (J + GAP);

                if fstr::le(&ARRAY[IORDER[J]], &ARRAY[IORDER[JG]]) {
                    J = 0;
                } else {
                    SWAPI_ARRAY(
                        IORDER.subscript(J),
                        IORDER.subscript(JG),
                        IORDER.as_slice_mut(),
                    );
                }

                J = (J - GAP);
            }
        }

        GAP = (GAP / 2);
    }
}
