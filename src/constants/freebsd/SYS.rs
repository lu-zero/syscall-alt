// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


use ::SyscallNumber;


// Originate in https://github.com/freebsd/freebsd/blob/master/sys/kern/syscalls.master
// We may want to add back some of the NOPROTO calls
pub const SYS_abort2: SyscallNumber = 463;
pub const SYS_accept4: SyscallNumber = 541;
pub const SYS_accept: SyscallNumber = 30;
pub const SYS_access: SyscallNumber = 33;
pub const SYS_acct: SyscallNumber = 51;
pub const SYS_adjtime: SyscallNumber = 140;
pub const SYS_aio_cancel: SyscallNumber = 316;
pub const SYS_aio_error: SyscallNumber = 317;
pub const SYS_aio_fsync: SyscallNumber = 465;
pub const SYS_aio_mlock: SyscallNumber = 543;
pub const SYS_aio_read: SyscallNumber = 255;
pub const SYS_aio_return: SyscallNumber = 314;
pub const SYS_aio_suspend: SyscallNumber = 315;
pub const SYS_aio_waitcomplete: SyscallNumber = 359;
pub const SYS_aio_write: SyscallNumber = 256;
pub const SYS_audit: SyscallNumber = 445;
pub const SYS_auditctl: SyscallNumber = 453;
pub const SYS_auditon: SyscallNumber = 446;
pub const SYS_bind: SyscallNumber = 104;
pub const SYS_bindat: SyscallNumber = 538;
pub const SYS_cap_enter: SyscallNumber = 516;
pub const SYS_cap_fcntls_get: SyscallNumber = 537;
pub const SYS_cap_fcntls_limit: SyscallNumber = 536;
pub const SYS_cap_getmode: SyscallNumber = 517;
pub const SYS_cap_ioctls_get: SyscallNumber = 535;
pub const SYS_cap_ioctls_limit: SyscallNumber = 534;
pub const SYS_cap_rights_limit: SyscallNumber = 533;
pub const SYS_chdir: SyscallNumber = 12;
pub const SYS_chflags: SyscallNumber = 34;
pub const SYS_chflagsat: SyscallNumber = 540;
pub const SYS_chmod: SyscallNumber = 15;
pub const SYS_chown: SyscallNumber = 16;
pub const SYS_chroot: SyscallNumber = 61;
pub const SYS_clock_getcpuclockid2: SyscallNumber = 247;
pub const SYS_clock_getres: SyscallNumber = 234;
pub const SYS_clock_gettime: SyscallNumber = 232;
pub const SYS_clock_settime: SyscallNumber = 233;
pub const SYS_close: SyscallNumber = 6;
pub const SYS_closefrom: SyscallNumber = 509;
pub const SYS_connect: SyscallNumber = 98;
pub const SYS_connectat: SyscallNumber = 539;
pub const SYS_cpuset: SyscallNumber = 484;
pub const SYS_cpuset_getaffinity: SyscallNumber = 487;
pub const SYS_cpuset_getid: SyscallNumber = 486;
pub const SYS_cpuset_setaffinity: SyscallNumber = 488;
pub const SYS_cpuset_setid: SyscallNumber = 485;
pub const SYS_dup2: SyscallNumber = 90;
pub const SYS_dup: SyscallNumber = 41;
pub const SYS_eaccess: SyscallNumber = 376;
pub const SYS_execve: SyscallNumber = 59;
pub const SYS_extattrctl: SyscallNumber = 355;
pub const SYS_extattr_delete_fd: SyscallNumber = 373;
pub const SYS_extattr_delete_file: SyscallNumber = 358;
pub const SYS_extattr_delete_link: SyscallNumber = 414;
pub const SYS_extattr_get_fd: SyscallNumber = 372;
pub const SYS_extattr_get_file: SyscallNumber = 357;
pub const SYS_extattr_get_link: SyscallNumber = 413;
pub const SYS_extattr_list_fd: SyscallNumber = 437;
pub const SYS_extattr_list_file: SyscallNumber = 438;
pub const SYS_extattr_list_link: SyscallNumber = 439;
pub const SYS_extattr_set_fd: SyscallNumber = 371;
pub const SYS_extattr_set_file: SyscallNumber = 356;
pub const SYS_extattr_set_link: SyscallNumber = 412;
pub const SYS_faccessat: SyscallNumber = 489;
pub const SYS_fchdir: SyscallNumber = 13;
pub const SYS_fchflags: SyscallNumber = 35;
pub const SYS_fchmod: SyscallNumber = 124;
pub const SYS_fchmodat: SyscallNumber = 490;
pub const SYS_fchown: SyscallNumber = 123;
pub const SYS_fchownat: SyscallNumber = 491;
pub const SYS_fcntl: SyscallNumber = 92;
pub const SYS_fdatasync: SyscallNumber = 550;
pub const SYS_fexecve: SyscallNumber = 492;
pub const SYS_ffclock_getcounter: SyscallNumber = 241;
pub const SYS_ffclock_getestimate: SyscallNumber = 243;
pub const SYS_ffclock_setestimate: SyscallNumber = 242;
pub const SYS_fhopen: SyscallNumber = 298;
pub const SYS_fhstat: SyscallNumber = 299;
pub const SYS_fhstatfs: SyscallNumber = 398;
pub const SYS_flock: SyscallNumber = 131;
pub const SYS_fork: SyscallNumber = 2;
pub const SYS_fpathconf: SyscallNumber = 192;
pub const SYS_fstat: SyscallNumber = 189;
pub const SYS_fstatat: SyscallNumber = 493;
pub const SYS_fstatfs: SyscallNumber = 397;
pub const SYS_fsync: SyscallNumber = 95;
pub const SYS_ftruncate: SyscallNumber = 480;
pub const SYS_futimens: SyscallNumber = 546;
pub const SYS_futimes: SyscallNumber = 206;
pub const SYS_futimesat: SyscallNumber = 494;
pub const SYS_getaudit: SyscallNumber = 449;
pub const SYS_getaudit_addr: SyscallNumber = 451;
pub const SYS_getauid: SyscallNumber = 447;
pub const SYS_getcontext: SyscallNumber = 421;
pub const SYS_getdents: SyscallNumber = 272;
pub const SYS_getdirentries: SyscallNumber = 196;
pub const SYS_getdtablesize: SyscallNumber = 89;
pub const SYS_getegid: SyscallNumber = 43;
pub const SYS_geteuid: SyscallNumber = 25;
pub const SYS_getfh: SyscallNumber = 161;
pub const SYS_getfsstat: SyscallNumber = 395;
pub const SYS_getgid: SyscallNumber = 47;
pub const SYS_getgroups: SyscallNumber = 79;
pub const SYS_getitimer: SyscallNumber = 86;
pub const SYS_getlogin: SyscallNumber = 49;
pub const SYS_getloginclass: SyscallNumber = 523;
pub const SYS_getpeername: SyscallNumber = 31;
pub const SYS_getpgid: SyscallNumber = 207;
pub const SYS_getpgrp: SyscallNumber = 81;
pub const SYS_getpid: SyscallNumber = 20;
pub const SYS_getppid: SyscallNumber = 39;
pub const SYS_getpriority: SyscallNumber = 100;
pub const SYS_getresgid: SyscallNumber = 361;
pub const SYS_getresuid: SyscallNumber = 360;
pub const SYS_getrlimit: SyscallNumber = 194;
pub const SYS_getrusage: SyscallNumber = 117;
pub const SYS_getsid: SyscallNumber = 310;
pub const SYS_getsockname: SyscallNumber = 32;
pub const SYS_getsockopt: SyscallNumber = 118;
pub const SYS_gettimeofday: SyscallNumber = 116;
pub const SYS_getuid: SyscallNumber = 24;
pub const SYS_ioctl: SyscallNumber = 54;
pub const SYS_issetugid: SyscallNumber = 253;
pub const SYS_jail: SyscallNumber = 338;
pub const SYS_jail_attach: SyscallNumber = 436;
pub const SYS_jail_get: SyscallNumber = 506;
pub const SYS_jail_remove: SyscallNumber = 508;
pub const SYS_jail_set: SyscallNumber = 507;
pub const SYS_kenv: SyscallNumber = 390;
pub const SYS_kevent: SyscallNumber = 363;
pub const SYS_kill: SyscallNumber = 37;
pub const SYS_kldfind: SyscallNumber = 306;
pub const SYS_kldfirstmod: SyscallNumber = 309;
pub const SYS_kldload: SyscallNumber = 304;
pub const SYS_kldnext: SyscallNumber = 307;
pub const SYS_kldstat: SyscallNumber = 308;
pub const SYS_kldsym: SyscallNumber = 337;
pub const SYS_kldunload: SyscallNumber = 305;
pub const SYS_kldunloadf: SyscallNumber = 444;
pub const SYS_kqueue: SyscallNumber = 362;
pub const SYS_ktimer_create: SyscallNumber = 235;
pub const SYS_ktimer_delete: SyscallNumber = 236;
pub const SYS_ktimer_getoverrun: SyscallNumber = 239;
pub const SYS_ktimer_gettime: SyscallNumber = 238;
pub const SYS_ktimer_settime: SyscallNumber = 237;
pub const SYS_ktrace: SyscallNumber = 45;
pub const SYS_lchflags: SyscallNumber = 391;
pub const SYS_lchmod: SyscallNumber = 274;
pub const SYS_lchown: SyscallNumber = 254;
pub const SYS_lgetfh: SyscallNumber = 160;
pub const SYS_link: SyscallNumber = 9;
pub const SYS_linkat: SyscallNumber = 495;
pub const SYS_lio_listio: SyscallNumber = 257;
pub const SYS_listen: SyscallNumber = 106;
pub const SYS_lpathconf: SyscallNumber = 513;
pub const SYS_lseek: SyscallNumber = 478;
pub const SYS_lstat: SyscallNumber = 190;
pub const SYS_lutimes: SyscallNumber = 276;
pub const SYS_mac_syscall: SyscallNumber = 394;
pub const SYS_madvise: SyscallNumber = 75;
pub const SYS_mincore: SyscallNumber = 78;
pub const SYS_minherit: SyscallNumber = 250;
pub const SYS_mkdir: SyscallNumber = 136;
pub const SYS_mkdirat: SyscallNumber = 496;
pub const SYS_mkfifo: SyscallNumber = 132;
pub const SYS_mkfifoat: SyscallNumber = 497;
pub const SYS_mknod: SyscallNumber = 14;
pub const SYS_mknodat: SyscallNumber = 498;
pub const SYS_mlock: SyscallNumber = 203;
pub const SYS_mlockall: SyscallNumber = 324;
pub const SYS_mmap: SyscallNumber = 477;
pub const SYS_modfind: SyscallNumber = 303;
pub const SYS_modfnext: SyscallNumber = 302;
pub const SYS_modnext: SyscallNumber = 300;
pub const SYS_modstat: SyscallNumber = 301;
pub const SYS_mount: SyscallNumber = 21;
pub const SYS_mprotect: SyscallNumber = 74;
pub const SYS_msync: SyscallNumber = 65;
pub const SYS_munlock: SyscallNumber = 204;
pub const SYS_munlockall: SyscallNumber = 325;
pub const SYS_munmap: SyscallNumber = 73;
pub const SYS_nanosleep: SyscallNumber = 240;
pub const SYS_nfstat: SyscallNumber = 279;
pub const SYS_nlstat: SyscallNumber = 280;
pub const SYS_nmount: SyscallNumber = 378;
pub const SYS_nosys: SyscallNumber = 0;
pub const SYS_nstat: SyscallNumber = 278;
pub const SYS_ntp_adjtime: SyscallNumber = 176;
pub const SYS_ntp_gettime: SyscallNumber = 248;
pub const SYS_numa_getaffinity: SyscallNumber = 548;
pub const SYS_numa_setaffinity: SyscallNumber = 549;
pub const SYS_obreak: SyscallNumber = 17;
pub const SYS_open: SyscallNumber = 5;
pub const SYS_openat: SyscallNumber = 499;
pub const SYS_ovadvise: SyscallNumber = 72;
pub const SYS_pathconf: SyscallNumber = 191;
pub const SYS_pdfork: SyscallNumber = 518;
pub const SYS_pdgetpid: SyscallNumber = 520;
pub const SYS_pdkill: SyscallNumber = 519;
pub const SYS_pipe2: SyscallNumber = 542;
pub const SYS_poll: SyscallNumber = 209;
pub const SYS_posix_fadvise: SyscallNumber = 531;
pub const SYS_posix_fallocate: SyscallNumber = 530;
pub const SYS_posix_openpt: SyscallNumber = 504;
pub const SYS_ppoll: SyscallNumber = 545;
pub const SYS_pread: SyscallNumber = 475;
pub const SYS_preadv: SyscallNumber = 289;
pub const SYS_procctl: SyscallNumber = 544;
pub const SYS_profil: SyscallNumber = 44;
pub const SYS_pselect: SyscallNumber = 522;
pub const SYS_ptrace: SyscallNumber = 26;
pub const SYS_pwrite: SyscallNumber = 476;
pub const SYS_pwritev: SyscallNumber = 290;
pub const SYS_quotactl: SyscallNumber = 148;
pub const SYS_rctl_add_rule: SyscallNumber = 528;
pub const SYS_rctl_get_limits: SyscallNumber = 527;
pub const SYS_rctl_get_racct: SyscallNumber = 525;
pub const SYS_rctl_get_rules: SyscallNumber = 526;
pub const SYS_rctl_remove_rule: SyscallNumber = 529;
pub const SYS_read: SyscallNumber = 3;
pub const SYS_readlink: SyscallNumber = 58;
pub const SYS_readlinkat: SyscallNumber = 500;
pub const SYS_readv: SyscallNumber = 120;
pub const SYS_reboot: SyscallNumber = 55;
pub const SYS_recvfrom: SyscallNumber = 29;
pub const SYS_recvmsg: SyscallNumber = 27;
pub const SYS_rename: SyscallNumber = 128;
pub const SYS_renameat: SyscallNumber = 501;
pub const SYS_revoke: SyscallNumber = 56;
pub const SYS_rfork: SyscallNumber = 251;
pub const SYS_rmdir: SyscallNumber = 137;
pub const SYS_rtprio: SyscallNumber = 166;
pub const SYS_rtprio_thread: SyscallNumber = 466;
pub const SYS_sbrk: SyscallNumber = 69;
pub const SYS_sched_getparam: SyscallNumber = 328;
pub const SYS_sched_getscheduler: SyscallNumber = 330;
pub const SYS_sched_get_priority_max: SyscallNumber = 332;
pub const SYS_sched_get_priority_min: SyscallNumber = 333;
pub const SYS_sched_rr_get_interval: SyscallNumber = 334;
pub const SYS_sched_setparam: SyscallNumber = 327;
pub const SYS_sched_setscheduler: SyscallNumber = 329;
pub const SYS_sched_yield: SyscallNumber = 331;
pub const SYS_select: SyscallNumber = 93;
pub const SYS_sendfile: SyscallNumber = 393;
pub const SYS_sendmsg: SyscallNumber = 28;
pub const SYS_sendto: SyscallNumber = 133;
pub const SYS_setaudit: SyscallNumber = 450;
pub const SYS_setaudit_addr: SyscallNumber = 452;
pub const SYS_setauid: SyscallNumber = 448;
pub const SYS_setcontext: SyscallNumber = 422;
pub const SYS_setegid: SyscallNumber = 182;
pub const SYS_seteuid: SyscallNumber = 183;
pub const SYS_setfib: SyscallNumber = 175;
pub const SYS_setgid: SyscallNumber = 181;
pub const SYS_setgroups: SyscallNumber = 80;
pub const SYS_setitimer: SyscallNumber = 83;
pub const SYS_setlogin: SyscallNumber = 50;
pub const SYS_setloginclass: SyscallNumber = 524;
pub const SYS_setpgid: SyscallNumber = 82;
pub const SYS_setpriority: SyscallNumber = 96;
pub const SYS_setregid: SyscallNumber = 127;
pub const SYS_setresgid: SyscallNumber = 312;
pub const SYS_setresuid: SyscallNumber = 311;
pub const SYS_setreuid: SyscallNumber = 126;
pub const SYS_setrlimit: SyscallNumber = 195;
pub const SYS_setsid: SyscallNumber = 147;
pub const SYS_setsockopt: SyscallNumber = 105;
pub const SYS_settimeofday: SyscallNumber = 122;
pub const SYS_setuid: SyscallNumber = 23;
pub const SYS_shm_open: SyscallNumber = 482;
pub const SYS_shm_unlink: SyscallNumber = 483;
pub const SYS_shutdown: SyscallNumber = 134;
pub const SYS_sigaction: SyscallNumber = 416;
pub const SYS_sigaltstack: SyscallNumber = 53;
pub const SYS_sigpending: SyscallNumber = 343;
pub const SYS_sigprocmask: SyscallNumber = 340;
pub const SYS_sigqueue: SyscallNumber = 456;
pub const SYS_sigreturn: SyscallNumber = 417;
pub const SYS_sigsuspend: SyscallNumber = 341;
pub const SYS_sigtimedwait: SyscallNumber = 345;
pub const SYS_sigwait: SyscallNumber = 429;
pub const SYS_sigwaitinfo: SyscallNumber = 346;
pub const SYS_socket: SyscallNumber = 97;
pub const SYS_socketpair: SyscallNumber = 135;
pub const SYS_sstk: SyscallNumber = 70;
pub const SYS_stat: SyscallNumber = 188;
pub const SYS_statfs: SyscallNumber = 396;
pub const SYS_swapcontext: SyscallNumber = 423;
pub const SYS_swapoff: SyscallNumber = 424;
pub const SYS_swapon: SyscallNumber = 85;
pub const SYS_symlink: SyscallNumber = 57;
pub const SYS_symlinkat: SyscallNumber = 502;
pub const SYS_sync: SyscallNumber = 36;
pub const SYS_sysarch: SyscallNumber = 165;
pub const SYS_sys_exit: SyscallNumber = 1;
pub const SYS_thr_create: SyscallNumber = 430;
pub const SYS_thr_exit: SyscallNumber = 431;
pub const SYS_thr_kill2: SyscallNumber = 481;
pub const SYS_thr_kill: SyscallNumber = 433;
pub const SYS_thr_new: SyscallNumber = 455;
pub const SYS_thr_self: SyscallNumber = 432;
pub const SYS_thr_set_name: SyscallNumber = 464;
pub const SYS_thr_suspend: SyscallNumber = 442;
pub const SYS_thr_wake: SyscallNumber = 443;
pub const SYS_truncate: SyscallNumber = 479;
pub const SYS_umask: SyscallNumber = 60;
pub const SYS_undelete: SyscallNumber = 205;
pub const SYS_unlink: SyscallNumber = 10;
pub const SYS_unlinkat: SyscallNumber = 503;
pub const SYS_unmount: SyscallNumber = 22;
pub const SYS_utimensat: SyscallNumber = 547;
pub const SYS_utimes: SyscallNumber = 138;
pub const SYS_utrace: SyscallNumber = 335;
pub const SYS_uuidgen: SyscallNumber = 392;
pub const SYS_vfork: SyscallNumber = 66;
pub const SYS_wait4: SyscallNumber = 7;
pub const SYS_wait6: SyscallNumber = 532;
pub const SYS_write: SyscallNumber = 4;
pub const SYS_writev: SyscallNumber = 121;
pub const SYS_yield: SyscallNumber = 321;
pub const SYS__umtx_op: SyscallNumber = 454;
pub const SYS___acl_aclcheck_fd: SyscallNumber = 354;
pub const SYS___acl_aclcheck_file: SyscallNumber = 353;
pub const SYS___acl_aclcheck_link: SyscallNumber = 428;
pub const SYS___acl_delete_fd: SyscallNumber = 352;
pub const SYS___acl_delete_file: SyscallNumber = 351;
pub const SYS___acl_delete_link: SyscallNumber = 427;
pub const SYS___acl_get_fd: SyscallNumber = 349;
pub const SYS___acl_get_file: SyscallNumber = 347;
pub const SYS___acl_get_link: SyscallNumber = 425;
pub const SYS___acl_set_fd: SyscallNumber = 350;
pub const SYS___acl_set_file: SyscallNumber = 348;
pub const SYS___acl_set_link: SyscallNumber = 426;
pub const SYS___cap_rights_get: SyscallNumber = 515;
pub const SYS___getcwd: SyscallNumber = 326;
pub const SYS___mac_execve: SyscallNumber = 415;
pub const SYS___mac_get_fd: SyscallNumber = 386;
pub const SYS___mac_get_file: SyscallNumber = 387;
pub const SYS___mac_get_link: SyscallNumber = 410;
pub const SYS___mac_get_pid: SyscallNumber = 409;
pub const SYS___mac_get_proc: SyscallNumber = 384;
pub const SYS___mac_set_fd: SyscallNumber = 388;
pub const SYS___mac_set_file: SyscallNumber = 389;
pub const SYS___mac_set_link: SyscallNumber = 411;
pub const SYS___mac_set_proc: SyscallNumber = 385;
pub const SYS___setugid: SyscallNumber = 374;
pub const SYS___sysctl: SyscallNumber = 202;