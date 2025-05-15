//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      ZZSIZEOK ( Determine if the size of a segment is ok )
pub fn ZZSIZEOK(
    SIZE: i32,
    PSIZE: i32,
    DSIZE: i32,
    OFFSET: i32,
    OK: &mut bool,
    N: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut Q: i32 = 0;
    let mut PD1: i32 = 0;
    let mut R: i32 = 0;
    let mut A: i32 = 0;

    //
    //  Here's the scoop.
    //
    //  Suppose N is a solution to SIZE = PSIZE*N + (N-OFFSET)/DSIZE
    //  N can be represented uniquely as
    //
    //      N = q*DSIZE + r
    //
    //  where OFFSET <= r <= DSIZE+OFFSET-1.  Therefore there must
    //  be values q and r such that
    //
    //     SIZE = PSIZE*(q*DSIZE + r ) + ( q*DSIZE + r - 1 ) / DSIZE
    //
    //          = PSIZE*DSIZE*q + q + PSIZE*r
    //
    //          = (PSIZE*DSIZE+1)*q + PSIZE*r
    //
    //  But SIZE can be represented uniquely as
    //
    //        SIZE = (PSIZE*DSIZE+1)*k + a
    //
    //  where  0 <= a < (PSIZE*DSIZE+1).
    //
    //  But   PSIZE*OFFSET < PSIZE*r < (PSIZE*DSIZE+OFFSET-1),
    // therefore  it must be that
    //
    //           SIZE mod(PSIZE*DSIZE+1) = PSIZE*r
    //  and                            q = k
    //
    //  Hence, there is a solution to our equation if and only if
    //
    //       PSIZE divides SIZE mod(PSIZE*DSIZE+1)
    //  and  OFFSET*PSIZE <= SIZE mod(PSIZE*DSIZE+1)
    //

    //
    // Handle the exceptional case first.
    //
    if (((SIZE <= 0) || (DSIZE <= 0)) || (PSIZE <= 0)) {
        *N = 0;
        *OK = false;
        return Ok(());
    }

    PD1 = ((PSIZE * DSIZE) + 1);

    RMAINI(SIZE, PD1, &mut Q, &mut A, ctx)?;

    if ((OFFSET * PSIZE) > A) {
        *N = 0;
        *OK = false;
        return Ok(());
    }

    if (A == ((A / PSIZE) * PSIZE)) {
        R = (A / PSIZE);
        *N = ((DSIZE * Q) + R);
        *OK = true;
    } else {
        *OK = false;
        *N = 0;
    }

    Ok(())
}
