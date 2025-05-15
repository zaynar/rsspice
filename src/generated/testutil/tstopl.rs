//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FILEN: i32 = 255;
const LNSIZE: i32 = 80;
const ROOM: i32 = 2;
const WDSIZE: i32 = 32;

struct SaveVars {
    LOGFIL: Vec<u8>,
    ENV: ActualCharArray,
    ATTR: ActualCharArray,
    TIME: Vec<u8>,
    TKV: Vec<u8>,
    N: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LOGFIL = vec![b' '; FILEN as usize];
        let mut ENV = ActualCharArray::new(LNSIZE, 1..=2);
        let mut ATTR = ActualCharArray::new(WDSIZE, 1..=ROOM);
        let mut TIME = vec![b' '; WDSIZE as usize];
        let mut TKV = vec![b' '; WDSIZE as usize];
        let mut N: i32 = 0;

        Self {
            LOGFIL,
            ENV,
            ATTR,
            TIME,
            TKV,
            N,
        }
    }
}

//$ Disclaimer
//
//     THIS SOFTWARE AND ANY RELATED MATERIALS WERE CREATED BY THE
//     CALIFORNIA INSTITUTE OF TECHNOLOGY (CALTECH) UNDER A U.S.
//     GOVERNMENT CONTRACT WITH THE NATIONAL AERONAUTICS AND SPACE
//     ADMINISTRATION (NASA). THE SOFTWARE IS TECHNOLOGY AND SOFTWARE
//     PUBLICLY AVAILABLE UNDER U.S. EXPORT LAWS AND IS PROVIDED "AS-IS"
//     TO THE RECIPIENT WITHOUT WARRANTY OF ANY KIND, INCLUDING ANY
//     WARRANTIES OF PERFORMANCE OR MERCHANTABILITY OR FITNESS FOR A
//     PARTICULAR USE OR PURPOSE (AS SET FORTH IN UNITED STATES UCC
//     SECTIONS 2312-2313) OR FOR ANY PURPOSE WHATSOEVER, FOR THE
//     SOFTWARE AND RELATED MATERIALS, HOWEVER USED.
//
//     IN NO EVENT SHALL CALTECH, ITS JET PROPULSION LABORATORY, OR NASA
//     BE LIABLE FOR ANY DAMAGES AND/OR COSTS, INCLUDING, BUT NOT
//     LIMITED TO, INCIDENTAL OR CONSEQUENTIAL DAMAGES OF ANY KIND,
//     INCLUDING ECONOMIC DAMAGE OR INJURY TO PROPERTY AND LOST PROFITS,
//     REGARDLESS OF WHETHER CALTECH, JPL, OR NASA BE ADVISED, HAVE
//     REASON TO KNOW, OR, IN FACT, SHALL KNOW OF THE POSSIBILITY.
//
//     RECIPIENT BEARS ALL RISK RELATING TO QUALITY AND PERFORMANCE OF
//     THE SOFTWARE AND ANY RELATED MATERIALS, AND AGREES TO INDEMNIFY
//     CALTECH AND NASA FOR ALL THIRD-PARTY CLAIMS RESULTING FROM THE
//     ACTIONS OF RECIPIENT IN THE USE OF THE SOFTWARE.
//
pub fn TSTOPL(LOGNAM: &[u8], VERSN: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    TSTFIL(LOGNAM, b"LOG", &mut save.LOGFIL, ctx)?;

    if spicelib::FAILED(ctx) {
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
            writer.start()?;
            writer.finish()?;
        }
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
            writer.start()?;
            writer.write_str(b"A log file cannot be opened.")?;
            writer.finish()?;
        }
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
            writer.start()?;
            writer.finish()?;
        }
        ctx.stop()?;
    }
    support::CURTIM(&mut save.TIME, ctx)?;
    support::PLTFRM(ROOM, &mut save.N, save.ATTR.as_arg_mut(), ctx);
    spicelib::TKVRSN(b"TOOLKIT", &mut save.TKV);

    fstr::assign(save.ENV.get_mut(1), b"--");
    fstr::assign(save.ENV.get_mut(2), b"--");
    spicelib::SUFFIX(&save.ATTR[1], 1, &mut save.ENV[1]);
    spicelib::SUFFIX(b"--", 1, &mut save.ENV[1]);
    spicelib::SUFFIX(&save.ATTR[2], 1, &mut save.ENV[2]);
    spicelib::SUFFIX(b"--", 1, &mut save.ENV[2]);

    TSTLOG(&save.ENV[1], false, ctx)?;
    TSTLOG(&save.ENV[2], false, ctx)?;
    TSTLOG(VERSN, false, ctx)?;
    TSTLOG(&save.TKV, false, ctx)?;
    TSTLOG(&save.TIME, false, ctx)?;
    TSTSAV(save.ENV.first(), VERSN, &save.TIME, ctx);

    Ok(())
}
