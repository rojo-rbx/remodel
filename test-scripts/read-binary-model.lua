local models = remodel.readModelFile("test-models/binary.rbxm")
assert(type(models) == "table")
assert(#models == 1)

local model = models[1]

assert(model.ClassName == "Folder")
assert(model.Name == "Example Model")

local children = model:GetChildren()
assert(#children == 2)

-- TODO: Children order is nondeterministic from rbx_binary. Once that's fixed,
-- we should assert things about child order here too.

local module = model:FindFirstChild("SomeModule")
assert(module.ClassName == "ModuleScript")

local value = model:FindFirstChild("CoolValue")
assert(value.ClassName == "StringValue")