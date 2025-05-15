//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;

//
// Detect path mismatch.
//
pub fn T_PTHCMP(PATH: CharArray, EXPPTH: CharArray, ctx: &mut Context) -> f2rust_std::Result<()> {
    let PATH = DummyCharArray::new(PATH, None, LBCELL..);
    let EXPPTH = DummyCharArray::new(EXPPTH, None, LBCELL..);
    let mut CP: i32 = 0;
    let mut CX: i32 = 0;
    let mut I: i32 = 0;
    let mut N: i32 = 0;
    let mut NM: i32 = 0;

    spicelib::CHKIN(b"T_PTHCMP", ctx)?;

    CP = spicelib::CARDC(PATH.as_arg(), ctx)?;
    CX = spicelib::CARDC(EXPPTH.as_arg(), ctx)?;

    NM = intrinsics::MIN0(&[CP, CX]);

    N = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NM;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if fstr::eq(PATH.get(I), EXPPTH.get(I)) {
                N = (N + 1);
            }

            I += m3__;
        }
    }

    if (CP != CX) {
        spicelib::SETMSG(b"Path was expected to have length # but had length #. Last element of actual path was <#>. Last element common to both paths was <#>.", ctx);
        spicelib::ERRINT(b"#", CX, ctx);
        spicelib::ERRINT(b"#", CP, ctx);

        if (CP > 0) {
            spicelib::ERRCH(b"#", &PATH[CP], ctx);
        } else {
            spicelib::ERRCH(b"#", b"<empty>", ctx);
        }

        if (N > 0) {
            spicelib::ERRCH(b"#", &EXPPTH[N], ctx);
        } else {
            spicelib::ERRCH(b"#", b"<empty>", ctx);
        }

        spicelib::SIGERR(b"SPICE(BADPATHLENGTH)", ctx)?;
        spicelib::CHKOUT(b"T_PTHCMP", ctx)?;
        return Ok(());
    }

    I = 1;

    while (I <= spicelib::CARDC(EXPPTH.as_arg(), ctx)?) {
        if fstr::ne(PATH.get(I), EXPPTH.get(I)) {
            spicelib::SETMSG(b"Path element # was expected to be <#> but was <#>.", ctx);
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRCH(b"#", &EXPPTH[I], ctx);
            spicelib::ERRCH(b"#", &PATH[I], ctx);
            spicelib::SIGERR(b"SPICE(BADPATH)", ctx)?;
            spicelib::CHKOUT(b"T_PTHCMP", ctx)?;
            return Ok(());
        }

        I = (I + 1);
    }

    spicelib::CHKOUT(b"T_PTHCMP", ctx)?;
    Ok(())
}

//
// Display path.
//
pub fn T_PTHDSP(PATH: CharArray, ctx: &mut Context) -> f2rust_std::Result<()> {
    let PATH = DummyCharArray::new(PATH, None, LBCELL..);
    let mut I: i32 = 0;

    spicelib::CHKIN(b"T_PTHDSP", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDC(PATH.as_arg(), ctx)?;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::TOSTDO(&PATH[I], ctx)?;
            I += m3__;
        }
    }

    spicelib::CHKOUT(b"T_PTHDSP", ctx)?;
    Ok(())
}
