#!/usr/bin/env bash
num=10

name=module4

if $(gcc -fopenmp $name.c -o $name); then
    echo $name
    ./$name $num
fi