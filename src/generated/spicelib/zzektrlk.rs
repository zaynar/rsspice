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
const ACCLEN: i32 = 15;

struct SaveVars {
    ACCESS: Vec<u8>,
    CHILD: i32,
    DATBAS: i32,
    DEPTH: i32,
    MINUS: i32,
    NEWKEY: i32,
    OLDHAN: i32,
    OLDIDX: i32,
    OLDLVL: i32,
    OLDMAX: i32,
    OLDNOD: i32,
    OLDNOF: i32,
    OLDTRE: i32,
    OLDKEY: i32,
    OLDVAL: i32,
    PAGE: StackArray<i32, 256>,
    PLUS: i32,
    PREV: i32,
    PRVKEY: i32,
    TOTKEY: i32,
    FIRST: bool,
    FOUND: bool,
    LEAF: bool,
    SAMKEY: bool,
    SAMTRE: bool,
    RDONLY: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ACCESS = vec![b' '; ACCLEN as usize];
        let mut CHILD: i32 = 0;
        let mut DATBAS: i32 = 0;
        let mut DEPTH: i32 = 0;
        let mut MINUS: i32 = 0;
        let mut NEWKEY: i32 = 0;
        let mut OLDHAN: i32 = 0;
        let mut OLDIDX: i32 = 0;
        let mut OLDLVL: i32 = 0;
        let mut OLDMAX: i32 = 0;
        let mut OLDNOD: i32 = 0;
        let mut OLDNOF: i32 = 0;
        let mut OLDTRE: i32 = 0;
        let mut OLDKEY: i32 = 0;
        let mut OLDVAL: i32 = 0;
        let mut PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
        let mut PLUS: i32 = 0;
        let mut PREV: i32 = 0;
        let mut PRVKEY: i32 = 0;
        let mut TOTKEY: i32 = 0;
        let mut FIRST: bool = false;
        let mut FOUND: bool = false;
        let mut LEAF: bool = false;
        let mut SAMKEY: bool = false;
        let mut SAMTRE: bool = false;
        let mut RDONLY: bool = false;

        FIRST = true;
        OLDHAN = 0;
        OLDIDX = 0;
        OLDKEY = 0;
        OLDLVL = 0;
        OLDMAX = 0;
        OLDNOD = 0;
        OLDNOF = 0;
        OLDTRE = 0;
        OLDVAL = 0;
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

        Self {
            ACCESS,
            CHILD,
            DATBAS,
            DEPTH,
            MINUS,
            NEWKEY,
            OLDHAN,
            OLDIDX,
            OLDLVL,
            OLDMAX,
            OLDNOD,
            OLDNOF,
            OLDTRE,
            OLDKEY,
            OLDVAL,
            PAGE,
            PLUS,
            PREV,
            PRVKEY,
            TOTKEY,
            FIRST,
            FOUND,
            LEAF,
            SAMKEY,
            SAMTRE,
            RDONLY,
        }
    }
}

//$Procedure      ZZEKTRLK ( EK tree, locate key )
pub fn ZZEKTRLK(
    HANDLE: i32,
    TREE: i32,
    KEY: i32,
    IDX: &mut i32,
    NODE: &mut i32,
    NOFFST: &mut i32,
    LEVEL: &mut i32,
    VALUE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
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
    // Use discovery check-in in this puppy.
    //
    // Nothing found to begin with.
    //
    save.FOUND = false;

    if save.FIRST {
        //
        // Find out the access method for the current file.
        //
        DASHAM(HANDLE, &mut save.ACCESS, ctx)?;

        save.RDONLY = fstr::eq(&save.ACCESS, b"READ");
        save.SAMKEY = false;
        save.SAMTRE = false;
        save.LEAF = false;
        save.FIRST = false;
    } else {
        //
        // See whether we're looking at the same key, or at least
        // the same tree, as last time.  Note that for the tree to
        // be guaranteed to be the same, it must belong to a file open
        // for read access only.
        //
        if (HANDLE != save.OLDHAN) {
            DASHAM(HANDLE, &mut save.ACCESS, ctx)?;

            save.RDONLY = fstr::eq(&save.ACCESS, b"READ");
            save.SAMTRE = false;
            save.SAMKEY = false;
        } else {
            save.SAMTRE = ((TREE == save.OLDTRE) && save.RDONLY);
            save.SAMKEY = ((KEY == save.OLDKEY) && save.SAMTRE);
        }
    }

    //
    // If we're lucky enough to be getting a request for the previously
    // returned key, we're set.  If we've been asked for a key that is
    // very close to the previously requested key, we still may make
    // out pretty well.
    //
    if save.SAMKEY {
        //
        // It's the same key as last time.
        //
        *IDX = save.OLDIDX;
        *NODE = save.OLDNOD;
        *NOFFST = save.OLDNOF;
        *LEVEL = save.OLDLVL;
        *VALUE = save.OLDVAL;

        return Ok(());
    } else if (save.SAMTRE && save.LEAF) {
        //
        // Compute the margins around the old key.  Keys that fall within
        // the interval defined by the old key and these margins are on
        // the same page as the old key.
        //
        save.PLUS = (save.OLDMAX - save.OLDIDX);
        save.MINUS = (save.OLDIDX - 1);

        if ((KEY <= (save.OLDKEY + save.PLUS)) && (KEY >= (save.OLDKEY - save.MINUS))) {
            //
            // The requested key lies on the same page as the old key.
            //
            *LEVEL = save.OLDLVL;

            if (*LEVEL == 1) {
                save.DATBAS = TRDATR;
            } else {
                save.DATBAS = TRDATC;
            }

            *IDX = (save.OLDIDX + (KEY - save.OLDKEY));
            *NODE = save.OLDNOD;
            *NOFFST = save.OLDNOF;
            *VALUE = save.PAGE[(save.DATBAS + *IDX)];

            save.OLDIDX = *IDX;
            save.OLDKEY = KEY;
            save.OLDVAL = *VALUE;

            return Ok(());
        }
    }

    //
    // If we arrived here, we have some actual work to do.
    // Start out by looking at the root page.  Save the tree depth;
    // we'll use this for error checking.
    //
    ZZEKPGRI(HANDLE, TREE, save.PAGE.as_slice_mut(), ctx)?;

    save.DEPTH = save.PAGE[TRDPTH];
    *LEVEL = 1;

    //
    // Find out how many keys are in the tree.  If KEY is outside
    // this range, we won't find it.
    //
    save.TOTKEY = save.PAGE[TRNKEY];

    if ((KEY < 1) || (KEY > save.TOTKEY)) {
        CHKIN(b"ZZEKTRLK", ctx)?;
        SETMSG(b"Key = #; valid range = 1:#. Tree = #, file = #", ctx);
        ERRINT(b"#", KEY, ctx);
        ERRINT(b"#", save.TOTKEY, ctx);
        ERRINT(b"#", TREE, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZEKTRLK", ctx)?;
        return Ok(());
    }

    //
    // Find the last key at this level that is less than or equal to
    // the requested key.
    //
    save.PREV = LSTLEI(KEY, save.PAGE[TRNKR], save.PAGE.subarray((TRKEYR + 1)));

    if (save.PREV > 0) {
        save.PRVKEY = save.PAGE[(TRKEYR + save.PREV)];
    } else {
        save.PRVKEY = 0;
    }

    //
    // If we were lucky enough to get an exact match, set our outputs
    // and return.  The key offset in the root is zero.
    //
    if (save.PRVKEY == KEY) {
        *NOFFST = 0;
        *IDX = save.PREV;
        *NODE = TREE;
        *VALUE = save.PAGE[(TRDATR + *IDX)];

        save.OLDHAN = HANDLE;
        save.OLDTRE = TREE;
        save.OLDKEY = KEY;
        save.OLDNOF = *NOFFST;
        save.OLDNOD = *NODE;
        save.OLDIDX = *IDX;
        save.OLDLVL = *LEVEL;
        save.OLDVAL = *VALUE;
        save.OLDMAX = save.PAGE[TRNKR];

        save.LEAF = (*LEVEL == save.DEPTH);

        //
        // The root has no parent or siblings, so these values
        // remain set to zero.  The same is true of the parent keys.
        //
        return Ok(());
    }

    //
    // Still here?  Traverse the pointer path until we find the key
    // or run out of progeny.
    //
    save.CHILD = save.PAGE[((TRKIDR + save.PREV) + 1)];
    *NOFFST = save.PRVKEY;

    while ((save.CHILD > 0) && !save.FOUND) {
        //
        // Look up the child node.
        //
        ZZEKPGRI(HANDLE, save.CHILD, save.PAGE.as_slice_mut(), ctx)?;

        *LEVEL = (*LEVEL + 1);

        if (*LEVEL > save.DEPTH) {
            CHKIN(b"ZZEKTRLK", ctx)?;
            SETMSG(
                b"Runaway node pointer chain.  Key = #; valid range = 1:#. Tree = #, file = #",
                ctx,
            );
            ERRINT(b"#", KEY, ctx);
            ERRINT(b"#", save.TOTKEY, ctx);
            ERRINT(b"#", TREE, ctx);
            ERRHAN(b"#", HANDLE, ctx)?;
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZEKTRLK", ctx)?;
            return Ok(());
        }

        //
        // Find the last key at this level that is less than or equal to
        // the requested key.  Since the keys we're looking at now are
        // ordinal positions relative to the subtree whose root is the
        // current node, we must subtract from KEY the position of the
        // node preceding the first key of this subtree.
        //
        save.NEWKEY = (KEY - *NOFFST);
        save.PREV = LSTLEI(
            save.NEWKEY,
            save.PAGE[TRNKC],
            save.PAGE.subarray((TRKEYC + 1)),
        );

        if (save.PREV > 0) {
            save.PRVKEY = save.PAGE[(TRKEYC + save.PREV)];
        } else {
            save.PRVKEY = 0;
        }

        //
        // If we were lucky enough to get an exact match, set our outputs
        // and return.  The key offset for the current node is stored
        // in NOFFST.
        //
        if (save.PRVKEY == save.NEWKEY) {
            save.FOUND = true;
            *IDX = save.PREV;
            *NODE = save.CHILD;
            *VALUE = save.PAGE[(TRDATC + *IDX)];

            save.OLDHAN = HANDLE;
            save.OLDTRE = TREE;
            save.OLDKEY = KEY;
            save.OLDNOF = *NOFFST;
            save.OLDNOD = *NODE;
            save.OLDIDX = *IDX;
            save.OLDLVL = *LEVEL;
            save.OLDVAL = *VALUE;
            save.OLDMAX = save.PAGE[TRNKC];

            save.LEAF = (*LEVEL == save.DEPTH);
        } else {
            save.CHILD = save.PAGE[((TRKIDC + save.PREV) + 1)];
            *NOFFST = (save.PRVKEY + *NOFFST);
        }
    }

    //
    // If we found the key, our outputs are already set.  If not, we've
    // got trouble.
    //
    if !save.FOUND {
        CHKIN(b"ZZEKTRLK", ctx)?;
        SETMSG(b"Key #; valid range = 1:#. Tree = #, file = #.  Key was not found.  This probably indicates a corrupted file or a bug in the EK code.", ctx);
        ERRINT(b"#", KEY, ctx);
        ERRINT(b"#", save.TOTKEY, ctx);
        ERRINT(b"#", TREE, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKTRLK", ctx)?;
        return Ok(());
    }

    Ok(())
}
