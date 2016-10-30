
extern crate libc;
use libc::*;
use bindings::core::*;
/* 
#include "ofxMultiThread.h"
typedef struct OfxMultiThreadSuiteV1 {
  OfxStatus (*multiThread)(OfxThreadFunctionV1 func,
			   unsigned int nThreads,
			   void *customArg);
  OfxStatus (*multiThreadNumCPUs)(unsigned int *nCPUs);
  OfxStatus (*multiThreadIndex)(unsigned int *threadIndex);
  int (*multiThreadIsSpawnedThread)(void);
  OfxStatus (*mutexCreate)(OfxMutexHandle *mutex, int lockCount);
  OfxStatus (*mutexDestroy)(const OfxMutexHandle mutex);
  OfxStatus (*mutexLock)(const OfxMutexHandle mutex);
  OfxStatus (*mutexUnLock)(const OfxMutexHandle mutex);
  OfxStatus (*mutexTryLock)(const OfxMutexHandle mutex);
 } OfxMultiThreadSuiteV1;
 */

pub type OfxMutexHandle = * mut c_void;
pub type OfxThreadFunctionV1 = extern fn (); // TODO 

pub extern fn multi_thread(func: OfxThreadFunctionV1, nb_thread: c_uint, custom_arg: * mut c_void) -> OfxStatus { kOfxStatOK }
pub extern fn multi_thread_num_cpus(num_cpu: * mut c_uint) -> OfxStatus { kOfxStatOK }
pub extern fn multi_thread_index(thread_index: * mut c_uint) -> OfxStatus { kOfxStatOK }
pub extern fn multi_thread_is_spawned_thread() -> c_int { 0 }
pub extern fn mutex_create(mutex: * mut OfxMutexHandle, lock_count: c_int) -> OfxStatus { kOfxStatOK }
pub extern fn mutex_destroy (mutex: OfxMutexHandle) -> OfxStatus { kOfxStatOK }
pub extern fn mutex_lock(mutex: OfxMutexHandle) -> OfxStatus { kOfxStatOK }
pub extern fn mutex_unlock(mutex: OfxMutexHandle) -> OfxStatus { kOfxStatOK }
pub extern fn mutex_try_lock(mutex: OfxMutexHandle) -> OfxStatus { kOfxStatOK }

#[repr(C)]
#[allow(non_snake_case)]
pub struct OfxMultiThreadSuiteV1 {
    pub multiThread: extern fn (OfxThreadFunctionV1, c_uint, * mut c_void) -> OfxStatus,
    pub multiThreadNumCPUs: extern fn (* mut c_uint) -> OfxStatus, 
    pub multiThreadIndex: extern fn (* mut c_uint) -> OfxStatus,
    pub multiThreadIsSpawnedThread: extern fn () -> c_int,
    pub mutexCreate: extern fn (* mut OfxMutexHandle, c_int) -> OfxStatus,
    pub mutexDestroy: extern fn (OfxMutexHandle) -> OfxStatus,
    pub mutexLock: extern fn (OfxMutexHandle) -> OfxStatus,
    pub mutexUnlock: extern fn (OfxMutexHandle) -> OfxStatus,
    pub mutexTryLock: extern fn (OfxMutexHandle) -> OfxStatus,
}

pub static OFX_MULTITHREAD_SUITE_V1
    : OfxMultiThreadSuiteV1 
        = OfxMultiThreadSuiteV1 {
            multiThread: multi_thread,
            multiThreadNumCPUs: multi_thread_num_cpus,
            multiThreadIndex: multi_thread_index,
            multiThreadIsSpawnedThread: multi_thread_is_spawned_thread,
            mutexCreate: mutex_create,
            mutexDestroy: mutex_destroy,
            mutexLock: mutex_lock,
            mutexUnlock: mutex_unlock,
            mutexTryLock: mutex_try_lock, 
        };

