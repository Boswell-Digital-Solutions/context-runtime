mod common;
use common::TempRepo;
use context_runtime::gather::gather;
use std::{fs, time::SystemTime};

#[test]
fn rejects_absolute_and_parent_targets() {
    let repo = TempRepo::python();
    for path in ["../README.md", "pkg/../../secret", "/etc/passwd"] {
        assert!(gather("fixture", &repo.root, path, SystemTime::now()).is_err());
    }
}

#[cfg(unix)]
#[test]
fn rejects_symlink_escape_in_every_read_role() {
    use std::os::unix::fs::symlink;
    for relative in ["pkg/target.py", "pkg/other.py", "README.md"] {
        let repo = TempRepo::python();
        let outside = TempRepo::python();
        fs::remove_file(repo.root.join(relative)).unwrap();
        symlink(outside.root.join("README.md"), repo.root.join(relative)).unwrap();
        assert!(
            gather("fixture", &repo.root, repo.target_rel(), SystemTime::now()).is_err(),
            "{relative}"
        );
    }
}

#[cfg(unix)]
#[test]
fn rejects_symlink_parent_escape() {
    let repo = TempRepo::python();
    let outside = TempRepo::python();
    std::os::unix::fs::symlink(&outside.root, repo.root.join("outside")).unwrap();
    assert!(
        gather(
            "fixture",
            &repo.root,
            "outside/pkg/target.py",
            SystemTime::now()
        )
        .is_err()
    );
}

#[test]
fn rejects_oversized_source() {
    let repo = TempRepo::python();
    fs::write(
        repo.root.join(repo.target_rel()),
        vec![b'x'; 2 * 1024 * 1024 + 1],
    )
    .unwrap();
    assert!(gather("fixture", &repo.root, repo.target_rel(), SystemTime::now()).is_err());
}
