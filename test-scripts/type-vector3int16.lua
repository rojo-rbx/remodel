local function assertVector(vec, x, y, z)
    assert(vec.X == x, ("%f ~= %f"):format(vec.X, x))
    assert(vec.Y == y, ("%f ~= %f"):format(vec.Y, y))
    assert(vec.Z == z, ("%f ~= %f"):format(vec.Z, z))
end

assertVector(Vector3int16.new(), 0, 0, 0)
assertVector(Vector3int16.new(1), 1, 0, 0)
assertVector(Vector3int16.new(1, 2), 1, 2, 0)
assertVector(Vector3int16.new(1, 2, 3), 1, 2, 3)

assertVector(Vector3int16.new(1, 2, 3) + Vector3int16.new(1, 2, 3), 2, 4, 6)
assertVector(Vector3int16.new(1, 2, 3) - Vector3int16.new(1, 2, 3), 0, 0, 0)

assert(Vector3int16.new(1, 2, 3) == Vector3int16.new(1, 2, 3))
assert(Vector3int16.new() ~= Vector3int16.new(1, 2, 3))

assert(tostring(Vector3int16.new(1, 2, 3)) == "1, 2, 3")

local terrainRegion = remodel.readModelFile("test-models/terrain-region.rbxmx")[1]

assert(remodel.getRawProperty(terrainRegion, "ExtentsMax") == Vector3int16.new(32000, 32000, 32000))

remodel.setRawProperty(
	terrainRegion,
	"ExtentsMax",
	"Vector3int16",
	Vector3int16.new(1, 2, 3)
)
assert(remodel.getRawProperty(terrainRegion, "ExtentsMax") == Vector3int16.new(1, 2, 3))
