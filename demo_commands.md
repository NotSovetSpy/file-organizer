# Demonstration commands

## Setup demonstration environment

```sh
./demo_setup.sh
```

## ── DOCUMENTATION ──

```sh
fo --help
fo find -h
```

## ── FIND ──

```sh
fo find fo_demo/find_demo/by_name -n report_2024.txt
fo find fo_demo/find_demo/by_name -n 'report_\d+' --regex
fo find fo_demo/find_demo/by_ext  -e rs
fo find fo_demo/find_demo/by_ext  -e 'rs|py|js' --regex
fo find fo_demo/find_demo/by_size -s 1024
fo find fo_demo/find_demo/by_size -s '(512|2048)' --regex
fo find fo_demo/find_demo/recursive  -e txt -r
fo find fo_demo/find_demo/hidden     -e txt -a
```

## ── LOG LEVEL ──

```sh
fo find fo_demo/find_demo/by_name -n report_2024.txt
fo find fo_demo/find_demo/by_name -n report_2024.txt -v
fo find fo_demo/find_demo/by_name -n report_2024.txt --trace
```

## ── SORT ──

```sh
fo sort fo_demo/sort_demo/by_extension --sort-by ext --copy
fo sort fo_demo/sort_demo/by_size      --sort-by size --copy
fo sort fo_demo/sort_demo/by_date      --sort-by date --move
fo sort fo_demo/sort_demo/recursive    --sort-by ext --copy -r
fo sort fo_demo/sort_demo/with_hidden  --sort-by ext --copy -a
```

## ── CLEAN ──

```sh
fo clean fo_demo/clean_demo
fo clean fo_demo/clean_demo -r
fo clean fo_demo/clean_demo -r -a
```
