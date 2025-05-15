//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBPOOL: i32 = -5;
const SIZIDX: i32 = 0;
const FREIDX: i32 = -1;

//$Procedure ZZHSI ( Private---Add-only Integer Hash )
pub fn ZZHSI(
    HASHSZ: i32,
    HEDLST: &[i32],
    COLLST: &[i32],
    ITEMS: &[i32],
    ITEM: i32,
    PARAM: &[u8],
    ITEMAT: i32,
    NEW: bool,
    AVAIL: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // Hash control area items.
    //

    //
    // SPICELIB functions.
    //

    //
    // Local variables.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZHSI", ctx)?;
    }

    //
    // Signal bogus entry error and check out.
    //
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZHSI", ctx)?;

    Ok(())
}

//$Procedure ZZHSIINI ( Private---Initialize Add-only Integer Hash )
pub fn ZZHSIINI(
    HASHSZ: i32,
    HEDLST: &mut [i32],
    COLLST: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut HEDLST = DummyArrayMut::new(HEDLST, 1..);
    let mut COLLST = DummyArrayMut::new(COLLST, LBPOOL..);
    let mut I: i32 = 0;

    //
    // Standard SPICE error handling.
    //
    CHKIN(b"ZZHSIINI", ctx)?;

    if (HASHSZ > 0) {
        //
        // Wipe out head node pointer list.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = HASHSZ;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                HEDLST[I] = 0;
                I += m3__;
            }
        }
        //
        // Reset control area.
        //
        COLLST[SIZIDX] = HASHSZ;
        COLLST[FREIDX] = 1;
    }

    //
    // The requested number of nodes must be valid. ZZHASHI will check
    // that.
    //
    I = ZZHASHI(1, HASHSZ, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZHSIINI", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZHSIINI", ctx)?;

    Ok(())
}

//$Procedure ZZHSIADD ( Private---Add an Item to Add-only Integer Hash )
pub fn ZZHSIADD(
    HEDLST: &mut [i32],
    COLLST: &mut [i32],
    ITEMS: &mut [i32],
    ITEM: i32,
    ITEMAT: &mut i32,
    NEW: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut HEDLST = DummyArrayMut::new(HEDLST, 1..);
    let mut COLLST = DummyArrayMut::new(COLLST, LBPOOL..);
    let mut ITEMS = DummyArrayMut::new(ITEMS, 1..);
    let mut LOOKAT: i32 = 0;
    let mut NODE: i32 = 0;
    let mut FULL: bool = false;
    let mut LFOUND: bool = false;

    //
    // Standard SPICE error handling. No checking-in here. We will do it
    // when we have to.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Set flag indicating whether the hash is full.
    //
    FULL = (COLLST[FREIDX] > COLLST[SIZIDX]);

    //
    // Use simple division hash function to get index of the head node.
    //
    if (COLLST[SIZIDX] < 1) {
        CHKIN(b"ZZHSIADD", ctx)?;
        SETMSG(b"Uninitialized hash. Size was #.", ctx);
        ERRINT(b"#", COLLST[SIZIDX], ctx);
        SIGERR(b"SPICE(UNINITIALIZEDHASH)", ctx)?;
        CHKOUT(b"ZZHSIADD", ctx)?;
        return Ok(());
    }

    LOOKAT = ZZHASHI(ITEM, COLLST[SIZIDX], ctx)?;
    NODE = HEDLST[LOOKAT];

    //
    // Set initial values.
    //
    LFOUND = false;
    *NEW = false;

    //
    // See if this item (or one colliding with it in the hash scheme)
    // has already been stored in the item list.
    //
    if (NODE > 0) {
        //
        // Start at the head node and check each item saved for this hash
        // value until we find a item that matches or run out of items in
        // this conflict resolution list.
        //
        while ((NODE > 0) && !LFOUND) {
            LFOUND = (ITEMS[NODE] == ITEM);
            *ITEMAT = NODE;
            NODE = COLLST[NODE];
        }

        //
        // If we didn't find this item on the conflict resolution list
        // and our hash is not full we will add this item to it.
        //
        if (!LFOUND && !FULL) {
            //
            // Get next free node.
            //
            NODE = COLLST[FREIDX];

            //
            // Increment next free node pointer.
            //
            COLLST[FREIDX] = (COLLST[FREIDX] + 1);

            //
            // Set current node pointer to the node we just picked for
            // this item.
            //
            COLLST[*ITEMAT] = NODE;

            //
            // Set new node pointer to 0, just in case.
            //
            COLLST[NODE] = 0;

            //
            // Save item.
            //
            ITEMS[NODE] = ITEM;

            //
            // Set output node ID and new and found flags.
            //
            *ITEMAT = NODE;
            *NEW = true;
            LFOUND = true;
        }
    } else if !FULL {
        //
        // Nothing like this item (in the hashing sense) has been stored
        // so far and the hash is not full.
        //
        // Get next free node.
        //
        NODE = COLLST[FREIDX];

        //
        // Increment next free node pointer.
        //
        COLLST[FREIDX] = (COLLST[FREIDX] + 1);

        //
        // Set new node pointer to 0, just in case.
        //
        COLLST[NODE] = 0;

        //
        // Set the head node pointer to this node.
        //
        HEDLST[LOOKAT] = NODE;

        //
        // Save item.
        //
        ITEMS[NODE] = ITEM;

        //
        // Set output node ID and new and found flags.
        //
        *ITEMAT = NODE;
        *NEW = true;
        LFOUND = true;
    }

    //
    // Set ITEMAT to 0 if LFOUND is FALSE.
    //
    if !LFOUND {
        *ITEMAT = 0;
    }

    //
    // If the item hash was full and we didn't find this item we've got
    // an error. Report it and return.
    //
    if (FULL && !LFOUND) {
        CHKIN(b"ZZHSIADD", ctx)?;

        SETMSG(b"The hash has no room for any more items.", ctx);
        SIGERR(b"SPICE(HASHISFULL)", ctx)?;

        CHKOUT(b"ZZHSIADD", ctx)?;
        return Ok(());
    }

    Ok(())
}

//$Procedure ZZHSICHK ( Private---Find an Item in Add-only Integer Hash )
pub fn ZZHSICHK(
    HEDLST: &[i32],
    COLLST: &[i32],
    ITEMS: &[i32],
    ITEM: i32,
    ITEMAT: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let HEDLST = DummyArray::new(HEDLST, 1..);
    let COLLST = DummyArray::new(COLLST, LBPOOL..);
    let ITEMS = DummyArray::new(ITEMS, 1..);
    let mut LOOKAT: i32 = 0;
    let mut NODE: i32 = 0;
    let mut LFOUND: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Use simple division hash function to get index of the head node.
    //
    if (COLLST[SIZIDX] < 1) {
        CHKIN(b"ZZHSICHK", ctx)?;
        SETMSG(b"Uninitialized hash. Size was #.", ctx);
        ERRINT(b"#", COLLST[SIZIDX], ctx);
        SIGERR(b"SPICE(UNINITIALIZEDHASH)", ctx)?;
        CHKOUT(b"ZZHSICHK", ctx)?;
        return Ok(());
    }

    LOOKAT = ZZHASHI(ITEM, COLLST[SIZIDX], ctx)?;
    NODE = HEDLST[LOOKAT];

    //
    // Set initial values.
    //
    LFOUND = false;

    //
    // See if this item (or one colliding with it in the hash scheme) is
    // in the item list.
    //
    if (NODE > 0) {
        //
        // Start at the head node and check each item saved for this hash
        // value until we find a item that matches or run out of items in
        // this conflict resolution list.
        //
        while ((NODE > 0) && !LFOUND) {
            LFOUND = (ITEMS[NODE] == ITEM);
            *ITEMAT = NODE;
            NODE = COLLST[NODE];
        }
    }

    //
    // If LFOUND is false, set ITEMAT to 0.
    //
    if !LFOUND {
        *ITEMAT = 0;
    }

    Ok(())
}

//$Procedure ZZHSIAVL ( Private---Get room available in Add-only Hash )
pub fn ZZHSIAVL(COLLST: &[i32], AVAIL: &mut i32) {
    let COLLST = DummyArray::new(COLLST, LBPOOL..);

    //
    // Set the number of unoccupied slots in the hash.
    //
    *AVAIL = ((COLLST[SIZIDX] - COLLST[FREIDX]) + 1);
}

//$Procedure ZZHSIINF ( Private---Get Information about Add-only Hash )
pub fn ZZHSIINF(
    HEDLST: &[i32],
    COLLST: &[i32],
    ITEMS: &[i32],
    PARAM: &[u8],
    AVAIL: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let HEDLST = DummyArray::new(HEDLST, 1..);
    let COLLST = DummyArray::new(COLLST, LBPOOL..);
    let mut I: i32 = 0;
    let mut LOOKAT: i32 = 0;
    let mut NODE: i32 = 0;

    //
    // Get the hash size.
    //
    if fstr::eq(PARAM, b"HASH SIZE") {
        *AVAIL = COLLST[SIZIDX];

    //
    // Get the count of used nodes in the head list.
    //
    } else if fstr::eq(PARAM, b"USED HEADNODE COUNT") {
        *AVAIL = 0;
        {
            let m1__: i32 = 1;
            let m2__: i32 = COLLST[SIZIDX];
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if (HEDLST[I] != 0) {
                    *AVAIL = (*AVAIL + 1);
                }
                I += m3__;
            }
        }

    //
    // Get the count of unused nodes in the head list.
    //
    } else if fstr::eq(PARAM, b"UNUSED HEADNODE COUNT") {
        *AVAIL = 0;
        {
            let m1__: i32 = 1;
            let m2__: i32 = COLLST[SIZIDX];
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if (HEDLST[I] == 0) {
                    *AVAIL = (*AVAIL + 1);
                }
                I += m3__;
            }
        }

    //
    // Get the count of used slots in the item list.
    //
    } else if fstr::eq(PARAM, b"USED ITEM COUNT") {
        *AVAIL = (COLLST[FREIDX] - 1);

    //
    // Get the count of unused slots in the item list.
    //
    } else if fstr::eq(PARAM, b"UNUSED ITEM COUNT") {
        *AVAIL = ((COLLST[SIZIDX] - COLLST[FREIDX]) + 1);

    //
    // Get the size of the longest item list for any hash value.
    //
    } else if fstr::eq(PARAM, b"LONGEST LIST SIZE") {
        *AVAIL = 0;
        {
            let m1__: i32 = 1;
            let m2__: i32 = COLLST[SIZIDX];
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                NODE = HEDLST[I];
                LOOKAT = 0;
                while (NODE > 0) {
                    LOOKAT = (LOOKAT + 1);
                    NODE = COLLST[NODE];
                }
                *AVAIL = intrinsics::MAX0(&[*AVAIL, LOOKAT]);
                I += m3__;
            }
        }

    //
    // This parameter is not supported.
    //
    } else {
        *AVAIL = 0;

        CHKIN(b"ZZHSIINF", ctx)?;
        SETMSG(b"Parameter \'#\' is not recognized.", ctx);
        ERRCH(b"#", PARAM, ctx);
        SIGERR(b"SPICE(ITEMNOTRECOGNIZED)", ctx)?;
        CHKOUT(b"ZZHSIINF", ctx)?;
    }

    Ok(())
}
