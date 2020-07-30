// Copyright (c) Facebook, Inc. and its affiliates.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::{bail, Error, Result};
use regex::Regex;
use std::str::FromStr;
use structopt::StructOpt;

// make_option macro will build a enum of tags that map to string values by
// implementing the FromStr trait.
// This is useful when are trying to processing or display fields base on
// a user's input. Here's a use case:
// We display fields in the order of user's input. After we got
// the input array, dfill trait will automatically generate a vec of fns base
// on that array. For example, user input `--fields cpu_usage cpu_user`,
// enum generated by make_option will auto translate string to enum tags. After
// that dfill trait will generate `vec![print_cpu_usage, print_cpu_user]`. And
// the dprint trait will just iterate over the fns and call it with current model.
//
// Another user case is for the select feature, we don't want a giant match
// of string patterns once user select some field to do some operations. Instead,
// we can use a match of enum tags, that will be much faster.
macro_rules! make_option {
    ($name:ident {$($str_field:tt: $enum_field:ident,)*}) => {
        #[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
        pub enum $name {
            $($enum_field,)*
        }

        impl FromStr for $name {
            type Err = Error;

            fn from_str(opt: &str) -> Result<Self> {
                match opt.to_lowercase().as_str() {
                    $($str_field => Ok($name::$enum_field),)*
                    _ => bail!("Fail to parse {}", opt)
                }
            }
        }
    }
}

make_option! (SysField {
    "timestamp": Timestamp,
    "datetime": Datetime,
    "stat": Stat,
    "cpu": Cpu,
    "mem": Mem,
    "vm": Vm,
    "hostname": Hostname,
    "total_interrupt_ct": TotalInterruptCt,
    "context_switches": ContextSwitches,
    "boot_time_epoch_secs": BootTimeEpochSecs,
    "total_procs": TotalProcs,
    "running_procs": RunningProcs,
    "blocked_procs": BlockedProcs,
    "cpu_usage": CpuUsagePct,
    "cpu_user": CpuUserPct,
    "cpu_idle": CpuIdlePct,
    "cpu_system": CpuSystemPct,
    "cpu_nice": CpuNicePct,
    "cpu_iowait": CpuIowaitPct,
    "cpu_irq": CpuIrq,
    "cpu_softirq": CpuSoftIrq,
    "cpu_stolen": CpuStolen,
    "cpu_guest": CpuGuest,
    "cpu_guest_nice": CpuGuestNice,
    "mem_total": MemTotal,
    "mem_free": MemFree,
    "mem_available": MemAvailable,
    "mem_buffers": MemBuffers,
    "mem_cached": MemCached,
    "mem_swap_cached": MemSwapCached,
    "mem_active": MemActive,
    "mem_inactive": MemInactive,
    "mem_anon": MemAnon,
    "mem_file": MemFile,
    "mem_unevictable": MemUnevictable,
    "mem_mlocked": MemMlocked,
    "mem_swap_total": MemSwapTotal,
    "mem_swap_free": MemSwapFree,
    "mem_dirty": MemDirty,
    "mem_writeback": MemWriteback,
    "mem_anon_pages": MemAnonPages,
    "mem_mapped": MemMapped,
    "mem_shmem": MemShmem,
    "mem_kreclaimable": MemKreclaimable,
    "mem_slab": MemSlab,
    "mem_slab_reclaimable": MemSlabReclaimable,
    "mem_slab_unreclaimable": MemSlabUnreclaimable,
    "mem_kernel_stack": MemKernelStack,
    "mem_page_tables": MemPageTables,
    "mem_anon_huge_pages": MemAnonHugePages,
    "mem_shmem_huge_pages": MemShmemHugePages,
    "mem_file_huge_pages": MemFileHugePages,
    "mem_total_huge_pages": MemTotalHugePages,
    "mem_free_huge_pages": MemFreeHugePages,
    "mem_huge_page_size": MemHugePageSize,
    "mem_cma_total": MemCmaTotal,
    "mem_cma_free": MemCmaFree,
    "mem_vmalloc_total": MemVmallocTotal,
    "mem_vmalloc_used": MemVmallocUsed,
    "mem_vmalloc_chunk": MemVmallocChunk,
    "mem_direct_map_4k": MemDirectMap4k,
    "mem_direct_map_2m": MemDirectMap2m,
    "mem_direct_map_1g": MemDirectMap1g,
    "vm_pgpgin": VmPgpgin,
    "vm_pgpgout": VmPgpgout,
    "vm_pswpin": VmPswpin,
    "vm_pswpout": VmPswpout,
    "vm_psteal_kswapd": VmPstealKswapd,
    "vm_psteal_direct": VmPstealDirect,
    "vm_pscan_kswapd": VmPscanKswapd,
    "vm_pscan_direct": VmPscanDirect,
    "vm_oom_kill": VmOomKill,
});

make_option! (DiskField {
    "timestamp": Timestamp,
    "datetime": Datetime,
    "read": Read,
    "write": Write,
    "discard": Discard,
    "name": Name,
    "total": TotalBytes,
    "read_bytes": ReadBytes,
    "write_bytes": WriteBytes,
    "discard_bytes": DiscardBytes,
    "read_completed": ReadComplated,
    "read_merged": ReadMerged,
    "read_sectors": ReadSectors,
    "time_spend_read": TimeSpendRead,
    "write_completed": WriteCompleted,
    "write_merged": WriteMerged,
    "write_sectors": WriteSectors,
    "time_spend_write": TimeSpendWrite,
    "discard_completed": DiscardCompleted,
    "discard_merged": DiscardMerged,
    "discard_sectors": DiscardSectors,
    "time_spend_discard": TimeSpendDiscard,
    "major": Major,
    "minor": Minor,
});

make_option! (ProcField {
    "timestamp": Timestamp,
    "datetime": Datetime,
    "io": Io,
    "mem": Mem,
    "cpu": Cpu,
    "pid": Pid,
    "ppid": Ppid,
    "comm": Comm,
    "state": State,
    "uptime": Uptime,
    "cgroup": Cgroup,
    "cpu_user": CpuUserPct,
    "cpu_sys": CpuSysPct,
    "cpu_threads": CpuNumThreads,
    "cpu_total": CpuTotalPct,
    "mem_rss": MemRssBytes,
    "mem_minorfaults": MemMinor,
    "mem_majorfaults": MemMajor,
    "io_read": IoRead,
    "io_write": IoWrite,
    "io_total": IoTotal,
    "cmdline": Cmdline,
});

make_option! (CgroupField {
    "timestamp": Timestamp,
    "datetime": Datetime,
    "cpu": Cpu,
    "mem": Mem,
    "io": Io,
    "pressure": Pressure,
    "name": Name,
    "full_path": FullPath,
    "cpu_usage": CpuUsage,
    "cpu_user": CpuUser,
    "cpu_system": CpuSystem,
    "cpu_nr_periods": CpuNrPeriods,
    "cpu_nr_throttled": CpuNrThrottled,
    "cpu_throttled": CpuThrottled,
    "mem_total": MemTotal,
    "mem_swap": MemSwap,
    "mem_anon": MemAnon,
    "mem_file": MemFile,
    "mem_kernel": MemKernel,
    "mem_slab": MemSlab,
    "mem_sock": MemSock,
    "mem_shmem": MemShem,
    "mem_file_mapped": MemFileMapped,
    "mem_file_dirty": MemFileDirty,
    "mem_file_writeback": MemFileWriteBack,
    "mem_anon_thp": MemAnonThp,
    "mem_inactive_anon": MemInactiveAnon,
    "mem_active_anon": MemActiveAnon,
    "mem_inactive_file": MemInactiveFile,
    "mem_active_file": MemActiveFile,
    "mem_unevictable": MemUnevictable,
    "mem_slab_reclaimable": MemSlabReclaimable,
    "mem_slab_unreclaimable": MemSlabUnreclaimable,
    "mem_pgfault": Pgfault,
    "mem_pgmajfault": MemPgmajfault,
    "mem_workingset_refault": MemWorkingsetRefault,
    "mem_workingset_activate": MemWorkingsetActivate,
    "mem_workingset_nodereclaim": MemWorkingsetNodereclaim,
    "mem_pgrefill": MemPgrefill,
    "mem_pgscan": MemPgscan,
    "mem_pgsteal": MemPgsteal,
    "mem_pgactivate": MemPgactivate,
    "mem_pgdeactivate": MemPgdeactivate,
    "mem_pglazyfree": MemPglazyfree,
    "mem_pglazyfreed": MemPglazyfreed,
    "mem_thp_fault_alloc": MemTHPFaultAlloc,
    "mem_thp_collapse_alloc": MemTHPCollapseAlloc,
    "mem_high": MemHigh,
    "io_read": IoRead,
    "io_write": IoWrite,
    "io_rios": IoRiops,
    "io_wios": IoWiops,
    "io_dbps": IoDbps,
    "io_diops": IoDiops,
    "io_total": IoTotal,
    "pressure_cpu_some": CpuSome,
    "pressure_io_some": IoSome,
    "pressure_io_full": IoFull,
    "pressure_mem_full": MemFull,
    "pressure_mem_some": MemSome,
});

make_option! (IfaceField {
    "timestamp": Timestamp,
    "datetime": Datetime,
    "rate": Rate,
    "rx": Rx,
    "tx": Tx,
    "interface": Interface,
    "rx_bytes_per_sec": RBps,
    "tx_bytes_per_sec": TBps,
    "throughput_per_sec": IOBps,
    "rx_packets_per_sec": RPktps,
    "tx_packets_per_sec": TPktps,
    "collisions": Collisions,
    "multicast": Multicast,
    "rx_bytes": RxBytes,
    "rx_compressed": RxCompressed,
    "rx_crc_errors": RxCrcErr,
    "rx_dropped": RxDropped,
    "rx_errors": RxErr,
    "rx_fifo_errors": RxFifoErr,
    "rx_frame_errors": RxFrameErr,
    "rx_length_errors": RxLengthErr,
    "rx_missed_errors": RxMissedErr,
    "rx_nohandler": RxNohandler,
    "rx_over_errors": RxOverErr,
    "rx_packets": RxPckt,
    "tx_bytes": TxBytes,
    "tx_aborted_errors": TxAbortedErr,
    "tx_carrier_errors": TxCarrierErr,
    "tx_compressed": TxCompressed,
    "tx_dropped": TxDropped,
    "tx_errors": TxErr,
    "tx_fifo_errors": TxFifoErr,
    "tx_heatbeat_errors": TxHeartBeatErr,
    "tx_packets": TxPckt,
    "tx_window_errors": TxWindowErr,
});

make_option! (NetworkField {
    "timestamp": Timestamp,
    "datetime": Datetime,
    "ip": Ip,
    "ip6": Ip6,
    "icmp": Icmp,
    "Icmp6": Icmp6,
    "ip_forwarding": ForwPkts,
    "ip_in_receives": InRecvPkts,
    "ip_forw_datagrams": ForwDgrm,
    "ip_in_discards": InDiscards,
    "ip_in_delivers": InDelivers,
    "ip_out_requests": OutRequests,
    "ip_out_discards": OutDiscards,
    "ip_out_no_routes": OutNoRoutes,
    "ip_in_mcast": InMcast,
    "ip_out_mcast": OutMcast,
    "ip_in_bcast": InBcast,
    "ip_out_bcast": OutBcast,
    "ip6_in_receives": InRecvPkts6,
    "ip6_forw_datagrams": ForwDgrm6,
    "ip6_in_discards": InDiscards6,
    "ip6_in_delivers": InDelivers6,
    "ip6_out_requests": OutRequests6,
    "ip6_in_no_routes": InNoRoutes6,
    "ip6_out_no_routes": OutNoRoutes6,
    "ip6_in_hdr_err": InHdrErr6,
    "ip6_in_addr_err": InAddrErr6,
    "ip6_in_mcast": InMcast6,
    "ip6_out_mcast": OutMcast6,
    "ip6_in_bcast": InBcast6,
    "ip6_out_bcast": OutBcast6,
    "icmp_in_msgs": InMsg,
    "icmp_in_errs": InErrs,
    "icmp_in_dest_unreachs": InDestUnreachs,
    "icmp_out_msg": OutMsg,
    "icmp_out_errs": OutErrs,
    "icmp_out_dest_unreachs": OutDestUnreachs,
    "icmp6_in_msgs": InMsg6,
    "icmp6_in_errs": InErrs6,
    "icmp6_in_dest_unreachs": InDestUnreachs6,
    "icmp6_out_msg": OutMsg6,
    "icmp6_out_errs": OutErrs6,
    "icmp6_out_dest_unreachs": OutDestUnreachs6,
});

make_option!(TransportField {
    "timestamp": Timestamp,
    "datetime": Datetime,
    "tcp": Tcp,
    "udp": Udp,
    "udp6": Udp6,
    "tcp_active_opens": ActiveOpens,
    "tcp_passive_opens": PassiveOpens,
    "tcp_attempt_fails": AttemptFailed,
    "tcp_estab_reset": EstabReset,
    "tcp_curr_estab": CurrEstab,
    "tcp_in_segs": InSegs,
    "tcp_out_segs": OutSegs,
    "tcp_retrans_segs_per_sec": RetransSegsPS,
    "tcp_retrans_segs": RetransSegs,
    "tcp_in_errs": TcpInErrs,
    "tcp_out_rsts": OutRsts,
    "tcp_in_csum_errs": InCsumErrs,
    "udp_in_datagrams": InDgrms,
    "udp_no_ports": NoPorts,
    "udp_in_errs": UdpInErrs,
    "udp_out_datagrams": OutDgrms,
    "udp_recv_buf_errs": RecvBufErrs,
    "udp_snd_buf_errs": SndBufErrs,
    "udp_ignored_multi": IgnoredMulti,
    "udp6_in_datagrams": InDgrms6,
    "udp6_no_ports": NoPorts6,
    "udp6_in_errs": UdpInErrs6,
    "udp6_out_datagrams": OutDgrms6,
    "udp6_recv_buf_errs": RecvBufErrs6,
    "udp6_snd_buf_errs": SndBufErrs6,
    "udp6_in_csum_errs": InCsumErrs6,
    "udp6_ignored_multi": IgnoredMulti6,
});

make_option! (OutputFormat {
    "raw": Raw,
    "csv": Csv,
    "json": Json,
    "kv": KeyVal,
});

#[derive(Debug, StructOpt, Default, Clone)]
pub struct GeneralOpt {
    /// Show all top layer fields. If --default is specified, it overrides any specified fields via --fields.
    #[structopt(long)]
    pub default: bool,
    /// Show all fields. If --everything is specified, --fields and --default are overridden.
    #[structopt(long)]
    pub everything: bool,
    /// Show more infomation other than default.
    #[structopt(short, long)]
    pub detail: bool,
    /// Begin time, same format as replay
    #[structopt(long, short)]
    pub begin: String,
    /// End time, same format as replay
    #[structopt(long, short)]
    pub end: Option<String>,
    /// Take a regex and apply to --select selected field. See command level doc for example.
    #[structopt(long, short = "F")]
    pub filter: Option<Regex>,
    /// Sort (lower to higher) by --select selected field. See command level doc for example.
    #[structopt(long)]
    pub sort: bool,
    /// Sort (higher to lower) by --select selected field. See command level doc for example.
    #[structopt(long)]
    pub rsort: bool,
    // display top N field. See command level doc for example.
    #[structopt(long, default_value = "0")]
    pub top: u32,
    /// Repeat title, for each N line, it will render a line of title. Only for raw output format.
    #[structopt(long = "repeat-title")]
    pub repeat_title: Option<usize>,
    /// Output format. Choose from raw, csv, kv, json. Default to raw
    #[structopt(long, short = "O")]
    pub output_format: Option<OutputFormat>,
    /// Output destination, default to stdout.
    #[structopt(long, short)]
    pub output: Option<String>,
    /// Disable title in raw or csv format output
    #[structopt(long)]
    pub disable_title: bool,
}

#[derive(Debug, StructOpt, Clone)]
pub enum DumpCommand {
    /// Dump system stats
    ///
    /// ********************** Available fields **********************
    ///
    /// timestamp, datetime, hostname, total_interrupt_ct, context_switches, boot_time_epoch_secs,
    /// total_procs, running_procs, blocked_procs
    ///
    /// cpu_usage, cpu_user, cpu_idle, cpu_system, cpu_nice, cpu_iowait, cpu_irq, cpu_softirq, cpu_stolen,
    /// cpu_guest, cpu_guest_nice
    ///
    /// mem_total, mem_free, mem_available, mem_buffers, mem_cached, mem_swap_cached, mem_anon, mem_file,
    /// mem_active, mem_inactive, mem_unevictable, mem_mlocked, mem_swap_total, mem_swap_free, mem_dirty, mem_writeback,
    /// mem_anon_pages, mem_mapped, mem_shmem, mem_kreclaimable, mem_slab, mem_slab_reclaimable, mem_slab_unreclaimable,
    /// mem_kernel_stack, mem_page_tables, mem_anon_huge_pages, mem_shmem_huge_pages, mem_file_huge_pages,
    /// mem_total_huge_pages, mem_free_huge_pages, mem_huge_page_size, mem_cma_total, mem_cma_free, mem_vmalloc_total,
    /// mem_vmalloc_used, mem_vmalloc_chunk, mem_direct_map_4k, mem_direct_map_2m, mem_direct_map_1g
    ///
    /// vm_pgpgin, vm_pgpgout, vm_pswpin, vm_pswpout, vm_psteal_kswapd, vm_psteal_direct, vm_pscan_kswapd, vm_pscan_direct,
    /// vm_oom_kill
    ///
    /// ********************** Aggregated fields **********************
    ///
    /// * cpu: includes [cpu_usage, cpu_user, cpu_system]. Additionally includes [cpu_*] if --detail is specified.
    ///
    /// * mem: includes [mem_total, mem_free]. Additionally includes [mem_*] if --detail is specified.
    ///
    /// * vm: includes [vm_*].
    ///
    /// --default will have all of [hostname, cpu, mem, vm]. To display everything, use --everything.
    ///
    /// ********************** Example Commands **********************
    ///
    /// $ below dump system -b "08:30:00" -e "08:30:30" -f datetime io hostname -O csv
    ///
    /// $ below dump system -b "08:30:00" -e "08:30:30" -f datetime -O csv -f hostname -f vm
    System {
        /// Select which fields to display and in what order.
        #[structopt(short, long)]
        fields: Option<Vec<SysField>>,
        #[structopt(flatten)]
        opts: GeneralOpt,
    },
    /// Dump disk stats
    ///
    /// ********************** Available fields **********************
    ///
    /// timestamp, datetime, name, total, major, minor
    ///
    /// read_bytes, read_completed, read_merged, read_sectors, time_spend_read
    ///
    /// write_bytes, write_completed, write_merged, write_sectors, time_spend_write
    ///
    /// discard_bytes, discard_completed, discard_merged, discard_sectors, time_spend_discard
    ///
    /// ********************** Aggregated fields **********************
    ///
    /// * read: includes [read*]
    ///
    /// * write: includes [write*]
    ///
    /// * discard: includes [discard*]
    ///
    /// --default will have all of [name, total, major, minor, read, write, discard]. To display everything, use --everything.
    ///
    /// ********************** Example Commands **********************
    ///
    /// Simple example:
    ///
    /// $ below dump disk -b "08:30:00" -e "08:30:30" -f read write discard -O csv
    ///
    /// Output stats for all "nvme0*" matched disk from 08:30:00 to 08:30:30:
    ///
    /// $ below dump process -b "08:30:00" -e "08:30:30" -s read -F nvme0* -O json
    ///
    /// Output stats for top 5 read partitions for each time slice from 08:30:00 to 08:30:30:
    ///
    /// $ below dump process -b "08:30:00" -e "08:30:30" -s read_bytes --rsort --top 5
    Disk {
        /// Select which fields to display and in what order.
        #[structopt(short, long)]
        fields: Option<Vec<DiskField>>,
        #[structopt(flatten)]
        opts: GeneralOpt,
        /// Select field for operation, use with --sort, --rsort, --filter, --top
        #[structopt(long, short)]
        select: Option<DiskField>,
    },
    /// Dump process stats
    ///
    /// ********************** Available fields **********************
    ///
    /// timestamp, datetime, pid, ppid, comm, state, uptime, cgroup, cmdline
    ///
    /// cpu_user, cpu_sys, cpu_threads, cpu_total
    ///
    /// mem_rss, mem_minorfaults, mem_majorfaults
    ///
    /// io_read, io_write, io_total
    ///
    /// ********************** Aggregated fields **********************
    ///
    /// * cpu: includes [cpu_total]. Additionally includes [cpu_user, cpu_sys, cpu_threads] if --detail specified
    ///
    /// * mem: includes [mem_rss]. Additionally includes [mem_minorfaults, mem_majorfaults] if --detail specified
    ///
    /// * io: includes [io_read, io_write]. Additionally includes[io_total] -if --detail specified
    ///
    /// --default will have all of [pid, comm, cpu, mem, io]. To display everything, use --everything.
    ///
    /// ********************** Example Commands **********************
    ///
    /// Simple example:
    ///
    /// $ below dump process -b "08:30:00" -e "08:30:30" -f comm cpu io_total -O csv
    ///
    /// Output stats for all "below*" matched processes from 08:30:00 to 08:30:30:
    ///
    /// $ below dump process -b "08:30:00" -e "08:30:30" -s comm -F below* -O json
    ///
    /// Output stats for top 5 CPU intense processes for each time slice from 08:30:00 to 08:30:30:
    ///
    /// $ below dump process -b "08:30:00" -e "08:30:30" -s cpu_total --rsort --top 5
    Process {
        /// Select which fields to display and in what order.
        #[structopt(short, long)]
        fields: Option<Vec<ProcField>>,
        #[structopt(flatten)]
        opts: GeneralOpt,
        /// Select field for operation, use with --sort, --rsort, --filter, --top
        #[structopt(long, short)]
        select: Option<ProcField>,
    },
    /// Dump cgroup stats
    ///
    /// ********************** Available fields **********************
    ///
    /// timestamp, datetime, name, full_path
    ///
    /// cpu_usage, cpu_user, cpu_system, cpu_nr_periods, cpu_nr_throttled
    ///
    /// mem_total, mem_anon, mem_file, mem_kernel, mem_slab, mem_sock, mem_shem,
    /// mem_file_mapped, mem_file_dirty, mem_file_writeback, mem_anon_thp, mem_inactive_anon,
    /// mem_active_anon, mem_inactive_file, mem_active_file, mem_unevictable, mem_slab_reclaimable,
    /// mem_slab_unreclaimable, mem_high
    ///
    /// io_read, io_write, io_wios, io_rios, io_dbps, io_diops, io_total
    ///
    /// pressure_cpu_some, pressure_io_some, pressure_io_full, pressure_mem_some, pressure_mem_full
    ///
    /// ********************** Aggregated fields **********************
    ///
    /// * cpu: includes [cpu_usage]. Additionally includes [cpu_*] if --detail specified.
    ///
    /// * mem: includes [mem_total]. Additionally includes [mem_*] if --detail specified.
    ///
    /// * io: incldues [io_read, io_write]. Additionally includes [io_*] if --detail specified.
    ///
    /// * pressure: includes [pressure_cpu_some, pressure_mem_full, pressure_io_full],
    /// Additionally includes [pressure_*] if --detail specified
    ///
    /// --default will have all of [name, cpu, mem, io, pressure]. To display everything, use --everything.
    ///
    /// ********************** Example Commands **********************
    ///
    /// Simple example:
    ///
    /// $ below dump cgroup -b "08:30:00" -e "08:30:30" -f name cpu -O csv
    ///
    /// Output stats for all cgroups matching pattern "below*" for time slices
    /// from 08:30:00 to 08:30:30:
    ///
    /// $ below dump cgroup -b "08:30:00" -e "08:30:30" -s name -F below* -O json
    ///
    /// Output stats for top 5 CPU intense cgroups for each time slice
    /// from 08:30:00 to 08:30:30 recursively:
    ///
    /// $ below dump cgroup -b "08:30:00" -e "08:30:30" -s cpu_usage --rsort --top 5
    Cgroup {
        /// Select which fields to display and in what order.
        #[structopt(short, long)]
        fields: Option<Vec<CgroupField>>,
        #[structopt(flatten)]
        opts: GeneralOpt,
        /// Select field for operation, use with --sort, --rsort, --filter, --top
        #[structopt(long, short)]
        select: Option<CgroupField>,
    },
    /// Dump the link layer iface stats
    ///
    /// ********************** Available fields **********************
    ///
    /// timestamp, datetime, interface
    ///
    /// rx_bytes_per_sec, tx_bytes_per_sec, throughput_per_sec, rx_packets_per_sec, tx_packets_per_sec,
    /// collisions, multicast
    ///
    /// rx_bytes, rx_compressed, rx_crc_errors, rx_dropped, rx_errors, rx_fifo_errors, rx_frame_errors,
    /// rx_length_errors, rx_missed_errors, rx_nohandler, rx_over_errors, rx_packets
    ///
    /// tx_bytes, tx_aborted_errors, tx_carrier_errors, tx_compressed, tx_dropped, tx_errors,
    /// tx_fifo_errors, tx_heatbeat_errors, tx_packets, tx_window_errors
    ///
    /// ********************** Aggregated fields **********************
    ///
    /// * rate: includes [*_bytes_per_sec, throughput_per_sec]. Additionally includes [*_packets_per_sec] if --detail specified.
    ///
    /// * rx: includes [rx_bytes, rx_dropped, rx_errors]. Additionally includes [rx_*] if --detail specified.
    ///
    /// * tx: incldues [tx_bytes, tx_dropped, tx_errors]. Additionally includes [tx_*] if --detail specified.
    ///
    /// --default will have all of [interface, rate, rx, tx]. To display everything, use --everything.
    ///
    /// ********************** Example Commands **********************
    ///
    /// Simple example:
    ///
    /// $ below dump iface -b "08:30:00" -e "08:30:30" -f interface rate -O csv
    ///
    /// Output stats for all iface stats matching pattern "eth*" for time slices
    /// from 08:30:00 to 08:30:30:
    ///
    /// $ below dump iface -b "08:30:00" -e "08:30:30" -s interface -F eth* -O json
    Iface {
        /// Select which fields to display and in what order.
        #[structopt(short, long)]
        fields: Option<Vec<IfaceField>>,
        #[structopt(flatten)]
        opts: GeneralOpt,
        /// Select field for operation, use with --filter
        #[structopt(long, short)]
        select: Option<IfaceField>,
    },
    /// Dump the network layer stats including ip and icmp
    ///
    /// ********************** Available fields **********************
    ///
    /// timestamp, datetime
    ///
    /// ip_forwarding, ip_in_receives, ip_forw_datagrams, ip_in_discards, ip_in_delivers, ip_out_requests,
    /// ip_out_discards, ip_out_no_routes, ip_in_mcast, ip_out_mcast, ip_in_bcast, ip_out_bcast
    ///
    /// ip6_in_receives, ip6_forw_datagrams, ip6_in_discards, ip6_in_delivers, ip6_out_requests, ip6_in_no_routes,
    /// ip6_out_no_routes, ip6_in_hdr_err, ip6_in_addr_err, ip6_in_mcast, ip6_out_mcast, ip6_in_bcast, ip6_in_bcast
    ///
    /// icmp_in_msgs, icmp_in_errs, icmp_in_dest_unreachs, icmp_out_msg, icmp_out_errs, icmp_out_dest_unreachs
    ///
    /// icmp6_in_msgs, icmp6_in_errs, icmp6_in_dest_unreachs, icmp6_out_msg, icmp6_out_errs, icmp6_out_dest_unreachs
    ///
    /// ********************** Aggregated fields **********************
    ///
    /// * ip: includes [ip_*].
    ///
    /// * ip6: includes [ip6_*].
    ///
    /// * icmp: includes [icmp_*].
    ///
    /// * icmp6: includes [icmp6_*].
    ///
    /// --default will have all of [ip, ip6, icmp, icmp6].
    ///
    /// ********************** Example Commands **********************
    ///
    /// Example:
    ///
    /// $ below dump network -b "08:30:00" -e "08:30:30" -f ip ip6 -O json
    ///
    Network {
        /// Select which fields to display and in what order.
        #[structopt(short, long)]
        fields: Option<Vec<NetworkField>>,
        #[structopt(flatten)]
        opts: GeneralOpt,
        /// Select field for operation, use with --filter
        #[structopt(long, short)]
        select: Option<NetworkField>,
    },
    /// Dump the transport layer stats including tcp and udp
    ///
    /// ********************** Available fields **********************
    ///
    /// timestamp, datetime
    ///
    /// tcp_active_opens, tcp_passive_opens, tcp_attempt_fails, tcp_estab_reset, tcp_curr_estab, tcp_in_segs,
    /// tcp_out_segs, tcp_retrans_segs_per_sec, tcp_retrans_segs, tcp_in_errs, tcp_out_rsts, tcp_in_csum_errs
    ///
    /// udp_in_datagrams, udp_no_ports, udp_in_errs, udp_out_datagrams, udp_recv_buf_errs, udp_snd_buf_errs, udp_ignored_multi
    ///
    /// udp6_in_datagrams, udp6_no_ports, udp6_in_errs, udp6_out_datagrams, udp6_recv_buf_errs, udp6_snd_buf_errs
    /// udp6_ignored_multi
    ///
    /// ********************** Aggregated fields **********************
    ///
    /// * tcp: includes [tcp_*].
    ///
    /// * udp: includes [udp_*].
    ///
    /// * udp6: includes [udo6_*].
    ///
    /// --default will have all of [tcp, udp, udp6].
    ///
    /// ********************** Example Commands **********************
    ///
    /// Example:
    ///
    /// $ below dump transport -b "08:30:00" -e "08:30:30" -f tcp udp -O json
    ///
    Transport {
        /// Select which fields to display and in what order.
        #[structopt(short, long)]
        fields: Option<Vec<TransportField>>,
        #[structopt(flatten)]
        opts: GeneralOpt,
        /// Select field for operation, use with --filter
        #[structopt(long, short)]
        select: Option<TransportField>,
    },
}
