#![allow(dead_code, non_camel_case_types)]

use libc::size_t;

pub type Enum_Unnamed1 = ::libc::c_uint;
pub static GIT_CAP_THREADS: ::libc::c_uint = 1;
pub static GIT_CAP_HTTPS: ::libc::c_uint = 2;
pub static GIT_CAP_SSH: ::libc::c_uint = 4;
pub type git_cap_t = Enum_Unnamed1;
pub type Enum_Unnamed2 = ::libc::c_uint;
pub static GIT_OPT_GET_MWINDOW_SIZE: ::libc::c_uint = 0;
pub static GIT_OPT_SET_MWINDOW_SIZE: ::libc::c_uint = 1;
pub static GIT_OPT_GET_MWINDOW_MAPPED_LIMIT: ::libc::c_uint = 2;
pub static GIT_OPT_SET_MWINDOW_MAPPED_LIMIT: ::libc::c_uint = 3;
pub static GIT_OPT_GET_SEARCH_PATH: ::libc::c_uint = 4;
pub static GIT_OPT_SET_SEARCH_PATH: ::libc::c_uint = 5;
pub static GIT_OPT_SET_CACHE_OBJECT_LIMIT: ::libc::c_uint = 6;
pub static GIT_OPT_SET_CACHE_MAX_SIZE: ::libc::c_uint = 7;
pub static GIT_OPT_ENABLE_CACHING: ::libc::c_uint = 8;
pub static GIT_OPT_GET_CACHED_MEMORY: ::libc::c_uint = 9;
pub static GIT_OPT_GET_TEMPLATE_PATH: ::libc::c_uint = 10;
pub static GIT_OPT_SET_TEMPLATE_PATH: ::libc::c_uint = 11;
pub type git_libgit2_opt_t = Enum_Unnamed2;
pub type git_off_t = i64;
pub type git_time_t = i64;
pub type Enum_Unnamed3 = ::libc::c_int;
pub static GIT_OBJ_ANY: ::libc::c_int = -2;
pub static GIT_OBJ_BAD: ::libc::c_int = -1;
pub static GIT_OBJ__EXT1: ::libc::c_int = 0;
pub static GIT_OBJ_COMMIT: ::libc::c_int = 1;
pub static GIT_OBJ_TREE: ::libc::c_int = 2;
pub static GIT_OBJ_BLOB: ::libc::c_int = 3;
pub static GIT_OBJ_TAG: ::libc::c_int = 4;
pub static GIT_OBJ__EXT2: ::libc::c_int = 5;
pub static GIT_OBJ_OFS_DELTA: ::libc::c_int = 6;
pub static GIT_OBJ_REF_DELTA: ::libc::c_int = 7;
pub type git_otype = Enum_Unnamed3;
pub enum Struct_git_odb { }
pub type git_odb = Struct_git_odb;
pub enum Struct_git_odb_backend { }
pub type git_odb_backend = Struct_git_odb_backend;
pub enum Struct_git_odb_object { }
pub type git_odb_object = Struct_git_odb_object;
pub enum Struct_git_odb_stream { }
pub type git_odb_stream = Struct_git_odb_stream;
pub enum Struct_git_odb_writepack { }
pub type git_odb_writepack = Struct_git_odb_writepack;
pub enum Struct_git_refdb { }
pub type git_refdb = Struct_git_refdb;
pub enum Struct_git_refdb_backend { }
pub type git_refdb_backend = Struct_git_refdb_backend;
pub enum Struct_git_repository { }
pub type git_repository = Struct_git_repository;
pub enum Struct_git_object { }
pub type git_object = Struct_git_object;
pub enum Struct_git_revwalk { }
pub type git_revwalk = Struct_git_revwalk;
pub enum Struct_git_tag { }
pub type git_tag = Struct_git_tag;
pub enum Struct_git_blob { }
pub type git_blob = Struct_git_blob;
pub enum Struct_git_commit { }
pub type git_commit = Struct_git_commit;
pub enum Struct_git_tree_entry { }
pub type git_tree_entry = Struct_git_tree_entry;
pub enum Struct_git_tree { }
pub type git_tree = Struct_git_tree;
pub enum Struct_git_treebuilder { }
pub type git_treebuilder = Struct_git_treebuilder;
pub enum Struct_git_index { }
pub type git_index = Struct_git_index;
pub enum Struct_git_index_conflict_iterator { }
pub type git_index_conflict_iterator = Struct_git_index_conflict_iterator;
pub enum Struct_git_config { }
pub type git_config = Struct_git_config;
pub enum Struct_git_config_backend { }
pub type git_config_backend = Struct_git_config_backend;
pub enum Struct_git_reflog_entry { }
pub type git_reflog_entry = Struct_git_reflog_entry;
pub enum Struct_git_reflog { }
pub type git_reflog = Struct_git_reflog;
pub enum Struct_git_note { }
pub type git_note = Struct_git_note;
pub enum Struct_git_packbuilder { }
pub type git_packbuilder = Struct_git_packbuilder;
#[repr(C)]
pub struct Struct_git_time {
    pub time: git_time_t,
    pub offset: ::libc::c_int,
}
pub type git_time = Struct_git_time;
#[repr(C)]
pub struct Struct_git_signature {
    pub name: *mut ::libc::c_char,
    pub email: *mut ::libc::c_char,
    pub when: git_time,
}
pub type git_signature = Struct_git_signature;
pub enum Struct_git_reference { }
pub type git_reference = Struct_git_reference;
pub enum Struct_git_reference_iterator { }
pub type git_reference_iterator = Struct_git_reference_iterator;
pub enum Struct_git_merge_head { }
pub type git_merge_head = Struct_git_merge_head;
pub enum Struct_git_merge_result { }
pub type git_merge_result = Struct_git_merge_result;
pub enum Struct_git_status_list { }
pub type git_status_list = Struct_git_status_list;
pub type Enum_Unnamed4 = ::libc::c_uint;
pub static GIT_REF_INVALID: ::libc::c_uint = 0;
pub static GIT_REF_OID: ::libc::c_uint = 1;
pub static GIT_REF_SYMBOLIC: ::libc::c_uint = 2;
pub static GIT_REF_LISTALL: ::libc::c_uint = 3;
pub type git_ref_t = Enum_Unnamed4;
pub type Enum_Unnamed5 = ::libc::c_uint;
pub static GIT_BRANCH_LOCAL: ::libc::c_uint = 1;
pub static GIT_BRANCH_REMOTE: ::libc::c_uint = 2;
pub type git_branch_t = Enum_Unnamed5;
pub type Enum_Unnamed6 = ::libc::c_uint;
pub static GIT_FILEMODE_NEW: ::libc::c_uint = 0;
pub static GIT_FILEMODE_TREE: ::libc::c_uint = 16384;
pub static GIT_FILEMODE_BLOB: ::libc::c_uint = 33188;
pub static GIT_FILEMODE_BLOB_EXECUTABLE: ::libc::c_uint = 33261;
pub static GIT_FILEMODE_LINK: ::libc::c_uint = 40960;
pub static GIT_FILEMODE_COMMIT: ::libc::c_uint = 57344;
pub type git_filemode_t = Enum_Unnamed6;
pub enum Struct_git_refspec { }
pub type git_refspec = Struct_git_refspec;
pub enum Struct_git_remote { }
pub type git_remote = Struct_git_remote;
pub enum Struct_git_push { }
pub type git_push = Struct_git_push;
pub type git_remote_head = Struct_git_remote_head;
pub type git_remote_callbacks = Struct_git_remote_callbacks;
#[repr(C)]
pub struct Struct_git_transfer_progress {
    pub total_objects: ::libc::c_uint,
    pub indexed_objects: ::libc::c_uint,
    pub received_objects: ::libc::c_uint,
    pub local_objects: ::libc::c_uint,
    pub total_deltas: ::libc::c_uint,
    pub indexed_deltas: ::libc::c_uint,
    pub received_bytes: size_t,
}
pub type git_transfer_progress = Struct_git_transfer_progress;
pub type git_transfer_progress_callback =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const git_transfer_progress,
                               arg2: *mut ::libc::c_void) -> ::libc::c_int>;
pub enum Struct_git_submodule { }
pub type git_submodule = Struct_git_submodule;
pub type Enum_Unnamed7 = ::libc::c_int;
pub static GIT_SUBMODULE_UPDATE_RESET: ::libc::c_int = -1;
pub static GIT_SUBMODULE_UPDATE_CHECKOUT: ::libc::c_int = 1;
pub static GIT_SUBMODULE_UPDATE_REBASE: ::libc::c_int = 2;
pub static GIT_SUBMODULE_UPDATE_MERGE: ::libc::c_int = 3;
pub static GIT_SUBMODULE_UPDATE_NONE: ::libc::c_int = 4;
pub static GIT_SUBMODULE_UPDATE_DEFAULT: ::libc::c_int = 0;
pub type git_submodule_update_t = Enum_Unnamed7;
pub type Enum_Unnamed8 = ::libc::c_int;
pub static GIT_SUBMODULE_IGNORE_RESET: ::libc::c_int = -1;
pub static GIT_SUBMODULE_IGNORE_NONE: ::libc::c_int = 1;
pub static GIT_SUBMODULE_IGNORE_UNTRACKED: ::libc::c_int = 2;
pub static GIT_SUBMODULE_IGNORE_DIRTY: ::libc::c_int = 3;
pub static GIT_SUBMODULE_IGNORE_ALL: ::libc::c_int = 4;
pub static GIT_SUBMODULE_IGNORE_DEFAULT: ::libc::c_int = 0;
pub type git_submodule_ignore_t = Enum_Unnamed8;
pub type Enum_Unnamed9 = ::libc::c_uint;
pub static GIT_ATTR_UNSPECIFIED_T: ::libc::c_uint = 0;
pub static GIT_ATTR_TRUE_T: ::libc::c_uint = 1;
pub static GIT_ATTR_FALSE_T: ::libc::c_uint = 2;
pub static GIT_ATTR_VALUE_T: ::libc::c_uint = 3;
pub type git_attr_t = Enum_Unnamed9;
pub type git_attr_foreach_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const ::libc::c_char,
                               arg2: *const ::libc::c_char,
                               arg3: *mut ::libc::c_void) -> ::libc::c_int>;
#[repr(C)]
pub struct Struct_git_oid {
    pub id: [::libc::c_uchar, ..20u],
}
pub type git_oid = Struct_git_oid;
pub enum Struct_git_oid_shorten { }
pub type git_oid_shorten = Struct_git_oid_shorten;
#[repr(C)]
pub struct Struct_Unnamed10 {
    pub ptr: *mut ::libc::c_char,
    pub asize: size_t,
    pub size: size_t,
}
pub type git_buf = Struct_Unnamed10;
pub type git_blob_chunk_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *mut ::libc::c_char, arg2: size_t,
                               arg3: *mut ::libc::c_void) -> ::libc::c_int>;
pub type Enum_Unnamed11 = ::libc::c_uint;
pub static GIT_BLAME_NORMAL: ::libc::c_uint = 0;
pub static GIT_BLAME_TRACK_COPIES_SAME_FILE: ::libc::c_uint = 1;
pub static GIT_BLAME_TRACK_COPIES_SAME_COMMIT_MOVES: ::libc::c_uint = 2;
pub static GIT_BLAME_TRACK_COPIES_SAME_COMMIT_COPIES: ::libc::c_uint = 4;
pub static GIT_BLAME_TRACK_COPIES_ANY_COMMIT_COPIES: ::libc::c_uint = 8;
pub type git_blame_flag_t = Enum_Unnamed11;
#[repr(C)]
pub struct Struct_git_blame_options {
    pub version: ::libc::c_uint,
    pub flags: u32,
    pub min_match_characters: u16,
    pub newest_commit: git_oid,
    pub oldest_commit: git_oid,
    pub min_line: u32,
    pub max_line: u32,
}
pub type git_blame_options = Struct_git_blame_options;
#[repr(C)]
pub struct Struct_git_blame_hunk {
    pub lines_in_hunk: u16,
    pub final_commit_id: git_oid,
    pub final_start_line_number: u16,
    pub final_signature: *mut git_signature,
    pub orig_commit_id: git_oid,
    pub orig_path: *const ::libc::c_char,
    pub orig_start_line_number: u16,
    pub orig_signature: *mut git_signature,
    pub boundary: ::libc::c_char,
}
pub type git_blame_hunk = Struct_git_blame_hunk;
pub enum Struct_git_blame { }
pub type git_blame = Struct_git_blame;
pub enum Struct_git_branch_iterator { }
pub type git_branch_iterator = Struct_git_branch_iterator;
pub type git_treebuilder_filter_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const git_tree_entry,
                               arg2: *mut ::libc::c_void) -> ::libc::c_int>;
pub type git_treewalk_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const ::libc::c_char,
                               arg2: *const git_tree_entry,
                               arg3: *mut ::libc::c_void) -> ::libc::c_int>;
pub type Enum_Unnamed12 = ::libc::c_uint;
pub static GIT_TREEWALK_PRE: ::libc::c_uint = 0;
pub static GIT_TREEWALK_POST: ::libc::c_uint = 1;
pub type git_treewalk_mode = Enum_Unnamed12;
#[repr(C)]
pub struct Struct_git_strarray {
    pub strings: *mut *mut ::libc::c_char,
    pub count: size_t,
}
pub type git_strarray = Struct_git_strarray;
pub type git_reference_foreach_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *mut git_reference,
                               arg2: *mut ::libc::c_void) -> ::libc::c_int>;
pub type git_reference_foreach_name_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const ::libc::c_char,
                               arg2: *mut ::libc::c_void) -> ::libc::c_int>;
pub type Enum_Unnamed13 = ::libc::c_uint;
pub static GIT_REF_FORMAT_NORMAL: ::libc::c_uint = 0;
pub static GIT_REF_FORMAT_ALLOW_ONELEVEL: ::libc::c_uint = 1;
pub static GIT_REF_FORMAT_REFSPEC_PATTERN: ::libc::c_uint = 2;
pub static GIT_REF_FORMAT_REFSPEC_SHORTHAND: ::libc::c_uint = 4;
pub type git_reference_normalize_t = Enum_Unnamed13;
pub type Enum_Unnamed14 = ::libc::c_uint;
pub static GIT_DIFF_NORMAL: ::libc::c_uint = 0;
pub static GIT_DIFF_REVERSE: ::libc::c_uint = 1;
pub static GIT_DIFF_INCLUDE_IGNORED: ::libc::c_uint = 2;
pub static GIT_DIFF_RECURSE_IGNORED_DIRS: ::libc::c_uint = 4;
pub static GIT_DIFF_INCLUDE_UNTRACKED: ::libc::c_uint = 8;
pub static GIT_DIFF_RECURSE_UNTRACKED_DIRS: ::libc::c_uint = 16;
pub static GIT_DIFF_INCLUDE_UNMODIFIED: ::libc::c_uint = 32;
pub static GIT_DIFF_INCLUDE_TYPECHANGE: ::libc::c_uint = 64;
pub static GIT_DIFF_INCLUDE_TYPECHANGE_TREES: ::libc::c_uint = 128;
pub static GIT_DIFF_IGNORE_FILEMODE: ::libc::c_uint = 256;
pub static GIT_DIFF_IGNORE_SUBMODULES: ::libc::c_uint = 512;
pub static GIT_DIFF_IGNORE_CASE: ::libc::c_uint = 1024;
pub static GIT_DIFF_DISABLE_PATHSPEC_MATCH: ::libc::c_uint = 4096;
pub static GIT_DIFF_SKIP_BINARY_CHECK: ::libc::c_uint = 8192;
pub static GIT_DIFF_ENABLE_FAST_UNTRACKED_DIRS: ::libc::c_uint = 16384;
pub static GIT_DIFF_FORCE_TEXT: ::libc::c_uint = 1048576;
pub static GIT_DIFF_FORCE_BINARY: ::libc::c_uint = 2097152;
pub static GIT_DIFF_IGNORE_WHITESPACE: ::libc::c_uint = 4194304;
pub static GIT_DIFF_IGNORE_WHITESPACE_CHANGE: ::libc::c_uint = 8388608;
pub static GIT_DIFF_IGNORE_WHITESPACE_EOL: ::libc::c_uint = 16777216;
pub static GIT_DIFF_SHOW_UNTRACKED_CONTENT: ::libc::c_uint = 33554432;
pub static GIT_DIFF_SHOW_UNMODIFIED: ::libc::c_uint = 67108864;
pub static GIT_DIFF_PATIENCE: ::libc::c_uint = 268435456;
pub static GIT_DIFF_MINIMAL: ::libc::c_uint = 536870912;
pub type git_diff_option_t = Enum_Unnamed14;
pub enum Struct_git_diff { }
pub type git_diff = Struct_git_diff;
pub type Enum_Unnamed15 = ::libc::c_uint;
pub static GIT_DIFF_FLAG_BINARY: ::libc::c_uint = 1;
pub static GIT_DIFF_FLAG_NOT_BINARY: ::libc::c_uint = 2;
pub static GIT_DIFF_FLAG_VALID_OID: ::libc::c_uint = 4;
pub type git_diff_flag_t = Enum_Unnamed15;
pub type Enum_Unnamed16 = ::libc::c_uint;
pub static GIT_DELTA_UNMODIFIED: ::libc::c_uint = 0;
pub static GIT_DELTA_ADDED: ::libc::c_uint = 1;
pub static GIT_DELTA_DELETED: ::libc::c_uint = 2;
pub static GIT_DELTA_MODIFIED: ::libc::c_uint = 3;
pub static GIT_DELTA_RENAMED: ::libc::c_uint = 4;
pub static GIT_DELTA_COPIED: ::libc::c_uint = 5;
pub static GIT_DELTA_IGNORED: ::libc::c_uint = 6;
pub static GIT_DELTA_UNTRACKED: ::libc::c_uint = 7;
pub static GIT_DELTA_TYPECHANGE: ::libc::c_uint = 8;
pub type git_delta_t = Enum_Unnamed16;
#[repr(C)]
pub struct Struct_Unnamed17 {
    pub oid: git_oid,
    pub path: *const ::libc::c_char,
    pub size: git_off_t,
    pub flags: u32,
    pub mode: u16,
}
pub type git_diff_file = Struct_Unnamed17;
#[repr(C)]
pub struct Struct_Unnamed18 {
    pub status: git_delta_t,
    pub flags: u32,
    pub similarity: u16,
    pub nfiles: u16,
    pub old_file: git_diff_file,
    pub new_file: git_diff_file,
}
pub type git_diff_delta = Struct_Unnamed18;
pub type git_diff_notify_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const git_diff,
                               arg2: *const git_diff_delta,
                               arg3: *const ::libc::c_char,
                               arg4: *mut ::libc::c_void) -> ::libc::c_int>;
#[repr(C)]
pub struct Struct_Unnamed19 {
    pub version: ::libc::c_uint,
    pub flags: u32,
    pub ignore_submodules: git_submodule_ignore_t,
    pub pathspec: git_strarray,
    pub notify_cb: git_diff_notify_cb,
    pub notify_payload: *mut ::libc::c_void,
    pub context_lines: u16,
    pub interhunk_lines: u16,
    pub oid_abbrev: u16,
    pub max_size: git_off_t,
    pub old_prefix: *const ::libc::c_char,
    pub new_prefix: *const ::libc::c_char,
}
pub type git_diff_options = Struct_Unnamed19;
pub type git_diff_file_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const git_diff_delta,
                               arg2: ::libc::c_float,
                               arg3: *mut ::libc::c_void) -> ::libc::c_int>;
pub type git_diff_hunk = Struct_git_diff_hunk;
#[repr(C)]
pub struct Struct_git_diff_hunk {
    pub old_start: ::libc::c_int,
    pub old_lines: ::libc::c_int,
    pub new_start: ::libc::c_int,
    pub new_lines: ::libc::c_int,
    pub header_len: size_t,
    pub header: [::libc::c_char, ..128u],
}
pub type git_diff_hunk_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const git_diff_delta,
                               arg2: *const git_diff_hunk,
                               arg3: *mut ::libc::c_void) -> ::libc::c_int>;
pub type Enum_Unnamed20 = ::libc::c_uint;
pub static GIT_DIFF_LINE_CONTEXT: ::libc::c_uint = 32;
pub static GIT_DIFF_LINE_ADDITION: ::libc::c_uint = 43;
pub static GIT_DIFF_LINE_DELETION: ::libc::c_uint = 45;
pub static GIT_DIFF_LINE_CONTEXT_EOFNL: ::libc::c_uint = 61;
pub static GIT_DIFF_LINE_ADD_EOFNL: ::libc::c_uint = 62;
pub static GIT_DIFF_LINE_DEL_EOFNL: ::libc::c_uint = 60;
pub static GIT_DIFF_LINE_FILE_HDR: ::libc::c_uint = 70;
pub static GIT_DIFF_LINE_HUNK_HDR: ::libc::c_uint = 72;
pub static GIT_DIFF_LINE_BINARY: ::libc::c_uint = 66;
pub type git_diff_line_t = Enum_Unnamed20;
pub type git_diff_line = Struct_git_diff_line;
#[repr(C)]
pub struct Struct_git_diff_line {
    pub origin: ::libc::c_char,
    pub old_lineno: ::libc::c_int,
    pub new_lineno: ::libc::c_int,
    pub num_lines: ::libc::c_int,
    pub content_len: size_t,
    pub content_offset: git_off_t,
    pub content: *const ::libc::c_char,
}
pub type git_diff_line_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const git_diff_delta,
                               arg2: *const git_diff_hunk,
                               arg3: *const git_diff_line,
                               arg4: *mut ::libc::c_void) -> ::libc::c_int>;
pub type Enum_Unnamed21 = ::libc::c_uint;
pub static GIT_DIFF_FIND_RENAMES: ::libc::c_uint = 1;
pub static GIT_DIFF_FIND_RENAMES_FROM_REWRITES: ::libc::c_uint = 2;
pub static GIT_DIFF_FIND_COPIES: ::libc::c_uint = 4;
pub static GIT_DIFF_FIND_COPIES_FROM_UNMODIFIED: ::libc::c_uint = 8;
pub static GIT_DIFF_FIND_REWRITES: ::libc::c_uint = 16;
pub static GIT_DIFF_BREAK_REWRITES: ::libc::c_uint = 32;
pub static GIT_DIFF_FIND_AND_BREAK_REWRITES: ::libc::c_uint = 48;
pub static GIT_DIFF_FIND_FOR_UNTRACKED: ::libc::c_uint = 64;
pub static GIT_DIFF_FIND_ALL: ::libc::c_uint = 255;
pub static GIT_DIFF_FIND_IGNORE_LEADING_WHITESPACE: ::libc::c_uint = 0;
pub static GIT_DIFF_FIND_IGNORE_WHITESPACE: ::libc::c_uint = 4096;
pub static GIT_DIFF_FIND_DONT_IGNORE_WHITESPACE: ::libc::c_uint = 8192;
pub static GIT_DIFF_FIND_EXACT_MATCH_ONLY: ::libc::c_uint = 16384;
pub static GIT_DIFF_BREAK_REWRITES_FOR_RENAMES_ONLY: ::libc::c_uint = 32768;
pub type git_diff_find_t = Enum_Unnamed21;
#[repr(C)]
pub struct Struct_Unnamed22 {
    pub file_signature: ::std::option::Option<unsafe extern "C" fn
                                                  (arg1:
                                                       *mut *mut ::libc::c_void,
                                                   arg2: *const git_diff_file,
                                                   arg3:
                                                       *const ::libc::c_char,
                                                   arg4: *mut ::libc::c_void)
                                                  -> ::libc::c_int>,
    pub buffer_signature: ::std::option::Option<unsafe extern "C" fn
                                                    (arg1:
                                                         *mut *mut ::libc::c_void,
                                                     arg2:
                                                         *const git_diff_file,
                                                     arg3:
                                                         *const ::libc::c_char,
                                                     arg4: size_t,
                                                     arg5:
                                                         *mut ::libc::c_void)
                                                    -> ::libc::c_int>,
    pub free_signature: ::std::option::Option<unsafe extern "C" fn
                                                  (arg1: *mut ::libc::c_void,
                                                   arg2:
                                                       *mut ::libc::c_void)>,
    pub similarity: ::std::option::Option<unsafe extern "C" fn
                                              (arg1: *mut ::libc::c_int,
                                               arg2: *mut ::libc::c_void,
                                               arg3: *mut ::libc::c_void,
                                               arg4: *mut ::libc::c_void)
                                              -> ::libc::c_int>,
    pub payload: *mut ::libc::c_void,
}
pub type git_diff_similarity_metric = Struct_Unnamed22;
#[repr(C)]
pub struct Struct_Unnamed23 {
    pub version: ::libc::c_uint,
    pub flags: u32,
    pub rename_threshold: u16,
    pub rename_from_rewrite_threshold: u16,
    pub copy_threshold: u16,
    pub break_rewrite_threshold: u16,
    pub rename_limit: size_t,
    pub metric: *mut git_diff_similarity_metric,
}
pub type git_diff_find_options = Struct_Unnamed23;
pub type Enum_Unnamed24 = ::libc::c_uint;
pub static GIT_DIFF_FORMAT_PATCH: ::libc::c_uint = 1;
pub static GIT_DIFF_FORMAT_PATCH_HEADER: ::libc::c_uint = 2;
pub static GIT_DIFF_FORMAT_RAW: ::libc::c_uint = 3;
pub static GIT_DIFF_FORMAT_NAME_ONLY: ::libc::c_uint = 4;
pub static GIT_DIFF_FORMAT_NAME_STATUS: ::libc::c_uint = 5;
pub type git_diff_format_t = Enum_Unnamed24;
pub type Enum_Unnamed25 = ::libc::c_uint;
pub static GIT_CHECKOUT_NONE: ::libc::c_uint = 0;
pub static GIT_CHECKOUT_SAFE: ::libc::c_uint = 1;
pub static GIT_CHECKOUT_SAFE_CREATE: ::libc::c_uint = 2;
pub static GIT_CHECKOUT_FORCE: ::libc::c_uint = 4;
pub static GIT_CHECKOUT_ALLOW_CONFLICTS: ::libc::c_uint = 16;
pub static GIT_CHECKOUT_REMOVE_UNTRACKED: ::libc::c_uint = 32;
pub static GIT_CHECKOUT_REMOVE_IGNORED: ::libc::c_uint = 64;
pub static GIT_CHECKOUT_UPDATE_ONLY: ::libc::c_uint = 128;
pub static GIT_CHECKOUT_DONT_UPDATE_INDEX: ::libc::c_uint = 256;
pub static GIT_CHECKOUT_NO_REFRESH: ::libc::c_uint = 512;
pub static GIT_CHECKOUT_SKIP_UNMERGED: ::libc::c_uint = 1024;
pub static GIT_CHECKOUT_USE_OURS: ::libc::c_uint = 2048;
pub static GIT_CHECKOUT_USE_THEIRS: ::libc::c_uint = 4096;
pub static GIT_CHECKOUT_DISABLE_PATHSPEC_MATCH: ::libc::c_uint = 8192;
pub static GIT_CHECKOUT_SKIP_LOCKED_DIRECTORIES: ::libc::c_uint = 262144;
pub static GIT_CHECKOUT_UPDATE_SUBMODULES: ::libc::c_uint = 65536;
pub static GIT_CHECKOUT_UPDATE_SUBMODULES_IF_CHANGED: ::libc::c_uint = 131072;
pub type git_checkout_strategy_t = Enum_Unnamed25;
pub type Enum_Unnamed26 = ::libc::c_uint;
pub static GIT_CHECKOUT_NOTIFY_NONE: ::libc::c_uint = 0;
pub static GIT_CHECKOUT_NOTIFY_CONFLICT: ::libc::c_uint = 1;
pub static GIT_CHECKOUT_NOTIFY_DIRTY: ::libc::c_uint = 2;
pub static GIT_CHECKOUT_NOTIFY_UPDATED: ::libc::c_uint = 4;
pub static GIT_CHECKOUT_NOTIFY_UNTRACKED: ::libc::c_uint = 8;
pub static GIT_CHECKOUT_NOTIFY_IGNORED: ::libc::c_uint = 16;
pub static GIT_CHECKOUT_NOTIFY_ALL: ::libc::c_uint = 65535;
pub type git_checkout_notify_t = Enum_Unnamed26;
pub type git_checkout_notify_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: git_checkout_notify_t,
                               arg2: *const ::libc::c_char,
                               arg3: *const git_diff_file,
                               arg4: *const git_diff_file,
                               arg5: *const git_diff_file,
                               arg6: *mut ::libc::c_void) -> ::libc::c_int>;
pub type git_checkout_progress_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const ::libc::c_char, arg2: size_t,
                               arg3: size_t, arg4: *mut ::libc::c_void)>;
#[repr(C)]
pub struct Struct_git_checkout_opts {
    pub version: ::libc::c_uint,
    pub checkout_strategy: ::libc::c_uint,
    pub disable_filters: ::libc::c_int,
    pub dir_mode: ::libc::c_uint,
    pub file_mode: ::libc::c_uint,
    pub file_open_flags: ::libc::c_int,
    pub notify_flags: ::libc::c_uint,
    pub notify_cb: git_checkout_notify_cb,
    pub notify_payload: *mut ::libc::c_void,
    pub progress_cb: git_checkout_progress_cb,
    pub progress_payload: *mut ::libc::c_void,
    pub paths: git_strarray,
    pub baseline: *mut git_tree,
    pub target_directory: *const ::libc::c_char,
    pub our_label: *const ::libc::c_char,
    pub their_label: *const ::libc::c_char,
}
pub type git_checkout_opts = Struct_git_checkout_opts;
pub enum Struct_git_indexer { }
pub type git_indexer = Struct_git_indexer;
pub type Enum_Unnamed27 = ::libc::c_uint;
pub static GIT_REPOSITORY_OPEN_NO_SEARCH: ::libc::c_uint = 1;
pub static GIT_REPOSITORY_OPEN_CROSS_FS: ::libc::c_uint = 2;
pub static GIT_REPOSITORY_OPEN_BARE: ::libc::c_uint = 4;
pub type git_repository_open_flag_t = Enum_Unnamed27;
pub type Enum_Unnamed28 = ::libc::c_uint;
pub static GIT_REPOSITORY_INIT_BARE: ::libc::c_uint = 1;
pub static GIT_REPOSITORY_INIT_NO_REINIT: ::libc::c_uint = 2;
pub static GIT_REPOSITORY_INIT_NO_DOTGIT_DIR: ::libc::c_uint = 4;
pub static GIT_REPOSITORY_INIT_MKDIR: ::libc::c_uint = 8;
pub static GIT_REPOSITORY_INIT_MKPATH: ::libc::c_uint = 16;
pub static GIT_REPOSITORY_INIT_EXTERNAL_TEMPLATE: ::libc::c_uint = 32;
pub type git_repository_init_flag_t = Enum_Unnamed28;
pub type Enum_Unnamed29 = ::libc::c_uint;
pub static GIT_REPOSITORY_INIT_SHARED_UMASK: ::libc::c_uint = 0;
pub static GIT_REPOSITORY_INIT_SHARED_GROUP: ::libc::c_uint = 1533;
pub static GIT_REPOSITORY_INIT_SHARED_ALL: ::libc::c_uint = 1535;
pub type git_repository_init_mode_t = Enum_Unnamed29;
#[repr(C)]
pub struct Struct_Unnamed30 {
    pub version: ::libc::c_uint,
    pub flags: u32,
    pub mode: u32,
    pub workdir_path: *const ::libc::c_char,
    pub description: *const ::libc::c_char,
    pub template_path: *const ::libc::c_char,
    pub initial_head: *const ::libc::c_char,
    pub origin_url: *const ::libc::c_char,
}
pub type git_repository_init_options = Struct_Unnamed30;
pub type git_repository_fetchhead_foreach_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const ::libc::c_char,
                               arg2: *const ::libc::c_char,
                               arg3: *const git_oid, arg4: ::libc::c_uint,
                               arg5: *mut ::libc::c_void) -> ::libc::c_int>;
pub type git_repository_mergehead_foreach_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const git_oid,
                               arg2: *mut ::libc::c_void) -> ::libc::c_int>;
pub type Enum_Unnamed31 = ::libc::c_uint;
pub static GIT_REPOSITORY_STATE_NONE: ::libc::c_uint = 0;
pub static GIT_REPOSITORY_STATE_MERGE: ::libc::c_uint = 1;
pub static GIT_REPOSITORY_STATE_REVERT: ::libc::c_uint = 2;
pub static GIT_REPOSITORY_STATE_CHERRY_PICK: ::libc::c_uint = 3;
pub static GIT_REPOSITORY_STATE_BISECT: ::libc::c_uint = 4;
pub static GIT_REPOSITORY_STATE_REBASE: ::libc::c_uint = 5;
pub static GIT_REPOSITORY_STATE_REBASE_INTERACTIVE: ::libc::c_uint = 6;
pub static GIT_REPOSITORY_STATE_REBASE_MERGE: ::libc::c_uint = 7;
pub static GIT_REPOSITORY_STATE_APPLY_MAILBOX: ::libc::c_uint = 8;
pub static GIT_REPOSITORY_STATE_APPLY_MAILBOX_OR_REBASE: ::libc::c_uint = 9;
pub type git_repository_state_t = Enum_Unnamed31;
pub type Enum_Unnamed32 = ::libc::c_uint;
pub static GIT_DIRECTION_FETCH: ::libc::c_uint = 0;
pub static GIT_DIRECTION_PUSH: ::libc::c_uint = 1;
pub type git_direction = Enum_Unnamed32;
#[repr(C)]
pub struct Struct_git_remote_head {
    pub local: ::libc::c_int,
    pub oid: git_oid,
    pub loid: git_oid,
    pub name: *mut ::libc::c_char,
}
pub type git_headlist_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *mut git_remote_head,
                               arg2: *mut ::libc::c_void) -> ::libc::c_int>;
pub type Enum_Unnamed33 = ::libc::c_uint;
pub static GIT_CREDTYPE_USERPASS_PLAINTEXT: ::libc::c_uint = 1;
pub static GIT_CREDTYPE_SSH_KEY: ::libc::c_uint = 2;
pub static GIT_CREDTYPE_SSH_CUSTOM: ::libc::c_uint = 4;
pub static GIT_CREDTYPE_DEFAULT: ::libc::c_uint = 8;
pub type git_credtype_t = Enum_Unnamed33;
pub type git_cred = Struct_git_cred;
#[repr(C)]
pub struct Struct_git_cred {
    pub credtype: git_credtype_t,
    pub free: ::std::option::Option<unsafe extern "C" fn(arg1: *mut git_cred)>,
}
#[repr(C)]
pub struct Struct_Unnamed34 {
    pub parent: git_cred,
    pub username: *mut ::libc::c_char,
    pub password: *mut ::libc::c_char,
}
pub type git_cred_userpass_plaintext = Struct_Unnamed34;
pub type git_cred_sign_callback =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::libc::c_void, ...)
                              -> ::libc::c_int>;
#[repr(C)]
pub struct Struct_git_cred_ssh_key {
    pub parent: git_cred,
    pub username: *mut ::libc::c_char,
    pub publickey: *mut ::libc::c_char,
    pub privatekey: *mut ::libc::c_char,
    pub passphrase: *mut ::libc::c_char,
}
pub type git_cred_ssh_key = Struct_git_cred_ssh_key;
#[repr(C)]
pub struct Struct_git_cred_ssh_custom {
    pub parent: git_cred,
    pub username: *mut ::libc::c_char,
    pub publickey: *mut ::libc::c_char,
    pub publickey_len: size_t,
    pub sign_callback: *mut ::libc::c_void,
    pub sign_data: *mut ::libc::c_void,
}
pub type git_cred_ssh_custom = Struct_git_cred_ssh_custom;
pub type git_cred_default = Struct_git_cred;
pub type git_cred_acquire_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *mut *mut git_cred,
                               arg2: *const ::libc::c_char,
                               arg3: *const ::libc::c_char,
                               arg4: ::libc::c_uint,
                               arg5: *mut ::libc::c_void) -> ::libc::c_int>;
pub type Enum_Unnamed35 = ::libc::c_uint;
pub static GIT_TRANSPORTFLAGS_NONE: ::libc::c_uint = 0;
pub static GIT_TRANSPORTFLAGS_NO_CHECK_CERT: ::libc::c_uint = 1;
pub type git_transport_flags_t = Enum_Unnamed35;
pub type git_transport_message_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const ::libc::c_char,
                               arg2: ::libc::c_int, arg3: *mut ::libc::c_void)
                              -> ::libc::c_int>;
pub type git_transport = Struct_git_transport;
#[repr(C)]
pub struct Struct_git_transport {
    pub version: ::libc::c_uint,
    pub set_callbacks: ::std::option::Option<unsafe extern "C" fn
                                                 (arg1: *mut git_transport,
                                                  arg2:
                                                      git_transport_message_cb,
                                                  arg3:
                                                      git_transport_message_cb,
                                                  arg4: *mut ::libc::c_void)
                                                 -> ::libc::c_int>,
    pub connect: ::std::option::Option<unsafe extern "C" fn
                                           (arg1: *mut git_transport,
                                            arg2: *const ::libc::c_char,
                                            arg3: git_cred_acquire_cb,
                                            arg4: *mut ::libc::c_void,
                                            arg5: ::libc::c_int,
                                            arg6: ::libc::c_int)
                                           -> ::libc::c_int>,
    pub ls: ::std::option::Option<unsafe extern "C" fn
                                      (arg1: *mut *mut *const git_remote_head,
                                       arg2: *mut size_t,
                                       arg3: *mut git_transport)
                                      -> ::libc::c_int>,
    pub push: ::std::option::Option<unsafe extern "C" fn
                                        (arg1: *mut git_transport,
                                         arg2: *mut git_push)
                                        -> ::libc::c_int>,
    pub negotiate_fetch: ::std::option::Option<unsafe extern "C" fn
                                                   (arg1: *mut git_transport,
                                                    arg2: *mut git_repository,
                                                    arg3:
                                                        *const *const git_remote_head,
                                                    arg4: size_t)
                                                   -> ::libc::c_int>,
    pub download_pack: ::std::option::Option<unsafe extern "C" fn
                                                 (arg1: *mut git_transport,
                                                  arg2: *mut git_repository,
                                                  arg3:
                                                      *mut git_transfer_progress,
                                                  arg4:
                                                      git_transfer_progress_callback,
                                                  arg5: *mut ::libc::c_void)
                                                 -> ::libc::c_int>,
    pub is_connected: ::std::option::Option<unsafe extern "C" fn
                                                (arg1: *mut git_transport)
                                                -> ::libc::c_int>,
    pub read_flags: ::std::option::Option<unsafe extern "C" fn
                                              (arg1: *mut git_transport,
                                               arg2: *mut ::libc::c_int)
                                              -> ::libc::c_int>,
    pub cancel: ::std::option::Option<unsafe extern "C" fn
                                          (arg1: *mut git_transport)>,
    pub close: ::std::option::Option<unsafe extern "C" fn(arg1: *mut git_transport)
                                         -> ::libc::c_int>,
    pub free: ::std::option::Option<unsafe extern "C" fn(arg1: *mut git_transport)>,
}
pub type git_transport_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *mut *mut git_transport,
                               arg2: *mut git_remote,
                               arg3: *mut ::libc::c_void) -> ::libc::c_int>;
pub type Enum_Unnamed36 = ::libc::c_uint;
pub static GIT_SERVICE_UPLOADPACK_LS: ::libc::c_uint = 1;
pub static GIT_SERVICE_UPLOADPACK: ::libc::c_uint = 2;
pub static GIT_SERVICE_RECEIVEPACK_LS: ::libc::c_uint = 3;
pub static GIT_SERVICE_RECEIVEPACK: ::libc::c_uint = 4;
pub type git_smart_service_t = Enum_Unnamed36;
pub type git_smart_subtransport = Struct_git_smart_subtransport;
pub type git_smart_subtransport_stream = Struct_git_smart_subtransport_stream;
#[repr(C)]
pub struct Struct_git_smart_subtransport_stream {
    pub subtransport: *mut git_smart_subtransport,
    pub read: ::std::option::Option<unsafe extern "C" fn
                                        (arg1:
                                             *mut git_smart_subtransport_stream,
                                         arg2: *mut ::libc::c_char,
                                         arg3: size_t, arg4: *mut size_t)
                                        -> ::libc::c_int>,
    pub write: ::std::option::Option<unsafe extern "C" fn
                                         (arg1:
                                              *mut git_smart_subtransport_stream,
                                          arg2: *const ::libc::c_char,
                                          arg3: size_t) -> ::libc::c_int>,
    pub free: ::std::option::Option<unsafe extern "C" fn
                                        (arg1:
                                             *mut git_smart_subtransport_stream)>,
}
#[repr(C)]
pub struct Struct_git_smart_subtransport {
    pub action: ::std::option::Option<unsafe extern "C" fn
                                          (arg1:
                                               *mut *mut git_smart_subtransport_stream,
                                           arg2: *mut git_smart_subtransport,
                                           arg3: *const ::libc::c_char,
                                           arg4: git_smart_service_t)
                                          -> ::libc::c_int>,
    pub close: ::std::option::Option<unsafe extern "C" fn
                                         (arg1: *mut git_smart_subtransport)
                                         -> ::libc::c_int>,
    pub free: ::std::option::Option<unsafe extern "C" fn
                                        (arg1: *mut git_smart_subtransport)>,
}
pub type git_smart_subtransport_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *mut *mut git_smart_subtransport,
                               arg2: *mut git_transport) -> ::libc::c_int>;
#[repr(C)]
pub struct Struct_git_smart_subtransport_definition {
    pub callback: git_smart_subtransport_cb,
    pub rpc: ::libc::c_uint,
}
pub type git_smart_subtransport_definition =
    Struct_git_smart_subtransport_definition;
pub type git_remote_rename_problem_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const ::libc::c_char,
                               arg2: *mut ::libc::c_void) -> ::libc::c_int>;
pub type Enum_git_remote_completion_type = ::libc::c_uint;
pub static GIT_REMOTE_COMPLETION_DOWNLOAD: ::libc::c_uint = 0;
pub static GIT_REMOTE_COMPLETION_INDEXING: ::libc::c_uint = 1;
pub static GIT_REMOTE_COMPLETION_ERROR: ::libc::c_uint = 2;
pub type git_remote_completion_type = Enum_git_remote_completion_type;
#[repr(C)]
pub struct Struct_git_remote_callbacks {
    pub version: ::libc::c_uint,
    pub progress: ::std::option::Option<unsafe extern "C" fn
                                            (arg1: *const ::libc::c_char,
                                             arg2: ::libc::c_int,
                                             arg3: *mut ::libc::c_void)
                                            -> ::libc::c_int>,
    pub completion: ::std::option::Option<unsafe extern "C" fn
                                              (arg1:
                                                   git_remote_completion_type,
                                               arg2: *mut ::libc::c_void)
                                              -> ::libc::c_int>,
    pub credentials: ::std::option::Option<unsafe extern "C" fn
                                               (arg1: *mut *mut git_cred,
                                                arg2: *const ::libc::c_char,
                                                arg3: *const ::libc::c_char,
                                                arg4: ::libc::c_uint,
                                                arg5: *mut ::libc::c_void)
                                               -> ::libc::c_int>,
    pub transfer_progress: ::std::option::Option<unsafe extern "C" fn
                                                     (arg1:
                                                          *const git_transfer_progress,
                                                      arg2:
                                                          *mut ::libc::c_void)
                                                     -> ::libc::c_int>,
    pub update_tips: ::std::option::Option<unsafe extern "C" fn
                                               (arg1: *const ::libc::c_char,
                                                arg2: *const git_oid,
                                                arg3: *const git_oid,
                                                arg4: *mut ::libc::c_void)
                                               -> ::libc::c_int>,
    pub payload: *mut ::libc::c_void,
}
pub type Enum_Unnamed37 = ::libc::c_uint;
pub static GIT_REMOTE_DOWNLOAD_TAGS_AUTO: ::libc::c_uint = 0;
pub static GIT_REMOTE_DOWNLOAD_TAGS_NONE: ::libc::c_uint = 1;
pub static GIT_REMOTE_DOWNLOAD_TAGS_ALL: ::libc::c_uint = 2;
pub type git_remote_autotag_option_t = Enum_Unnamed37;
#[repr(C)]
pub struct Struct_git_clone_options {
    pub version: ::libc::c_uint,
    pub checkout_opts: git_checkout_opts,
    pub remote_callbacks: git_remote_callbacks,
    pub bare: ::libc::c_int,
    pub ignore_cert_errors: ::libc::c_int,
    pub remote_name: *const ::libc::c_char,
    pub checkout_branch: *const ::libc::c_char,
}
pub type git_clone_options = Struct_git_clone_options;
pub type Enum_Unnamed38 = ::libc::c_int;
pub static GIT_CONFIG_LEVEL_SYSTEM: ::libc::c_int = 1;
pub static GIT_CONFIG_LEVEL_XDG: ::libc::c_int = 2;
pub static GIT_CONFIG_LEVEL_GLOBAL: ::libc::c_int = 3;
pub static GIT_CONFIG_LEVEL_LOCAL: ::libc::c_int = 4;
pub static GIT_CONFIG_LEVEL_APP: ::libc::c_int = 5;
pub static GIT_CONFIG_HIGHEST_LEVEL: ::libc::c_int = -1;
pub type git_config_level_t = Enum_Unnamed38;
#[repr(C)]
pub struct Struct_Unnamed39 {
    pub name: *const ::libc::c_char,
    pub value: *const ::libc::c_char,
    pub level: git_config_level_t,
}
pub type git_config_entry = Struct_Unnamed39;
pub type git_config_foreach_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const git_config_entry,
                               arg2: *mut ::libc::c_void) -> ::libc::c_int>;
pub enum Struct_git_config_iterator { }
pub type git_config_iterator = Struct_git_config_iterator;
pub type Enum_Unnamed40 = ::libc::c_uint;
pub static GIT_CVAR_FALSE: ::libc::c_uint = 0;
pub static GIT_CVAR_TRUE: ::libc::c_uint = 1;
pub static GIT_CVAR_INT32: ::libc::c_uint = 2;
pub static GIT_CVAR_STRING: ::libc::c_uint = 3;
pub type git_cvar_t = Enum_Unnamed40;
#[repr(C)]
pub struct Struct_Unnamed41 {
    pub cvar_type: git_cvar_t,
    pub str_match: *const ::libc::c_char,
    pub map_value: ::libc::c_int,
}
pub type git_cvar_map = Struct_Unnamed41;
pub type Enum_Unnamed42 = ::libc::c_int;
pub static GIT_OK: ::libc::c_int = 0;
pub static GIT_ERROR: ::libc::c_int = -1;
pub static GIT_ENOTFOUND: ::libc::c_int = -3;
pub static GIT_EEXISTS: ::libc::c_int = -4;
pub static GIT_EAMBIGUOUS: ::libc::c_int = -5;
pub static GIT_EBUFS: ::libc::c_int = -6;
pub static GIT_EUSER: ::libc::c_int = -7;
pub static GIT_EBAREREPO: ::libc::c_int = -8;
pub static GIT_EUNBORNBRANCH: ::libc::c_int = -9;
pub static GIT_EUNMERGED: ::libc::c_int = -10;
pub static GIT_ENONFASTFORWARD: ::libc::c_int = -11;
pub static GIT_EINVALIDSPEC: ::libc::c_int = -12;
pub static GIT_EMERGECONFLICT: ::libc::c_int = -13;
pub static GIT_ELOCKED: ::libc::c_int = -14;
pub static GIT_PASSTHROUGH: ::libc::c_int = -30;
pub static GIT_ITEROVER: ::libc::c_int = -31;
pub type git_error_code = Enum_Unnamed42;
#[repr(C)]
pub struct Struct_Unnamed43 {
    pub message: *mut ::libc::c_char,
    pub klass: ::libc::c_int,
}
pub type git_error = Struct_Unnamed43;
pub type Enum_Unnamed44 = ::libc::c_uint;
pub static GITERR_NONE: ::libc::c_uint = 0;
pub static GITERR_NOMEMORY: ::libc::c_uint = 1;
pub static GITERR_OS: ::libc::c_uint = 2;
pub static GITERR_INVALID: ::libc::c_uint = 3;
pub static GITERR_REFERENCE: ::libc::c_uint = 4;
pub static GITERR_ZLIB: ::libc::c_uint = 5;
pub static GITERR_REPOSITORY: ::libc::c_uint = 6;
pub static GITERR_CONFIG: ::libc::c_uint = 7;
pub static GITERR_REGEX: ::libc::c_uint = 8;
pub static GITERR_ODB: ::libc::c_uint = 9;
pub static GITERR_INDEX: ::libc::c_uint = 10;
pub static GITERR_OBJECT: ::libc::c_uint = 11;
pub static GITERR_NET: ::libc::c_uint = 12;
pub static GITERR_TAG: ::libc::c_uint = 13;
pub static GITERR_TREE: ::libc::c_uint = 14;
pub static GITERR_INDEXER: ::libc::c_uint = 15;
pub static GITERR_SSL: ::libc::c_uint = 16;
pub static GITERR_SUBMODULE: ::libc::c_uint = 17;
pub static GITERR_THREAD: ::libc::c_uint = 18;
pub static GITERR_STASH: ::libc::c_uint = 19;
pub static GITERR_CHECKOUT: ::libc::c_uint = 20;
pub static GITERR_FETCHHEAD: ::libc::c_uint = 21;
pub static GITERR_MERGE: ::libc::c_uint = 22;
pub static GITERR_SSH: ::libc::c_uint = 23;
pub static GITERR_FILTER: ::libc::c_uint = 24;
pub type git_error_t = Enum_Unnamed44;
pub type Enum_Unnamed45 = ::libc::c_uint;
pub static GIT_FILTER_TO_WORKTREE: ::libc::c_uint = 0;
pub static GIT_FILTER_SMUDGE: ::libc::c_uint = 0;
pub static GIT_FILTER_TO_ODB: ::libc::c_uint = 1;
pub static GIT_FILTER_CLEAN: ::libc::c_uint = 1;
pub type git_filter_mode_t = Enum_Unnamed45;
pub enum Struct_git_filter { }
pub type git_filter = Struct_git_filter;
pub enum Struct_git_filter_list { }
pub type git_filter_list = Struct_git_filter_list;
#[repr(C)]
pub struct Struct_Unnamed46 {
    pub seconds: git_time_t,
    pub nanoseconds: ::libc::c_uint,
}
pub type git_index_time = Struct_Unnamed46;
#[repr(C)]
pub struct Struct_git_index_entry {
    pub ctime: git_index_time,
    pub mtime: git_index_time,
    pub dev: ::libc::c_uint,
    pub ino: ::libc::c_uint,
    pub mode: ::libc::c_uint,
    pub uid: ::libc::c_uint,
    pub gid: ::libc::c_uint,
    pub file_size: git_off_t,
    pub oid: git_oid,
    pub flags: ::libc::c_ushort,
    pub flags_extended: ::libc::c_ushort,
    pub path: *mut ::libc::c_char,
}
pub type git_index_entry = Struct_git_index_entry;
pub type Enum_Unnamed47 = ::libc::c_uint;
pub static GIT_INDEXCAP_IGNORE_CASE: ::libc::c_uint = 1;
pub static GIT_INDEXCAP_NO_FILEMODE: ::libc::c_uint = 2;
pub static GIT_INDEXCAP_NO_SYMLINKS: ::libc::c_uint = 4;
pub static GIT_INDEXCAP_FROM_OWNER: ::libc::c_uint = -1;
pub type git_indexcap_t = Enum_Unnamed47;
pub type git_index_matched_path_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const ::libc::c_char,
                               arg2: *const ::libc::c_char,
                               arg3: *mut ::libc::c_void) -> ::libc::c_int>;
pub type Enum_Unnamed48 = ::libc::c_uint;
pub static GIT_INDEX_ADD_DEFAULT: ::libc::c_uint = 0;
pub static GIT_INDEX_ADD_FORCE: ::libc::c_uint = 1;
pub static GIT_INDEX_ADD_DISABLE_PATHSPEC_MATCH: ::libc::c_uint = 2;
pub static GIT_INDEX_ADD_CHECK_PATHSPEC: ::libc::c_uint = 4;
pub type git_index_add_option_t = Enum_Unnamed48;
pub type Enum_Unnamed49 = ::libc::c_uint;
pub static GIT_MERGE_TREE_FIND_RENAMES: ::libc::c_uint = 1;
pub type git_merge_tree_flag_t = Enum_Unnamed49;
pub type Enum_Unnamed50 = ::libc::c_uint;
pub static GIT_MERGE_AUTOMERGE_NORMAL: ::libc::c_uint = 0;
pub static GIT_MERGE_AUTOMERGE_NONE: ::libc::c_uint = 1;
pub static GIT_MERGE_AUTOMERGE_FAVOR_OURS: ::libc::c_uint = 2;
pub static GIT_MERGE_AUTOMERGE_FAVOR_THEIRS: ::libc::c_uint = 3;
pub type git_merge_automerge_flags = Enum_Unnamed50;
#[repr(C)]
pub struct Struct_Unnamed51 {
    pub version: ::libc::c_uint,
    pub flags: git_merge_tree_flag_t,
    pub rename_threshold: ::libc::c_uint,
    pub target_limit: ::libc::c_uint,
    pub metric: *mut git_diff_similarity_metric,
    pub automerge_flags: git_merge_automerge_flags,
}
pub type git_merge_tree_opts = Struct_Unnamed51;
pub type Enum_Unnamed52 = ::libc::c_uint;
pub static GIT_MERGE_NO_FASTFORWARD: ::libc::c_uint = 1;
pub static GIT_MERGE_FASTFORWARD_ONLY: ::libc::c_uint = 2;
pub type git_merge_flags_t = Enum_Unnamed52;
#[repr(C)]
pub struct Struct_Unnamed53 {
    pub version: ::libc::c_uint,
    pub merge_flags: git_merge_flags_t,
    pub merge_tree_opts: git_merge_tree_opts,
    pub checkout_opts: git_checkout_opts,
}
pub type git_merge_opts = Struct_Unnamed53;
pub type git_note_foreach_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const git_oid, arg2: *const git_oid,
                               arg3: *mut ::libc::c_void) -> ::libc::c_int>;
pub enum Struct_git_iterator { }
pub type git_note_iterator = Struct_git_iterator;
pub type git_odb_foreach_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const git_oid,
                               arg2: *mut ::libc::c_void) -> ::libc::c_int>;
pub type Enum_Unnamed54 = ::libc::c_uint;
pub static GIT_PACKBUILDER_ADDING_OBJECTS: ::libc::c_uint = 0;
pub static GIT_PACKBUILDER_DELTAFICATION: ::libc::c_uint = 1;
pub type git_packbuilder_stage_t = Enum_Unnamed54;
pub type git_packbuilder_foreach_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *mut ::libc::c_void, arg2: size_t,
                               arg3: *mut ::libc::c_void) -> ::libc::c_int>;
pub type git_packbuilder_progress =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: ::libc::c_int, arg2: ::libc::c_uint,
                               arg3: ::libc::c_uint,
                               arg4: *mut ::libc::c_void) -> ::libc::c_int>;
pub enum Struct_git_patch { }
pub type git_patch = Struct_git_patch;
pub enum Struct_git_pathspec { }
pub type git_pathspec = Struct_git_pathspec;
pub enum Struct_git_pathspec_match_list { }
pub type git_pathspec_match_list = Struct_git_pathspec_match_list;
pub type Enum_Unnamed55 = ::libc::c_uint;
pub static GIT_PATHSPEC_DEFAULT: ::libc::c_uint = 0;
pub static GIT_PATHSPEC_IGNORE_CASE: ::libc::c_uint = 1;
pub static GIT_PATHSPEC_USE_CASE: ::libc::c_uint = 2;
pub static GIT_PATHSPEC_NO_GLOB: ::libc::c_uint = 4;
pub static GIT_PATHSPEC_NO_MATCH_ERROR: ::libc::c_uint = 8;
pub static GIT_PATHSPEC_FIND_FAILURES: ::libc::c_uint = 16;
pub static GIT_PATHSPEC_FAILURES_ONLY: ::libc::c_uint = 32;
pub type git_pathspec_flag_t = Enum_Unnamed55;
#[repr(C)]
pub struct Struct_Unnamed56 {
    pub version: ::libc::c_uint,
    pub pb_parallelism: ::libc::c_uint,
}
pub type git_push_options = Struct_Unnamed56;
pub type git_push_transfer_progress =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: ::libc::c_uint, arg2: ::libc::c_uint,
                               arg3: size_t, arg4: *mut ::libc::c_void)
                              -> ::libc::c_int>;
pub type Enum_Unnamed57 = ::libc::c_uint;
pub static GIT_RESET_SOFT: ::libc::c_uint = 1;
pub static GIT_RESET_MIXED: ::libc::c_uint = 2;
pub static GIT_RESET_HARD: ::libc::c_uint = 3;
pub type git_reset_t = Enum_Unnamed57;
pub type Enum_Unnamed58 = ::libc::c_uint;
pub static GIT_REVPARSE_SINGLE: ::libc::c_uint = 1;
pub static GIT_REVPARSE_RANGE: ::libc::c_uint = 2;
pub static GIT_REVPARSE_MERGE_BASE: ::libc::c_uint = 4;
pub type git_revparse_mode_t = Enum_Unnamed58;
#[repr(C)]
pub struct Struct_Unnamed59 {
    pub from: *mut git_object,
    pub to: *mut git_object,
    pub flags: ::libc::c_uint,
}
pub type git_revspec = Struct_Unnamed59;
pub type Enum_Unnamed60 = ::libc::c_uint;
pub static GIT_STASH_DEFAULT: ::libc::c_uint = 0;
pub static GIT_STASH_KEEP_INDEX: ::libc::c_uint = 1;
pub static GIT_STASH_INCLUDE_UNTRACKED: ::libc::c_uint = 2;
pub static GIT_STASH_INCLUDE_IGNORED: ::libc::c_uint = 4;
pub type git_stash_flags = Enum_Unnamed60;
pub type git_stash_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: size_t, arg2: *const ::libc::c_char,
                               arg3: *const git_oid,
                               arg4: *mut ::libc::c_void) -> ::libc::c_int>;
pub type Enum_Unnamed61 = ::libc::c_uint;
pub static GIT_STATUS_CURRENT: ::libc::c_uint = 0;
pub static GIT_STATUS_INDEX_NEW: ::libc::c_uint = 1;
pub static GIT_STATUS_INDEX_MODIFIED: ::libc::c_uint = 2;
pub static GIT_STATUS_INDEX_DELETED: ::libc::c_uint = 4;
pub static GIT_STATUS_INDEX_RENAMED: ::libc::c_uint = 8;
pub static GIT_STATUS_INDEX_TYPECHANGE: ::libc::c_uint = 16;
pub static GIT_STATUS_WT_NEW: ::libc::c_uint = 128;
pub static GIT_STATUS_WT_MODIFIED: ::libc::c_uint = 256;
pub static GIT_STATUS_WT_DELETED: ::libc::c_uint = 512;
pub static GIT_STATUS_WT_TYPECHANGE: ::libc::c_uint = 1024;
pub static GIT_STATUS_WT_RENAMED: ::libc::c_uint = 2048;
pub static GIT_STATUS_IGNORED: ::libc::c_uint = 16384;
pub type git_status_t = Enum_Unnamed61;
pub type git_status_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const ::libc::c_char,
                               arg2: ::libc::c_uint,
                               arg3: *mut ::libc::c_void) -> ::libc::c_int>;
pub type Enum_Unnamed62 = ::libc::c_uint;
pub static GIT_STATUS_SHOW_INDEX_AND_WORKDIR: ::libc::c_uint = 0;
pub static GIT_STATUS_SHOW_INDEX_ONLY: ::libc::c_uint = 1;
pub static GIT_STATUS_SHOW_WORKDIR_ONLY: ::libc::c_uint = 2;
pub type git_status_show_t = Enum_Unnamed62;
pub type Enum_Unnamed63 = ::libc::c_uint;
pub static GIT_STATUS_OPT_INCLUDE_UNTRACKED: ::libc::c_uint = 1;
pub static GIT_STATUS_OPT_INCLUDE_IGNORED: ::libc::c_uint = 2;
pub static GIT_STATUS_OPT_INCLUDE_UNMODIFIED: ::libc::c_uint = 4;
pub static GIT_STATUS_OPT_EXCLUDE_SUBMODULES: ::libc::c_uint = 8;
pub static GIT_STATUS_OPT_RECURSE_UNTRACKED_DIRS: ::libc::c_uint = 16;
pub static GIT_STATUS_OPT_DISABLE_PATHSPEC_MATCH: ::libc::c_uint = 32;
pub static GIT_STATUS_OPT_RECURSE_IGNORED_DIRS: ::libc::c_uint = 64;
pub static GIT_STATUS_OPT_RENAMES_HEAD_TO_INDEX: ::libc::c_uint = 128;
pub static GIT_STATUS_OPT_RENAMES_INDEX_TO_WORKDIR: ::libc::c_uint = 256;
pub static GIT_STATUS_OPT_SORT_CASE_SENSITIVELY: ::libc::c_uint = 512;
pub static GIT_STATUS_OPT_SORT_CASE_INSENSITIVELY: ::libc::c_uint = 1024;
pub static GIT_STATUS_OPT_RENAMES_FROM_REWRITES: ::libc::c_uint = 2048;
pub static GIT_STATUS_OPT_NO_REFRESH: ::libc::c_uint = 4096;
pub type git_status_opt_t = Enum_Unnamed63;
#[repr(C)]
pub struct Struct_Unnamed64 {
    pub version: ::libc::c_uint,
    pub show: git_status_show_t,
    pub flags: ::libc::c_uint,
    pub pathspec: git_strarray,
}
pub type git_status_options = Struct_Unnamed64;
#[repr(C)]
pub struct Struct_Unnamed65 {
    pub status: git_status_t,
    pub head_to_index: *mut git_diff_delta,
    pub index_to_workdir: *mut git_diff_delta,
}
pub type git_status_entry = Struct_Unnamed65;
pub type Enum_Unnamed66 = ::libc::c_uint;
pub static GIT_SUBMODULE_STATUS_IN_HEAD: ::libc::c_uint = 1;
pub static GIT_SUBMODULE_STATUS_IN_INDEX: ::libc::c_uint = 2;
pub static GIT_SUBMODULE_STATUS_IN_CONFIG: ::libc::c_uint = 4;
pub static GIT_SUBMODULE_STATUS_IN_WD: ::libc::c_uint = 8;
pub static GIT_SUBMODULE_STATUS_INDEX_ADDED: ::libc::c_uint = 16;
pub static GIT_SUBMODULE_STATUS_INDEX_DELETED: ::libc::c_uint = 32;
pub static GIT_SUBMODULE_STATUS_INDEX_MODIFIED: ::libc::c_uint = 64;
pub static GIT_SUBMODULE_STATUS_WD_UNINITIALIZED: ::libc::c_uint = 128;
pub static GIT_SUBMODULE_STATUS_WD_ADDED: ::libc::c_uint = 256;
pub static GIT_SUBMODULE_STATUS_WD_DELETED: ::libc::c_uint = 512;
pub static GIT_SUBMODULE_STATUS_WD_MODIFIED: ::libc::c_uint = 1024;
pub static GIT_SUBMODULE_STATUS_WD_INDEX_MODIFIED: ::libc::c_uint = 2048;
pub static GIT_SUBMODULE_STATUS_WD_WD_MODIFIED: ::libc::c_uint = 4096;
pub static GIT_SUBMODULE_STATUS_WD_UNTRACKED: ::libc::c_uint = 8192;
pub type git_submodule_status_t = Enum_Unnamed66;
pub type git_tag_foreach_cb =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: *const ::libc::c_char,
                               arg2: *mut git_oid, arg3: *mut ::libc::c_void)
                              -> ::libc::c_int>;
#[link(name = "git2")]
extern "C" {
    pub fn git_libgit2_version(major: *mut ::libc::c_int,
                               minor: *mut ::libc::c_int,
                               rev: *mut ::libc::c_int);
    pub fn git_libgit2_capabilities() -> ::libc::c_int;
    pub fn git_libgit2_opts(option: ::libc::c_int, ...) -> ::libc::c_int;
    pub fn git_attr_value(attr: *const ::libc::c_char) -> git_attr_t;
    pub fn git_attr_get(value_out: *mut *const ::libc::c_char,
                        repo: *mut git_repository, flags: u32,
                        path: *const ::libc::c_char,
                        name: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_attr_get_many(values_out: *mut *const ::libc::c_char,
                             repo: *mut git_repository, flags: u32,
                             path: *const ::libc::c_char, num_attr: size_t,
                             names: *mut *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_attr_foreach(repo: *mut git_repository, flags: u32,
                            path: *const ::libc::c_char,
                            callback: git_attr_foreach_cb,
                            payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_attr_cache_flush(repo: *mut git_repository);
    pub fn git_attr_add_macro(repo: *mut git_repository,
                              name: *const ::libc::c_char,
                              values: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_oid_fromstr(out: *mut git_oid, str: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_oid_fromstrp(out: *mut git_oid, str: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_oid_fromstrn(out: *mut git_oid, str: *const ::libc::c_char,
                            length: size_t) -> ::libc::c_int;
    pub fn git_oid_fromraw(out: *mut git_oid, raw: *const ::libc::c_uchar);
    pub fn git_oid_fmt(out: *mut ::libc::c_char, id: *const git_oid);
    pub fn git_oid_nfmt(out: *mut ::libc::c_char, n: size_t,
                        id: *const git_oid);
    pub fn git_oid_pathfmt(out: *mut ::libc::c_char, id: *const git_oid);
    pub fn git_oid_allocfmt(id: *const git_oid) -> *mut ::libc::c_char;
    pub fn git_oid_tostr(out: *mut ::libc::c_char, n: size_t,
                         id: *const git_oid) -> *mut ::libc::c_char;
    pub fn git_oid_cpy(out: *mut git_oid, src: *const git_oid);
    pub fn git_oid_cmp(a: *const git_oid, b: *const git_oid) -> ::libc::c_int;
    pub fn git_oid_ncmp(a: *const git_oid, b: *const git_oid, len: size_t) ->
     ::libc::c_int;
    pub fn git_oid_streq(id: *const git_oid, str: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_oid_strcmp(id: *const git_oid, str: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_oid_iszero(id: *const git_oid) -> ::libc::c_int;
    pub fn git_oid_shorten_new(min_length: size_t) -> *mut git_oid_shorten;
    pub fn git_oid_shorten_add(os: *mut git_oid_shorten,
                               text_id: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_oid_shorten_free(os: *mut git_oid_shorten);
    pub fn git_object_lookup(object: *mut *mut git_object,
                             repo: *mut git_repository, id: *const git_oid,
                             _type: git_otype) -> ::libc::c_int;
    pub fn git_object_lookup_prefix(object_out: *mut *mut git_object,
                                    repo: *mut git_repository,
                                    id: *const git_oid, len: size_t,
                                    _type: git_otype) -> ::libc::c_int;
    pub fn git_object_lookup_bypath(out: *mut *mut git_object,
                                    treeish: *const git_object,
                                    path: *const ::libc::c_char,
                                    _type: git_otype) -> ::libc::c_int;
    pub fn git_object_id(obj: *const git_object) -> *const git_oid;
    pub fn git_object_type(obj: *const git_object) -> git_otype;
    pub fn git_object_owner(obj: *const git_object) -> *mut git_repository;
    pub fn git_object_free(object: *mut git_object);
    pub fn git_object_type2string(_type: git_otype) -> *const ::libc::c_char;
    pub fn git_object_string2type(str: *const ::libc::c_char) -> git_otype;
    pub fn git_object_typeisloose(_type: git_otype) -> ::libc::c_int;
    pub fn git_object__size(_type: git_otype) -> size_t;
    pub fn git_object_peel(peeled: *mut *mut git_object,
                           object: *const git_object, target_type: git_otype)
     -> ::libc::c_int;
    pub fn git_object_dup(dest: *mut *mut git_object, source: *mut git_object)
     -> ::libc::c_int;
    pub fn git_buf_free(buffer: *mut git_buf);
    pub fn git_buf_grow(buffer: *mut git_buf, target_size: size_t) ->
     ::libc::c_int;
    pub fn git_buf_set(buffer: *mut git_buf, data: *const ::libc::c_void,
                       datalen: size_t) -> ::libc::c_int;
    pub fn git_blob_lookup(blob: *mut *mut git_blob,
                           repo: *mut git_repository, id: *const git_oid) ->
     ::libc::c_int;
    pub fn git_blob_lookup_prefix(blob: *mut *mut git_blob,
                                  repo: *mut git_repository,
                                  id: *const git_oid, len: size_t) ->
     ::libc::c_int;
    pub fn git_blob_free(blob: *mut git_blob);
    pub fn git_blob_id(blob: *const git_blob) -> *const git_oid;
    pub fn git_blob_owner(blob: *const git_blob) -> *mut git_repository;
    pub fn git_blob_rawcontent(blob: *const git_blob) ->
     *const ::libc::c_void;
    pub fn git_blob_rawsize(blob: *const git_blob) -> git_off_t;
    pub fn git_blob_filtered_content(out: *mut git_buf, blob: *mut git_blob,
                                     as_path: *const ::libc::c_char,
                                     check_for_binary_data: ::libc::c_int) ->
     ::libc::c_int;
    pub fn git_blob_create_fromworkdir(id: *mut git_oid,
                                       repo: *mut git_repository,
                                       relative_path: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn git_blob_create_fromdisk(id: *mut git_oid,
                                    repo: *mut git_repository,
                                    path: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_blob_create_fromchunks(id: *mut git_oid,
                                      repo: *mut git_repository,
                                      hintpath: *const ::libc::c_char,
                                      callback: git_blob_chunk_cb,
                                      payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_blob_create_frombuffer(oid: *mut git_oid,
                                      repo: *mut git_repository,
                                      buffer: *const ::libc::c_void,
                                      len: size_t) -> ::libc::c_int;
    pub fn git_blob_is_binary(blob: *mut git_blob) -> ::libc::c_int;
    pub fn git_blame_get_hunk_count(blame: *mut git_blame) -> u32;
    pub fn git_blame_get_hunk_byindex(blame: *mut git_blame, index: u32)
     -> *const git_blame_hunk;
    pub fn git_blame_get_hunk_byline(blame: *mut git_blame, lineno: u32)
     -> *const git_blame_hunk;
    pub fn git_blame_file(out: *mut *mut git_blame, repo: *mut git_repository,
                          path: *const ::libc::c_char,
                          options: *mut git_blame_options) -> ::libc::c_int;
    pub fn git_blame_buffer(out: *mut *mut git_blame,
                            reference: *mut git_blame,
                            buffer: *const ::libc::c_char,
                            buffer_len: u32) -> ::libc::c_int;
    pub fn git_blame_free(blame: *mut git_blame);
    pub fn git_branch_create(out: *mut *mut git_reference,
                             repo: *mut git_repository,
                             branch_name: *const ::libc::c_char,
                             target: *const git_commit, force: ::libc::c_int)
     -> ::libc::c_int;
    pub fn git_branch_delete(branch: *mut git_reference) -> ::libc::c_int;
    pub fn git_branch_iterator_new(out: *mut *mut git_branch_iterator,
                                   repo: *mut git_repository,
                                   list_flags: git_branch_t) -> ::libc::c_int;
    pub fn git_branch_next(out: *mut *mut git_reference,
                           out_type: *mut git_branch_t,
                           iter: *mut git_branch_iterator) -> ::libc::c_int;
    pub fn git_branch_iterator_free(iter: *mut git_branch_iterator);
    pub fn git_branch_move(out: *mut *mut git_reference,
                           branch: *mut git_reference,
                           new_branch_name: *const ::libc::c_char,
                           force: ::libc::c_int) -> ::libc::c_int;
    pub fn git_branch_lookup(out: *mut *mut git_reference,
                             repo: *mut git_repository,
                             branch_name: *const ::libc::c_char,
                             branch_type: git_branch_t) -> ::libc::c_int;
    pub fn git_branch_name(out: *mut *const ::libc::c_char,
                           _ref: *mut git_reference) -> ::libc::c_int;
    pub fn git_branch_upstream(out: *mut *mut git_reference,
                               branch: *mut git_reference) -> ::libc::c_int;
    pub fn git_branch_set_upstream(branch: *mut git_reference,
                                   upstream_name: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_branch_upstream_name(tracking_branch_name_out:
                                        *mut ::libc::c_char,
                                    buffer_size: size_t,
                                    repo: *mut git_repository,
                                    canonical_branch_name:
                                        *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_branch_is_head(branch: *mut git_reference) -> ::libc::c_int;
    pub fn git_branch_remote_name(remote_name_out: *mut ::libc::c_char,
                                  buffer_size: size_t,
                                  repo: *mut git_repository,
                                  canonical_branch_name:
                                      *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_tree_lookup(out: *mut *mut git_tree, repo: *mut git_repository,
                           id: *const git_oid) -> ::libc::c_int;
    pub fn git_tree_lookup_prefix(out: *mut *mut git_tree,
                                  repo: *mut git_repository,
                                  id: *const git_oid, len: size_t) ->
     ::libc::c_int;
    pub fn git_tree_free(tree: *mut git_tree);
    pub fn git_tree_id(tree: *const git_tree) -> *const git_oid;
    pub fn git_tree_owner(tree: *const git_tree) -> *mut git_repository;
    pub fn git_tree_entrycount(tree: *const git_tree) -> size_t;
    pub fn git_tree_entry_byname(tree: *const git_tree,
                                 filename: *const ::libc::c_char) ->
     *const git_tree_entry;
    pub fn git_tree_entry_byindex(tree: *const git_tree, idx: size_t) ->
     *const git_tree_entry;
    pub fn git_tree_entry_byoid(tree: *const git_tree, oid: *const git_oid) ->
     *const git_tree_entry;
    pub fn git_tree_entry_bypath(out: *mut *mut git_tree_entry,
                                 root: *const git_tree,
                                 path: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_tree_entry_dup(entry: *const git_tree_entry) ->
     *mut git_tree_entry;
    pub fn git_tree_entry_free(entry: *mut git_tree_entry);
    pub fn git_tree_entry_name(entry: *const git_tree_entry) ->
     *const ::libc::c_char;
    pub fn git_tree_entry_id(entry: *const git_tree_entry) -> *const git_oid;
    pub fn git_tree_entry_type(entry: *const git_tree_entry) -> git_otype;
    pub fn git_tree_entry_filemode(entry: *const git_tree_entry) ->
     git_filemode_t;
    pub fn git_tree_entry_filemode_raw(entry: *const git_tree_entry) ->
     git_filemode_t;
    pub fn git_tree_entry_cmp(e1: *const git_tree_entry,
                              e2: *const git_tree_entry) -> ::libc::c_int;
    pub fn git_tree_entry_to_object(object_out: *mut *mut git_object,
                                    repo: *mut git_repository,
                                    entry: *const git_tree_entry) ->
     ::libc::c_int;
    pub fn git_treebuilder_create(out: *mut *mut git_treebuilder,
                                  source: *const git_tree) -> ::libc::c_int;
    pub fn git_treebuilder_clear(bld: *mut git_treebuilder);
    pub fn git_treebuilder_entrycount(bld: *mut git_treebuilder) ->
     ::libc::c_uint;
    pub fn git_treebuilder_free(bld: *mut git_treebuilder);
    pub fn git_treebuilder_get(bld: *mut git_treebuilder,
                               filename: *const ::libc::c_char) ->
     *const git_tree_entry;
    pub fn git_treebuilder_insert(out: *mut *const git_tree_entry,
                                  bld: *mut git_treebuilder,
                                  filename: *const ::libc::c_char,
                                  id: *const git_oid,
                                  filemode: git_filemode_t) -> ::libc::c_int;
    pub fn git_treebuilder_remove(bld: *mut git_treebuilder,
                                  filename: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_treebuilder_filter(bld: *mut git_treebuilder,
                                  filter: git_treebuilder_filter_cb,
                                  payload: *mut ::libc::c_void);
    pub fn git_treebuilder_write(id: *mut git_oid, repo: *mut git_repository,
                                 bld: *mut git_treebuilder) -> ::libc::c_int;
    pub fn git_tree_walk(tree: *const git_tree, mode: git_treewalk_mode,
                         callback: git_treewalk_cb,
                         payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_strarray_free(array: *mut git_strarray);
    pub fn git_strarray_copy(tgt: *mut git_strarray, src: *const git_strarray)
     -> ::libc::c_int;
    pub fn git_reference_lookup(out: *mut *mut git_reference,
                                repo: *mut git_repository,
                                name: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_reference_name_to_id(out: *mut git_oid,
                                    repo: *mut git_repository,
                                    name: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_reference_dwim(out: *mut *mut git_reference,
                              repo: *mut git_repository,
                              shorthand: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_reference_symbolic_create(out: *mut *mut git_reference,
                                         repo: *mut git_repository,
                                         name: *const ::libc::c_char,
                                         target: *const ::libc::c_char,
                                         force: ::libc::c_int) ->
     ::libc::c_int;
    pub fn git_reference_create(out: *mut *mut git_reference,
                                repo: *mut git_repository,
                                name: *const ::libc::c_char,
                                id: *const git_oid, force: ::libc::c_int) ->
     ::libc::c_int;
    pub fn git_reference_target(_ref: *const git_reference) -> *const git_oid;
    pub fn git_reference_target_peel(_ref: *const git_reference) ->
     *const git_oid;
    pub fn git_reference_symbolic_target(_ref: *const git_reference) ->
     *const ::libc::c_char;
    pub fn git_reference_type(_ref: *const git_reference) -> git_ref_t;
    pub fn git_reference_name(_ref: *const git_reference) ->
     *const ::libc::c_char;
    pub fn git_reference_resolve(out: *mut *mut git_reference,
                                 _ref: *const git_reference) -> ::libc::c_int;
    pub fn git_reference_owner(_ref: *const git_reference) ->
     *mut git_repository;
    pub fn git_reference_symbolic_set_target(out: *mut *mut git_reference,
                                             _ref: *mut git_reference,
                                             target: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_reference_set_target(out: *mut *mut git_reference,
                                    _ref: *mut git_reference,
                                    id: *const git_oid) -> ::libc::c_int;
    pub fn git_reference_rename(new_ref: *mut *mut git_reference,
                                _ref: *mut git_reference,
                                new_name: *const ::libc::c_char,
                                force: ::libc::c_int) -> ::libc::c_int;
    pub fn git_reference_delete(_ref: *mut git_reference) -> ::libc::c_int;
    pub fn git_reference_list(array: *mut git_strarray,
                              repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_reference_foreach(repo: *mut git_repository,
                                 callback: git_reference_foreach_cb,
                                 payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_reference_foreach_name(repo: *mut git_repository,
                                      callback: git_reference_foreach_name_cb,
                                      payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_reference_free(_ref: *mut git_reference);
    pub fn git_reference_cmp(ref1: *mut git_reference,
                             ref2: *mut git_reference) -> ::libc::c_int;
    pub fn git_reference_iterator_new(out: *mut *mut git_reference_iterator,
                                      repo: *mut git_repository) ->
     ::libc::c_int;
    pub fn git_reference_iterator_glob_new(out:
                                               *mut *mut git_reference_iterator,
                                           repo: *mut git_repository,
                                           glob: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_reference_next(out: *mut *mut git_reference,
                              iter: *mut git_reference_iterator) ->
     ::libc::c_int;
    pub fn git_reference_next_name(out: *mut *const ::libc::c_char,
                                   iter: *mut git_reference_iterator) ->
     ::libc::c_int;
    pub fn git_reference_iterator_free(iter: *mut git_reference_iterator);
    pub fn git_reference_foreach_glob(repo: *mut git_repository,
                                      glob: *const ::libc::c_char,
                                      callback: git_reference_foreach_name_cb,
                                      payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_reference_has_log(_ref: *mut git_reference) -> ::libc::c_int;
    pub fn git_reference_is_branch(_ref: *mut git_reference) -> ::libc::c_int;
    pub fn git_reference_is_remote(_ref: *mut git_reference) -> ::libc::c_int;
    pub fn git_reference_is_tag(_ref: *mut git_reference) -> ::libc::c_int;
    pub fn git_reference_normalize_name(buffer_out: *mut ::libc::c_char,
                                        buffer_size: size_t,
                                        name: *const ::libc::c_char,
                                        flags: ::libc::c_uint) ->
     ::libc::c_int;
    pub fn git_reference_peel(out: *mut *mut git_object,
                              _ref: *mut git_reference, _type: git_otype) ->
     ::libc::c_int;
    pub fn git_reference_is_valid_name(refname: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_reference_shorthand(_ref: *mut git_reference) ->
     *const ::libc::c_char;
    pub fn git_diff_free(diff: *mut git_diff);
    pub fn git_diff_tree_to_tree(diff: *mut *mut git_diff,
                                 repo: *mut git_repository,
                                 old_tree: *mut git_tree,
                                 new_tree: *mut git_tree,
                                 opts: *const git_diff_options) ->
     ::libc::c_int;
    pub fn git_diff_tree_to_index(diff: *mut *mut git_diff,
                                  repo: *mut git_repository,
                                  old_tree: *mut git_tree,
                                  index: *mut git_index,
                                  opts: *const git_diff_options) ->
     ::libc::c_int;
    pub fn git_diff_index_to_workdir(diff: *mut *mut git_diff,
                                     repo: *mut git_repository,
                                     index: *mut git_index,
                                     opts: *const git_diff_options) ->
     ::libc::c_int;
    pub fn git_diff_tree_to_workdir(diff: *mut *mut git_diff,
                                    repo: *mut git_repository,
                                    old_tree: *mut git_tree,
                                    opts: *const git_diff_options) ->
     ::libc::c_int;
    pub fn git_diff_tree_to_workdir_with_index(diff: *mut *mut git_diff,
                                               repo: *mut git_repository,
                                               old_tree: *mut git_tree,
                                               opts: *const git_diff_options)
     -> ::libc::c_int;
    pub fn git_diff_merge(onto: *mut git_diff, from: *const git_diff) ->
     ::libc::c_int;
    pub fn git_diff_find_similar(diff: *mut git_diff,
                                 options: *const git_diff_find_options) ->
     ::libc::c_int;
    pub fn git_diff_options_init(options: *mut git_diff_options,
                                 version: ::libc::c_uint) -> ::libc::c_int;
    pub fn git_diff_num_deltas(diff: *const git_diff) -> size_t;
    pub fn git_diff_num_deltas_of_type(diff: *const git_diff,
                                       _type: git_delta_t) -> size_t;
    pub fn git_diff_get_delta(diff: *const git_diff, idx: size_t) ->
     *const git_diff_delta;
    pub fn git_diff_is_sorted_icase(diff: *const git_diff) -> ::libc::c_int;
    pub fn git_diff_foreach(diff: *mut git_diff, file_cb: git_diff_file_cb,
                            hunk_cb: git_diff_hunk_cb,
                            line_cb: git_diff_line_cb,
                            payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_diff_status_char(status: git_delta_t) -> ::libc::c_char;
    pub fn git_diff_print(diff: *mut git_diff, format: git_diff_format_t,
                          print_cb: git_diff_line_cb,
                          payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_diff_blobs(old_blob: *const git_blob,
                          old_as_path: *const ::libc::c_char,
                          new_blob: *const git_blob,
                          new_as_path: *const ::libc::c_char,
                          options: *const git_diff_options,
                          file_cb: git_diff_file_cb,
                          hunk_cb: git_diff_hunk_cb,
                          line_cb: git_diff_line_cb,
                          payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_diff_blob_to_buffer(old_blob: *const git_blob,
                                   old_as_path: *const ::libc::c_char,
                                   buffer: *const ::libc::c_char,
                                   buffer_len: size_t,
                                   buffer_as_path: *const ::libc::c_char,
                                   options: *const git_diff_options,
                                   file_cb: git_diff_file_cb,
                                   hunk_cb: git_diff_hunk_cb,
                                   line_cb: git_diff_line_cb,
                                   payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_checkout_head(repo: *mut git_repository,
                             opts: *const git_checkout_opts) -> ::libc::c_int;
    pub fn git_checkout_index(repo: *mut git_repository,
                              index: *mut git_index,
                              opts: *const git_checkout_opts) ->
     ::libc::c_int;
    pub fn git_checkout_tree(repo: *mut git_repository,
                             treeish: *const git_object,
                             opts: *const git_checkout_opts) -> ::libc::c_int;
    pub fn git_indexer_new(out: *mut *mut git_indexer,
                           path: *const ::libc::c_char, mode: ::libc::c_uint,
                           odb: *mut git_odb,
                           progress_cb: git_transfer_progress_callback,
                           progress_cb_payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_indexer_append(idx: *mut git_indexer,
                              data: *const ::libc::c_void, size: size_t,
                              stats: *mut git_transfer_progress) ->
     ::libc::c_int;
    pub fn git_indexer_commit(idx: *mut git_indexer,
                              stats: *mut git_transfer_progress) ->
     ::libc::c_int;
    pub fn git_indexer_hash(idx: *const git_indexer) -> *const git_oid;
    pub fn git_indexer_free(idx: *mut git_indexer);
    pub fn git_repository_open(out: *mut *mut git_repository,
                               path: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_repository_wrap_odb(out: *mut *mut git_repository,
                                   odb: *mut git_odb) -> ::libc::c_int;
    pub fn git_repository_discover(path_out: *mut ::libc::c_char,
                                   path_size: size_t,
                                   start_path: *const ::libc::c_char,
                                   across_fs: ::libc::c_int,
                                   ceiling_dirs: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_repository_open_ext(out: *mut *mut git_repository,
                                   path: *const ::libc::c_char,
                                   flags: ::libc::c_uint,
                                   ceiling_dirs: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_repository_open_bare(out: *mut *mut git_repository,
                                    bare_path: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_repository_free(repo: *mut git_repository);
    pub fn git_repository_init(out: *mut *mut git_repository,
                               path: *const ::libc::c_char,
                               is_bare: ::libc::c_uint) -> ::libc::c_int;
    pub fn git_repository_init_ext(out: *mut *mut git_repository,
                                   repo_path: *const ::libc::c_char,
                                   opts: *mut git_repository_init_options) ->
     ::libc::c_int;
    pub fn git_repository_head(out: *mut *mut git_reference,
                               repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_repository_head_detached(repo: *mut git_repository) ->
     ::libc::c_int;
    pub fn git_repository_head_unborn(repo: *mut git_repository) ->
     ::libc::c_int;
    pub fn git_repository_is_empty(repo: *mut git_repository) ->
     ::libc::c_int;
    pub fn git_repository_path(repo: *mut git_repository) ->
     *const ::libc::c_char;
    pub fn git_repository_workdir(repo: *mut git_repository) ->
     *const ::libc::c_char;
    pub fn git_repository_set_workdir(repo: *mut git_repository,
                                      workdir: *const ::libc::c_char,
                                      update_gitlink: ::libc::c_int) ->
     ::libc::c_int;
    pub fn git_repository_is_bare(repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_repository_config(out: *mut *mut git_config,
                                 repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_repository_odb(out: *mut *mut git_odb,
                              repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_repository_refdb(out: *mut *mut git_refdb,
                                repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_repository_index(out: *mut *mut git_index,
                                repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_repository_message(out: *mut ::libc::c_char, len: size_t,
                                  repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_repository_message_remove(repo: *mut git_repository) ->
     ::libc::c_int;
    pub fn git_repository_merge_cleanup(repo: *mut git_repository) ->
     ::libc::c_int;
    pub fn git_repository_fetchhead_foreach(repo: *mut git_repository,
                                            callback:
                                                git_repository_fetchhead_foreach_cb,
                                            payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_repository_mergehead_foreach(repo: *mut git_repository,
                                            callback:
                                                git_repository_mergehead_foreach_cb,
                                            payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_repository_hashfile(out: *mut git_oid,
                                   repo: *mut git_repository,
                                   path: *const ::libc::c_char,
                                   _type: git_otype,
                                   as_path: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_repository_set_head(repo: *mut git_repository,
                                   refname: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_repository_set_head_detached(repo: *mut git_repository,
                                            commitish: *const git_oid) ->
     ::libc::c_int;
    pub fn git_repository_detach_head(repo: *mut git_repository) ->
     ::libc::c_int;
    pub fn git_repository_state(repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_repository_set_namespace(repo: *mut git_repository,
                                        nmspace: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_repository_get_namespace(repo: *mut git_repository) ->
     *const ::libc::c_char;
    pub fn git_repository_is_shallow(repo: *mut git_repository) ->
     ::libc::c_int;
    pub fn git_refspec_src(refspec: *const git_refspec) ->
     *const ::libc::c_char;
    pub fn git_refspec_dst(refspec: *const git_refspec) ->
     *const ::libc::c_char;
    pub fn git_refspec_string(refspec: *const git_refspec) ->
     *const ::libc::c_char;
    pub fn git_refspec_force(refspec: *const git_refspec) -> ::libc::c_int;
    pub fn git_refspec_direction(spec: *const git_refspec) -> git_direction;
    pub fn git_refspec_src_matches(refspec: *const git_refspec,
                                   refname: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_refspec_dst_matches(refspec: *const git_refspec,
                                   refname: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_refspec_transform(out: *mut ::libc::c_char, outlen: size_t,
                                 spec: *const git_refspec,
                                 name: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_refspec_rtransform(out: *mut ::libc::c_char, outlen: size_t,
                                  spec: *const git_refspec,
                                  name: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_cred_has_username(cred: *mut git_cred) -> ::libc::c_int;
    pub fn git_cred_userpass_plaintext_new(out: *mut *mut git_cred,
                                           username: *const ::libc::c_char,
                                           password: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_cred_ssh_key_new(out: *mut *mut git_cred,
                                username: *const ::libc::c_char,
                                publickey: *const ::libc::c_char,
                                privatekey: *const ::libc::c_char,
                                passphrase: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_cred_ssh_custom_new(out: *mut *mut git_cred,
                                   username: *const ::libc::c_char,
                                   publickey: *const ::libc::c_char,
                                   publickey_len: size_t,
                                   sign_fn: git_cred_sign_callback,
                                   sign_data: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_cred_default_new(out: *mut *mut git_cred) -> ::libc::c_int;
    pub fn git_transport_new(out: *mut *mut git_transport,
                             owner: *mut git_remote,
                             url: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_transport_register(prefix: *const ::libc::c_char,
                                  priority: ::libc::c_uint,
                                  cb: git_transport_cb,
                                  param: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_transport_unregister(prefix: *const ::libc::c_char,
                                    priority: ::libc::c_uint) ->
     ::libc::c_int;
    pub fn git_transport_dummy(out: *mut *mut git_transport,
                               owner: *mut git_remote,
                               payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_transport_local(out: *mut *mut git_transport,
                               owner: *mut git_remote,
                               payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_transport_smart(out: *mut *mut git_transport,
                               owner: *mut git_remote,
                               payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_smart_subtransport_http(out: *mut *mut git_smart_subtransport,
                                       owner: *mut git_transport) ->
     ::libc::c_int;
    pub fn git_smart_subtransport_git(out: *mut *mut git_smart_subtransport,
                                      owner: *mut git_transport) ->
     ::libc::c_int;
    pub fn git_smart_subtransport_ssh(out: *mut *mut git_smart_subtransport,
                                      owner: *mut git_transport) ->
     ::libc::c_int;
    pub fn git_remote_create(out: *mut *mut git_remote,
                             repo: *mut git_repository,
                             name: *const ::libc::c_char,
                             url: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_remote_create_with_fetchspec(out: *mut *mut git_remote,
                                            repo: *mut git_repository,
                                            name: *const ::libc::c_char,
                                            url: *const ::libc::c_char,
                                            fetch: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_remote_create_inmemory(out: *mut *mut git_remote,
                                      repo: *mut git_repository,
                                      fetch: *const ::libc::c_char,
                                      url: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_remote_load(out: *mut *mut git_remote,
                           repo: *mut git_repository,
                           name: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_remote_save(remote: *const git_remote) -> ::libc::c_int;
    pub fn git_remote_owner(remote: *const git_remote) -> *mut git_repository;
    pub fn git_remote_name(remote: *const git_remote) ->
     *const ::libc::c_char;
    pub fn git_remote_url(remote: *const git_remote) -> *const ::libc::c_char;
    pub fn git_remote_pushurl(remote: *const git_remote) ->
     *const ::libc::c_char;
    pub fn git_remote_set_url(remote: *mut git_remote,
                              url: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_remote_set_pushurl(remote: *mut git_remote,
                                  url: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_remote_add_fetch(remote: *mut git_remote,
                                refspec: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_remote_get_fetch_refspecs(array: *mut git_strarray,
                                         remote: *mut git_remote) ->
     ::libc::c_int;
    pub fn git_remote_set_fetch_refspecs(remote: *mut git_remote,
                                         array: *mut git_strarray) ->
     ::libc::c_int;
    pub fn git_remote_add_push(remote: *mut git_remote,
                               refspec: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_remote_get_push_refspecs(array: *mut git_strarray,
                                        remote: *mut git_remote) ->
     ::libc::c_int;
    pub fn git_remote_set_push_refspecs(remote: *mut git_remote,
                                        array: *mut git_strarray) ->
     ::libc::c_int;
    pub fn git_remote_clear_refspecs(remote: *mut git_remote);
    pub fn git_remote_refspec_count(remote: *mut git_remote) -> size_t;
    pub fn git_remote_get_refspec(remote: *mut git_remote, n: size_t) ->
     *const git_refspec;
    pub fn git_remote_connect(remote: *mut git_remote,
                              direction: git_direction) -> ::libc::c_int;
    pub fn git_remote_ls(out: *mut *mut *const git_remote_head,
                         size: *mut size_t, remote: *mut git_remote) ->
     ::libc::c_int;
    pub fn git_remote_download(remote: *mut git_remote) -> ::libc::c_int;
    pub fn git_remote_connected(remote: *mut git_remote) -> ::libc::c_int;
    pub fn git_remote_stop(remote: *mut git_remote);
    pub fn git_remote_disconnect(remote: *mut git_remote);
    pub fn git_remote_free(remote: *mut git_remote);
    pub fn git_remote_update_tips(remote: *mut git_remote) -> ::libc::c_int;
    pub fn git_remote_fetch(remote: *mut git_remote) -> ::libc::c_int;
    pub fn git_remote_valid_url(url: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_remote_supported_url(url: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_remote_list(out: *mut git_strarray, repo: *mut git_repository)
     -> ::libc::c_int;
    pub fn git_remote_check_cert(remote: *mut git_remote,
                                 check: ::libc::c_int);
    pub fn git_remote_set_transport(remote: *mut git_remote,
                                    transport: *mut git_transport) ->
     ::libc::c_int;
    pub fn git_remote_set_callbacks(remote: *mut git_remote,
                                    callbacks: *const git_remote_callbacks) ->
     ::libc::c_int;
    pub fn git_remote_stats(remote: *mut git_remote) ->
     *const git_transfer_progress;
    pub fn git_remote_autotag(remote: *mut git_remote) ->
     git_remote_autotag_option_t;
    pub fn git_remote_set_autotag(remote: *mut git_remote,
                                  value: git_remote_autotag_option_t);
    pub fn git_remote_rename(remote: *mut git_remote,
                             new_name: *const ::libc::c_char,
                             callback: git_remote_rename_problem_cb,
                             payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_remote_update_fetchhead(remote: *mut git_remote) ->
     ::libc::c_int;
    pub fn git_remote_set_update_fetchhead(remote: *mut git_remote,
                                           value: ::libc::c_int);
    pub fn git_remote_is_valid_name(remote_name: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_clone(out: *mut *mut git_repository,
                     url: *const ::libc::c_char,
                     local_path: *const ::libc::c_char,
                     options: *const git_clone_options) -> ::libc::c_int;
    pub fn git_clone_into(repo: *mut git_repository, remote: *mut git_remote,
                          co_opts: *const git_checkout_opts,
                          branch: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_commit_lookup(commit: *mut *mut git_commit,
                             repo: *mut git_repository, id: *const git_oid) ->
     ::libc::c_int;
    pub fn git_commit_lookup_prefix(commit: *mut *mut git_commit,
                                    repo: *mut git_repository,
                                    id: *const git_oid, len: size_t) ->
     ::libc::c_int;
    pub fn git_commit_free(commit: *mut git_commit);
    pub fn git_commit_id(commit: *const git_commit) -> *const git_oid;
    pub fn git_commit_owner(commit: *const git_commit) -> *mut git_repository;
    pub fn git_commit_message_encoding(commit: *const git_commit) ->
     *const ::libc::c_char;
    pub fn git_commit_message(commit: *const git_commit) ->
     *const ::libc::c_char;
    pub fn git_commit_message_raw(commit: *const git_commit) ->
     *const ::libc::c_char;
    pub fn git_commit_time(commit: *const git_commit) -> git_time_t;
    pub fn git_commit_time_offset(commit: *const git_commit) -> ::libc::c_int;
    pub fn git_commit_committer(commit: *const git_commit) ->
     *const git_signature;
    pub fn git_commit_author(commit: *const git_commit) ->
     *const git_signature;
    pub fn git_commit_raw_header(commit: *const git_commit) ->
     *const ::libc::c_char;
    pub fn git_commit_tree(tree_out: *mut *mut git_tree,
                           commit: *const git_commit) -> ::libc::c_int;
    pub fn git_commit_tree_id(commit: *const git_commit) -> *const git_oid;
    pub fn git_commit_parentcount(commit: *const git_commit) ->
     ::libc::c_uint;
    pub fn git_commit_parent(out: *mut *mut git_commit,
                             commit: *const git_commit, n: ::libc::c_uint) ->
     ::libc::c_int;
    pub fn git_commit_parent_id(commit: *const git_commit, n: ::libc::c_uint)
     -> *const git_oid;
    pub fn git_commit_nth_gen_ancestor(ancestor: *mut *mut git_commit,
                                       commit: *const git_commit,
                                       n: ::libc::c_uint) -> ::libc::c_int;
    pub fn git_commit_create(id: *mut git_oid, repo: *mut git_repository,
                             update_ref: *const ::libc::c_char,
                             author: *const git_signature,
                             committer: *const git_signature,
                             message_encoding: *const ::libc::c_char,
                             message: *const ::libc::c_char,
                             tree: *const git_tree,
                             parent_count: ::libc::c_int,
                             parents: *mut *const git_commit) ->
     ::libc::c_int;
    pub fn git_commit_create_v(id: *mut git_oid, repo: *mut git_repository,
                               update_ref: *const ::libc::c_char,
                               author: *const git_signature,
                               committer: *const git_signature,
                               message_encoding: *const ::libc::c_char,
                               message: *const ::libc::c_char,
                               tree: *const git_tree,
                               parent_count: ::libc::c_int, ...) ->
     ::libc::c_int;
    pub fn git_config_find_global(out: *mut ::libc::c_char, length: size_t) ->
     ::libc::c_int;
    pub fn git_config_find_xdg(out: *mut ::libc::c_char, length: size_t) ->
     ::libc::c_int;
    pub fn git_config_find_system(out: *mut ::libc::c_char, length: size_t) ->
     ::libc::c_int;
    pub fn git_config_open_default(out: *mut *mut git_config) ->
     ::libc::c_int;
    pub fn git_config_new(out: *mut *mut git_config) -> ::libc::c_int;
    pub fn git_config_add_file_ondisk(cfg: *mut git_config,
                                      path: *const ::libc::c_char,
                                      level: git_config_level_t,
                                      force: ::libc::c_int) -> ::libc::c_int;
    pub fn git_config_open_ondisk(out: *mut *mut git_config,
                                  path: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_config_open_level(out: *mut *mut git_config,
                                 parent: *const git_config,
                                 level: git_config_level_t) -> ::libc::c_int;
    pub fn git_config_open_global(out: *mut *mut git_config,
                                  config: *mut git_config) -> ::libc::c_int;
    pub fn git_config_refresh(cfg: *mut git_config) -> ::libc::c_int;
    pub fn git_config_free(cfg: *mut git_config);
    pub fn git_config_get_entry(out: *mut *const git_config_entry,
                                cfg: *const git_config,
                                name: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_config_get_int32(out: *mut i32, cfg: *const git_config,
                                name: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_config_get_int64(out: *mut i64, cfg: *const git_config,
                                name: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_config_get_bool(out: *mut ::libc::c_int,
                               cfg: *const git_config,
                               name: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_config_get_string(out: *mut *const ::libc::c_char,
                                 cfg: *const git_config,
                                 name: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_config_get_multivar_foreach(cfg: *const git_config,
                                           name: *const ::libc::c_char,
                                           regexp: *const ::libc::c_char,
                                           callback: git_config_foreach_cb,
                                           payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_config_multivar_iterator_new(out:
                                                *mut *mut git_config_iterator,
                                            cfg: *const git_config,
                                            name: *const ::libc::c_char,
                                            regexp: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_config_next(entry: *mut *mut git_config_entry,
                           iter: *mut git_config_iterator) -> ::libc::c_int;
    pub fn git_config_iterator_free(iter: *mut git_config_iterator);
    pub fn git_config_set_int32(cfg: *mut git_config,
                                name: *const ::libc::c_char, value: i32)
     -> ::libc::c_int;
    pub fn git_config_set_int64(cfg: *mut git_config,
                                name: *const ::libc::c_char, value: i64)
     -> ::libc::c_int;
    pub fn git_config_set_bool(cfg: *mut git_config,
                               name: *const ::libc::c_char,
                               value: ::libc::c_int) -> ::libc::c_int;
    pub fn git_config_set_string(cfg: *mut git_config,
                                 name: *const ::libc::c_char,
                                 value: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_config_set_multivar(cfg: *mut git_config,
                                   name: *const ::libc::c_char,
                                   regexp: *const ::libc::c_char,
                                   value: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_config_delete_entry(cfg: *mut git_config,
                                   name: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_config_delete_multivar(cfg: *mut git_config,
                                      name: *const ::libc::c_char,
                                      regexp: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_config_foreach(cfg: *const git_config,
                              callback: git_config_foreach_cb,
                              payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_config_iterator_new(out: *mut *mut git_config_iterator,
                                   cfg: *const git_config) -> ::libc::c_int;
    pub fn git_config_iterator_glob_new(out: *mut *mut git_config_iterator,
                                        cfg: *const git_config,
                                        regexp: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_config_foreach_match(cfg: *const git_config,
                                    regexp: *const ::libc::c_char,
                                    callback: git_config_foreach_cb,
                                    payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_config_get_mapped(out: *mut ::libc::c_int,
                                 cfg: *const git_config,
                                 name: *const ::libc::c_char,
                                 maps: *const git_cvar_map, map_n: size_t) ->
     ::libc::c_int;
    pub fn git_config_lookup_map_value(out: *mut ::libc::c_int,
                                       maps: *const git_cvar_map,
                                       map_n: size_t,
                                       value: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_config_parse_bool(out: *mut ::libc::c_int,
                                 value: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_config_parse_int32(out: *mut i32,
                                  value: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_config_parse_int64(out: *mut i64,
                                  value: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_config_backend_foreach_match(backend: *mut git_config_backend,
                                            regexp: *const ::libc::c_char,
                                            _fn:
                                                ::std::option::Option<unsafe extern "C" fn
                                                                          (arg1:
                                                                               *const git_config_entry,
                                                                           arg2:
                                                                               *mut ::libc::c_void)
                                                                          ->
                                                                              ::libc::c_int>,
                                            data: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn giterr_last() -> *const git_error;
    pub fn giterr_clear();
    pub fn giterr_detach(cpy: *mut git_error) -> ::libc::c_int;
    pub fn giterr_set_str(error_class: ::libc::c_int,
                          string: *const ::libc::c_char);
    pub fn giterr_set_oom();
    pub fn git_filter_list_load(filters: *mut *mut git_filter_list,
                                repo: *mut git_repository,
                                blob: *mut git_blob,
                                path: *const ::libc::c_char,
                                mode: git_filter_mode_t) -> ::libc::c_int;
    pub fn git_filter_list_apply_to_data(out: *mut git_buf,
                                         filters: *mut git_filter_list,
                                         _in: *mut git_buf) -> ::libc::c_int;
    pub fn git_filter_list_apply_to_file(out: *mut git_buf,
                                         filters: *mut git_filter_list,
                                         repo: *mut git_repository,
                                         path: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_filter_list_apply_to_blob(out: *mut git_buf,
                                         filters: *mut git_filter_list,
                                         blob: *mut git_blob) ->
     ::libc::c_int;
    pub fn git_filter_list_free(filters: *mut git_filter_list);
    pub fn git_graph_ahead_behind(ahead: *mut size_t, behind: *mut size_t,
                                  repo: *mut git_repository,
                                  local: *const git_oid,
                                  upstream: *const git_oid) -> ::libc::c_int;
    pub fn git_ignore_add_rule(repo: *mut git_repository,
                               rules: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_ignore_clear_internal_rules(repo: *mut git_repository) ->
     ::libc::c_int;
    pub fn git_ignore_path_is_ignored(ignored: *mut ::libc::c_int,
                                      repo: *mut git_repository,
                                      path: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_index_open(out: *mut *mut git_index,
                          index_path: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_index_new(out: *mut *mut git_index) -> ::libc::c_int;
    pub fn git_index_free(index: *mut git_index);
    pub fn git_index_owner(index: *const git_index) -> *mut git_repository;
    pub fn git_index_caps(index: *const git_index) -> ::libc::c_uint;
    pub fn git_index_set_caps(index: *mut git_index, caps: ::libc::c_uint) ->
     ::libc::c_int;
    pub fn git_index_read(index: *mut git_index, force: ::libc::c_int) ->
     ::libc::c_int;
    pub fn git_index_write(index: *mut git_index) -> ::libc::c_int;
    pub fn git_index_path(index: *mut git_index) -> *const ::libc::c_char;
    pub fn git_index_read_tree(index: *mut git_index, tree: *const git_tree)
     -> ::libc::c_int;
    pub fn git_index_write_tree(out: *mut git_oid, index: *mut git_index) ->
     ::libc::c_int;
    pub fn git_index_write_tree_to(out: *mut git_oid, index: *mut git_index,
                                   repo: *mut git_repository) ->
     ::libc::c_int;
    pub fn git_index_entrycount(index: *const git_index) -> size_t;
    pub fn git_index_clear(index: *mut git_index);
    pub fn git_index_get_byindex(index: *mut git_index, n: size_t) ->
     *const git_index_entry;
    pub fn git_index_get_bypath(index: *mut git_index,
                                path: *const ::libc::c_char,
                                stage: ::libc::c_int) ->
     *const git_index_entry;
    pub fn git_index_remove(index: *mut git_index,
                            path: *const ::libc::c_char, stage: ::libc::c_int)
     -> ::libc::c_int;
    pub fn git_index_remove_directory(index: *mut git_index,
                                      dir: *const ::libc::c_char,
                                      stage: ::libc::c_int) -> ::libc::c_int;
    pub fn git_index_add(index: *mut git_index,
                         source_entry: *const git_index_entry) ->
     ::libc::c_int;
    pub fn git_index_entry_stage(entry: *const git_index_entry) ->
     ::libc::c_int;
    pub fn git_index_add_bypath(index: *mut git_index,
                                path: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_index_remove_bypath(index: *mut git_index,
                                   path: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_index_add_all(index: *mut git_index,
                             pathspec: *const git_strarray,
                             flags: ::libc::c_uint,
                             callback: git_index_matched_path_cb,
                             payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_index_remove_all(index: *mut git_index,
                                pathspec: *const git_strarray,
                                callback: git_index_matched_path_cb,
                                payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_index_update_all(index: *mut git_index,
                                pathspec: *const git_strarray,
                                callback: git_index_matched_path_cb,
                                payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_index_find(at_pos: *mut size_t, index: *mut git_index,
                          path: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_index_conflict_add(index: *mut git_index,
                                  ancestor_entry: *const git_index_entry,
                                  our_entry: *const git_index_entry,
                                  their_entry: *const git_index_entry) ->
     ::libc::c_int;
    pub fn git_index_conflict_get(ancestor_out: *mut *const git_index_entry,
                                  our_out: *mut *const git_index_entry,
                                  their_out: *mut *const git_index_entry,
                                  index: *mut git_index,
                                  path: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_index_conflict_remove(index: *mut git_index,
                                     path: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_index_conflict_cleanup(index: *mut git_index);
    pub fn git_index_has_conflicts(index: *const git_index) -> ::libc::c_int;
    pub fn git_index_conflict_iterator_new(iterator_out:
                                               *mut *mut git_index_conflict_iterator,
                                           index: *mut git_index) ->
     ::libc::c_int;
    pub fn git_index_conflict_next(ancestor_out: *mut *const git_index_entry,
                                   our_out: *mut *const git_index_entry,
                                   their_out: *mut *const git_index_entry,
                                   iterator: *mut git_index_conflict_iterator)
     -> ::libc::c_int;
    pub fn git_index_conflict_iterator_free(iterator:
                                                *mut git_index_conflict_iterator);
    pub fn git_merge_base(out: *mut git_oid, repo: *mut git_repository,
                          one: *const git_oid, two: *const git_oid) ->
     ::libc::c_int;
    pub fn git_merge_base_many(out: *mut git_oid, repo: *mut git_repository,
                               length: size_t, input_array: *const git_oid) ->
     ::libc::c_int;
    pub fn git_merge_head_from_ref(out: *mut *mut git_merge_head,
                                   repo: *mut git_repository,
                                   _ref: *mut git_reference) -> ::libc::c_int;
    pub fn git_merge_head_from_fetchhead(out: *mut *mut git_merge_head,
                                         repo: *mut git_repository,
                                         branch_name: *const ::libc::c_char,
                                         remote_url: *const ::libc::c_char,
                                         oid: *const git_oid) ->
     ::libc::c_int;
    pub fn git_merge_head_from_oid(out: *mut *mut git_merge_head,
                                   repo: *mut git_repository,
                                   oid: *const git_oid) -> ::libc::c_int;
    pub fn git_merge_head_free(head: *mut git_merge_head);
    pub fn git_merge_trees(out: *mut *mut git_index,
                           repo: *mut git_repository,
                           ancestor_tree: *const git_tree,
                           our_tree: *const git_tree,
                           their_tree: *const git_tree,
                           opts: *const git_merge_tree_opts) -> ::libc::c_int;
    pub fn git_merge(out: *mut *mut git_merge_result,
                     repo: *mut git_repository,
                     their_heads: *mut *const git_merge_head,
                     their_heads_len: size_t, opts: *const git_merge_opts) ->
     ::libc::c_int;
    pub fn git_merge_result_is_uptodate(merge_result: *mut git_merge_result)
     -> ::libc::c_int;
    pub fn git_merge_result_is_fastforward(merge_result:
                                               *mut git_merge_result) ->
     ::libc::c_int;
    pub fn git_merge_result_fastforward_oid(out: *mut git_oid,
                                            merge_result:
                                                *mut git_merge_result) ->
     ::libc::c_int;
    pub fn git_merge_result_free(merge_result: *mut git_merge_result);
    pub fn git_message_prettify(out: *mut ::libc::c_char, out_size: size_t,
                                message: *const ::libc::c_char,
                                strip_comments: ::libc::c_int) ->
     ::libc::c_int;
    pub fn git_note_iterator_new(out: *mut *mut git_note_iterator,
                                 repo: *mut git_repository,
                                 notes_ref: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_note_iterator_free(it: *mut git_note_iterator);
    pub fn git_note_next(note_id: *mut git_oid, annotated_id: *mut git_oid,
                         it: *mut git_note_iterator) -> ::libc::c_int;
    pub fn git_note_read(out: *mut *mut git_note, repo: *mut git_repository,
                         notes_ref: *const ::libc::c_char,
                         oid: *const git_oid) -> ::libc::c_int;
    pub fn git_note_message(note: *const git_note) -> *const ::libc::c_char;
    pub fn git_note_oid(note: *const git_note) -> *const git_oid;
    pub fn git_note_create(out: *mut git_oid, repo: *mut git_repository,
                           author: *const git_signature,
                           committer: *const git_signature,
                           notes_ref: *const ::libc::c_char,
                           oid: *const git_oid, note: *const ::libc::c_char,
                           force: ::libc::c_int) -> ::libc::c_int;
    pub fn git_note_remove(repo: *mut git_repository,
                           notes_ref: *const ::libc::c_char,
                           author: *const git_signature,
                           committer: *const git_signature,
                           oid: *const git_oid) -> ::libc::c_int;
    pub fn git_note_free(note: *mut git_note);
    pub fn git_note_default_ref(out: *mut *const ::libc::c_char,
                                repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_note_foreach(repo: *mut git_repository,
                            notes_ref: *const ::libc::c_char,
                            note_cb: git_note_foreach_cb,
                            payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_odb_new(out: *mut *mut git_odb) -> ::libc::c_int;
    pub fn git_odb_open(out: *mut *mut git_odb,
                        objects_dir: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_odb_add_disk_alternate(odb: *mut git_odb,
                                      path: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_odb_free(db: *mut git_odb);
    pub fn git_odb_read(out: *mut *mut git_odb_object, db: *mut git_odb,
                        id: *const git_oid) -> ::libc::c_int;
    pub fn git_odb_read_prefix(out: *mut *mut git_odb_object,
                               db: *mut git_odb, short_id: *const git_oid,
                               len: size_t) -> ::libc::c_int;
    pub fn git_odb_read_header(len_out: *mut size_t, type_out: *mut git_otype,
                               db: *mut git_odb, id: *const git_oid) ->
     ::libc::c_int;
    pub fn git_odb_exists(db: *mut git_odb, id: *const git_oid) ->
     ::libc::c_int;
    pub fn git_odb_refresh(db: *mut Struct_git_odb) -> ::libc::c_int;
    pub fn git_odb_foreach(db: *mut git_odb, cb: git_odb_foreach_cb,
                           payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_odb_write(out: *mut git_oid, odb: *mut git_odb,
                         data: *const ::libc::c_void, len: size_t,
                         _type: git_otype) -> ::libc::c_int;
    pub fn git_odb_open_wstream(out: *mut *mut git_odb_stream,
                                db: *mut git_odb, size: size_t,
                                _type: git_otype) -> ::libc::c_int;
    pub fn git_odb_stream_write(stream: *mut git_odb_stream,
                                buffer: *const ::libc::c_char, len: size_t) ->
     ::libc::c_int;
    pub fn git_odb_stream_finalize_write(out: *mut git_oid,
                                         stream: *mut git_odb_stream) ->
     ::libc::c_int;
    pub fn git_odb_stream_read(stream: *mut git_odb_stream,
                               buffer: *mut ::libc::c_char, len: size_t) ->
     ::libc::c_int;
    pub fn git_odb_stream_free(stream: *mut git_odb_stream);
    pub fn git_odb_open_rstream(out: *mut *mut git_odb_stream,
                                db: *mut git_odb, oid: *const git_oid) ->
     ::libc::c_int;
    pub fn git_odb_write_pack(out: *mut *mut git_odb_writepack,
                              db: *mut git_odb,
                              progress_cb: git_transfer_progress_callback,
                              progress_payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_odb_hash(out: *mut git_oid, data: *const ::libc::c_void,
                        len: size_t, _type: git_otype) -> ::libc::c_int;
    pub fn git_odb_hashfile(out: *mut git_oid, path: *const ::libc::c_char,
                            _type: git_otype) -> ::libc::c_int;
    pub fn git_odb_object_dup(dest: *mut *mut git_odb_object,
                              source: *mut git_odb_object) -> ::libc::c_int;
    pub fn git_odb_object_free(object: *mut git_odb_object);
    pub fn git_odb_object_id(object: *mut git_odb_object) -> *const git_oid;
    pub fn git_odb_object_data(object: *mut git_odb_object) ->
     *const ::libc::c_void;
    pub fn git_odb_object_size(object: *mut git_odb_object) -> size_t;
    pub fn git_odb_object_type(object: *mut git_odb_object) -> git_otype;
    pub fn git_odb_add_backend(odb: *mut git_odb,
                               backend: *mut git_odb_backend,
                               priority: ::libc::c_int) -> ::libc::c_int;
    pub fn git_odb_add_alternate(odb: *mut git_odb,
                                 backend: *mut git_odb_backend,
                                 priority: ::libc::c_int) -> ::libc::c_int;
    pub fn git_odb_num_backends(odb: *mut git_odb) -> size_t;
    pub fn git_odb_get_backend(out: *mut *mut git_odb_backend,
                               odb: *mut git_odb, pos: size_t) ->
     ::libc::c_int;
    pub fn git_packbuilder_new(out: *mut *mut git_packbuilder,
                               repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_packbuilder_set_threads(pb: *mut git_packbuilder,
                                       n: ::libc::c_uint) -> ::libc::c_uint;
    pub fn git_packbuilder_insert(pb: *mut git_packbuilder,
                                  id: *const git_oid,
                                  name: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_packbuilder_insert_tree(pb: *mut git_packbuilder,
                                       id: *const git_oid) -> ::libc::c_int;
    pub fn git_packbuilder_insert_commit(pb: *mut git_packbuilder,
                                         id: *const git_oid) -> ::libc::c_int;
    pub fn git_packbuilder_write(pb: *mut git_packbuilder,
                                 path: *const ::libc::c_char,
                                 mode: ::libc::c_uint,
                                 progress_cb: git_transfer_progress_callback,
                                 progress_cb_payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_packbuilder_hash(pb: *mut git_packbuilder) -> *const git_oid;
    pub fn git_packbuilder_foreach(pb: *mut git_packbuilder,
                                   cb: git_packbuilder_foreach_cb,
                                   payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_packbuilder_object_count(pb: *mut git_packbuilder) -> u32;
    pub fn git_packbuilder_written(pb: *mut git_packbuilder) -> u32;
    pub fn git_packbuilder_set_callbacks(pb: *mut git_packbuilder,
                                         progress_cb:
                                             git_packbuilder_progress,
                                         progress_cb_payload:
                                             *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_packbuilder_free(pb: *mut git_packbuilder);
    pub fn git_patch_from_diff(out: *mut *mut git_patch, diff: *mut git_diff,
                               idx: size_t) -> ::libc::c_int;
    pub fn git_patch_from_blobs(out: *mut *mut git_patch,
                                old_blob: *const git_blob,
                                old_as_path: *const ::libc::c_char,
                                new_blob: *const git_blob,
                                new_as_path: *const ::libc::c_char,
                                opts: *const git_diff_options) ->
     ::libc::c_int;
    pub fn git_patch_from_blob_and_buffer(out: *mut *mut git_patch,
                                          old_blob: *const git_blob,
                                          old_as_path: *const ::libc::c_char,
                                          buffer: *const ::libc::c_char,
                                          buffer_len: size_t,
                                          buffer_as_path:
                                              *const ::libc::c_char,
                                          opts: *const git_diff_options) ->
     ::libc::c_int;
    pub fn git_patch_free(patch: *mut git_patch);
    pub fn git_patch_get_delta(patch: *mut git_patch) ->
     *const git_diff_delta;
    pub fn git_patch_num_hunks(patch: *mut git_patch) -> size_t;
    pub fn git_patch_line_stats(total_context: *mut size_t,
                                total_additions: *mut size_t,
                                total_deletions: *mut size_t,
                                patch: *const git_patch) -> ::libc::c_int;
    pub fn git_patch_get_hunk(out: *mut *const git_diff_hunk,
                              lines_in_hunk: *mut size_t,
                              patch: *mut git_patch, hunk_idx: size_t) ->
     ::libc::c_int;
    pub fn git_patch_num_lines_in_hunk(patch: *mut git_patch,
                                       hunk_idx: size_t) -> ::libc::c_int;
    pub fn git_patch_get_line_in_hunk(out: *mut *const git_diff_line,
                                      patch: *mut git_patch, hunk_idx: size_t,
                                      line_of_hunk: size_t) -> ::libc::c_int;
    pub fn git_patch_size(patch: *mut git_patch,
                          include_context: ::libc::c_int,
                          include_hunk_headers: ::libc::c_int,
                          include_file_headers: ::libc::c_int) -> size_t;
    pub fn git_patch_print(patch: *mut git_patch, print_cb: git_diff_line_cb,
                           payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_patch_to_str(string: *mut *mut ::libc::c_char,
                            patch: *mut git_patch) -> ::libc::c_int;
    pub fn git_pathspec_new(out: *mut *mut git_pathspec,
                            pathspec: *const git_strarray) -> ::libc::c_int;
    pub fn git_pathspec_free(ps: *mut git_pathspec);
    pub fn git_pathspec_matches_path(ps: *const git_pathspec, flags: u32,
                                     path: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_pathspec_match_workdir(out: *mut *mut git_pathspec_match_list,
                                      repo: *mut git_repository,
                                      flags: u32, ps: *mut git_pathspec)
     -> ::libc::c_int;
    pub fn git_pathspec_match_index(out: *mut *mut git_pathspec_match_list,
                                    index: *mut git_index, flags: u32,
                                    ps: *mut git_pathspec) -> ::libc::c_int;
    pub fn git_pathspec_match_tree(out: *mut *mut git_pathspec_match_list,
                                   tree: *mut git_tree, flags: u32,
                                   ps: *mut git_pathspec) -> ::libc::c_int;
    pub fn git_pathspec_match_diff(out: *mut *mut git_pathspec_match_list,
                                   diff: *mut git_diff, flags: u32,
                                   ps: *mut git_pathspec) -> ::libc::c_int;
    pub fn git_pathspec_match_list_free(m: *mut git_pathspec_match_list);
    pub fn git_pathspec_match_list_entrycount(m:
                                                  *const git_pathspec_match_list)
     -> size_t;
    pub fn git_pathspec_match_list_entry(m: *const git_pathspec_match_list,
                                         pos: size_t) ->
     *const ::libc::c_char;
    pub fn git_pathspec_match_list_diff_entry(m:
                                                  *const git_pathspec_match_list,
                                              pos: size_t) ->
     *const git_diff_delta;
    pub fn git_pathspec_match_list_failed_entrycount(m:
                                                         *const git_pathspec_match_list)
     -> size_t;
    pub fn git_pathspec_match_list_failed_entry(m:
                                                    *const git_pathspec_match_list,
                                                pos: size_t) ->
     *const ::libc::c_char;
    pub fn git_push_new(out: *mut *mut git_push, remote: *mut git_remote) ->
     ::libc::c_int;
    pub fn git_push_set_options(push: *mut git_push,
                                opts: *const git_push_options) ->
     ::libc::c_int;
    pub fn git_push_set_callbacks(push: *mut git_push,
                                  pack_progress_cb: git_packbuilder_progress,
                                  pack_progress_cb_payload:
                                      *mut ::libc::c_void,
                                  transfer_progress_cb:
                                      git_push_transfer_progress,
                                  transfer_progress_cb_payload:
                                      *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_push_add_refspec(push: *mut git_push,
                                refspec: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_push_update_tips(push: *mut git_push) -> ::libc::c_int;
    pub fn git_push_finish(push: *mut git_push) -> ::libc::c_int;
    pub fn git_push_unpack_ok(push: *mut git_push) -> ::libc::c_int;
    pub fn git_push_status_foreach(push: *mut git_push,
                                   cb:
                                       ::std::option::Option<unsafe extern "C" fn
                                                                 (arg1:
                                                                      *const ::libc::c_char,
                                                                  arg2:
                                                                      *const ::libc::c_char,
                                                                  arg3:
                                                                      *mut ::libc::c_void)
                                                                 ->
                                                                     ::libc::c_int>,
                                   data: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_push_free(push: *mut git_push);
    pub fn git_refdb_new(out: *mut *mut git_refdb, repo: *mut git_repository)
     -> ::libc::c_int;
    pub fn git_refdb_open(out: *mut *mut git_refdb, repo: *mut git_repository)
     -> ::libc::c_int;
    pub fn git_refdb_compress(refdb: *mut git_refdb) -> ::libc::c_int;
    pub fn git_refdb_free(refdb: *mut git_refdb);
    pub fn git_reflog_read(out: *mut *mut git_reflog,
                           repo: *mut git_repository,
                           name: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_reflog_write(reflog: *mut git_reflog) -> ::libc::c_int;
    pub fn git_reflog_append(reflog: *mut git_reflog, id: *const git_oid,
                             committer: *const git_signature,
                             msg: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_reflog_append_to(repo: *mut git_repository,
                                name: *const ::libc::c_char,
                                id: *const git_oid,
                                committer: *const git_signature,
                                msg: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_reflog_rename(repo: *mut git_repository,
                             old_name: *const ::libc::c_char,
                             name: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_reflog_delete(repo: *mut git_repository,
                             name: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_reflog_entrycount(reflog: *mut git_reflog) -> size_t;
    pub fn git_reflog_entry_byindex(reflog: *mut git_reflog, idx: size_t) ->
     *const git_reflog_entry;
    pub fn git_reflog_drop(reflog: *mut git_reflog, idx: size_t,
                           rewrite_previous_entry: ::libc::c_int) ->
     ::libc::c_int;
    pub fn git_reflog_entry_id_old(entry: *const git_reflog_entry) ->
     *const git_oid;
    pub fn git_reflog_entry_id_new(entry: *const git_reflog_entry) ->
     *const git_oid;
    pub fn git_reflog_entry_committer(entry: *const git_reflog_entry) ->
     *const git_signature;
    pub fn git_reflog_entry_message(entry: *const git_reflog_entry) ->
     *const ::libc::c_char;
    pub fn git_reflog_free(reflog: *mut git_reflog);
    pub fn git_reset(repo: *mut git_repository, target: *mut git_object,
                     reset_type: git_reset_t) -> ::libc::c_int;
    pub fn git_reset_default(repo: *mut git_repository,
                             target: *mut git_object,
                             pathspecs: *mut git_strarray) -> ::libc::c_int;
    pub fn git_revparse_single(out: *mut *mut git_object,
                               repo: *mut git_repository,
                               spec: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_revparse_ext(object_out: *mut *mut git_object,
                            reference_out: *mut *mut git_reference,
                            repo: *mut git_repository,
                            spec: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_revparse(revspec: *mut git_revspec, repo: *mut git_repository,
                        spec: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_revwalk_new(out: *mut *mut git_revwalk,
                           repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_revwalk_reset(walker: *mut git_revwalk);
    pub fn git_revwalk_push(walk: *mut git_revwalk, id: *const git_oid) ->
     ::libc::c_int;
    pub fn git_revwalk_push_glob(walk: *mut git_revwalk,
                                 glob: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_revwalk_push_head(walk: *mut git_revwalk) -> ::libc::c_int;
    pub fn git_revwalk_hide(walk: *mut git_revwalk, commit_id: *const git_oid)
     -> ::libc::c_int;
    pub fn git_revwalk_hide_glob(walk: *mut git_revwalk,
                                 glob: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_revwalk_hide_head(walk: *mut git_revwalk) -> ::libc::c_int;
    pub fn git_revwalk_push_ref(walk: *mut git_revwalk,
                                refname: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_revwalk_hide_ref(walk: *mut git_revwalk,
                                refname: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_revwalk_next(out: *mut git_oid, walk: *mut git_revwalk) ->
     ::libc::c_int;
    pub fn git_revwalk_sorting(walk: *mut git_revwalk,
                               sort_mode: ::libc::c_uint);
    pub fn git_revwalk_push_range(walk: *mut git_revwalk,
                                  range: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_revwalk_simplify_first_parent(walk: *mut git_revwalk);
    pub fn git_revwalk_free(walk: *mut git_revwalk);
    pub fn git_revwalk_repository(walk: *mut git_revwalk) ->
     *mut git_repository;
    pub fn git_signature_new(out: *mut *mut git_signature,
                             name: *const ::libc::c_char,
                             email: *const ::libc::c_char, time: git_time_t,
                             offset: ::libc::c_int) -> ::libc::c_int;
    pub fn git_signature_now(out: *mut *mut git_signature,
                             name: *const ::libc::c_char,
                             email: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_signature_default(out: *mut *mut git_signature,
                                 repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_signature_dup(sig: *const git_signature) -> *mut git_signature;
    pub fn git_signature_free(sig: *mut git_signature);
    pub fn git_stash_save(out: *mut git_oid, repo: *mut git_repository,
                          stasher: *const git_signature,
                          message: *const ::libc::c_char,
                          flags: ::libc::c_uint) -> ::libc::c_int;
    pub fn git_stash_foreach(repo: *mut git_repository,
                             callback: git_stash_cb,
                             payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_stash_drop(repo: *mut git_repository, index: size_t) ->
     ::libc::c_int;
    pub fn git_status_foreach(repo: *mut git_repository,
                              callback: git_status_cb,
                              payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_status_foreach_ext(repo: *mut git_repository,
                                  opts: *const git_status_options,
                                  callback: git_status_cb,
                                  payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_status_file(status_flags: *mut ::libc::c_uint,
                           repo: *mut git_repository,
                           path: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_status_list_new(out: *mut *mut git_status_list,
                               repo: *mut git_repository,
                               opts: *const git_status_options) ->
     ::libc::c_int;
    pub fn git_status_list_entrycount(statuslist: *mut git_status_list) ->
     size_t;
    pub fn git_status_byindex(statuslist: *mut git_status_list, idx: size_t)
     -> *const git_status_entry;
    pub fn git_status_list_free(statuslist: *mut git_status_list);
    pub fn git_status_should_ignore(ignored: *mut ::libc::c_int,
                                    repo: *mut git_repository,
                                    path: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_submodule_lookup(submodule: *mut *mut git_submodule,
                                repo: *mut git_repository,
                                name: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_submodule_foreach(repo: *mut git_repository,
                                 callback:
                                     ::std::option::Option<unsafe extern "C" fn
                                                               (arg1:
                                                                    *mut git_submodule,
                                                                arg2:
                                                                    *const ::libc::c_char,
                                                                arg3:
                                                                    *mut ::libc::c_void)
                                                               ->
                                                                   ::libc::c_int>,
                                 payload: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn git_submodule_add_setup(submodule: *mut *mut git_submodule,
                                   repo: *mut git_repository,
                                   url: *const ::libc::c_char,
                                   path: *const ::libc::c_char,
                                   use_gitlink: ::libc::c_int) ->
     ::libc::c_int;
    pub fn git_submodule_add_finalize(submodule: *mut git_submodule) ->
     ::libc::c_int;
    pub fn git_submodule_add_to_index(submodule: *mut git_submodule,
                                      write_index: ::libc::c_int) ->
     ::libc::c_int;
    pub fn git_submodule_save(submodule: *mut git_submodule) -> ::libc::c_int;
    pub fn git_submodule_owner(submodule: *mut git_submodule) ->
     *mut git_repository;
    pub fn git_submodule_name(submodule: *mut git_submodule) ->
     *const ::libc::c_char;
    pub fn git_submodule_path(submodule: *mut git_submodule) ->
     *const ::libc::c_char;
    pub fn git_submodule_url(submodule: *mut git_submodule) ->
     *const ::libc::c_char;
    pub fn git_submodule_set_url(submodule: *mut git_submodule,
                                 url: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_submodule_index_id(submodule: *mut git_submodule) ->
     *const git_oid;
    pub fn git_submodule_head_id(submodule: *mut git_submodule) ->
     *const git_oid;
    pub fn git_submodule_wd_id(submodule: *mut git_submodule) ->
     *const git_oid;
    pub fn git_submodule_ignore(submodule: *mut git_submodule) ->
     git_submodule_ignore_t;
    pub fn git_submodule_set_ignore(submodule: *mut git_submodule,
                                    ignore: git_submodule_ignore_t) ->
     git_submodule_ignore_t;
    pub fn git_submodule_update(submodule: *mut git_submodule) ->
     git_submodule_update_t;
    pub fn git_submodule_set_update(submodule: *mut git_submodule,
                                    update: git_submodule_update_t) ->
     git_submodule_update_t;
    pub fn git_submodule_fetch_recurse_submodules(submodule:
                                                      *mut git_submodule) ->
     ::libc::c_int;
    pub fn git_submodule_set_fetch_recurse_submodules(submodule:
                                                          *mut git_submodule,
                                                      fetch_recurse_submodules:
                                                          ::libc::c_int) ->
     ::libc::c_int;
    pub fn git_submodule_init(submodule: *mut git_submodule,
                              overwrite: ::libc::c_int) -> ::libc::c_int;
    pub fn git_submodule_sync(submodule: *mut git_submodule) -> ::libc::c_int;
    pub fn git_submodule_open(repo: *mut *mut git_repository,
                              submodule: *mut git_submodule) -> ::libc::c_int;
    pub fn git_submodule_reload(submodule: *mut git_submodule) ->
     ::libc::c_int;
    pub fn git_submodule_reload_all(repo: *mut git_repository) ->
     ::libc::c_int;
    pub fn git_submodule_status(status: *mut ::libc::c_uint,
                                submodule: *mut git_submodule) ->
     ::libc::c_int;
    pub fn git_submodule_location(location_status: *mut ::libc::c_uint,
                                  submodule: *mut git_submodule) ->
     ::libc::c_int;
    pub fn git_tag_lookup(out: *mut *mut git_tag, repo: *mut git_repository,
                          id: *const git_oid) -> ::libc::c_int;
    pub fn git_tag_lookup_prefix(out: *mut *mut git_tag,
                                 repo: *mut git_repository,
                                 id: *const git_oid, len: size_t) ->
     ::libc::c_int;
    pub fn git_tag_free(tag: *mut git_tag);
    pub fn git_tag_id(tag: *const git_tag) -> *const git_oid;
    pub fn git_tag_owner(tag: *const git_tag) -> *mut git_repository;
    pub fn git_tag_target(target_out: *mut *mut git_object,
                          tag: *const git_tag) -> ::libc::c_int;
    pub fn git_tag_target_id(tag: *const git_tag) -> *const git_oid;
    pub fn git_tag_target_type(tag: *const git_tag) -> git_otype;
    pub fn git_tag_name(tag: *const git_tag) -> *const ::libc::c_char;
    pub fn git_tag_tagger(tag: *const git_tag) -> *const git_signature;
    pub fn git_tag_message(tag: *const git_tag) -> *const ::libc::c_char;
    pub fn git_tag_create(oid: *mut git_oid, repo: *mut git_repository,
                          tag_name: *const ::libc::c_char,
                          target: *const git_object,
                          tagger: *const git_signature,
                          message: *const ::libc::c_char,
                          force: ::libc::c_int) -> ::libc::c_int;
    pub fn git_tag_annotation_create(oid: *mut git_oid,
                                     repo: *mut git_repository,
                                     tag_name: *const ::libc::c_char,
                                     target: *const git_object,
                                     tagger: *const git_signature,
                                     message: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn git_tag_create_frombuffer(oid: *mut git_oid,
                                     repo: *mut git_repository,
                                     buffer: *const ::libc::c_char,
                                     force: ::libc::c_int) -> ::libc::c_int;
    pub fn git_tag_create_lightweight(oid: *mut git_oid,
                                      repo: *mut git_repository,
                                      tag_name: *const ::libc::c_char,
                                      target: *const git_object,
                                      force: ::libc::c_int) -> ::libc::c_int;
    pub fn git_tag_delete(repo: *mut git_repository,
                          tag_name: *const ::libc::c_char) -> ::libc::c_int;
    pub fn git_tag_list(tag_names: *mut git_strarray,
                        repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_tag_list_match(tag_names: *mut git_strarray,
                              pattern: *const ::libc::c_char,
                              repo: *mut git_repository) -> ::libc::c_int;
    pub fn git_tag_foreach(repo: *mut git_repository,
                           callback: git_tag_foreach_cb,
                           payload: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn git_tag_peel(tag_target_out: *mut *mut git_object,
                        tag: *const git_tag) -> ::libc::c_int;
    pub fn git_threads_init() -> ::libc::c_int;
    pub fn git_threads_shutdown();
}
