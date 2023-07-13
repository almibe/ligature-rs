# ligature-redb

ligature-redb is a library that allows storing Ligature's data model in an embedded instance of redb.
Below are some notes about how this library works.
Most of it is based on work I did on the JVM using a similar embedded key-value store called xodus.

## Creating an instance

| Function                   | Location                          |    
| -------------------------- | --------------------------------- |
| LigatureRedb::default()    | HOME/.ligature/redb/ligature.redb |
| LigatureRedb::create(path) | The path given                    |
| LigatureRedb::temp()       | HOME/.ligature/redb/{UUID}.redb   |

