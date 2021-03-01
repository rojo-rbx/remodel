local terrainRegion = remodel.readModelFile("test-models/terrain-region.rbxmx")[1]

assert(remodel.getRawProperty(terrainRegion, "ExtentsMax") == Vector3int16.new(32000, 32000, 32000))

remodel.setRawProperty(
	terrainRegion,
	"ExtentsMax",
	"Vector3int16",
	Vector3int16.new(1, 2, 3)
)
assert(remodel.getRawProperty(terrainRegion, "ExtentsMax") == Vector3int16.new(1, 2, 3))
