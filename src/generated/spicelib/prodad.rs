//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Product of a double precision array
///
/// Return the product of the elements of a double precision array.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ARRAY      I   Input array.
///  N          I   Number of elements in ARRAY.
///
///  The function returns the product of the elements of ARRAY.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ARRAY    is the input array.
///
///  N        is the number of elements in the array.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the product of the elements of the input
///  array. That is,
///
///     PRODAD = ARRAY(1) * ARRAY(2) * ... * ARRAY(N)
///
///  If N is zero or negative, PRODAD is one.
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
///  The value of the function is initially set to one. The elements
///  of the array are then multiplied. If the number of elements is
///  zero or negative, PRODAD is one.
/// ```
///
/// # Examples
///
/// ```text
///  Let ARRAY contain the following elements.
///
///        ARRAY(1) = 12.D0
///        ARRAY(2) =  2.D0
///        ARRAY(3) =  4.D0
///        ARRAY(4) = 75.D0
///        ARRAY(5) = 18.D0
///
///  Then
///
///        PRODAD ( ARRAY,   -3 )       =      1.D0
///        PRODAD ( ARRAY,    0 )       =      1.D0
///        PRODAD ( ARRAY,    1 )       =     12.D0
///        PRODAD ( ARRAY,    2 )       =     24.D0
///        PRODAD ( ARRAY,    5 )       = 129600.D0
///        PRODAD ( ARRAY(3), 3 )       =   5400.D0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  PRODAD does not check for overflow.
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
/// -    SPICELIB Version 1.1.0, 07-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn prodad(array: &[f64], n: i32) -> f64 {
    let ret = PRODAD(array, n);
    ret
}

//$Procedure PRODAD ( Product of a double precision array )
pub fn PRODAD(ARRAY: &[f64], N: i32) -> f64 {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let mut PRODAD: f64 = 0.0;
    let mut PROD: f64 = 0.0;

    //
    // Local variables
    //

    //
    // Begin at one.
    //
    PROD = 1.0;

    //
    // Multiply the elements. If N is zero or negative, nothing happens.
    //
    for I in 1..=N {
        PROD = (PROD * ARRAY[I]);
    }

    //
    // Return the product.
    //
    PRODAD = PROD;

    PRODAD
}
