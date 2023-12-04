### rafos-executor

This crate based on [Embassy](https://embassy.dev/) provides the structures related to rust async runtime.

- Task
- Executor
- Waker

#### Task

The `Task` structure is the Coroutine Control Block in the whole system. It's space is allocated by `Heap Allocator`. The space in the heap is recycled according to the `Rust` `ownership` mechenism.

##### new
When spawning a new `Task`, we will wrap it with `Arc` to ensure it will be allocated in heap.

##### drop

When the execution of `Task` returns `Ready`, it's time to drop the `Task`. The drop operation is ensured by the `Rust` `ownership`.

#### Executor

This structure will only hold the `TaskRef` which is the raw pointer of `Task`. `TaskRef` is the actually position the `Task` stored.

When a `Task` is spawned, The `Arc<Task>` will transmute into `TaskRef`, so the Rust code will not drop it. When fetching a task from `Executor`, it will return the `TaskRef`. When the task is executing, it will transmute into `Arc<Task>` to ensure the Rust ownership mechenism can work correctly.

#### Waker

We customized a Waker from [Embassy](https://embassy.dev/). The Waker in the Rust code must be generated from this crate, otherwise, undefined errors will occur.

When waking a `Task`, we can get the position of `Executor` according to the `TaskRef` argument. Then we can push the `TaskRef` into the `Executor` to waker the target `Task`. 
- Waking kernel `Task` in kernel, we can directly do this operation. 
- Waking user `Task` in user space, just like the first.
- Waking user `Task` in kernel, we must translate the virtual address of  `TaskRef` and `Executor` into the physical address, then we directly write the physical address to waker the task.