//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const GRPOFF: i32 = -2;

//$Procedure      PODRGI ( Pod, replace group, integer )
pub fn PODRGI(POD: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut POD = DummyArrayMut::new(POD, LBCELL..);
    let mut NEWOFF: i32 = 0;
    let mut NUMBER: i32 = 0;
    let mut OFFSET: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"PODRGI", ctx)?;
    }

    //
    // At any given time, the offset of the active group is stored
    // in location GRPOFF of the control area, so POD(GRPOFF) tells
    // us the location of the element preceding the active group.
    //
    // This element is a backward pointer, containing the offset of
    // the previous group; and so on, with turtles all the way down.
    // For example, consider a pod with three groups
    //
    //     G.  <10>
    //     1.  Bob
    //     2.  Carol
    //     3.  Ted
    //     4.  Alice
    //     5.  <0>
    //     6.  Fred
    //     7.  Wilma
    //     8.  Barney
    //     9.  Bettey
    //    10.  <5>
    //    11.  Ricky
    //    12.  Lucy
    //    13.  Fred
    //    14.  Ethel
    //
    // When the second group was created, the offset of the first
    // group (zero) was appended to the pod; the location of this
    // offset became the offset for the second group. When the
    // third group was created, the offset of the second group (5)
    // was appended; the location of this offset became the offset for
    // the third group. The offset for the third group is located
    // in element GRPOFF.
    //
    // To remove a group then, all that is necessary is to look at
    // element GRPOFF to get the offset of the current group; go to
    // that location to get the offset of the previous group; and
    // move that offset into element GRPOFF. To replace the previous
    // group, just move all of the elements of the active group
    // toward the front of the pod. The new cardinality, of course,
    // should be the new offset plus the number of elements in the
    // original group.
    //
    // If the pod contains only one group, we don't have to do
    // anything.
    //
    PODONI(POD.as_slice(), &mut OFFSET, &mut NUMBER, ctx)?;

    if (OFFSET != 0) {
        POD[GRPOFF] = POD[OFFSET];
        DCODEI(POD[OFFSET], &mut NEWOFF, ctx)?;

        for I in 1..=NUMBER {
            POD[(NEWOFF + I)] = POD[(OFFSET + I)];
        }

        spicelib::SCARDI((NEWOFF + NUMBER), POD.as_slice_mut(), ctx)?;
    }

    spicelib::CHKOUT(b"PODRGI", ctx)?;
    Ok(())
}
