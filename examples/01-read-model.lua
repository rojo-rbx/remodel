local rojo = remodel.load("rojo.rbxmx")

local root = rojo:getRootInstance()

local function printInstance(instance, indentLevel)
	indentLevel = indentLevel or 0

	print(("    "):rep(indentLevel) .. instance.Name .. " (" .. instance.ClassName .. ")")

	for _, child in ipairs(instance:GetChildren()) do
		printInstance(child, indentLevel + 1)
	end
end

printInstance(root)