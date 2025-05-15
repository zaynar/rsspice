//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Product of an integer array
///
/// Return the product of the elements of an integer array.
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
///     PRODAI = ARRAY(1) * ARRAY(2) * ... * ARRAY(N)
///
///  If N is zero or negative, PRODAI is one.
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
///  zero or negative, PRODAI is one.
/// ```
///
/// # Examples
///
/// ```text
///  Let ARRAY contain the following elements.
///
///        ARRAY(1) = 12
///        ARRAY(2) =  2
///        ARRAY(3) =  4
///        ARRAY(4) = 75
///        ARRAY(5) = 18
///
///  Then
///
///        PRODAI ( ARRAY,   -3 )       =      1
///        PRODAI ( ARRAY,    0 )       =      1
///        PRODAI ( ARRAY,    1 )       =     12
///        PRODAI ( ARRAY,    2 )       =     24
///        PRODAI ( ARRAY,    5 )       = 129600
///        PRODAI ( ARRAY(3), 3 )       =   5400
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  PRODAI does not check for overflow. (For integers, this can
///      occur relatively quickly.)
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
/// -    SPICELIB Version 1.1.0, 08-APR-2021 (JDR)
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
pub fn prodai(array: &[i32], n: i32) -> i32 {
    let ret = PRODAI(array, n);
    ret
}

//$Procedure PRODAI ( Product of an integer array )
pub fn PRODAI(ARRAY: &[i32], N: i32) -> i32 {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let mut PRODAI: i32 = 0;
    let mut PROD: i32 = 0;

    //
    // Local variables
    //

    //
    // Begin at one.
    //
    PROD = 1;

    //
    // Multiply the elements. If N is zero or negative, nothing happens.
    //
    for I in 1..=N {
        PROD = (PROD * ARRAY[I]);
    }

    //
    // Return the product.
    //
    PRODAI = PROD;

    PRODAI
}
