//! EK General Limit Parameters
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
//! C     Include Section:  EK General Limit Parameters
//! C
//! C        ekglimit.inc  Version 1    21-MAY-1995 (NJB)
//! C
//! C
//! C     This file contains general limits for the EK system.
//! C
//! C     MXCLSG is the maximum number of columns allowed in a segment.
//! C     This limit applies to logical tables as well, since all segments
//! C     in a logical table must have the same column definitions.
//! C
//!       INTEGER               MXCLSG
//!       PARAMETER           ( MXCLSG = 100 )
//!  
//! C
//! C     End Include Section:  EK General Limit Parameters
//! C
//! ```

pub const MXCLSG: i32 = 100;
