local contents = "Hello, world!"
remodel.writeFile("temp/foo.txt", contents)

remodel.removeFile("temp/foo.txt")

local ok = pcall(function()
	remodel.readFile("temp/foo.txt")
end)

assert(ok == false)