//! Generational read/write barrier implementations.

use crate::plan::barriers::BarrierSemantics;
use crate::plan::PlanTraceObject;
use crate::plan::VectorQueue;
use crate::policy::gc_work::DEFAULT_TRACE;
use crate::scheduler::WorkBucketStage;
use crate::util::constants::BYTES_IN_INT;
use crate::util::*;
use crate::vm::slot::MemorySlice;
use crate::vm::VMBinding;
use crate::MMTK;


//for now leave it empty and check if nothing crashes
pub struct RCBarrierSemantics<VM: VMBinding>
{
    mmtk: &'static MMTK<VM>,
}

impl<VM: VMBinding> RCBarrierSemantics<VM>
{
    pub fn new(mmtk: &'static MMTK<VM>) -> Self {
        return RCBarrierSemantics::<VM>{mmtk};
    }

    fn flush_modbuf(&mut self) {

    }

    fn flush_region_modbuf(&mut self) {

    }
}
//impliment!
impl<VM: VMBinding> BarrierSemantics for RCBarrierSemantics<VM>
{
    type VM = VM;



    /// Flush thread-local buffers or remembered sets.
    /// Normally this is called by the slow-path implementation whenever the thread-local buffers are full.
    /// This will also be called externally by the VM, when the thread is being destroyed.
    fn flush(&mut self){}

    /// Slow-path call for object field write operations.
    fn object_reference_write_slow(
        &mut self,
        src: ObjectReference,
        slot: <Self::VM as VMBinding>::VMSlot,
        target: Option<ObjectReference>,
    ){
        
    }

    /// Slow-path call for mempry slice copy operations. For example, array-copy operations.
    fn memory_region_copy_slow(
        &mut self,
        src: <Self::VM as VMBinding>::VMMemorySlice,
        dst: <Self::VM as VMBinding>::VMMemorySlice,
    ){

    }

    /// Object will probably be modified
    /// ignor this for now
    fn object_probable_write_slow(&mut self, _obj: ObjectReference) {}
}
