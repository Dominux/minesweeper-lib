# minesweeper-nn-solver

Minesweeper neural network solver

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
- [Poetry](https://python-poetry.org/)
- [Cargo](https://doc.rust-lang.org/cargo/)
</details>

<br/>

Preparing:

```sh
$ cd ./nn_solver
$ poetry install
$ poetry run maturin develop -m ../py_lib/Cargo.toml
```

If you had no faced any errors then _Minesweeper_ is successfully installed and you can use it as you wish, _por ejemplo_ by running it right inside the Python REPL:

```sh
$ poetry run python
>>> from minesweeper import Minesweeper
>>> game = Minesweeper(10, 10, 10)
>>> game.open_cell(6, 3)
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
>>> game.open_cell(2, 3)
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
