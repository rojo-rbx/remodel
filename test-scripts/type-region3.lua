local function assertRegion3(region, cframe, size)
    assert(
        region.CFrame == cframe,
        ("CFrame %s ~= %s"):format(region.CFrame, cframe)
    )
    assert(
        region.Size == size,
        ("Size %s ~= %s"):format(region.Size, size)
    )
end

assertRegion3(
    Region3.new(Vector3.new(2, 3, 4), Vector3.new(8, 6, 5)),
    CFrame.new(5, 4.5, 4.5),
    Vector3.new(6, 3, 1)
)

do
    local region = Region3.new(Vector3.new(1, 2, 3), Vector3.new(7, 8, 9))
    local format = tostring(region)
    assert(format == "4, 5, 6, 1, 0, 0, 0, 1, 0, 0, 0, 1; 6, 6, 6", "got " .. format)
end

local min = Vector3.new(2, 3, 4)
local max = Vector3.new(8, 6, 5)
assert(Region3.new(min, max) == Region3.new(min, max))
