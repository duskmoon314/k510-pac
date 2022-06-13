#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x08 - real-time counter register\n\nA constant frequency real-time counter. A timer interrupt is generated when mtime >= mtimecmp"]
    pub mtime: crate::Reg<mtime::MTIME_SPEC>,
    #[doc = "0x08..0x18 - stores a 64-bit value for comparing with mtime for generate core\\[i\\]
timer interrupt\n\nstores a 64-bit value for comparing with mtime for generate core\\[i\\]
timer interrupt when mtime >= mtimecmp\\[i\\]"]
    pub mtimecmp: [crate::Reg<mtimecmp::MTIMECMP_SPEC>; 2],
}
#[doc = "MTIME register accessor: an alias for `Reg<MTIME_SPEC>`"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "real-time counter register\n\nA constant frequency real-time counter. A timer interrupt is generated when mtime >= mtimecmp"]
pub mod mtime;
#[doc = "MTIMECMP register accessor: an alias for `Reg<MTIMECMP_SPEC>`"]
pub type MTIMECMP = crate::Reg<mtimecmp::MTIMECMP_SPEC>;
#[doc = "stores a 64-bit value for comparing with mtime for generate core\\[i\\]
timer interrupt\n\nstores a 64-bit value for comparing with mtime for generate core\\[i\\]
timer interrupt when mtime >= mtimecmp\\[i\\]"]
pub mod mtimecmp;
