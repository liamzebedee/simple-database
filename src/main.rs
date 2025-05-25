

use std::fs::File;
use std::path::Path;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;

// Page number 0 is invalid and used as a sentinel (e.g. end of overflow chain, null pointer).
const PAGE_SIZE: usize = 8192;

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum PageType {
    DATA = 0,
    INDEX = 1,
    OVERFLOW = 2,
}

struct Database {
    file: File,
}

impl Database {
    fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        if path.try_into<String>() == ":memory:" {

        }
        let mut file = File::open(path)?;
        let size = file.metadata()?.len();
        Ok(Self { file })
    }

    fn num_pages(&self) -> std::io::Result<u32> {
        let size = self.file.metadata()?.len();
        Ok((size / PAGE_SIZE as u64) as u32)
    }

    fn get_page(&mut self, id: u32) -> Option<Page> {
        if id == 0 || id > self.num_pages().unwrap() {
            return None;
        }

        let offset = (id as u64 - 1) * PAGE_SIZE as u64;
        let mut buf: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
        self.file.seek(SeekFrom::Start(offset)).ok()?;
        self.file.read_exact(&mut buf).ok()?;
        
        // let page_type = PageType::from(buf[0]);
        // match page_type {
        // }
        None
    }
}

enum Page {
    DataPage,
    IndexPage,
    OverflowPage
}

// For typical small rows (~100–200B), SQLite might fit 30–50 rows per 8KB page. Fewer if rows are larger or include blobs.
// Each page stores either data rows or B-tree nodes.
struct PageHeader {
    page_type: PageType,
    id: u32,

    // 3–4: Start of free space
    // Offset to the beginning of unallocated space in the page.
    // New cell pointer entries (2 bytes each) are written here.

    // Offset to where the cell content area begins (i.e. the highest-used byte for row data).
    // Grows backward as more rows are inserted.
    // New row data is placed before this point.
}

struct DataPage {
    num_rows: u8,
    row_offset_array_end: u16,   // End of the row offset array (grows forward)
    row_data_start: u16,         // Start of row data area (grows backward)
    fragmented_free_bytes: u16,  // Total fragmented space from deletes
    
    row_offsets: Vec<u16>,             // Pointers to start of each row in the data area
    data: [u8; PAGE_SIZE],             // Full raw page buffer (used to parse rows)
}

struct IndexPage {

}
struct OverflowPage {
    // The next overflow page ID.
    // If this is the end of the chain, the sentinal ("0") value is used.
    next: u32,
}


// Each cell = a row:
// varint for payload size
// varint for rowid
// Record header (with column types)
// Column values
struct Row {

}

fn main() {
    

    // 1. Open DB.
        // Load root page and unfurl overflow until full meta view.
        // cache stored positions of btree pages.
    // 2. Save.
    // 3. Close DB.

    // 1. Open DB.
    // 2. Write a row (add a schema definition).
    // 3. Close DB.

    // 1. Open DB.
    // 2. Create table.
    // 3. Create table index.
    // 4. Insert 3 rows. 
    // 5. Close DB.


    // Debug DB.
    // Unit test:
    // - pages in memory.
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_open_db() {
        let db = Database::open("test.db");
    }
}


// Database:
// Create file
// PageStore
    // stores pages in cache
    // marks pages as dirty when modified
    // keeps "dirty" set which it can flush to disk
    // assumes no-one else is using disk


// So the essence of this is:
// - split things into fixed-size chunks (blocks)
// - define chunks such that they have different types
// - an overflow chunk is an example of a type
// - then when you want to query a table, you get the btree page for that table, and then do a log(n) traversal down it to find the right records? if you want to accumulate results, I guess you do a breadth-first search
// - right.
// - when you want to write a record, it gets the current page, appends the row to the end of the page if there's space for the row. if the row's length in bytes exceeds the size of the page, then we create a new page and link it to the previous one.
// - then we also update any indices - there are btree pages for each index. so when we update a btree, what we are doing. well it's essentially a binary tree. and recall a binary tree is basically
// - a root node and then intermediate nodes where we always have 2 children. a b tree is just n children where n is a block size matched to good underlying block size (ie. of disk). 
// - so the intermediate nodes store pointers to more intermediate nodes or leaf nodes.
// - b+ trees have all their leaf nodes linked together so you can do rapid linear scan of the nodes (linked list traversal)
// - right so say we're inserting a new node. where does it go in the btree? are we doing rebalancings? yeah we do a rebalance every so often to amortize.
// okay so next up.
// we have this database schema. it defines tables, columns, each column has a type. and then each row has a length (so we can traverse quickly) and each field has its own length too if it's not fixed size by default.
// we insert these rows, which creates pages usually or modifies existing pages. and it also updates the btree with the pointer to the row (what's the row pointer? is it the id? it could be page+cell index. that'd make sense.)
// ok nah it's actually, every table must have primary key in sqlite. the btrees which aren't for the primary key column. -they use the primary key as the row pointer
// yes this starts to make a lot of sense.
// then okay so we insert these rows. and it updates the main btree. we generate a primary key for the row which is its row id. then we insert the row data into the main table btree leaf page. there are no data pages. everything is just btrees.
// btrees and pages morty, btrees and pages.
// okay next up then.
// what happens when you update rows? just update the btree page.
// what happens when you need to reindex? the btrees are recomputed and updated in place.
// what happens when you just need to traverse all rows? we basically find the root table btree page. and then we traverse down to the leaftmost btree leaf. and then we do the linear scan through that (as each btree leaf page links to the next btree leaf page).
// what happens when we are doing a query that's basically doing some joins and stuff? i guess that's the table engine at play. I'm not really sure how that works.
// it basically:
// - queries the inner table
// - constructs a lightweight index
// - does an inmemory thing to store that or whatever
// - then queries another one
// - constructs the table layout
// - some sort of table-column virtual datatype which allows you to merge schemas, rename rows, etc.
// that makes sense
// so what's the general gist of database systems? 
// read and write
// split things up into fixed-size pages.
// start with the root page
// to efficiently jump around, we use the page id to page offset calculation.
// this is basically addressing in a disk context.
// we store data by default in a btree sorted by primary key
// when we insert a row, we find the right page to insert into. importantly, the row can be a dynamic size. if it exceeds the page capacity, we create a new page. if we exceed the btree leaf node capacity, we partition the leaf's internal nodes based on the median key into 2 btree leaf nodes, push the median key to the parent and link it to these children, and potentially recurse that.
// if the row is too big for the page, it's stored in an overflow page. 
// okay, so that's a write.
// what about a read? 
// read usually occurs in the form of queries.
// so when we query a row, how does that work? 
// say the query is "give us teh row wiht username liam"
// we want to binary search based on the username column
// so we identify the username btree (or otherwise, do a linear scan of the base table btree)
// let's assume it's a b+tree, where the nodes are literally all stored in leaves.
// then we traverse left/right until we are at a leaf node based on the value (liam)
// and then we again do that on the inner nodes till we find a row or nothing
// i guess there's gotta be a special left/right byte-by-byte string comparison going on as well? that's interesting
// anyways we get to the leaf, perform the comparison
// if it's right, we then load the row from the cell
// which entails returning the tuple of values which represent each column for that row
// and that's how the database query works
