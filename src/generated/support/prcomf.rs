//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NFILES: i32 = 8;

struct SaveVars {
    NEST: i32,
    FILES: ActualCharArray,
    UNITS: StackArray<i32, 8>,
    IOSTAT: i32,
    I: i32,
    J: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NEST: i32 = 0;
        let mut FILES = ActualCharArray::new(80, 1..=NFILES);
        let mut UNITS = StackArray::<i32, 8>::new(1..=NFILES);
        let mut IOSTAT: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;

        NEST = 0;

        Self {
            NEST,
            FILES,
            UNITS,
            IOSTAT,
            I,
            J,
        }
    }
}

//$ Procedure
//
pub fn PRCOMF(FILE: &[u8], DELIM: &[u8], COMMAND: &[u8], ERROR: &[u8], LEVEL: &[u8]) {

    //

    //
    // OPTLIB functions
    //

    //
    // Local variables
    //

    //
    // NFILES is the maximum number of files that may be open at
    // any given time. THus, nesting of procedures is limited to
    // a depth of NFILES.
    //

    //
    // NEST is the number of files currently open.
    //

    //
    // FILES are the names of the files on the stack. UNITS are
    // the logical units to which they are connected.
    //
}

//
pub fn PRCLR(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    while (save.NEST > 0) {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(save.UNITS[save.NEST]),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        save.NEST = (save.NEST - 1);
    }

    Ok(())
}

pub fn PRSTRT(FILE: &[u8], ERROR: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(ERROR, b" ");

    if (save.NEST == NFILES) {
        fstr::assign(ERROR, b"PRSTRT: Command files are nested too deeply.");
        return Ok(());
    } else {
        save.NEST = (save.NEST + 1);
    }

    spicelib::TXTOPR(FILE, &mut save.UNITS[save.NEST], ctx)?;

    if HAVE(CharArrayMut::from_mut(ERROR), ctx)? {
        save.NEST = (save.NEST - 1);
    } else {
        fstr::assign(save.FILES.get_mut(save.NEST), FILE);
    }

    Ok(())
}

pub fn PRREAD(DELIM: &[u8], COMMAND: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DELIM = &DELIM[..1];

    if (save.NEST == 0) {
        fstr::assign(COMMAND, DELIM);
        return Ok(());
    }

    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader =
            io::FormattedReader::new(ctx.io_unit(save.UNITS[save.NEST])?, None, b"(A)")?;
        save.IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(COMMAND)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    while ((save.IOSTAT != 0) && (save.NEST > 0)) {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(save.UNITS[save.NEST]),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        save.NEST = (save.NEST - 1);

        if (save.NEST >= 1) {
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader =
                    io::FormattedReader::new(ctx.io_unit(save.UNITS[save.NEST])?, None, b"(A)")?;
                save.IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(COMMAND)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }
        }
    }

    RSTBUF(ctx);

    if (save.NEST == 0) {
        fstr::assign(COMMAND, DELIM);
        PUTBUF(COMMAND, ctx);
        return Ok(());
    }

    PUTBUF(COMMAND, ctx);

    save.J = 1;

    save.I = intrinsics::INDEX(COMMAND, DELIM);

    while ((save.I == 0) && (save.IOSTAT == 0)) {
        save.J = (spicelib::LASTNB(COMMAND) + 1);
        fstr::assign(fstr::substr_mut(COMMAND, save.J..=save.J), b" ");
        save.J = (save.J + 1);

        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader =
                io::FormattedReader::new(ctx.io_unit(save.UNITS[save.NEST])?, None, b"(A)")?;
            save.IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(fstr::substr_mut(COMMAND, save.J..))?;
                reader.finish()?;
                Ok(())
            })?;
        }

        PUTBUF(fstr::substr(COMMAND, save.J..), ctx);
        save.I = intrinsics::INDEX(COMMAND, DELIM);
    }

    if (save.I > 0) {
        fstr::assign(fstr::substr_mut(COMMAND, save.I..), b" ");
    }

    Ok(())
}

pub fn PREXIT(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if (save.NEST > 0) {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(save.UNITS[save.NEST]),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        save.NEST = (save.NEST - 1);
    }

    Ok(())
}

pub fn PRTRCE(LEVEL: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(LEVEL, b" ");

    if (save.NEST > 0) {
        spicelib::LBUILD(save.FILES.as_arg(), save.NEST, b":", LEVEL);
    }
}
