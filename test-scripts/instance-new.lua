local folder = Instance.new("Folder")
assert(folder ~= nil)
assert(folder.Name == "Folder")
assert(folder.ClassName == "Folder")
assert(folder.Parent == nil)

local other = Instance.new("Model")
assert(other ~= nil)
assert(other.Name == "Model")
assert(other.ClassName == "Model")
assert(other.Parent == nil)

folder.Parent = other
assert(folder.Parent == other)
assert(other.Folder == folder)