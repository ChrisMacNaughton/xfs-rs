#[macro_use] extern crate nom;

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
      },
      _ => unreachable!(),
    }
  }

  #[test]
  fn it_parses_allocation_btree() {
    let example_output = b"abt 29491162 337391304 11257328 11133039";
    match super::abt(example_output) {
      nom::IResult::Done(_, result) => {
        assert_eq!(result.inserts, 11257328);
      },
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
  }
}

pub struct XfsStat {
  extent_allocation: ExtentAllocation,
  allocation_btree: AllocationBTree
}

pub struct ExtentAllocation {
  pub allocated_extents: u32,
  pub allocated_blocks: u32,
  pub freed_extents: u32,
  pub freed_blocks: u32,
}

pub struct AllocationBTree {
  pub lookups: u32,
  pub compares: u32,
  pub inserts: u32,
  pub deletes: u32,
}

pub fn parse(input: &[u8]) -> Option<XfsStat> {
  match xfs_stat(input) {
    nom::IResult::Done(_, stat) =>{
      Some(stat)
    },
    _ => None
  }
}

named!(xfs_stat <XfsStat>,
  chain!(
    extent_alloc: extent_alloc ~
    newline ~
    abt: abt,
    || {
      XfsStat {
        extent_allocation: extent_alloc,
        allocation_btree: abt,
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