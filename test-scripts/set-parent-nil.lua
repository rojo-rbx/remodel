local root = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]
assert(root.Parent == nil)

local stringValue = root.String
assert(stringValue ~= nil)
assert(stringValue.Parent == root)

stringValue.Parent = nil

assert(stringValue.Parent == nil)
assert(root:FindFirstChild("String") == nil)

remodel.writeModelFile("temp/written-from-nil.rbxmx", stringValue)

local reloaded = remodel.readModelFile("temp/written-from-nil.rbxmx")[1]

assert(reloaded.Parent == nil)
assert(reloaded.Name == "String")