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
const TNAMSZ: i32 = 64;

//$Procedure      ZZEKSINF ( EK, return segment information )
pub fn ZZEKSINF(
    HANDLE: i32,
    SEGNO: i32,
    TABNAM: &mut [u8],
    SEGDSC: &mut [i32],
    CNAMES: CharArrayMut,
    CDSCRS: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SEGDSC = DummyArrayMut::new(SEGDSC, 1..=SDSCSZ);
    let mut CNAMES = DummyCharArrayMut::new(CNAMES, None, 1..);
    let mut CDSCRS = DummyArrayMut2D::new(CDSCRS, 1..=CDSCSZ, 1..);
    let mut BASE: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut NSEG: i32 = 0;
    let mut P: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZEKSINF", ctx)?;
    }

    //
    // Verify that the target file is a paged DAS EK open for read
    // access, or we can't summarize the file.
    //
    ZZEKPGCH(HANDLE, b"READ", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZEKSINF", ctx)?;
        return Ok(());
    }

    //
    // Find out how many segments are in the file, so we can check
    // the index for validity.
    //
    NSEG = EKNSEG(HANDLE, ctx)?;

    if ((SEGNO < 1) || (SEGNO > NSEG)) {
        SETMSG(b"Segment index was #; valid range is 1:#", ctx);
        ERRINT(b"#", SEGNO, ctx);
        ERRINT(b"#", NSEG, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZEKSINF", ctx)?;
        return Ok(());
    }

    //
    // We're ready to proceed.  The first step is to find the
    // segment's metadata location and look up the segment descriptor.
    //
    ZZEKMLOC(HANDLE, SEGNO, &mut P, &mut BASE, ctx)?;
    DASRDI(
        HANDLE,
        (BASE + 1),
        (BASE + SDSCSZ),
        SEGDSC.as_slice_mut(),
        ctx,
    )?;

    //
    // Get the table name.  The table's base address is in the segment
    // descriptor.
    //
    DASRDC(
        HANDLE,
        (SEGDSC[TNMIDX] + 1),
        (SEGDSC[TNMIDX] + TNAMSZ),
        1,
        TNAMSZ,
        CharArrayMut::from_mut(TABNAM),
        ctx,
    )?;

    if (intrinsics::LEN(TABNAM) > TNAMSZ) {
        fstr::assign(fstr::substr_mut(TABNAM, (TNAMSZ + 1)..), b" ");
    }

    //
    // Read the column descriptors.  The first one starts at DAS
    // integer address
    //
    //    BASE + CDOFF + 1.
    //
    //
    NCOLS = SEGDSC[NCIDX];

    DASRDI(
        HANDLE,
        ((BASE + CDOFF) + 1),
        ((BASE + CDOFF) + (NCOLS * CDSCSZ)),
        CDSCRS.as_slice_mut(),
        ctx,
    )?;

    //
    // Now read the column names into the names array.
    //
    DASRDC(
        HANDLE,
        (SEGDSC[NMLIDX] + 1),
        (SEGDSC[NMLIDX] + (NCOLS * CNAMSZ)),
        1,
        CNAMSZ,
        CNAMES.as_arg_mut(),
        ctx,
    )?;

    if (intrinsics::LEN(&CNAMES[1]) > CNAMSZ) {
        for I in 1..=NCOLS {
            fstr::assign(fstr::substr_mut(CNAMES.get_mut(I), (CNAMSZ + 1)..), b" ");
        }
    }

    //
    // All output arguments are set, or else FAILED() is .TRUE.
    //

    CHKOUT(b"ZZEKSINF", ctx)?;
    Ok(())
}
