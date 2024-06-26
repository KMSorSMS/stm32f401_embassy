# monitor reset
# b *0x08000000
# b main
# b main.rs:62
# b main.rs:65
# b uc_thread/os_core.rs:47
b uc_thread/os_task.rs:26
# b uc_thread/os_core.rs:146
# b uc_thread/os_core.rs:57
# b uc_thread/os_cpu.rs:106
b uc_thread/os_cpu.rs:44
b uc_thread/os_cpu.rs:66
# ignore $bpnum 13

start

define dss
  dashboard source -output /dev/pts/$arg0
  dashboard source -style height 0
end

define dsa
  dashboard assembly -output /dev/pts/$arg0
  dashboard assembly -style height 0
end