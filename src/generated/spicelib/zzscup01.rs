//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const NDELIM: i32 = 5;
const DELIMS: &[u8; NDELIM as usize] = &fstr::extend_const::<{ NDELIM as usize }>(b".:-, ");
const TDB: i32 = 1;
const TDT: i32 = 2;
const MXPART: i32 = 9999;
const MXCOEF: i32 = 100000;
const MXNFLD: i32 = 10;
const DPLEN: i32 = 30;
const IXNFLD: i32 = 1;
const IXDLIM: i32 = (IXNFLD + 1);
const IXTSYS: i32 = (IXDLIM + 1);
const IXNCOF: i32 = (IXTSYS + 1);
const IXNPRT: i32 = (IXNCOF + 1);
const IXBCOF: i32 = (IXNPRT + 1);
const IXBSTR: i32 = (IXBCOF + 1);
const IXBEND: i32 = (IXBSTR + 1);
const IXBMOD: i32 = (IXBEND + 1);
const IXBOFF: i32 = (IXBMOD + 1);
const NIVALS: i32 = IXBOFF;
const MXNCLK: i32 = 100;
const DBUFSZ: i32 = (((3 * MXCOEF) + (2 * MXPART)) + (2 * MXNFLD));
const IBUFSZ: i32 = (NIVALS * MXNCLK);
const LBSNGL: i32 = -5;
const CPLSIZ: i32 = ((MXNCLK - LBSNGL) + 1);

//$Procedure ZZSCUP01 ( Private, SCLK database update, type 01 )
pub fn ZZSCUP01(
    SC: i32,
    POLCTR: &mut [i32],
    HDSCLK: &mut [i32],
    SCPOOL: &mut [i32],
    CLKLST: &mut [i32],
    DPFREE: &mut i32,
    DPBUFF: &mut [f64],
    IFREE: &mut i32,
    INTBUF: &mut [i32],
    SCBASE: &mut [i32],
    PRVSC: &mut i32,
    NFIELD: &mut i32,
    DELCDE: &mut i32,
    TIMSYS: &mut i32,
    NCOEFF: &mut i32,
    NPART: &mut i32,
    COFBAS: &mut i32,
    STRBAS: &mut i32,
    ENDBAS: &mut i32,
    MODBAS: &mut i32,
    OFFBAS: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut POLCTR = DummyArrayMut::new(POLCTR, 1..=CTRSIZ);
    let mut HDSCLK = DummyArrayMut::new(HDSCLK, 1..);
    let mut SCPOOL = DummyArrayMut::new(SCPOOL, LBSNGL..);
    let mut CLKLST = DummyArrayMut::new(CLKLST, 1..);
    let mut DPBUFF = DummyArrayMut::new(DPBUFF, 1..);
    let mut INTBUF = DummyArrayMut::new(INTBUF, 1..);
    let mut SCBASE = DummyArrayMut::new(SCBASE, 1..);
    let mut IBASE: i32 = 0;
    let mut SCLKAT: i32 = 0;
    let mut SAMCLK: bool = false;
    let mut UPDATE: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSCUP01", ctx)?;
    //
    // Reinitialize local database if the kernel pool has been
    // updated. The call below syncs POLCTR.
    //
    ZZPCTRCK(POLCTR.as_slice_mut(), &mut UPDATE, ctx);

    if UPDATE {
        //
        // Initialize local data structures.
        //
        ZZSCIN01(
            HDSCLK.as_slice_mut(),
            SCPOOL.as_slice_mut(),
            CLKLST.as_slice_mut(),
            DPFREE,
            IFREE,
            PRVSC,
            ctx,
        )?;

        SAMCLK = false;
    } else {
        SAMCLK = ((SC != 0) && (SC == *PRVSC));
    }

    //
    // Set the output arguments only if the clock has changed or if
    // a kernel pool update has occurred.
    //
    if !SAMCLK {
        //
        // Look up the input clock. If the clock's information is not
        // buffered, try to look up the information from the kernel pool.
        //
        ZZHSICHK(
            HDSCLK.as_slice(),
            SCPOOL.as_slice(),
            CLKLST.as_slice(),
            SC,
            &mut SCLKAT,
            ctx,
        )?;

        if (SCLKAT == 0) {
            //
            // This clock is unknown to the SC01 subsystem. Try to look up
            // the clock's specification from the kernel pool.
            //
            ZZSCAD01(
                SC,
                HDSCLK.as_slice_mut(),
                SCPOOL.as_slice_mut(),
                CLKLST.as_slice_mut(),
                DPFREE,
                DPBUFF.as_slice_mut(),
                IFREE,
                INTBUF.as_slice_mut(),
                SCBASE.as_slice_mut(),
                &mut SCLKAT,
                ctx,
            )?;

            if FAILED(ctx) {
                //
                // ZZSCAD01 will have initialized the type 01 SCLK
                // database; there's no need to do it again. Initialize
                // the SCLK integer parameters.
                //
                *NFIELD = 0;
                *DELCDE = 0;
                *TIMSYS = 0;
                *NCOEFF = 0;
                *NPART = 0;

                //
                // Indicate valid data for the previous were not obtained.
                //
                *PRVSC = 0;

                CHKOUT(b"ZZSCUP01", ctx)?;
                return Ok(());
            }
        }

        //
        // IBASE is the base address in the integer buffer of the integer
        // data for this clock.
        //
        IBASE = SCBASE[SCLKAT];

        //
        // Set integer SCLK parameters to the correct values for
        // the clock designated by SC.
        //
        *NFIELD = INTBUF[(IBASE + IXNFLD)];
        *DELCDE = INTBUF[(IBASE + IXDLIM)];
        *TIMSYS = INTBUF[(IBASE + IXTSYS)];
        *NCOEFF = INTBUF[(IBASE + IXNCOF)];
        *NPART = INTBUF[(IBASE + IXNPRT)];
        *COFBAS = INTBUF[(IBASE + IXBCOF)];
        *STRBAS = INTBUF[(IBASE + IXBSTR)];
        *ENDBAS = INTBUF[(IBASE + IXBEND)];
        *MODBAS = INTBUF[(IBASE + IXBMOD)];
        *OFFBAS = INTBUF[(IBASE + IXBOFF)];
    }

    CHKOUT(b"ZZSCUP01", ctx)?;
    Ok(())
}
