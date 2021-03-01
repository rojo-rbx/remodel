local valueInstance = remodel.readModelFile("test-models/binarystringvalue.rbxmx")[1]

assert(remodel.getRawProperty(valueInstance, "Value") == "S2FtcGZrYXJyZW4gd3V6IGhlcmU=")

remodel.setRawProperty(valueInstance, "Value", "BinaryString", "aGVsbG8gd29ybGQ=")
assert(remodel.getRawProperty(valueInstance, "Value") == "aGVsbG8gd29ybGQ=")
