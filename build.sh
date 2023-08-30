#!/bin/bash
cd tool && cargo run && cd ..
latexmk -shell-escape -lualatex main.en.tex
latexmk -shell-escape -lualatex main.nl.tex
