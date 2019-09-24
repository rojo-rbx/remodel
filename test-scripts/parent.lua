local root = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]
assert(root.Name == "Root")
assert(root.Parent == nil)

local child1 = root:GetChildren()[1]
assert(child1 ~= nil)
assert(child1.Parent.Name == "Root")