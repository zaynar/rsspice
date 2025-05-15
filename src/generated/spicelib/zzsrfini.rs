//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MXNSRF: i32 = 2000;
const SFNMLN: i32 = 36;
const NROOM: i32 = 2003;
const LBSNGL: i32 = -5;
const SIZIDX: i32 = 0;
const FREIDX: i32 = -1;

//$Procedure ZZSRFINI ( Private --- Surface-Code Hash Initialization )
pub fn ZZSRFINI(
    NORNAM: CharArray,
    CODES: &[i32],
    BODIES: &[i32],
    NVALS: i32,
    MAXVAL: i32,
    SNMHLS: &mut [i32],
    SNMPOL: &mut [i32],
    SNMIDX: &mut [i32],
    SIDHLS: &mut [i32],
    SIDPOL: &mut [i32],
    SIDIDX: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let NORNAM = DummyCharArray::new(NORNAM, None, 1..);
    let CODES = DummyArray::new(CODES, 1..);
    let BODIES = DummyArray::new(BODIES, 1..);
    let mut SNMHLS = DummyArrayMut::new(SNMHLS, 1..);
    let mut SNMPOL = DummyArrayMut::new(SNMPOL, LBSNGL..);
    let mut SNMIDX = DummyArrayMut::new(SNMIDX, 1..);
    let mut SIDHLS = DummyArrayMut::new(SIDHLS, 1..);
    let mut SIDPOL = DummyArrayMut::new(SIDPOL, LBSNGL..);
    let mut SIDIDX = DummyArrayMut::new(SIDIDX, 1..);
    let mut SQSHNM = [b' '; SFNMLN as usize];
    let mut HEAD: i32 = 0;
    let mut ITEMAT: i32 = 0;
    let mut LOOKAT: i32 = 0;
    let mut NODE: i32 = 0;
    let mut FULL: bool = false;
    let mut IDNEW: bool = false;
    let mut LFOUND: bool = false;
    let mut NAMNEW: bool = false;

    //
    // SPICELIB functions.
    //

    //
    // Hash control area items.
    //

    //
    // Local Variables
    //

    //
    // This routine uses discovery check-in.
    //
    //
    // Consistency check.
    //
    if (MAXVAL < NVALS) {
        CHKIN(b"ZZSRFINI", ctx)?;
        SETMSG(b"There is an inconsistency between the number of input bodies and the size of the output hashes. The number of input bodies was #. The size of the output hashes was #.", ctx);
        ERRINT(b"#", NVALS, ctx);
        ERRINT(b"#", MAXVAL, ctx);
        SIGERR(b"SPICE(BUG1)", ctx)?;
        CHKOUT(b"ZZSRFINI", ctx)?;
        return Ok(());
    }

    //
    // Initialize output hashes. Set all collision list pointers
    // to 0, which is the null value.
    //
    ZZHSIINI(MAXVAL, SIDHLS.as_slice_mut(), SIDPOL.as_slice_mut(), ctx)?;
    ZZHSCINI(MAXVAL, SNMHLS.as_slice_mut(), SNMPOL.as_slice_mut(), ctx)?;

    CLEARI(SIDPOL[0], SIDPOL.subarray_mut(1));
    CLEARI(SNMPOL[0], SNMPOL.subarray_mut(1));

    //
    // Loop through the input arrays to populate hashes. We do it
    // backwards to pick and register only the highest priority (latest)
    // values for each pair of normalized surface name and body ID code.
    //
    // If multiple surface ID codes are associated with a name, only
    // the highest priority ID is associated with the name in the
    // mapping data structures.
    //
    for I in intrinsics::range(NVALS, 1, -1) {
        //
        // Register this normalized surface name and body ID, but only if
        // the name from this pair is not already in the hash.
        //
        // We must traverse the collision list for the normalized surface
        // name "manually," since we have to check the body ID for each
        // matching name.
        //
        // Use hash function to get index of the head node. We apply the
        // hash function to a version of the normalized name that has
        // all blanks compressed out. (A normalized name may have single
        // blanks separating words comprising the name.)
        //
        CMPRSS(b" ", 0, &NORNAM[I], &mut SQSHNM);

        LOOKAT = ZZHASH2(&SQSHNM, SNMPOL[SIZIDX], ctx)?;

        HEAD = SNMHLS[LOOKAT];

        //
        // Indicate name and body were not found to begin with.
        //
        LFOUND = false;
        ITEMAT = 0;
        NAMNEW = true;

        //
        // See if this normalized name and corresponding body ID are,
        // respectively, in the normalized name list and body ID list.
        // Note that the body ID list is not a parallel array to the
        // normalized name array: we use the name pool pointer array
        // SNMIDX to indicate the location of the body ID corresponding
        // to a name.
        //
        NODE = HEAD;

        if (NODE > 0) {
            //
            // Start at the head node and check each normalized name saved
            // for this hash value until we find a name and body ID that
            // match or run out of items in the collision list.
            //
            while ((NODE > 0) && !LFOUND) {
                LFOUND = (fstr::eq(NORNAM.get(SNMIDX[NODE]), NORNAM.get(I))
                    && (BODIES[SNMIDX[NODE]] == BODIES[I]));

                ITEMAT = NODE;
                NODE = SNMPOL[NODE];
            }

            NAMNEW = !LFOUND;
        }

        //
        // ITEMAT is the node at which a name match was found by the
        // above loop, if a match was found. ITEMAT is the tail node of
        // the list if no match was found. It is 0 if the list is empty.
        //
        // ITEMAT will be used below only if the list is non-empty and
        // no match was found, in which case ITEMAT is non-zero.
        //
        if NAMNEW {
            //
            // We need to add the current normalized name and BODY ID
            // to the hash. Make sure there's room.
            //
            FULL = (SNMPOL[FREIDX] > SNMPOL[SIZIDX]);

            if FULL {
                CHKIN(b"ZZSRFINI", ctx)?;
                SETMSG(b"Could not add name # body ID # to the hash.", ctx);
                ERRCH(b"#", &NORNAM[I], ctx);
                ERRINT(b"#", BODIES[I], ctx);
                SIGERR(b"SPICE(BUG2)", ctx)?;
                CHKOUT(b"ZZSRFINI", ctx)?;
                return Ok(());
            } else {
                //
                // Store the item at the first free location in
                // the collision pool.
                //
                NODE = SNMPOL[FREIDX];
                SNMPOL[FREIDX] = (SNMPOL[FREIDX] + 1);

                if (HEAD > 0) {
                    //
                    // Link the new entry at the tail of the applicable
                    // collision list. The index of the tail node is ITEMAT.
                    //
                    SNMPOL[ITEMAT] = NODE;
                } else {
                    //
                    // Insert the new head node into the head list.
                    //
                    SNMHLS[LOOKAT] = NODE;
                }
                //
                // Set the index in the data arrays for the new pool
                // entry.
                //
                SNMIDX[NODE] = I;
            }

            //
            // NAMNEW indicates that the Ith normalized name and body ID
            // pair was not in the hash prior to the above block of code.
            //
            // We may have a situation when a single normalized surface
            // name and body ID pair maps to more than one surface ID. In
            // such cases we want to completely mask all surface IDs with
            // the lower priority. This is easy to do by simply not
            // attempting to register any more surface IDs if the name was
            // already registered due to a higher-indexed assignment.
            //
            // Register this surface ID and body ID pair, but only if it
            // is not already in the hash.
            //
            // We must traverse the collision list for the normalized
            // surface name "manually," since we have to check the body ID
            // for each matching surface ID.
            //
            // Use hash function to get index of the head node.
            //
            LOOKAT = ZZHASHI(CODES[I], SIDPOL[SIZIDX], ctx)?;

            HEAD = SIDHLS[LOOKAT];

            //
            // Indicate surface ID and body were not found to begin with.
            //
            LFOUND = false;
            ITEMAT = 0;
            IDNEW = true;

            //
            // See if this surface ID and corresponding body ID are,
            // respectively, in the surface ID list and body ID list.
            //
            NODE = HEAD;

            if (NODE > 0) {
                //
                // Start at the head node and check each surface ID saved
                // for this hash value until we find a surface ID and body
                // ID that match or run out of items in this collision
                // list.
                //
                while ((NODE > 0) && !LFOUND) {
                    LFOUND =
                        ((CODES[SIDIDX[NODE]] == CODES[I]) && (BODIES[SIDIDX[NODE]] == BODIES[I]));

                    ITEMAT = NODE;
                    NODE = SIDPOL[NODE];
                }

                IDNEW = !LFOUND;
            }

            //
            // ITEMAT is the node at which a surface ID code match was
            // found by the above loop, if a match was found. ITEMAT is
            // the tail node of the list if no match was found. It is 0 if
            // the list is empty.
            //
            // ITEMAT will be used below only if the list is non-empty and
            // no match was found, in which case ITEMAT is non-zero.
            //
            if IDNEW {
                //
                // We need to add the current surface ID and BODY ID
                // to the hash. Make sure there's room.
                //
                FULL = (SIDPOL[FREIDX] > SIDPOL[SIZIDX]);

                if FULL {
                    CHKIN(b"ZZSRFINI", ctx)?;
                    SETMSG(b"Could not add surface ID # body ID # to the hash.", ctx);
                    ERRINT(b"#", CODES[I], ctx);
                    ERRINT(b"#", BODIES[I], ctx);
                    SIGERR(b"SPICE(BUG3)", ctx)?;
                    CHKOUT(b"ZZSRFINI", ctx)?;
                    return Ok(());
                } else {
                    //
                    // Store the item at the first free location in the
                    // collision pool.
                    //
                    NODE = SIDPOL[FREIDX];
                    SIDPOL[FREIDX] = (SIDPOL[FREIDX] + 1);

                    if (HEAD > 0) {
                        //
                        // Link the new entry at the tail of the applicable
                        // collision list. The index of the tail node is
                        // ITEMAT.
                        //
                        SIDPOL[ITEMAT] = NODE;
                    } else {
                        //
                        // Insert the new head node into the head list.
                        //
                        SIDHLS[LOOKAT] = NODE;
                    }
                    //
                    // Set the index in the data arrays for the new pool
                    // entry.
                    //
                    SIDIDX[NODE] = I;
                }
            }
            //
            // We've processed the new (surface ID, body ID) pair.
            //
        }
        //
        // We've processed the Ith mapping between (surface name, body
        // ID) and (surface ID, body ID).
        //
    }

    Ok(())
}
