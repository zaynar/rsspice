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
const KEYLEN: i32 = 32;

struct SaveVars {
    NBC: Vec<u8>,
    NBN: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NBC = vec![b' '; KEYLEN as usize];
        let mut NBN = vec![b' '; KEYLEN as usize];

        fstr::assign(&mut NBC, b"NAIF_BODY_CODE");
        fstr::assign(&mut NBN, b"NAIF_BODY_NAME");

        Self { NBC, NBN }
    }
}

//$Procedure ZZBODKER ( Private --- Process Body-Name Kernel Pool Maps )
pub fn ZZBODKER(
    NAMES: CharArrayMut,
    NORNAM: CharArrayMut,
    CODES: &mut [i32],
    NVALS: &mut i32,
    EXTKER: &mut bool,
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
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut NAMES = DummyCharArrayMut::new(NAMES, Some(MAXL), 1..=NROOM);
    let mut NORNAM = DummyCharArrayMut::new(NORNAM, Some(MAXL), 1..=NROOM);
    let mut CODES = DummyArrayMut::new(CODES, 1..=NROOM);
    let mut BNMLST = DummyArrayMut::new(BNMLST, 1..=NROOM);
    let mut BNMPOL = DummyArrayMut::new(BNMPOL, LBPOOL..=NROOM);
    let mut BNMNMS = DummyCharArrayMut::new(BNMNMS, Some(MAXL), 1..=NROOM);
    let mut BNMIDX = DummyArrayMut::new(BNMIDX, 1..=NROOM);
    let mut BIDLST = DummyArrayMut::new(BIDLST, 1..=NROOM);
    let mut BIDPOL = DummyArrayMut::new(BIDPOL, LBPOOL..=NROOM);
    let mut BIDIDS = DummyArrayMut::new(BIDIDS, 1..=NROOM);
    let mut BIDIDX = DummyArrayMut::new(BIDIDX, 1..=NROOM);
    let mut TYPE = ActualCharArray::new(1, 1..=2);
    let mut NSIZ = StackArray::<i32, 2>::new(1..=2);
    let mut NUM = StackArray::<i32, 2>::new(1..=2);
    let mut FOUND: bool = false;
    let mut PLFIND = StackArray::<bool, 2>::new(1..=2);

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved Variables
    //

    //
    // Data Statements
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZBODKER", ctx)?;
    }

    //
    // Until the code below proves otherwise, we shall assume
    // we lack kernel pool name/code mappings.
    //
    *EXTKER = false;

    //
    // Check for the external body ID variables in the kernel pool.
    //
    GCPOOL(
        &save.NBN,
        1,
        NROOM,
        &mut NUM[1],
        NAMES.as_arg_mut(),
        &mut PLFIND[1],
        ctx,
    )?;
    GIPOOL(
        &save.NBC,
        1,
        NROOM,
        &mut NUM[2],
        CODES.as_slice_mut(),
        &mut PLFIND[2],
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZBODKER", ctx)?;
        return Ok(());
    }

    //
    // Examine PLFIND(1) and PLFIND(2) for problems.
    //
    if (PLFIND[1] != PLFIND[2]) {
        //
        // If they are not both present or absent, signal an error.
        //
        SETMSG(b"The kernel pool vector, #, used in mapping between names and ID-codes is absent, while # is not.  This is often due to an improperly constructed text kernel.  Check loaded kernels for these keywords.", ctx);

        if PLFIND[1] {
            ERRCH(b"#", &save.NBC, ctx);
            ERRCH(b"#", &save.NBN, ctx);
        } else {
            ERRCH(b"#", &save.NBN, ctx);
            ERRCH(b"#", &save.NBC, ctx);
        }

        SIGERR(b"SPICE(MISSINGKPV)", ctx)?;
        CHKOUT(b"ZZBODKER", ctx)?;
        return Ok(());
    } else if !PLFIND[1] {
        //
        // Return if both keywords are absent.
        //
        CHKOUT(b"ZZBODKER", ctx)?;
        return Ok(());
    }

    //
    // If we reach here, then both kernel pool variables are present.
    // Perform some simple sanity checks on their lengths.
    //
    DTPOOL(&save.NBN, &mut FOUND, &mut NSIZ[1], &mut TYPE[1], ctx)?;
    DTPOOL(&save.NBC, &mut FOUND, &mut NSIZ[2], &mut TYPE[2], ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZBODKER", ctx)?;
        return Ok(());
    }

    if ((NSIZ[1] > NROOM) || (NSIZ[2] > NROOM)) {
        SETMSG(b"The kernel pool vectors used to define the names/ID-codes mappingexceeds the max size. The size of the NAME vector is #1. The size of the CODE vector is #2. The max number allowed of elements is #3.", ctx);
        ERRINT(b"#1", NSIZ[1], ctx);
        ERRINT(b"#2", NSIZ[2], ctx);
        ERRINT(b"#3", NROOM, ctx);
        SIGERR(b"SPICE(KERVARTOOBIG)", ctx)?;
        CHKOUT(b"ZZBODKER", ctx)?;
        return Ok(());
    } else if (NSIZ[1] != NSIZ[2]) {
        SETMSG(b"The kernel pool vectors used for mapping between names and ID-codes are not the same size.  The size of the name vector, NAIF_BODY_NAME is #. The size of the ID-code vector, NAIF_BODY_CODE is #. You need to examine the ID-code kernel you loaded and correct the mismatch.", ctx);
        ERRINT(b"#", NSIZ[1], ctx);
        ERRINT(b"#", NSIZ[2], ctx);
        SIGERR(b"SPICE(BADDIMENSIONS)", ctx)?;
        CHKOUT(b"ZZBODKER", ctx)?;
        return Ok(());
    }

    //
    // Compute the canonical member of the equivalence class of NAMES,
    // NORNAM. This normalization compresses groups of spaces into a
    // single space, left justifies the string, and upper-cases the
    // contents.  While passing through the NAMES array, look for any
    // blank strings and signal an appropriate error.
    //
    *NVALS = NUM[1];

    for I in 1..=*NVALS {
        //
        // Check for blank strings.
        //
        if fstr::eq(NAMES.get(I), b" ") {
            SETMSG(b"An attempt to assign the code, #, to a blank string was made.  Check loaded text kernels for a blank string in the NAIF_BODY_NAME array.", ctx);
            ERRINT(b"#", I, ctx);
            SIGERR(b"SPICE(BLANKNAMEASSIGNED)", ctx)?;
            CHKOUT(b"ZZBODKER", ctx)?;
            return Ok(());
        }

        //
        // Compute the canonical member of the equivalence class.
        //
        LJUCRS(1, &NAMES[I], &mut NORNAM[I], ctx);
    }

    //
    // Populate hashes required by ZZBODTRN.
    //
    ZZBODINI(
        NAMES.as_arg(),
        NORNAM.as_arg(),
        CODES.as_slice(),
        *NVALS,
        NROOM,
        BNMLST.as_slice_mut(),
        BNMPOL.as_slice_mut(),
        BNMNMS.as_arg_mut(),
        BNMIDX.as_slice_mut(),
        BIDLST.as_slice_mut(),
        BIDPOL.as_slice_mut(),
        BIDIDS.as_slice_mut(),
        BIDIDX.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZBODKER", ctx)?;
        return Ok(());
    }

    //
    // We're on the home stretch if we make it to this point. Set EXTKER
    // to .TRUE., check out and return.
    //
    *EXTKER = true;

    CHKOUT(b"ZZBODKER", ctx)?;
    Ok(())
}
