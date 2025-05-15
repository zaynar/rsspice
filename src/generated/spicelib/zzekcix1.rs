//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CDOFF: i32 = 24;
const CDSCSZ: i32 = 11;
const CLSIDX: i32 = 1;
const TYPIDX: i32 = (CLSIDX + 1);
const LENIDX: i32 = (TYPIDX + 1);
const SIZIDX: i32 = (LENIDX + 1);
const NAMIDX: i32 = (SIZIDX + 1);
const IXTIDX: i32 = (NAMIDX + 1);
const IXPIDX: i32 = (IXTIDX + 1);
const NFLIDX: i32 = (IXPIDX + 1);
const ORDIDX: i32 = (NFLIDX + 1);
const METIDX: i32 = (ORDIDX + 1);

//$Procedure      ZZEKCIX1 ( EK, create index, type 1 )
pub fn ZZEKCIX1(HANDLE: i32, COLDSC: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut COLDSC = DummyArrayMut::new(COLDSC, 1..=CDSCSZ);

    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZEKCIX1", ctx)?;
    }

    //
    // Before trying to actually write anything, do every error
    // check we can.
    //
    // Is this file handle valid--is the file open for paged write
    // access?  Signal an error if not.
    //
    ZZEKPGCH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZEKCIX1", ctx)?;
        return Ok(());
    }

    //
    // An empty type 1 segment is just an empty B*-tree.  The root
    // page number of the tree serves as the index pointer.
    //
    COLDSC[IXTIDX] = 1;

    ZZEKTRIT(HANDLE, &mut COLDSC[IXPIDX], ctx)?;

    CHKOUT(b"ZZEKCIX1", ctx)?;
    Ok(())
}
