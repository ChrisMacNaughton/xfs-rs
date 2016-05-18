#[macro_use]
extern crate nom;

use std::str::FromStr;

use self::nom::{is_digit, space, newline};

#[cfg(test)]
mod tests {
    use nom;

    #[test]
    fn it_parses_a_u32() {
        let input = b"12345";
        let result = super::take_u32(input);
        match result {
            nom::IResult::Done(_, f) => assert_eq!(f, 12345u32),
            _ => unreachable!(),
        }

    }

    #[test]
    fn it_parses_extent_allocation() {
        let example_output = b"extent_alloc 4260849 125170297 4618726 131131897";
        match super::extent_alloc(example_output) {
            nom::IResult::Done(_, result) => {
                assert_eq!(result.allocated_extents, 4260849);
                assert_eq!(result.freed_blocks, 131131897);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_parses_allocation_btree() {
        let example_output = b"abt 29491162 337391304 11257328 11133039";
        match super::abt(example_output) {
            nom::IResult::Done(_, result) => {
                assert_eq!(result.inserts, 11257328);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_parses_block_mapping() {
        let example_output = b"blk_map 381213360 115456141 10903633 69612322 7448401 507596777 0";
        match super::blk_map(example_output) {
            nom::IResult::Done(_, result) => {
                assert_eq!(result.list_delete, 7448401);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_parses_block_map_btree() {
        let example_output = b"bmbt 771328 6236258 602114 86646";
        match super::bmbt(example_output) {
            nom::IResult::Done(_, result) => {
                assert_eq!(result.deletes, 86646);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_parses_directory_operations() {
        let example_output = b"dir 21253907 6921870 6969079 779205554";
        match super::dir(example_output) {
            nom::IResult::Done(_, result) => {
                assert_eq!(result.lookups, 21253907);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_parses_transactions() {
        let example_output = b"trans 126946406 38184616 6342392";
        match super::trans(example_output) {
            nom::IResult::Done(_, result) => {
                assert_eq!(result.waited, 126946406);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_parses_inode_operations() {
        let example_output = b"ig 17754368 2019571 102 15734797 0 15672217 3962470";
        match super::ig(example_output) {
            nom::IResult::Done(_, result) => {
                assert_eq!(result.cache_lookups, 17754368);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_parseslog_operations() {
        let example_output = b"log 129491915 3992515264 458018 153771989 127040250";
        match super::log(example_output) {
            nom::IResult::Done(_, result) => {
                assert_eq!(result.log_writes, 129491915);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_parses_example() {
        let example_output = b"extent_alloc 4260849 125170297 4618726 131131897
abt 29491162 337391304 11257328 11133039
blk_map 381213360 115456141 10903633 69612322 7448401 507596777 0
bmbt 771328 6236258 602114 86646
dir 21253907 6921870 6969079 779205554
trans 126946406 38184616 6342392
ig 17754368 2019571 102 15734797 0 15672217 3962470
log 129491915 3992515264 458018 153771989 127040250
push_ail 171473415 0 6896837 3324292 8069877 65884 1289485 0 22535 7337
xstrat 4140059 0
rw 1595677950 1046884251
attr 194724197 0 7 0
icluster 20772185 2488203 13909520
vnodes 62578 15959666 0 0 15897088 15897088 15897088 0
buf 2090581631 1972536890 118044776 225145 9486625 0 0 2000152616 809762
xpc 6908312903680 67735504884757 19760115252482
debug 0";
        let result = super::parse(example_output).unwrap();

        assert_eq!(result.extent_allocation.freed_extents, 4618726);
        assert_eq!(result.allocation_btree.lookups, 29491162);
        assert_eq!(result.block_mapping.unmap, 10903633);
        assert_eq!(result.block_map_btree.inserts, 602114);
    }
}

pub struct XfsStat {
    pub extent_allocation: ExtentAllocation,
    pub allocation_btree: AllocationBTree,
    pub block_mapping: BlockMapping,
    pub block_map_btree: BlockMapBTree,
    pub directory_operations: DirectoryOperations,
    pub transactions: Transactions,
    pub inode_operations: InodeOperations,
    pub log_operations: LogOperations,
    pub tail_pushing_stats: TailPushingStats,
}

pub struct ExtentAllocation {
    /// Number of file system extents allocated over all XFS filesystems.
    pub allocated_extents: u32,
    /// Number of file system blocks allocated over all XFS filesystems.
    pub allocated_blocks: u32,
    /// Number of file system extents freed over all XFS filesystems.
    pub freed_extents: u32,
    /// Number of file system blocks freed over all XFS filesystems.
    pub freed_blocks: u32,
}

pub struct AllocationBTree {
    /// Number of lookup operations in XFS filesystem allocation btrees.
    pub lookups: u32,
    /// Number of compares in XFS filesystem allocation btree lookups.
    pub compares: u32,
    /// Number of extent records inserted into XFS filesystem allocation btrees.
    pub inserts: u32,
    /// Number of extent records deleted from XFS filesystem allocation btrees.
    pub deletes: u32,
}

pub struct BlockMapping {
    /// Number of block map for read operations performed on XFS files.
    pub map_read: u32,
    /// Number of block map for write operations performed on XFS files.
    pub map_write: u32,
    /// Number of block unmap (delete) operations performed on XFS files.
    pub unmap: u32,
    /// Number of extent list insertion operations for XFS files.
    pub list_insert: u32,
    /// Number of extent list deletion operations for XFS files.
    pub list_delete: u32,
    /// Number of extent list lookup operations for XFS files.
    pub list_lookup: u32,
    /// Number of extent list comparisons in XFS extent list lookups.
    pub list_compare: u32,
}

pub struct BlockMapBTree {
    /// Number of block map btree lookup operations on XFS files.
    pub lookups: u32,
    /// Number of block map btree compare operations in XFS block map lookups.
    pub compares: u32,
    /// Number of block map btree records inserted for XFS files.
    pub inserts: u32,
    /// Number of block map btree records deleted for XFS files.
    pub deletes: u32,
}

pub struct DirectoryOperations {
    /// This is a count of the number of file name directory lookups in XFS
    /// filesystems. It counts only those lookups which miss in the operating
    /// system's directory name lookup cache and must search the real directory
    /// structure for the name in question. The count is incremented once for each
    /// level of a pathname search that results in a directory lookup.
    pub lookups: u32,
    /// This is the number of times a new directory entry was created in XFS filesystems. Each time that a new file, directory, link, symbolic link, or special file is created in the directory hierarchy the count is incremented.
    pub creates: u32,
    /// This is the number of times an existing directory entry was removed in XFS filesystems. Each time that a file, directory, link, symbolic link, or special file is removed from the directory hierarchy the count is incremented.
    pub removes: u32,
    /// This is the number of times the XFS directory getdents operation was performed. The getdents operation is used by programs to read the contents of directories in a file system independent fashion. This count corresponds exactly to the number of times the getdents(2) system call was successfully used on an XFS directory.
    pub get_dents: u32,
}

pub struct Transactions {
    /// This is the number of meta-data transactions which waited to be committed to the on-disk log before allowing the process performing the transaction to continue. These transactions are slower and more expensive than asynchronous transactions, because they force the in memory log buffers to be forced to disk more often and they wait for the completion of the log buffer writes. Synchronous transactions include file truncations and all directory updates when the file system is mounted with the 'wsync' option.
    pub waited: u32,
    /// This is the number of meta-data transactions which did not wait to be committed to the on-disk log before allowing the process performing the transaction to continue. These transactions are faster and more efficient than synchronous transactions, because they commit their data to the in memory log buffers without forcing those buffers to be written to disk. This allows multiple asynchronous transactions to be committed to disk in a single log buffer write. Most transactions used in XFS file systems are asynchronous.
    pub async: u32,
    /// This is the number of meta-data transactions which did not actually change anything. These are transactions which were started for some purpose, but in the end it turned out that no change was necessary.
    pub empty: u32,
}

pub struct InodeOperations {
    /// This is the number of times the operating system looked for an XFS inode in the inode cache. Whether the inode was found in the cache or needed to be read in from the disk is not indicated here, but this can be computed from the ig_found and ig_missed counts.
    pub cache_lookups: u32,
    /// This is the number of times the operating system looked for an XFS inode in the inode cache and found it. The closer this count is to the ig_attempts count the better the inode cache is performing.
    pub cache_hits: u32,
    /// This is the number of times the operating system looked for an XFS inode in the inode cache and saw that it was there but was unable to use the in memory inode because it was being recycled by another process.
    pub cache_recycle: u32,
    /// This is the number of times the operating system looked for an XFS inode in the inode cache and the inode was not there. The further this count is from the ig_attempts count the better.
    pub cache_missed: u32,
    /// This is the number of times the operating system looked for an XFS inode in the inode cache and found that it was not there but upon attempting to add the inode to the cache found that another process had already inserted it.
    pub cache_dup: u32,
    /// This is the number of times the operating system recycled an XFS inode from the inode cache in order to use the memory for that inode for another purpose. Inodes are recycled in order to keep the inode cache from growing without bound. If the reclaim rate is high it may be beneficial to raise the vnode_free_ratio kernel tunable variable to increase the size of the inode cache.
    pub cache_reclaime: u32,
    /// This is the number of times the operating system explicitly changed the attributes of an XFS inode. For example, this could be to change the inode's owner, the inode's size, or the inode's timestamps.
    pub inode_attr_changes: u32,
}

pub struct LogOperations {
    /// This variable counts the number of log buffer writes going to the physical log partitions of all XFS filesystems. Log data traffic is proportional to the level of meta-data updating. Log buffer writes get generated when they fill up or external syncs occur.
    pub log_writes: u32,
    /// This variable counts (in 512-byte units) the information being written to the physical log partitions of all XFS filesystems. Log data traffic is proportional to the level of meta-data updating. The rate with which log data gets written depends on the size of internal log buffers and disk write speed. Therefore, filesystems with very high meta-data updating may need to stripe the log partition or put the log partition on a separate drive.
    pub log_blocks: u32,
    /// This variable keeps track of times when a logged transaction can not get any log buffer space. When this occurs, all of the internal log buffers are busy flushing their data to the physical on-disk log.
    pub noiclogs: u32,
    /// The number of times the in-core log is forced to disk. It is equivalent to the number of successful calls to the function xfs_log_force().
    pub log_forced: u32,
    /// Value exported from the xs_log_force_sleep field of struct xfsstats.
    pub force_sleep: u32,
}

pub struct TailPushingStats {
    /// Value from the xs_try_logspace field of struct xfsstats.
    pub logspace: u32,
    /// Value from the xs_sleep_logspace field of struct xfsstats.
    pub sleep_logspace: u32,
    /// The number of times the tail of the AIL is moved forward. It is equivalent to the number of successful calls to the function xfs_trans_push_ail().
    pub push_ails: u32,
    /// Value from xs_push_ail_success field of struct xfsstats.
    pub push_ail_success: u32,
    /// Value from xs_push_ail_pushbuf field of struct xfsstats.
    pub push_ail_pushbuf: u32,
    /// Value from xs_push_ail_pinned field of struct xfsstats.
    pub push_ail_pinned: u32,
    /// Value from xs_push_ail_locked field of struct xfsstats.
    pub push_ail_locked: u32,
    /// Value from xs_push_ail_flushing field of struct xfsstats.
    pub push_ail_flushing: u32,
    /// Value from xs_push_ail_restarts field of struct xfsstats.
    pub push_ail_restarts: u32,
    /// Value from xs_push_ail_flush field of struct xfsstats.
    pub push_ail_flush: u32,
}

pub fn parse(input: &[u8]) -> Option<XfsStat> {
    match xfs_stat(input) {
        nom::IResult::Done(_, stat) => Some(stat),
        _ => None,
    }
}

named!(xfs_stat <XfsStat>,
  chain!(
    extent_alloc: extent_alloc ~
    newline ~
    abt: abt ~
    newline ~
    blk_map: blk_map ~
    newline ~
    block_map_btree: bmbt ~
    newline ~
    directory_operations: dir ~
    newline ~
    transactions: trans ~
    newline ~
    inode_operations: ig ~
    newline ~
    log_operations: log ~
    newline ~
    tail_pushing_stats: push_ail,
    || {
      XfsStat {
        extent_allocation: extent_alloc,
        allocation_btree: abt,
        block_mapping: blk_map,
        block_map_btree: block_map_btree,
        directory_operations: directory_operations,
        transactions: transactions,
        inode_operations: inode_operations,
        log_operations:log_operations,
        tail_pushing_stats: tail_pushing_stats,
      }
    }
  )
);

named!(take_u32 <u32>,
  chain!(
    uint_slice: take_while!(is_digit),
    || {
      let int_str = String::from_utf8_lossy(uint_slice);
      u32::from_str(&int_str[..]).unwrap()
    }
  )
);

named!(extent_alloc <ExtentAllocation>,
  chain!(
    tag!("extent_alloc") ~
    space ~
    allocx: take_u32 ~
    space ~
    allocb: take_u32 ~
    space ~
    freex: take_u32 ~
    space ~
    freeb: take_u32,
    || {
      ExtentAllocation {
        allocated_extents: allocx,
        allocated_blocks: allocb,
        freed_extents: freex,
        freed_blocks: freeb,
      }
    }
  )
);

named!(abt <AllocationBTree>,
  chain!(
    tag!("abt") ~
    space ~
    lookups: take_u32 ~
    space ~
    compares: take_u32 ~
    space ~
    inserts: take_u32 ~
    space ~
    deletes: take_u32,
    || {
      AllocationBTree {
        lookups: lookups,
        compares: compares,
        inserts: inserts,
        deletes: deletes,
      }
    }
  )
);

named!(blk_map <BlockMapping>,
  chain!(
    tag!("blk_map") ~
    space ~
    map_read: take_u32 ~
    space ~
    map_write: take_u32 ~
    space ~
    unmap: take_u32 ~
    space ~
    list_insert: take_u32 ~
    space ~
    list_delete: take_u32 ~
    space ~
    list_lookup: take_u32 ~
    space ~
    list_compare: take_u32,
    ||{
      BlockMapping {
        map_read: map_read,
        map_write: map_write,
        unmap: unmap,
        list_insert: list_insert,
        list_delete: list_delete,
        list_lookup: list_lookup,
        list_compare: list_compare,
      }
    }
  )
);

named!(bmbt <BlockMapBTree>,
  chain!(
    tag!("bmbt") ~
    space ~
    lookup: take_u32 ~
    space ~
    compare: take_u32 ~
    space ~
    insrec: take_u32 ~
    space ~
    delrec: take_u32,
    || {
      BlockMapBTree {
        lookups: lookup,
        compares: compare,
        inserts: insrec,
        deletes: delrec,
      }
    }
  )
);

named!(dir <DirectoryOperations>,
    chain!(
        tag!("dir") ~
        space ~
        lookups: take_u32 ~
        space ~
        creates: take_u32 ~
        space ~
        removes: take_u32 ~
        space ~
        get_dents: take_u32,
        || {
            DirectoryOperations {
                lookups: lookups,
                creates: creates,
                removes: removes,
                get_dents: get_dents,
            }
        }
    )
);

named!(trans <Transactions>,
  chain!(
    tag!("trans") ~
    space ~
    waited: take_u32 ~
    space ~
    async: take_u32 ~
    space ~
    empty: take_u32,
    ||{
      Transactions {
        waited: waited,
        async: async,
        empty: empty,
      }
    }
  )
);

named!(ig <InodeOperations>,
  chain!(
    tag!("ig") ~
    space ~
    cache_lookups: take_u32 ~
    space ~
    cache_hits: take_u32 ~
    space ~
    cache_recycle: take_u32 ~
    space ~
    cache_missed: take_u32 ~
    space ~
    cache_dup: take_u32 ~
    space ~
    cache_reclaime: take_u32 ~
    space ~
    inode_attr_changes: take_u32,
    || {
      InodeOperations {
        cache_lookups: cache_lookups,
        cache_hits: cache_hits,
        cache_recycle: cache_recycle,
        cache_missed: cache_missed,
        cache_dup: cache_dup,
        cache_reclaime: cache_reclaime,
        inode_attr_changes: inode_attr_changes,
      }
    }
  )
);

named!(log <LogOperations>,
  chain!(
    tag!("log") ~
    space ~
    log_writes: take_u32 ~
    space ~
    log_blocks: take_u32 ~
    space ~
    noiclogs: take_u32 ~
    space ~
    log_forced: take_u32 ~
    space ~
    force_sleep: take_u32,
    || {
      LogOperations {
          log_writes: log_writes,
          log_blocks: log_blocks,
          noiclogs: noiclogs,
          log_forced: log_forced,
          force_sleep: force_sleep,
      }
    }
  )
);

named!(push_ail <TailPushingStats>,
    chain!(
        tag!("push_ail") ~
        space ~
        logspace: take_u32 ~
        space ~
        sleep_logspace: take_u32 ~
        space ~
        push_ails: take_u32 ~
        space ~
        push_ail_success: take_u32 ~
        space ~
        push_ail_pushbuf: take_u32 ~
        space ~
        push_ail_pinned: take_u32 ~
        space ~
        push_ail_locked: take_u32 ~
        space ~
        push_ail_flushing: take_u32 ~
        space ~
        push_ail_restarts: take_u32 ~
        space ~
        push_ail_flush: take_u32,
        || {
            TailPushingStats {
                logspace: logspace,
                sleep_logspace: sleep_logspace,
                push_ails: push_ails,
                push_ail_success: push_ail_success,
                push_ail_pushbuf: push_ail_pushbuf,
                push_ail_pinned: push_ail_pinned,
                push_ail_locked: push_ail_locked,
                push_ail_flushing: push_ail_flushing,
                push_ail_restarts: push_ail_restarts,
                push_ail_flush: push_ail_flush,
            }
        }
    )
);
