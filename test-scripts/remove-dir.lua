local contents = "Hello, world!"
remodel.createDirAll("temp/nested")
remodel.writeFile("temp/nested/foo.txt", contents)

remodel.removeDir("temp/nested")

local ok = pcall(function()
	remodel.readFile("temp/nested/foo.txt")
end)

assert(ok == false)