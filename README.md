# System Programming and OS Internals - Labs

This repository contains my laboratory assignments and projects for the **System Programming and Operating Systems** course. 

The course and this repository are structured into two main tracks:
1. **API Programming**: System programming interfaces, concurrent programming, and resource management using Rust, Linux, and Win32 APIs.
2. **OS Internals**: Architecture, design principles, and module implementation of an operating system.

---

## 📂 Repository Structure

Based on the course curriculum, the labs are divided into two main directories:

### 1. API Programming (`/API_Programming`)
This section focuses on system programming techniques within state-of-the-art operating systems (Unix/Linux and Windows). 
* **Language:** Rust 🦀
* **Key Topics:** * Rust fundamentals (ownership, borrowing, lifetimes, compound data types).
  * System calls for file systems, process/thread creation, and termination.
  * Concurrent programming with advanced synchronization primitives, message passing, and data sharing.
  * Exception and signal handling.
  * Advanced memory management (dynamic allocators, memory mapping, dynamic libraries).

**Completed Labs:**
* [x] **Lab 1: CSV Inspector** - A robust CLI tool built in Rust to parse and inspect CSV files, demonstrating safe error handling, CLI argument parsing (`std::env::args`), and I/O file management without panics.

### 2. OS Internals (`/OS_Internals`)


[Image of Operating System Architecture Diagram]

This section deals with the internal modules of an operating system and the strategies for efficient resource management.
* **Environment:** OS161 (Educational OS inspired by Unix) / C
* **Key Topics:**
  * Modifying and installing the OS161 kernel.
  * Virtual memory, pagination, and TLB management.
  * Peripheral device management and disk access (Device Drivers).
  * Implementation of synchronization primitives and system calls for I/O.
  * OS bootstrap and process/thread management.

**Completed Labs:**
* [ ] *Labs will be added here as the course progresses.*

---

## 🛠️ Technologies & Tools
* **Rust** (Cargo, standard library)
* **C / OS161** * **Linux / POSIX APIs**
* **Win32 APIs**
* **Git** for version control

## 📄 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.