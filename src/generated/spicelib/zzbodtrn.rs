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
const CTRSIZ: i32 = 2;
const WDSIZE: i32 = 32;
const LBPOOL: i32 = -5;

struct SaveVars {
    DEFNAM: ActualCharArray,
    DEFNOR: ActualCharArray,
    DEFCOD: ActualArray<i32>,
    DEFSIZ: i32,
    DNMLST: ActualArray<i32>,
    DNMPOL: ActualArray<i32>,
    DNMNMS: ActualCharArray,
    DNMIDX: ActualArray<i32>,
    DIDLST: ActualArray<i32>,
    DIDPOL: ActualArray<i32>,
    DIDIDS: ActualArray<i32>,
    DIDIDX: ActualArray<i32>,
    KERNAM: ActualCharArray,
    KERNOR: ActualCharArray,
    KERCOD: ActualArray<i32>,
    KERSIZ: i32,
    KNMLST: ActualArray<i32>,
    KNMPOL: ActualArray<i32>,
    KNMNMS: ActualCharArray,
    KNMIDX: ActualArray<i32>,
    KIDLST: ActualArray<i32>,
    KIDPOL: ActualArray<i32>,
    KIDIDS: ActualArray<i32>,
    KIDIDX: ActualArray<i32>,
    SUBCTR: StackArray<i32, 2>,
    PULCTR: StackArray<i32, 2>,
    BODCHG: bool,
    EXTKER: bool,
    NODATA: bool,
    TMPNAM: Vec<u8>,
    WNAMES: ActualCharArray,
    CODIDX: i32,
    I: i32,
    INDEX: i32,
    J: i32,
    NWATCH: i32,
    FIRST: bool,
    LUPDTE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DEFNAM = ActualCharArray::new(MAXL, 1..=MAXE);
        let mut DEFNOR = ActualCharArray::new(MAXL, 1..=MAXE);
        let mut DEFCOD = ActualArray::<i32>::new(1..=MAXE);
        let mut DEFSIZ: i32 = 0;
        let mut DNMLST = ActualArray::<i32>::new(1..=MAXE);
        let mut DNMPOL = ActualArray::<i32>::new(LBPOOL..=MAXE);
        let mut DNMNMS = ActualCharArray::new(MAXL, 1..=MAXE);
        let mut DNMIDX = ActualArray::<i32>::new(1..=MAXE);
        let mut DIDLST = ActualArray::<i32>::new(1..=MAXE);
        let mut DIDPOL = ActualArray::<i32>::new(LBPOOL..=MAXE);
        let mut DIDIDS = ActualArray::<i32>::new(1..=MAXE);
        let mut DIDIDX = ActualArray::<i32>::new(1..=MAXE);
        let mut KERNAM = ActualCharArray::new(MAXL, 1..=NROOM);
        let mut KERNOR = ActualCharArray::new(MAXL, 1..=NROOM);
        let mut KERCOD = ActualArray::<i32>::new(1..=NROOM);
        let mut KERSIZ: i32 = 0;
        let mut KNMLST = ActualArray::<i32>::new(1..=NROOM);
        let mut KNMPOL = ActualArray::<i32>::new(LBPOOL..=NROOM);
        let mut KNMNMS = ActualCharArray::new(MAXL, 1..=NROOM);
        let mut KNMIDX = ActualArray::<i32>::new(1..=NROOM);
        let mut KIDLST = ActualArray::<i32>::new(1..=NROOM);
        let mut KIDPOL = ActualArray::<i32>::new(LBPOOL..=NROOM);
        let mut KIDIDS = ActualArray::<i32>::new(1..=NROOM);
        let mut KIDIDX = ActualArray::<i32>::new(1..=NROOM);
        let mut SUBCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut PULCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut BODCHG: bool = false;
        let mut EXTKER: bool = false;
        let mut NODATA: bool = false;
        let mut TMPNAM = vec![b' '; MAXL as usize];
        let mut WNAMES = ActualCharArray::new(WDSIZE, 1..=2);
        let mut CODIDX: i32 = 0;
        let mut I: i32 = 0;
        let mut INDEX: i32 = 0;
        let mut J: i32 = 0;
        let mut NWATCH: i32 = 0;
        let mut FIRST: bool = false;
        let mut LUPDTE: bool = false;

        BODCHG = false;
        FIRST = true;
        EXTKER = false;
        NODATA = true;
        NWATCH = 2;
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"NAIF_BODY_NAME"), Val::C(b"NAIF_BODY_CODE")].into_iter();
            WNAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            DEFNAM,
            DEFNOR,
            DEFCOD,
            DEFSIZ,
            DNMLST,
            DNMPOL,
            DNMNMS,
            DNMIDX,
            DIDLST,
            DIDPOL,
            DIDIDS,
            DIDIDX,
            KERNAM,
            KERNOR,
            KERCOD,
            KERSIZ,
            KNMLST,
            KNMPOL,
            KNMNMS,
            KNMIDX,
            KIDLST,
            KIDPOL,
            KIDIDS,
            KIDIDX,
            SUBCTR,
            PULCTR,
            BODCHG,
            EXTKER,
            NODATA,
            TMPNAM,
            WNAMES,
            CODIDX,
            I,
            INDEX,
            J,
            NWATCH,
            FIRST,
            LUPDTE,
        }
    }
}

//$Procedure ZZBODTRN ( Private --- Body name and code translation )
pub fn ZZBODTRN(
    NAME: &[u8],
    CODE: i32,
    FOUND: bool,
    USRCTR: &[i32],
    UPDATE: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Length of watched POOL keywords.
    //

    //
    // Lower bound of the built-in/BODDEF and kernel-defined hash
    // collision list arrays.
    //

    //
    // Local variables.
    //

    //
    // DEFNAM, DEFNOR, and DEFCOD are the name, normalized name, and ID
    // code lists storing built-in/BODDEF name-code mappings. DEFSIZ is
    // the current size of the lists
    //

    //
    // Name-based hash for built-in/BODDEF bodies. DNMLST, DNMPOL, and
    // DNMNMS provide the index in DNMIDX which stores the index of the
    // body name, normalized name, and ID in the arrays DEFNAM, DEFNOR,
    // DEFCOD.
    //

    //
    // ID-based hash for built-in/BODDEF bodies. DIDLST, DIDPOL, and
    // DIDIDS provide the index in DIDIDX which stores the index of the
    // body name, normalized name, and ID in the arrays DEFNAM, DEFNOR,
    // DEFCOD.
    //

    //
    // KERNAM, KERNOR, and KERCOD are the name, normalized name, and ID
    // code lists storing kernel POOL name-code mappings. KERSIZ is the
    // current size of the lists
    //

    //
    // Name-based hash for kernel POOL bodies. KNMLST, KNMPOL, and
    // KNMNMS provide the index in KNMIDX which stores the index of the
    // body name, normalized name, and ID in the arrays KERNAM, KERNOR,
    // KERCOD.
    //

    //
    // ID-based hash for kernel POOL bodies. KIDLST, KIDPOL, and KIDIDS
    // provide the index in KIDIDX which stores the index of the body
    // name, normalized name, and ID in the arrays KERNAM, KERNOR,
    // KERCOD.
    //

    //
    // ZZBODTRN state counter and POOL state counter.
    //

    //
    // Flag indicating whether the built-in list was altered by BODDEF
    // calls.
    //

    //
    // Flag indicating whether valid kernel POOL defined mappings
    // are present in the KERNAM, KERNOR, and KERCOD lists.
    //

    //
    // Flag indicating whether valid kernel POOL defined mappings
    // were successfully fetched from the POOL.
    //

    //
    // Other variables.
    //

    //
    // Save all variables.
    //

    //
    // Data statements.
    //

    //
    // Standard SPICELIB error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZBODTRN", ctx)?;
        SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
        CHKOUT(b"ZZBODTRN", ctx)?;
    }

    Ok(())
}

//$Procedure ZZBODN2C ( Private --- Body name to code )
pub fn ZZBODN2C(
    NAME: &[u8],
    CODE: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZBODN2C", ctx)?;
    }

    //
    // Assume we will not find the code we seek.
    //
    *FOUND = false;

    //
    // On the first pass through this entry point, initialize the
    // built-in arrays, set the kernel pool watchers, and state
    // counters.
    //
    if save.FIRST {
        //
        // Initialize counters. Set ZZBODTRN state counter, for
        // which this umbrella is the owner, to subsystem values. Set
        // POOL counter, for which this umbrella is the user, to user
        // values.
        //
        ZZCTRSIN(save.SUBCTR.as_slice_mut(), ctx);
        ZZCTRUIN(save.PULCTR.as_slice_mut(), ctx);

        //
        // Populate the initial values of the DEFNAM, DEFNOR, and DEFCOD
        // arrays from the built-in code-name list.
        //
        ZZBODGET(
            MAXE,
            save.DEFNAM.as_arg_mut(),
            save.DEFNOR.as_arg_mut(),
            save.DEFCOD.as_slice_mut(),
            &mut save.DEFSIZ,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODN2C", ctx)?;
            return Ok(());
        }

        //
        // Populate the initial built-in code-name hashes.
        //
        ZZBODINI(
            save.DEFNAM.as_arg(),
            save.DEFNOR.as_arg(),
            save.DEFCOD.as_slice(),
            save.DEFSIZ,
            MAXE,
            save.DNMLST.as_slice_mut(),
            save.DNMPOL.as_slice_mut(),
            save.DNMNMS.as_arg_mut(),
            save.DNMIDX.as_slice_mut(),
            save.DIDLST.as_slice_mut(),
            save.DIDPOL.as_slice_mut(),
            save.DIDIDS.as_slice_mut(),
            save.DIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODN2C", ctx)?;
            return Ok(());
        }

        //
        // Set up the watchers for the kernel pool name-code mapping
        // variables.
        //
        SWPOOL(b"ZZBODTRN", save.NWATCH, save.WNAMES.as_arg(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODN2C", ctx)?;
            return Ok(());
        }

        //
        // Set FIRST to .FALSE. to not repeat initialization again.
        //
        save.FIRST = false;
    }

    //
    // Check for updates to the kernel pool variables. Note: the first
    // call to ZZCVPOOL after initialization always returns .TRUE. for
    // LUPDTE.  This ensures that any initial assignments are properly
    // processed.
    //
    ZZCVPOOL(
        b"ZZBODTRN",
        save.PULCTR.as_slice_mut(),
        &mut save.LUPDTE,
        ctx,
    )?;

    if (save.LUPDTE || save.NODATA) {
        //
        // Conservatively increment the ZZBODTRN state counter
        // in expectation of successful update.
        //
        ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

        //
        // Update kernel pool mapping lists and hashes.
        //
        ZZBODKER(
            save.KERNAM.as_arg_mut(),
            save.KERNOR.as_arg_mut(),
            save.KERCOD.as_slice_mut(),
            &mut save.KERSIZ,
            &mut save.EXTKER,
            save.KNMLST.as_slice_mut(),
            save.KNMPOL.as_slice_mut(),
            save.KNMNMS.as_arg_mut(),
            save.KNMIDX.as_slice_mut(),
            save.KIDLST.as_slice_mut(),
            save.KIDPOL.as_slice_mut(),
            save.KIDIDS.as_slice_mut(),
            save.KIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            save.NODATA = true;

            CHKOUT(b"ZZBODN2C", ctx)?;
            return Ok(());
        }

        save.NODATA = false;
    }

    //
    // Normalize the input argument NAME. We will look this normalized
    // name up in the built-in and kernel pool names hashes.
    //
    LJUCRS(1, NAME, &mut save.TMPNAM, ctx);

    //
    // If necessary, first examine the contents of the kernel pool
    // name-code mapping list.
    //
    if save.EXTKER {
        //
        // Check if this name is in the kernel pool names hash.
        //
        ZZHSCCHK(
            save.KNMLST.as_slice(),
            save.KNMPOL.as_slice(),
            save.KNMNMS.as_arg(),
            &save.TMPNAM,
            &mut save.I,
            ctx,
        )?;

        if (save.I != 0) {
            *CODE = save.KERCOD[save.KNMIDX[save.I]];
            *FOUND = true;
            CHKOUT(b"ZZBODN2C", ctx)?;
            return Ok(());
        }
    }

    //
    // If we reach here, we did not find this name in the kernel pool
    // names hash. Check the built-in names hash.
    //
    ZZHSCCHK(
        save.DNMLST.as_slice(),
        save.DNMPOL.as_slice(),
        save.DNMNMS.as_arg(),
        &save.TMPNAM,
        &mut save.I,
        ctx,
    )?;

    if (save.I != 0) {
        *CODE = save.DEFCOD[save.DNMIDX[save.I]];
        *FOUND = true;
    }

    CHKOUT(b"ZZBODN2C", ctx)?;
    Ok(())
}

//$Procedure ZZBODC2N ( Private --- Body code to name )
pub fn ZZBODC2N(
    CODE: i32,
    NAME: &mut [u8],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZBODC2N", ctx)?;
    }

    //
    // Assume we will not find the name we seek.
    //
    *FOUND = false;

    //
    // On the first pass through this entry point, initialize the
    // built-in arrays, set the kernel pool watchers, and state
    // counters.
    //
    if save.FIRST {
        //
        // Initialize counters. Set ZZBODTRN state counter, for
        // which this umbrella is the owner, to subsystem values. Set
        // POOL counter, for which this umbrella is the user, to user
        // values.
        //
        ZZCTRSIN(save.SUBCTR.as_slice_mut(), ctx);
        ZZCTRUIN(save.PULCTR.as_slice_mut(), ctx);

        //
        // Populate the initial values of the DEFNAM, DEFNOR, and DEFCOD
        // arrays from the built-in code list.
        //
        ZZBODGET(
            MAXE,
            save.DEFNAM.as_arg_mut(),
            save.DEFNOR.as_arg_mut(),
            save.DEFCOD.as_slice_mut(),
            &mut save.DEFSIZ,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODC2N", ctx)?;
            return Ok(());
        }

        //
        // Populate the initial built-in code-name hashes.
        //
        ZZBODINI(
            save.DEFNAM.as_arg(),
            save.DEFNOR.as_arg(),
            save.DEFCOD.as_slice(),
            save.DEFSIZ,
            MAXE,
            save.DNMLST.as_slice_mut(),
            save.DNMPOL.as_slice_mut(),
            save.DNMNMS.as_arg_mut(),
            save.DNMIDX.as_slice_mut(),
            save.DIDLST.as_slice_mut(),
            save.DIDPOL.as_slice_mut(),
            save.DIDIDS.as_slice_mut(),
            save.DIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODC2N", ctx)?;
            return Ok(());
        }

        //
        // Set up the watchers for the kernel pool name-code mapping
        // variables.
        //
        SWPOOL(b"ZZBODTRN", save.NWATCH, save.WNAMES.as_arg(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODC2N", ctx)?;
            return Ok(());
        }

        //
        // Set FIRST to .FALSE. to not repeat initialization again.
        //
        save.FIRST = false;
    }

    //
    // Check for updates to the kernel pool variables. Note: the first
    // call to ZZCVPOOL after initialization always returns .TRUE. for
    // LUPDTE. This ensures that any initial assignments are properly
    // processed.
    //
    ZZCVPOOL(
        b"ZZBODTRN",
        save.PULCTR.as_slice_mut(),
        &mut save.LUPDTE,
        ctx,
    )?;

    if (save.LUPDTE || save.NODATA) {
        //
        // Conservatively increment the ZZBODTRN state counter
        // in expectation of successful update.
        //
        ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

        //
        // Update kernel pool mapping lists and hashes.
        //
        ZZBODKER(
            save.KERNAM.as_arg_mut(),
            save.KERNOR.as_arg_mut(),
            save.KERCOD.as_slice_mut(),
            &mut save.KERSIZ,
            &mut save.EXTKER,
            save.KNMLST.as_slice_mut(),
            save.KNMPOL.as_slice_mut(),
            save.KNMNMS.as_arg_mut(),
            save.KNMIDX.as_slice_mut(),
            save.KIDLST.as_slice_mut(),
            save.KIDPOL.as_slice_mut(),
            save.KIDIDS.as_slice_mut(),
            save.KIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            save.NODATA = true;

            CHKOUT(b"ZZBODC2N", ctx)?;
            return Ok(());
        }

        save.NODATA = false;
    }

    //
    // If necessary, first examine the contents of the kernel pool
    // name-code mapping list.
    //
    if save.EXTKER {
        //
        // Check if this code is in the kernel pool codes hash.
        //
        ZZHSICHK(
            save.KIDLST.as_slice(),
            save.KIDPOL.as_slice(),
            save.KIDIDS.as_slice(),
            CODE,
            &mut save.I,
            ctx,
        )?;

        if (save.I != 0) {
            fstr::assign(NAME, save.KERNAM.get(save.KIDIDX[save.I]));
            *FOUND = true;
            CHKOUT(b"ZZBODC2N", ctx)?;
            return Ok(());
        }
    }

    //
    // If we reach here, we did not find this code in the kernel pool
    // codes hash. Check the built-in codes hash.
    //
    ZZHSICHK(
        save.DIDLST.as_slice(),
        save.DIDPOL.as_slice(),
        save.DIDIDS.as_slice(),
        CODE,
        &mut save.I,
        ctx,
    )?;

    //
    // If we find a match, verify that it is not masked by a kernel pool
    // entry before returning.
    //
    if (save.I != 0) {
        if save.EXTKER {
            //
            // Only bother performing this check if there are actually
            // mappings present in the kernel pool lists.
            //
            ZZHSCCHK(
                save.KNMLST.as_slice(),
                save.KNMPOL.as_slice(),
                save.KNMNMS.as_arg(),
                &save.DEFNOR[save.DIDIDX[save.I]],
                &mut save.J,
                ctx,
            )?;

            if (save.J != 0) {
                //
                // This name is defined in the kernel pool mappings. Set
                // FOUND to .FALSE., as the contents of the kernel pool
                // have higher precedence than any entries in the built-in
                // mapping list.
                //
                *FOUND = false;
            } else {
                //
                // No match for this name in the kernel pool mapping list.
                // Return the name.
                //
                fstr::assign(NAME, save.DEFNAM.get(save.DIDIDX[save.I]));
                *FOUND = true;
            }
        } else {
            //
            // No kernel pool mappings were defined, simply return
            // return the name.
            //
            fstr::assign(NAME, save.DEFNAM.get(save.DIDIDX[save.I]));
            *FOUND = true;
        }
    }

    CHKOUT(b"ZZBODC2N", ctx)?;
    Ok(())
}

//$Procedure ZZBODDEF ( Private --- Body name/code definition )
pub fn ZZBODDEF(NAME: &[u8], CODE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZBODDEF", ctx)?;
    }

    //
    // On the first pass through this entry point, initialize the
    // built-in arrays, set the kernel pool watchers, and state
    // counters.
    //
    if save.FIRST {
        //
        // Initialize counters. Set ZZBODTRN state counter, for
        // which this umbrella is the owner, to subsystem values. Set
        // POOL counter, for which this umbrella is the user, to user
        // values.
        //
        ZZCTRSIN(save.SUBCTR.as_slice_mut(), ctx);
        ZZCTRUIN(save.PULCTR.as_slice_mut(), ctx);

        //
        // Populate the initial values of the DEFNAM, DEFNOR, and DEFCOD
        // arrays from the built-in code list.
        //
        ZZBODGET(
            MAXE,
            save.DEFNAM.as_arg_mut(),
            save.DEFNOR.as_arg_mut(),
            save.DEFCOD.as_slice_mut(),
            &mut save.DEFSIZ,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODDEF", ctx)?;
            return Ok(());
        }

        //
        // Populate the initial built-in code-name hashes.
        //
        ZZBODINI(
            save.DEFNAM.as_arg(),
            save.DEFNOR.as_arg(),
            save.DEFCOD.as_slice(),
            save.DEFSIZ,
            MAXE,
            save.DNMLST.as_slice_mut(),
            save.DNMPOL.as_slice_mut(),
            save.DNMNMS.as_arg_mut(),
            save.DNMIDX.as_slice_mut(),
            save.DIDLST.as_slice_mut(),
            save.DIDPOL.as_slice_mut(),
            save.DIDIDS.as_slice_mut(),
            save.DIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODDEF", ctx)?;
            return Ok(());
        }

        //
        // Set up the watchers for the kernel pool name-code mapping
        // variables.
        //
        SWPOOL(b"ZZBODTRN", save.NWATCH, save.WNAMES.as_arg(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODDEF", ctx)?;
            return Ok(());
        }

        //
        // Set FIRST to .FALSE. to not repeat initialization again.
        //
        save.FIRST = false;
    }

    //
    // Begin by verifying that the user is not attempting to assign a
    // blank string a code.
    //
    if fstr::eq(NAME, b" ") {
        SETMSG(b"An attempt to assign the code, #, to a blank string was made.  Check loaded text kernels for a blank string in the NAIF_BODY_NAME array.", ctx);
        ERRINT(b"#", save.I, ctx);
        SIGERR(b"SPICE(BLANKNAMEASSIGNED)", ctx)?;
        CHKOUT(b"ZZBODDEF", ctx)?;
        return Ok(());
    }

    //
    // Conservatively increment the ZZBODTRN state counter in
    // expectation of successful addition.
    //
    ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

    //
    // Get normalized form of the input NAME.
    //
    LJUCRS(1, NAME, &mut save.TMPNAM, ctx);

    //
    // Determine if we are going to replace an entry currently present
    // in the DEF* lists.
    //
    ZZHSCCHK(
        save.DNMLST.as_slice(),
        save.DNMPOL.as_slice(),
        save.DNMNMS.as_arg(),
        &save.TMPNAM,
        &mut save.I,
        ctx,
    )?;

    if (save.I != 0) {
        save.INDEX = save.DNMIDX[save.I];

        //
        // We are going to replace an existing entry.  There are
        // two possible ways in which a replace operation can
        // happen:
        //
        //    1) The caller is attempting to replace the highest
        //       precedent name-code mapping for a particular ID code.
        //       When this happens, we need only change the entry in
        //       DEFNAM at position INDEX. The user is simply changing
        //       the name that maps to the same normalized name.
        //
        //    2) The caller is attempting to change the code
        //       associated with a name, bump a lower precedence
        //       name-code mapping to highest precedence, or some
        //       combination of the two.
        //
        // See if we should handle 1) first.
        //
        ZZHSICHK(
            save.DIDLST.as_slice(),
            save.DIDPOL.as_slice(),
            save.DIDIDS.as_slice(),
            CODE,
            &mut save.I,
            ctx,
        )?;

        if (save.I != 0) {
            save.CODIDX = save.DIDIDX[save.I];
        } else {
            save.CODIDX = 0;
        }

        //
        // If CODIDX matches INDEX, then we simply have to replace the
        // entry in DEFNAM and return.
        //
        if (save.CODIDX == save.INDEX) {
            //
            // We altered the built-in body list. Set BODCHG to .TRUE.
            //
            save.BODCHG = true;

            fstr::assign(save.DEFNAM.get_mut(save.INDEX), NAME);

            CHKOUT(b"ZZBODDEF", ctx)?;
            return Ok(());
        }

        //
        // At this point we have to replace all of the values for the
        // mapping defined at the INDEX position in DEFNAM, DEFNOR, and
        // DEFCOD. This will require recomputing the hashes. First
        // compress out the existing entry.
        //
        {
            let m1__: i32 = (save.INDEX + 1);
            let m2__: i32 = save.DEFSIZ;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                let val = save.DEFNAM.get(save.I).to_vec();
                fstr::assign(save.DEFNAM.get_mut((save.I - 1)), &val);
                let val = save.DEFNOR.get(save.I).to_vec();
                fstr::assign(save.DEFNOR.get_mut((save.I - 1)), &val);
                save.DEFCOD[(save.I - 1)] = save.DEFCOD[save.I];

                save.I += m3__;
            }
        }
    } else {
        //
        // We need to add this entry to the list.  See if there
        // is room; signal an error and return if there is not.
        //
        if (save.DEFSIZ >= MAXE) {
            SETMSG(b"There is no room available for adding \'#\'  to the list of name/code pairs. The number of names that can be supported is #.  This number has been reached. ", ctx);
            ERRCH(b"#", NAME, ctx);
            ERRINT(b"#", save.DEFSIZ, ctx);
            SIGERR(b"SPICE(TOOMANYPAIRS)", ctx)?;
            CHKOUT(b"ZZBODDEF", ctx)?;
            return Ok(());
        }

        //
        // If we reach here, then there is room in the list. Increase
        // it's size counter.
        //
        save.DEFSIZ = (save.DEFSIZ + 1);
    }

    //
    // We are changing the body list, inform ZZBODRST by setting BODCHG
    // to .TRUE.
    //
    save.BODCHG = true;

    //
    // Now, we need to add the new entry on to the end of the
    // DEFNAM, DEFNOR, and DEFCOD lists.
    //
    fstr::assign(save.DEFNAM.get_mut(save.DEFSIZ), NAME);
    fstr::assign(save.DEFNOR.get_mut(save.DEFSIZ), &save.TMPNAM);
    save.DEFCOD[save.DEFSIZ] = CODE;

    //
    // Reset the built-in/BODDEF hashes.
    //
    ZZBODINI(
        save.DEFNAM.as_arg(),
        save.DEFNOR.as_arg(),
        save.DEFCOD.as_slice(),
        save.DEFSIZ,
        MAXE,
        save.DNMLST.as_slice_mut(),
        save.DNMPOL.as_slice_mut(),
        save.DNMNMS.as_arg_mut(),
        save.DNMIDX.as_slice_mut(),
        save.DIDLST.as_slice_mut(),
        save.DIDPOL.as_slice_mut(),
        save.DIDIDS.as_slice_mut(),
        save.DIDIDX.as_slice_mut(),
        ctx,
    )?;

    CHKOUT(b"ZZBODDEF", ctx)?;
    Ok(())
}

//$Procedure ZZBODKIK ( Private --- Run the kernel read block )
pub fn ZZBODKIK(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZBODKIK", ctx)?;
    }

    //
    // On the first pass through this entry point, initialize the
    // built-in arrays, set the kernel pool watchers, and state
    // counters.
    //
    if save.FIRST {
        //
        // Initialize counters. Set ZZBODTRN state counter, for
        // which this umbrella is the owner, to subsystem values. Set
        // POOL counter, for which this umbrella is the user, to user
        // values.
        //
        ZZCTRSIN(save.SUBCTR.as_slice_mut(), ctx);
        ZZCTRUIN(save.PULCTR.as_slice_mut(), ctx);

        //
        // Populate the initial values of the DEFNAM, DEFNOR, and DEFCOD
        // arrays from the built-in code list.
        //
        ZZBODGET(
            MAXE,
            save.DEFNAM.as_arg_mut(),
            save.DEFNOR.as_arg_mut(),
            save.DEFCOD.as_slice_mut(),
            &mut save.DEFSIZ,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODKIK", ctx)?;
            return Ok(());
        }

        //
        // Populate the initial built-in/BODDEF hashes.
        //
        ZZBODINI(
            save.DEFNAM.as_arg(),
            save.DEFNOR.as_arg(),
            save.DEFCOD.as_slice(),
            save.DEFSIZ,
            MAXE,
            save.DNMLST.as_slice_mut(),
            save.DNMPOL.as_slice_mut(),
            save.DNMNMS.as_arg_mut(),
            save.DNMIDX.as_slice_mut(),
            save.DIDLST.as_slice_mut(),
            save.DIDPOL.as_slice_mut(),
            save.DIDIDS.as_slice_mut(),
            save.DIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODKIK", ctx)?;
            return Ok(());
        }

        //
        // Set up the watchers for the kernel pool name-code mapping
        // variables.
        //
        SWPOOL(b"ZZBODTRN", save.NWATCH, save.WNAMES.as_arg(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODKIK", ctx)?;
            return Ok(());
        }

        //
        // Set FIRST to .FALSE. to not repeat initialization again.
        //
        save.FIRST = false;
    }

    //
    // Check for updates to the kernel pool variables. Note: the first
    // call to ZZCVPOOL after initialization always returns .TRUE. for
    // LUPDTE. This ensures that any initial assignments are properly
    // processed.
    //
    ZZCVPOOL(
        b"ZZBODTRN",
        save.PULCTR.as_slice_mut(),
        &mut save.LUPDTE,
        ctx,
    )?;

    if (save.LUPDTE || save.NODATA) {
        //
        // Conservatively increment the ZZBODTRN state counter
        // in expectation of successful update.
        //
        ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

        //
        // Update kernel pool mapping lists and hashes.
        //
        ZZBODKER(
            save.KERNAM.as_arg_mut(),
            save.KERNOR.as_arg_mut(),
            save.KERCOD.as_slice_mut(),
            &mut save.KERSIZ,
            &mut save.EXTKER,
            save.KNMLST.as_slice_mut(),
            save.KNMPOL.as_slice_mut(),
            save.KNMNMS.as_arg_mut(),
            save.KNMIDX.as_slice_mut(),
            save.KIDLST.as_slice_mut(),
            save.KIDPOL.as_slice_mut(),
            save.KIDIDS.as_slice_mut(),
            save.KIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            save.NODATA = true;

            CHKOUT(b"ZZBODKIK", ctx)?;
            return Ok(());
        }

        save.NODATA = false;
    }

    CHKOUT(b"ZZBODKIK", ctx)?;
    Ok(())
}

//$Procedure ZZBODRST ( Private --- Body List Reset )
pub fn ZZBODRST(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZBODRST", ctx)?;
    }

    //
    // On the first pass through this entry point, initialize the
    // built-in arrays, set the kernel pool watchers, and state
    // counters.
    //
    if save.FIRST {
        //
        // Initialize counters. Set ZZBODTRN state counter, for
        // which this umbrella is the owner, to subsystem values. Set
        // POOL counter, for which this umbrella is the user, to user
        // values.
        //
        ZZCTRSIN(save.SUBCTR.as_slice_mut(), ctx);
        ZZCTRUIN(save.PULCTR.as_slice_mut(), ctx);

        //
        // Populate the initial values of the DEFNAM, DEFNOR, and DEFCOD
        // arrays from the built-in code list.
        //
        ZZBODGET(
            MAXE,
            save.DEFNAM.as_arg_mut(),
            save.DEFNOR.as_arg_mut(),
            save.DEFCOD.as_slice_mut(),
            &mut save.DEFSIZ,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODRST", ctx)?;
            return Ok(());
        }

        //
        // Populate the initial built-in code-name hashes.
        //
        ZZBODINI(
            save.DEFNAM.as_arg(),
            save.DEFNOR.as_arg(),
            save.DEFCOD.as_slice(),
            save.DEFSIZ,
            MAXE,
            save.DNMLST.as_slice_mut(),
            save.DNMPOL.as_slice_mut(),
            save.DNMNMS.as_arg_mut(),
            save.DNMIDX.as_slice_mut(),
            save.DIDLST.as_slice_mut(),
            save.DIDPOL.as_slice_mut(),
            save.DIDIDS.as_slice_mut(),
            save.DIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODRST", ctx)?;
            return Ok(());
        }

        //
        // Set up the watchers for the kernel pool name-code mapping
        // variables.
        //
        SWPOOL(b"ZZBODTRN", save.NWATCH, save.WNAMES.as_arg(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZBODRST", ctx)?;
            return Ok(());
        }

        //
        // Set FIRST to .FALSE. to not repeat initialization again.
        //
        save.FIRST = false;
    }

    //
    // See if the body list needs to be reset.
    //
    if save.BODCHG {
        save.BODCHG = false;

        //
        // Conservatively increment the ZZBODTRN state counter
        // in expectation of successful update.
        //
        ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

        //
        // Fetch the initial body name-code mapping list. Note: we need
        // not check FAILED() here, because if an error had occurred due
        // to the improper specification of MAXE it would have been
        // signaled already to the user.
        //
        ZZBODGET(
            MAXE,
            save.DEFNAM.as_arg_mut(),
            save.DEFNOR.as_arg_mut(),
            save.DEFCOD.as_slice_mut(),
            &mut save.DEFSIZ,
            ctx,
        )?;

        //
        // Reset the built-in/BODDEF hashes.
        //
        ZZBODINI(
            save.DEFNAM.as_arg(),
            save.DEFNOR.as_arg(),
            save.DEFCOD.as_slice(),
            save.DEFSIZ,
            MAXE,
            save.DNMLST.as_slice_mut(),
            save.DNMPOL.as_slice_mut(),
            save.DNMNMS.as_arg_mut(),
            save.DNMIDX.as_slice_mut(),
            save.DIDLST.as_slice_mut(),
            save.DIDPOL.as_slice_mut(),
            save.DIDIDS.as_slice_mut(),
            save.DIDIDX.as_slice_mut(),
            ctx,
        )?;
    }

    CHKOUT(b"ZZBODRST", ctx)?;
    Ok(())
}

//$Procedure ZZBCTRCK ( Private -- check/update user's ZZBODTRN counter )
pub fn ZZBCTRCK(
    USRCTR: &mut [i32],
    UPDATE: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut USRCTR = DummyArrayMut::new(USRCTR, 1..=CTRSIZ);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Check for updates to the kernel pool variables.
    //
    ZZCVPOOL(
        b"ZZBODTRN",
        save.PULCTR.as_slice_mut(),
        &mut save.LUPDTE,
        ctx,
    )?;

    if (save.LUPDTE || save.NODATA) {
        //
        // Check in because ZZBODKER can fail.
        //
        CHKIN(b"ZZBCTRCK", ctx)?;

        //
        // Conservatively increment the ZZBODTRN state counter in
        // expectation of successful update.
        //
        ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

        //
        // Update kernel pool mapping lists and hashes.
        //
        ZZBODKER(
            save.KERNAM.as_arg_mut(),
            save.KERNOR.as_arg_mut(),
            save.KERCOD.as_slice_mut(),
            &mut save.KERSIZ,
            &mut save.EXTKER,
            save.KNMLST.as_slice_mut(),
            save.KNMPOL.as_slice_mut(),
            save.KNMNMS.as_arg_mut(),
            save.KNMIDX.as_slice_mut(),
            save.KIDLST.as_slice_mut(),
            save.KIDPOL.as_slice_mut(),
            save.KIDIDS.as_slice_mut(),
            save.KIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            save.NODATA = true;

            CHKOUT(b"ZZBCTRCK", ctx)?;
            return Ok(());
        }

        save.NODATA = false;

        CHKOUT(b"ZZBCTRCK", ctx)?;
    }

    //
    // Check the input counter against the ZZBODTRN counter.
    //
    ZZCTRCHK(save.SUBCTR.as_slice(), USRCTR.as_slice_mut(), UPDATE, ctx);

    Ok(())
}
