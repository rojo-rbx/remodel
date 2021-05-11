local folder = Instance.new("Folder")

assert(folder:IsA("Folder"))
assert(folder:IsA("Instance"))
assert(not folder:IsA("BasePart"))

local spawnLocation = Instance.new("SpawnLocation")

assert(spawnLocation:IsA("SpawnLocation"))
assert(spawnLocation:IsA("Part"))
assert(spawnLocation:IsA("FormFactorPart"))
assert(spawnLocation:IsA("BasePart"))
assert(spawnLocation:IsA("PVInstance"))
assert(spawnLocation:IsA("Instance"))
assert(not spawnLocation:IsA("Folder"))
