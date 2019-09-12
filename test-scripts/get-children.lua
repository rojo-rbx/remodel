local game = remodel.load("test-models/folder-and-value.rbxmx")

local children = game:GetChildren()
assert(#children == 1)

local folder = children[1]
assert(folder.Name == "Root")

local values = folder:GetChildren()
assert(#values == 2)

local stringValue = values[1]
assert(stringValue.Name == "String")

local numberValue = values[2]
assert(numberValue.Name == "Number")