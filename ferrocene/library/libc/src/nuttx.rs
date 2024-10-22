macro_rules! ffi {
    ($(pub fn $name:ident($($param:ident : $ty:ty),*) -> $ret:ty;)+) => {
        $(
            pub unsafe extern "C" fn $name($($param : $ty),*) -> $ret {
                extern "C" {
                    fn $name($($param : $ty),*) -> $ret;
                }

                let f: unsafe extern "C" fn($($ty),*) -> $ret = $name;
                // `read_volatile` is a no-std alternative to `std::hint::black_box`
                // and used to force the function call below to be done through a function pointer
                // as direct FFI calls appear to not be supported by the nuttx v8-R ELF loader
                let f = core::ptr::read_volatile(&f);
                f($($param),*)
            }
        )+
    }
}

// include/sys/stat.h
pub const S_IFSOCK: mode_t = 12 << 12;

// include/dirent.h
pub const DT_FIFO: u8 = 1;
pub const DT_CHR: u8 = 2;
pub const DT_LNK: u8 = 10;
pub const DT_REG: u8 = 8;
pub const DT_SOCK: u8 = 12;
pub const DT_DIR: u8 = 4;
pub const DT_BLK: u8 = 6;

// include/unistd.h
pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

// include/fcntl.h
pub const AT_FDCWD: c_int = -100;
pub const F_GETFD: c_int = 1;
pub const O_TRUNC: c_int = 1 << 5;
pub const O_CREAT: c_int = 1 << 2;
pub const O_EXCL: c_int = 1 << 3;
pub const O_APPEND: c_int = 1 << 4;
pub const O_CLOEXEC: c_int = 1 << 10;
pub const O_NONBLOCK: c_int = 1 << 16;
pub const F_GETFL: c_int = 2;
pub const F_SETFL: c_int = 9;
pub const F_DUPFD_CLOEXEC: c_int = 18;
pub const O_RDONLY: c_int = 1 << 0;
pub const O_WRONLY: c_int = 1 << 1;
pub const O_RDWR: c_int = O_RDOK | O_WROK;
pub const O_ACCMODE: c_int = O_RDWR;
const O_RDOK: c_int = O_RDONLY;
const O_WROK: c_int = O_WRONLY;

pub type c_ulong = u32; // NOTE assumes ARMv8-R
pub type c_long = i32; // NOTE assumes ARMv8-R
pub type c_char = u8; // NOTE assumes ARMv8-R

// include/sys/poll.h
pub type pollevent_t = u32;
pub const POLLIN: pollevent_t = 0x01;
pub const POLLNVAL: pollevent_t = 0x01;
pub type nfds_t = c_uint;

// include/limits.h
const NAME_MAX: usize = _POSIX_NAME_MAX;
const _POSIX_NAME_MAX: usize = 32; // NOTE this is user configurable

s! {
    // include/time.h
    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: c_long,
    }

    // include/sys/uio.h
    pub struct iovec {
        iov_base: *mut c_void,
        iov_len: size_t,
    }

    // include/sys/stat.h
    pub struct stat {
        st_dev: dev_t,
        st_ino: ino_t,
        pub st_mode: mode_t,
        st_nlink: nlink_t,
        st_uid: uid_t,
        st_gid: gid_t,
        st_rdev: dev_t,
        pub st_size: off_t,

        // NOTE original
        // st_atim: timespec,
        // st_mtim: timespec,
        // st_ctim: timespec,

        // NOTE flattened as libstd relies on these specific field names
        pub st_atime: time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: time_t,
        pub st_ctime_nsec: c_long,

        st_blksize: blksize_t,
        st_blocks: blkcnt_t,
    }

    // include/dirent.h
    pub struct DIR {
        fd: c_int,
        entry: dirent,
    }

    pub struct dirent {
        pub d_type: u8,
        pub d_name: [c_char; NAME_MAX + 1],
    }

    // include/signal.h
    pub struct sigset_t {
        _elem: [u32; _SIGSET_NELEM],
    }

    // include/sys/poll.h
    pub struct pollfd {
        pub fd: c_int,
        pub events: pollevent_t,
        pub revents: pollevent_t,

        arg: *mut c_void,
        cb: pollcb_t,
        r#priv: *mut c_void,
    }
}

type pollcb_t = *const c_void; // this is some sort of function; the signature is not imporant as the field is private

const _SIGSET_NELEM: usize = (_NSIG + 31) >> 5;
const _NSIG: usize = MAX_SIGNO + 1;
const MAX_SIGNO: usize = 63;

// include/sys/stat.h
pub const UTIME_OMIT: c_long = (1 << 30) - 2;
pub const S_IFMT: mode_t = 15 << 12;
pub const S_IFDIR: mode_t = 4 << 12;
pub const S_IFBLK: mode_t = 6 << 12;
pub const S_IFCHR: mode_t = 2 << 12;
pub const S_IFLNK: mode_t = 10 << 12;
pub const S_IFIFO: mode_t = 1 << 12;
pub const S_IFREG: mode_t = 8 << 12;
pub const S_IRUSR: mode_t = 1 << 8;
pub const S_IWUSR: mode_t = 1 << 7;
pub const S_IXUSR: mode_t = 1 << 6;
pub const S_ISUID: mode_t = 1 << 11;
pub const S_IRGRP: mode_t = 1 << 5;
pub const S_IWGRP: mode_t = 1 << 4;
pub const S_IXGRP: mode_t = 1 << 3;
pub const S_ISGID: mode_t = 1 << 10;
pub const S_IROTH: mode_t = 1 << 2;
pub const S_IWOTH: mode_t = 1 << 1;
pub const S_IXOTH: mode_t = 1 << 0;
pub const S_ISVTX: mode_t = 1 << 9;

// include/stdlib.h
pub const EXIT_FAILURE: c_int = 1;
pub const EXIT_SUCCESS: c_int = 0;

// include/sys/types.h
pub type pid_t = c_int;
pub type dev_t = u32;
pub type ino_t = u16;
pub type nlink_t = u16;
pub type uid_t = c_uint; // NOTE assumes CONFIG_SMALL_MEMORY is DISabled
pub type gid_t = c_uint; // NOTE assumes CONFIG_SMALL_MEMORY is DISabled
pub type blksize_t = u16;
pub type blkcnt_t = u32; // NOTE assumes CONFIG_FS_LARGEFILE is DISabled
pub type socklen_t = c_uint;
pub type clockid_t = c_int;
pub type time_t = u64; // NOTE assumes CONFIG_SYSTEM_TIME64 is ENabled
pub type off_t = i32; // NOTE assumes CONFIG_FS_LARGEFILE is DISabled
pub type mode_t = c_uint;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
pub const SEEK_SET: c_int = 0;

// include/pthread.h
pub type pthread_key_t = c_int;

// include/netinet/in.h
pub type in_addr_t = u32;

// include/time.h
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_REALTIME: clockid_t = 0;

// include/errno.h
pub const EBADF: c_int = 9;
pub const EINVAL: c_int = 22;
pub const E2BIG: c_int = 7;
pub const EACCES: c_int = 13;
pub const EADDRINUSE: c_int = 98;
pub const EADDRNOTAVAIL: c_int = 99;
pub const EAGAIN: c_int = 11;
pub const EBUSY: c_int = 16;
pub const ECONNABORTED: c_int = 103;
pub const ECONNREFUSED: c_int = 111;
pub const ECONNRESET: c_int = 104;
pub const EDEADLK: c_int = 35;
pub const EDQUOT: c_int = 122;
pub const EEXIST: c_int = 17;
pub const EFBIG: c_int = 27;
pub const EHOSTUNREACH: c_int = 113;
pub const EINTR: c_int = 4;
pub const EISDIR: c_int = 21;
pub const ELOOP: c_int = 40;
pub const EMLINK: c_int = 31;
pub const ENAMETOOLONG: c_int = 36;
pub const ENETDOWN: c_int = 100;
pub const ENETUNREACH: c_int = 101;
pub const ENOENT: c_int = 2;
pub const ENOMEM: c_int = 12;
pub const ENOSPC: c_int = 28;
pub const ENOSYS: c_int = 38;
pub const ENOTCONN: c_int = 107;
pub const ENOTDIR: c_int = 20;
pub const ENOTEMPTY: c_int = 39;
pub const EPERM: c_int = 1;
pub const EPIPE: c_int = 32;
pub const EROFS: c_int = 30;
pub const ESPIPE: c_int = 29;
pub const ESTALE: c_int = 116;
pub const ETIMEDOUT: c_int = 110;
pub const ETXTBSY: c_int = 26;
pub const EWOULDBLOCK: c_int = EAGAIN;
pub const EXDEV: c_int = 18;
pub const ECHILD: c_int = 10;

// include/nuttx/fs/ioctl.h
pub const FIOCLEX: c_ulong = 0x0300 | 0x000b;

// include/signal.h
pub const SIGABRT: c_int = 6;
pub const SIGALRM: c_int = 14;
pub const SIGBUS: c_int = 7;
pub const SIGCHLD: c_int = 17;
pub const SIGCONT: c_int = 18;
pub const SIGFPE: c_int = 8;
pub const SIGHUP: c_int = 1;
pub const SIGILL: c_int = 4;
pub const SIGINT: c_int = 2;
pub const SIGIO: c_int = SIGPOLL;
pub const SIGKILL: c_int = 9;
pub const SIGPIPE: c_int = 13;
pub const SIGPOLL: c_int = 29;
pub const SIGPROF: c_int = 27;
pub const SIGQUIT: c_int = 3;
pub const SIGSEGV: c_int = 11;
pub const SIGSTOP: c_int = 19;
pub const SIGSYS: c_int = 31;
pub const SIGTERM: c_int = 15;
pub const SIGTRAP: c_int = 5;
pub const SIGTSTP: c_int = 20;
pub const SIGTTIN: c_int = 21;
pub const SIGTTOU: c_int = 22;
pub const SIGURG: c_int = 23;
pub const SIGUSR1: c_int = 10;
pub const SIGUSR2: c_int = 12;
pub const SIGVTALRM: c_int = 26;
pub const SIGWINCH: c_int = 28;
pub const SIGXCPU: c_int = 24;
pub const SIGXFSZ: c_int = 25;

pub const SIG_ERR: sighandler_t = size_t::MAX;
pub const SIG_IGN: sighandler_t = 0;
pub const SIG_DFL: sighandler_t = 0; // NOTE assumes that CONFIG_SIG_DEFAULT is DISabled

// include/sys/wait.h
pub const WNOHANG: c_int = 1 << 1;

// from other libc definitions
pub type sighandler_t = size_t;

// include/unistd.h
pub use execv as execvp;
pub use execve as execvpe;
pub use up_fork as fork;

// all these symbols must be present and defined ('T') in the `nuttx` binary
// if symbol lookup fails at runtime then add the symbol to `/syscall/syscall.csv` in the nuttx repo
ffi! {
    /* POSIX-y API that we assume has the same signature on all POSIX-y OSes */
    pub fn __errno() -> *mut c_int;
    pub fn abort() -> !;
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn chmod(path: *const c_char, mode: mode_t) -> c_int;
    pub fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    pub fn close(fd: c_int) -> c_int;
    pub fn closedir(dirp: *mut DIR) -> c_int;
    pub fn fchmod(fd: c_int, mode: mode_t) -> c_int;
    pub fn fcntl(fd: c_int, cmd: c_int) -> c_int;
    pub fn free(p: *mut c_void) -> ();
    pub fn fstat(fildes: c_int, buf: *mut stat) -> c_int;
    pub fn fsync(fd: c_int) -> c_int;
    pub fn ftruncate(fd: c_int, length: off_t) -> c_int;
    pub fn futimens(fd: c_int, times: *const timespec) -> c_int;
    pub fn ioctl(fd: c_int, request: c_ulong) -> c_int;
    pub fn linkat(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char, flags: c_int) -> c_int; // NOTE CONFIG_PSEUDOFS_SOFTLINKS must be ENabled
    pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    pub fn lstat(path: *const c_char, buf: *mut stat) -> c_int;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn mkdir(path: *const c_char, mode: mode_t) -> c_int;
    pub fn open(path: *const c_char, oflag: c_int) -> c_int;
    pub fn opendir(dirname: *const c_char) -> *mut DIR;
    pub fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int;
    pub fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;
    pub fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn readdir_r(dirp: *mut DIR, entry: *mut dirent, result: *mut *mut dirent) -> c_int;
    pub fn readv(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
    pub fn realpath(pathname: *const c_char, resolver: *mut c_char) -> *mut c_char;
    pub fn rename(oldname: *const c_char, newname: *const c_char) -> c_int;
    pub fn rmdir(path: *const c_char) -> c_int;
    pub fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    pub fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    pub fn symlink(path1: *const c_char, path2: *const c_char) -> c_int; // NOTE CONFIG_PSEUDOFS_SOFTLINKS must be ENabled
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    pub fn writev(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;
    pub fn readlink(path: *const c_char, buf: *mut c_char, bufsz: size_t) -> ssize_t;
    pub fn unlink(c: *const c_char) -> c_int;
    pub fn getenv(s: *const c_char) -> *mut c_char;
    pub fn setenv(name: *const c_char, val: *const c_char, overwrite: c_int) -> c_int;
    pub fn sigemptyset(set: *mut sigset_t) -> c_int;
    pub fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    pub fn _exit(status: c_int) -> !;
    pub fn dup2(src: c_int, dst: c_int) -> c_int;
    pub fn setgid(gid: gid_t) -> c_int; // NOTE CONFIG_SCHED_USER_IDENTITY must be ENabled
    pub fn chdir(dir: *const c_char) -> c_int;
    pub fn signal(signum: c_int, handle: sighandler_t) -> sighandler_t;
    pub fn kill(pid: pid_t, sig: c_int) -> c_int;
    pub fn execv(c: *const c_char, argv: *const *const c_char) -> c_int;
    pub fn execve(c: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int;
    pub fn setuid(uid: uid_t) -> c_int;
    pub fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    pub fn pipe2(fds: *mut c_int, flags: c_int) -> c_int;
    pub fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int;
    pub fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> c_int;
    pub fn exit(status: c_int) -> !;
    pub fn unsetenv(name: *const c_char) -> c_int;

    // nuttx-specific
    pub fn up_fork() -> pid_t;
    pub fn get_environ_ptr() -> *const *const c_char;
}

// these are defined as macros in include/sys/wait.h
pub fn WIFCONTINUED(_status: c_int) -> bool {
    false
}

pub fn WIFSIGNALED(_status: c_int) -> bool {
    false
}

pub fn WIFSTOPPED(_status: c_int) -> bool {
    false
}

pub fn WSTOPSIG(_status: c_int) -> c_int {
    false as c_int
}

pub fn WTERMSIG(_status: c_int) -> c_int {
    false as c_int
}

pub fn WIFEXITED(s: c_int) -> bool {
    (s & 0xff) == 0
}

pub fn WEXITSTATUS(s: c_int) -> c_int {
    (s >> 8) & 0xff
}

pub use self::variadic::{fcntl as fcntl3, open as open3};

mod variadic {
    #![allow(clashing_extern_declarations)]

    use super::{c_char, c_int};

    ffi! {
        pub fn fcntl(fd: c_int, cmd: c_int, arg2: c_int) -> c_int;
        pub fn open(path: *const c_char, oflag: c_int, arg2: c_int) -> c_int;
    }
}

/* the code below this point comes from `/src/unix/mod.rs` */
pub type c_int = i32;
pub type c_uint = u32;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;

cfg_if! {
    if #[cfg(libc_core_cvoid)] {
        pub use ::ffi::c_void;
    } else {
        // Use repr(u8) as LLVM expects `void*` to be the same as `i8*` to help
        // enable more optimization opportunities around it recognizing things
        // like malloc/free.
        #[repr(u8)]
        #[allow(missing_copy_implementations)]
        #[allow(missing_debug_implementations)]
        pub enum c_void {
            // Two dummy variants so the #[repr] attribute can be used.
            #[doc(hidden)]
            __variant1,
            #[doc(hidden)]
            __variant2,
        }
    }
}
