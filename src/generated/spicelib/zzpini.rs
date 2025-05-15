//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const LBCELL: i32 = -5;
const LBPOOL: i32 = -5;
const SHRTLN: i32 = 9;
const BSLASH: i32 = 92;
const DAT: &[u8; SHRTLN as usize] = &fstr::extend_const::<{ SHRTLN as usize }>(b"begindata");
const TXT: &[u8; SHRTLN as usize] = &fstr::extend_const::<{ SHRTLN as usize }>(b"begintext");

//$Procedure      ZZPINI ( Private --- kernel pool initialization )
pub fn ZZPINI(
    FIRST: &mut bool,
    MAXVAR: i32,
    MAXVAL: i32,
    MAXLIN: i32,
    BEGDAT: &mut [u8],
    BEGTXT: &mut [u8],
    NMPOOL: &mut [i32],
    DPPOOL: &mut [i32],
    CHPOOL: &mut [i32],
    NAMLST: &mut [i32],
    DATLST: &mut [i32],
    MAXAGT: i32,
    MXNOTE: i32,
    WTVARS: CharArrayMut,
    WTPTRS: &mut [i32],
    WTPOOL: &mut [i32],
    WTAGNT: CharArrayMut,
    AGENTS: CharArrayMut,
    ACTIVE: CharArrayMut,
    NOTIFY: CharArrayMut,
    SUBCTR: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut NMPOOL = DummyArrayMut2D::new(NMPOOL, 1..=2, LBPOOL..=MAXVAR);
    let mut DPPOOL = DummyArrayMut2D::new(DPPOOL, 1..=2, LBPOOL..=MAXVAL);
    let mut CHPOOL = DummyArrayMut2D::new(CHPOOL, 1..=2, LBPOOL..=MAXLIN);
    let mut NAMLST = DummyArrayMut::new(NAMLST, 1..=MAXVAR);
    let mut DATLST = DummyArrayMut::new(DATLST, 1..=MAXVAR);
    let mut WTVARS = DummyCharArrayMut::new(WTVARS, None, LBCELL..=MAXVAR);
    let mut WTPTRS = DummyArrayMut::new(WTPTRS, 1..=MAXVAR);
    let mut WTPOOL = DummyArrayMut2D::new(WTPOOL, 1..=2, LBPOOL..=MXNOTE);
    let mut WTAGNT = DummyCharArrayMut::new(WTAGNT, None, 1..=MXNOTE);
    let mut AGENTS = DummyCharArrayMut::new(AGENTS, None, LBCELL..);
    let mut ACTIVE = DummyCharArrayMut::new(ACTIVE, None, LBCELL..);
    let mut NOTIFY = DummyCharArrayMut::new(NOTIFY, None, LBCELL..);
    let mut SUBCTR = DummyArrayMut::new(SUBCTR, 1..=CTRSIZ);
    let mut DUMMY: i32 = 0;

    //
    // SPICELIB Functions.
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if *FIRST {
        CHKIN(b"ZZPINI", ctx)?;

        for I in 1..=MAXVAR {
            NAMLST[I] = 0;
            DATLST[I] = 0;
        }

        //
        // Set up hash function. Use TOUCHI to suppress
        // compiler warnings.
        //
        DUMMY = ZZSHSH(MAXVAR, ctx)?;
        DUMMY = TOUCHI(DUMMY);

        fstr::assign(BEGDAT, &fstr::concat(&intrinsics::CHAR(BSLASH), DAT));
        fstr::assign(BEGTXT, &fstr::concat(&intrinsics::CHAR(BSLASH), TXT));

        LNKINI(MAXVAR, NMPOOL.as_slice_mut(), ctx)?;
        LNKINI(MAXVAL, DPPOOL.as_slice_mut(), ctx)?;
        LNKINI(MAXLIN, CHPOOL.as_slice_mut(), ctx)?;

        SSIZEC(MAXVAR, WTVARS.as_arg_mut(), ctx)?;
        CLEARI(MAXVAR, WTPTRS.as_slice_mut());
        LNKINI(MXNOTE, WTPOOL.as_slice_mut(), ctx)?;
        CLEARC(MXNOTE, WTAGNT.as_arg_mut());

        SSIZEC(MXNOTE, AGENTS.as_arg_mut(), ctx)?;
        SSIZEC(MXNOTE, ACTIVE.as_arg_mut(), ctx)?;
        SSIZEC(MXNOTE, NOTIFY.as_arg_mut(), ctx)?;

        ZZCTRSIN(SUBCTR.as_slice_mut(), ctx);

        if !FAILED(ctx) {
            *FIRST = false;
        }

        CHKOUT(b"ZZPINI", ctx)?;
        return Ok(());
    }

    Ok(())
}
