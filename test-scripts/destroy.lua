local root = Instance.new("Folder")
root.Name = "Root"

local child = Instance.new("Folder")
child.Name = "Child"
child.Parent = root

assert(#root:GetChildren() == 1)
assert(root:GetChildren()[1] == child)

child:Destroy()

assert(#root:GetChildren() == 0)

local ok = pcall(function() return child.Name end)
assert(not ok)

local ok = pcall(function() return child.Parent end)
assert(not ok)