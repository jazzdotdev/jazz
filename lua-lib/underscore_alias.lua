local underscore = require "underscore"

for _, name in ipairs(underscore.functions()) do
  _G[name] = underscore[name]
end