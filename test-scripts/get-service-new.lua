local game = Instance.new("DataModel")

local workspace = game:GetService("Workspace")
assert(workspace.Parent == game)
assert(workspace.ClassName == "Workspace")