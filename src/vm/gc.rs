use std::any::Any;
use crate::vm::obj::Gc;
use crate::vm::vm::VM;
use std::io::Write;

impl<W: Write> VM<W> {
    pub fn alloc<T: Any>(&mut self, obj: T) -> Gc<T> {
        // if self.should_collect() {
        //     self.collect_garbage();
        // }
        //
        // let size = mem::size_of::<T>();
        // self.total_allocations += size;
        //
        let ptr = Gc::new(obj);
        // self.objects.push(ptr.as_any());
        //
        // #[cfg(feature = "trace-gc")]
        // log::debug!(
        //     "{:p} allocate {} bytes for {}",
        //     ptr,
        //     size,
        //     std::any::type_name::<T>()
        // ); FIXME TODO

        ptr
    }
}