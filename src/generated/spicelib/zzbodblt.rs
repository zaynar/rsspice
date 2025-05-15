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

struct SaveVars {
    BLTNAM: ActualCharArray,
    BLTNOR: ActualCharArray,
    BLTCOD: ActualArray<i32>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BLTNAM = ActualCharArray::new(MAXL, 1..=NPERM);
        let mut BLTNOR = ActualCharArray::new(MAXL, 1..=NPERM);
        let mut BLTCOD = ActualArray::<i32>::new(1..=NPERM);
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            BLTNAM,
            BLTNOR,
            BLTCOD,
            FIRST,
        }
    }
}

//$Procedure ZZBODBLT ( Private --- Retrieve Built-In Body-Code Maps )
pub fn ZZBODBLT(
    ROOM: i32,
    NAMES: CharArray,
    NORNAM: CharArray,
    CODES: &[i32],
    NVALS: i32,
    DEVICE: &[u8],
    REQST: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
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
        CHKIN(b"ZZBODBLT", ctx)?;
        SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
        CHKOUT(b"ZZBODBLT", ctx)?;
    }

    Ok(())
}

//$Procedure ZZBODGET ( Private --- Body-Code Get Built-In List )
pub fn ZZBODGET(
    ROOM: i32,
    NAMES: CharArrayMut,
    NORNAM: CharArrayMut,
    CODES: &mut [i32],
    NVALS: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut NAMES = DummyCharArrayMut::new(NAMES, None, 1..);
    let mut NORNAM = DummyCharArrayMut::new(NORNAM, None, 1..);
    let mut CODES = DummyArrayMut::new(CODES, 1..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZBODGET", ctx)?;
    }

    //
    // On the first invocation compute the normalized forms of BLTNAM
    // and store them in BLTNOR.
    //
    if save.FIRST {
        //
        // Retrieve the default mapping list.
        //
        ZZIDMAP(save.BLTCOD.as_slice_mut(), save.BLTNAM.as_arg_mut());

        for I in 1..=NPERM {
            LJUST(&save.BLTNAM[I], &mut save.BLTNOR[I]);
            UCASE(&save.BLTNOR[I].to_vec(), &mut save.BLTNOR[I], ctx);
            CMPRSS(b" ", 1, &save.BLTNOR[I].to_vec(), &mut save.BLTNOR[I]);
        }

        //
        // Do not do this again.
        //
        save.FIRST = false;
    }

    //
    // Copy the contents of BLTNAM, BLTNOR, and BLTCOD to the output
    // arguments, but only if there is sufficient room.
    //
    if (ROOM < NPERM) {
        SETMSG(b"Insufficient room to copy the stored body name-code mappings to the output arguments.  Space required is #, but the caller supplied #.", ctx);
        ERRINT(b"#", NPERM, ctx);
        ERRINT(b"#", ROOM, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZBODGET", ctx)?;
        return Ok(());
    }

    MOVEC(save.BLTNAM.as_arg(), NPERM, NAMES.as_arg_mut());
    MOVEC(save.BLTNOR.as_arg(), NPERM, NORNAM.as_arg_mut());
    MOVEI(save.BLTCOD.as_slice(), NPERM, CODES.as_slice_mut());

    *NVALS = NPERM;

    CHKOUT(b"ZZBODGET", ctx)?;
    Ok(())
}

//$Procedure ZZBODLST ( Output permanent collection to some device. )
pub fn ZZBODLST(DEVICE: &[u8], REQST: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ZZRQST = [b' '; 4 as usize];
    let mut ZZINT = [b' '; MAXL as usize];
    let mut ZZLINE = [b' '; ((2 * MAXL) + 3) as usize];
    let mut ZZOCOD = ActualArray::<i32>::new(1..=NPERM);
    let mut ZZONAM = ActualArray::<i32>::new(1..=NPERM);

    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZBODLST", ctx)?;
    }

    //
    // Upper case the ZZRQST value.
    //
    UCASE(REQST, &mut ZZRQST, ctx);
    INTSTR(NPERM, &mut ZZINT, ctx);

    fstr::assign(
        &mut ZZLINE,
        &fstr::concat(b"Total number of name/ID mappings: ", &ZZINT),
    );

    WRLINE(DEVICE, fstr::substr(&ZZLINE, 1..=LASTNB(&ZZLINE)), ctx)?;

    //
    // Retrieve the current set of name/ID mappings
    //
    ZZIDMAP(save.BLTCOD.as_slice_mut(), save.BLTNAM.as_arg_mut());

    //
    // Branch as defined by the value of ZZRQST. 'ID' or 'BOTH'.
    //
    if (EQSTR(&ZZRQST, b"ID") || EQSTR(&ZZRQST, b"BOTH")) {
        ORDERI(save.BLTCOD.as_slice(), NPERM, ZZOCOD.as_slice_mut());

        WRLINE(DEVICE, b" ", ctx)?;
        WRLINE(DEVICE, b"ID to name mappings.", ctx)?;

        for I in 1..=NPERM {
            INTSTR(save.BLTCOD[ZZOCOD[I]], &mut ZZINT, ctx);

            fstr::assign(
                &mut ZZLINE,
                &fstr::concat(&fstr::concat(&ZZINT, b" | "), save.BLTNAM.get(ZZOCOD[I])),
            );

            WRLINE(DEVICE, fstr::substr(&ZZLINE, 1..=LASTNB(&ZZLINE)), ctx)?;
        }
    }

    //
    // ... 'NAME' or 'BOTH'.
    //
    if (EQSTR(&ZZRQST, b"NAME") || EQSTR(&ZZRQST, b"BOTH")) {
        ORDERC(save.BLTNAM.as_arg(), NPERM, ZZONAM.as_slice_mut());

        WRLINE(DEVICE, b" ", ctx)?;
        WRLINE(DEVICE, b"Name to ID mappings.", ctx)?;

        for I in 1..=NPERM {
            INTSTR(save.BLTCOD[ZZONAM[I]], &mut ZZINT, ctx);

            fstr::assign(
                &mut ZZLINE,
                &fstr::concat(&fstr::concat(save.BLTNAM.get(ZZONAM[I]), b" | "), &ZZINT),
            );

            WRLINE(DEVICE, fstr::substr(&ZZLINE, 1..=LASTNB(&ZZLINE)), ctx)?;
        }
    }

    CHKOUT(b"ZZBODLST", ctx)?;
    Ok(())
}
