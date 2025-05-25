simple-database
===============

I always wanted to understand how you implement a data structure that gets scaled onto disk.

ie. when your RAM runs out, you need to put your data on disk. How do you do that? It sounds dumb and that's what a database is.

This project is a small exploration of how to implement a database like SQLite. In terms of concepts I learnt:

 - pages
 - btree nodes
 - table btrees + index btrees
 - row id's
 - page offsets
 - overflow pages
 - b+trees
 - b+tree rebalancing
 - database page compaction when rows are deleted / defragmentation