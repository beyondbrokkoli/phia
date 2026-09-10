-- string_03_nested.lua  [POSITIVE — string tables under handles:
-- Table(Table(String))]. Storing a string-element table into another
-- table flips the program to handle mode; the probe shows the outer
-- handle (1), the row handle (2), and a string read THROUGH a handle
-- chain (outer[0][1] = "beta"). The arena checksums cover both: outer
-- holds handles, row holds FNV-hashed strings.
-- Pins embed physical register names BY DESIGN (see probe_01).
-- EXPECT: PROBE nested: t_r17=1 len_r17=1 t_r21=2 len_r21=2 s_r17="beta" t_r20=2 len_r20=2
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 1 LEN 2 NZ 2 CHECKSUM 8580739319088111993
-- EXPECT: NTABLES 2
local outer = {}
local row = {}
row[0] = "alpha"
row[1] = "beta"
outer[0] = row
print("nested", outer, outer[0], outer[0][1], row)
