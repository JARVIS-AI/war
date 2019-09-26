.method In
0x0001 .param_count 0
0x0001 .line 12
0x0006     nop
0x0007 .line 14
0x000c     int 0
0x0011     identifier this
0x0020     method onFindBestGlyph
0x003a     pop
0x003b .line 15
0x0040     nop
0x0041     return_null

.method onFindBestGlyph
0x0001 .param_count 0
0x0001 .line 17
0x0006     inc_scope
0x0007 .line 19
0x000c     var targetobj
0x0020 .line 20
0x0025     var monsterobj
0x003a .line 21
0x003f     var tempGlyph
0x0053 .line 22
0x0058     var tempDistance
0x006f .line 23
0x0074     float 100000000
0x0079     var_assign distance
0x008c .line 24
0x0091     var spawnPosition
0x00a9 .line 25
0x00ae     var spawnRotation
0x00c6 .line 28
0x00cb     identifier this
0x00da     property Stewards
0x00ed     iterator
0x00ee .label 0x0557
0x00f3     iterator_test
0x00f4     branch_false 0x0478
0x00f9     iterator_assign ei
0x0106 .line 29
0x010b     nop
0x010c     identifier monsterobj
0x0121     identifier ei
0x012e     int 1
0x0133     identifier World
0x0143     method findObjectByName
0x015e     assign
0x015f     pop
0x0160 .line 31
0x0165     identifier monsterobj
0x017a     null_object
0x017b     not_equal
0x017c     branch_false 0x03c3
0x0181 .line 33
0x0186     nop
0x0187 .line 34
0x018c     identifier this
0x019b     property Glyphs
0x01ac     iterator
0x01ad .label 0x055a
0x01b2     iterator_test
0x01b3     branch_false 0x03b2
0x01b8     iterator_assign vi
0x01c5 .line 35
0x01ca     nop
0x01cb     identifier tempGlyph
0x01df     identifier vi
0x01ec     int 1
0x01f1     identifier World
0x0201     method findObjectByName
0x021c     assign
0x021d     pop
0x021e .line 37
0x0223     identifier tempGlyph
0x0237     null_object
0x0238     not_equal
0x0239     branch_false 0x03a6
0x023e .line 39
0x0243     nop
0x0244     identifier tempDistance
0x025b     identifier tempGlyph
0x026f     int 1
0x0274     identifier monsterobj
0x0289     method getDistToActor
0x02a2     assign
0x02a3     pop
0x02a4 .line 41
0x02a9     identifier tempDistance
0x02c0     identifier distance
0x02d3     less
0x02d4     identifier tempGlyph
0x02e8     property isOccupied
0x02fd     bool false
0x02ff     equal
0x0300     and
0x0301     identifier tempGlyph
0x0315     property Enable
0x0326     bool true
0x0328     equal
0x0329     and
0x032a     branch_false 0x039b
0x032f .line 43
0x0334     nop
0x0335     identifier distance
0x0348     identifier tempDistance
0x035f     assign
0x0360     pop
0x0361 .line 44
0x0366     identifier targetobj
0x037a     identifier tempGlyph
0x038e     assign
0x038f     pop
0x0390 .line 45
0x0395     nop
0x0396 .line 46
0x039b .label 0x055d
0x03a0     nop
0x03a1 .line 47
0x03a6 .label 0x055c
0x03ab     nop
0x03ac     inc
0x03ad     goto 0x01ad
0x03b2 .label 0x055b
0x03b7     pop
0x03b8 .line 48
0x03bd     nop
0x03be .line 51
0x03c3 .label 0x0559
0x03c8     identifier targetobj
0x03dc     null_object
0x03dd     not_equal
0x03de     branch_false 0x0431
0x03e3 .line 52
0x03e8     identifier targetobj
0x03fc     int 1
0x0401     identifier monsterobj
0x0416     method onTeleport
0x042b     pop
0x042c .line 55
0x0431 .label 0x055e
0x0436     identifier distance
0x0449     float 100000000
0x044e     assign
0x044f     pop
0x0450 .line 56
0x0455     identifier targetobj
0x0469     null_object
0x046a     assign
0x046b     pop
0x046c .line 57
0x0471     nop
0x0472     inc
0x0473     goto 0x00ee
0x0478 .label 0x0558
0x047d     pop
0x047e .line 58
0x0483     dec_scope
0x0484     return_null

.method Disable
0x0001 .param_count 0
0x0001 .line 60
0x0006     inc_scope
0x0007 .line 62
0x000c     var monsterobj
0x0021 .line 63
0x0026     var glyph
0x0036 .line 66
0x003b     identifier this
0x004a     property Stewards
0x005d     iterator
0x005e .label 0x055f
0x0063     iterator_test
0x0064     branch_false 0x0205
0x0069     iterator_assign ei
0x0076 .line 67
0x007b     nop
0x007c     identifier monsterobj
0x0091     identifier ei
0x009e     int 1
0x00a3     identifier World
0x00b3     method findObjectByName
0x00ce     assign
0x00cf     pop
0x00d0 .line 70
0x00d5     identifier monsterobj
0x00ea     null_object
0x00eb     not_equal
0x00ec     branch_false 0x01f9
0x00f1 .line 72
0x00f6     nop
0x00f7     int 0
0x00fc     identifier monsterobj
0x0111     method onFollowWar
0x0127     pop
0x0128 .line 75
0x012d     identifier this
0x013c     property Glyphs
0x014d     iterator
0x014e .label 0x0562
0x0153     iterator_test
0x0154     branch_false 0x01e8
0x0159     iterator_assign vi
0x0166 .line 76
0x016b     nop
0x016c     identifier glyph
0x017c     identifier vi
0x0189     int 1
0x018e     identifier World
0x019e     method findObjectByName
0x01b9     assign
0x01ba     pop
0x01bb .line 79
0x01c0     identifier glyph
0x01d0     null_object
0x01d1     not_equal
0x01d2     branch_false 0x01dc
0x01d7 .line 83
0x01dc .label 0x0564
0x01e1     nop
0x01e2     inc
0x01e3     goto 0x014e
0x01e8 .label 0x0563
0x01ed     pop
0x01ee .line 84
0x01f3     nop
0x01f4 .line 85
0x01f9 .label 0x0561
0x01fe     nop
0x01ff     inc
0x0200     goto 0x005e
0x0205 .label 0x0560
0x020a     pop
0x020b .line 87
0x0210     string "Out"
0x021e     int 1
0x0223     identifier this
0x0232     method doEvent
0x0244     pop
0x0245 .line 88
0x024a     dec_scope
0x024b     return_null
