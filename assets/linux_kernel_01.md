Creation Date: 2026-08-10 16:49:02

========= How Linux is built with Greg Kroah-Hartman.srt =========

### Overview

This transcript features a conversation with Greg Kroah-Hartman, a prominent Linux kernel maintainer and Linux Foundation fellow. The discussion explores how Linux became the world's most ubiquitous operating system, its strict time-based release cycle, its decentralized "trust-based" development model, and the ongoing integration of the Rust programming language into the codebase.

### Core Takeaways

* **Ubiquity & Mobile Complexity:** Linux powers billions of devices worldwide. Interestingly, mobile phone kernels run roughly three times more lines of code (~4 million) than server versions (~1.5 million) due to highly complex hardware, clock, and power management drivers.
* **Strict 9-Week Release Cycle:** Linux avoids milestone-delayed deadlines by releasing on a strict, time-based 9-week clock. This begins with a 2-week "merge window" for accepted features, followed by 7 weeks of regression and bug fixes only.
* **A Decentralized Trust Network:** Rather than relying on corporate hierarchy, the development process is a pyramid of human trust. Approximately 800 subsystem maintainers vet, test, and accept responsibility for patches from 4,000+ annual contributors before sending pull requests to Linus Torvalds.
* **Zero Project Managers:** The open-source project employs no project or product managers. Planning and prioritization are handled internally by contributing companies before they submit clean, working code upstream.
* **The "Selfish" Contribution Model:** Companies fund and contribute to Linux out of self-interest to solve their own specific hardware or software problems. Because maintainers force these solutions to be generic, everyone benefits (e.g., mobile battery efficiency changes ultimately saved data centers billions of dollars in power costs).
* **Integrating Rust:** Rust is gradually being introduced (with ~25,000 lines currently in the kernel) to eliminate common memory-safety and object-lifetime bugs. However, meshing C's memory model with Rust's strict safety rules remains highly complex.
* **An Accessible Resume Builder:** Kroah-Hartman emphasizes that contributing to Linux is an excellent way for developers to learn and boost their resumes. Beginners can easily get started by visiting [kernelnewbies.org](https://kernelnewbies.org) to help clean up formatting, coding style, and spelling in basic drivers.

