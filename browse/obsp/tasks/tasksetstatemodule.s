.method In
0x0001 .param_count 0
0x0001 .line 11
0x0006     inc_scope
0x0007 .line 13
0x000c     int 0
0x0011     identifier Player
0x0022     method getInventory
0x0039     identifier this
0x0048     property Task
0x0057     int 1
0x005c     method_chain getItemByType
0x0074     var_assign task
0x0083 .line 14
0x0088     identifier task
0x0097     null_object
0x0098     not_equal
0x0099     branch_false 0x00fa
0x009e .line 16
0x00a3     nop
0x00a4     identifier this
0x00b3     property State
0x00c3     int 1
0x00c8     identifier task
0x00d7     method setTaskState
0x00ee     pop
0x00ef .line 17
0x00f4     nop
0x00f5 .line 19
0x00fa .label 0x04a1
0x00ff     string "Out"
0x010d     int 1
0x0112     identifier this
0x0121     method doEvent
0x0133     pop
0x0134 .line 20
0x0139     dec_scope
0x013a     return_null
