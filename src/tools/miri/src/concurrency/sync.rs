use std::collections::{hash_map::Entry, VecDeque};
use std::ops::Not;
use std::time::Duration;

use rustc_data_structures::fx::FxHashMap;
use rustc_index::{Idx, IndexVec};
use rustc_middle::ty::layout::TyAndLayout;

use super::init_once::InitOnce;
use super::vector_clock::VClock;
use crate::*;

pub trait SyncId {
    fn from_u32(id: u32) -> Self;
    fn to_u32(&self) -> u32;
}

/// We cannot use the `newtype_index!` macro because we have to use 0 as a
/// sentinel value meaning that the identifier is not assigned. This is because
/// the pthreads static initializers initialize memory with zeros (see the
/// `src/shims/sync.rs` file).
macro_rules! declare_id {
    ($name: ident) => {
        /// 0 is used to indicate that the id was not yet assigned and,
        /// therefore, is not a valid identifier.
        #[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
        pub struct $name(std::num::NonZero<u32>);

        impl $crate::concurrency::sync::SyncId for $name {
            // Panics if `id == 0`.
            fn from_u32(id: u32) -> Self {
                Self(std::num::NonZero::new(id).unwrap())
            }
            fn to_u32(&self) -> u32 {
                self.0.get()
            }
        }

        impl $crate::VisitProvenance for $name {
            fn visit_provenance(&self, _visit: &mut VisitWith<'_>) {}
        }

        impl Idx for $name {
            fn new(idx: usize) -> Self {
                // We use 0 as a sentinel value (see the comment above) and,
                // therefore, need to shift by one when converting from an index
                // into a vector.
                let shifted_idx = u32::try_from(idx).unwrap().checked_add(1).unwrap();
                $name(std::num::NonZero::new(shifted_idx).unwrap())
            }
            fn index(self) -> usize {
                // See the comment in `Self::new`.
                // (This cannot underflow because `self.0` is `NonZero<u32>`.)
                usize::try_from(self.0.get() - 1).unwrap()
            }
        }

        impl $name {
            pub fn to_u32_scalar(&self) -> Scalar {
                Scalar::from_u32(self.0.get())
            }
        }
    };
}
pub(super) use declare_id;

declare_id!(MutexId);

/// The mutex state.
#[derive(Default, Debug)]
struct Mutex {
    /// The thread that currently owns the lock.
    owner: Option<ThreadId>,
    /// How many times the mutex was locked by the owner.
    lock_count: usize,
    /// The queue of threads waiting for this mutex.
    queue: VecDeque<ThreadId>,
    /// Mutex clock. This tracks the moment of the last unlock.
    clock: VClock,
}

declare_id!(RwLockId);

/// The read-write lock state.
#[derive(Default, Debug)]
struct RwLock {
    /// The writer thread that currently owns the lock.
    writer: Option<ThreadId>,
    /// The readers that currently own the lock and how many times they acquired
    /// the lock.
    readers: FxHashMap<ThreadId, usize>,
    /// The queue of writer threads waiting for this lock.
    writer_queue: VecDeque<ThreadId>,
    /// The queue of reader threads waiting for this lock.
    reader_queue: VecDeque<ThreadId>,
    /// Data race clock for writers. Tracks the happens-before
    /// ordering between each write access to a rwlock and is updated
    /// after a sequence of concurrent readers to track the happens-
    /// before ordering between the set of previous readers and
    /// the current writer.
    /// Contains the clock of the last thread to release a writer
    /// lock or the joined clock of the set of last threads to release
    /// shared reader locks.
    clock_unlocked: VClock,
    /// Data race clock for readers. This is temporary storage
    /// for the combined happens-before ordering for between all
    /// concurrent readers and the next writer, and the value
    /// is stored to the main data_race variable once all
    /// readers are finished.
    /// Has to be stored separately since reader lock acquires
    /// must load the clock of the last write and must not
    /// add happens-before orderings between shared reader
    /// locks.
    /// This is only relevant when there is an active reader.
    clock_current_readers: VClock,
}

declare_id!(CondvarId);

/// The conditional variable state.
#[derive(Default, Debug)]
struct Condvar {
    waiters: VecDeque<ThreadId>,
    /// Tracks the happens-before relationship
    /// between a cond-var signal and a cond-var
    /// wait during a non-spurious signal event.
    /// Contains the clock of the last thread to
    /// perform a condvar-signal.
    clock: VClock,
}

/// The futex state.
#[derive(Default, Debug)]
struct Futex {
    waiters: VecDeque<FutexWaiter>,
    /// Tracks the happens-before relationship
    /// between a futex-wake and a futex-wait
    /// during a non-spurious wake event.
    /// Contains the clock of the last thread to
    /// perform a futex-wake.
    clock: VClock,
}

/// A thread waiting on a futex.
#[derive(Debug)]
struct FutexWaiter {
    /// The thread that is waiting on this futex.
    thread: ThreadId,
    /// The bitset used by FUTEX_*_BITSET, or u32::MAX for other operations.
    bitset: u32,
}

/// The state of all synchronization objects.
#[derive(Default, Debug)]
pub struct SynchronizationObjects {
    mutexes: IndexVec<MutexId, Mutex>,
    rwlocks: IndexVec<RwLockId, RwLock>,
    condvars: IndexVec<CondvarId, Condvar>,
    futexes: FxHashMap<u64, Futex>,
    pub(super) init_onces: IndexVec<InitOnceId, InitOnce>,
}

// Private extension trait for local helper methods
impl<'tcx> EvalContextExtPriv<'tcx> for crate::MiriInterpCx<'tcx> {}
pub(super) trait EvalContextExtPriv<'tcx>: crate::MiriInterpCxExt<'tcx> {
    /// Lazily initialize the ID of this Miri sync structure.
    /// ('0' indicates uninit.)
    #[inline]
    fn get_or_create_id<Id: SyncId>(
        &mut self,
        next_id: Id,
        lock_op: &OpTy<'tcx>,
        lock_layout: TyAndLayout<'tcx>,
        offset: u64,
    ) -> InterpResult<'tcx, Option<Id>> {
        let this = self.eval_context_mut();
        let value_place =
            this.deref_pointer_and_offset(lock_op, offset, lock_layout, this.machine.layouts.u32)?;

        // Since we are lazy, this update has to be atomic.
        let (old, success) = this
            .atomic_compare_exchange_scalar(
                &value_place,
                &ImmTy::from_uint(0u32, this.machine.layouts.u32),
                Scalar::from_u32(next_id.to_u32()),
                AtomicRwOrd::Relaxed, // deliberately *no* synchronization
                AtomicReadOrd::Relaxed,
                false,
            )?
            .to_scalar_pair();

        Ok(if success.to_bool().expect("compare_exchange's second return value is a bool") {
            // Caller of the closure needs to allocate next_id
            None
        } else {
            Some(Id::from_u32(old.to_u32().expect("layout is u32")))
        })
    }

    /// Provides the closure with the next MutexId. Creates that mutex if the closure returns None,
    /// otherwise returns the value from the closure.
    #[inline]
    fn mutex_get_or_create<F>(&mut self, existing: F) -> InterpResult<'tcx, MutexId>
    where
        F: FnOnce(&mut MiriInterpCx<'tcx>, MutexId) -> InterpResult<'tcx, Option<MutexId>>,
    {
        let this = self.eval_context_mut();
        let next_index = this.machine.sync.mutexes.next_index();
        if let Some(old) = existing(this, next_index)? {
            if this.machine.sync.mutexes.get(old).is_none() {
                throw_ub_format!("mutex has invalid ID");
            }
            Ok(old)
        } else {
            let new_index = this.machine.sync.mutexes.push(Default::default());
            assert_eq!(next_index, new_index);
            Ok(new_index)
        }
    }

    /// Provides the closure with the next RwLockId. Creates that RwLock if the closure returns None,
    /// otherwise returns the value from the closure.
    #[inline]
    fn rwlock_get_or_create<F>(&mut self, existing: F) -> InterpResult<'tcx, RwLockId>
    where
        F: FnOnce(&mut MiriInterpCx<'tcx>, RwLockId) -> InterpResult<'tcx, Option<RwLockId>>,
    {
        let this = self.eval_context_mut();
        let next_index = this.machine.sync.rwlocks.next_index();
        if let Some(old) = existing(this, next_index)? {
            if this.machine.sync.rwlocks.get(old).is_none() {
                throw_ub_format!("rwlock has invalid ID");
            }
            Ok(old)
        } else {
            let new_index = this.machine.sync.rwlocks.push(Default::default());
            assert_eq!(next_index, new_index);
            Ok(new_index)
        }
    }

    /// Provides the closure with the next CondvarId. Creates that Condvar if the closure returns None,
    /// otherwise returns the value from the closure.
    #[inline]
    fn condvar_get_or_create<F>(&mut self, existing: F) -> InterpResult<'tcx, CondvarId>
    where
        F: FnOnce(&mut MiriInterpCx<'tcx>, CondvarId) -> InterpResult<'tcx, Option<CondvarId>>,
    {
        let this = self.eval_context_mut();
        let next_index = this.machine.sync.condvars.next_index();
        if let Some(old) = existing(this, next_index)? {
            if this.machine.sync.condvars.get(old).is_none() {
                throw_ub_format!("condvar has invalid ID");
            }
            Ok(old)
        } else {
            let new_index = this.machine.sync.condvars.push(Default::default());
            assert_eq!(next_index, new_index);
            Ok(new_index)
        }
    }

    fn condvar_reacquire_mutex(
        &mut self,
        mutex: MutexId,
        retval: Scalar,
        dest: MPlaceTy<'tcx>,
    ) -> InterpResult<'tcx> {
        let this = self.eval_context_mut();
        if this.mutex_is_locked(mutex) {
            assert_ne!(this.mutex_get_owner(mutex), this.active_thread());
            this.mutex_enqueue_and_block(mutex, retval, dest);
        } else {
            // We can have it right now!
            this.mutex_lock(mutex);
            // Don't forget to write the return value.
            this.write_scalar(retval, &dest)?;
        }
        Ok(())
    }
}

// Public interface to synchronization primitives. Please note that in most
// cases, the function calls are infallible and it is the client's (shim
// implementation's) responsibility to detect and deal with erroneous
// situations.
impl<'tcx> EvalContextExt<'tcx> for crate::MiriInterpCx<'tcx> {}
pub trait EvalContextExt<'tcx>: crate::MiriInterpCxExt<'tcx> {
    fn mutex_get_or_create_id(
        &mut self,
        lock_op: &OpTy<'tcx>,
        lock_layout: TyAndLayout<'tcx>,
        offset: u64,
    ) -> InterpResult<'tcx, MutexId> {
        let this = self.eval_context_mut();
        this.mutex_get_or_create(|ecx, next_id| {
            ecx.get_or_create_id(next_id, lock_op, lock_layout, offset)
        })
    }

    fn rwlock_get_or_create_id(
        &mut self,
        lock_op: &OpTy<'tcx>,
        lock_layout: TyAndLayout<'tcx>,
        offset: u64,
    ) -> InterpResult<'tcx, RwLockId> {
        let this = self.eval_context_mut();
        this.rwlock_get_or_create(|ecx, next_id| {
            ecx.get_or_create_id(next_id, lock_op, lock_layout, offset)
        })
    }

    fn condvar_get_or_create_id(
        &mut self,
        lock_op: &OpTy<'tcx>,
        lock_layout: TyAndLayout<'tcx>,
        offset: u64,
    ) -> InterpResult<'tcx, CondvarId> {
        let this = self.eval_context_mut();
        this.condvar_get_or_create(|ecx, next_id| {
            ecx.get_or_create_id(next_id, lock_op, lock_layout, offset)
        })
    }

    #[inline]
    /// Get the id of the thread that currently owns this lock.
    fn mutex_get_owner(&mut self, id: MutexId) -> ThreadId {
        let this = self.eval_context_ref();
        this.machine.sync.mutexes[id].owner.unwrap()
    }

    #[inline]
    /// Check if locked.
    fn mutex_is_locked(&self, id: MutexId) -> bool {
        let this = self.eval_context_ref();
        this.machine.sync.mutexes[id].owner.is_some()
    }

    /// Lock by setting the mutex owner and increasing the lock count.
    fn mutex_lock(&mut self, id: MutexId) {
        let this = self.eval_context_mut();
        let thread = this.active_thread();
        let mutex = &mut this.machine.sync.mutexes[id];
        if let Some(current_owner) = mutex.owner {
            assert_eq!(thread, current_owner, "mutex already locked by another thread");
            assert!(
                mutex.lock_count > 0,
                "invariant violation: lock_count == 0 iff the thread is unlocked"
            );
        } else {
            mutex.owner = Some(thread);
        }
        mutex.lock_count = mutex.lock_count.checked_add(1).unwrap();
        if let Some(data_race) = &this.machine.data_race {
            data_race.acquire_clock(&mutex.clock, &this.machine.threads);
        }
    }

    /// Try unlocking by decreasing the lock count and returning the old lock
    /// count. If the lock count reaches 0, release the lock and potentially
    /// give to a new owner. If the lock was not locked by the current thread,
    /// return `None`.
    fn mutex_unlock(&mut self, id: MutexId) -> InterpResult<'tcx, Option<usize>> {
        let this = self.eval_context_mut();
        let mutex = &mut this.machine.sync.mutexes[id];
        Ok(if let Some(current_owner) = mutex.owner {
            // Mutex is locked.
            if current_owner != this.machine.threads.active_thread() {
                // Only the owner can unlock the mutex.
                return Ok(None);
            }
            let old_lock_count = mutex.lock_count;
            mutex.lock_count = old_lock_count
                .checked_sub(1)
                .expect("invariant violation: lock_count == 0 iff the thread is unlocked");
            if mutex.lock_count == 0 {
                mutex.owner = None;
                // The mutex is completely unlocked. Try transferring ownership
                // to another thread.
                if let Some(data_race) = &this.machine.data_race {
                    mutex.clock.clone_from(&data_race.release_clock(&this.machine.threads));
                }
                if let Some(thread) = this.machine.sync.mutexes[id].queue.pop_front() {
                    this.unblock_thread(thread, BlockReason::Mutex(id))?;
                }
            }
            Some(old_lock_count)
        } else {
            // Mutex is not locked.
            None
        })
    }

    /// Put the thread into the queue waiting for the mutex.
    /// Once the Mutex becomes available, `retval` will be written to `dest`.
    #[inline]
    fn mutex_enqueue_and_block(&mut self, id: MutexId, retval: Scalar, dest: MPlaceTy<'tcx>) {
        let this = self.eval_context_mut();
        assert!(this.mutex_is_locked(id), "queing on unlocked mutex");
        let thread = this.active_thread();
        this.machine.sync.mutexes[id].queue.push_back(thread);
        this.block_thread(
            BlockReason::Mutex(id),
            None,
            callback!(
                @capture<'tcx> {
                    id: MutexId,
                    retval: Scalar,
                    dest: MPlaceTy<'tcx>,
                }
                @unblock = |this| {
                    assert!(!this.mutex_is_locked(id));
                    this.mutex_lock(id);
                    this.write_scalar(retval, &dest)?;
                    Ok(())
                }
            ),
        );
    }

    #[inline]
    /// Check if locked.
    fn rwlock_is_locked(&self, id: RwLockId) -> bool {
        let this = self.eval_context_ref();
        let rwlock = &this.machine.sync.rwlocks[id];
        trace!(
            "rwlock_is_locked: {:?} writer is {:?} and there are {} reader threads (some of which could hold multiple read locks)",
            id,
            rwlock.writer,
            rwlock.readers.len(),
        );
        rwlock.writer.is_some() || rwlock.readers.is_empty().not()
    }

    /// Check if write locked.
    #[inline]
    fn rwlock_is_write_locked(&self, id: RwLockId) -> bool {
        let this = self.eval_context_ref();
        let rwlock = &this.machine.sync.rwlocks[id];
        trace!("rwlock_is_write_locked: {:?} writer is {:?}", id, rwlock.writer);
        rwlock.writer.is_some()
    }

    /// Read-lock the lock by adding the `reader` the list of threads that own
    /// this lock.
    fn rwlock_reader_lock(&mut self, id: RwLockId) {
        let this = self.eval_context_mut();
        let thread = this.active_thread();
        assert!(!this.rwlock_is_write_locked(id), "the lock is write locked");
        trace!("rwlock_reader_lock: {:?} now also held (one more time) by {:?}", id, thread);
        let rwlock = &mut this.machine.sync.rwlocks[id];
        let count = rwlock.readers.entry(thread).or_insert(0);
        *count = count.checked_add(1).expect("the reader counter overflowed");
        if let Some(data_race) = &this.machine.data_race {
            data_race.acquire_clock(&rwlock.clock_unlocked, &this.machine.threads);
        }
    }

    /// Try read-unlock the lock for the current threads and potentially give the lock to a new owner.
    /// Returns `true` if succeeded, `false` if this `reader` did not hold the lock.
    fn rwlock_reader_unlock(&mut self, id: RwLockId) -> InterpResult<'tcx, bool> {
        let this = self.eval_context_mut();
        let thread = this.active_thread();
        let rwlock = &mut this.machine.sync.rwlocks[id];
        match rwlock.readers.entry(thread) {
            Entry::Occupied(mut entry) => {
                let count = entry.get_mut();
                assert!(*count > 0, "rwlock locked with count == 0");
                *count -= 1;
                if *count == 0 {
                    trace!("rwlock_reader_unlock: {:?} no longer held by {:?}", id, thread);
                    entry.remove();
                } else {
                    trace!("rwlock_reader_unlock: {:?} held one less time by {:?}", id, thread);
                }
            }
            Entry::Vacant(_) => return Ok(false), // we did not even own this lock
        }
        if let Some(data_race) = &this.machine.data_race {
            // Add this to the shared-release clock of all concurrent readers.
            rwlock.clock_current_readers.join(&data_race.release_clock(&this.machine.threads));
        }

        // The thread was a reader. If the lock is not held any more, give it to a writer.
        if this.rwlock_is_locked(id).not() {
            // All the readers are finished, so set the writer data-race handle to the value
            // of the union of all reader data race handles, since the set of readers
            // happen-before the writers
            let rwlock = &mut this.machine.sync.rwlocks[id];
            rwlock.clock_unlocked.clone_from(&rwlock.clock_current_readers);
            // See if there is a thread to unblock.
            if let Some(writer) = rwlock.writer_queue.pop_front() {
                this.unblock_thread(writer, BlockReason::RwLock(id))?;
            }
        }
        Ok(true)
    }

    /// Put the reader in the queue waiting for the lock and block it.
    /// Once the lock becomes available, `retval` will be written to `dest`.
    #[inline]
    fn rwlock_enqueue_and_block_reader(
        &mut self,
        id: RwLockId,
        retval: Scalar,
        dest: MPlaceTy<'tcx>,
    ) {
        let this = self.eval_context_mut();
        let thread = this.active_thread();
        assert!(this.rwlock_is_write_locked(id), "read-queueing on not write locked rwlock");
        this.machine.sync.rwlocks[id].reader_queue.push_back(thread);
        this.block_thread(
            BlockReason::RwLock(id),
            None,
            callback!(
                @capture<'tcx> {
                    id: RwLockId,
                    retval: Scalar,
                    dest: MPlaceTy<'tcx>,
                }
                @unblock = |this| {
                    this.rwlock_reader_lock(id);
                    this.write_scalar(retval, &dest)?;
                    Ok(())
                }
            ),
        );
    }

    /// Lock by setting the writer that owns the lock.
    #[inline]
    fn rwlock_writer_lock(&mut self, id: RwLockId) {
        let this = self.eval_context_mut();
        let thread = this.active_thread();
        assert!(!this.rwlock_is_locked(id), "the rwlock is already locked");
        trace!("rwlock_writer_lock: {:?} now held by {:?}", id, thread);
        let rwlock = &mut this.machine.sync.rwlocks[id];
        rwlock.writer = Some(thread);
        if let Some(data_race) = &this.machine.data_race {
            data_race.acquire_clock(&rwlock.clock_unlocked, &this.machine.threads);
        }
    }

    /// Try to unlock an rwlock held by the current thread.
    /// Return `false` if it is held by another thread.
    #[inline]
    fn rwlock_writer_unlock(&mut self, id: RwLockId) -> InterpResult<'tcx, bool> {
        let this = self.eval_context_mut();
        let thread = this.active_thread();
        let rwlock = &mut this.machine.sync.rwlocks[id];
        Ok(if let Some(current_writer) = rwlock.writer {
            if current_writer != thread {
                // Only the owner can unlock the rwlock.
                return Ok(false);
            }
            rwlock.writer = None;
            trace!("rwlock_writer_unlock: {:?} unlocked by {:?}", id, thread);
            // Record release clock for next lock holder.
            if let Some(data_race) = &this.machine.data_race {
                rwlock.clock_unlocked.clone_from(&*data_race.release_clock(&this.machine.threads));
            }
            // The thread was a writer.
            //
            // We are prioritizing writers here against the readers. As a
            // result, not only readers can starve writers, but also writers can
            // starve readers.
            if let Some(writer) = rwlock.writer_queue.pop_front() {
                this.unblock_thread(writer, BlockReason::RwLock(id))?;
            } else {
                // Take the entire read queue and wake them all up.
                let readers = std::mem::take(&mut rwlock.reader_queue);
                for reader in readers {
                    this.unblock_thread(reader, BlockReason::RwLock(id))?;
                }
            }
            true
        } else {
            false
        })
    }

    /// Put the writer in the queue waiting for the lock.
    /// Once the lock becomes available, `retval` will be written to `dest`.
    #[inline]
    fn rwlock_enqueue_and_block_writer(
        &mut self,
        id: RwLockId,
        retval: Scalar,
        dest: MPlaceTy<'tcx>,
    ) {
        let this = self.eval_context_mut();
        assert!(this.rwlock_is_locked(id), "write-queueing on unlocked rwlock");
        let thread = this.active_thread();
        this.machine.sync.rwlocks[id].writer_queue.push_back(thread);
        this.block_thread(
            BlockReason::RwLock(id),
            None,
            callback!(
                @capture<'tcx> {
                    id: RwLockId,
                    retval: Scalar,
                    dest: MPlaceTy<'tcx>,
                }
                @unblock = |this| {
                    this.rwlock_writer_lock(id);
                    this.write_scalar(retval, &dest)?;
                    Ok(())
                }
            ),
        );
    }

    /// Is the conditional variable awaited?
    #[inline]
    fn condvar_is_awaited(&mut self, id: CondvarId) -> bool {
        let this = self.eval_context_mut();
        !this.machine.sync.condvars[id].waiters.is_empty()
    }

    /// Release the mutex and let the current thread wait on the given condition variable.
    /// Once it is signaled, the mutex will be acquired and `retval_succ` will be written to `dest`.
    /// If the timeout happens first, `retval_timeout` will be written to `dest`.
    fn condvar_wait(
        &mut self,
        condvar: CondvarId,
        mutex: MutexId,
        timeout: Option<(TimeoutClock, TimeoutAnchor, Duration)>,
        retval_succ: Scalar,
        retval_timeout: Scalar,
        dest: MPlaceTy<'tcx>,
    ) -> InterpResult<'tcx> {
        let this = self.eval_context_mut();
        if let Some(old_locked_count) = this.mutex_unlock(mutex)? {
            if old_locked_count != 1 {
                throw_unsup_format!(
                    "awaiting a condvar on a mutex acquired multiple times is not supported"
                );
            }
        } else {
            throw_ub_format!(
                "awaiting a condvar on a mutex that is unlocked or owned by a different thread"
            );
        }
        let thread = this.active_thread();
        let waiters = &mut this.machine.sync.condvars[condvar].waiters;
        waiters.push_back(thread);
        this.block_thread(
            BlockReason::Condvar(condvar),
            timeout,
            callback!(
                @capture<'tcx> {
                    condvar: CondvarId,
                    mutex: MutexId,
                    retval_succ: Scalar,
                    retval_timeout: Scalar,
                    dest: MPlaceTy<'tcx>,
                }
                @unblock = |this| {
                    // The condvar was signaled. Make sure we get the clock for that.
                    if let Some(data_race) = &this.machine.data_race {
                        data_race.acquire_clock(
                            &this.machine.sync.condvars[condvar].clock,
                            &this.machine.threads,
                        );
                    }
                    // Try to acquire the mutex.
                    // The timeout only applies to the first wait (until the signal), not for mutex acquisition.
                    this.condvar_reacquire_mutex(mutex, retval_succ, dest)
                }
                @timeout = |this| {
                    // We have to remove the waiter from the queue again.
                    let thread = this.active_thread();
                    let waiters = &mut this.machine.sync.condvars[condvar].waiters;
                    waiters.retain(|waiter| *waiter != thread);
                    // Now get back the lock.
                    this.condvar_reacquire_mutex(mutex, retval_timeout, dest)
                }
            ),
        );
        return Ok(());
    }

    /// Wake up some thread (if there is any) sleeping on the conditional
    /// variable. Returns `true` iff any thread was woken up.
    fn condvar_signal(&mut self, id: CondvarId) -> InterpResult<'tcx, bool> {
        let this = self.eval_context_mut();
        let condvar = &mut this.machine.sync.condvars[id];
        let data_race = &this.machine.data_race;

        // Each condvar signal happens-before the end of the condvar wake
        if let Some(data_race) = data_race {
            condvar.clock.clone_from(&*data_race.release_clock(&this.machine.threads));
        }
        let Some(waiter) = condvar.waiters.pop_front() else {
            return Ok(false);
        };
        this.unblock_thread(waiter, BlockReason::Condvar(id))?;
        Ok(true)
    }

    /// Wait for the futex to be signaled, or a timeout.
    /// On a signal, `retval_succ` is written to `dest`.
    /// On a timeout, `retval_timeout` is written to `dest` and `errno_timeout` is set as the last error.
    fn futex_wait(
        &mut self,
        addr: u64,
        bitset: u32,
        timeout: Option<(TimeoutClock, TimeoutAnchor, Duration)>,
        retval_succ: Scalar,
        retval_timeout: Scalar,
        dest: MPlaceTy<'tcx>,
        errno_timeout: Scalar,
    ) {
        let this = self.eval_context_mut();
        let thread = this.active_thread();
        let futex = &mut this.machine.sync.futexes.entry(addr).or_default();
        let waiters = &mut futex.waiters;
        assert!(waiters.iter().all(|waiter| waiter.thread != thread), "thread is already waiting");
        waiters.push_back(FutexWaiter { thread, bitset });
        this.block_thread(
            BlockReason::Futex { addr },
            timeout,
            callback!(
                @capture<'tcx> {
                    addr: u64,
                    retval_succ: Scalar,
                    retval_timeout: Scalar,
                    dest: MPlaceTy<'tcx>,
                    errno_timeout: Scalar,
                }
                @unblock = |this| {
                    let futex = this.machine.sync.futexes.get(&addr).unwrap();
                    // Acquire the clock of the futex.
                    if let Some(data_race) = &this.machine.data_race {
                        data_race.acquire_clock(&futex.clock, &this.machine.threads);
                    }
                    // Write the return value.
                    this.write_scalar(retval_succ, &dest)?;
                    Ok(())
                }
                @timeout = |this| {
                    // Remove the waiter from the futex.
                    let thread = this.active_thread();
                    let futex = this.machine.sync.futexes.get_mut(&addr).unwrap();
                    futex.waiters.retain(|waiter| waiter.thread != thread);
                    // Set errno and write return value.
                    this.set_last_error(errno_timeout)?;
                    this.write_scalar(retval_timeout, &dest)?;
                    Ok(())
                }
            ),
        );
    }

    /// Returns whether anything was woken.
    fn futex_wake(&mut self, addr: u64, bitset: u32) -> InterpResult<'tcx, bool> {
        let this = self.eval_context_mut();
        let Some(futex) = this.machine.sync.futexes.get_mut(&addr) else {
            return Ok(false);
        };
        let data_race = &this.machine.data_race;

        // Each futex-wake happens-before the end of the futex wait
        if let Some(data_race) = data_race {
            futex.clock.clone_from(&*data_race.release_clock(&this.machine.threads));
        }

        // Wake up the first thread in the queue that matches any of the bits in the bitset.
        let Some(i) = futex.waiters.iter().position(|w| w.bitset & bitset != 0) else {
            return Ok(false);
        };
        let waiter = futex.waiters.remove(i).unwrap();
        this.unblock_thread(waiter.thread, BlockReason::Futex { addr })?;
        Ok(true)
    }
}
