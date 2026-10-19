// tsrs-only: tests for the per-root streams (`fseventsMaxRootStreams`); Go has one set of streams for every root.

use std::sync::Arc;

use super::{
    flagKernelDropped, flagMustScanSubDirs, flagUserDropped, fsEventsBackend,
    fseventsMaxRootStreams, new_fs_events_backend,
};
use crate::debounce::debounce;
use crate::fsevents_darwin_ffi::{dispatch_test_event, teardown_stream};
use crate::pathcompare::pathComparer;
use crate::watcher::{ErrOverflow, dirWatch, watcherImpl};

fn root_dirs(n: usize) -> (std::path::PathBuf, Vec<String>) {
    let base = std::env::temp_dir().join(format!("tsrs-fswatch-roots-{}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let base = {
        std::fs::create_dir_all(&base).unwrap();
        std::fs::canonicalize(&base).unwrap()
    };
    let dirs = (0..n)
        .map(|i| {
            let dir = base.join(format!("r{i:04}"));
            std::fs::create_dir(&dir).unwrap();
            dir.to_str().unwrap().to_string()
        })
        .collect();
    (base, dirs)
}

fn stream_counts(backend: &fsEventsBackend) -> (usize, usize, usize) {
    let st = backend.mu.lock().unwrap();
    (st.root_streams.len(), st.shared_roots.len(), st.shared_streams.len())
}

#[test]
fn each_root_gets_its_own_stream_up_to_the_limit_and_the_rest_share() {
    let (base, dirs) = root_dirs(fseventsMaxRootStreams + 3);
    let db: Arc<debounce> = debounce::new();
    let watches: Vec<Arc<dirWatch>> = dirs
        .iter()
        .map(|dir| {
            dirWatch::new(dir.clone(), dir.clone(), &db, pathComparer::default(), None, true)
        })
        .collect();
    let backend = new_fs_events_backend();

    backend.subscribe_many(&watches[..2]).unwrap().unwrap();
    assert_eq!(stream_counts(&backend), (2, 0, 0));

    backend.subscribe_many(&watches[2..]).unwrap().unwrap();
    assert_eq!(stream_counts(&backend), (fseventsMaxRootStreams, 3, 1));

    // A second watch on a root that has its own stream (here non-recursive) joins that stream.
    let same_root =
        dirWatch::new(dirs[0].clone(), dirs[0].clone(), &db, pathComparer::default(), None, false);
    backend.subscribe(&same_root).unwrap();
    assert_eq!(stream_counts(&backend), (fseventsMaxRootStreams, 3, 1));
    backend.close_watch(&watches[0]).unwrap();
    assert_eq!(stream_counts(&backend), (fseventsMaxRootStreams, 3, 1));
    backend.close_watch(&same_root).unwrap();
    assert_eq!(stream_counts(&backend), (fseventsMaxRootStreams - 1, 3, 1));

    let last = watches.len() - 1;
    backend.close_watch(&watches[last]).unwrap();
    assert_eq!(stream_counts(&backend), (fseventsMaxRootStreams - 1, 2, 1));

    for watch in &watches[1..last] {
        backend.close_watch(watch).unwrap();
    }
    assert_eq!(stream_counts(&backend), (0, 0, 0));
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn a_drop_under_one_root_does_not_overflow_another() {
    let (base, dirs) = root_dirs(2);
    let db = debounce::new();
    let quiet =
        dirWatch::new(dirs[0].clone(), dirs[0].clone(), &db, pathComparer::default(), None, true);
    let busy =
        dirWatch::new(dirs[1].clone(), dirs[1].clone(), &db, pathComparer::default(), None, true);
    let busy_non_recursive =
        dirWatch::new(dirs[1].clone(), dirs[1].clone(), &db, pathComparer::default(), None, false);
    let watches = [quiet, busy, busy_non_recursive];
    let backend = new_fs_events_backend();
    backend.subscribe_many(&watches).unwrap().unwrap();

    // Keep the callbacks the real subscription created, but stop native delivery before injecting events:
    // whether fseventsd drops a burst of writes depends on the OS and machine load, not watcher correctness.
    let streams = {
        let st = backend.mu.lock().unwrap();
        dirs.iter().map(|dir| Arc::clone(&st.root_streams[dir])).collect::<Vec<_>>()
    };
    let callbacks: Vec<_> = streams
        .iter()
        .map(|stream| {
            let mut cb = stream.cb.lock().unwrap().take().unwrap();
            let native = stream.stream.lock().unwrap().take().unwrap();
            teardown_stream(native, Some(&mut cb));
            cb
        })
        .collect();
    for watch in &watches {
        let _ = watch.events.drain();
    }

    // An overflow can name the root or an ancestor. The ancestor case would match the quiet root too if
    // its watch leaked into the busy stream's callback. Both logical watches on the busy root must receive it.
    for path in [dirs[1].as_str(), base.to_str().unwrap()] {
        for dropped in [flagUserDropped, flagKernelDropped, 0] {
            dispatch_test_event(&callbacks[1], path, flagMustScanSubDirs | dropped);
            let (events, err) = watches[0].events.drain();
            assert_eq!(events, []);
            assert!(err.is_none(), "the busy root's drop overflowed the quiet root");
            for watch in &watches[1..] {
                let (events, err) = watch.events.drain();
                assert_eq!(events, []);
                assert!(
                    err.is_some_and(|err| err.is(ErrOverflow)),
                    "the busy root missed its overflow"
                );
            }
        }
    }

    for watch in &watches {
        backend.close_watch(watch).unwrap();
    }
    let _ = std::fs::remove_dir_all(&base);
}
