local function round(number)
    local integral, fraction = math.modf(number)
    return integral + (fraction > 0.5 and 1 or 0)
end

local function assertColor(color3Object, red, green, blue)
    assert(round(color3Object.r * 255) == red)
    assert(round(color3Object.R * 255) == red)
    assert(round(color3Object.g * 255) == green)
    assert(round(color3Object.G * 255) == green)
    assert(round(color3Object.b * 255) == blue)
    assert(round(color3Object.B * 255) == blue)
end

local value = remodel.readModelFile("test-models/color3value.rbxmx")[1]
local color3 = remodel.getRawProperty(value, "Value")

assertColor(color3, 255, 186, 65)

local red = 45
local green = 7
local blue = 12

local newRGBColor = Color3.fromRGB(red, green, blue)
assertColor(newRGBColor, red, green, blue)

local newColor = Color3.new(red / 255, green / 255, blue / 255)
assertColor(newColor, red, green, blue)
