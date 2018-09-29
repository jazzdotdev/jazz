local underscore = require "utils.underscore"

for _, name in ipairs(underscore.functions()) do
  _G[name] = underscore[name]
end