# monitor reset
# b *0x08000000
# b main
b main.rs:62
b main.rs:60
b uc_thread/os_core.rs:47
b uc_thread/os_task.rs:20
start -y

define dss
  dashboard source -output /dev/pts/$arg0
  dashboard source -style height 0
end

define dsa
  dashboard assembly -output /dev/pts/$arg0
  dashboard assembly -style height 0
end