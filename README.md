# LeetCode Solutions in Rust

**Why would I solve LeetCode problems in Rust?!** I wanted to learn Rust, without committing myself to some large real-world project. LeetCode monthly challenges are good for consistently practicing.

**Why would someone not use Rust for LeetCode problems?** Mainly because of the learning curve. It's a little difficult to successfully run a Rust program in one go unlike other common languages. You don't want to fight the borrow checker in a contest. Also, C++/Java/Python/JavaScript are more expressive to quickly solve these problems.

## Running

Most of the solutions here are self contained programs. I did not use cargo because I wanted to keep it real simple.

You can copy paste a code and run it online in [Rust Playground](https://play.rust-lang.org/).

To run locally:

```bash
rustc 1234-some-file.rs -g -o run
./run
```

### With VS Code

Run the default build task to compile and run:

1. <kbd>Ctrl+Shift+p</kbd>
2. Select: "Tasks: Run build task".

Pro tip: Assign a keyboard shortcut to the default build task. I use <kbd>Ctrl+Shift+B</kbd>.
Set the backtrace level in tasks.json, if needed.

There is also a "test" task for formatting the code with `rustfmt`. I've assigned it to <kbd>Ctrl+Shift+V</kbd>.
