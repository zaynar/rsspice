//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXL: i32 = 36;
const MAXP: i32 = 150;
const NPERM: i32 = 692;
const MAXE: i32 = 853;
const NROOM: i32 = 14983;
const LBPOOL: i32 = -5;

//$Procedure ZZBODINI ( Private --- Body-Code Hash Initialization )
pub fn ZZBODINI(
    NAMES: CharArray,
    NORNAM: CharArray,
    CODES: &[i32],
    NVALS: i32,
    MAXVAL: i32,
    BNMLST: &mut [i32],
    BNMPOL: &mut [i32],
    BNMNMS: CharArrayMut,
    BNMIDX: &mut [i32],
    BIDLST: &mut [i32],
    BIDPOL: &mut [i32],
    BIDIDS: &mut [i32],
    BIDIDX: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let NORNAM = DummyCharArray::new(NORNAM, Some(MAXL), 1..);
    let CODES = DummyArray::new(CODES, 1..);
    let mut BNMLST = DummyArrayMut::new(BNMLST, 1..);
    let mut BNMPOL = DummyArrayMut::new(BNMPOL, LBPOOL..);
    let mut BNMNMS = DummyCharArrayMut::new(BNMNMS, Some(MAXL), 1..);
    let mut BNMIDX = DummyArrayMut::new(BNMIDX, 1..);
    let mut BIDLST = DummyArrayMut::new(BIDLST, 1..);
    let mut BIDPOL = DummyArrayMut::new(BIDPOL, LBPOOL..);
    let mut BIDIDS = DummyArrayMut::new(BIDIDS, 1..);
    let mut BIDIDX = DummyArrayMut::new(BIDIDX, 1..);
    let mut ITEM: i32 = 0;
    let mut NEW: bool = false;

    //
    // Local Variables
    //

    //
    // Consistency check.
    //
    if (MAXVAL < NVALS) {
        CHKIN(b"ZZBODINI", ctx)?;
        SETMSG(b"There is an inconsistency between the number of input bodies and the size of the output hashes. The number of input bodies was #. The size of the output hashes was #.", ctx);
        ERRINT(b"#", NVALS, ctx);
        ERRINT(b"#", MAXVAL, ctx);
        SIGERR(b"SPICE(BUG1)", ctx)?;
        CHKOUT(b"ZZBODINI", ctx)?;
        return Ok(());
    }

    //
    // Initialize output hashes.
    //
    ZZHSIINI(MAXVAL, BIDLST.as_slice_mut(), BIDPOL.as_slice_mut(), ctx)?;
    ZZHSCINI(MAXVAL, BNMLST.as_slice_mut(), BNMPOL.as_slice_mut(), ctx)?;

    //
    // Loop through the input arrays to populate hashes. We do it
    // backwards to pick and register only the highest priority (latest)
    // values for each normalized name.
    //
    for I in intrinsics::range(NVALS, 1, -1) {
        //
        // Register this normalized name, but only if it is not already
        // in the hash.
        //
        ZZHSCADD(
            BNMLST.as_slice_mut(),
            BNMPOL.as_slice_mut(),
            BNMNMS.as_arg_mut(),
            &NORNAM[I],
            &mut ITEM,
            &mut NEW,
            ctx,
        )?;
        if NEW {
            if (ITEM != 0) {
                BNMIDX[ITEM] = I;
            } else {
                CHKIN(b"ZZBODINI", ctx)?;
                SETMSG(b"Could not add name # to the hash.", ctx);
                ERRCH(b"#", &NORNAM[I], ctx);
                SIGERR(b"SPICE(BUG3)", ctx)?;
                CHKOUT(b"ZZBODINI", ctx)?;
            }
        }

        //
        // We may have a situation when a single normalized name maps to
        // more than one ID. In such cases we want to completely mask all
        // IDs with the lower priority. This is easy to do by simply not
        // attempting to register any more IDs if the name is already
        // registered.
        //
        if NEW {
            //
            // Register this ID, but only if it is not already in the hash.
            //
            ZZHSIADD(
                BIDLST.as_slice_mut(),
                BIDPOL.as_slice_mut(),
                BIDIDS.as_slice_mut(),
                CODES[I],
                &mut ITEM,
                &mut NEW,
                ctx,
            )?;
            if NEW {
                if (ITEM != 0) {
                    BIDIDX[ITEM] = I;
                } else {
                    CHKIN(b"ZZBODINI", ctx)?;
                    SETMSG(b"Could not add ID # to the hash.", ctx);
                    ERRINT(b"#", CODES[I], ctx);
                    SIGERR(b"SPICE(BUG2)", ctx)?;
                    CHKOUT(b"ZZBODINI", ctx)?;
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}
