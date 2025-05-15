//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Is it an order vector
///
/// Determine whether an array of N items contains the integers
/// 1 through N.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ARRAY      I   Array of integers.
///  N          I   Number of integers in ARRAY.
///
///  The function returns .TRUE. if the array contains the integers
///  1 through N, otherwise it returns .FALSE.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ARRAY    is an array of integers. Often this will be an array
///           that is a candidate order vector to be passed to
///           a routine for re-ordering some parallel array.
///
///  N        is the number of elements in ARRAY.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns .TRUE. if the array contains the integers
///  1 through N. Otherwise it returns .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If N < 1, the function returns .FALSE.
/// ```
///
/// # Particulars
///
/// ```text
///  This function provides a simple means of determining whether
///  or not an array of N integers contains exactly the integers
///  1 through N.
/// ```
///
/// # Examples
///
/// ```text
///  1) Suppose you wished to reorder an array of strings based upon
///     a ranking array supplied by a user. If the ranking array
///     contains any duplicates or refers to indices that are out
///     of the range of valid indices for the array of strings,
///     the attempt to reorder the array of strings cannot succeed.
///     Its usually better to detect such a possibility before
///     you begin trying to reorder the array of strings. This routine
///     will detect the error.
///
///     The block of code below illustrates this idea.
///
///
///        IF ( ISORDV ( ORDVEC, N ) ) THEN
///
///           ...reorder the input array of strings
///
///           CALL REORDC ( ORDVEC, N, STRNGS )
///
///        ELSE
///
///           ...state the problem and let the user decide what
///           to do about it.
///                 .
///                 .
///                 .
///
///        END IF
///
///
///  2) This routine can also be used to determine whether or not an
///     array contains every integer between K and N (where K < N ).
///
///
///        First subtract K-1 from each integer
///
///        DO I = 1, N-K+1
///           ARRAY(I) = ARRAY(I) - K + 1
///        END DO
///
///        See if the modified array is an order vector
///
///        OK = ISORDV ( ARRAY, N-K )
///
///        Return the array to its original state.
///
///        DO I = 1, N-K+1
///           ARRAY(I) = ARRAY(I) + K - 1
///        END DO
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 03-JUL-2021 (JDR)
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
/// -    SPICELIB Version 1.0.0, 06-MAR-1991 (NJB) (WLT) (IMU)
/// ```
pub fn isordv(array: &mut [i32], n: i32) -> bool {
    let ret = ISORDV(array, n);
    ret
}

//$Procedure ISORDV ( Is it an order vector )
pub fn ISORDV(ARRAY: &mut [i32], N: i32) -> bool {
    let mut ARRAY = DummyArrayMut::new(ARRAY, 1..);
    let mut ISORDV: bool = false;
    let mut J: i32 = 0;

    //
    // Local variables
    //

    //
    // Let's take care of the goofy case first.
    //
    if (N < 1) {
        ISORDV = false;
        return ISORDV;
    } else if (N == 1) {
        ISORDV = (ARRAY[1] == 1);
        return ISORDV;
    }

    //
    // Make an initial pass through the array to be sure we
    // have legitimate values.
    //
    for I in 1..=N {
        if ((ARRAY[I] < 1) || (ARRAY[I] > N)) {
            ISORDV = false;
            return ISORDV;
        }
    }

    //
    // Ok. All of the values are in range.  We just need to check
    // that this array could actually be used as an order vector.
    //
    // For each I between 1 and N,  ARRAY(I) is some integer between 1
    // and N.  The only question remaining is whether the set
    // { ARRAY(I), I=1,N } contains every integer between 1 and N.
    //
    // Suppose for a moment we could allocate a logical array called HITS
    //
    //       LOGICAL               HITS(N)
    //
    // Then the following scheme could be used to determine whether or
    // not { ARRAY(I), I=1,N } contains every integer between 1 and N.
    //
    //    Initialize every entry of HITS to .FALSE.
    //
    //       DO I = 1, N
    //          HITS(I) = .FALSE.
    //       END DO
    //
    //    Then for each I set HITS(ARRAY(I)) to .TRUE.
    //
    //       DO I = 1, N
    //          HITS(ARRAY(I)) = .TRUE.
    //       END DO
    //
    // What can be said about HITS at this point? If for any entry J,
    // HITS(J) is true then some ARRAY(I) is equal to J.
    //
    // If all HITS are .TRUE. then {ARRAY(I), I=1,N} is in fact the
    // set of integers 1 to N.  Otherwise those J such that
    // HITS(J) = .FALSE. are the integers between 1 and N that are
    // missed by ARRAY.
    //
    // It turns out we don't need to allocate an array of logicals;
    // we can use just use part of the input array, ARRAY.
    //
    // The storage locations ARRAY(1) through ARRAY(N) can be viewed
    // as two parallel arrays:  SIGN_BIT and UNSIGNED
    //
    //      SIGN
    //      BIT  UNSIGNED PORTION
    //     +----+-----------------+
    //  1  |    |                 |
    //     +----+-----------------+
    //  2  |    |                 |
    //     +----+-----------------+
    //  3  |    |                 |
    //     +----+-----------------+
    //
    //             .
    //             .
    //             .
    //
    //     +----+-----------------+
    // N-1 |    |                 |
    //     +----+-----------------+
    // N   |    |                 |
    //     +----+-----------------+
    //
    //
    // Since we know the value of all of the sign bits (it's '+') we can
    // alter them and then reset them once we are done.
    //
    // We will choose for our array of HITS the SIGN_BITS of ARRAY.
    // We regard '+' as FALSE and '-' as TRUE.
    //
    //    DO I = 1, N
    //       SIGN_BIT ( UNSIGNED(I) ) = '-'
    //    END DO
    //
    // Then check to make sure that all of the sign bits are '-'.
    //
    for I in 1..=N {
        J = i32::abs(ARRAY[I]);
        ARRAY[J] = -ARRAY[J];
    }

    //
    // Check each item to see if it's been hit.
    //
    ISORDV = true;

    for I in 1..=N {
        ISORDV = (ISORDV && (ARRAY[I] < 0));
        ARRAY[I] = i32::abs(ARRAY[I]);
    }

    ISORDV
}
