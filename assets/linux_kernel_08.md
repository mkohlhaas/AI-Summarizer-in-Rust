Creation Date: 2026-08-10 17:07:23

========= Keynote Rust and Linux How the Rust Language is Going to Help Limix.srt =========

### Overview

In this talk, Linux kernel developer Greg discusses the integration of Rust into the Linux kernel and its profound impact on C development. By catching common resource-management and locking bugs at compile-time rather than review-time, Rust significantly improves kernel security and eases the burden on the kernel's scarce maintainers. Crucially, the influence of Rust has also pushed developers to modernize C APIs with similar guard mechanisms, making the entire Linux ecosystem more robust.

### Key Points

* **The Reviewer Bottleneck:** With over 5,000 developers but only about 150 active core maintainers, the kernel development process must prioritize making code reviews easier and more automated.
* **Shifting Security to Compile-Time:** Rust prevents trivial but critical bugs—like null-pointer dereferences and forgotten lock releases—by enforcing safety rules during compilation rather than at runtime.
* **Eliminating 80% of CVEs:** The speaker estimates that 80% of the kernel's historical CVEs (vulnerabilities) would have been entirely prevented if the code had been written in Rust. 
* **Rust's Positive Influence on C:** Lessons from Rust's safety paradigms have been backported to C. The kernel now uses modern C "guards" and scoped locks to automate cleanups and prevent leaks. 
* **Pragmatic Integration Policy:** The kernel community is not rewriting old, stable C code. Instead, they are leaving working code alone and focusing Rust implementation on new subsystems and drivers (such as Android’s Binder and the graphics stack).
* **The "Rust Experiment" is Over:** Rust is now a permanent, fully integrated part of the Linux kernel, boasting over 113,000 lines of Rust code (primarily bindings) and official support from major compiler toolchains like GCC.

