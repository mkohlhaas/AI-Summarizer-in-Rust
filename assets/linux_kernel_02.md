Creation Date: 2026-08-10 16:52:16

========= Keynote Rust in the Linux Kernel Why - Greg Kroah-Hartman.srt =========

### Overview

In this talk, Linux kernel maintainer Greg discusses the massive scale of the Linux project and explains why integrating Rust is a crucial evolutionary step. While C remains foundational to the kernel, adopting Rust shifts the burden of verifying resource management, locking, and error handling from human reviewers to the compiler. This transition significantly reduces security vulnerabilities and eases the cognitive load on kernel maintainers.

### Key Points

* **Linux runs the world:** Linux remains the largest software project in existence, powering cloud infrastructure, Android devices, smart home appliances, and more, with new releases deployed every 8 to 9 weeks.
* **Reviewers are the bottleneck:** With 5,000 developers contributing to the kernel, the primary constraint is human review. Keep-it-in-your-head state tracking makes reviewing C code incredibly difficult.
* **Rust shifts verification to the compiler:** By leveraging features like the `?` error operator and compile-time lock guarding, Rust ensures that code will not compile if its resource-handling or locking logic is flawed.
* **Eliminating 60% to 80% of security bugs:** Automating the validation of resource lifetimes, memory safety, and locking rules instantly eradicates the vast majority of common kernel vulnerabilities.
* **Crashing safely vs. exploitable bugs:** Rust is not a silver bullet; logic errors (like array out-of-bounds) still happen, but Rust forces a safe runtime crash rather than allowing memory corruption that leads to exploits.
* **Rust is in the kernel today:** The kernel currently contains about 65,000 lines of Rust code, supporting key components like Nvidia/ARM GPU drivers, Qualcomm SOC drivers, and Android Binder.
* **Rust makes C code better:** Designing safe Rust bindings has forced kernel developers to re-evaluate, clean up, and secure archaic C APIs that have existed for decades.

