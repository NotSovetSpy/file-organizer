#!/usr/bin/env bash
set -euo pipefail

# =============================================================================
# Demo setup script for file-organizer (fo)
# Creates a directory tree with files covering most usage scenarios:
#   - sort by extension / size / date (--copy / --move, -r, -a)
#   - find by name / ext / size / datetime / modified (exact & regex, -r, -a)
#   - clean (junk files, junk dirs, dotfiles, -r, -a)
# =============================================================================

DEMO_ROOT="fo_demo"

if [[ -d "$DEMO_ROOT" ]]; then
    echo "Removing existing '$DEMO_ROOT' directory..."
    rm -rf "$DEMO_ROOT"
fi

echo "Creating demo directory tree in '$DEMO_ROOT'..."
mkdir -p "$DEMO_ROOT"

# ─── Helper ──────────────────────────────────────────────────────────────────

# make_file <path> [size_in_bytes]
#   Creates a file with random content of the given size (default 64).
make_file() {
    local path="$1"
    local size="${2:-64}"
    mkdir -p "$(dirname "$path")"
    head -c "$size" /dev/urandom > "$path"
}

# make_text <path> <content>
make_text() {
    local path="$1"; shift
    mkdir -p "$(dirname "$path")"
    printf '%s\n' "$@" > "$path"
}

# =============================================================================
# 1. SORT scenarios  (fo sort <dir> --sort-by <ext|size|date> --copy/--move)
# =============================================================================
SORT_DIR="$DEMO_ROOT/sort_demo"

# --- 1a. Sort by extension ------------------------------------------------
EXT_DIR="$SORT_DIR/by_extension"
# Images
make_file "$EXT_DIR/photo_beach.jpg"       4096
make_file "$EXT_DIR/screenshot.png"        2048
make_file "$EXT_DIR/logo.gif"              1024
make_file "$EXT_DIR/banner.webp"           3072

# Documents
make_text "$EXT_DIR/report.pdf"            "fake-pdf-content"
make_text "$EXT_DIR/notes.txt"             "Some important notes"
make_text "$EXT_DIR/readme.md"             "# README"
make_text "$EXT_DIR/data.csv"              "a,b,c"

# Code
make_text "$EXT_DIR/main.rs"               "fn main() {}"
make_text "$EXT_DIR/app.py"                "print('hello')"
make_text "$EXT_DIR/index.js"              "console.log('hi')"
make_text "$EXT_DIR/style.css"             "body { margin: 0; }"
make_text "$EXT_DIR/page.html"             "<html></html>"

# Archives
make_file "$EXT_DIR/backup.zip"            512
make_file "$EXT_DIR/source.tar.gz"         768

# No extension
make_text "$EXT_DIR/Makefile"              "all: build"
make_text "$EXT_DIR/Dockerfile"            "FROM ubuntu"
make_text "$EXT_DIR/LICENSE"               "MIT License"

# --- 1b. Sort by size ----------------------------------------------------
SIZE_DIR="$SORT_DIR/by_size"
# Tiny   < 1 KB  → 0-99 KB bucket
make_file "$SIZE_DIR/tiny.txt"             100
# Small  ~10 KB  → 0-99 KB
make_file "$SIZE_DIR/small.log"            10240
# Medium ~50 KB  → 0-99 KB
make_file "$SIZE_DIR/medium.dat"           51200
# Large  ~150 KB → 100-199 KB
make_file "$SIZE_DIR/large.bin"            153600
# XLarge ~500 KB → 400-499 KB
make_file "$SIZE_DIR/xlarge.iso"           512000
# Mega   ~1.5 MB → 0-99 MB
make_file "$SIZE_DIR/mega.dump"            1572864

# --- 1c. Sort by date (files with different creation times) ---------------
DATE_DIR="$SORT_DIR/by_date"
make_text "$DATE_DIR/today_1.txt"          "created now"
make_text "$DATE_DIR/today_2.log"          "also created now"
make_text "$DATE_DIR/today_3.rs"           "fn today() {}"
# (all will land in today's date bucket; historical dates need real FS tricks)

# --- 1d. Recursive sort ---------------------------------------------------
REC_DIR="$SORT_DIR/recursive"
make_text "$REC_DIR/root_file.txt"            "root"
make_text "$REC_DIR/inner/nested_code.py"     "import os"
make_text "$REC_DIR/inner/nested_doc.pdf"     "pdf-data"
make_file "$REC_DIR/inner/deep/big.bin"        204800
make_file "$REC_DIR/inner/deep/small.txt"      256

# --- 1e. Hidden files in sort (--all) ------------------------------------
HIDDEN_DIR="$SORT_DIR/with_hidden"
make_text "$HIDDEN_DIR/visible.txt"        "I am visible"
make_text "$HIDDEN_DIR/.hidden_config"     "secret=42"
make_text "$HIDDEN_DIR/.env"               "DB_HOST=localhost"
make_file "$HIDDEN_DIR/.hidden_data.bin"   8192
make_text "$HIDDEN_DIR/normal.rs"          "fn main() {}"

# =============================================================================
# 2. FIND scenarios  (fo find <dir> -n/-e/-s/-d/-m [--regex] [-r] [-a])
# =============================================================================
FIND_DIR="$DEMO_ROOT/find_demo"

# --- 2a. Find by name (exact & regex) ------------------------------------
NAME_DIR="$FIND_DIR/by_name"
make_text "$NAME_DIR/report_2024.txt"      "Annual report 2024"
make_text "$NAME_DIR/report_2025.txt"      "Annual report 2025"
make_text "$NAME_DIR/summary.txt"          "Summary"
make_text "$NAME_DIR/report_final.pdf"     "Final report"
make_text "$NAME_DIR/notes.md"             "Meeting notes"
make_text "$NAME_DIR/README.md"            "# Project"

# --- 2b. Find by extension (exact & regex) --------------------------------
EXT_FIND_DIR="$FIND_DIR/by_ext"
make_text "$EXT_FIND_DIR/code.rs"          "fn main() {}"
make_text "$EXT_FIND_DIR/code.py"          "print('hi')"
make_text "$EXT_FIND_DIR/code.js"          "console.log(1)"
make_text "$EXT_FIND_DIR/code.ts"          "const x = 1;"
make_text "$EXT_FIND_DIR/data.json"        "{}"
make_text "$EXT_FIND_DIR/data.yaml"        "key: value"
make_text "$EXT_FIND_DIR/data.xml"         "<root/>"
make_text "$EXT_FIND_DIR/image.png"        "png-bytes"
make_text "$EXT_FIND_DIR/image.jpg"        "jpg-bytes"

# --- 2c. Find by size (exact & regex) ------------------------------------
SIZE_FIND_DIR="$FIND_DIR/by_size"
make_file "$SIZE_FIND_DIR/exact_1024.dat"  1024
make_file "$SIZE_FIND_DIR/exact_512.dat"   512
make_file "$SIZE_FIND_DIR/exact_2048.dat"  2048
make_file "$SIZE_FIND_DIR/exact_256.dat"   256
make_file "$SIZE_FIND_DIR/exact_4096.dat"  4096

# --- 2d. Find recursive ---------------------------------------------------
REC_FIND_DIR="$FIND_DIR/recursive"
make_text "$REC_FIND_DIR/top.txt"              "top level"
make_text "$REC_FIND_DIR/sub1/a.txt"           "sub1 a"
make_text "$REC_FIND_DIR/sub1/b.rs"            "fn b() {}"
make_text "$REC_FIND_DIR/sub1/sub2/c.txt"      "deep file"
make_text "$REC_FIND_DIR/sub1/sub2/d.py"       "def d(): pass"
make_text "$REC_FIND_DIR/other/e.txt"          "other"

# --- 2e. Find hidden files (-a) -------------------------------------------
HIDDEN_FIND_DIR="$FIND_DIR/hidden"
make_text "$HIDDEN_FIND_DIR/visible.txt"        "visible"
make_text "$HIDDEN_FIND_DIR/.gitignore"         "target/"
make_text "$HIDDEN_FIND_DIR/.env"               "SECRET=abc"
make_text "$HIDDEN_FIND_DIR/.config.yaml"       "setting: true"
make_text "$HIDDEN_FIND_DIR/normal.rs"          "fn main() {}"

# =============================================================================
# 3. CLEAN scenarios  (fo clean <dir> [-r] [-a])
# =============================================================================
CLEAN_DIR="$DEMO_ROOT/clean_demo"

# --- 3a. Junk files by pattern --------------------------------------------
make_text "$CLEAN_DIR/.DS_Store"           ""
make_text "$CLEAN_DIR/Thumbs.db"           "thumbnail-cache"
make_text "$CLEAN_DIR/desktop.ini"         "[.ShellClassInfo]"

# --- 3b. Junk files by extension ------------------------------------------
make_file "$CLEAN_DIR/crash.core"          128
make_file "$CLEAN_DIR/session.tmp"         64
make_file "$CLEAN_DIR/backup.bak"          256
make_file "$CLEAN_DIR/old_version.old"     512
make_file "$CLEAN_DIR/editor.swp"          64
make_file "$CLEAN_DIR/editor.swo"          64
make_text "$CLEAN_DIR/app.log"             "2024-01-01 ERROR something"
make_file "$CLEAN_DIR/program.pyc"         128
make_file "$CLEAN_DIR/Module.class"        256
make_file "$CLEAN_DIR/debug.ilk"           512
make_file "$CLEAN_DIR/autosave.autosave"   128
make_file "$CLEAN_DIR/stack.stackdump"     64
make_file "$CLEAN_DIR/core.dmp"            256

# --- 3c. Files ending with ~ (editor backups) ----------------------------
make_text "$CLEAN_DIR/document.txt~"       "old version"
make_text "$CLEAN_DIR/config.yaml~"        "outdated"

# --- 3d. Log rotation pattern (*.log.*) -----------------------------------
make_text "$CLEAN_DIR/app.log.1"           "rotated log 1"
make_text "$CLEAN_DIR/app.log.2"           "rotated log 2"
make_text "$CLEAN_DIR/error.log.2024-01-01" "dated log"

# --- 3e. Junk directories -------------------------------------------------
mkdir -p "$CLEAN_DIR/tmp"
mkdir -p "$CLEAN_DIR/__pycache__"
mkdir -p "$CLEAN_DIR/.cache"
mkdir -p "$CLEAN_DIR/build"
mkdir -p "$CLEAN_DIR/dist"
mkdir -p "$CLEAN_DIR/target"
mkdir -p "$CLEAN_DIR/.gradle"
mkdir -p "$CLEAN_DIR/.pytest_cache"
mkdir -p "$CLEAN_DIR/.mypy_cache"
mkdir -p "$CLEAN_DIR/.next"
mkdir -p "$CLEAN_DIR/node_modules_fake"  # NOT junk — should survive

# Put some content inside junk dirs so they have size
make_file "$CLEAN_DIR/tmp/tempfile.dat"             1024
make_file "$CLEAN_DIR/__pycache__/module.cpython-311.pyc" 2048
make_file "$CLEAN_DIR/.cache/thumbnails.dat"        4096
make_file "$CLEAN_DIR/build/output.o"               8192
make_file "$CLEAN_DIR/target/debug_binary"          16384
make_file "$CLEAN_DIR/.pytest_cache/v/cache/lastfailed" 64

# --- 3f. Legitimate files that must NOT be deleted -------------------------
make_text "$CLEAN_DIR/important.txt"       "Do not delete me!"
make_text "$CLEAN_DIR/project.rs"          "fn main() { println!(\"safe\"); }"
make_text "$CLEAN_DIR/data.csv"            "id,name\n1,Alice"

# --- 3g. Recursive clean --------------------------------------------------
CLEAN_REC_DIR="$CLEAN_DIR/nested_project"
make_text "$CLEAN_REC_DIR/src/main.rs"              "fn main() {}"
make_file "$CLEAN_REC_DIR/src/backup.bak"           128
make_file "$CLEAN_REC_DIR/temp_file.tmp"            64
make_text "$CLEAN_REC_DIR/.DS_Store"                ""
mkdir -p  "$CLEAN_REC_DIR/__pycache__"
make_file "$CLEAN_REC_DIR/__pycache__/mod.pyc"      512
mkdir -p  "$CLEAN_REC_DIR/build"
make_file "$CLEAN_REC_DIR/build/artifact.o"         1024
make_text "$CLEAN_REC_DIR/README.md"                "# Nested"

# --- 3h. Hidden junk (needs -a flag) --------------------------------------
make_text "$CLEAN_DIR/.hidden_junk.tmp"    "hidden temp"
make_text "$CLEAN_DIR/.backup.bak"         "hidden backup"

# =============================================================================
# Summary
# =============================================================================

file_count=$(find "$DEMO_ROOT" -type f | wc -l)
dir_count=$(find "$DEMO_ROOT" -type d | wc -l)
total_size=$(du -sh "$DEMO_ROOT" | cut -f1)

echo ""
echo "===  Demo setup complete  ==="
echo "  Root:        $DEMO_ROOT/"
echo "  Files:       $file_count"
echo "  Directories: $dir_count"
echo "  Total size:  $total_size"
echo ""
echo "=== Example commands to try ==="
echo ""
echo "# ── SORT ──"
echo "fo sort $SORT_DIR/by_extension --sort-by ext --copy"
echo "fo sort $SORT_DIR/by_size      --sort-by size --copy"
echo "fo sort $SORT_DIR/by_date      --sort-by date --copy"
echo "fo sort $SORT_DIR/recursive    --sort-by ext --copy -r"
echo "fo sort $SORT_DIR/with_hidden  --sort-by ext --copy -a"
echo ""
echo "# ── FIND ──"
echo "fo find $FIND_DIR/by_name -n report_2024.txt"
echo "fo find $FIND_DIR/by_name -n 'report_\d+' --regex"
echo "fo find $FIND_DIR/by_ext  -e rs"
echo "fo find $FIND_DIR/by_ext  -e 'rs|py|js' --regex"
echo "fo find $FIND_DIR/by_size -s 1024"
echo "fo find $FIND_DIR/by_size -s '(512|2048)' --regex"
echo "fo find $FIND_DIR/recursive  -e txt -r"
echo "fo find $FIND_DIR/hidden     -e txt -a"
echo ""
echo "# ── CLEAN ──"
echo "fo clean $CLEAN_DIR"
echo "fo clean $CLEAN_DIR -r"
echo "fo clean $CLEAN_DIR -r -a"