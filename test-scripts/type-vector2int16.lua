local function assertVector(vec, x, y)
    assert(vec.X == x, ("%f ~= %f"):format(vec.X, x))
    assert(vec.Y == y, ("%f ~= %f"):format(vec.Y, y))
end

assertVector(Vector2int16.new(), 0, 0, 0)
assertVector(Vector2int16.new(1), 1, 0, 0)
assertVector(Vector2int16.new(1, 2), 1, 2, 0)

assert(tostring(Vector2int16.new(1, 2)) == "1, 2")

assertVector(Vector2int16.new(1, 2) + Vector2int16.new(1, 2), 2, 4)
assertVector(Vector2int16.new(1, 2) - Vector2int16.new(1, 2), 0, 0)

assert(Vector2int16.new(1, 2) == Vector2int16.new(1, 2))
assert(Vector2int16.new() ~= Vector2int16.new(1, 2))
