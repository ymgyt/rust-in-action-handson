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

## chapter3 Compound Data Types

自作の`std::fs::File`を作りながら、struct, enumについて学びます。  
`Vec<u8>`には`io::Write`が実装されていて便利だなと思いました。

## chapter4 LIFETIMES, OWNERSHIP AND BORROWING

Rustの所有権システム、borrow checker, Rcといったトピックを扱います。

## chapter5 DATA IN DEPTH

* integerのoverflowについて  
* endian
* floating point(IEEE754の単精度倍数を通じてのbit manipulation)

chip8という命令セット?の非常に簡易的なemulatorを作りながら、bit manipulationの利用例やstackの概念を学べます。
既にRustで作成されている方もおられるようです。
https://qiita.com/yukinarit/items/4bdc821f1e46b0688d0d

## chapter6 MEMORY

* Rust uses the same underlying representation in memory for multiple semantics.  
  For example, `&T`,`&mut T` both look the same once your source code has been compiled.
  
* The terms pointer and address are sometimes used interchangeably.  
  There is one important difference. A pointer knows its referent's width in bytes
  
* Raw pointers should be considered an intermediary step between receiving something from the operation system
  or another programming language and into a reference.
  
* Raw Pointer
  * They do no own their values. The Rust compiler does not check that the referent data is still valid  
    when they're accessed.
    
  * Multiple raw pointer to the same data is allowed. Every raw pointer can have write/read access to that data.
  
```rust
fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    // transmute() interprets the bytes at the location as another types.
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };

    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7);
}
```

* HeapとStack

* Virtual Memory

## chapter 7 FILES & STORAGE

この章では、active kvという非常にシンプルなkey value storeを作ります。CRCとkey,valueの長さをもつ固定長ヘッダーとkey valueの情報を
append onlyでFileに保存して、cliでCRUD操作を提供するものです。  
この章あたりからいよいよ実用性があるものを作っていくので楽しくなっていきます。
crcやbyteorder crateの使い方も学べます。

## chapter 8 NETWORKING

* trait object
  `Box<dyn std::error::Error>` means a pointer to any type implements `std::error::Error`

  * Common usecase
    * creating collections of heterogeneous objects  
    * as a return value, they can enable functions to returning multiple concrete types.

* trust-dnsを利用した名前解決

* TUN/TAPとsmoltcpを利用して、自前でhttp GETを実装する。TUN/TAPとsmoltcpを利用する箇所が難しく、理解できませんでした。
  ネットワークまわりの理解を深めてから再挑戦したいです。

## chapter 9 TIME AND TIME KEEPING

* Some of the hardest software engineering involves distributed systems that need to agree on what the time is.



## chapter12 SIGNALS, INTERRUPTS AND EXCEPTIONS

The two concepts that are most important to distinguish are signals and interrupts.  
A signal is a software-level abstraction that is associated with an operating system.  
An interrupt is a CPU-related abstraction and ic closely associated with the system's hardware.

Signals are a form of limited inter-process communication.  
They don't contain content, but their presence indicate something.

signalがsoftware interruptと表現されたりするから紛らわしい。
