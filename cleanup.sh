#!/bin/bash
find . -name "*.aux" | xargs rm -rf
find . -name "*.fdb_latexmk" | xargs rm -rf
find . -name "*.fls" | xargs rm -rf
find . -name "*.log" | xargs rm -rf
find . -name "*.markdown.in" | xargs rm -rf
find . -name "*.out" | xargs rm -rf
find . -name "main.*.tex" | xargs rm -rf
rm -rf svg-inkscape
rm -rf _markdown_*
rm -rf latex
