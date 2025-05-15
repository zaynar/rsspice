//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const NATTR: i32 = 3;

//$Procedure      PLTFRM ( Get platform attributes )
pub fn PLTFRM(ROOM: i32, N: &mut i32, ATTR: CharArrayMut, ctx: &mut Context) {
    let mut ATTR = DummyCharArrayMut::new(ATTR, None, 1..);
    let mut ITEM = ActualCharArray::new(WDSIZE, 1..=NATTR);
    let mut LIMIT: i32 = 0;

    //~  NEXT
    //*    IMPLICIT NONE
    //~~

    fstr::assign(ITEM.get_mut(1), b"SYSTEM");
    fstr::assign(ITEM.get_mut(2), b"COMPILER");
    fstr::assign(ITEM.get_mut(3), b"O/S");

    LIMIT = intrinsics::MAX0(&[0, intrinsics::MIN0(&[NATTR, ROOM])]);

    for I in 1..=LIMIT {
        spicelib::ZZPLATFM(&ITEM[I], &mut ATTR[I], ctx);
    }

    *N = LIMIT;
}
