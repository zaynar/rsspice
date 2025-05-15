//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 6;
const SCALE: i32 = 10000;

pub fn T_CRCKDS(
    SEGNO: i32,
    INST: i32,
    START: f64,
    STOP: f64,
    DESCR: &mut [f64],
    GOTAV: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DESCR = DummyArrayMut::new(DESCR, 1..);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut IC = StackArray::<i32, 6>::new(1..=NI);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    spicelib::CHKIN(b"T_CRCKDS", ctx)?;

    spicelib::CLEARI(NI, IC.as_slice_mut());
    spicelib::CLEARD(ND, DC.as_slice_mut());

    IC[1] = INST;

    for I in 2..=NI {
        IC[I] = ((SCALE * SEGNO) + I);
    }

    if GOTAV {
        IC[4] = 1;
    } else {
        IC[4] = 0;
    }

    DC[1] = START;
    DC[2] = STOP;

    spicelib::DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), DESCR.as_slice_mut());

    spicelib::CHKOUT(b"T_CRCKDS", ctx)?;
    Ok(())
}
