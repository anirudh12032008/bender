// Include directories must keep dependency precedence: first match wins, so a package's own
// include dirs come before those of its dependencies.

use assert_cmd::cargo;

/// Include directories per source group (vsim starts a new group at each `vlog`), relative to the
/// fixture root.
fn incdirs(format: &str) -> Vec<Vec<String>> {
    let out = cargo::cargo_bin_cmd!()
        .args(["-d", "tests/incdir_order", "script", format])
        .output()
        .expect("Failed to execute bender binary");
    assert!(
        out.status.success(),
        "script command failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).expect("stdout must be utf-8");
    let mut groups = vec![vec![]];
    for l in stdout.lines() {
        if l.contains("vlog") {
            groups.push(vec![]);
        }
        if let Some(i) = l.find("+incdir+") {
            let p = l[i + 8..]
                .trim_end_matches(['"', '\\', ' '])
                .replace('\\', "/");
            let p = p.rsplit_once("incdir_order/").map_or(p.as_str(), |x| x.1);
            let p = p.strip_prefix("${ROOT}/").unwrap_or(p);
            groups.last_mut().unwrap().push(p.to_string());
        }
    }
    groups.retain(|g| !g.is_empty());
    groups
}

/// `zroot_inc` sorts after the dependencies' directories, so alphabetical order would put it last.
#[test]
fn flist_plus_root_incdir_first() {
    assert_eq!(
        incdirs("flist-plus"),
        [["zroot_inc", "adep/inc", "mid/inc"]]
    );
}

/// `mid` depends on `adep`: within its group, `mid/inc` must stay ahead of `adep/inc`, which is
/// the reverse of alphabetical order.
#[test]
fn vsim_group_incdirs_keep_dependency_order() {
    assert_eq!(
        incdirs("vsim"),
        [
            vec!["adep/inc"],
            vec!["mid/inc", "adep/inc"],
            vec!["zroot_inc", "adep/inc", "mid/inc"],
        ]
    );
}
