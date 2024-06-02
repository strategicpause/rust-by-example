use std::io::{BufWriter, Write};
use std::ops::DerefMut;
use std::{cmp, io, process};
use procfs::process::Process;
use std::io::prelude::*;
use std::path::{PathBuf};
use memmap2::{MmapOptions};
use cgroupfs::CgroupReader;
use std::str::FromStr;
use tempfile::{NamedTempFile};

const MIB: f64 = (1024 * 1024) as f64;
const GIB: usize = 1 << 30; // 1 GiB in bytes

fn main() {
    let pid = process::id() as i32;

    println!("Initializing");
    print_mem(pid);
    anon_mem(pid);
    file_mem(pid);
    shared_mem(pid);
    println!("Cleaning up");
    print_mem(pid);
}

fn print_mem(pid: i32) {
    let cgroup_reader = get_cgroup_reader_for_pid(pid);
    let memory_stat = cgroup_reader.read_memory_stat().unwrap();

    let memory_usage = cgroup_reader.read_memory_current().unwrap() as f64;
    let max_memory = cgroup_reader.read_memory_max().unwrap() as f64;
    let swap_usage = cgroup_reader.read_memory_swap_current().unwrap() as f64;
    let swap_max = cgroup_reader.read_memory_swap_max().unwrap() as f64;
    let active_anon = memory_stat.active_anon.unwrap() as f64;
    let inactive_anon = memory_stat.inactive_anon.unwrap() as f64;
    let active_file = memory_stat.active_file.unwrap() as f64;
    let inactive_file = memory_stat.inactive_file.unwrap() as f64;
    let shmem = memory_stat.shmem.unwrap() as f64;
    let kernel = memory_stat.kernel.unwrap() as f64;
    let slab_reclaimable = memory_stat.slab_reclaimable.unwrap() as f64;

    let used = memory_usage - (active_file + inactive_file ) / 2.0 - slab_reclaimable;

    println!("Usage: {} / {} MiB", memory_usage / MIB, max_memory / MIB);
    println!("Mem Used: {} MiB", used / MIB);
    println!("Swap: {} / {} MiB", swap_usage / MIB, swap_max / MIB);
    println!("Active Anon: {} MiB", active_anon / MIB);
    println!("Inactive Anon: {} MiB", inactive_anon / MIB);
    println!("Active File: {} MiB", active_file / MIB);
    println!("Inactive File: {} MiB", inactive_file / MIB);
    println!("Shared: {} MiB", shmem / MIB);
    println!("Kernel: {} MiB", kernel / MIB);
    println!("Slab Reclaimable: {} MiB", slab_reclaimable / MIB);

    println!("Press any key to continue...");
    println!();

    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn get_cgroup_reader_for_pid(pid: i32) -> CgroupReader {
    let process = Process::new(pid).unwrap();
    let process_cgroups = process.cgroups().unwrap();
    let cgroup = process_cgroups.0.get(0).unwrap();
    let cgroup_path = cgroup.pathname.as_str();

    let cgroup_path_buf = PathBuf::from_str(cgroup_path).unwrap();

    let root_cgroup = PathBuf::from("/sys/fs/cgroup");

    CgroupReader::new_with_relative_path(root_cgroup, cgroup_path_buf).unwrap()
}

fn anon_mem(pid: i32) {
    println!("Allocating 1 GiB of Anon memory.");
    let mut mmap = MmapOptions::new().len(GIB).map_anon().unwrap();
    mmap.deref_mut().fill(0);

    print_mem(pid);
}

fn file_mem(pid: i32) {
    println!("Allocating 1 GiB of File memory.");
    let mut file = NamedTempFile::new().unwrap();
    fill_file_to_size(&mut file, GIB);

    print_mem(pid);
}

fn shared_mem(pid: i32) {
    println!("Allocating 1 GiB of shared memory.");

    let mut file = NamedTempFile::new_in("/dev/shm").unwrap();
    fill_file_to_size(&mut file, GIB);

    print_mem(pid);
}

fn fill_file_to_size(file: &mut NamedTempFile, size: usize) {
    let mut writer = BufWriter::new(file);

    let mut buffer = [1; MIB as usize];
    let mut remaining_size = size;
    while remaining_size > 0 {
        let to_write = cmp::min(remaining_size, buffer.len());
        let buffer=  &mut buffer[..to_write];
        writer.write(buffer).unwrap();

        remaining_size -= to_write;
    }
}

fn print_memory_pressure(pid: i32) {
    let cgroup_reader = get_cgroup_reader_for_pid(pid);

    let memory_pressure = cgroup_reader.read_memory_pressure().unwrap();
    let full_psi = memory_pressure.full;
    let some_psi = memory_pressure.some;
    println!("Memory PSI Some: {}", some_psi.avg10.unwrap());
    println!("Memory PSI Full: {}", full_psi.avg10.unwrap());

    let io_pressure = cgroup_reader.read_io_pressure().unwrap();
    let full_psi = io_pressure.full;
    let some_psi = io_pressure.some;
    println!("IO PSI Some: {}", some_psi.avg10.unwrap());
    println!("IO PSI Full: {}", full_psi.avg10.unwrap());

    let cpu_pressure = cgroup_reader.read_cpu_pressure().unwrap();
    let full_psi = cpu_pressure.full.unwrap();
    let some_psi = cpu_pressure.some;
    println!("CPU PSI Some: {}", some_psi.avg10.unwrap());
    println!("CPU PSI Full: {}", full_psi.avg10.unwrap());
}