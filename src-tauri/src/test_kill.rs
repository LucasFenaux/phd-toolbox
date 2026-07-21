use sysinfo::{System, Pid};
use std::process::Command;
use std::time::Duration;
use std::thread;

fn kill_process_tree(pid: Pid, sys: &System) {
    let mut pids_to_kill = vec![pid];
    let mut i = 0;
    while i < pids_to_kill.len() {
        let current_pid = pids_to_kill[i];
        for (child_pid, process) in sys.processes() {
            if let Some(parent) = process.parent() {
                if parent == current_pid {
                    pids_to_kill.push(*child_pid);
                }
            }
        }
        i += 1;
    }
    println!("PIDs to kill: {:?}", pids_to_kill);
    for p in pids_to_kill.into_iter().rev() {
        if let Some(process) = sys.process(p) {
            println!("Killing {:?}", p);
            process.kill();
        }
    }
}

fn main() {
    let mut child = Command::new("bash")
        .arg("dummy_uvicorn.sh")
        .current_dir("/Users/lucasfenaux/.gemini/antigravity/scratch/productivity-app")
        .spawn()
        .unwrap();
    
    println!("Spawned bash script with PID {}", child.id());
    thread::sleep(Duration::from_secs(2));
    
    let sys = System::new_with_specifics(
        sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::everything())
    );
    
    kill_process_tree(Pid::from_u32(child.id()), &sys);
    
    thread::sleep(Duration::from_secs(1));
    let mut sys2 = System::new_with_specifics(
        sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::everything())
    );
    let mut found = false;
    for (p, proc) in sys2.processes() {
        if proc.name().contains("python") && proc.cmd().join(" ").contains("child running") {
            println!("Python process leaked! PID: {}", p);
            found = true;
        }
    }
    if !found {
        println!("No leaks!");
    }
}
