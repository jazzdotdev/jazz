-- read decrypted private key file
local f = io.open("private_key", "r")
local sign_priv = f:read()

local torchbear_bin = fs.read_file("torchbear")

local priv_key = crypto.sign.load_secret(sign_priv)
local signature = priv_key:sign_detached(torchbear_bin)

local file = io.open("torchbear.sig", "w")
file:write(signature, "\n")
