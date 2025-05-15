//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Sum of an integer array
///
/// Return the sum of the elements of an integer array.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ARRAY      I   Input array.
///  N          I   Number of elements in ARRAY.
///
///  The function returns the sum of the elements of ARRAY.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ARRAY    is the input integer array.
///
///  N        is the number of elements in the array.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the sum of the elements of the input array.
///  That is,
///
///     SUMAI( ARRAY, N ) = ARRAY(1) + ARRAY(2) + ... + ARRAY(N)
///
///  If N is zero or negative, SUMAI is zero.
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
///  The value of the function is initially set to zero. The elements
///  of the array are then added. If the number of elements is zero or
///  negative, SUMAI is zero.
/// ```
///
/// # Examples
///
/// ```text
///  Let ARRAY contain the following elements.
///
///        ARRAY(1) = 12
///        ARRAY(2) =  1
///        ARRAY(3) =  4
///        ARRAY(4) = 75
///        ARRAY(5) = 18
///
///  Then
///
///        SUMAI ( ARRAY,   -3 )       =   0
///        SUMAI ( ARRAY,    0 )       =   0
///        SUMAI ( ARRAY,    1 )       =  12
///        SUMAI ( ARRAY,    2 )       =  13
///        SUMAI ( ARRAY,    5 )       = 110
///        SUMAI ( ARRAY(3), 3 )       =  97
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  SUMAI does not check for overflow.
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
/// -    SPICELIB Version 1.1.0, 09-APR-2021 (JDR)
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
pub fn sumai(array: &[i32], n: i32) -> i32 {
    let ret = SUMAI(array, n);
    ret
}

//$Procedure SUMAI ( Sum of an integer array )
pub fn SUMAI(ARRAY: &[i32], N: i32) -> i32 {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let mut SUMAI: i32 = 0;
    let mut SUM: i32 = 0;

    //
    // Local variables
    //

    //
    // Begin at zero.
    //
    SUM = 0;

    //
    // Sum the elements. If N is zero or negative, nothing happens.
    //
    for I in 1..=N {
        SUM = (SUM + ARRAY[I]);
    }

    //
    // Return the sum.
    //
    SUMAI = SUM;

    SUMAI
}
