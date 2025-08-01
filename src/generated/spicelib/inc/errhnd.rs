//! Constants
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
//! C     Include File:  SPICELIB Error Handling Parameters
//! C
//! C        errhnd.inc  Version 2    18-JUN-1997 (WLT)
//! C
//! C           The size of the long error message was
//! C           reduced from 25*80 to 23*80 so that it
//! C           will be accepted by the Microsoft Power Station
//! C           FORTRAN compiler which has an upper bound
//! C           of 1900 for the length of a character string.
//! C
//! C        errhnd.inc  Version 1    29-JUL-1997 (NJB)
//! C
//! C
//! C
//! C     Maximum length of the long error message:
//! C
//!       INTEGER               LMSGLN
//!       PARAMETER           ( LMSGLN = 23 * 80 )
//!
//! C
//! C     Maximum length of the short error message:
//! C
//!       INTEGER               SMSGLN
//!       PARAMETER           ( SMSGLN = 25 )
//!  
//! C
//! C     End Include File:  SPICELIB Error Handling Parameters
//! C
//! ```

pub const LMSGLN: i32 = (23 * 80);
pub const SMSGLN: i32 = 25;
