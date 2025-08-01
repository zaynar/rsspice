//! EK Template Matching Wild Characters
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
//! C     Include Section:  EK Template Matching Wild Characters
//! C
//! C
//! C        ekwild.inc  Version 1   16-JAN-1995 (NJB)
//! C
//! C
//! C     Within the EK system, templates used for pattern matching
//! C     are those accepted by the SPICELIB routine MATCHW.  MATCHW
//! C     accepts two special characters:  one representing wild
//! C     strings and one representing wild characters.  This include
//! C     file defines those special characters for use within the EK
//! C     system.
//! C
//! C
//! C     Wild string symbol:  this character matches any string.
//! C
//!       CHARACTER*(1)         WSTR
//!       PARAMETER           ( WSTR = '*' )
//!  
//! C
//! C     Wild character symbol:  this character matches any character.
//! C
//!       CHARACTER*(1)         WCHR
//!       PARAMETER           ( WCHR = '%' )
//!  
//! C
//! C     End Include Section:  EK Template Matching Wild Characters
//! C
//! ```

pub const WSTR: &str = "*";
pub const WCHR: &str = "%";
