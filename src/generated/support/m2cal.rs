//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

//$Procedure      M2CAL ( Parse a UTC time string )
pub fn M2CAL(UTCSTR: &[u8], MSSG: &mut [u8], TCODE: &mut i32, ctx: &mut Context) {
    let mut MODIFY = ActualCharArray::new(16, 1..=5);
    let mut TYPE = [b' '; 8 as usize];
    let mut PICTUR = [b' '; LNSIZE as usize];
    let mut TVEC = StackArray::<f64, 8>::new(1..=8);
    let mut NTVEC: i32 = 0;
    let mut MODS: bool = false;
    let mut SUCCES: bool = false;
    let mut YABBRV: bool = false;

    //
    // NAIFLIB functions
    //

    if spicelib::RETURN(ctx) {
        return;
    }

    fstr::assign(MSSG, b" ");
    *TCODE = 0;

    spicelib::TPARTV(
        UTCSTR,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        MSSG,
        ctx,
    );

    if !SUCCES {
        *TCODE = 1;
    } else if fstr::eq(&TYPE, b"JD") {
        //
        // Don't do anything.
        //
    } else {
        spicelib::TCHECK(
            TVEC.as_slice(),
            &TYPE,
            MODS,
            MODIFY.as_arg(),
            &mut SUCCES,
            MSSG,
            ctx,
        );

        if !SUCCES {
            *TCODE = 2;
        }
    }
}
