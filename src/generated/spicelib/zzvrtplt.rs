//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const IXNV: i32 = 1;
const IXNP: i32 = (IXNV + 1);
const IXNVXT: i32 = (IXNP + 1);
const IXVGRX: i32 = (IXNVXT + 1);
const IXCGSC: i32 = (IXVGRX + 3);
const IXVXPS: i32 = (IXCGSC + 1);
const IXVXLS: i32 = (IXVXPS + 1);
const IXVTLS: i32 = (IXVXLS + 1);
const IXPLAT: i32 = (IXVTLS + 1);
const IXDSCR: i32 = 1;
const DSCSZ2: i32 = 24;
const IXVTBD: i32 = (IXDSCR + DSCSZ2);
const IXVXOR: i32 = (IXVTBD + 6);
const IXVXSZ: i32 = (IXVXOR + 3);
const IXVERT: i32 = (IXVXSZ + 1);
const KWNV: i32 = 1;
const KWNP: i32 = (KWNV + 1);
const KWNVXT: i32 = (KWNP + 1);
const KWVGRX: i32 = (KWNVXT + 1);
const KWCGSC: i32 = (KWVGRX + 1);
const KWVXPS: i32 = (KWCGSC + 1);
const KWVXLS: i32 = (KWVXPS + 1);
const KWVTLS: i32 = (KWVXLS + 1);
const KWPLAT: i32 = (KWVTLS + 1);
const KWVXPT: i32 = (KWPLAT + 1);
const KWVXPL: i32 = (KWVXPT + 1);
const KWVTPT: i32 = (KWVXPL + 1);
const KWVTPL: i32 = (KWVTPT + 1);
const KWCGPT: i32 = (KWVTPL + 1);
const KWDSC: i32 = (KWCGPT + 1);
const KWVTBD: i32 = (KWDSC + 1);
const KWVXOR: i32 = (KWVTBD + 1);
const KWVXSZ: i32 = (KWVXOR + 1);
const KWVERT: i32 = (KWVXSZ + 1);
const MAXVRT: i32 = 16000002;
const MAXPLT: i32 = (2 * (MAXVRT - 2));
const MAXNPV: i32 = (((3 * MAXPLT) / 2) + 1);
const MAXVOX: i32 = 100000000;
const MAXCGR: i32 = 100000;
const MAXEDG: i32 = 120;
const SIVGRX: i32 = 1;
const SICGSC: i32 = (SIVGRX + 3);
const SIVXNP: i32 = (SICGSC + 1);
const SIVXNL: i32 = (SIVXNP + 1);
const SIVTNL: i32 = (SIVXNL + 1);
const SICGRD: i32 = (SIVTNL + 1);
const IXIFIX: i32 = (MAXCGR + 7);
const SIVTBD: i32 = 1;
const SIVXOR: i32 = (SIVTBD + 6);
const SIVXSZ: i32 = (SIVXOR + 3);
const IXDFIX: i32 = 10;
const MAXVXP: i32 = (MAXPLT / 2);
const MAXCEL: i32 = 60000000;
const MXNVLS: i32 = (MAXCEL + (MAXVXP / 2));
const SPAISZ: i32 = ((((IXIFIX + MAXVXP) + MXNVLS) + MAXVRT) + MAXNPV);

struct SaveVars {
    NCELL: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NCELL: i32 = 0;

        Self { NCELL }
    }
}

//$Procedure  ZZVRTPLT  ( create vertex-plate mapping )
pub fn ZZVRTPLT(
    NV: i32,
    NP: i32,
    PLATES: &[i32],
    CELLSZ: i32,
    MAXLST: i32,
    CELLS: &mut [i32],
    VRTPTR: &mut [i32],
    NLIST: &mut i32,
    PLTLST: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let PLATES = DummyArray2D::new(PLATES, 1..=3, 1..);
    let mut CELLS = DummyArrayMut2D::new(CELLS, 1..=2, 1..=CELLSZ);
    let mut VRTPTR = DummyArrayMut::new(VRTPTR, 1..);
    let mut PLTLST = DummyArrayMut::new(PLTLST, 1..);

    //
    // SPICELIB functions
    //

    //
    // Local variables.
    //

    //
    // Standard SPICELIB error handling.
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZVRTPLT", ctx)?;

    if (NV < 1) {
        SETMSG(
            b"Vertex count NV = #; count must be positive.be positive.",
            ctx,
        );
        ERRINT(b"#", NV, ctx);
        SIGERR(b"SPICE(BADVERTEXCOUNT)", ctx)?;
        CHKOUT(b"ZZVRTPLT", ctx)?;
        return Ok(());
    }

    if (NP < 1) {
        SETMSG(
            b"Plate count NP = #; count must be positive.be positive.",
            ctx,
        );
        ERRINT(b"#", NP, ctx);
        SIGERR(b"SPICE(BADPLATECOUNT)", ctx)?;
        CHKOUT(b"ZZVRTPLT", ctx)?;
        return Ok(());
    }

    if (CELLSZ < (3 * NP)) {
        SETMSG(
            b"Cell array size CELLSZ = #; size must be >= 3*NP. NP is the plate count #.",
            ctx,
        );
        ERRINT(b"#", CELLSZ, ctx);
        ERRINT(b"#", NP, ctx);
        SIGERR(b"SPICE(CELLARRAYTOOSMALL)", ctx)?;
        CHKOUT(b"ZZVRTPLT", ctx)?;
        return Ok(());
    }

    if (MAXLST < (NV + (3 * NP))) {
        SETMSG(b"Plate list array size MAXPLT = #; size must be >= 3*NP + NV, which is #. (NV = vertex count, NP = plate count.)", ctx);
        ERRINT(b"#", MAXLST, ctx);
        ERRINT(b"#", ((3 * NP) + NV), ctx);
        SIGERR(b"SPICE(PLATELISTTOOSMALL)", ctx)?;
        CHKOUT(b"ZZVRTPLT", ctx)?;
        return Ok(());
    }

    //
    // Initialize pointer and cell structure.
    //
    ZZINILNK(
        NV,
        CELLSZ,
        &mut save.NCELL,
        VRTPTR.as_slice_mut(),
        CELLS.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZVRTPLT", ctx)?;
        return Ok(());
    }

    //
    // Loop over all plate IDS. Add each plate/vertex
    // combination to the linked list.
    //
    for I in 1..=NP {
        for J in 1..=3 {
            //
            // AVAL = PLATES(J,I), vertex J of plate ID I.
            // BVAL = I, plate ID value I.
            //
            ZZADDLNK(
                PLATES[[J, I]],
                I,
                NV,
                CELLSZ,
                VRTPTR.as_slice_mut(),
                &mut save.NCELL,
                CELLS.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZVRTPLT", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // Generate two linked lists mapping vertex ID to the plates
    // including that vertex as a member.
    //
    // VRTPTR: An array, indexed by vertex ID. For an array element,
    //         VRTPTR(VERT_ID), greater than zero, the value identifies
    //         an index in PLTLST, the value of that PLTLST array
    //         element  equaling the number of plates that include
    //         the vertex specified by the ID as a member. The
    //         condition VRTPTR(VERT_ID) = -1 indicates a bug.
    //
    // PLTLST: An array, indexed by the positive entries in VRTPTR.
    //         The element, N, identified by a VRTPTR value describes
    //         the number of plates of which the vertex is a member.
    //
    //             N = PLTLST( VRTPTR(VERT_ID) )
    //
    //         The N elements following PLTLST( VRTPTR(VERT_ID) ),
    //         contain the IDs of those plates which have the vertex
    //         as a member.
    //
    ZZUNTNGL(
        NV,
        CELLSZ,
        CELLS.as_slice(),
        MAXLST,
        VRTPTR.as_slice_mut(),
        NLIST,
        PLTLST.as_slice_mut(),
        ctx,
    )?;

    //
    // Standard SPICE error handling.
    //
    CHKOUT(b"ZZVRTPLT", ctx)?;
    Ok(())
}
