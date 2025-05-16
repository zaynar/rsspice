KPL/MK

   The names and contents of the kernels referenced by this
   meta-kernel are as follows:


   File Name                   Description
   --------------------------  ----------------------------------
   naif0008.tls                Generic LSK.
   cas00084.tsc                Cassini SCLK.
   020514_SE_SAT105.bsp        Saturnian Satellite Ephemeris SPK.
   030201AP_SK_SM546_T45.bsp   Cassini Spacecraft SPK.
   981005_PLTEPH-DE405S.bsp    Planetary Ephemeris SPK.
   04135_04171pc_psiv2.bc      Cassini Spacecraft CK.
   cas_v37.tf                  Cassini FK.
   cpck05Mar2004.tpc           Cassini project PCK.


\begindata
   KERNELS_TO_LOAD = (
                     'kernels/lsk/naif0008.tls'
                     'kernels/sclk/cas00084.tsc'
                     'kernels/spk/020514_SE_SAT105.bsp'
                     'kernels/spk/030201AP_SK_SM546_T45.bsp'
                     'kernels/spk/981005_PLTEPH-DE405S.bsp'
                     'kernels/ck/04135_04171pc_psiv2.bc'
                     'kernels/fk/cas_v37.tf'
                     'kernels/pck/cpck05Mar2004.tpc'
                     )
\begintext
