# minesweeper-lib

_Minesweeper logic without Runtime stuff to use as a backend with any programming language_

<br>

### Two words (or a bit more)

It's written on Rust so it doesn't have a runtime, garbage collector and any other stuff that gets program bigger during it's runtime, makes it taking an enormous amount of RAM, forcing CPU to process more memory and spend extra operations/time on working with cleaning memory/reference counting stuff and so on, or even to work with a heavy interpretators like [CPython](https://github.com/python/cpython) and [V8](https://v8.dev/) or with a much heavier platforms like [JVM](https://ru.wikipedia.org/wiki/Java_Virtual_Machine), [.NET/.NET Core](https://dotnet.microsoft.com/en-us/) or [NodeJS](https://nodejs.dev/). Speaking of platforms, they takes a huge peace of a hard disk space also and they are being hard to setup and use especially on no-name operating systems or incompatible hardware.

Moreover, Rust has the best typesystem in the whole humanity and programs written on it have no runtime panics or segmentation faults, like C/C++ ones. Actually, it's possible, but this program should have panics or "unsafe" sections inside it's body. And at this point my program doesn't have them at all (and you can check by trying to, maybe, grep them by "panic" or "unsafe" keywords).

<br/>

### Testing the minesweeper logic itself:

<details>
  <summary>Dependencies</summary>

- [Cargo](https://doc.rust-lang.org/cargo/)
</details>

<br/>

```sh
$ make test_logic
```

<br/>

### Running the game inside Python:

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
