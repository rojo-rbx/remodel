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

local function contains(haystack, needle)
	for _, thing in ipairs(haystack) do
		if thing == needle then
			return true
		end
	end

	return false
end

assert(contains(descendants, childA), "`A` was not in descendants")
assert(contains(descendants, childB), "`B` was not in descendants")
assert(contains(descendants, grandchild), "`Grandchild` was not in descendants")
assert(#descendants == 3, "There were not exactly 3 descendants.")
