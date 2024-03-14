pub struct CpuLoad {
    pub user : u64,
    pub nice : u64,
    pub system : u64,
    pub idle : u64,
    pub iowait : u64,
    pub irq : u64,
    pub softirq : u64,
    pub steal : u64,
    pub guest : u64,
    pub guest_nice : u64,
}
