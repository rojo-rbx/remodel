local function assertCFramePosition(vec, x, y, z)
    assert(vec.X == x, ("%f ~= %f"):format(vec.X, x))
    assert(vec.Y == y, ("%f ~= %f"):format(vec.Y, y))
    assert(vec.Z == z, ("%f ~= %f"):format(vec.Z, z))
end

local function assertVector(vec, x, y, z)
    assert(vec.X == x, ("x: %f ~= %f (%s)"):format(vec.X, x, tostring(vec)))
    assert(vec.Y == y, ("y: %f ~= %f (%s)"):format(vec.Y, y, tostring(vec)))
    assert(vec.Z == z, ("z: %f ~= %f (%s)"):format(vec.Z, z, tostring(vec)))
end

-- new with combinations of integer and floats
assertCFramePosition(CFrame.new(), 0, 0, 0)
assertCFramePosition(CFrame.new(1, 2, 3), 1, 2, 3)
assertCFramePosition(CFrame.new(1.5, 2, 3), 1.5, 2, 3)
assertCFramePosition(CFrame.new(1, 2.5, 3), 1, 2.5, 3)
assertCFramePosition(CFrame.new(1, 2, 3.5), 1, 2, 3.5)
assertCFramePosition(CFrame.new(1.5, 2.5, 3), 1.5, 2.5, 3)
assertCFramePosition(CFrame.new(1, 2.5, 3.5), 1, 2.5, 3.5)
assertCFramePosition(CFrame.new(1.5, 2.5, 3.5), 1.5, 2.5, 3.5)

-- new from Vector3
assertCFramePosition(CFrame.new(Vector3.new(1, 2, 3)), 1, 2, 3)

-- properties
assertVector(CFrame.new().XVector, 1, 0, 0)
assertVector(CFrame.new().YVector, 0, 1, 0)
assertVector(CFrame.new().ZVector, 0, 0, 1)

assertVector(CFrame.new().RightVector, 1, 0, 0)
assertVector(CFrame.new().UpVector, 0, 1, 0)
assertVector(CFrame.new().LookVector, -0, -0, -1)

assert(tostring(CFrame.new(7, 8, 9)) == "7, 8, 9, 1, 0, 0, 0, 1, 0, 0, 0, 1", "got " .. tostring(CFrame.new()))
