//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      TSTTXT (Create at test text file.)
pub fn TSTTXT(
    NAMTXT: &[u8],
    TXT: CharArray,
    NLINES: i32,
    LOAD: bool,
    KEEP: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let TXT = DummyCharArray::new(TXT, None, 1..);
    let mut UNIT: i32 = 0;

    //
    // Local Variables.
    //

    KILFIL(NAMTXT, ctx)?;

    //
    // Create the text file.
    //
    UNIT = 6;
    spicelib::TXTOPN(NAMTXT, &mut UNIT, ctx)?;
    spicelib::WRITLA(NLINES, TXT.as_arg(), UNIT, ctx)?;
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    //
    // If this file needs to be loaded.  Do it now.  If not we are
    // done and can return.
    //
    if LOAD {
        spicelib::LDPOOL(NAMTXT, ctx)?;

        if KEEP {
            TFILES(NAMTXT, ctx);
            return Ok(());
        } else {
            KILFIL(NAMTXT, ctx)?;
        }
    }

    TFILES(NAMTXT, ctx);

    Ok(())
}
