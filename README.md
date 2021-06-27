/dev/leftpad
===

Linux kernel module for left-padding strings. In Rust.


Motivation
---
I've been contemplating making something in Rust for a while, and now that [Rust in The Kernel](https://lkml.org/lkml/2021/4/14/1023]) is all the rage FOMO kicked in I guess? \
So this is my Rust hello world.


Build & load
---
On Ubuntu 20.04:
```shell
sudo apt install build-essential llvm clang-11 linux-headers-"$(uname -r)" curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly
rustup component add --toolchain=nightly rust-src rustfmt

cd leftpad
make CC=clang-11
make load
```


Usage
---
Write the target len, the padding char and the string-to-be-padded, separated by spaces into the leftpad file. \
Read the padded string back from the file.
```shell
echo 3 0 7  > /dev/leftpad
cat /dev/leftpad
```
Mind the newline...
```shell
echo -n 3 0 7  > /dev/leftpad
cat /dev/leftpad; echo
```
Akchually, the padding char can be a string.
```shell
echo 16 na ' batman!' > /dev/leftpad
cat /dev/leftpad
```
It supports UTF-8 at a certain extent: the target length is measured in Unicode Scalar Value units, so it might not does exactly what you expect, but it works if you dont push it too hard.
```shell
echo 16 💩 psicle > /dev/leftpad
cat /dev/leftpad
```
The general intent was to mimic the operation of [JavaScrip padStart()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/padStart)


FQA
---

Q: But https://github.com/Rust-for-Linux/linux? \
A: Yes!

Q: But the write ignores the offset! \
A: Yes!

Q: But it's racy! \
A: Yes!

Q: You should have used sysctl! \
A: LOL!


References
---
https://lkml.org/lkml/2016/3/31/1176 \
https://github.com/fishinabarrel/linux-kernel-module-rust \
https://github.com/jbaublitz/knock-out/issues/9 \
https://github.com/jbaublitz/knock-out/issues/11 \
https://github.com/lizhuohua/linux-kernel-module-rust/issues/1
