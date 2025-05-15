//!  This file contains public, global parameter declarations
//!  for the SPICELIB Direct Access Segregated (DAS) subsystem.
//!
//! ```text
//! C$ Abstract
//! C
//! C     This file contains public, global parameter declarations
//! C     for the SPICELIB Direct Access Segregated (DAS) subsystem.
//! C
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
//! C$ Required_Reading
//! C
//! C     DAS
//! C
//! C$ Keywords
//! C
//! C     None.
//! C
//! C$ Restrictions
//! C
//! C     None.
//! C
//! C$ Author_and_Institution
//! C
//! C     N.J. Bachman       (JPL)
//! C     J. Diaz del Rio    (ODC Space)
//! C
//! C$ Version
//! C
//! C-    SPICELIB Version 1.1.0, 07-APR-2020 (JDR)
//! C
//! C        Added CHARDT, DPDT and INTDT parameters.
//! C
//! C-    SPICELIB Version 1.0.0, 10-FEB-2017 (NJB)
//! C
//! C-&
//!
//! C
//! C     Parameter declarations follow.
//! C
//!
//! C
//! C     DAS file table size:
//! C
//! C        The parameter name is FTSIZE. The value of the parameter is
//! C        defined in the include file
//! C
//! C           zzddhman.inc
//! C
//! C        That value is duplicated here, since zzddhman.inc contains
//! C        other declarations that conflict with some of those in DAS
//! C        routines.
//! C
//!       INTEGER               FTSIZE
//!       PARAMETER           ( FTSIZE  = 5000 )
//!
//! C
//! C     Capacity of DAS data records:
//! C
//! C        -- NWD double precision numbers.
//! C        -- NWI integers.
//! C        -- NWC characters.
//! C
//! C     These parameters are named to enhance ease of maintenance of
//! C     the code; the values should not be changed.
//!  
//!       INTEGER               NWD
//!       PARAMETER           ( NWD     =  128 )
//!  
//!       INTEGER               NWI
//!       PARAMETER           ( NWI     =  256 )
//!  
//!       INTEGER               NWC
//!       PARAMETER           ( NWC     = 1024 )
//!
//! C
//! C     DAS data type specifiers used in all DAS routines that require
//! C     a data type either as input or to extract data from an output
//! C     array.
//! C
//! C     CHARDT,
//! C     DPDT,
//! C     INTDT    are data type specifiers which indicate CHARACTER,
//! C              DOUBLE PRECISION, and INTEGER respectively. These
//! C              parameters are used in all DAS routines that require a
//! C              data type specifier.
//! C
//!       INTEGER               CHARDT
//!       PARAMETER           ( CHARDT  =  1   )
//!
//!       INTEGER               DPDT
//!       PARAMETER           ( DPDT    =  2   )
//!
//!       INTEGER               INTDT
//!       PARAMETER           ( INTDT   =  3   )
//!
//! C
//! C     End of include file das.inc
//! C
//! ```

pub const FTSIZE: i32 = 5000;
pub const NWD: i32 = 128;
pub const NWI: i32 = 256;
pub const NWC: i32 = 1024;
pub const CHARDT: i32 = 1;
pub const DPDT: i32 = 2;
pub const INTDT: i32 = 3;
