use sys_info::{
    proc_total,
    cpu_num,
    mem_info,
    hostname,
    linux_os_release,
    os_release
};

use std::process::Command;

use sysinfo::{
    CpuExt,
    SystemExt,
    System
};

use std::env::var;

pub fn all_infos(color: &str) -> String {
    let get_type = linux_os_release().unwrap().pretty_name.unwrap();

    let total_mem:f32 = (mem_info().unwrap().total/102400) as f32 / 10.0;
    let free_mem:f32 = (mem_info().unwrap().free/102400) as f32 / 10.0;

    let host = hostname().unwrap();

    let kernel = os_release().unwrap();
    let uptime = get_uptime();

    let procs = proc_total().unwrap();

    let cpu = get_cpus();
    let shell = get_shell();
    format!("


      {color}OS\x1b[0m -> {get_type}
{color}HOSTNAME\x1b[0m -> {host}
  {color}KERNEL\x1b[0m -> {kernel}
  {color}UPTIME\x1b[0m -> {uptime}
     {color}MEM\x1b[0m -> {free_mem}Gb/{total_mem}Gb (free/total)
     {color}CPU\x1b[0m -> {cpu}
   {color}PROCS\x1b[0m -> {procs}
   {color}SHELL\x1b[0m -> {shell}
    ")

}

fn get_uptime() -> String {
    let uptime = Command::new("uptime").arg("-p").output().expect("Can't execute uptime command");
    String::from_utf8_lossy(&uptime.stdout)[3..].trim().to_string()
}

fn get_cpus() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    format!("{} x{}", sys.cpus()[0].brand(), cpu_num().unwrap()).to_string()
}

fn get_shell() -> String {
    match var("SHELL") {
        Result::Ok(shell) => shell.to_string(),
        Result::Err(_) => "Can't get the shell".to_string()
    }
}
