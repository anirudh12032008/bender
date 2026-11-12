// Include directories must keep dependency precedence: first match wins, so a package's own
// include dirs come before those of its dependencies.

use assert_cmd::cargo;

fn incdirs(format: &str) -> Vec<String> {
    let out = cargo::cargo_bin_cmd!()
        .args(["-d", "tests/incdir_order", "script", format])
        .output()
        .expect("Failed to execute bender binary");
    assert!(
        out.status.success(),
        "script command failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("stdout must be utf-8")
        .lines()
        .filter_map(|l| {
            l.find("+incdir+")
                .map(|i| l[i + 8..].trim_end_matches(['"', '\\', ' ']))
        })
        .map(|p| p.replace('\\', "/"))
        .collect()
}

/// `zroot_inc` sorts after the dependencies' directories, so alphabetical order would put it last.
#[test]
fn flist_plus_root_incdir_first() {
    let dirs = incdirs("flist-plus");
    assert_eq!(dirs.len(), 3, "{dirs:?}");
    assert!(dirs[0].ends_with("zroot_inc"), "{dirs:?}");
}

/// `mid` depends on `adep`: within its group, `mid/inc` must stay ahead of `adep/inc`.
#[test]
fn vsim_group_incdirs_keep_dependency_order() {
    let dirs = incdirs("vsim");
    assert!(
        dirs.windows(2)
            .any(|w| w[0].ends_with("mid/inc") && w[1].ends_with("adep/inc")),
        "{dirs:?}"
    );
}
