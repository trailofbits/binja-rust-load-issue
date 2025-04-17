# Test Instructions

Tested on Binary Ninja v5.0.7255-dev and binaryninja-api 6769870863027693679119152863982393639882.

Make sure you initialize the test binaries submodule

```bash
git submodule update --init
```

There seems to be an issue with creating multiple headless sessions and loading binaries.

There are two tests
1. `cargo test --test test_fail` fails with multiple sessions
2. `cargo test --test test_pass` passes with a single session

## Reproduce

```bash
$ cargo test --test test_fail -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running tests/test_fail.rs (target/debug/deps/test_fail-70e6a56178d5c12b)

running 1 test
Processing /Users/ekilmer/src/binaryninja/binja-rust-load-issue/tests/binaries/ROPemporium/x86/1-split/split32
Processing /Users/ekilmer/src/binaryninja/binja-rust-load-issue/tests/binaries/ROPemporium/x86/2-callme/callme32
error: test failed, to rerun pass `--test test_fail`

Caused by:
  process didn't exit successfully: `/Users/ekilmer/src/binaryninja/binja-rust-load-issue/target/debug/deps/test_fail-70e6a56178d5c12b --nocapture` (signal: 11, SIGSEGV: invalid memory reference)
```

Then, run the binary under lldb

```bash
$ lldb -- /Users/ekilmer/src/binaryninja/binja-rust-load-issue/target/debug/deps/test_fail-70e6a56178d5c12b --nocapture
(lldb) target create "/Users/ekilmer/src/binaryninja/binja-rust-load-issue/target/debug/deps/test_fail-70e6a56178d5c12b"
Current executable set to '/Users/ekilmer/src/binaryninja/binja-rust-load-issue/target/debug/deps/test_fail-70e6a56178d5c12b' (arm64).
(lldb) settings set -- target.run-args  "--nocapture"
(lldb) r
Process 82453 launched: '/Users/ekilmer/src/binaryninja/binja-rust-load-issue/target/debug/deps/test_fail-70e6a56178d5c12b' (arm64)

running 1 test
Processing /Users/ekilmer/src/binaryninja/binja-rust-load-issue/tests/binaries/ROPemporium/x86/1-split/split32
Processing /Users/ekilmer/src/binaryninja/binja-rust-load-issue/tests/binaries/ROPemporium/x86/2-callme/callme32
Process 82453 stopped
* thread #4, name = 'test_fail::test_x86_fail', stop reason = EXC_BAD_ACCESS (code=1, address=0x10)
    frame #0: 0x0000000111b7a6ac libbinaryninjacore.1.dylib`___lldb_unnamed_symbol9754 + 852
libbinaryninjacore.1.dylib`___lldb_unnamed_symbol9754:
->  0x111b7a6ac <+852>: ldr    x8, [x8, #0x10]
    0x111b7a6b0 <+856>: mov    x28, x25
    0x111b7a6b4 <+860>: mov    x0, x25
    0x111b7a6b8 <+864>: blr    x8
Target 0: (test_fail-70e6a56178d5c12b) stopped.
(lldb) bt
* thread #4, name = 'test_fail::test_x86_fail', stop reason = EXC_BAD_ACCESS (code=1, address=0x10)
  * frame #0: 0x0000000111b7a6ac libbinaryninjacore.1.dylib`___lldb_unnamed_symbol9754 + 852
    frame #1: 0x0000000111bcd984 libbinaryninjacore.1.dylib`BNLoadFilename + 336
    frame #2: 0x00000001000022e8 test_fail-70e6a56178d5c12b`binaryninja::load_with_progress::hda82de2c17f3ee56(file_path=0x000000017011e6e0, progress=NoProgressCallback @ 0x000000017011e007) at lib.rs:134:9
    frame #3: 0x0000000100002380 test_fail-70e6a56178d5c12b`binaryninja::load::h194808d777f7ebf4(file_path=0x000000017011e6e0) at lib.rs:121:5
    frame #4: 0x0000000100002208 test_fail-70e6a56178d5c12b`binaryninja::headless::Session::load::h57c40d92806f8327(self=0x000000017011e0e8, file_path=0x000000017011e6e0) at headless.rs:375:9
    frame #5: 0x0000000100002068 test_fail-70e6a56178d5c12b`binja_load_uaf::binja_load::hfe9113c2233e9ca4(path=String @ 0x000000017011e6e0) at lib.rs:27:23
    frame #6: 0x000000010000153c test_fail-70e6a56178d5c12b`test_fail::test_fail::test_x86_fail::h32cfdb090d825d9a at test_fail.rs:18:22
    frame #7: 0x0000000100000ba8 test_fail-70e6a56178d5c12b`test_fail::test_fail::test_x86_fail::_$u7b$$u7b$closure$u7d$$u7d$::h9a10fb5e3e778a27((null)=0x000000017011e75e) at test_fail.rs:6:23
    frame #8: 0x0000000100000bf8 test_fail-70e6a56178d5c12b`core::ops::function::FnOnce::call_once::h04f3c89d2a855525((null)={closure_env#0} @ 0x000000017011e75e, (null)=<unavailable>) at function.rs:250:5
    frame #9: 0x000000010004fac8 test_fail-70e6a56178d5c12b`test::__rust_begin_short_backtrace::h443bb50894df8dd3 [inlined] core::ops::function::FnOnce::call_once::ha2975a4611cc007e at function.rs:250:5 [opt]
    frame #10: 0x000000010004fac0 test_fail-70e6a56178d5c12b`test::__rust_begin_short_backtrace::h443bb50894df8dd3 at lib.rs:637:18 [opt]
    frame #11: 0x000000010004ed18 test_fail-70e6a56178d5c12b`test::run_test::_$u7b$$u7b$closure$u7d$$u7d$::h20a003e9b8e075dc [inlined] test::run_test_in_process::_$u7b$$u7b$closure$u7d$$u7d$::h04cabb2f8a79822c at lib.rs:660:60 [opt]
    frame #12: 0x000000010004ed0c test_fail-70e6a56178d5c12b`test::run_test::_$u7b$$u7b$closure$u7d$$u7d$::h20a003e9b8e075dc [inlined] _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h86639489b42bd8ed at unwind_safe.rs:272:9 [opt]
    frame #13: 0x000000010004ed0c test_fail-70e6a56178d5c12b`test::run_test::_$u7b$$u7b$closure$u7d$$u7d$::h20a003e9b8e075dc [inlined] std::panicking::try::do_call::h5ad624282f55fe03 at panicking.rs:587:40 [opt]
    frame #14: 0x000000010004ed08 test_fail-70e6a56178d5c12b`test::run_test::_$u7b$$u7b$closure$u7d$$u7d$::h20a003e9b8e075dc [inlined] std::panicking::try::h20069f0d36a62d17 at panicking.rs:550:19 [opt]
    frame #15: 0x000000010004ed08 test_fail-70e6a56178d5c12b`test::run_test::_$u7b$$u7b$closure$u7d$$u7d$::h20a003e9b8e075dc [inlined] std::panic::catch_unwind::h3089312a451a646f at panic.rs:358:14 [opt]
    frame #16: 0x000000010004ed08 test_fail-70e6a56178d5c12b`test::run_test::_$u7b$$u7b$closure$u7d$$u7d$::h20a003e9b8e075dc [inlined] test::run_test_in_process::h450cfe5b0b087665 at lib.rs:660:27 [opt]
    frame #17: 0x000000010004ec50 test_fail-70e6a56178d5c12b`test::run_test::_$u7b$$u7b$closure$u7d$$u7d$::h20a003e9b8e075dc at lib.rs:581:43 [opt]
    frame #18: 0x000000010001e264 test_fail-70e6a56178d5c12b`std::sys::backtrace::__rust_begin_short_backtrace::hc399b2d6639a98e4 [inlined] test::run_test::_$u7b$$u7b$closure$u7d$$u7d$::h64eb6aae999a7062 at lib.rs:611:41 [opt]
    frame #19: 0x000000010001e200 test_fail-70e6a56178d5c12b`std::sys::backtrace::__rust_begin_short_backtrace::hc399b2d6639a98e4 at backtrace.rs:152:18 [opt]
    frame #20: 0x00000001000214d4 test_fail-70e6a56178d5c12b`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd4b9fae0935c8011 [inlined] std::thread::Builder::spawn_unchecked_::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h7a13894e6e3828c9 at mod.rs:559:17 [opt]
    frame #21: 0x00000001000214c4 test_fail-70e6a56178d5c12b`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd4b9fae0935c8011 [inlined] _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hc871c22b1c5976c0 at unwind_safe.rs:272:9 [opt]
    frame #22: 0x00000001000214c4 test_fail-70e6a56178d5c12b`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd4b9fae0935c8011 [inlined] std::panicking::try::do_call::h1bd00fa7e68b6160 at panicking.rs:587:40 [opt]
    frame #23: 0x00000001000214c0 test_fail-70e6a56178d5c12b`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd4b9fae0935c8011 [inlined] std::panicking::try::h2d2e9886979ac95f at panicking.rs:550:19 [opt]
    frame #24: 0x00000001000214c0 test_fail-70e6a56178d5c12b`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd4b9fae0935c8011 [inlined] std::panic::catch_unwind::h44fbb6d91e1b045a at panic.rs:358:14 [opt]
    frame #25: 0x00000001000214c0 test_fail-70e6a56178d5c12b`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd4b9fae0935c8011 [inlined] std::thread::Builder::spawn_unchecked_::_$u7b$$u7b$closure$u7d$$u7d$::hf89983859c688692 at mod.rs:557:30 [opt]
    frame #26: 0x0000000100021478 test_fail-70e6a56178d5c12b`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd4b9fae0935c8011 at function.rs:250:5 [opt]
    frame #27: 0x000000010007d6b0 test_fail-70e6a56178d5c12b`std::sys::pal::unix::thread::Thread::new::thread_start::h6d53b1b0c047a3b9 [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h79019d5e89074f15 at boxed.rs:1976:9 [opt]
    frame #28: 0x000000010007d6a4 test_fail-70e6a56178d5c12b`std::sys::pal::unix::thread::Thread::new::thread_start::h6d53b1b0c047a3b9 [inlined] _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::h8d739a27f97a48eb at boxed.rs:1976:9 [opt]
    frame #29: 0x000000010007d6a0 test_fail-70e6a56178d5c12b`std::sys::pal::unix::thread::Thread::new::thread_start::h6d53b1b0c047a3b9 at thread.rs:106:17 [opt]
    frame #30: 0x000000018c105c0c libsystem_pthread.dylib`_pthread_start + 136
```
