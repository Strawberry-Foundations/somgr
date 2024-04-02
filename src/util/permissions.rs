use nix::unistd::Uid;

pub fn check_root_permissions() -> bool {
    if !Uid::effective().is_root() {
        return false;
    }
    true
}