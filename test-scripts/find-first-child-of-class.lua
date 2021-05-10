local root = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]

assert(root ~= nil)
assert(root.Name == "Root")

local stringValue = root:FindFirstChildOfClass("StringValue")
assert(stringValue ~= nil)
assert(stringValue.Name == "String")
assert(stringValue.ClassName == "StringValue")

local numberValue = root:FindFirstChildOfClass("NumberValue")
assert(numberValue ~= nil)
assert(numberValue.Name == "Number")
assert(numberValue.ClassName == "NumberValue")
