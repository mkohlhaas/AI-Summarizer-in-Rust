Creation Date: 2026-08-10 17:05:43

========= Learning from the mistakes of history Greg Kroah-Hartman.srt =========

### Overview

This transcript features prominent Linux kernel maintainer Greg Kroah-Hartman advising hardware developers—specifically those in the RISC-V ecosystem—to learn from industry history and upstream their kernel code early in the design cycle. He explains that submitting code during the emulation phase, rather than after physical chips are manufactured, is a proven business strategy that saves time, cuts costs, and ensures immediate, seamless hardware compatibility upon launch.

### Key Takeaways

* **Upstream Early to Save Money:** Industry leaders like IBM, Intel, and ARM have proven that submitting kernel code during the emulation phase—well before physical chips leave the factory—is a highly profitable business strategy that avoids wasted development cycles.
* **The "Catch-Up" Trap:** Submitting code *after* hardware bring-up breaks the feedback loop; by the time the code is reviewed and merged, designers have moved on, and customers are forced to run outdated, buggy systems.
* **Prioritize Generic Kernel Images:** To support RISC-V's modularity, extensions should be dynamically discoverable at runtime so a single, generic kernel image can boot on all hardware. 
* **Favor Self-Describable Buses:** Hardware designers should focus on self-describing buses rather than complex, static device trees, allowing the kernel to seamlessly auto-configure itself at boot.
* **Upstreaming is Free QA:** Submitting code to the Linux community provides free, expert code review, which typically makes driver code one-third smaller, cleaner, and less vulnerable.
* **AI is Already Reviewing Patches:** Subsystems are actively leveraging LLM-based pattern matchers (such as Google’s *Sashiko* tool) to run automated, highly effective code reviews.
* **The Verdict on No-MMU (Nommu):** While useful for early hardware bring-up and testing, skipping an Memory Management Unit (MMU) is an obsolete practice; modern chip space and power budgets easily accommodate them.

