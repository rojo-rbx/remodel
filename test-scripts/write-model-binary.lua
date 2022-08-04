local root = remodel.readModelFile("test-models/binary.rbxm")[1]
assert(root.Name == "Example Model")

local module = root:FindFirstChild("SomeModule")
assert(module.ClassName == "ModuleScript")

remodel.writeModelFile("temp/just-module.rbxm", module)

local module2 = remodel.readModelFile("temp/just-module.rbxm")[1]
assert(module2.ClassName == "ModuleScript")
assert(module2.Name == "SomeModule")