local game = Instance.new("DataModel")

local ok, err = pcall(function()
	return game:GetService("GitService") -- this service will never exist
end)

assert(not ok)
assert(tostring(err):find("GitService") ~= nil)