//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MXNSRF: i32 = 2000;
const SFNMLN: i32 = 36;
const CTRSIZ: i32 = 2;
const NROOM: i32 = 2003;
const LBSNGL: i32 = -5;
const SIZIDX: i32 = 0;
const FREIDX: i32 = -1;

struct SaveVars {
    KERSID: ActualArray<i32>,
    KERBID: ActualArray<i32>,
    KERNAM: ActualCharArray,
    NKVAR: i32,
    NORNAM: ActualCharArray,
    SIDHLS: ActualArray<i32>,
    SIDPOL: ActualArray<i32>,
    SIDIDX: ActualArray<i32>,
    SNMHLS: ActualArray<i32>,
    SNMPOL: ActualArray<i32>,
    SNMIDX: ActualArray<i32>,
    NSRFNM: Vec<u8>,
    SQSHNM: Vec<u8>,
    ITEMAT: i32,
    LOOKAT: i32,
    NODE: i32,
    POLCTR: StackArray<i32, 2>,
    SRFCTR: StackArray<i32, 2>,
    EXTKER: bool,
    PASS1: bool,
    LUPDTE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut KERSID = ActualArray::<i32>::new(1..=MXNSRF);
        let mut KERBID = ActualArray::<i32>::new(1..=MXNSRF);
        let mut KERNAM = ActualCharArray::new(SFNMLN, 1..=MXNSRF);
        let mut NKVAR: i32 = 0;
        let mut NORNAM = ActualCharArray::new(SFNMLN, 1..=MXNSRF);
        let mut SIDHLS = ActualArray::<i32>::new(1..=NROOM);
        let mut SIDPOL = ActualArray::<i32>::new(LBSNGL..=NROOM);
        let mut SIDIDX = ActualArray::<i32>::new(1..=NROOM);
        let mut SNMHLS = ActualArray::<i32>::new(1..=NROOM);
        let mut SNMPOL = ActualArray::<i32>::new(LBSNGL..=NROOM);
        let mut SNMIDX = ActualArray::<i32>::new(1..=NROOM);
        let mut NSRFNM = vec![b' '; SFNMLN as usize];
        let mut SQSHNM = vec![b' '; SFNMLN as usize];
        let mut ITEMAT: i32 = 0;
        let mut LOOKAT: i32 = 0;
        let mut NODE: i32 = 0;
        let mut POLCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SRFCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut EXTKER: bool = false;
        let mut PASS1: bool = false;
        let mut LUPDTE: bool = false;

        EXTKER = false;
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), CTRSIZ as usize))
                .chain([]);

            POLCTR
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), CTRSIZ as usize))
                .chain([]);

            SRFCTR
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        PASS1 = true;

        Self {
            KERSID,
            KERBID,
            KERNAM,
            NKVAR,
            NORNAM,
            SIDHLS,
            SIDPOL,
            SIDIDX,
            SNMHLS,
            SNMPOL,
            SNMIDX,
            NSRFNM,
            SQSHNM,
            ITEMAT,
            LOOKAT,
            NODE,
            POLCTR,
            SRFCTR,
            EXTKER,
            PASS1,
            LUPDTE,
        }
    }
}

//$Procedure ZZSRFTRN ( Surface name/ID mapping umbrella )
pub fn ZZSRFTRN(
    BODYID: i32,
    SRFNAM: &[u8],
    SURFID: i32,
    USRCTR: &[i32],
    FOUND: bool,
    UPDATE: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Hash control area items.
    //

    //
    // Local variables
    //

    //
    //   Data structures in this package
    //   ===============================
    //
    //   The kernel variable table
    //   -------------------------
    //
    //   This table contains file scope arrays populated with the values
    //   of the kernel variables that define the surface name/ID mapping.
    //   These arrays contain
    //
    //      - surface names
    //      - surface ID codes
    //      - bodies associated with surface name/ID pairs
    //      - an array of normalized names. These names are
    //        upper case, left-justified, and compressed so that
    //        the names contain no consecutive, embedded blanks
    //
    //
    //   The surface ID table
    //   --------------------
    //
    //   This table enables pairs of surface IDs and body IDs to be mapped
    //   to surface names. The table consists of
    //
    //      - a singly linked list pool
    //      - a list head array
    //      - a pointer array that maps pool nodes to
    //        indices in the kernel variable table
    //
    //   The pointer array maps each node belonging to a collision list in
    //   the pool to the index in the kernel table of the associated
    //   values. The kernel table values used by this mapping are
    //
    //      - the  surface ID code
    //      - the body ID code
    //      - the original surface name
    //
    //   An integer hash function is used to map each surface ID to the
    //   index in the list head array where the index of the head node for
    //   that surface ID is located.
    //
    //   The layout of the structure is:
    //
    //                                               Kernel variable table
    //                                               (only the portions
    //                                               used here are shown)
    //
    //                                                       body IDs
    //                                                          |
    // +-- integer_hash( surface ID )                           |  original
    // |                                            surface IDs |  surface
    // |                                                   |    |   names
    // |                                                   |    |    |
    // |  list heads     list pool   pointer array         |    |    |
    // |  +---------+    +--------+  +-----------+        +--+ +--+ +--+
    // |  |         |    |        |  |           |   +--->|  | |  | |  |
    // |  +---------+    +--------+  +-----------+   |    +--+ +--+ +--+
    // +->|         |-+  | ^   *  |->|           |---+    |  | |  | |  |
    //    +---------+ |  +-|---|--+  +-----------+        +--+ +--+ +--+
    //                |    |   |
    //        ...     |    |...|          ...                   ...
    //                |    |   |
    //    +---------+ |  +-|---|--+  +-----------+        +--+ +--+ +--+
    //    |         | +->| *   |  |->|           |---+ +->|  | |  | |  |
    //    +---------+    +-----|--+  +-----------+   | |  +--+ +--+ +--+
    //        ...           ...|          ...      +-|-+        ...
    //    +---------+    +-----|--+  +-----------+ | |    +--+ +--+ +--+
    //    |         |    |     v  |->|           |-+ +--->|  | |  | |  |
    //    +---------+    +--------+  +-----------+        +--+ +--+ +--+
    //
    //    ----------------------------------------        --------------
    //                     NROOM                              MXNSRF
    //
    //
    //    The diagram above is not to scale: the arrays on the left have
    //    available length NROOM, while the arrays on the right have
    //    length MXNSRF.
    //
    //    Note that the pool array is dimensioned (LBSNGL:NROOM). Elements
    //    at indices -1 and 0 contain the size and location of the
    //    first free element, respectively.
    //
    //
    //   The surface name table
    //   ----------------------
    //
    //   This table enables pairs of surface names and body IDs to be
    //   mapped to surface IDs. The structure is parallel to that of
    //   the surface ID table; it contains
    //
    //      - a singly linked list pool
    //      - a list head array
    //      - a pointer array that maps pool nodes to
    //        indices in the kernel variable table
    //
    //   The pointer array maps each node belonging to a collision list in
    //   the pool to the index in the kernel table of the associated
    //   values. The kernel table values used by this mapping are
    //
    //      - the normalized surface name
    //      - the surface ID code
    //      - the body ID code
    //
    //   An string hash function is used to map each surface name to the
    //   index in the list head array where the index of the head node for
    //   that surface ID is located.
    //
    //   The hash function is applied to the input string after it has
    //   been normalized and then had all embedded blanks compressed out.
    //   This allows the hash function terminate when it encounters the
    //   first blank in the input string, while taking into account all
    //   non-blank characters in the string. This makes it efficient while
    //   enabling it to discriminate well between strings that may have
    //   initial words in common. These compressed strings are not used
    //   for any other purpose than hashing. For detection of the correct
    //   matching elements in the kernel table, the normalized version
    //   of the input string (which may contain blanks) is used.
    //
    //   The layout of the structure is:
    //
    //
    //                                               Kernel variable table
    //                                               (only the portions
    //                                               used here are shown)
    //
    //                                                       body IDs
    //                                                          |
    // +-- string_hash(surface name)                            |
    // |                                         normalized     |  surface
    // |                                         surface names  |   IDs
    // |                                                   |    |    |
    // |                                                   |    |    |
    // |  list heads     list pool   pointer array         |    |    |
    // |  +---------+    +--------+  +-----------+        +--+ +--+ +--+
    // |  |         |    |        |  |           |   +--->|  | |  | |  |
    // |  +---------+    +--------+  +-----------+   |    +--+ +--+ +--+
    // +->|         |-+  | ^   *  |->|           |---+    |  | |  | |  |
    //    +---------+ |  +-|---|--+  +-----------+        +--+ +--+ +--+
    //                |    |   |
    //        ...     |    |...|          ...                   ...
    //                |    |   |
    //    +---------+ |  +-|---|--+  +-----------+        +--+ +--+ +--+
    //    |         | +->| *   |  |->|           |---+ +->|  | |  | |  |
    //    +---------+    +-----|--+  +-----------+   | |  +--+ +--+ +--+
    //        ...           ...|          ...      +-|-+        ...
    //    +---------+    +-----|--+  +-----------+ | |    +--+ +--+ +--+
    //    |         |    |     v  |->|           |-+ +--->|  | |  | |  |
    //    +---------+    +--------+  +-----------+        +--+ +--+ +--+
    //
    //
    //    ----------------------------------------        --------------
    //                     NROOM                              MXNSRF
    //
    //
    //    The diagram above is not to scale: the arrays on the left have
    //    available length NROOM, while the arrays on the right have
    //    length MXNSRF.
    //
    //    Note that the pool array is dimensioned (LBSNGL:NROOM). Elements
    //    at indices -1 and 0 contain the size and location of the
    //    first free element, respectively.
    //
    //
    //
    //
    //   Declarations of data structures
    //   ===============================
    //
    //      Kernel variable table
    //      =====================
    //
    //      Input names:                 KERNAM
    //      Input surface IDs:           KERSID
    //      Input body IDs:              KERBID
    //
    //
    //    Normalized names:            NRMNAM
    //
    // Each of these surface names is prefixed with an 11-character
    // string containing the associated body ID.
    //

    //
    //
    // Surface ID table
    // ================
    //
    // Surface ID list heads:       SIDHLS
    // Surface ID pool:             SIDPOL
    // Surface ID name pointers:    SIDIDX
    //
    //

    // Surface Name table
    // ==================
    //
    // Surface name list heads:     SNMHLS
    // Surface name pool:           SNMPOL
    // Surface name ID pointers:    SNMIDP
    //
    //

    //
    // Other local declarations:
    //

    //
    // POLCTR tracks the state of the kernel pool.
    // SRFCTR tracks the state of the surface mapping
    // kernel variables.
    //

    //
    //
    // Saved variables
    //

    //
    // Initial values
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSRFTRN", ctx)?;
    SETMSG(
        b"ZZSRFTRN is an umbrella routine. It should never be called directly.",
        ctx,
    );
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZSRFTRN", ctx)?;
    Ok(())
}

//$Procedure ZZSRFN2C ( Surface name to ID code mapping )
pub fn ZZSRFN2C(
    SRFNAM: &[u8],
    BODYID: i32,
    SURFID: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSRFN2C", ctx)?;

    //
    // No result has been found.
    //
    *FOUND = false;

    if save.PASS1 {
        //
        // Initialize the surface kernel variable update counter
        // and the local pool counter. Note that this routine
        // is a "subsystem" as seen by its callers and a "user"
        // with respect to the kernel pool. Hence the different
        // initializations.
        //
        ZZCTRSIN(save.SRFCTR.as_slice_mut(), ctx);
        ZZCTRUIN(save.POLCTR.as_slice_mut(), ctx);

        //
        // Initialize local data structures. The first instance of this
        // call also sets a watch on the surface mapping kernel
        // variables.
        //
        ZZSRFKER(
            save.KERNAM.as_arg_mut(),
            save.NORNAM.as_arg_mut(),
            save.KERSID.as_slice_mut(),
            save.KERBID.as_slice_mut(),
            &mut save.EXTKER,
            &mut save.NKVAR,
            save.SNMHLS.as_slice_mut(),
            save.SNMPOL.as_slice_mut(),
            save.SNMIDX.as_slice_mut(),
            save.SIDHLS.as_slice_mut(),
            save.SIDPOL.as_slice_mut(),
            save.SIDIDX.as_slice_mut(),
            ctx,
        )?;
        //
        // Sync POLCTR with the kernel pool counter.
        //
        ZZCVPOOL(
            b"ZZSRFTRN",
            save.POLCTR.as_slice_mut(),
            &mut save.LUPDTE,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZSRFN2C", ctx)?;
            return Ok(());
        }

        save.PASS1 = false;
    }

    //
    // Determine whether the data structures need to be updated
    // due to a change in the kernel pool contents.
    //
    ZZCVPOOL(
        b"ZZSRFTRN",
        save.POLCTR.as_slice_mut(),
        &mut save.LUPDTE,
        ctx,
    )?;

    if save.LUPDTE {
        //
        // Conservatively increment the ZZSRFTRN state counter in
        // expectation of successful update.
        //
        ZZCTRINC(save.SRFCTR.as_slice_mut(), ctx)?;
        //
        // Initialize local data structures.
        //
        ZZSRFKER(
            save.KERNAM.as_arg_mut(),
            save.NORNAM.as_arg_mut(),
            save.KERSID.as_slice_mut(),
            save.KERBID.as_slice_mut(),
            &mut save.EXTKER,
            &mut save.NKVAR,
            save.SNMHLS.as_slice_mut(),
            save.SNMPOL.as_slice_mut(),
            save.SNMIDX.as_slice_mut(),
            save.SIDHLS.as_slice_mut(),
            save.SIDPOL.as_slice_mut(),
            save.SIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZSRFN2C", ctx)?;
            return Ok(());
        }
    }

    //
    // No translation can be done if the surface mapping variables
    // are not in the pool.
    //
    if !save.EXTKER {
        CHKOUT(b"ZZSRFN2C", ctx)?;
        return Ok(());
    }

    //
    // Get a "normalized" copy of the input name: left-justified,
    // compressed, upper case.
    //
    LJUCRS(1, SRFNAM, &mut save.NSRFNM, ctx);

    //
    // Get a "squished" version of the above name: a version
    // containing no blanks.
    //
    CMPRSS(b" ", 0, &save.NSRFNM, &mut save.SQSHNM);

    //
    // Find the hash value of the squished input name.
    //
    save.LOOKAT = ZZHASH2(&save.SQSHNM, save.SNMPOL[SIZIDX], ctx)?;
    save.NODE = save.SNMHLS[save.LOOKAT];

    *FOUND = false;

    if (save.NODE > 0) {
        //
        // Start at the head node and check each normalized name saved
        //    for this hash value until we find a name and body ID that
        //    match or run out of items in the collision list.
        //
        while ((save.NODE > 0) && !*FOUND) {
            *FOUND = (fstr::eq(&save.NSRFNM, save.NORNAM.get(save.SNMIDX[save.NODE]))
                && (BODYID == save.KERBID[save.SNMIDX[save.NODE]]));

            save.ITEMAT = save.NODE;
            save.NODE = save.SNMPOL[save.NODE];
        }
        //
        // ITEMAT is the value of the last node checked, or
        // 0 if the list is empty.
        //
    }

    if *FOUND {
        *SURFID = save.KERSID[save.SNMIDX[save.ITEMAT]];
    }

    CHKOUT(b"ZZSRFN2C", ctx)?;
    Ok(())
}

//$Procedure ZZSRFC2N ( Surface ID code to name mapping )
pub fn ZZSRFC2N(
    SURFID: i32,
    BODYID: i32,
    SRFNAM: &mut [u8],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSRFC2N", ctx)?;

    //
    // No result has been found.
    //
    *FOUND = false;

    if save.PASS1 {
        //
        // Initialize the surface kernel variable update counter
        // and the local pool counter. Note that this routine
        // is a "subsystem" as seen by its callers and a "user"
        // with respect to the kernel pool. Hence the different
        // initializations.
        //
        ZZCTRSIN(save.SRFCTR.as_slice_mut(), ctx);
        ZZCTRUIN(save.POLCTR.as_slice_mut(), ctx);

        //
        // Initialize local data structures. The first instance of this
        // call also sets a watch on the surface mapping kernel
        // variables.
        //
        ZZSRFKER(
            save.KERNAM.as_arg_mut(),
            save.NORNAM.as_arg_mut(),
            save.KERSID.as_slice_mut(),
            save.KERBID.as_slice_mut(),
            &mut save.EXTKER,
            &mut save.NKVAR,
            save.SNMHLS.as_slice_mut(),
            save.SNMPOL.as_slice_mut(),
            save.SNMIDX.as_slice_mut(),
            save.SIDHLS.as_slice_mut(),
            save.SIDPOL.as_slice_mut(),
            save.SIDIDX.as_slice_mut(),
            ctx,
        )?;
        //
        // Sync SRFCTR with the kernel pool counter.
        //
        ZZCVPOOL(
            b"ZZSRFTRN",
            save.POLCTR.as_slice_mut(),
            &mut save.LUPDTE,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZSRFC2N", ctx)?;
            return Ok(());
        }

        save.PASS1 = false;
    }

    //
    // Determine whether the data structures need to be updated
    // due to a change in the kernel pool contents.
    //
    ZZCVPOOL(
        b"ZZSRFTRN",
        save.POLCTR.as_slice_mut(),
        &mut save.LUPDTE,
        ctx,
    )?;

    if save.LUPDTE {
        //
        // Conservatively increment the ZZSRFTRN state counter in
        // expectation of successful update.
        //
        ZZCTRINC(save.SRFCTR.as_slice_mut(), ctx)?;

        //
        // Initialize local data structures.
        //
        ZZSRFKER(
            save.KERNAM.as_arg_mut(),
            save.NORNAM.as_arg_mut(),
            save.KERSID.as_slice_mut(),
            save.KERBID.as_slice_mut(),
            &mut save.EXTKER,
            &mut save.NKVAR,
            save.SNMHLS.as_slice_mut(),
            save.SNMPOL.as_slice_mut(),
            save.SNMIDX.as_slice_mut(),
            save.SIDHLS.as_slice_mut(),
            save.SIDPOL.as_slice_mut(),
            save.SIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZSRFC2N", ctx)?;
            return Ok(());
        }
    }

    //
    // No translation can be done if the surface mapping variables
    // are not in the pool.
    //
    if !save.EXTKER {
        CHKOUT(b"ZZSRFC2N", ctx)?;
        return Ok(());
    }

    //
    // Find the hash value of the squished input name.
    //
    save.LOOKAT = ZZHASHI(SURFID, save.SIDPOL[SIZIDX], ctx)?;
    save.NODE = save.SIDHLS[save.LOOKAT];

    *FOUND = false;

    if (save.NODE > 0) {
        //
        // Start at the head node and check each normalized name saved
        //    for this hash value until we find a name and body ID that
        //    match or run out of items in the collision list.
        //
        while ((save.NODE > 0) && !*FOUND) {
            *FOUND = ((SURFID == save.KERSID[save.SIDIDX[save.NODE]])
                && (BODYID == save.KERBID[save.SIDIDX[save.NODE]]));

            save.ITEMAT = save.NODE;
            save.NODE = save.SIDPOL[save.NODE];
        }
        //
        // ITEMAT is the value of the last node checked, or
        // 0 if the list is empty.
        //
    }

    if *FOUND {
        fstr::assign(SRFNAM, save.KERNAM.get(save.SIDIDX[save.ITEMAT]));
    }

    CHKOUT(b"ZZSRFC2N", ctx)?;
    Ok(())
}

//$Procedure ZZSRFTRK ( Surface mapping tracker )
pub fn ZZSRFTRK(
    USRCTR: &mut [i32],
    UPDATE: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut USRCTR = DummyArrayMut::new(USRCTR, 1..=2);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    if save.PASS1 {
        //
        // Check in because ZZSRFKER can fail.
        //
        CHKIN(b"ZZSRFTRK", ctx)?;
        //
        // Initialize the surface kernel variable update counter
        // and the local pool counter. Note that this routine
        // is a "subsystem" as seen by its callers and a "user"
        // with respect to the kernel pool. Hence the different
        // initializations.
        //
        ZZCTRSIN(save.SRFCTR.as_slice_mut(), ctx);
        ZZCTRUIN(save.POLCTR.as_slice_mut(), ctx);

        //
        // Initialize local data structures. The first instance of this
        // call also sets a watch on the surface mapping kernel
        // variables.
        //
        ZZSRFKER(
            save.KERNAM.as_arg_mut(),
            save.NORNAM.as_arg_mut(),
            save.KERSID.as_slice_mut(),
            save.KERBID.as_slice_mut(),
            &mut save.EXTKER,
            &mut save.NKVAR,
            save.SNMHLS.as_slice_mut(),
            save.SNMPOL.as_slice_mut(),
            save.SNMIDX.as_slice_mut(),
            save.SIDHLS.as_slice_mut(),
            save.SIDPOL.as_slice_mut(),
            save.SIDIDX.as_slice_mut(),
            ctx,
        )?;
        //
        // Sync SRFCTR with the kernel pool counter.
        //
        ZZCVPOOL(
            b"ZZSRFTRN",
            save.POLCTR.as_slice_mut(),
            &mut save.LUPDTE,
            ctx,
        )?;
        //
        // Check out here since this routine doesn't check out
        // before its normal exit.
        //
        CHKOUT(b"ZZSRFTRK", ctx)?;

        if FAILED(ctx) {
            return Ok(());
        }

        save.PASS1 = false;
    }

    //
    // Check for updates to the kernel pool variables.
    //
    ZZCVPOOL(
        b"ZZSRFTRN",
        save.POLCTR.as_slice_mut(),
        &mut save.LUPDTE,
        ctx,
    )?;

    if save.LUPDTE {
        //
        // Check in because ZZSRFKER can fail.
        //
        CHKIN(b"ZZSRFTRK", ctx)?;

        //
        // Conservatively increment the ZZSRFTRN state counter in
        // expectation of successful update.
        //
        ZZCTRINC(save.SRFCTR.as_slice_mut(), ctx)?;

        //
        // Update kernel pool mapping lists and hashes.
        //
        ZZSRFKER(
            save.KERNAM.as_arg_mut(),
            save.NORNAM.as_arg_mut(),
            save.KERSID.as_slice_mut(),
            save.KERBID.as_slice_mut(),
            &mut save.EXTKER,
            &mut save.NKVAR,
            save.SNMHLS.as_slice_mut(),
            save.SNMPOL.as_slice_mut(),
            save.SNMIDX.as_slice_mut(),
            save.SIDHLS.as_slice_mut(),
            save.SIDPOL.as_slice_mut(),
            save.SIDIDX.as_slice_mut(),
            ctx,
        )?;

        CHKOUT(b"ZZSRFTRK", ctx)?;

        if FAILED(ctx) {
            return Ok(());
        }
    }

    //
    // Check the input counter against the ZZSRFTRN counter;
    // sync the user counter.
    //
    ZZCTRCHK(save.SRFCTR.as_slice(), USRCTR.as_slice_mut(), UPDATE, ctx);

    Ok(())
}
