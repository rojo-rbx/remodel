local root = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]

assert(root == root)

local child = root:GetChildren()[1]
local childAgain = root:GetChildren()[1]

assert(child == childAgain)
assert(child.Parent == childAgain.Parent)

assert(root ~= child)