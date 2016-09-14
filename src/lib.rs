// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


#![feature(asm)]

// RUST BUG: The include! macro has an incorrect path inside an expanded macro
// macro_rules! linux_like_syscall
// {
// 	($t:item) =>
// 	(
// 		#[cfg(all(any(target_os = "linux", target_os = "android"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64")))]
// 		$t
// 	)
// }


#[cfg(all(any(target_os = "linux", target_os = "android"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64", target_arch = "s390x")))] mod linux_like_syscall;
#[cfg(all(any(target_os = "linux", target_os = "android"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86", target_arch = "x86_64", target_arch = "s390x")))] pub use self::linux_like_syscall::*;