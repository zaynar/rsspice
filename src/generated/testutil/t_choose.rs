//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      T_CHOOSE ( Compute "N choose K" )
pub fn T_CHOOSE(N: i32, K: i32) -> i32 {
    let mut T_CHOOSE: i32 = 0;
    let mut R: i32 = 0;

    //
    // Local variables
    //

    //
    // Deal with the exceptional cases first.
    //
    if (N < 0) {
        T_CHOOSE = 0;
        return T_CHOOSE;
    } else if (K < 0) {
        T_CHOOSE = 0;
        return T_CHOOSE;
    } else if (K > N) {
        T_CHOOSE = 0;
        return T_CHOOSE;
    }

    //
    // We wish to compute
    //
    //        n!
    //    ---------
    //    (n-k)! k!
    //
    //
    // This is equal to
    //
    //    [ N (N-1) (N-2)...(N-K+1) ] / [ K (K-1)... 1 ]
    //
    // The above number is an integer because it's the number
    // of possible ways to choose a subset of k items out of
    // a set of n items. One can also prove the number is
    // an integer by mathematical induction; the induction
    // step is to prove, for 0 < k < n, that
    //
    //
    //        n!                (n-1)!                 (n-1)!
    //    ---------  =  ---------------------   +  -------------
    //    (n-k)! k!     ((n-1)-(k-1))! (k-1)!      ((n-1)-k)! k!
    //
    // The right hand side terms are integers by induction. The
    // case of k = 0 is evident by inspection.
    //
    // Since we know that
    //
    //    n * (n-1) * ... * (n-k+1)
    //    -------------------------
    //    1 *   2   * ... * k
    //
    // is an integer for any k,  0 < k <= n, we can build up the
    // expression we want by iterating on k.
    //
    // We must take care to avoid any division that would have a
    // non-integer result; we can do this by taking as the numerator the
    // product of the first i numerator terms in the above quotient and
    // taking as the denominator the product of the first i terms of the
    // denominator above. The quotient of those two products is
    // guaranteed to be an integer since it's equal to "n choose i."
    //

    R = ((N - K) + 1);
    T_CHOOSE = 1;

    for I in intrinsics::range(N, R, -1) {
        T_CHOOSE = ((T_CHOOSE * I) / ((N + 1) - I));
    }

    T_CHOOSE
}
