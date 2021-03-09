local root = Instance.new("Folder")
root.Name = "Root"

local childA = Instance.new("Folder")
childA.Name = "A"
childA.Parent = root

local grandchild = Instance.new("Folder")
grandchild.Name = "Grandchild"
grandchild.Parent = childA

local childB = Instance.new("Folder")
childB.Name = "B"
childB.Parent = root

local descendants = root:GetDescendants()

assert(descendants[1] == childA, "`A` was not the first in descendants, instead it was " .. tostring(descendants[1]))
assert(descendants[2] == childB, "`B` was not the first in descendants, instead it was " .. tostring(descendants[2]))
assert(descendants[3] == grandchild, "`Grandchild` was not the first in descendants, instead it was " .. tostring(descendants[3]))
assert(#descendants == 3, "There were not exactly 3 descendants.")
