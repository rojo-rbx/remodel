local function assertRegion3int16(region, min, max)
    assert(
        region.Min == min,
        ("Min %s ~= %s"):format(region.Min, min)
    )
    assert(
        region.Max == max,
        ("Max %s ~= %s"):format(region.Max, max)
    )
end

assertRegion3int16(
    Region3int16.new(Vector3int16.new(2, 3, 4), Vector3int16.new(8, 6, 5)),
    Vector3int16.new(2, 3, 4),
    Vector3int16.new(8, 6, 5)
)

do
    local region = Region3int16.new(Vector3int16.new(1, 2, 3), Vector3int16.new(7, 8, 9))
    local format = tostring(region)
    assert(format == "1, 2, 3; 7, 8, 9", "got " .. format)
end

local min = Vector3int16.new(2, 3, 4)
local max = Vector3int16.new(8, 6, 5)
assert(Region3int16.new(min, max) == Region3int16.new(min, max))
