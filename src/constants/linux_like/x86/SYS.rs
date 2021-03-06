// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


use ::SyscallNumber;


pub const SYS_restart_syscall: SyscallNumber = 0;
pub const SYS_exit: SyscallNumber = 1;
pub const SYS_fork: SyscallNumber = 2;
pub const SYS_read: SyscallNumber = 3;
pub const SYS_write: SyscallNumber = 4;
pub const SYS_open: SyscallNumber = 5;
pub const SYS_close: SyscallNumber = 6;
pub const SYS_waitpid: SyscallNumber = 7;
pub const SYS_creat: SyscallNumber = 8;
pub const SYS_link: SyscallNumber = 9;
pub const SYS_unlink: SyscallNumber = 10;
pub const SYS_execve: SyscallNumber = 11;
pub const SYS_chdir: SyscallNumber = 12;
pub const SYS_time: SyscallNumber = 13;
pub const SYS_mknod: SyscallNumber = 14;
pub const SYS_chmod: SyscallNumber = 15;
pub const SYS_lchown: SyscallNumber = 16;
pub const SYS_break: SyscallNumber = 17;
pub const SYS_oldstat: SyscallNumber = 18;
pub const SYS_lseek: SyscallNumber = 19;
pub const SYS_getpid: SyscallNumber = 20;
pub const SYS_mount: SyscallNumber = 21;
pub const SYS_umount: SyscallNumber = 22;
pub const SYS_setuid: SyscallNumber = 23;
pub const SYS_getuid: SyscallNumber = 24;
pub const SYS_stime: SyscallNumber = 25;
pub const SYS_ptrace: SyscallNumber = 26;
pub const SYS_alarm: SyscallNumber = 27;
pub const SYS_oldfstat: SyscallNumber = 28;
pub const SYS_pause: SyscallNumber = 29;
pub const SYS_utime: SyscallNumber = 30;
pub const SYS_stty: SyscallNumber = 31;
pub const SYS_gtty: SyscallNumber = 32;
pub const SYS_access: SyscallNumber = 33;
pub const SYS_nice: SyscallNumber = 34;
pub const SYS_ftime: SyscallNumber = 35;
pub const SYS_sync: SyscallNumber = 36;
pub const SYS_kill: SyscallNumber = 37;
pub const SYS_rename: SyscallNumber = 38;
pub const SYS_mkdir: SyscallNumber = 39;
pub const SYS_rmdir: SyscallNumber = 40;
pub const SYS_dup: SyscallNumber = 41;
pub const SYS_pipe: SyscallNumber = 42;
pub const SYS_times: SyscallNumber = 43;
pub const SYS_prof: SyscallNumber = 44;
pub const SYS_brk: SyscallNumber = 45;
pub const SYS_setgid: SyscallNumber = 46;
pub const SYS_getgid: SyscallNumber = 47;
pub const SYS_signal: SyscallNumber = 48;
pub const SYS_geteuid: SyscallNumber = 49;
pub const SYS_getegid: SyscallNumber = 50;
pub const SYS_acct: SyscallNumber = 51;
pub const SYS_umount2: SyscallNumber = 52;
pub const SYS_lock: SyscallNumber = 53;
pub const SYS_ioctl: SyscallNumber = 54;
pub const SYS_fcntl: SyscallNumber = 55;
pub const SYS_mpx: SyscallNumber = 56;
pub const SYS_setpgid: SyscallNumber = 57;
pub const SYS_ulimit: SyscallNumber = 58;
pub const SYS_oldolduname: SyscallNumber = 59;
pub const SYS_umask: SyscallNumber = 60;
pub const SYS_chroot: SyscallNumber = 61;
pub const SYS_ustat: SyscallNumber = 62;
pub const SYS_dup2: SyscallNumber = 63;
pub const SYS_getppid: SyscallNumber = 64;
pub const SYS_getpgrp: SyscallNumber = 65;
pub const SYS_setsid: SyscallNumber = 66;
pub const SYS_sigaction: SyscallNumber = 67;
pub const SYS_sgetmask: SyscallNumber = 68;
pub const SYS_ssetmask: SyscallNumber = 69;
pub const SYS_setreuid: SyscallNumber = 70;
pub const SYS_setregid: SyscallNumber = 71;
pub const SYS_sigsuspend: SyscallNumber = 72;
pub const SYS_sigpending: SyscallNumber = 73;
pub const SYS_sethostname: SyscallNumber = 74;
pub const SYS_setrlimit: SyscallNumber = 75;
pub const SYS_getrlimit: SyscallNumber = 76;
pub const SYS_getrusage: SyscallNumber = 77;
pub const SYS_gettimeofday: SyscallNumber = 78;
pub const SYS_settimeofday: SyscallNumber = 79;
pub const SYS_getgroups: SyscallNumber = 80;
pub const SYS_setgroups: SyscallNumber = 81;
pub const SYS_select: SyscallNumber = 82;
pub const SYS_symlink: SyscallNumber = 83;
pub const SYS_oldlstat: SyscallNumber = 84;
pub const SYS_readlink: SyscallNumber = 85;
pub const SYS_uselib: SyscallNumber = 86;
pub const SYS_swapon: SyscallNumber = 87;
pub const SYS_reboot: SyscallNumber = 88;
pub const SYS_readdir: SyscallNumber = 89;
pub const SYS_mmap: SyscallNumber = 90;
pub const SYS_munmap: SyscallNumber = 91;
pub const SYS_truncate: SyscallNumber = 92;
pub const SYS_ftruncate: SyscallNumber = 93;
pub const SYS_fchmod: SyscallNumber = 94;
pub const SYS_fchown: SyscallNumber = 95;
pub const SYS_getpriority: SyscallNumber = 96;
pub const SYS_setpriority: SyscallNumber = 97;
pub const SYS_profil: SyscallNumber = 98;
pub const SYS_statfs: SyscallNumber = 99;
pub const SYS_fstatfs: SyscallNumber = 100;
pub const SYS_ioperm: SyscallNumber = 101;
pub const SYS_socketcall: SyscallNumber = 102;
pub const SYS_syslog: SyscallNumber = 103;
pub const SYS_setitimer: SyscallNumber = 104;
pub const SYS_getitimer: SyscallNumber = 105;
pub const SYS_stat: SyscallNumber = 106;
pub const SYS_lstat: SyscallNumber = 107;
pub const SYS_fstat: SyscallNumber = 108;
pub const SYS_olduname: SyscallNumber = 109;
pub const SYS_iopl: SyscallNumber = 110;
pub const SYS_vhangup: SyscallNumber = 111;
pub const SYS_idle: SyscallNumber = 112;
pub const SYS_vm86old: SyscallNumber = 113;
pub const SYS_wait4: SyscallNumber = 114;
pub const SYS_swapoff: SyscallNumber = 115;
pub const SYS_sysinfo: SyscallNumber = 116;
pub const SYS_ipc: SyscallNumber = 117;
pub const SYS_fsync: SyscallNumber = 118;
pub const SYS_sigreturn: SyscallNumber = 119;
pub const SYS_clone: SyscallNumber = 120;
pub const SYS_setdomainname: SyscallNumber = 121;
pub const SYS_uname: SyscallNumber = 122;
pub const SYS_modify_ldt: SyscallNumber = 123;
pub const SYS_adjtimex: SyscallNumber = 124;
pub const SYS_mprotect: SyscallNumber = 125;
pub const SYS_sigprocmask: SyscallNumber = 126;
pub const SYS_create_module: SyscallNumber = 127;
pub const SYS_init_module: SyscallNumber = 128;
pub const SYS_delete_module: SyscallNumber = 129;
pub const SYS_get_kernel_syms: SyscallNumber = 130;
pub const SYS_quotactl: SyscallNumber = 131;
pub const SYS_getpgid: SyscallNumber = 132;
pub const SYS_fchdir: SyscallNumber = 133;
pub const SYS_bdflush: SyscallNumber = 134;
pub const SYS_sysfs: SyscallNumber = 135;
pub const SYS_personality: SyscallNumber = 136;
pub const SYS_afs_syscall: SyscallNumber = 137;
pub const SYS_setfsuid: SyscallNumber = 138;
pub const SYS_setfsgid: SyscallNumber = 139;
pub const SYS_llseek: SyscallNumber = 140;
pub const SYS_getdents: SyscallNumber = 141;
pub const SYS_newselect: SyscallNumber = 142;
pub const SYS_flock: SyscallNumber = 143;
pub const SYS_msync: SyscallNumber = 144;
pub const SYS_readv: SyscallNumber = 145;
pub const SYS_writev: SyscallNumber = 146;
pub const SYS_getsid: SyscallNumber = 147;
pub const SYS_fdatasync: SyscallNumber = 148;
pub const SYS_sysctl: SyscallNumber = 149;
pub const SYS_mlock: SyscallNumber = 150;
pub const SYS_munlock: SyscallNumber = 151;
pub const SYS_mlockall: SyscallNumber = 152;
pub const SYS_munlockall: SyscallNumber = 153;
pub const SYS_sched_setparam: SyscallNumber = 154;
pub const SYS_sched_getparam: SyscallNumber = 155;
pub const SYS_sched_setscheduler: SyscallNumber = 156;
pub const SYS_sched_getscheduler: SyscallNumber = 157;
pub const SYS_sched_yield: SyscallNumber = 158;
pub const SYS_sched_get_priority_max: SyscallNumber = 159;
pub const SYS_sched_get_priority_min: SyscallNumber = 160;
pub const SYS_sched_rr_get_interval: SyscallNumber = 161;
pub const SYS_nanosleep: SyscallNumber = 162;
pub const SYS_mremap: SyscallNumber = 163;
pub const SYS_setresuid: SyscallNumber = 164;
pub const SYS_getresuid: SyscallNumber = 165;
pub const SYS_vm86: SyscallNumber = 166;
pub const SYS_query_module: SyscallNumber = 167;
pub const SYS_poll: SyscallNumber = 168;
pub const SYS_nfsservctl: SyscallNumber = 169;
pub const SYS_setresgid: SyscallNumber = 170;
pub const SYS_getresgid: SyscallNumber = 171;
pub const SYS_prctl: SyscallNumber = 172;
pub const SYS_rt_sigreturn: SyscallNumber = 173;
pub const SYS_rt_sigaction: SyscallNumber = 174;
pub const SYS_rt_sigprocmask: SyscallNumber = 175;
pub const SYS_rt_sigpending: SyscallNumber = 176;
pub const SYS_rt_sigtimedwait: SyscallNumber = 177;
pub const SYS_rt_sigqueueinfo: SyscallNumber = 178;
pub const SYS_rt_sigsuspend: SyscallNumber = 179;
pub const SYS_pread64: SyscallNumber = 180;
pub const SYS_pwrite64: SyscallNumber = 181;
pub const SYS_chown: SyscallNumber = 182;
pub const SYS_getcwd: SyscallNumber = 183;
pub const SYS_capget: SyscallNumber = 184;
pub const SYS_capset: SyscallNumber = 185;
pub const SYS_sigaltstack: SyscallNumber = 186;
pub const SYS_sendfile: SyscallNumber = 187;
pub const SYS_getpmsg: SyscallNumber = 188;
pub const SYS_putpmsg: SyscallNumber = 189;
pub const SYS_vfork: SyscallNumber = 190;
pub const SYS_ugetrlimit: SyscallNumber = 191;
pub const SYS_mmap2: SyscallNumber = 192;
pub const SYS_truncate64: SyscallNumber = 193;
pub const SYS_ftruncate64: SyscallNumber = 194;
pub const SYS_stat64: SyscallNumber = 195;
pub const SYS_lstat64: SyscallNumber = 196;
pub const SYS_fstat64: SyscallNumber = 197;
pub const SYS_lchown32: SyscallNumber = 198;
pub const SYS_getuid32: SyscallNumber = 199;
pub const SYS_getgid32: SyscallNumber = 200;
pub const SYS_geteuid32: SyscallNumber = 201;
pub const SYS_getegid32: SyscallNumber = 202;
pub const SYS_setreuid32: SyscallNumber = 203;
pub const SYS_setregid32: SyscallNumber = 204;
pub const SYS_getgroups32: SyscallNumber = 205;
pub const SYS_setgroups32: SyscallNumber = 206;
pub const SYS_fchown32: SyscallNumber = 207;
pub const SYS_setresuid32: SyscallNumber = 208;
pub const SYS_getresuid32: SyscallNumber = 209;
pub const SYS_setresgid32: SyscallNumber = 210;
pub const SYS_getresgid32: SyscallNumber = 211;
pub const SYS_chown32: SyscallNumber = 212;
pub const SYS_setuid32: SyscallNumber = 213;
pub const SYS_setgid32: SyscallNumber = 214;
pub const SYS_setfsuid32: SyscallNumber = 215;
pub const SYS_setfsgid32: SyscallNumber = 216;
pub const SYS_pivot_root: SyscallNumber = 217;
pub const SYS_mincore: SyscallNumber = 218;
pub const SYS_madvise: SyscallNumber = 219;
pub const SYS_madvise1: SyscallNumber = 219;
pub const SYS_getdents64: SyscallNumber = 220;
pub const SYS_fcntl64: SyscallNumber = 221;
// There is no 223
pub const SYS_gettid: SyscallNumber = 224;
pub const SYS_readahead: SyscallNumber = 225;
pub const SYS_setxattr: SyscallNumber = 226;
pub const SYS_lsetxattr: SyscallNumber = 227;
pub const SYS_fsetxattr: SyscallNumber = 228;
pub const SYS_getxattr: SyscallNumber = 229;
pub const SYS_lgetxattr: SyscallNumber = 230;
pub const SYS_fgetxattr: SyscallNumber = 231;
pub const SYS_listxattr: SyscallNumber = 232;
pub const SYS_llistxattr: SyscallNumber = 233;
pub const SYS_flistxattr: SyscallNumber = 234;
pub const SYS_removexattr: SyscallNumber = 235;
pub const SYS_lremovexattr: SyscallNumber = 236;
pub const SYS_fremovexattr: SyscallNumber = 237;
pub const SYS_tkill: SyscallNumber = 238;
pub const SYS_sendfile64: SyscallNumber = 239;
pub const SYS_futex: SyscallNumber = 240;
pub const SYS_sched_setaffinity: SyscallNumber = 241;
pub const SYS_sched_getaffinity: SyscallNumber = 242;
pub const SYS_set_thread_area: SyscallNumber = 243;
pub const SYS_get_thread_area: SyscallNumber = 244;
pub const SYS_io_setup: SyscallNumber = 245;
pub const SYS_io_destroy: SyscallNumber = 246;
pub const SYS_io_getevents: SyscallNumber = 247;
pub const SYS_io_submit: SyscallNumber = 248;
pub const SYS_io_cancel: SyscallNumber = 249;
pub const SYS_fadvise64: SyscallNumber = 250;
// There is no 251 (was used as sys_set_zone_reclaim)
pub const SYS_exit_group: SyscallNumber = 252;
pub const SYS_lookup_dcookie: SyscallNumber = 253;
pub const SYS_epoll_create: SyscallNumber = 254;
pub const SYS_epoll_ctl: SyscallNumber = 255;
pub const SYS_epoll_wait: SyscallNumber = 256;
pub const SYS_remap_file_pages: SyscallNumber = 257;
pub const SYS_set_tid_address: SyscallNumber = 258;
pub const SYS_timer_create: SyscallNumber = 259;
pub const SYS_timer_settime: SyscallNumber = SYS_timer_create + 1;
pub const SYS_timer_gettime: SyscallNumber = SYS_timer_create + 2;
pub const SYS_timer_getoverrun: SyscallNumber = SYS_timer_create + 3;
pub const SYS_timer_delete: SyscallNumber = SYS_timer_create + 4;
pub const SYS_clock_settime: SyscallNumber = SYS_timer_create + 5;
pub const SYS_clock_gettime: SyscallNumber = SYS_timer_create + 6;
pub const SYS_clock_getres: SyscallNumber = SYS_timer_create + 7;
pub const SYS_clock_nanosleep: SyscallNumber = SYS_timer_create + 8;
pub const SYS_statfs64: SyscallNumber = 268;
pub const SYS_fstatfs64: SyscallNumber = 269;
pub const SYS_tgkill: SyscallNumber = 270;
pub const SYS_utimes: SyscallNumber = 271;
pub const SYS_fadvise64_64: SyscallNumber = 272;
pub const SYS_vserver: SyscallNumber = 273;
pub const SYS_mbind: SyscallNumber = 274;
pub const SYS_get_mempolicy: SyscallNumber = 275;
pub const SYS_set_mempolicy: SyscallNumber = 276;
pub const SYS_mq_open: SyscallNumber = 277;
pub const SYS_mq_unlink: SyscallNumber = SYS_mq_open + 1;
pub const SYS_mq_timedsend: SyscallNumber = SYS_mq_open + 2;
pub const SYS_mq_timedreceive: SyscallNumber = SYS_mq_open + 3;
pub const SYS_mq_notify: SyscallNumber = SYS_mq_open + 4;
pub const SYS_mq_getsetattr: SyscallNumber = SYS_mq_open + 5;
pub const SYS_kexec_load: SyscallNumber = 283;
pub const SYS_waitid: SyscallNumber = 284;
// Reserved SYS_sys_setaltroot: SyscallNumber = 285;
pub const SYS_add_key: SyscallNumber = 286;
pub const SYS_request_key: SyscallNumber = 287;
pub const SYS_keyctl: SyscallNumber = 288;
pub const SYS_ioprio_set: SyscallNumber = 289;
pub const SYS_ioprio_get: SyscallNumber = 290;
pub const SYS_inotify_init: SyscallNumber = 291;
pub const SYS_inotify_add_watch: SyscallNumber = 292;
pub const SYS_inotify_rm_watch: SyscallNumber = 293;
pub const SYS_migrate_pages: SyscallNumber = 294;
pub const SYS_openat: SyscallNumber = 295;
pub const SYS_mkdirat: SyscallNumber = 296;
pub const SYS_mknodat: SyscallNumber = 297;
pub const SYS_fchownat: SyscallNumber = 298;
pub const SYS_futimesat: SyscallNumber = 299;
pub const SYS_fstatat64: SyscallNumber = 300;
pub const SYS_unlinkat: SyscallNumber = 301;
pub const SYS_renameat: SyscallNumber = 302;
pub const SYS_linkat: SyscallNumber = 303;
pub const SYS_symlinkat: SyscallNumber = 304;
pub const SYS_readlinkat: SyscallNumber = 305;
pub const SYS_fchmodat: SyscallNumber = 306;
pub const SYS_faccessat: SyscallNumber = 307;
pub const SYS_pselect6: SyscallNumber = 308;
pub const SYS_ppoll: SyscallNumber = 309;
pub const SYS_unshare: SyscallNumber = 310;
pub const SYS_set_robust_list: SyscallNumber = 311;
pub const SYS_get_robust_list: SyscallNumber = 312;
pub const SYS_splice: SyscallNumber = 313;
pub const SYS_sync_file_range: SyscallNumber = 314;
pub const SYS_tee: SyscallNumber = 315;
pub const SYS_vmsplice: SyscallNumber = 316;
pub const SYS_move_pages: SyscallNumber = 317;
pub const SYS_getcpu: SyscallNumber = 318;
pub const SYS_epoll_pwait: SyscallNumber = 319;
pub const SYS_utimensat: SyscallNumber = 320;
pub const SYS_signalfd: SyscallNumber = 321;
pub const SYS_timerfd_create: SyscallNumber = 322;
pub const SYS_eventfd: SyscallNumber = 323;
pub const SYS_fallocate: SyscallNumber = 324;
pub const SYS_timerfd_settime: SyscallNumber = 325;
pub const SYS_timerfd_gettime: SyscallNumber = 326;
pub const SYS_signalfd4: SyscallNumber = 327;
pub const SYS_eventfd2: SyscallNumber = 328;
pub const SYS_epoll_create1: SyscallNumber = 329;
pub const SYS_dup3: SyscallNumber = 330;
pub const SYS_pipe2: SyscallNumber = 331;
pub const SYS_inotify_init1: SyscallNumber = 332;
pub const SYS_preadv: SyscallNumber = 333;
pub const SYS_pwritev: SyscallNumber = 334;
pub const SYS_rt_tgsigqueueinfo: SyscallNumber = 335;
pub const SYS_perf_event_open: SyscallNumber = 336;
pub const SYS_recvmmsg: SyscallNumber = 337;
pub const SYS_fanotify_init: SyscallNumber = 338;
pub const SYS_fanotify_mark: SyscallNumber = 339;
pub const SYS_prlimit64: SyscallNumber = 340;
pub const SYS_name_to_handle_at: SyscallNumber = 341;
pub const SYS_open_by_handle_at: SyscallNumber = 342;
pub const SYS_clock_adjtime: SyscallNumber = 343;
pub const SYS_syncfs: SyscallNumber = 344;
pub const SYS_sendmmsg: SyscallNumber = 345;
pub const SYS_setns: SyscallNumber = 346;
pub const SYS_process_vm_readv: SyscallNumber = 347;
pub const SYS_process_vm_writev: SyscallNumber = 348;
pub const SYS_kcmp: SyscallNumber = 349;
pub const SYS_finit_module: SyscallNumber = 350;
pub const SYS_sched_setattr: SyscallNumber = 351;
pub const SYS_sched_getattr: SyscallNumber = 352;
pub const SYS_renameat2: SyscallNumber = 353;
pub const SYS_seccomp: SyscallNumber = 354;
pub const SYS_getrandom: SyscallNumber = 355;
pub const SYS_memfd_create: SyscallNumber = 356;
pub const SYS_bpf: SyscallNumber = 357;
pub const SYS_execveat: SyscallNumber = 358;
pub const SYS_socket: SyscallNumber = 359;
pub const SYS_socketpair: SyscallNumber = 360;
pub const SYS_bind: SyscallNumber = 361;
pub const SYS_connect: SyscallNumber = 362;
pub const SYS_listen: SyscallNumber = 363;
pub const SYS_accept4: SyscallNumber = 364;
pub const SYS_getsockopt: SyscallNumber = 365;
pub const SYS_setsockopt: SyscallNumber = 366;
pub const SYS_getsockname: SyscallNumber = 367;
pub const SYS_getpeername: SyscallNumber = 368;
pub const SYS_sendto: SyscallNumber = 369;
pub const SYS_sendmsg: SyscallNumber = 370;
pub const SYS_recvfrom: SyscallNumber = 371;
pub const SYS_recvmsg: SyscallNumber = 372;
pub const SYS_shutdown: SyscallNumber = 373;
pub const SYS_userfaultfd: SyscallNumber = 374;
pub const SYS_membarrier: SyscallNumber = 375;
pub const SYS_mlock2: SyscallNumber = 376;
pub const SYS_copy_file_range: SyscallNumber = 377;
pub const SYS_preadv2: SyscallNumber = 378;
pub const SYS_pwritev2: SyscallNumber = 379;
