local root = Instance.new("Folder")
root.Name = "Root"

local parent = Instance.new("Folder")
parent.Name = "Parent"
parent.Parent = root

local child = Instance.new("Folder")
child.Name = "Child"
child.Parent = parent

local parentClone = parent:Clone()
assert(parent.Parent == root)
assert(parentClone.Parent == nil)
assert(parentClone.Name == "Parent")

assert(#parentClone:GetChildren() == 1)

local childClone = parentClone:GetChildren()[1]
assert(child.Parent == parent)
assert(childClone.Parent == parentClone)
assert(childClone.Name == "Child")