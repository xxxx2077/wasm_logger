from bcc import BPF
import ctypes as ct
import datetime
import argparse
import os
import re
import yaml

def get_pid(program_name):
    get_pid_command = "ps -ef | grep %s | grep -v grep |grep -v program | awk \'{print $2}\'" % program_name
    result = os.popen(get_pid_command)
    pid = result.read()

    if "\n" in pid:
        pid = pid.strip("\n")

    print(pid)
    if pid == "":
        print("program not found")
        return 0
    return int(pid)

def extract_python_template(template):
    start = len(template.split("[")[0])
    location = "[" + template.split("[")[1]
    print("python", start, location)
    return start, location

def extract_golang_template(template):
    start = len(template.split("/")[0]) + 2
    location = template.split("/")[1].split(",")[0]
    print("golang", start, location)
    return start, location

parser = argparse.ArgumentParser(
    description="filter logs of specific program and location",
    formatter_class=argparse.RawDescriptionHelpFormatter)
parser.add_argument("--program", type=str, default="hello",
                    help="only filter the program named here")
parser.add_argument("--language", type=str, default="python",
                    help="for filter template")
parser.add_argument("--template", type=str, default="2022-08-15 10:13:22 - ERROR - [/data/home/logbench/python/hello.py:26]",
                    help="only filter the template named here")

args = parser.parse_args()

pid = get_pid(args.program)

if args.language == "python":
    start, location = extract_python_template(args.template)
elif args.language == "golang":
    start, location = extract_golang_template(args.template)

class WritePackage(ct.Structure):
    _fields_ = [
        ("pid", ct.c_uint),
        ("fd", ct.c_uint),
        ("contents", ct.c_char * 64),
        ("fileName", ct.c_char * 32)
    ]

b = BPF(src_file="logReducer.c")

b.attach_kprobe(event="do_sys_open", fn_name="detect_file_open")
b.attach_kprobe(event=b.get_syscall_fnname("write"), fn_name="detect_file_write")
b.attach_kretprobe(event="do_sys_open", fn_name="detect_file_open_ret")
b.attach_kprobe(event="io_uring_sqe", fn_name="detect_io_uring_write")

def trigger_alert_write(cpu, data, size):
    event = ct.cast(data, ct.POINTER(WritePackage)).contents
    contents = event.contents.decode('utf-8', 'ignore')
    timestamp = datetime.datetime.now()
    print("%-29s (pid:%ld fd:%ld name:%s):%s" % (
        timestamp,
        event.pid,
        event.fd,
        event.fileName,
        contents))

def trigger_alert_find(cpu, data, size):
    global b
    listen_map = b["listen_map"]
    print("trigger")
    for listen_key in listen_map:
        listen_val = listen_map[listen_key]
        if listen_val.needListen == 0:
            path = os.readlink("/proc/%d/fd/%d" %
                               (listen_key.pid, listen_key.fd))
            filename = path.split("/")[-1]
            if re.match(r".+\.log$", filename):
                listen_map[listen_key] = listen_map.Leaf(
                    1, bytes(filename, encoding="utf-8"))
            else:
                listen_map[listen_key] = listen_map.Leaf(2, b"")

b["events_write"].open_perf_buffer(trigger_alert_write)
b["events_find"].open_perf_buffer(trigger_alert_find)

listen_map = b.get_table("listen_map")
pid_map = b.get_table("pid_map")
path = '/proc/%d/fd' % pid
for fd in os.listdir(path):
    filename = os.readlink("%s/%s" % (path, fd)).split("/")[-1]
    if filename:
        print(pid, filename, fd)
        listen_key = listen_map.Key(pid, int(fd))
        if re.match(r".+\.log$", filename):
            listen_map[listen_key] = listen_map.Leaf(
                1, bytes(filename, encoding="utf-8"))
        else:
            listen_map[listen_key] = listen_map.Leaf(2, b"")
    pid_map[ct.c_uint(pid)] = ct.c_uint(pid)

filter_map = b.get_table("filter_map")
start_map = b.get_table("start_map")

filter_map[ct.c_uint(0)] = filter_map.Leaf(
    bytes(location, encoding="utf-8"))
start_map[ct.c_uint(0)] = ct.c_uint(start)

print("start reading... ")
while 1:
    try:
        b.perf_buffer_poll()
    except KeyboardInterrupt:
        exit()