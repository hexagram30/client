# Exploration 1

## Goal

Create a stand-alone work that is useful to some personal, professional, or
academic intersest and to do so using a new set of interesting yet unfamilar
tools.

## Project

An ASCII world in ANSI color, based upon Herbert Wolverson's
Rust tutorial<sup>[1][1]</sup> for Rogue-like<sup>[2][2]</sup> games.

## Questions

* **What new (to you!) web tool, application, or program did you try out this week? What new tech skills did you learn from doing this exploration?**

   * The Rust programming language ([Wikipedia article](https://en.wikipedia.org/wiki/Rust_(programming_language)), [project page](https://www.rust-lang.org/), [source code](https://github.com/rust-lang)), one that I am only faintly familiar with and one that I have never used to create a project from scratch (I’ve only used it previously to make small modifications to others’ code, and not with any real understanding of what I was doing).
   * [The Roguelike Toolkit for Rust](https://github.com/thebracket/rltk_rs), a library for the Rust programming language that allows one to create ASCII-based games, traditionally for exploring caves and dungeons. (The [roguelike genre of games](https://en.wikipedia.org/wiki/Roguelike) takes its name from the 1980 game by the name of Rogue, though the term rose to popularity in the early to mid 1990s. Games of the same genre predate Rogue by as much as five years, with the first fully roguelike game having been released in 1978.)

* **Were you able to create something useful to your Final Project, or your job, or one of your passions? (Did you at least enjoy your exploration?)**

   * I was able to create several demos of a small roguelike world, following [the excellent tutorial](https://bfnightly.bracketproductions.com/rustbook/) provided by the software library’s author. In all of the demos I was able to fully customize the behaviour I desired, and by the fourth demo, I had diverged significantly from the tutorial, using a tiny debugging library I had created as well as pulling game data in from a configuration file (the tutorial hard-codes all of that).

* **Do you see yourself using this application or program in the future? For what? If not, why not?**

   * Progress was so good that I expect further explorations to yield more fruit, and there is a good chance I will be able to use this codebase for future efforts in creating an initial game client for the Hexagram30 project. Not only will I be able to utilize the library’s mechanisms for navigating in a world, it also supports custom textual rendering, so I will be able to support dynamic text rendering in different parts of the screen, including an area for messaging fellow players, in-game.

* **How did you spend your 5 hours?  What did you do in that time?**

   1. Within seconds of seeing the note in the Unit 1 grading comment recommending that we begin thinking about our semester project, the idea for using Rust to create a roguelike popped into my head. After a few minutes of pondering, I arrived at a possible plan to make it work for the assignment and also drive my own Hexagram30 project forward.
   1. I probably spent about two hours doing intense investigation: first, reviewing the tutorial space online for the Rust programming language in the genre of the old-school games in which I was interested; then trying out examples that had been created with the tools from the different tutorials, seeing which were easier to install, which ones ran faster and more smoothly, etc.; then finally, comparing the tutorials themselves, looking at code style, pedagogical approach and the like.
   1. After selecting the best library and corresponding tutorial, I created a Rust project and began walking through the instructions; I had executable code ready to test almost immediately. After 2 hours, I had not only created four separate demos, but had begun the process of making my own modifications to the code.

<!-- Named page links below: /-->

[1]: http://bfnightly.bracketproductions.com/rustbook/chapter_0.html
[2]: https://en.wikipedia.org/wiki/Roguelike
