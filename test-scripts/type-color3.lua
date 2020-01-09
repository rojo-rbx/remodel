local value = remodel.readModelFile("test-models/color3value.rbxmx")[1]

local color3 = remodel.getRawProperty(value, "Value")

local function round(number)
    local integral, fraction = math.modf(number)
    return integral + (fraction > 0.5 and 1 or 0)
end

assert(round(color3.r * 255) == 255)
assert(round(color3.g * 255) == 186)
assert(round(color3.b * 255) == 65)
