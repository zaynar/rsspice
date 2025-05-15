//! EK Keyword Code Parameters
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
//! C     Include Section:  EK Keyword Code Parameters
//! C
//! C        ekkeyw.inc  Version 4    24-JAN-1995 (NJB)
//! C
//! C
//! C
//! C     The EK query language keywords and codes are:
//! C
//! C        ALL
//! C        AND
//! C        ASC
//! C        AVG
//! C        BETWEEN
//! C        BY
//! C        COUNT
//! C        DESC
//! C        DISTINCT
//! C        EQ
//! C        FROM
//! C        GE
//! C        GROUP
//! C        GT
//! C        HAVING
//! C        IS
//! C        LE
//! C        LT
//! C        LIKE
//! C        MAX
//! C        MIN
//! C        NE
//! C        NOT
//! C        NULL
//! C        OR
//! C        ORDER
//! C        SELECT
//! C        SUM
//! C        WHERE
//! C
//!  
//!       INTEGER               KWLEN
//!       PARAMETER           ( KWLEN  =   32 )
//!  
//!       INTEGER               NKEYWD
//!       PARAMETER           ( NKEYWD =   29 )
//!  
//!  
//!       INTEGER               KWALL
//!       PARAMETER           ( KWALL  =          1 )
//!  
//!       INTEGER               KWAND
//!       PARAMETER           ( KWAND  = KWALL  + 1 )
//!  
//!       INTEGER               KWASND
//!       PARAMETER           ( KWASND = KWAND  + 1 )
//!  
//!       INTEGER               KWAVG
//!       PARAMETER           ( KWAVG  = KWASND + 1 )
//!  
//!       INTEGER               KWBETW
//!       PARAMETER           ( KWBETW = KWAVG  + 1 )
//!  
//!       INTEGER               KWBY
//!       PARAMETER           ( KWBY   = KWBETW + 1 )
//!  
//!       INTEGER               KWCNT
//!       PARAMETER           ( KWCNT  = KWBY   + 1 )
//!  
//!       INTEGER               KWDSND
//!       PARAMETER           ( KWDSND = KWCNT  + 1 )
//!  
//!       INTEGER               KWDSTN
//!       PARAMETER           ( KWDSTN = KWDSND + 1 )
//!  
//!       INTEGER               KWEQ
//!       PARAMETER           ( KWEQ   = KWDSTN + 1 )
//!  
//!       INTEGER               KWFROM
//!       PARAMETER           ( KWFROM = KWEQ   + 1 )
//!  
//!       INTEGER               KWGE
//!       PARAMETER           ( KWGE   = KWFROM + 1 )
//!  
//!       INTEGER               KWGRP
//!       PARAMETER           ( KWGRP  = KWGE   + 1 )
//!  
//!       INTEGER               KWGT
//!       PARAMETER           ( KWGT   = KWGRP  + 1 )
//!  
//!       INTEGER               KWHAV
//!       PARAMETER           ( KWHAV  = KWGT   + 1 )
//!  
//!       INTEGER               KWIS
//!       PARAMETER           ( KWIS   = KWHAV  + 1 )
//!  
//!       INTEGER               KWLE
//!       PARAMETER           ( KWLE   = KWIS   + 1 )
//!  
//!       INTEGER               KWLIKE
//!       PARAMETER           ( KWLIKE = KWLE   + 1 )
//!  
//!       INTEGER               KWLT
//!       PARAMETER           ( KWLT   = KWLIKE + 1 )
//!  
//!       INTEGER               KWMAX
//!       PARAMETER           ( KWMAX  = KWLT   + 1 )
//!  
//!       INTEGER               KWMIN
//!       PARAMETER           ( KWMIN  = KWMAX  + 1 )
//!  
//!       INTEGER               KWNE
//!       PARAMETER           ( KWNE   = KWMIN  + 1 )
//!  
//!       INTEGER               KWNOT
//!       PARAMETER           ( KWNOT  = KWNE   + 1 )
//!  
//!       INTEGER               KWNULL
//!       PARAMETER           ( KWNULL = KWNOT  + 1 )
//!  
//!       INTEGER               KWOR
//!       PARAMETER           ( KWOR   = KWNULL + 1 )
//!  
//!       INTEGER               KWORDR
//!       PARAMETER           ( KWORDR = KWOR   + 1 )
//!  
//!       INTEGER               KWSEL
//!       PARAMETER           ( KWSEL  = KWORDR + 1 )
//!  
//!       INTEGER               KWSUM
//!       PARAMETER           ( KWSUM  = KWSEL  + 1 )
//!  
//!       INTEGER               KWWHER
//!       PARAMETER           ( KWWHER = KWSUM  + 1 )
//!  
//! C
//! C     End Include Section:  EK Keyword Code Parameters
//! C
//! ```

pub const KWLEN: i32 = 32;
pub const NKEYWD: i32 = 29;
pub const KWALL: i32 = 1;
pub const KWAND: i32 = (KWALL + 1);
pub const KWASND: i32 = (KWAND + 1);
pub const KWAVG: i32 = (KWASND + 1);
pub const KWBETW: i32 = (KWAVG + 1);
pub const KWBY: i32 = (KWBETW + 1);
pub const KWCNT: i32 = (KWBY + 1);
pub const KWDSND: i32 = (KWCNT + 1);
pub const KWDSTN: i32 = (KWDSND + 1);
pub const KWEQ: i32 = (KWDSTN + 1);
pub const KWFROM: i32 = (KWEQ + 1);
pub const KWGE: i32 = (KWFROM + 1);
pub const KWGRP: i32 = (KWGE + 1);
pub const KWGT: i32 = (KWGRP + 1);
pub const KWHAV: i32 = (KWGT + 1);
pub const KWIS: i32 = (KWHAV + 1);
pub const KWLE: i32 = (KWIS + 1);
pub const KWLIKE: i32 = (KWLE + 1);
pub const KWLT: i32 = (KWLIKE + 1);
pub const KWMAX: i32 = (KWLT + 1);
pub const KWMIN: i32 = (KWMAX + 1);
pub const KWNE: i32 = (KWMIN + 1);
pub const KWNOT: i32 = (KWNE + 1);
pub const KWNULL: i32 = (KWNOT + 1);
pub const KWOR: i32 = (KWNULL + 1);
pub const KWORDR: i32 = (KWOR + 1);
pub const KWSEL: i32 = (KWORDR + 1);
pub const KWSUM: i32 = (KWSEL + 1);
pub const KWWHER: i32 = (KWSUM + 1);
