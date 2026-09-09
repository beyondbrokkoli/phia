-- probe_multihop: t[0][0][i] — two const-key hops. Current: declines (the
-- feeder's parent is itself a GetTable; get_table_root stops at None).
-- Target: chained materialization of both hops in the pre-header, fast
-- store against the leaf child.
local t = {}
local mid = {}
local inner = {}
inner[0] = 0
mid[0] = inner
t[0] = mid
local i = 0
while i < 8 do
    t[0][0][i] = 1
    i = i + 1
end
