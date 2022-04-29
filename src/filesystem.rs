use std::io::Read;
use std::str::FromStr;
static REMOTE_ONLY_FS_TYPES: &[&str] = &[
    "afs",
    "coda",
    "auristorfs",
    "fhgfs",
    "gpfs",
    "ibrix",
    "ocfs2",
    "vxfs",
];

//// A mount info as described in /proc/self/mountinfo
#[derive(Debug, Clone)]
pub struct MountInfo {
    ///
    // specified discriminant is interpreted as an isize
    /// (0) The ID of the mount (may be reused after umount)
    pub mount_id: usize,
    //// (1) The parent ID of the mount (may be reused after umount)
    pub parent_id: usize,
    //// (2) The number of children of the mount
    pub major_minor: String,
    /*  (3) root: the pathname of the directory in the filesystem
                   which forms the root of this mount.
    */
    pub root: String,
    /// (4) mount point: the pathname of the mount point relative
    ///             to the process's root directory.
    pub mount_point: String,
    /// (5) filesystem type: the filesystem type in the form "type.subtype".
    pub fs_type: String,
    /// (6) mount point: the pathname of the mount point relative to the process's root directory.
    pub mount_options: Vec<String>,

    /// (7) mount options: per-mount options (see mount(2)).
    pub mount_source: String,
    /// (8) optional fields: zero or more fields of the form
    ///     "tag[:value]"; see below.
    pub optional_fields: String,
    /// (9) super options: per-superblock options
    pub super_options: String,
}
pub enum MountInfoParameter {
    /// A line of self/ has the following structure:
    /// 36  35  98:0 /mnt1 /mnt2 rw,noatime master:1 - ext3 /dev/root rw,errors=continue
    /// (0) (1) (2)   (3)   (4)      (5)      (6)   (7) (8)    (9)           (10)
    ///
    /// (0) mount ID: unique identifier of the mount (may be reused after umount).
    MountID = 0,
    /// (1) parent ID: ID of parent (or of self for the top of the mount tree).
    ParentID = 1,
    /// (2) major:minor: value of st_dev for files on filesystem.
    MajorMinor = 2,
    /// (3) root: root of the mount within the filesystem.
    Root = 3,
    /// (4) mount point: mount point relative to the process's root.
    /// the sub-directory in the "/" directory. "/boot", "/home"
    MountPoint = 4,
    /// (5) mount options: per mount options.
    MountOptions = 5,
    /// (6) optional fields: zero or more fields terminated by "-".
    OptionalFields = 6,
    /// (7) filesystem type: name of filesystem of the form.
    FsType = 7,
    /// (8) mount source: filesystem specific information or "none".
    /// the partition in the disk. "/dev/nvme0n1p1"
    MountSource = 8,
    /// (9) super options: per super block options. "rw"
    SuperOptions = 9,
}
impl MountInfo {
    pub fn is_remote(&self) -> bool {
        self.fs_type.contains(':')
            || (self.fs_type.starts_with("//")
                && ["cifs", "smb3", "smbfs"].contains(&self.fs_type.as_ref()))
            || REMOTE_ONLY_FS_TYPES.contains(&self.fs_type.as_ref())
            || self.fs_type == "-hosts"
    }
}
use self::MountInfoParameter::*;
impl FromStr for MountInfo {
    type Err = Box<dyn std::error::Error>;
    fn from_str(line: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut fields = line.split_whitespace().collect::<Vec<_>>();
        fields.retain(|v| *v != "-");
        let mount_id = fields[MountID as usize].parse::<usize>()?;
        let parent_id = fields[ParentID as usize].parse::<usize>()?;
        let major_minor = fields[MajorMinor as usize].to_string();
        let root = fields[Root as usize].to_string();
        let mount_point = fields[MountPoint as usize].to_string();
        let mount_options = fields[MountOptions as usize]
            .split(',')
            .map(|v| v.to_string())
            .collect::<Vec<_>>();
        let optional_fields = fields[OptionalFields as usize].to_string();
        let fs_type = fields[FsType as usize].to_string();
        let mount_source = fields[MountSource as usize].to_string();
        let super_options = fields[SuperOptions as usize].to_string();

        Ok({
            MountInfo {
                mount_id,
                parent_id,
                major_minor,
                root,
                mount_point,
                mount_options,
                optional_fields,
                fs_type,
                mount_source,
                super_options,
            }
        })
    }
}
pub fn read_mount_info_file() -> Result<Vec<MountInfo>, Box<dyn std::error::Error>> {
    let mut mount_info_file = std::fs::File::open("/proc/self/mountinfo")?;
    let mut mount_info_file_content = String::new();
    mount_info_file.read_to_string(&mut mount_info_file_content)?;
    let mount_info_lines = mount_info_file_content.lines().collect::<Vec<_>>();
    let mount_info = mount_info_lines
        .iter()
        .map(|line| line.parse::<MountInfo>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(mount_info)
}
#[test]
fn test_from_str() {
    let line = "65 1 0:33 /root / rw,relatime shared:1 - btrFsType /dev/nvme0n1p3 rw,seclabel,compress=zstd:1,ssd,space_cache,subvolid=257,subvol=/root";
    let mount_info = MountInfo::from_str(line).unwrap();
    assert_eq!(mount_info.mount_id, 65);
    assert_eq!(mount_info.parent_id, 1);
    assert_eq!(mount_info.major_minor, "0:33");
    assert_eq!(mount_info.root, "/root");
    assert_eq!(mount_info.mount_point, "/");
    assert_eq!(mount_info.mount_options, vec!["rw", "relatime"]);
    assert_eq!(mount_info.optional_fields, "shared:1");
    assert_eq!(mount_info.fs_type, "btrFsType");
    assert_eq!(mount_info.mount_source, "/dev/nvme0n1p3");
    assert_eq!(
        mount_info.super_options,
        "rw,seclabel,compress=zstd:1,ssd,space_cache,subvolid=257,subvol=/root"
    );
}
