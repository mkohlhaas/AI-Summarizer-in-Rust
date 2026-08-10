Creation Date: 2026-08-10 16:59:30

========= Untrusted data in Linux How Rust is going to save us Greg Kroah.srt =========

### Overview

In this talk, Linux Foundation Fellow and core kernel maintainer Greg Kroah-Hartman discusses how integrating Rust into the Linux kernel has transitioned from an experiment to a permanent, production-ready reality. He outlines how Rust’s compile-time safety features—combined with new data validation models—can eliminate up to 80% of the kernel's security vulnerabilities (CVEs) while drastically reducing the burden on maintainers.

### Key Points

* **The "Rust Experiment" is Officially Over:** Rust is no longer an experimental language in Linux; it is a permanent, production-grade tool actively being deployed to billions of devices.
* **Shifting Bug Detection to the Compiler:** Rust checks error returns and lock releases at compile-time, eliminating roughly 60% of the common C-code bugs (like memory leaks and unreleased locks) before they even reach code review.
* **Rust Has Already Made C Better:** Inspired by Rust, kernel developers have backported safer patterns—like scoped locks and automated dereferencing guards—to clean up existing C code.
* **"All Input is Evil" & The `untrusted` Type:** Because modern developers cannot trust user-space data or buggy hardware, a new `untrusted` Rust type wrapper is being introduced to force explicit validation before data can be accessed.
* **An 80% Reduction in CVEs:** Combining Rust's native memory/lock safety with the new `untrusted` validation pipeline is projected to wipe out up to 80% of kernel security vulnerabilities.
* **Optimizing for Maintainers:** With over 5,000 developers but only 150 core maintainers, Linux is optimizing for reviewability; if a Rust patch builds successfully, maintainers can trust its safety and focus purely on logic.
* **No Mass Rewrites of Existing C Code:** To prevent regressions, working C code will not be rewritten. Instead, Rust will be introduced incrementally for new drivers, subsystems, and APIs.
* **Open and Email-Driven Evolution:** Linux continues to grow at a rate of nine changes per hour, operating entirely in public through a highly scalable, email-based workflow designed to lower the barrier for new contributors.

