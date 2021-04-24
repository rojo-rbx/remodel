local function assertVector(vec, x, y, z)
    assert(vec.X == x, ("%f ~= %f"):format(vec.X, x))
    assert(vec.Y == y, ("%f ~= %f"):format(vec.Y, y))
    assert(vec.Z == z, ("%f ~= %f"):format(vec.Z, z))
end

assertVector(Vector3.new(), 0, 0, 0)
assertVector(Vector3.new(1), 1, 0, 0)
assertVector(Vector3.new(1, 2), 1, 2, 0)
assertVector(Vector3.new(1, 2, 3), 1, 2, 3)

assert(tostring(Vector3.new(1, 2, 3)) == "1, 2, 3")

assertVector(Vector3.new(1, 2, 3) + Vector3.new(1, 2, 3), 2, 4, 6)
assertVector(Vector3.new(1, 2, 3) - Vector3.new(1, 2, 3), 0, 0, 0)

assert(Vector3.new(1, 2, 3) == Vector3.new(1, 2, 3))
assert(Vector3.new() ~= Vector3.new(1, 2, 3))
