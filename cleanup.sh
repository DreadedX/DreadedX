#!/bin/bash
find . -name "*.aux" | xargs rm -rf
find . -name "*.fdb_latexmk" | xargs rm -rf
find . -name "*.fls" | xargs rm -rf
find . -name "*.log" | xargs rm -rf
find . -name "*.markdown.in" | xargs rm -rf
find . -name "*.out" | xargs rm -rf
rm -rf _markdown_main
rm -rf latex
