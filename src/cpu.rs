#[derive(Debug, Clone)]
pub struct CpuInfo {
    processor usize,
    vendor_id String,
    cpu_family String,
    model String,
    model_name String,
    stepping String,
    microcode String,
    cpu_mhz usize,
    cache_size usize,
    physical_id usize,
    siblings usize,
    core_id usize,
    cpu_cores   usize,
    apicid usize,
    initial_apicid usize,
    fpu bool,
    fpu_exception
    cpuid_level
    wp
    flags
    vmx_flags
    bugs
    bogomips
    clflush_size
    cache_alignment
    address_sizes
    power_management
}

/// Split a list into a list of lists
pub fn split_list<T: std::cmp::PartialEq>(ls: Vec<T>, sep: T) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = vec![];
    let mut temp: Vec<T> = vec![];
    for x in ls {
        if x == sep {
            result.push(temp);
            temp = vec![];
        } else {
            temp.push(x);
        }
    }
    result
}
