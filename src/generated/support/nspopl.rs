//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNGSIZ: i32 = 800;
const LNSIZE: i32 = 80;
const ROOM: i32 = 2;
const WDSIZE: i32 = 32;
const MAXCOM: i32 = 20;
const FILEN: i32 = 128;
const COMSIZ: i32 = 1024;
const ERRSIZ: i32 = 1760;
const STYSIZ: i32 = 120;

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
pub fn NSPOPL(LOGNAM: &[u8], VERSN: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut LOGFIL = [b' '; FILEN as usize];
    let mut MYERR = ActualCharArray::new(LNGSIZ, 1..=2);
    let mut REST = [b' '; LNGSIZ as usize];
    let mut STYLE = [b' '; LNSIZE as usize];
    let mut ENV = [b' '; LNSIZE as usize];
    let mut TKV = [b' '; LNSIZE as usize];
    let mut ERR = [b' '; LNSIZE as usize];
    let mut ATTR = ActualCharArray::new(WDSIZE, 1..=ROOM);
    let mut WAS = [b' '; WDSIZE as usize];
    let mut VALUE = [b' '; WDSIZE as usize];
    let mut IO = [b' '; WDSIZE as usize];
    let mut TIME = [b' '; WDSIZE as usize];
    let mut WARN = [b' '; WDSIZE as usize];
    let mut I: i32 = 0;
    let mut N: i32 = 0;
    let mut PTR: i32 = 0;
    let mut START: i32 = 0;
    let mut FOUND: bool = false;

    //

    fstr::assign(MYERR.get_mut(1), b" ");
    fstr::assign(MYERR.get_mut(2), b" ");

    {
        let m1__: i32 = 1;
        let m2__: i32 = ROOM;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(ATTR.get_mut(I), b" ");
            I += m3__;
        }
    }

    NEWFIL(LOGNAM, b"LOG", &mut LOGFIL, ctx)?;

    if HAVE(MYERR.as_arg_mut(), ctx)? {
        START = spicelib::POS(&MYERR[1], b"IOSTAT", 1);

        if (START > 0) {
            fstr::assign(&mut REST, fstr::substr(MYERR.get(1), START..));

            spicelib::NEXTWD(&REST.clone(), &mut IO, &mut REST);
            spicelib::NEXTWD(&REST.clone(), &mut WAS, &mut REST);
            spicelib::NEXTWD(&REST.clone(), &mut VALUE, &mut REST);

            if (spicelib::EQSTR(&WAS, b"was") && fstr::ne(&VALUE, b" ")) {
                fstr::assign(&mut ERR, b" ");
                spicelib::NPARSI(&VALUE, &mut I, &mut ERR, &mut PTR, ctx);

                if fstr::eq(&ERR, b" ") {
                    DCYPHR(I, &mut FOUND, &mut REST, ctx);

                    if FOUND {
                        fstr::assign(fstr::substr_mut(MYERR.get_mut(1), START..), &REST);
                    }
                }
            }
        }

        fstr::assign(&mut REST, MYERR.get(1));
        fstr::assign(&mut WARN, b" ");

        TRNLAT(b"WARNING", &mut WARN, ctx);
        TRNLAT(b"CANNOTOPENLOG", &mut MYERR[2], ctx);

        START = spicelib::RTRIM(&MYERR[2]);

        spicelib::PREFIX(fstr::substr(&MYERR[2], 1..=START), 1, &mut REST);

        fstr::assign(
            &mut STYLE,
            &fstr::concat(b"LEFT 1 RIGHT 78 NEWLINE /cr FLAG ", &WARN),
        );

        NICEPR_1(&REST, &STYLE, NSPWLN, ctx)?;
    } else {
        CURTIM(&mut TIME, ctx)?;
        PLTFRM(ROOM, &mut N, ATTR.as_arg_mut(), ctx);
        spicelib::TKVRSN(b"TOOLKIT", &mut TKV);
        fstr::assign(&mut ENV, ATTR.get(1));
        spicelib::SUFFIX(b"---", 1, &mut ENV);
        spicelib::SUFFIX(&ATTR[2], 1, &mut ENV);
        spicelib::PREFIX(b"SPICE Toolkit ", 1, &mut TKV);

        NSPLOG(&ENV, true, ctx)?;
        NSPLOG(VERSN, true, ctx)?;
        NSPLOG(&TKV, true, ctx)?;
        NSPLOG(&TIME, true, ctx)?;
    }

    Ok(())
}
