//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MXNAME: i32 = 100;
pub const MXVALS: i32 = 400;
const LBCELL: i32 = -5;

struct SaveVars {
    NAMES: ActualCharArray,
    PTRS: StackArray<i32, 106>,
    VALS: ActualArray<i32>,
    ANAMES: ActualCharArray,
    APTRS: StackArray<i32, 106>,
    AVALS: ActualArray<i32>,
    BEGEND: StackArray<i32, 2>,
    J: i32,
    K: i32,
    MYNAME: Vec<u8>,
    TEMP: ActualArray<i32>,
    TOTAL: i32,
    FIRST: bool,
    GOTIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NAMES = ActualCharArray::new(32, LBCELL..=MXNAME);
        let mut PTRS = StackArray::<i32, 106>::new(LBCELL..=MXNAME);
        let mut VALS = ActualArray::<i32>::new(LBCELL..=MXVALS);
        let mut ANAMES = ActualCharArray::new(32, LBCELL..=MXNAME);
        let mut APTRS = StackArray::<i32, 106>::new(LBCELL..=MXNAME);
        let mut AVALS = ActualArray::<i32>::new(LBCELL..=MXVALS);
        let mut BEGEND = StackArray::<i32, 2>::new(1..=2);
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut MYNAME = vec![b' '; 32 as usize];
        let mut TEMP = ActualArray::<i32>::new(1..=MXVALS);
        let mut TOTAL: i32 = 0;
        let mut FIRST: bool = false;
        let mut GOTIT: bool = false;

        FIRST = true;

        Self {
            NAMES,
            PTRS,
            VALS,
            ANAMES,
            APTRS,
            AVALS,
            BEGEND,
            J,
            K,
            MYNAME,
            TEMP,
            TOTAL,
            FIRST,
            GOTIT,
        }
    }
}

//$Procedure M2PARS ( META/2 --- Parsing utility. )
pub fn M2PARS(NAME: &[u8], B: i32, E: i32, NTH: i32, FOUND: bool, SIZE: i32) {

    //
    // Spicelib Functions.
    //

    //
    // Private Parameters
    //

    //
    // Local Variables.
    //
}

//$Procedure M2SAVE ( META/2 --- save substring boundaries )
pub fn M2SAVE(NAME: &[u8], B: i32, E: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if save.FIRST {
        save.FIRST = false;
        //
        // Initialize the keepers table.
        //
        spicelib::SSIZEC(MXNAME, save.NAMES.as_arg_mut(), ctx)?;
        spicelib::SSIZEI(MXNAME, save.PTRS.as_slice_mut(), ctx)?;
        spicelib::SSIZEI(MXVALS, save.VALS.as_slice_mut(), ctx)?;

        //
        // Initialize the accumulation table
        //
        spicelib::SSIZEC(MXNAME, save.ANAMES.as_arg_mut(), ctx)?;
        spicelib::SSIZEI(MXNAME, save.APTRS.as_slice_mut(), ctx)?;
        spicelib::SSIZEI(MXVALS, save.AVALS.as_slice_mut(), ctx)?;
    }

    //
    // Enque the new string boundaries in the accumulation table.
    //
    spicelib::SYENQI(
        NAME,
        B,
        save.ANAMES.as_arg_mut(),
        save.APTRS.as_slice_mut(),
        save.AVALS.as_slice_mut(),
        ctx,
    )?;
    spicelib::SYENQI(
        NAME,
        E,
        save.ANAMES.as_arg_mut(),
        save.APTRS.as_slice_mut(),
        save.AVALS.as_slice_mut(),
        ctx,
    )?;

    Ok(())
}

//$Procedure      M2PCLR ( META/2 --- Parse table clear )
pub fn M2PCLR(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    save.FIRST = false;
    //
    // Initialize the keepers table.
    //
    spicelib::SSIZEC(MXNAME, save.NAMES.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(MXNAME, save.PTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MXVALS, save.VALS.as_slice_mut(), ctx)?;

    //
    // Initialize the accumulation table
    //
    spicelib::SSIZEC(MXNAME, save.ANAMES.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(MXNAME, save.APTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MXVALS, save.AVALS.as_slice_mut(), ctx)?;

    Ok(())
}

//$Procedure      M2TCLR ( META/2 --- Temporary parse table clear )
pub fn M2TCLR(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    //
    // Just in case, we initialize the keepers table if it hasn't been
    // initialized already.
    //
    if save.FIRST {
        save.FIRST = false;

        //
        // Initialize the keepers table.
        //
        spicelib::SSIZEC(MXNAME, save.NAMES.as_arg_mut(), ctx)?;
        spicelib::SSIZEI(MXNAME, save.PTRS.as_slice_mut(), ctx)?;
        spicelib::SSIZEI(MXVALS, save.VALS.as_slice_mut(), ctx)?;
    }
    //
    // Initialize the accumulation table
    //
    spicelib::SSIZEC(MXNAME, save.ANAMES.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(MXNAME, save.APTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MXVALS, save.AVALS.as_slice_mut(), ctx)?;

    Ok(())
}

//$Procedure M2KEEP ( META/2 --- Keep temporary parse table values )
pub fn M2KEEP(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    //
    // For each entry in the accumulation table...
    //
    for I in 1..=spicelib::CARDC(save.ANAMES.as_arg(), ctx)? {
        //
        // Find out its name,
        //

        spicelib::SYFETI(
            I,
            save.ANAMES.as_arg(),
            save.APTRS.as_slice(),
            save.AVALS.as_slice(),
            &mut save.MYNAME,
            &mut save.GOTIT,
            ctx,
        )?;

        if save.GOTIT {
            //
            // ...extract the values.
            //
            spicelib::SYGETI(
                &save.MYNAME,
                save.ANAMES.as_arg(),
                save.APTRS.as_slice(),
                save.AVALS.as_slice(),
                &mut save.TOTAL,
                save.TEMP.as_slice_mut(),
                &mut save.GOTIT,
                ctx,
            )?;

            //
            // and put them in the keepers table.
            //
            spicelib::SYPUTI(
                &save.MYNAME,
                save.TEMP.as_slice(),
                save.TOTAL,
                save.NAMES.as_arg_mut(),
                save.PTRS.as_slice_mut(),
                save.VALS.as_slice_mut(),
                ctx,
            )?;
        }
    }

    Ok(())
}

//$Procedure M2VGET ( META/2 --- Get variable )
pub fn M2VGET(
    NAME: &[u8],
    NTH: i32,
    FOUND: &mut bool,
    B: &mut i32,
    E: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    //
    // Look up any parsed values.
    //
    *FOUND = false;

    save.J = ((2 * NTH) - 1);
    save.K = (2 * NTH);

    spicelib::SYSELI(
        NAME,
        save.J,
        save.K,
        save.NAMES.as_arg(),
        save.PTRS.as_slice(),
        save.VALS.as_slice(),
        save.BEGEND.as_slice_mut(),
        FOUND,
        ctx,
    )?;

    if *FOUND {
        *B = save.BEGEND[1];
        *E = save.BEGEND[2];
    }
    //
    // That's all folks....
    //

    Ok(())
}

//$Procedure M2VSIZ ( META/2 --- matched variable template size )
pub fn M2VSIZ(NAME: &[u8], SIZE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    //
    // Just look up the number of word boundaries and divide by two.
    //
    save.TOTAL = spicelib::SYDIMI(
        NAME,
        save.NAMES.as_arg(),
        save.PTRS.as_slice(),
        save.VALS.as_slice(),
        ctx,
    )?;
    *SIZE = (save.TOTAL / 2);
    Ok(())
}
