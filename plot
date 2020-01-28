#!/usr/bin/env gnuplot -c

set key off
set term png
set datafile separator ","
set output ARG2

set grid

set title 'Intensity of Receiver Disturbance over Time'

set xlabel 'time (s)'
set ylabel 'observed intensity (W/m^2) (log_{10} scale)'

set logscale y 10

plot ARG1 using 1:2 with points
