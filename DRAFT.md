## chapter1 INTRODUCING RUST

2016, 2017, 2018, 2019のStack Overflowのmost loved programming languageに選ばれた。  
現在の使用者のうち、使い続けたいと思っているひとの割合が最も高い。

technology leadersの間でも採用されるにいたっている。  
AWS, Dropbox, Cloudflare, Google, Microsoft, Mozilla, Samusung, Oracle

Startupsの事例
sourcegraph, Figma, Parity

as a programming language Rust's distinguishing feature is its ability to analyse prevent invalid data access
in your program at compile time. It guarantees that your program is memory safe without imposing any run time costs.

completeなcomputerへのaccessとmemory safetyを両立していること

Until late 2018: Rust is a system programming language that runs blazingly fast, prevents segfaults and guarantees thread safety.
Now: A Language empowering everyone to build reliable and efficient software.

### Safety

Rust programs are free from

- dangling pointers: live references to data that has become invalid over the course of the program.
- data races: unable to determine how a program will behave from run to run because external factors are changing
- buffer overflow: attempting to access the 12th element of an array of only 6 elements.
- iterator invalidation: an issue caused by something that is being iterated over being altered mid-way through

プログラムが安全であると知ることは、プログラマにある種の(with a degree)自由をあたえ、これがcommunity内では
"fealess concurrency"と表現されるらしい。


### Productivity

Rust has many ergonomic features. It offers generics, sophisticated, data types, pattern matching, and closures.
Programmer productivity is a difficult concept to demonstrate in a book example.

### Control

Control over memory access, memory layout and specific CPU instructions is very important squeezing the best performance out of code.


## Downside

- "High level syntax with low level performance"
- "Concurrency without crashed"
- "C with perfect safety"

のような(ときに誇張された)スローガンにあるmeritもあるが

### Compile times

Rust is slower at compiling code than its peer languages.  
It has a complex compiler toolchain that includes multiple intermediate representations and sending lots of code to LLVM.


### Strictness

Programs won't compile until everything is just right.  
Over time, it's likely that you'll come to appreciate this features.


### Size of the Language

The language is large.  
It has a type system, multiple ways to gain access to values, an ownership system that is paired with enforced object lifetimes.  
The language is also comprehensive.



## chapter2 LANGUAGE FOUNDATIONS

Rustのsyntaxやprimitive typeについて簡単に触れたあと、grep likeなcliを作ります。


## chapter12 SIGNALS, INTERRUPTS AND EXCEPTIONS

The two concepts that are most important to distinguish are signals and interrupts.  
A signal is a software-level abstraction that is associated with an operating system.  
An interrupt is a CPU-related abstraction and ic closely associated with the system's hardware.

Signals are a form of limited inter-process communication.  
They don't contain content, but their presence indicate something.

signalがsoftware interruptと表現されたりするから紛らわしい。