/*
TODO:
- add GPTPartitionSpec:
  - partition_id
  - size
  - type
  - attributes
  - starting offset (if 0, use the first available free area like how `fdisk` would)

- add create.rs: create_partition(disk, partition_spec)
- add delete.rs: delete_partition(disk, partition_id)
- add other stuff we may want, e.g: setting partition attributes, getting partition data, etc, etc
*/