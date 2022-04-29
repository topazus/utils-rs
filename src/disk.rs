use std::io::Read;

use regex::Regex;
/// what we have most looking like a physical device
#[derive(Debug, Clone)]
pub struct Disk {
    /// a name, like "sda", "sdc", "nvme0n1", etc.
    pub name: String,

    /// true for HDD, false for SSD, None for unknown.
    /// This information isn't reliable for USB devices
    pub rotational: Option<bool>,

    /// whether the system thinks the media is removable.
    /// Seems reliable when not mapped
    pub removable: Option<bool>,

    /// whether it's a RAM disk
    pub ram: bool,

    /// whether it's on LVM
    pub lvm: bool,

    /// whether it's a crypted disk
    pub crypted: bool,
}
pub fn read_file_as_bool(path: &str) -> Option<bool> {
    // ok(), Converts from Result<T, E> to Option<T>.
    let mut file = std::fs::File::open(path).ok()?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).ok()?;
    match buf.trim() {
        "0" => Some(false),
        "1" => Some(true),
        _ => None,
    }
}
impl Disk {
    pub fn new(name: String) -> Self {
        let rotational = read_file_as_bool(&format!("/sys/block/{}/queue/rotational", name));
        let removable = read_file_as_bool(&format!("/sys/block/{}/removable", name));

        let ram = Regex::new(r"^zram\d*$").unwrap().is_match(&name);
        let dm_uuid = std::fs::read_to_string(&format!("/sys/block/{}/dm/uuid", name)).ok();
        let crypted = dm_uuid
            .as_ref()
            .map_or(false, |uuid| uuid.starts_with("CRYPT-"));
        let lvm = dm_uuid
            .as_ref()
            .map_or(false, |uuid| uuid.starts_with("LVM-"));
        Self {
            name,
            rotational,
            removable,
            ram,
            lvm,
            crypted,
        }
    }
    /// a synthetic code trying to express the essence of the type of media,
    /// an empty str being returned when information couldn't be gathered.
    /// This code is for humans and may change in future minor versions.
    pub fn disk_type(&self) -> &'static str {
        if self.ram {
            "RAM"
        } else if self.crypted {
            "crypt"
        } else if self.lvm {
            "LVM"
        } else {
            match (self.removable, self.rotational) {
                (Some(true), _) => "removable disk",
                (Some(false), Some(true)) => "HDD",
                (Some(false), Some(false)) => "SSD",
                _ => "",
            }
        }
    }
}
#[test]
fn test_disk() {
    //use crate::disk::Disk;
    std::fs::read_dir("/sys/block").unwrap().for_each(|entry| {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        let disk = Disk::new(name);
        println!("{}: {}", disk.name, disk.disk_type());
    });
}
