<img src="builderman.png" alt="Remodel Logo" width="327" height="277" align="right" />
<h1 align="left">Remodel</h1>

![Remodel on crates.io](https://img.shields.io/crates/v/remodel.svg?label=version)

Remodel is a command line tool to manipulate Roblox files and the instances contained within them. It's intended as a building block for Roblox automation tooling.

**Remodel is still in early development. Its API will change as it reaches stability.**

## Installation

### From GitHub Releases (Windows only!)
You can download pre-built Windows binaries from [Remodel's GitHub Releases page](https://github.com/rojo-rbx/remodel/releases).

### From crates.io
You'll need Rust 1.37+

```bash
cargo install remodel
```

### Latest development changes (unstable!!)
You'll need Rust 1.37+.

```bash
cargo install --git https://github.com/rojo-rbx/remodel
```

## Quick Start
Most of Remodel's interface is its Lua API. Users write Lua 5.3 scripts that Remodel runs, providing them with a special set of APIs.

One use for Remodel is to break a place file apart into multiple model files. Imagine we have a place file named `my-place.rbxlx` that has some models stored in `ReplicatedStorage.Models`.

We want to take those models and save them to individual files in a folder named `models`.

```lua
local game = remodel.load("my-place.rbxlx")

-- If the directory does not exist yet, we'll create it.
remodel.createDirAll("models")

local Models = game.ReplicatedStorage.Models

for _, model in ipairs(Models:GetChildren()) do
	-- Save out each child as an rbxmx model
	remodel.save(model, "models/" .. model.Name .. ".rbxmx")
end
```

For more examples, see the [`examples`](examples) folder.

## API

### `remodel.load`
```
remodel.load(path: string): Instance
```

Load an rbxmx or rbxlx file from the filesystem.

Always returns a `DataModel` instance containing the file's content, since models and places can contain more than one top-level instance.

Throws on error.

### `remodel.save`
```
remodel.save(instance: Instance, path: string)
```

Saves an rbxmx or rbxlx file from the given instance.

Throws on error.

### `remodel.createDirAll`
```
remodel.createDirAll(path: string)
```
Makes a directory at the given path, as well as all parent directories that do not yet exist.

This is a thin wrapper around Rust's [`fs::create_dir_all`](https://doc.rust-lang.org/std/fs/fn.create_dir_all.html) function. Similar to `mkdir -p` from Unix.

Throws on error.

## Remodel vs rbxmk
Remodel is similar to [rbxmk](https://github.com/Anaminus/rbxmk):
* Both Remodel and rbxmk use Lua
* Remodel and rbxmk have a similar feature set and use cases
* Remodel is imperative, while rbxmk is declarative
* Remodel emulates Roblox's API, while rbxmk has its own, very unique API

## License
Remodel is available under the terms of the MIT license. See [LICENSE.txt](LICENSE.txt) for details.

[Logo source](https://pixabay.com/illustrations/factory-worker-industry-2318026/).