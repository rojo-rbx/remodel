local model = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]

local numberValue = model.Number
assert(numberValue.ClassName == "NumberValue")

local value = remodel.getRawProperty(numberValue, "Value")
assert(value == 42)

remodel.setRawProperty(numberValue, "Value", "Float64", 8)
local value = remodel.getRawProperty(numberValue, "Value")
assert(value == 8)