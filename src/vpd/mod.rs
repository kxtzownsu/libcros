/*
pub fn set_vpd_value(partition, key, value) -> ();
pub fn get_vpd_value(partition, key) -> String;
pub fn del_vpd_key(partition, key) -> ();
pub fn erase_vpd_partition(partition) -> ();
pub fn get_vpd_keys(partition) -> Vec<String>;

partition will only ever be "RO_VPD" or "RW_VPD"
we need to use libflashrom.
*/

pub mod flashrom;