local root = Instance.new("Folder")
root.Name = "Root"

local child1 = Instance.new("Folder")
child1.Name = "Child1"
child1.Parent = root

local child2 = Instance.new("Folder")
child2.Name = "Child2"
child2.Parent = root

assert(#root:GetChildren() == 2)
assert(root:GetChildren()[1] == child1)
assert(root:GetChildren()[2] == child2)

root:ClearAllChildren()

assert(#root:GetChildren() == 0)

local ok = pcall(function() return child1.Name end)
assert(not ok)

local ok = pcall(function() return child1.Parent end)
assert(not ok)

local ok = pcall(function() return child2.Name end)
assert(not ok)

local ok = pcall(function() return child2.Parent end)
assert(not ok)