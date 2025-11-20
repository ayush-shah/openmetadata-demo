# Getting Started with Rust Learning Path 🚀

Welcome! This guide will help you get started with your Rust journey.

## Quick Start

### 1. Install Rust

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts and choose option 1 (default installation)

# Restart your terminal or run:
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

Expected output:
```
rustc 1.x.x (xxxxx xxxx-xx-xx)
cargo 1.x.x (xxxxx xxxx-xx-xx)
```

### 2. Setup Your IDE

#### VS Code (Recommended)

Install these extensions:
1. **rust-analyzer** - Rust language server (essential!)
2. **CodeLLDB** - Debugging support
3. **crates** - Manage Cargo.toml dependencies
4. **Better TOML** - Syntax highlighting for Cargo.toml

#### Other IDEs
- **IntelliJ IDEA / CLion**: Install Rust plugin
- **Vim/Neovim**: Install rust.vim and configure LSP
- **Emacs**: Install rust-mode and configure LSP

### 3. Verify Your Setup

Create a test project:

```bash
cd rust-learning/phase-01-basics
cargo new hello_test
cd hello_test
cargo run
```

You should see:
```
   Compiling hello_test v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/hello_test`
Hello, world!
```

### 4. Setup OpenMetadata (Optional - needed from Phase 8 onwards)

If you don't have OpenMetadata running:

```bash
# Clone OpenMetadata quickstart
cd ~/
git clone https://github.com/open-metadata/openmetadata-demo.git
cd openmetadata-demo

# Start OpenMetadata with Docker
docker-compose up -d

# Wait for services to start (2-3 minutes)
# OpenMetadata will be available at: http://localhost:8585
```

Default credentials:
- Username: `admin`
- Password: `admin`

### 5. Get Your API Token

1. Open http://localhost:8585
2. Login with admin/admin
3. Go to Settings → Bots → ingestion-bot
4. Copy the JWT token (you'll need this in Phase 8+)

Or use the default token from the example_apis.py file in the repo.

## Learning Path Overview

### Phase 1-3: Beginner (1-2 weeks)
Focus: Rust fundamentals, ownership, structs, enums
- **Time**: 3-4 hours per day
- **Projects**: Calculator, text parser, config manager

### Phase 4-6: Intermediate (2-3 weeks)
Focus: Error handling, traits, testing, project organization
- **Time**: 3-4 hours per day
- **Projects**: Validation library, generic data structures, workspace

### Phase 7-10: Advanced (4-6 weeks)
Focus: Async, HTTP APIs, OpenMetadata SDK, CLI application
- **Time**: 4-5 hours per day
- **Projects**: HTTP client, full SDK, production CLI tool

**Total Time**: 7-11 weeks of dedicated learning

## Daily Study Routine

### Recommended Schedule (3-4 hours/day)

**Option 1: Morning Learner**
- 6:00-6:30 AM: Read concepts
- 6:30-8:00 AM: Code examples and exercises
- Evening: 30 min review and planning

**Option 2: Evening Learner**
- Lunch break: 30 min reading
- 7:00-9:30 PM: Coding and exercises
- Before bed: 30 min review

### Study Approach

1. **Read** (30 minutes)
   - Read the phase README
   - Go through examples
   - Take notes

2. **Code Along** (60-90 minutes)
   - Type out examples (don't copy-paste!)
   - Run and modify them
   - Understand each line

3. **Practice** (60-90 minutes)
   - Complete exercises
   - Try to solve without looking at solutions
   - Review solutions after attempting

4. **Build** (30 minutes)
   - Work on mini-project
   - Apply what you learned
   - Experiment with variations

5. **Review** (15-30 minutes)
   - Check knowledge checklist
   - Write summary of key concepts
   - Plan next day's topics

## Tips for Success

### 1. Fight the Borrow Checker
Don't get discouraged! Everyone struggles with it initially.
- Read error messages carefully
- Use `cargo check` frequently
- When stuck, simplify your code

### 2. Read Compiler Messages
Rust's compiler is incredibly helpful:
```
error[E0382]: borrow of moved value: `s1`
  --> src/main.rs:5:20
   |
3  |     let s1 = String::from("hello");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
4  |     let s2 = s1;
   |              -- value moved here
5  |     println!("{}", s1);
   |                    ^^ value borrowed here after move
```

Read it! It tells you exactly what's wrong.

### 3. Type Everything
Don't copy-paste code:
- Typing helps muscle memory
- Forces you to understand each character
- Makes you think about syntax

### 4. Use `cargo check` Often
Faster than `cargo build`:
```bash
cargo check  # Quick syntax check
cargo build  # Full compilation
cargo run    # Build and run
```

### 5. Read Others' Code
Study well-written Rust projects:
- [ripgrep](https://github.com/BurntSushi/ripgrep) - Fast grep
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime
- [serde](https://github.com/serde-rs/serde) - Serialization

### 6. Ask for Help
Rust community is very welcoming:
- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)
- [r/rust](https://reddit.com/r/rust)

### 7. Practice Daily
Consistency > Intensity
- 1 hour daily > 7 hours once a week
- Build the habit
- Track your progress

### 8. Don't Rush
Take time to understand:
- Phase 2 (Ownership) is crucial - spend extra time if needed
- It's okay to repeat phases
- Quality > Speed

## Common Beginner Questions

### Q: How long until I'm productive?
**A**: Basic productivity: 2-4 weeks. Comfortable: 2-3 months. Proficient: 6-12 months.

### Q: Should I learn Rust if I know Python?
**A**: Yes! Rust complements Python:
- Use Python for scripting/prototyping
- Use Rust for performance-critical parts
- Can write Python extensions in Rust (PyO3)

### Q: Is Rust hard to learn?
**A**: It has a learning curve, especially ownership. But:
- Great error messages
- Excellent documentation
- Helpful community
- Prevents bugs you'd spend hours debugging in other languages

### Q: What if I get stuck?
**A**: Normal! Try this:
1. Read the error message carefully
2. Check the phase examples
3. Search the issue (likely answered on Stack Overflow)
4. Simplify your code
5. Ask in Rust forums

### Q: Should I memorize all the syntax?
**A**: No! Focus on understanding concepts. Syntax comes with practice.

## Troubleshooting

### Issue: "cargo: command not found"
**Solution**: Restart terminal or run:
```bash
source $HOME/.cargo/env
```

### Issue: Slow compilation
**Solution**: Use `cargo check` for syntax checking (much faster):
```bash
cargo check  # Instead of cargo build
```

### Issue: "borrowed value does not live long enough"
**Solution**: Review Phase 2 (Ownership). This is the most common beginner error.

### Issue: Too many dependencies slow down builds
**Solution**: Use `sccache` to cache compilations:
```bash
cargo install sccache
export RUSTC_WRAPPER=sccache
```

## Useful Commands Reference

```bash
# Project Management
cargo new my_project          # Create new project
cargo init                    # Initialize in current dir
cargo build                   # Build project
cargo run                     # Build and run
cargo check                   # Fast syntax check
cargo test                    # Run tests
cargo doc --open             # Generate and open docs

# Code Quality
cargo fmt                     # Format code
cargo clippy                  # Linting
cargo fix                     # Auto-fix issues

# Dependencies
cargo add reqwest            # Add dependency (requires cargo-edit)
cargo update                 # Update dependencies
cargo tree                   # Show dependency tree

# Publishing
cargo publish                # Publish to crates.io
cargo install my_crate      # Install binary

# Useful Tools
cargo install cargo-edit     # cargo add/rm commands
cargo install cargo-watch    # Auto-rebuild on changes
cargo install cargo-expand   # See macro expansions
```

## Phase-by-Phase Approach

### Starting Phase 1?
1. Read the phase-01-basics/README.md
2. Run each example: `cd examples && rustc 01_variables.rs && ./01_variables`
3. Complete exercises in order
4. Don't move to Phase 2 until you complete the mini-project

### Completed a Phase?
1. Check off all items in knowledge checklist
2. Review key concepts
3. Complete the mini-project
4. Take a day to experiment and solidify knowledge
5. Move to next phase

### Stuck on a Phase?
- Spend an extra week if needed
- Phases 2, 5, and 7 are typically hardest
- Review previous phases
- Ask for help
- Take breaks

## Progress Tracking

Use the table in the main README.md to track progress:

| Phase | Status | Started | Completed | Notes |
|-------|--------|---------|-----------|-------|
| 1 | ⬜ | | | |
| 2 | ⬜ | | | |

Update it as you progress!

## Resources

### Official Documentation
- [The Rust Book](https://doc.rust-lang.org/book/) - Essential reading
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises

### Video Resources
- [Rust Crash Course](https://www.youtube.com/watch?v=zF34dRivLOw) - Traversy Media
- [Crust of Rust](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) - Jon Gjengset

### Practice
- [Exercism Rust Track](https://exercism.org/tracks/rust)
- [CodeWars Rust](https://www.codewars.com/)
- [Advent of Code](https://adventofcode.com/) - Great for practice

## Getting Help

1. **Phase Issues**: Check the phase README and examples
2. **Code Questions**: Search or ask on [users.rust-lang.org](https://users.rust-lang.org/)
3. **OpenMetadata Questions**: [OpenMetadata Slack](https://slack.open-metadata.org/)
4. **Bug Reports**: Open an issue in this repository

---

## Ready to Start?

```bash
cd rust-learning/phase-01-basics
cat README.md
```

**Let's begin your Rust mastery journey! 🦀✨**

Remember: Everyone starts as a beginner. Be patient with yourself, practice daily, and enjoy the journey!
