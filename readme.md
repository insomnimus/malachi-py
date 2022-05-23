# Malachi-py
Python bindings for the rust [malachi](https://github.com/insomnimus/malachi) pattern matching language.

## Building the Project
Use [maturin](https://github.com/PyO3/maturin) to build a python module:

```sh
maturin build
```

## Examples
See the [example script](example.py).

## The API
The module exposes just a single class, `Command`.

It has a single constructor, accepting a string as a malachi command.

It has two methods:
- `Command.get_matches(str): Dictionary`: Returns a dictionary with the matches or None if the command did not match the input.
- `Command.has_prefix(str): bool`: Returns true if the command at least partially matches the input.

The dictionary returned from `get_matches` will have a special key representing the trailing text; the key is `"_"`.

The captures with the quantifiers `*` and `+` will have lists as the dictionary values; other quantifiers will have `str`.
