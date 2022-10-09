if not rojo then
	print("Skipping test, unstable_rojo_api not enabled.")
	return
end

local project = {
	name = "Rojo from Remodel",
	tree = {
		["$path"] = "rojo.lua",
	},
	fileLocation = "test-scripts/rojo.lua",
}

local tree = rojo.buildProject(project)

remodel.writeModelFile("temp/rojo-output.rbxmx", tree)