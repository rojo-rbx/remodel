local value = Instance.new("NumberValue")
remodel.setRawProperty(value, "Value", "Float64", 32)

local value = remodel.getRawProperty(value, "Value")
assert(value == 32)