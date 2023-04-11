#!/usr/bin/bash
git pull origin master
rm .gitignore
find * -size +50M | cat >> .gitignore
git add --all
git commit -m "$1"
git push origin master
