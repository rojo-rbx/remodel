local root = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]

assert(root ~= nil)
assert(root.Name == "Root")

local stringValue = root:FindFirstChild("String")
assert(stringValue ~= nil)
assert(stringValue.Name == "String")

local numberValue = root:FindFirstChild("Number")
assert(numberValue ~= nil)
assert(numberValue.Name == "Number")