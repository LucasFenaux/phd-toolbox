use sysinfo::{System, ProcessRefreshKind, RefreshKind, Pid};

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
    
    for p in pids_to_kill.into_iter().rev() {
        if let Some(process) = sys.process(p) {
            process.kill();
        }
    }
}
fn main() {}
