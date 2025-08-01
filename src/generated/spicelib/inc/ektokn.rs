//! EK Token Code Parameters
//!
//! ```text
//! C$ Disclaimer
//! C
//! C     THIS SOFTWARE AND ANY RELATED MATERIALS WERE CREATED BY THE
//! C     CALIFORNIA INSTITUTE OF TECHNOLOGY (CALTECH) UNDER A U.S.
//! C     GOVERNMENT CONTRACT WITH THE NATIONAL AERONAUTICS AND SPACE
//! C     ADMINISTRATION (NASA). THE SOFTWARE IS TECHNOLOGY AND SOFTWARE
//! C     PUBLICLY AVAILABLE UNDER U.S. EXPORT LAWS AND IS PROVIDED "AS-IS"
//! C     TO THE RECIPIENT WITHOUT WARRANTY OF ANY KIND, INCLUDING ANY
//! C     WARRANTIES OF PERFORMANCE OR MERCHANTABILITY OR FITNESS FOR A
//! C     PARTICULAR USE OR PURPOSE (AS SET FORTH IN UNITED STATES UCC
//! C     SECTIONS 2312-2313) OR FOR ANY PURPOSE WHATSOEVER, FOR THE
//! C     SOFTWARE AND RELATED MATERIALS, HOWEVER USED.
//! C
//! C     IN NO EVENT SHALL CALTECH, ITS JET PROPULSION LABORATORY, OR NASA
//! C     BE LIABLE FOR ANY DAMAGES AND/OR COSTS, INCLUDING, BUT NOT
//! C     LIMITED TO, INCIDENTAL OR CONSEQUENTIAL DAMAGES OF ANY KIND,
//! C     INCLUDING ECONOMIC DAMAGE OR INJURY TO PROPERTY AND LOST PROFITS,
//! C     REGARDLESS OF WHETHER CALTECH, JPL, OR NASA BE ADVISED, HAVE
//! C     REASON TO KNOW, OR, IN FACT, SHALL KNOW OF THE POSSIBILITY.
//! C
//! C     RECIPIENT BEARS ALL RISK RELATING TO QUALITY AND PERFORMANCE OF
//! C     THE SOFTWARE AND ANY RELATED MATERIALS, AND AGREES TO INDEMNIFY
//! C     CALTECH AND NASA FOR ALL THIRD-PARTY CLAIMS RESULTING FROM THE
//! C     ACTIONS OF RECIPIENT IN THE USE OF THE SOFTWARE.
//! C
//! C
//! C     Include Section:  EK Token Code Parameters
//! C
//! C        ektokn.inc  Version 2    25-JAN-1995 (NJB)
//! C
//! C           Updated to distinguish between special characters.
//! C
//! C
//! C        ektokn.inc  Version 1    05-DEC-1994 (NJB)
//! C
//! C
//! C     The EK query language tokens and codes are:
//! C
//! C        <keyword>
//! C        <identifier>
//! C        <integer>
//! C        <d.p. number>
//! C        <quoted string>
//! C        <left parenthesis>
//! C        <right parenthesis>
//! C        <comma>
//! C        <period>
//! C        <end of query>
//! C
//! C
//!       INTEGER               TKKEY
//!       PARAMETER           ( TKKEY  = 1 )
//!  
//!       INTEGER               TKID
//!       PARAMETER           ( TKID   = TKKEY + 1 )
//!  
//!       INTEGER               TKINT
//!       PARAMETER           ( TKINT  = TKID + 1 )
//!  
//!       INTEGER               TKDP
//!       PARAMETER           ( TKDP   = TKINT + 1 )
//!  
//!       INTEGER               TKQSTR
//!       PARAMETER           ( TKQSTR = TKDP + 1 )
//!  
//!       INTEGER               TKLPAR
//!       PARAMETER           ( TKLPAR = TKQSTR + 1 )
//!  
//!       INTEGER               TKRPAR
//!       PARAMETER           ( TKRPAR = TKLPAR + 1 )
//!  
//!       INTEGER               TKCOMA
//!       PARAMETER           ( TKCOMA = TKRPAR + 1 )
//!  
//!       INTEGER               TKDOT
//!       PARAMETER           ( TKDOT  = TKCOMA + 1 )
//!  
//!       INTEGER               TKSTAR
//!       PARAMETER           ( TKSTAR = TKDOT  + 1 )
//!  
//!       INTEGER               TKEOQ
//!       PARAMETER           ( TKEOQ  = TKSTAR + 1 )
//!  
//! C
//! C     End Include Section:  EK Token Code Parameters
//! C
//! ```

pub const TKKEY: i32 = 1;
pub const TKID: i32 = (TKKEY + 1);
pub const TKINT: i32 = (TKID + 1);
pub const TKDP: i32 = (TKINT + 1);
pub const TKQSTR: i32 = (TKDP + 1);
pub const TKLPAR: i32 = (TKQSTR + 1);
pub const TKRPAR: i32 = (TKLPAR + 1);
pub const TKCOMA: i32 = (TKRPAR + 1);
pub const TKDOT: i32 = (TKCOMA + 1);
pub const TKSTAR: i32 = (TKDOT + 1);
pub const TKEOQ: i32 = (TKSTAR + 1);
