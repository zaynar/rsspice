//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CNAMSZ: i32 = 32;
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
const SDSCSZ: i32 = 24;
const EKTIDX: i32 = 1;
const SNOIDX: i32 = (EKTIDX + 1);
const IMDIDX: i32 = (SNOIDX + 1);
const TNMIDX: i32 = (IMDIDX + 1);
const NCIDX: i32 = (TNMIDX + 1);
const NRIDX: i32 = (NCIDX + 1);
const RTIDX: i32 = (NRIDX + 1);
const CPTIDX: i32 = (RTIDX + 1);
const DPTIDX: i32 = (CPTIDX + 1);
const IPTIDX: i32 = (DPTIDX + 1);
const MFLIDX: i32 = (IPTIDX + 1);
const IFLIDX: i32 = (MFLIDX + 1);
const SHDIDX: i32 = (IFLIDX + 1);
const CFHIDX: i32 = (SHDIDX + 1);
const CSNIDX: i32 = (CFHIDX + 1);
const LCPIDX: i32 = (CSNIDX + 1);
const LDPIDX: i32 = (LCPIDX + 1);
const LIPIDX: i32 = (LDPIDX + 1);
const LCWIDX: i32 = (LIPIDX + 1);
const LDWIDX: i32 = (LCWIDX + 1);
const LIWIDX: i32 = (LDWIDX + 1);
const NMLIDX: i32 = (LIWIDX + 1);

//$Procedure  ZZEKCDSC ( Private: EK, return column descriptor )
pub fn ZZEKCDSC(
    HANDLE: i32,
    SEGDSC: &[i32],
    COLUMN: &[u8],
    COLDSC: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SEGDSC = DummyArray::new(SEGDSC, 1..=SDSCSZ);
    let mut COLDSC = DummyArrayMut::new(COLDSC, 1..=CDSCSZ);
    let mut CNAME = [b' '; CNAMSZ as usize];
    let mut DSCBAS: i32 = 0;
    let mut I: i32 = 0;
    let mut MBASE: i32 = 0;
    let mut NAMBAS: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    // Get the segment's integer metadata's base address.
    //
    MBASE = SEGDSC[IMDIDX];

    //
    // Get the number of columns.
    //
    NCOLS = SEGDSC[NCIDX];

    //
    // Search linearly through the column descriptors, looking for
    // a column name match.  It's an error if we don't find the input
    // name.
    //
    FOUND = false;
    I = 1;

    while ((I <= NCOLS) && !FOUND) {
        DSCBAS = ((MBASE + SDSCSZ) + ((I - 1) * CDSCSZ));
        //
        // Get the character base address of the column name from the
        // current descriptor.
        //
        DASRDI(
            HANDLE,
            (DSCBAS + 1),
            (DSCBAS + CDSCSZ),
            COLDSC.as_slice_mut(),
            ctx,
        )?;

        NAMBAS = COLDSC[NAMIDX];

        //
        // Look up the name and compare.
        //
        DASRDC(
            HANDLE,
            (NAMBAS + 1),
            (NAMBAS + CNAMSZ),
            1,
            CNAMSZ,
            CharArrayMut::from_mut(&mut CNAME),
            ctx,
        )?;

        if EQSTR(&CNAME, COLUMN) {
            FOUND = true;
        } else {
            I = (I + 1);
        }
    }

    if !FOUND {
        CHKIN(b"ZZEKCDSC", ctx)?;
        SETMSG(
            b"Descriptor for column # was not found. Segment base = #; file = #.",
            ctx,
        );
        ERRCH(b"#", COLUMN, ctx);
        ERRINT(b"#", MBASE, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKCDSC", ctx)?;
        return Ok(());
    }

    Ok(())
}
