local root = Instance.new("Folder")
root.Name = "Root"

local childA = Instance.new("Folder")
childA.Name = "A"
childA.Parent = root

local grandchild = Instance.new("Folder")
grandchild.Name = "Grandchild"
grandchild.Parent = childA

local greatGrandchild = Instance.new("Folder")
greatGrandchild.Name = "GreatGrandchild"
greatGrandchild.Parent = grandchild

local childB = Instance.new("Folder")
childB.Name = "B"
childB.Parent = root

local grandchildB = Instance.new("Folder")
grandchildB.Name = "GrandchildB"
grandchildB.Parent = childB

local descendants = root:GetDescendants()

local expectedDescendants = {
	childA,
	grandchild,
	greatGrandchild,
	childB,
	grandchildB,
}

assert(#descendants == #expectedDescendants, "Got bad number of descendants: " .. #descendants)

for index, expected in ipairs(expectedDescendants) do
	assert(descendants[index] == expected, "Invalid descendant at " .. index .. ": " .. descendants[index].Name)
end
