use nix::{libc::c_int, sys::stat::Mode};

const FP_SPECIAL: c_int = 1;

#[rustfmt::skip]
pub fn file_perm_str(perm: Mode, flags: c_int) -> String {
    format!(
            "{}{}{}{}{}{}{}{}{}",
            if perm.contains(Mode::S_IRUSR) { 'r' } else { '-' },
            if perm.contains(Mode::S_IWUSR) { 'w' } else { '-' },
            if perm.contains(Mode::S_IXUSR) {
                if perm.contains(Mode::S_ISUID) && (flags & FP_SPECIAL) > 0 { 's' } else { 'x' }
            } else {
                if perm.contains(Mode::S_ISUID) && (flags & FP_SPECIAL) > 0 { 'S' } else { '-' }
            },
            if perm.contains(Mode::S_IRGRP) { 'r' } else { '-' },
            if perm.contains(Mode::S_IWGRP) { 'w' } else { '-' },
            if perm.contains(Mode::S_IXGRP) {
                if perm.contains(Mode::S_ISGID) && (flags & FP_SPECIAL) > 0 { 's' } else { 'x' }
            } else {
                if perm.contains(Mode::S_ISGID) && (flags & FP_SPECIAL) > 0 { 'S' } else { '-' }
            },
            if perm.contains(Mode::S_IROTH) { 'r' } else { '-' },
            if perm.contains(Mode::S_IWOTH) { 'w' } else { '-' },
            if perm.contains(Mode::S_IXOTH) {
                if perm.contains(Mode::S_ISVTX) && (flags & FP_SPECIAL) > 0 { 't' } else { 'x' }
            } else {
                if perm.contains(Mode::S_ISVTX) && (flags & FP_SPECIAL) > 0 { 'T' } else { '-' }
            }
    )
}
