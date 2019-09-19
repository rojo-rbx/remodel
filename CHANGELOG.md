# Remodel Changelog

## Unreleased Changes
* Added `remodel.readDir` for enumerating directories.
* Added early support for `rbxm` models in `remodel.readModelFile` and `remodel.writeModelFile`.

## 0.3.0 (2019-09-15)
* Added `remodel.writeFile` and `remodel.readFile` for handling regular files.
* Added support for `==` on instances.
* Added support for reading and writing `Parent` on instances.
* Added script file name in error stack traces.

## 0.2.0 (2019-09-14)
* Improved CLI documentation. Try `remodel --help`!
* Added support for extra arguments. They're passed into the script as `...`.
* Added support for reading from stdin. Use `-` as the input file!
	* `echo "print('Hi')" | remodel -`
* **Breaking:** split `remodel.load` into `remodel.readPlaceFile` and `remodel.readModelFile`.
	* `readPlaceFile` can only read `rbxlx` files, and returns a `DataModel` instance.
	* `readModelFile` can only read `rbxmx` files, and returns a list of instances.
* **Breaking:**: split `remodel.save` into `remodel.writePlaceFile` and `remodel.writeModelFile`.
	* `writePlaceFile` can only write `rbxlx` files.
	* `writeModelFile` can only write `rbxmx` files.
	* This split helps Remodel avoid funny tricks to detect what encoding scheme to use.

## 0.1.0 (2019-09-12)
Initial release!

* Basic API for loading and saving places, as well as creating directories
* Single-command CLI that runs a Lua 5.3 script with Remodel APIs