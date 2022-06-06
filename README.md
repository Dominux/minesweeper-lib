# minesweeper-lib

_Minesweeper logic without Runtime stuff to use as a backend with any programming language_

### TL/DR

[Try it right here, right now](https://dominux.github.io/minesweeper-lib/)

### Two words (or a bit more)

It's written on Rust so it doesn't have a runtime, garbage collector and any other stuff that gets program bigger during it's runtime, makes it taking an enormous amount of RAM, forcing CPU to process more memory and spend extra operations/time on working with cleaning memory/reference counting stuff and so on, or even to work with a heavy interpretators like [CPython](https://github.com/python/cpython) and [V8](https://v8.dev/) or with a much heavier platforms like [JVM](https://ru.wikipedia.org/wiki/Java_Virtual_Machine), [.NET/.NET Core](https://dotnet.microsoft.com/en-us/) or [NodeJS](https://nodejs.dev/). Speaking of platforms, they takes a huge peace of a hard disk space also and they are being hard to setup and use especially on no-name operating systems or incompatible hardware.

Moreover, Rust has the best typesystem in the whole humanity and programs written on it have no runtime panics or segmentation faults, like C/C++ ones. Actually, it's possible, but this program should have panics or ["unsafe"](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) sections inside it's body. And at this point my program doesn't have them at all (and you can check by trying to, maybe, grep them by "panic" or "unsafe" keywords).

### Testing the minesweeper logic itself:

<details>
  <summary>Dependencies</summary>

- [Cargo](https://doc.rust-lang.org/cargo/)
</details>

<br/>

```sh
$ make test_logic
```

### Running inside Python:

<details>
  <summary>Dependencies</summary>

- [CPython3](https://www.python.org/)
- [Poetry](https://python-poetry.org/) or [pip](https://pypi.org/project/pip/)
- [Cargo](https://doc.rust-lang.org/cargo/)
</details>

<br/>

Preparing:

- Poetry:

```sh
$ cd ./py_lib
$ poetry install --no-dev
$ poetry run maturin develop
```

- pip:

```sh
$ cd ./py_lib
$ pip install maturin
$ maturin develop
```

If you had no faced any errors then _Minesweeper_ is successfully installed and you can use it as you wish, _por ejemplo_ by running it right inside the Python REPL:

```sh
# With pip you can just run "python" or "python3",
# or any other name of your desired python interpretor
# in your $PATH

$ poetry run python
>>> from minesweeper import Minesweeper
>>> game = Minesweeper(10, 10, 10)
>>> game.open_cell(column=6, row=3)
>>> print(game.view)
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ 1 █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
>>> game.open_cell(7, 3)
>>> print(game.view)
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ 1 2 █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
>>> game.open_cell(2, row=4)
>>> print(game.view)
█ 1 0 0 0 1 █ █ █ █
1 1 0 0 0 1 █ █ █ █
0 0 0 0 0 1 2 █ █ █
0 0 0 0 0 0 1 █ █ █
0 0 0 1 1 1 1 █ █ █
1 1 1 1 █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
█ █ █ █ █ █ █ █ █ █
```

High-level wrapper provides turning internal Rust lib's errors into Pythonic ones, like next one:

```sh
>>> game.open_cell(16, 6)
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
ValueError: Coordinates out of range (column: 16, row: 6)
```

### Running inside web browser

Since web browsers support webassembly (wasm) programs, you can write webassembly code by yourself or just use any programing language that supports compiling into webassembly ([some representation of existing ones](https://webassembly.org/getting-started/developers-guide/)). Since Rust doesn't have any runtime ~~trash~~ dependencies, garbage collectors and it's platform independent language, it's excellent choice for a Wasm preprocessor because programs written in it are as lightweight and fast as a handwritten Wasm ones.

So here we have [a link to minesweeper UI](https://dominux.github.io/minesweeper-lib/). Right now the UI is Wasm driven too because [Yew](https://yew.rs/), Rust-based Web UI framework those compilation target is wasm, was used as a Web UI framework. But my nearest indent is to create a JS (actually TS ofc) driven interface too, preferably with the same UI (thanks to .css styles file that I can easily share between multiple projects), generally to show that it's also possible to use this multilingual library in the JS/TS too, and also to show similarities/differencies of traditional JS/TS and Wasm based solutions; you will be able to make your own pros/cons during comparing of them.
