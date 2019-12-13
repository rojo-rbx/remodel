local project = {
	name = "Rojo from Remodel",
	tree = {
		["$path"] = "rojo.lua",
	},
	fileLocation = "test-scripts/rojo.lua",
}

local tree = rojo.buildProject(project)

remodel.writeModelFile(tree, "temp/rojo-output.rbxmx")