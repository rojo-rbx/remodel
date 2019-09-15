local contents = "Hello, world!"

remodel.writeFile("temp/foo.txt", contents)

assert(remodel.readFile("temp/foo.txt") == contents)