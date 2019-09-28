local model = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]

local stringValue = model.String
assert(stringValue.ClassName == "StringValue")

local value = remodel.getRawProperty(stringValue, "Value")
assert(value == "Hello")

remodel.setRawProperty(stringValue, "Value", "String", "Hello, world!")
local value = remodel.getRawProperty(stringValue, "Value")
assert(value == "Hello, world!")