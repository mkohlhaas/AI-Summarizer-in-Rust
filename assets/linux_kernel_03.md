Creation Date: 2026-08-10 16:55:44

========= How Does Linux Work Greg Kroah-Hartman.srt =========

### Overview

This text is a transcript of an interview with veteran Linux kernel maintainer Greg Kroah-Hartman. He discusses his programming background, the architectural philosophy of Linux, the development and release process of the kernel, the integration of Rust, and how the vendor-neutral Linux Foundation facilitates open-source collaboration among competing tech giants.

### Key Points

* **Deep Institutional Knowledge:** Unlike proprietary companies where engineers rotate frequently, open-source projects like Linux retain decades of specialized expertise. Over half of Linux's ~700 maintainers have managed their respective subsystems for more than a decade.
* **Monolithic Efficiency over Microkernels:** While microkernels are academically praised for security, monolithic kernels like Linux dominate in reality because direct function calls are dramatically faster and more power-efficient than message-passing systems (which can triple power consumption for basic tasks).
* **An Unbreakable User-Space Rule:** The primary rule of Linux development is to never break user-space. While internal kernel APIs are aggressively refactored, the user-to-kernel boundary remains so stable that programs written in the 1990s still run seamlessly on modern kernels.
* **Bugs and the Value of Rust:** The most severe kernel security issues stem from untrusted user-space and network data. While Kroah-Hartman strongly advocates for Rust to automatically eliminate memory-safety bugs, rewriting old code is impractical; Rust will only be used for new drivers and subsystems via highly complex C-to-Rust bindings.
* **A Relentless 24/7 Development Pace:** The Linux kernel undergoes an astonishing 9 to 10 changes per hour, 24 hours a day, 365 days a year. This relentless pace is sustained by over 5,000 developers representing hundreds of companies.
* **The Stable Branch and CVEs:** Kroah-Hartman manages the stable releases of Linux, which ingest roughly 35 bug fixes a day from Linus Torvalds' main branch. On average, 13 of these daily fixes are designated as CVEs (Common Vulnerabilities and Exposures).
* **The Minimalist Linux Foundation:** The Linux Foundation is primarily a trade and legal facilitator, not an employer of developers. In fact, only three kernel developers are directly employed by the Foundation: Linus Torvalds, Greg Kroah-Hartman, and Shuah Khan. 
* **Evolution, Not Intelligent Design:** Linux has no centralized corporate roadmap. It evolves purely by reacting to the real-world needs of its contributors—such as Valve's work on the Steam Deck, which prompted kernel-level locking optimizations to make Windows games run faster on Linux.

