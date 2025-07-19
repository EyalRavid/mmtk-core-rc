use crate::plan::global::BasePlan;
use crate::plan::global::CreateGeneralPlanArgs;
use crate::plan::global::CreateSpecificPlanArgs;
use crate::plan::rc::mutator::ALLOCATOR_MAPPING;
use crate::plan::AllocationSemantics;
use crate::plan::Plan;
use crate::plan::PlanConstraints;
use crate::policy::space::Space;
use crate::scheduler::GCWorkScheduler;
use crate::util::alloc::allocators::AllocatorSelector;
use crate::util::heap::gc_trigger::SpaceStats;
#[allow(unused_imports)]
use crate::util::heap::VMRequest;
use crate::util::metadata::side_metadata::SideMetadataContext;
use crate::util::opaque_pointer::*;
use crate::vm::VMBinding;
use enum_map::EnumMap;
use mmtk_macros::HasSpaces;
use crate::policy::immix::{ImmixSpace, ImmixSpaceArgs};
use crate::plan::global::CommonPlan;
use crate::BarrierSelector;
//use crate::util::metadata::side_metadata::spec_defs::RC_TABLE;

#[derive(HasSpaces)]
pub struct RC<VM: VMBinding> {
    #[space]
    pub immix_space: ImmixSpace<VM>,
    #[parent]
    pub common: CommonPlan<VM>,
}

/// The plan constraints for the RC plan.
pub const RC_CONSTRAINTS: PlanConstraints = PlanConstraints {
    // Max immix object size is half of a block.
    max_non_los_default_alloc_bytes: crate::policy::immix::MAX_IMMIX_OBJECT_SIZE,
    barrier: BarrierSelector::ObjectPreBarrier,
    needs_log_bit: true,
    ..PlanConstraints::default()
};


impl<VM: VMBinding> Plan for RC<VM> {
    fn constraints(&self) -> &'static PlanConstraints {
        &RC_CONSTRAINTS
    }

    fn collection_required(&self, space_full: bool, _space: Option<SpaceStats<Self::VM>>) -> bool {
        self.base().collection_required(self, space_full)
    }

    fn common(&self) -> &CommonPlan<VM> {
        &self.common
    }

    fn base(&self) -> &BasePlan<VM> {
        &self.common.base
    }

    fn base_mut(&mut self) -> &mut BasePlan<Self::VM> {
        &mut self.common.base
    }

    fn prepare(&mut self, _tls: VMWorkerThread) {
        unreachable!()
    }

    fn release(&mut self, _tls: VMWorkerThread) {
        unreachable!()
    }

    fn end_of_gc(&mut self, _tls: VMWorkerThread) {
        unreachable!()
    }

    fn get_allocator_mapping(&self) -> &'static EnumMap<AllocationSemantics, AllocatorSelector> {
        &ALLOCATOR_MAPPING
    }

    fn schedule_collection(&'static self, _scheduler: &GCWorkScheduler<VM>) {
        unreachable!("GC triggered in rc")
    }

    fn current_gc_may_move_object(&self) -> bool {
        false
    }

    fn get_collection_reserved_pages(&self) -> usize {
        self.immix_space.defrag_headroom_pages()
    }

    fn get_used_pages(&self) -> usize {
        self.immix_space.reserved_pages() + self.common.get_used_pages()
    }
}

impl<VM: VMBinding> RC<VM> {
    
    pub fn new(args: CreateGeneralPlanArgs<VM>) -> Self {

        //let global_side_metadata_specs = SideMetadataContext::new_global_specs(&[RC_TABLE]);

        let mut plan_args = CreateSpecificPlanArgs {
            global_args: args,
            constraints: &RC_CONSTRAINTS,
            global_side_metadata_specs: SideMetadataContext::new_global_specs(&[]),
        };

        let res = RC {
            immix_space: ImmixSpace::new(
                plan_args.get_space_args("immix", true, false, VMRequest::discontiguous()),
                ImmixSpaceArgs {
                    unlog_object_when_traced: false,
                    #[cfg(feature = "vo_bit")]
                    mixed_age: false,
                    never_move_objects: true,
                },
            ),
            common: CommonPlan::new(plan_args)
        };

        res.verify_side_metadata_sanity();

        res
    }
}
