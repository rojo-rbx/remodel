local root = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]

local stringValue = root.String
assert(stringValue ~= nil)
assert(stringValue.Parent == root)

local numberValue = root.Number
assert(numberValue ~= nil)
assert(numberValue.Parent == root)

stringValue.Parent = numberValue
assert(stringValue.Parent == numberValue)
assert(stringValue.Parent.Parent == root)