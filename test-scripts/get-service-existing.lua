local game = Instance.new("DataModel")

local workspace = Instance.new("Workspace")
workspace.Parent = game

local found = game:GetService("Workspace")
assert(found == workspace)