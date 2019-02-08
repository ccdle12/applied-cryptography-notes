#!/bin/bash

g++ -o datastructs_tests -lboost_unit_test_framework ./datastructs_tests.cpp && ./datastructs_tests --log_level=test_suite

