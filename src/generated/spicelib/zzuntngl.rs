//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZUNTNGL ( Untangle an AB linked-cell list  )
pub fn ZZUNTNGL(
    NPTR: i32,
    MAXCEL: i32,
    CELLS: &[i32],
    MAXB: i32,
    PNTRS: &mut [i32],
    NOUT: &mut i32,
    OUTLST: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CELLS = DummyArray2D::new(CELLS, 1..=2, 1..);
    let mut PNTRS = DummyArrayMut::new(PNTRS, 1..);
    let mut OUTLST = DummyArrayMut::new(OUTLST, 1..);
    let mut PTRDEX: i32 = 0;
    let mut NFOUND: i32 = 0;
    let mut ROOM: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZUNTNGL", ctx)?;

    if (NPTR > MAXCEL) {
        SETMSG(b"Input pointer array is larger than cell array. Pointer array size = #1. Cell array size = #2.", ctx);
        ERRINT(b"#1", NPTR, ctx);
        ERRINT(b"#2", MAXCEL, ctx);
        SIGERR(b"SPICE(BARRAYTOOSMALL)", ctx)?;
        CHKOUT(b"ZZUNTNGL", ctx)?;
        return Ok(());
    }

    //
    // ROOM is the remaining room in the output list.
    //
    ROOM = MAXB;

    //
    // Initialize pointer index.
    //
    PTRDEX = 0;

    //
    // Loop over all A-values.
    //
    for AVAL in 1..=NPTR {
        //
        // Traverse the chained list for a particular A-value
        // and collect associated B-values. If B-values exists,
        // return the number of B-vals to the OUTLST array
        // at element PTRDEX + 1; return the list of B-vals
        // starting at element PTRDEX + 2.
        //
        // Make sure the output pointers below are in range.
        //
        if ((PTRDEX + 2) > MAXB) {
            SETMSG(
                b"Index larger than output array. Index = #1. Array size = #2.",
                ctx,
            );
            ERRINT(b"#1", (PTRDEX + 2), ctx);
            ERRINT(b"#2", MAXB, ctx);
            SIGERR(b"SPICE(BARRAYTOOSMALL)", ctx)?;
            CHKOUT(b"ZZUNTNGL", ctx)?;
            return Ok(());
        }

        if (ROOM <= 0) {
            SETMSG(b"Remaining room in output array is #1. Current input pointer index = #2. Output array size = #3. Output pointer index is #4.", ctx);
            ERRINT(b"#1", ROOM, ctx);
            ERRINT(b"#2", AVAL, ctx);
            ERRINT(b"#3", MAXB, ctx);
            ERRINT(b"#4", PTRDEX, ctx);
            SIGERR(b"SPICE(BARRAYTOOSMALL)", ctx)?;
            CHKOUT(b"ZZUNTNGL", ctx)?;
            return Ok(());
        }

        let [arg6, arg7] = OUTLST
            .get_disjoint_slices_mut([(PTRDEX + 1), (PTRDEX + 2)])
            .unwrap();
        ZZTRVLNK(
            AVAL,
            NPTR,
            PNTRS.as_slice(),
            MAXCEL,
            CELLS.as_slice(),
            ROOM,
            arg6.first_mut().unwrap(),
            arg7,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZUNTNGL", ctx)?;
            return Ok(());
        }

        //
        // Increment pointer PTRDEX if we found any B-vals.
        //
        NFOUND = OUTLST[(PTRDEX + 1)];

        if (NFOUND > 0) {
            //
            // Store in PNTRS the pointer to the returned list.
            // This assignment overwrites the input pointer from
            // AVAL to the head of the associated list in CELLS.
            //
            PNTRS[AVAL] = (PTRDEX + 1);
            //
            // Update the count of available spaces in the
            // output list. Account for the list size stored
            // at the front of the list.
            //
            ROOM = (ROOM - (1 + NFOUND));
            //
            // Increment PTRDEX to mark the position of the final
            // B-val.
            //
            PTRDEX = ((PTRDEX + 1) + NFOUND);
        } else {
            //
            // If no associated B-values exist, set the
            // PNTRS element to -1, indicating no B-value.
            //
            PNTRS[AVAL] = -1;
        }
    }

    //
    // Return the current pointer value.
    //
    *NOUT = PTRDEX;

    //
    // Standard SPICE error handling.
    //
    CHKOUT(b"ZZUNTNGL", ctx)?;
    Ok(())
}
