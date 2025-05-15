//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const EPARCH: i32 = 1;
const EPNIPT: i32 = 5;
const EPPSZC: i32 = (EPARCH + 1);
const EPBASC: i32 = (EPPSZC + 1);
const EPNPC: i32 = (EPBASC + 1);
const EPNFPC: i32 = (EPNPC + 1);
const EPFPC: i32 = (EPNFPC + 1);
const EPPSZD: i32 = (EPPSZC + EPNIPT);
const EPBASD: i32 = (EPPSZD + 1);
const EPNPD: i32 = (EPBASD + 1);
const EPNFPD: i32 = (EPNPD + 1);
const EPFPD: i32 = (EPNFPD + 1);
const EPPSZI: i32 = (EPPSZD + EPNIPT);
const EPBASI: i32 = (EPPSZI + 1);
const EPNPI: i32 = (EPBASI + 1);
const EPNFPI: i32 = (EPNPI + 1);
const EPFPI: i32 = (EPNFPI + 1);
const EPMDSZ: i32 = (1 + (3 * EPNIPT));
const PGSIZC: i32 = 1024;
const PGSIZD: i32 = 128;
const PGSIZI: i32 = 256;
const PGBASC: i32 = 0;
const PGBASD: i32 = 0;
const PGBASI: i32 = 256;
const MXKIDC: i32 = 63;
const MXKEYC: i32 = (MXKIDC - 1);
const MNKIDC: i32 = (((2 * MXKIDC) + 1) / 3);
const MNKEYC: i32 = (MNKIDC - 1);
const MXKIDR: i32 = ((2 * (((2 * MXKIDC) - 2) / 3)) + 1);
const MXKEYR: i32 = (MXKIDR - 1);
const MNKIDR: i32 = 2;
const TRTYPE: i32 = 1;
const TRVERS: i32 = 1;
const TRNNOD: i32 = (TRTYPE + 1);
const TRNKEY: i32 = (TRNNOD + 1);
const TRDPTH: i32 = (TRNKEY + 1);
const TRNKR: i32 = (TRDPTH + 1);
const TRKEYR: i32 = TRNKR;
const TRKIDR: i32 = ((TRKEYR + MXKEYR) + 1);
const TRDATR: i32 = ((TRKIDR + MXKIDR) + 1);
const TRSIZR: i32 = ((TRDATR + MXKEYR) + 1);
const TRNKC: i32 = 1;
const TRKEYC: i32 = TRNKC;
const TRKIDC: i32 = ((TRKEYC + MXKEYC) + 1);
const TRDATC: i32 = ((TRKIDC + MXKIDC) + 1);
const TRSIZC: i32 = ((TRDATC + MXKEYC) + 1);
const TRMXDP: i32 = 10;
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;

struct SaveVars {
    PAGE: StackArray<i32, 256>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), PGSIZI as usize))
                .chain([]);

            PAGE.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { PAGE }
    }
}

//$Procedure      ZZEKTRIT ( EK tree, initialize )
pub fn ZZEKTRIT(HANDLE: i32, TREE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut BASE: i32 = 0;
    let mut P: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZEKTRIT", ctx)?;

    //
    // Start out by allocating a DAS integer page.  We'll write the root
    // node out to this page.
    //
    ZZEKPGAL(HANDLE, INT, &mut P, &mut BASE, ctx)?;

    save.PAGE[TRTYPE] = TRVERS;
    save.PAGE[TRNNOD] = 1;
    save.PAGE[TRNKEY] = 0;
    save.PAGE[TRNKR] = 0;
    save.PAGE[TRDPTH] = 1;

    //
    // Set all keys to zero; set all child and data pointers to null.
    //
    CLEARI(MXKEYR, save.PAGE.subarray_mut((TRKEYR + 1)));
    CLEARI(MXKEYR, save.PAGE.subarray_mut((TRDATR + 1)));
    CLEARI(MXKIDR, save.PAGE.subarray_mut((TRKIDR + 1)));

    //
    // Write out the page.
    //
    ZZEKPGWI(HANDLE, P, save.PAGE.as_slice(), ctx)?;

    //
    // The identifier we return is just the page number of the tree's
    // root.
    //
    *TREE = P;

    CHKOUT(b"ZZEKTRIT", ctx)?;
    Ok(())
}
