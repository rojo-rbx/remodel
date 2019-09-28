local folder = Instance.new("Folder")

local ok = pcall(function()
	remodel.setRawProperty(folder, "PROPERTY_NAME", "Float64", "hello")
end)

assert(not ok)