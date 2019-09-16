local models = remodel.readModelFile("test-models/folder-and-value.rbxmx")

assert(type(models) == "table")
assert(#models == 1)
assert(models[1].ClassName == "Folder")
assert(models[1].Parent == nil)