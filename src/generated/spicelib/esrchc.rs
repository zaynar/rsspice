//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Equivalence search, character
///
/// Search for a given value within a character string array.
/// Return the index of the first equivalent array entry, or zero
/// if no equivalent element is found.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  VALUE      I   Key value to be found in ARRAY.
///  NDIM       I   Dimension of ARRAY.
///  ARRAY      I   Character string array to search.
///
///  The function returns the index of the first array entry equivalent
///  to VALUE, or zero if none is found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  VALUE    is an arbitrary character string.
///
///  NDIM     is the dimension of (number of elements in) an array of
///           character strings.
///
///  ARRAY    is the array.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the index of the first element of the input
///  array equivalent to the input value, or zero if the array contains
///  no such elements.
///
///  Two strings are equivalent if they contain the same characters in
///  the same order, when blanks are ignored and uppercase and
///  lowercase characters are considered equal.
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
///  ESRCHC is identical to ISRCHC, except that it looks for the first
///  equivalent string (as defined by EQSTR) instead of the first
///  identical one.
/// ```
///
/// # Examples
///
/// ```text
///  Let ARRAY contain the following elements:
///
///     ARRAY(1) = 'This'
///     ARRAY(2) = 'little'
///     ARRAY(3) = 'piggy'
///     ARRAY(4) = 'went'
///     ARRAY(5) = 'to'
///     ARRAY(6) = 'market'
///
///  Then
///
///     ESRCHC ( 'PIGGY',      6, ARRAY )  =  3
///     ESRCHC ( ' LiTtLe  ',  6, ARRAY )  =  2
///     ESRCHC ( 'W e n t',    6, ARRAY )  =  4
///     ESRCHC ( 'mall',       6, ARRAY )  =  0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  ESRCHC assumes that the function EQSTR does not participate
///      in normal SPICELIB error handling.
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
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
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
pub fn esrchc(value: &str, ndim: i32, array: CharArray) -> i32 {
    let ret = ESRCHC(value.as_bytes(), ndim, array);
    ret
}

//$Procedure ESRCHC ( Equivalence search, character )
pub fn ESRCHC(VALUE: &[u8], NDIM: i32, ARRAY: CharArray) -> i32 {
    let ARRAY = DummyCharArray::new(ARRAY, None, 1..);
    let mut ESRCHC: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Just like ISRCHC.
    //
    ESRCHC = 0;

    for I in 1..=NDIM {
        if EQSTR(&ARRAY[I], VALUE) {
            ESRCHC = I;
            return ESRCHC;
        }
    }

    ESRCHC
}
