//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXSYS: i32 = 200;
const WDSIZE: i32 = 32;
const ON: bool = true;
const OFF: bool = false;

struct SaveVars {
    SYSTMS: ActualCharArray,
    OVFLOW: i32,
    NSYS: i32,
    STATE: StackArray<bool, 200>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SYSTMS = ActualCharArray::new(WDSIZE, 1..=MAXSYS);
        let mut OVFLOW: i32 = 0;
        let mut NSYS: i32 = 0;
        let mut STATE = StackArray::<bool, 200>::new(1..=MAXSYS);

        NSYS = 0;
        OVFLOW = MAXSYS;

        Self {
            SYSTMS,
            OVFLOW,
            NSYS,
            STATE,
        }
    }
}

//$Procedure   ISON ( Determine whether a system is on or off )
pub fn ISON(SYSTEM: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ISON: bool = false;
    let mut I: i32 = 0;

    //
    // SPICELIB Functions
    //

    I = spicelib::ISRCHC(SYSTEM, save.NSYS, save.SYSTMS.as_arg());

    if (I > 0) {
        ISON = save.STATE[I];
    } else {
        ISON = ON;
    }

    ISON
}

pub fn ISOFF(SYSTEM: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ISOFF: bool = false;
    let mut I: i32 = 0;

    I = spicelib::ISRCHC(SYSTEM, save.NSYS, save.SYSTMS.as_arg());

    if (I > 0) {
        ISOFF = !save.STATE[I];
    } else {
        ISOFF = OFF;
    }

    ISOFF
}

pub fn SETON(SYSTEM: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SETON: bool = false;
    let mut I: i32 = 0;

    //
    // Provide a dummy return value to satisfy various compiler
    // checks.
    //
    SETON = true;

    //
    // See if we recognize this system.
    //
    I = spicelib::ISRCHC(SYSTEM, save.NSYS, save.SYSTMS.as_arg());

    if (I == 0) {
        //
        // Nope, add it into the list.
        //
        if (save.NSYS < MAXSYS) {
            save.NSYS = (save.NSYS + 1);
            fstr::assign(save.SYSTMS.get_mut(save.NSYS), SYSTEM);
            save.STATE[save.NSYS] = ON;
        } else {
            save.OVFLOW = (save.OVFLOW + 1);
            if (save.OVFLOW > MAXSYS) {
                save.OVFLOW = 1;
            }

            fstr::assign(save.SYSTMS.get_mut(save.NSYS), SYSTEM);
            save.STATE[save.NSYS] = ON;
        }
    } else {
        save.STATE[I] = ON;
    }

    SETON
}

pub fn SETOFF(SYSTEM: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SETOFF: bool = false;
    let mut I: i32 = 0;

    //
    // Provide a dummy return value to satisfy various compiler
    // checks.
    //
    SETOFF = true;

    //
    // See if we recognize this system.
    //
    I = spicelib::ISRCHC(SYSTEM, save.NSYS, save.SYSTMS.as_arg());

    if (I == 0) {
        //
        // Nope, add it into the list.
        //
        if (save.NSYS < MAXSYS) {
            save.NSYS = (save.NSYS + 1);
            fstr::assign(save.SYSTMS.get_mut(save.NSYS), SYSTEM);
            save.STATE[save.NSYS] = OFF;
        } else {
            save.OVFLOW = (save.OVFLOW + 1);
            if (save.OVFLOW > MAXSYS) {
                save.OVFLOW = 1;
            }

            fstr::assign(save.SYSTMS.get_mut(save.NSYS), SYSTEM);
            save.STATE[save.NSYS] = OFF;
        }
    } else {
        save.STATE[I] = OFF;
    }

    SETOFF
}
