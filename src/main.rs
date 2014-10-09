#![feature(globs, macro_rules)]

extern crate libc;

use std::ptr;
use std::c_str::CString;
use std::finally::Finally;
use std::collections::HashMap;
use std::mem::transmute;

use libc::{c_char, c_int, c_void};

use git2_ffi::*;

mod git2_ffi;

macro_rules! c_str {
    ($s:expr) => { {
        (concat!($s, "\0")).as_ptr() as *const i8
    } }
}

fn main() {
    unsafe {
        do_the_thing();
    }
}

unsafe fn or_fail(status: c_int) {
    if status != GIT_OK {
        fail_on_git_err();
    }
}

unsafe fn fail_on_git_err() -> ! {
    let msg = CString::new((*giterr_last()).message as *const c_char, false);
    fail!(msg.to_string());
}

unsafe extern "C" fn tree_walk_cb(root: *const c_char,
                entry: *const git_tree_entry,
                cb: *mut c_void) -> c_int {
    type ClosureType<'a> = |*const c_char, *const git_tree_entry|:'a -> c_int;
    let closure = cb as *const ClosureType;

    return (*closure)(root, entry);
}

unsafe fn add_blame(blames: &mut HashMap<Vec<u8>, uint>,
                    path: *const c_char,
                    repos: *mut git_repository,
                    error: &mut Option<String>) {
    let mut blame = ptr::null_mut();
    println!("blame for {}", CString::new(path, false).as_str().unwrap());
    if git_blame_file(&mut blame, repos, path, ptr::null_mut()) != GIT_OK {
        let msg = CString::new((*giterr_last()).message as *const c_char, false);
        *error = Some(msg.to_string());
    } (|| {

    for i in range(0, git_blame_get_hunk_count(blame)) {
        let hunk = git_blame_get_hunk_byindex(blame, i);

        let final_commit_id = (*hunk).final_commit_id;
        let mut commit: *mut git_commit = ptr::null_mut();
        if git_commit_lookup(&mut commit, repos, &final_commit_id) != GIT_OK {
            let msg = CString::new((*giterr_last()).message as *const c_char, false);
            *error = Some(msg.to_string());
        if commit.is_null() { continue; }
        }
        (|| {

        let author = git_commit_author(commit as *const git_commit);
        let lines = (*hunk).lines_in_hunk as uint;
        let len = libc::strlen((*author).name as *const c_char);
        let author_name_tuple = (len, len, (*author).name);
        let author_name_fake_vec: &Vec<u8> = transmute(&author_name_tuple);
        let done = match blames.find_mut(author_name_fake_vec) {
            Some(p) => { *p += lines; true }
            _ => { false }
        };
        if !done { blames.insert(author_name_fake_vec.clone(), lines); }

        }).finally(||{ git_commit_free(commit); });
    }

    }).finally(||{ git_blame_free(blame); });
}

unsafe fn do_the_thing() {
    let mut blames = HashMap::new();
    or_fail(git_threads_init()); (|| {
    let mut repos = ptr::null_mut();
    or_fail(git_repository_open(&mut repos, c_str!("."))); (|| {

    let mut tree = ptr::null_mut();

    let mut head = ptr::null_mut();
    or_fail(git_repository_head(&mut head, repos)); (|| {

    let commit_id = git_reference_target(head as *const git_reference);

    let mut commit = ptr::null_mut();
    or_fail(git_commit_lookup(&mut commit, repos, commit_id)); (|| {
    or_fail(git_commit_tree(&mut tree, commit as *const git_commit));

    }).finally(||{ git_commit_free(commit); });
    }).finally(||{ git_reference_free(head); });
    (|| {

    let mut error = None;
    git_tree_walk(tree as *const _, GIT_TREEWALK_PRE, Some(tree_walk_cb), (&mut |root, entry| {
        // i guess we shouldn't fail in here...
        let root = CString::new(root, false);
        let entry_type = git_tree_entry_type(entry);
        let entry_name = CString::new(git_tree_entry_name(entry), false);
        if entry_type == GIT_OBJ_BLOB {
            let root = root.as_bytes_no_nul();
            let name = entry_name.as_bytes();
            let mut path = Vec::with_capacity(root.len() + name.len());
            path.push_all(root);
            path.push_all(name);

            add_blame(&mut blames, path.as_ptr() as *const c_char, repos, &mut error);
            if error.is_some() { return -1 as c_int; }
        }

        return 0 as c_int;
    }) as *mut _ as *mut c_void);
    match error { Some(e) => fail!(e), _ => () }
    }).finally(||{ git_tree_free(tree); });
    }).finally(||{ git_repository_free(repos); });
    }).finally(||{ git_threads_shutdown(); });
    for (name_vec, &lines) in blames.iter() {
        let name = String::from_utf8_lossy(name_vec[]);
        println!("{}\t{}", lines, name.as_slice());
    }
}
