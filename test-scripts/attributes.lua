local folder = Instance.new("Folder")

assert(folder:GetAttribute("Attribute") == nil)
folder:SetAttribute("Attribute", 10)
assert(folder:GetAttribute("Attribute") == 10)
folder:SetAttribute("Attribute", "String")
assert(folder:GetAttribute("Attribute") == "String")
folder:SetAttribute("Attribute", nil)
assert(folder:GetAttribute("Attribute") == nil)
