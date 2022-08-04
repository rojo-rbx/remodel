local root = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]
assert(root.Name == "Root")

local stringValue = root:FindFirstChild("String")

remodel.writeModelFile("temp/just-stringvalue.rbxmx", stringValue)

local stringValue2 = remodel.readModelFile("temp/just-stringvalue.rbxmx")[1]
assert(stringValue2.Name == "String")