//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MXNSRF: i32 = 2000;
const SFNMLN: i32 = 36;
const NROOM: i32 = 2003;
const LBSNGL: i32 = -5;
const KVSFBD: &[u8] = b"NAIF_SURFACE_BODY";
const KVSFCD: &[u8] = b"NAIF_SURFACE_CODE";
const KVSFNM: &[u8] = b"NAIF_SURFACE_NAME";
const KVNMLN: i32 = 32;
const NNAMES: i32 = 3;

struct SaveVars {
    NAMES: ActualCharArray,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NAMES = ActualCharArray::new(KVNMLN, 1..=NNAMES);
        let mut PASS1: bool = false;

        PASS1 = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(KVSFBD), Val::C(KVSFCD), Val::C(KVSFNM)].into_iter();
            NAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { NAMES, PASS1 }
    }
}

//$Procedure ZZSRFKER ( Surface translation, process kernel update )
pub fn ZZSRFKER(
    KERNAM: CharArrayMut,
    NORNAM: CharArrayMut,
    KERSID: &mut [i32],
    KERBID: &mut [i32],
    EXTKER: &mut bool,
    NKVAR: &mut i32,
    SNMHLS: &mut [i32],
    SNMPOL: &mut [i32],
    SNMIDX: &mut [i32],
    SIDHLS: &mut [i32],
    SIDPOL: &mut [i32],
    SIDIDX: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut KERNAM = DummyCharArrayMut::new(KERNAM, Some(SFNMLN), 1..);
    let mut NORNAM = DummyCharArrayMut::new(NORNAM, Some(SFNMLN), 1..);
    let mut KERSID = DummyArrayMut::new(KERSID, 1..);
    let mut KERBID = DummyArrayMut::new(KERBID, 1..);
    let mut SNMHLS = DummyArrayMut::new(SNMHLS, 1..);
    let mut SNMPOL = DummyArrayMut::new(SNMPOL, LBSNGL..);
    let mut SNMIDX = DummyArrayMut::new(SNMIDX, 1..);
    let mut SIDHLS = DummyArrayMut::new(SIDHLS, 1..);
    let mut SIDPOL = DummyArrayMut::new(SIDPOL, LBSNGL..);
    let mut SIDIDX = DummyArrayMut::new(SIDIDX, 1..);
    let mut BDTYPE = [b' '; 1 as usize];
    let mut CDTYPE = [b' '; 1 as usize];
    let mut NDTYPE = [b' '; 1 as usize];
    let mut NBODY: i32 = 0;
    let mut NCODE: i32 = 0;
    let mut NNAME: i32 = 0;
    let mut FNDBOD: bool = false;
    let mut FNDCDE: bool = false;
    let mut FNDNAM: bool = false;

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

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSRFKER", ctx)?;

    //
    // The primary functions performed inline in this routine are
    //
    //    - Setting a watch on the mapping kernel variables
    //
    //    - Fetching the mapping kernel variables' values
    //
    //    - Performing error checks on the kernel variables that define
    //      the surface name/ID mapping. Initialization of data
    //      structures is delegated to ZZSRFINI.
    //
    //
    if save.PASS1 {
        //
        // Set watch on kernel variables used for the surface mapping.
        //
        SWPOOL(b"ZZSRFTRN", NNAMES, save.NAMES.as_arg(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZSRFKER", ctx)?;
            return Ok(());
        }

        save.PASS1 = false;
    }

    //
    // Indicate that no data are available until we find out
    // otherwise.
    //
    *EXTKER = false;
    *NKVAR = 0;

    //
    // Fetch attributes of the surface mapping kernel variables.
    //
    DTPOOL(KVSFNM, &mut FNDNAM, &mut NNAME, &mut NDTYPE, ctx)?;
    DTPOOL(KVSFCD, &mut FNDCDE, &mut NCODE, &mut CDTYPE, ctx)?;
    DTPOOL(KVSFBD, &mut FNDBOD, &mut NBODY, &mut BDTYPE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZSRFKER", ctx)?;
        return Ok(());
    }

    //
    // The variables must all be present or all be absent.
    //
    if ((FNDCDE != FNDNAM) || (FNDBOD != FNDNAM)) {
        SETMSG(b"Surface mapping kernel variables are in an inconsistent state. # was #; # was #; # was #.", ctx);

        ERRCH(b"#", KVSFNM, ctx);

        if FNDNAM {
            ERRCH(b"#", b"found", ctx);
        } else {
            ERRCH(b"#", b"not found", ctx);
        }

        ERRCH(b"#", KVSFCD, ctx);

        if FNDCDE {
            ERRCH(b"#", b"found", ctx);
        } else {
            ERRCH(b"#", b"not found", ctx);
        }

        ERRCH(b"#", KVSFBD, ctx);

        if FNDBOD {
            ERRCH(b"#", b"found", ctx);
        } else {
            ERRCH(b"#", b"not found", ctx);
        }

        SIGERR(b"SPICE(BADSURFACEMAP)", ctx)?;
        CHKOUT(b"ZZSRFKER", ctx)?;
        return Ok(());
    }

    //
    // If the variables are not present, leave now.
    //
    *EXTKER = ((FNDNAM && FNDCDE) && FNDBOD);

    if !*EXTKER {
        CHKOUT(b"ZZSRFKER", ctx)?;
        return Ok(());
    }

    //
    // Make sure the kernel variables aren't larger than our arrays.
    // Also make sure the variable have matching dimensions.
    //
    // Check variable types.
    //
    if ((fstr::ne(&NDTYPE, b"C") || fstr::ne(&CDTYPE, b"N")) || fstr::ne(&BDTYPE, b"N")) {
        SETMSG(b"Surface mapping kernel variable types are: # = #; # = #; # = #. These types must be, respectively, \'C\', \'N\', \'N\'.", ctx);
        ERRCH(b"#", KVSFNM, ctx);
        ERRCH(b"#", &NDTYPE, ctx);
        ERRCH(b"#", KVSFCD, ctx);
        ERRCH(b"#", &CDTYPE, ctx);
        ERRCH(b"#", KVSFBD, ctx);
        ERRCH(b"#", &BDTYPE, ctx);
        SIGERR(b"SPICE(BADVARIABLETYPE)", ctx)?;
        CHKOUT(b"ZZSRFKER", ctx)?;
        return Ok(());
    }

    //
    // Check variable dimensions.
    //
    if (((NNAME > MXNSRF) || (NCODE > MXNSRF)) || (NBODY > MXNSRF)) {
        SETMSG(b"Surface mapping kernel variable sizes are: # = #; # = #; # = #. Maximum allowed size is #.", ctx);
        ERRCH(b"#", KVSFNM, ctx);
        ERRINT(b"#", NNAME, ctx);
        ERRCH(b"#", KVSFCD, ctx);
        ERRINT(b"#", NCODE, ctx);
        ERRCH(b"#", KVSFBD, ctx);
        ERRINT(b"#", NBODY, ctx);
        ERRINT(b"#", MXNSRF, ctx);
        SIGERR(b"SPICE(TOOMANYSURFACES)", ctx)?;
        CHKOUT(b"ZZSRFKER", ctx)?;
        return Ok(());
    }

    if ((NCODE != NNAME) || (NBODY != NNAME)) {
        SETMSG(
            b"Surface variable sizes do not match. Size of # is #; size of # is #; size of # is #.",
            ctx,
        );
        ERRCH(b"#", KVSFNM, ctx);
        ERRINT(b"#", NNAME, ctx);
        ERRCH(b"#", KVSFCD, ctx);
        ERRINT(b"#", NCODE, ctx);
        ERRCH(b"#", KVSFBD, ctx);
        ERRINT(b"#", NBODY, ctx);
        SIGERR(b"SPICE(ARRAYSIZEMISMATCH)", ctx)?;
        CHKOUT(b"ZZSRFKER", ctx)?;
        return Ok(());
    }

    //
    // Fetch mapping variables.
    //
    // Note that we'll check the variable sizes below.
    //
    GCPOOL(
        KVSFNM,
        1,
        MXNSRF,
        &mut NNAME,
        KERNAM.as_arg_mut(),
        &mut FNDNAM,
        ctx,
    )?;
    GIPOOL(
        KVSFCD,
        1,
        MXNSRF,
        &mut NCODE,
        KERSID.as_slice_mut(),
        &mut FNDCDE,
        ctx,
    )?;
    GIPOOL(
        KVSFBD,
        1,
        MXNSRF,
        &mut NBODY,
        KERBID.as_slice_mut(),
        &mut FNDBOD,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZSRFKER", ctx)?;
        return Ok(());
    }

    *NKVAR = NNAME;

    //
    // Produce normalized name array. Check for blank names
    // as we go.
    //
    for I in 1..=*NKVAR {
        if fstr::eq(KERNAM.get(I), b" ") {
            SETMSG(b"An attempt to assign the code, #, to a blank string was made.  Check loaded text kernels for a blank string in the NAIF_SURFACE_NAME array.", ctx);
            ERRINT(b"#", I, ctx);
            SIGERR(b"SPICE(BLANKNAMEASSIGNED)", ctx)?;
            CHKOUT(b"ZZSRFKER", ctx)?;
            return Ok(());
        }

        LJUCRS(1, &KERNAM[I], &mut NORNAM[I], ctx);
    }

    //
    // Initialize hash data structures.
    //
    ZZSRFINI(
        NORNAM.as_arg(),
        KERSID.as_slice(),
        KERBID.as_slice(),
        *NKVAR,
        NROOM,
        SNMHLS.as_slice_mut(),
        SNMPOL.as_slice_mut(),
        SNMIDX.as_slice_mut(),
        SIDHLS.as_slice_mut(),
        SIDPOL.as_slice_mut(),
        SIDIDX.as_slice_mut(),
        ctx,
    )?;

    CHKOUT(b"ZZSRFKER", ctx)?;
    Ok(())
}
