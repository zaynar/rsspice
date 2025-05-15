//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 132;
const SMSIZE: i32 = 8;

//$Procedure      MPRINT ( Matrix Print )
pub fn MPRINT(
    MAT: &[f64],
    ROWS: i32,
    COLS: i32,
    FMT: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let MAT = DummyArray2D::new(MAT, 1..=ROWS, 1..=COLS);
    let mut TEMP = [b' '; LNSIZE as usize];
    let mut LINE = [b' '; LNSIZE as usize];
    let mut TOOBIG = [b' '; SMSIZE as usize];
    let mut WIDTH: i32 = 0;
    let mut C: i32 = 0;
    let mut SPACE: i32 = 0;
    let mut SIGDIG: i32 = 0;
    let mut STARS: bool = false;

    //
    // Spicelib functions
    //

    //
    // Local Variables.
    //

    WIDTH = intrinsics::MAX0(&[1, spicelib::NBLEN(FMT)]);
    SPACE = intrinsics::MIN0(&[4, intrinsics::MAX0(&[1, (((80 - WIDTH) / COLS) - WIDTH)])]);
    SIGDIG = intrinsics::MAX0(&[1, (((WIDTH - 6) + SPACE) - 1)]);

    if ((((WIDTH - 6) + SPACE) - 1) < 1) {
        STARS = true;
        fstr::assign(&mut TOOBIG, b"********");
    } else {
        STARS = false;
    }

    for I in 1..=ROWS {
        C = 1;

        for J in 1..=COLS {
            spicelib::DPFMT(MAT[[I, J]], FMT, &mut TEMP, ctx)?;

            if (spicelib::RTRIM(&TEMP) > WIDTH) {
                if STARS {
                    fstr::assign(&mut TEMP, b" ");
                    fstr::assign(fstr::substr_mut(&mut TEMP, 1..=WIDTH), &TOOBIG);
                } else {
                    spicelib::DPSTRF(MAT[[I, J]], SIGDIG, b"E", &mut TEMP, ctx);
                }
            }

            if (C < LNSIZE) {
                fstr::assign(fstr::substr_mut(&mut LINE, C..), &TEMP);

                if ((C + WIDTH) > LNSIZE) {
                    fstr::assign(fstr::substr_mut(&mut LINE, LNSIZE..=LNSIZE), b"*");
                }
            } else if (C == LNSIZE) {
                fstr::assign(fstr::substr_mut(&mut LINE, C..=C), b"*");
            }

            C = ((C + WIDTH) + SPACE);
        }

        spicelib::TOSTDO(fstr::substr(&LINE, 1..=spicelib::RTRIM(&LINE)), ctx)?;
    }

    Ok(())
}
