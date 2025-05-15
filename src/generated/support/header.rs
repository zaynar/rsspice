//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NCOL: i32 = 40;
const NCOMP: i32 = 10;
const LNSIZE: i32 = 1600;

struct SaveVars {
    BUFFER: ActualCharArray2D,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BUFFER = ActualCharArray2D::new(LNSIZE, 1..=NCOL, 1..=NCOMP);
        let mut FIRST: bool = false;

        FIRST = true;

        Self { BUFFER, FIRST }
    }
}

//$Procedure      HEADER (HEADER for a report)
pub fn HEADER(N: i32, COMP: i32, VALUE: &[u8], WDTH: i32) {

    //
    // SPICELIB Functions
    //

    //
    // Buffer declarations
    //
}

//
// Set a column component value.
//
pub fn SCOLMN(N: i32, COMP: i32, VALUE: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if save.FIRST {
        for I in 1..=NCOL {
            for J in 1..=NCOMP {
                fstr::assign(save.BUFFER.get_mut([I, J]), b" ");
            }
        }

        save.FIRST = false;
    }

    if ((((N >= 1) && (N <= NCOL)) && (COMP >= 1)) && (COMP <= NCOMP)) {
        fstr::assign(save.BUFFER.get_mut([N, COMP]), VALUE);
    }
}

//
// Get a column component value.
//
pub fn GCOLMN(
    N: i32,
    COMP: i32,
    VALUE: &mut [u8],
    WDTH: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if save.FIRST {
        for I in 1..=NCOL {
            for J in 1..=NCOMP {
                fstr::assign(save.BUFFER.get_mut([I, J]), b" ");
            }
        }

        save.FIRST = false;
    }

    if ((((N >= 1) && (N <= NCOL)) && (COMP >= 1)) && (COMP <= NCOMP)) {
        fstr::assign(VALUE, save.BUFFER.get([N, COMP]));
        *WDTH = spicelib::RTRIM(VALUE);
    } else {
        fstr::assign(VALUE, b" ");
        *WDTH = 1;
    }

    Ok(())
}

pub fn CCOLMN(ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    for I in 1..=NCOL {
        for J in 1..=NCOMP {
            fstr::assign(save.BUFFER.get_mut([I, J]), b" ");
        }
    }
}
