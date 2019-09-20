local models = remodel.readModelAsset("19084487")

assert(type(models) == "table")
assert(#models == 1)

local root = models[1]
assert(root.ClassName == "Model")
assert(root.Name == "lpghatguy.com")