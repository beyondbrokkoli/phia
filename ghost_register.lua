-- ghost_register.lua
local ghost = {}
local ghost_val = ghost[1] -- Dead code: never affects the final arena state

if true then
    local shadow = 42      -- Reuses ghost_val's physical register (r5)!
end

print("ghost_test", ghost_val)
