#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Feature Enable Register"]
    pub feature_enable: crate::Reg<feature_enable::FEATURE_ENABLE_SPEC>,
    #[doc = "0x04..0x200 - Priority Register for Source\\[i\\]"]
    pub source_priority: [crate::Reg<source_priority::SOURCE_PRIORITY_SPEC>; 127],
    _reserved2: [u8; 0x0e00],
    #[doc = "0x1000..0x1010 - Souce\\[i:i+31\\]
interrupt pending"]
    pub pending: [crate::Reg<pending::PENDING_SPEC>; 4],
    _reserved3: [u8; 0x70],
    #[doc = "0x1080..0x1090 - Souce\\[i:i+31\\]
interrupt trigger type"]
    pub trigger: [crate::Reg<trigger::TRIGGER_SPEC>; 4],
    _reserved4: [u8; 0x70],
    #[doc = "0x1100 - Number of interrupt and target configuration register"]
    pub number_interrupt_target: crate::Reg<number_interrupt_target::NUMBER_INTERRUPT_TARGET_SPEC>,
    #[doc = "0x1104 - Version and Maximum priority configuration register"]
    pub version_max_priority_configuration:
        crate::Reg<version_max_priority_configuration::VERSION_MAX_PRIORITY_CONFIGURATION_SPEC>,
    _reserved6: [u8; 0x0ef8],
    #[doc = "0x2000 - Target\\[M\\]
enable registers"]
    pub target_enable: crate::ArrayProxy<TARGET_ENABLE, 4, 0x80>,
    _reserved7: [u8; 0x001f_e000],
    #[doc = "0x200000 - Target\\[M\\]'s context: priority threshold, claim and complete, preempted priority"]
    pub context: crate::ArrayProxy<CONTEXT, 4, 0x1000>,
}
impl RegisterBlock {
    #[doc = "0x04 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source1_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[0]
    }
    #[doc = "0x08 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source2_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[1]
    }
    #[doc = "0x0c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source3_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[2]
    }
    #[doc = "0x10 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source4_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[3]
    }
    #[doc = "0x14 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source5_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[4]
    }
    #[doc = "0x18 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source6_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[5]
    }
    #[doc = "0x1c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source7_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[6]
    }
    #[doc = "0x20 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source8_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[7]
    }
    #[doc = "0x24 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source9_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[8]
    }
    #[doc = "0x28 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source10_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[9]
    }
    #[doc = "0x2c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source11_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[10]
    }
    #[doc = "0x30 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source12_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[11]
    }
    #[doc = "0x34 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source13_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[12]
    }
    #[doc = "0x38 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source14_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[13]
    }
    #[doc = "0x3c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source15_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[14]
    }
    #[doc = "0x40 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source16_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[15]
    }
    #[doc = "0x44 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source17_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[16]
    }
    #[doc = "0x48 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source18_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[17]
    }
    #[doc = "0x4c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source19_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[18]
    }
    #[doc = "0x50 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source20_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[19]
    }
    #[doc = "0x54 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source21_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[20]
    }
    #[doc = "0x58 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source22_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[21]
    }
    #[doc = "0x5c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source23_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[22]
    }
    #[doc = "0x60 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source24_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[23]
    }
    #[doc = "0x64 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source25_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[24]
    }
    #[doc = "0x68 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source26_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[25]
    }
    #[doc = "0x6c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source27_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[26]
    }
    #[doc = "0x70 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source28_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[27]
    }
    #[doc = "0x74 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source29_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[28]
    }
    #[doc = "0x78 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source30_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[29]
    }
    #[doc = "0x7c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source31_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[30]
    }
    #[doc = "0x80 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source32_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[31]
    }
    #[doc = "0x84 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source33_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[32]
    }
    #[doc = "0x88 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source34_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[33]
    }
    #[doc = "0x8c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source35_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[34]
    }
    #[doc = "0x90 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source36_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[35]
    }
    #[doc = "0x94 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source37_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[36]
    }
    #[doc = "0x98 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source38_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[37]
    }
    #[doc = "0x9c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source39_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[38]
    }
    #[doc = "0xa0 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source40_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[39]
    }
    #[doc = "0xa4 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source41_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[40]
    }
    #[doc = "0xa8 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source42_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[41]
    }
    #[doc = "0xac - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source43_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[42]
    }
    #[doc = "0xb0 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source44_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[43]
    }
    #[doc = "0xb4 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source45_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[44]
    }
    #[doc = "0xb8 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source46_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[45]
    }
    #[doc = "0xbc - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source47_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[46]
    }
    #[doc = "0xc0 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source48_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[47]
    }
    #[doc = "0xc4 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source49_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[48]
    }
    #[doc = "0xc8 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source50_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[49]
    }
    #[doc = "0xcc - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source51_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[50]
    }
    #[doc = "0xd0 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source52_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[51]
    }
    #[doc = "0xd4 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source53_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[52]
    }
    #[doc = "0xd8 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source54_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[53]
    }
    #[doc = "0xdc - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source55_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[54]
    }
    #[doc = "0xe0 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source56_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[55]
    }
    #[doc = "0xe4 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source57_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[56]
    }
    #[doc = "0xe8 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source58_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[57]
    }
    #[doc = "0xec - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source59_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[58]
    }
    #[doc = "0xf0 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source60_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[59]
    }
    #[doc = "0xf4 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source61_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[60]
    }
    #[doc = "0xf8 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source62_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[61]
    }
    #[doc = "0xfc - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source63_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[62]
    }
    #[doc = "0x100 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source64_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[63]
    }
    #[doc = "0x104 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source65_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[64]
    }
    #[doc = "0x108 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source66_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[65]
    }
    #[doc = "0x10c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source67_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[66]
    }
    #[doc = "0x110 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source68_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[67]
    }
    #[doc = "0x114 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source69_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[68]
    }
    #[doc = "0x118 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source70_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[69]
    }
    #[doc = "0x11c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source71_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[70]
    }
    #[doc = "0x120 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source72_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[71]
    }
    #[doc = "0x124 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source73_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[72]
    }
    #[doc = "0x128 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source74_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[73]
    }
    #[doc = "0x12c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source75_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[74]
    }
    #[doc = "0x130 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source76_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[75]
    }
    #[doc = "0x134 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source77_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[76]
    }
    #[doc = "0x138 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source78_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[77]
    }
    #[doc = "0x13c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source79_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[78]
    }
    #[doc = "0x140 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source80_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[79]
    }
    #[doc = "0x144 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source81_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[80]
    }
    #[doc = "0x148 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source82_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[81]
    }
    #[doc = "0x14c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source83_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[82]
    }
    #[doc = "0x150 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source84_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[83]
    }
    #[doc = "0x154 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source85_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[84]
    }
    #[doc = "0x158 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source86_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[85]
    }
    #[doc = "0x15c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source87_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[86]
    }
    #[doc = "0x160 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source88_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[87]
    }
    #[doc = "0x164 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source89_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[88]
    }
    #[doc = "0x168 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source90_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[89]
    }
    #[doc = "0x16c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source91_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[90]
    }
    #[doc = "0x170 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source92_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[91]
    }
    #[doc = "0x174 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source93_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[92]
    }
    #[doc = "0x178 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source94_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[93]
    }
    #[doc = "0x17c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source95_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[94]
    }
    #[doc = "0x180 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source96_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[95]
    }
    #[doc = "0x184 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source97_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[96]
    }
    #[doc = "0x188 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source98_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[97]
    }
    #[doc = "0x18c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source99_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[98]
    }
    #[doc = "0x190 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source100_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[99]
    }
    #[doc = "0x194 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source101_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[100]
    }
    #[doc = "0x198 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source102_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[101]
    }
    #[doc = "0x19c - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source103_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[102]
    }
    #[doc = "0x1a0 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source104_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[103]
    }
    #[doc = "0x1a4 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source105_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[104]
    }
    #[doc = "0x1a8 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source106_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[105]
    }
    #[doc = "0x1ac - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source107_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[106]
    }
    #[doc = "0x1b0 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source108_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[107]
    }
    #[doc = "0x1b4 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source109_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[108]
    }
    #[doc = "0x1b8 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source110_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[109]
    }
    #[doc = "0x1bc - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source111_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[110]
    }
    #[doc = "0x1c0 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source112_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[111]
    }
    #[doc = "0x1c4 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source113_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[112]
    }
    #[doc = "0x1c8 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source114_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[113]
    }
    #[doc = "0x1cc - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source115_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[114]
    }
    #[doc = "0x1d0 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source116_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[115]
    }
    #[doc = "0x1d4 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source117_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[116]
    }
    #[doc = "0x1d8 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source118_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[117]
    }
    #[doc = "0x1dc - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source119_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[118]
    }
    #[doc = "0x1e0 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source120_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[119]
    }
    #[doc = "0x1e4 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source121_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[120]
    }
    #[doc = "0x1e8 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source122_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[121]
    }
    #[doc = "0x1ec - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source123_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[122]
    }
    #[doc = "0x1f0 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source124_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[123]
    }
    #[doc = "0x1f4 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source125_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[124]
    }
    #[doc = "0x1f8 - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source126_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[125]
    }
    #[doc = "0x1fc - Priority Register for Source\\[i\\]"]
    #[inline(always)]
    pub fn source127_priority(&self) -> &crate::Reg<source_priority::SOURCE_PRIORITY_SPEC> {
        &self.source_priority[126]
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TARGET_ENABLE {
    #[doc = "0x00..0x10 - Target\\[i:i+31\\]
enable registers"]
    pub enable: [crate::Reg<self::target_enable::enable::ENABLE_SPEC>; 4],
}
#[doc = r"Register block"]
#[doc = "Target\\[M\\]
enable registers"]
pub mod target_enable;
#[doc = r"Register block"]
#[repr(C)]
pub struct CONTEXT {
    #[doc = "0x00 - Target\\[M\\]'s priority threshold register"]
    pub threshold: crate::Reg<self::context::threshold::THRESHOLD_SPEC>,
    #[doc = "0x04 - Target\\[M\\]'s claim and complete register"]
    pub claim_complete: crate::Reg<self::context::claim_complete::CLAIM_COMPLETE_SPEC>,
    #[doc = "0x08 - Target\\[M\\]'s preempted priority register\n\nprovide the preempted priority stack information for each target"]
    pub preempted_priority: crate::Reg<self::context::preempted_priority::PREEMPTED_PRIORITY_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Target\\[M\\]'s context: priority threshold, claim and complete, preempted priority"]
pub mod context;
#[doc = "FEATURE_ENABLE register accessor: an alias for `Reg<FEATURE_ENABLE_SPEC>`"]
pub type FEATURE_ENABLE = crate::Reg<feature_enable::FEATURE_ENABLE_SPEC>;
#[doc = "Feature Enable Register"]
pub mod feature_enable;
#[doc = "SOURCE_PRIORITY register accessor: an alias for `Reg<SOURCE_PRIORITY_SPEC>`"]
pub type SOURCE_PRIORITY = crate::Reg<source_priority::SOURCE_PRIORITY_SPEC>;
#[doc = "Priority Register for Source\\[i\\]"]
pub mod source_priority;
#[doc = "PENDING register accessor: an alias for `Reg<PENDING_SPEC>`"]
pub type PENDING = crate::Reg<pending::PENDING_SPEC>;
#[doc = "Souce\\[i:i+31\\]
interrupt pending"]
pub mod pending;
#[doc = "TRIGGER register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "Souce\\[i:i+31\\]
interrupt trigger type"]
pub mod trigger;
#[doc = "NUMBER_INTERRUPT_TARGET register accessor: an alias for `Reg<NUMBER_INTERRUPT_TARGET_SPEC>`"]
pub type NUMBER_INTERRUPT_TARGET =
    crate::Reg<number_interrupt_target::NUMBER_INTERRUPT_TARGET_SPEC>;
#[doc = "Number of interrupt and target configuration register"]
pub mod number_interrupt_target;
#[doc = "VERSION_MAX_PRIORITY_CONFIGURATION register accessor: an alias for `Reg<VERSION_MAX_PRIORITY_CONFIGURATION_SPEC>`"]
pub type VERSION_MAX_PRIORITY_CONFIGURATION =
    crate::Reg<version_max_priority_configuration::VERSION_MAX_PRIORITY_CONFIGURATION_SPEC>;
#[doc = "Version and Maximum priority configuration register"]
pub mod version_max_priority_configuration;
