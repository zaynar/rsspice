//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    YEAR: Vec<u8>,
    MONTH: ActualCharArray,
    DAY: Vec<u8>,
    HOUR: Vec<u8>,
    MIN: Vec<u8>,
    SEC: Vec<u8>,
    TVEC: StackArray<f64, 6>,
    IVEC: StackArray<i32, 6>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut YEAR = vec![b' '; 4 as usize];
        let mut MONTH = ActualCharArray::new(3, 1..=12);
        let mut DAY = vec![b' '; 2 as usize];
        let mut HOUR = vec![b' '; 2 as usize];
        let mut MIN = vec![b' '; 2 as usize];
        let mut SEC = vec![b' '; 2 as usize];
        let mut TVEC = StackArray::<f64, 6>::new(1..=6);
        let mut IVEC = StackArray::<i32, 6>::new(1..=6);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"JAN"),
                Val::C(b"FEB"),
                Val::C(b"MAR"),
                Val::C(b"APR"),
                Val::C(b"MAY"),
                Val::C(b"JUN"),
                Val::C(b"JUL"),
                Val::C(b"AUG"),
                Val::C(b"SEP"),
                Val::C(b"OCT"),
                Val::C(b"NOV"),
                Val::C(b"DEC"),
            ]
            .into_iter();
            MONTH
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            YEAR,
            MONTH,
            DAY,
            HOUR,
            MIN,
            SEC,
            TVEC,
            IVEC,
        }
    }
}

//$Procedure CURTIM (Current Time)
pub fn CURTIM(TIME: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    spicelib::ZZCPUTIM(save.TVEC.as_slice_mut(), ctx)?;

    for I in 1..=6 {
        save.IVEC[I] = (save.TVEC[I] as i32);
    }

    spicelib::INTSTR(save.IVEC[1], &mut save.YEAR, ctx);
    spicelib::INTSTR(save.IVEC[3], &mut save.DAY, ctx);
    spicelib::INTSTR(save.IVEC[4], &mut save.HOUR, ctx);
    spicelib::INTSTR(save.IVEC[5], &mut save.MIN, ctx);
    spicelib::INTSTR(save.IVEC[6], &mut save.SEC, ctx);

    spicelib::RJUST(&save.DAY.to_vec(), &mut save.DAY);
    spicelib::RJUST(&save.HOUR.to_vec(), &mut save.HOUR);
    spicelib::RJUST(&save.MIN.to_vec(), &mut save.MIN);
    spicelib::RJUST(&save.SEC.to_vec(), &mut save.SEC);

    spicelib::REPLCH(&save.DAY.to_vec(), b" ", b"0", &mut save.DAY);
    spicelib::REPLCH(&save.HOUR.to_vec(), b" ", b"0", &mut save.HOUR);
    spicelib::REPLCH(&save.MIN.to_vec(), b" ", b"0", &mut save.MIN);
    spicelib::REPLCH(&save.SEC.to_vec(), b" ", b"0", &mut save.SEC);

    fstr::assign(
        TIME,
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(
                        &fstr::concat(
                            &fstr::concat(
                                &fstr::concat(
                                    &fstr::concat(
                                        &fstr::concat(
                                            &fstr::concat(&save.YEAR, b"-"),
                                            save.MONTH.get(save.IVEC[2]),
                                        ),
                                        b"-",
                                    ),
                                    &save.DAY,
                                ),
                                b" ",
                            ),
                            &save.HOUR,
                        ),
                        b":",
                    ),
                    &save.MIN,
                ),
                b":",
            ),
            &save.SEC,
        ),
    );

    Ok(())
}
