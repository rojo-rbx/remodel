local folder = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]

assert(folder.Name == "Root")

local values = folder:GetChildren()
assert(#values == 2)

local stringValue = values[1]
assert(stringValue.Name == "String")

local numberValue = values[2]
assert(numberValue.Name == "Number")