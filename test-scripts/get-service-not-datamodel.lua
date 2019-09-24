local folder = Instance.new("Folder")

local ok, err = pcall(function()
	return folder:GetService("Workspace")
end)

assert(not ok)
assert(tostring(err):find("DataModel") ~= nil)