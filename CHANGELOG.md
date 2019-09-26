# Remodel Changelog

## Unreleased Changes
* **Breaking:** `Instance.new` now only works for instances that actually exist.
* Added `Instance:Clone()` for copying instances all over the place, as is Roblox tradition. ([#12](https://github.com/rojo-rbx/remodel/issues/12))
* Added `DataModel:GetService()` for finding services and creating them if they don't exist, like Roblox does. ([#10](https://github.com/rojo-rbx/remodel/issues/10))
* Added `remodel.getRawProperty(instance, name)` for an initial stab at reading property values.
* Fixed Remodel dropping unknown properties when reading/writing XML models. This should make Remodel's behavior line up with Rojo.
* Improved error messages in preparation for [#7](https://github.com/rojo-rbx/remodel/issues/7) to be fixed upstream.

## 0.5.0 (2019-09-21)
* Added `Instance.new` for creating instances.
* Added `Instance:Destroy()` for destroying instances instead of just parenting them to nil.
	* Unlike Roblox, no properties can be accessed on a destroyed instance or else Remodel will throw an error. Be careful!
* Added APIs for interacting with models and places on Roblox.com:
	* `remodel.readModelAsset`
	* `remodel.readPlaceAsset`
	* `remodel.writeExistingModelAsset`
	* `remodel.writeExistingPlaceAsset`
	* These APIs will pull your `.ROBLOSECURITY` cookie from Roblox Studio if you're on Windows, or you can pass a cookie explicitly using `--auth [cookie]`

## 0.4.0 (2019-09-18)
* Added `remodel.readDir` for enumerating directories.
* Added early support for `rbxm` models in `remodel.readModelFile` and `remodel.writeModelFile`.
	* When an `rbxm` model is written or read, a warning will be printed to the console.

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