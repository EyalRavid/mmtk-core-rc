use crate::plan::mutator_context::unreachable_prepare_func;
use crate::plan::mutator_context::unreachable_release_func;
use crate::plan::mutator_context::Mutator;
use crate::plan::mutator_context::MutatorBuilder;
use crate::plan::mutator_context::MutatorConfig;
use crate::plan::mutator_context::{
    create_allocator_mapping, create_space_mapping, ReservedAllocators,
};
use crate::plan::AllocationSemantics;
use crate::util::alloc::allocators::AllocatorSelector;
use crate::util::VMMutatorThread;
use crate::vm::VMBinding;
use crate::MMTK;
use super::RC;
use enum_map::{enum_map, EnumMap};
use crate::plan::barriers::ObjectPreBarrier;
use crate::plan::rc::barrier::RCBarrierSemantics;
use crate::plan::barriers::ObjectBarrier;

const RESERVED_ALLOCATORS: ReservedAllocators = ReservedAllocators {
    n_immix: 1,
    ..ReservedAllocators::DEFAULT
};

lazy_static! {
    pub static ref ALLOCATOR_MAPPING: EnumMap<AllocationSemantics, AllocatorSelector> = {
        let mut map = create_allocator_mapping(RESERVED_ALLOCATORS, true);
        map[AllocationSemantics::Default] = AllocatorSelector::Immix(0);
        map
    };
}

pub fn create_rc_mutator<VM: VMBinding>(
    mutator_tls: VMMutatorThread,
    mmtk: &'static MMTK<VM>,
) -> Mutator<VM> {
    let plan = mmtk.get_plan().downcast_ref::<RC<VM>>().unwrap();
    let config = MutatorConfig {
        allocator_mapping: &ALLOCATOR_MAPPING,
        space_mapping: Box::new({
            let mut vec = create_space_mapping(RESERVED_ALLOCATORS, true, plan);
            vec.push((AllocatorSelector::Immix(0), &plan.immix_space));
            vec
        }),
        prepare_func: &unreachable_prepare_func,
        release_func: &unreachable_release_func,
    };

    let builder = MutatorBuilder::new(mutator_tls, mmtk, config);
    builder
    .barrier(Box::new(ObjectPreBarrier::new(
        RCBarrierSemantics::new(mmtk),
    )))
    .build()
    
    //builder.build()
}
