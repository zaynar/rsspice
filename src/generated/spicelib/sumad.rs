//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Sum of a double precision array
///
/// Return the sum of the elements of a double precision array.
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
///  ARRAY    is the input double precision array.
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
///     SUMAD( ARRAY, N ) = ARRAY(1) + ARRAY(2) + ... + ARRAY(N)
///
///  If N is zero or negative, SUMAD is zero.
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
///  negative, SUMAD is zero.
/// ```
///
/// # Examples
///
/// ```text
///  Let ARRAY contain the following elements.
///
///        ARRAY(1) = 12.D0
///        ARRAY(2) =  1.D0
///        ARRAY(3) =  4.D0
///        ARRAY(4) = 75.D0
///        ARRAY(5) = 18.D0
///
///  Then
///
///        SUMAD ( ARRAY,   -3 )       =   0.D0
///        SUMAD ( ARRAY,    0 )       =   0.D0
///        SUMAD ( ARRAY,    1 )       =  12.D0
///        SUMAD ( ARRAY,    2 )       =  13.D0
///        SUMAD ( ARRAY,    5 )       = 110.D0
///        SUMAD ( ARRAY(3), 3 )       =  97.D0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  SUMAD does not check for overflow.
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
pub fn sumad(array: &[f64], n: i32) -> f64 {
    let ret = SUMAD(array, n);
    ret
}

//$Procedure SUMAD ( Sum of a double precision array )
pub fn SUMAD(ARRAY: &[f64], N: i32) -> f64 {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let mut SUMAD: f64 = 0.0;
    let mut SUM: f64 = 0.0;

    //
    // Local variables
    //

    //
    // Begin at zero.
    //
    SUM = 0.0;

    //
    // Sum the elements. If N is zero or negative, nothing happens.
    //
    for I in 1..=N {
        SUM = (SUM + ARRAY[I]);
    }

    //
    // Return the sum.
    //
    SUMAD = SUM;

    SUMAD
}
