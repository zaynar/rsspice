//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXPLC: i32 = 14;
const MAXSIZ: i32 = (MAXPLC + 12);

//$Procedure      ZZMKPC ( Make a time format picture mark )
pub fn ZZMKPC(PICTUR: &mut [u8], B: i32, E: i32, MARK: &[u8], PATTRN: &[u8]) {
    let mut PLACES = [b' '; MAXPLC as usize];
    let mut MYMARK = [b' '; MAXSIZ as usize];
    let mut LPAT: i32 = 0;
    let mut LMRK: i32 = 0;
    let mut POINT: i32 = 0;
    let mut USE: i32 = 0;
    let mut LAST: i32 = 0;

    fstr::assign(&mut PLACES, b"##############");
    //
    // Construct the replacement marker.  First the unmodified
    // portion of the marker.  (We use LAST as the pointer to the
    // last valid character of the marker).
    //
    LMRK = LASTNB(MARK);
    LPAT = intrinsics::LEN(PATTRN);
    fstr::assign(&mut MYMARK, MARK);
    LAST = LMRK;
    //
    // Is there a decimal point in the pattern?
    //
    POINT = intrinsics::INDEX(PATTRN, b".");

    if (POINT > 0) {
        //
        // We've got a decimal point.  We have to at least put this
        // into the marker.
        //
        LAST = (LAST + 1);
        fstr::assign(fstr::substr_mut(&mut MYMARK, LAST..=LAST), b".");
        //
        // If the decimal point is not at the end of the pattern, we
        // will need to add some #'s to the marker (but not more than
        // MAXPLC of them).
        //
        if (POINT < LPAT) {
            USE = intrinsics::MIN0(&[MAXPLC, (LPAT - POINT)]);
            fstr::assign(
                fstr::substr_mut(&mut MYMARK, (LAST + 1)..),
                fstr::substr(&PLACES, 1..=USE),
            );
            LAST = (LAST + USE);
        }
    }
    //
    // We now let REPSUB do the work of replacing the substring
    // PICTUR(B:E) with the marker we've constructed.
    //
    ZZREPSUB(
        &PICTUR.to_vec(),
        B,
        E,
        fstr::substr(&MYMARK, 1..=LAST),
        PICTUR,
    );
}
