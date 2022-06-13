#[doc = "THRESHOLD register accessor: an alias for `Reg<THRESHOLD_SPEC>`"]
pub type THRESHOLD = crate::Reg<threshold::THRESHOLD_SPEC>;
#[doc = "Target\\[M\\]'s priority threshold register"]
pub mod threshold;
#[doc = "CLAIM_COMPLETE register accessor: an alias for `Reg<CLAIM_COMPLETE_SPEC>`"]
pub type CLAIM_COMPLETE = crate::Reg<claim_complete::CLAIM_COMPLETE_SPEC>;
#[doc = "Target\\[M\\]'s claim and complete register"]
pub mod claim_complete;
#[doc = "PREEMPTED_PRIORITY register accessor: an alias for `Reg<PREEMPTED_PRIORITY_SPEC>`"]
pub type PREEMPTED_PRIORITY = crate::Reg<preempted_priority::PREEMPTED_PRIORITY_SPEC>;
#[doc = "Target\\[M\\]'s preempted priority register\n\nprovide the preempted priority stack information for each target"]
pub mod preempted_priority;
