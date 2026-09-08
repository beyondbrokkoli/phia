-- float_07_while_cond.lua  [NEGATIVE — conditions are Boolean, not truthy]
-- A float condition is not implicitly truthy (another pinned divergence
-- from Lua); 'while' demands a Boolean exactly.
-- EXPECT_BUILD_FAIL: Type Error: 'while' condition must be a Boolean
while 1.5 do
end
