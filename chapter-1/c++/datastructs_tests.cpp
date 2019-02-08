#define BOOST_TEST_DYN_LINK
#define BOOST_TEST_MODULE DataStructs_tests
#include <boost/test/unit_test.hpp>
#include <iostream>
#include "DataStructs.cpp"

BOOST_AUTO_TEST_SUITE(datastructs_transformation_tests)

BOOST_AUTO_TEST_CASE(initialize_bijection)
{
    // Initialise sets.
    std::vector<int> x;
    for (int i = 1; i < 17; i++) {
      x.push_back(i);
    }

    BOOST_CHECK_EQUAL(x.size(), 16);

    int yInts[] = {3, 9, 10, 13, 5, 15, 11, 16, 14, 8, 7, 4, 12, 2, 6, 1};
    std::vector<int> y(yInts, yInts + sizeof(yInts) / sizeof(int));

    BOOST_CHECK_EQUAL(y.size(), 16);
    
    Bijection bijection = Bijection(x, y);
    BOOST_CHECK(&bijection);
};

BOOST_AUTO_TEST_CASE(transform_sets)
{
    // Initialise sets.
    std::vector<int> x;
    for (int i = 1; i < 17; i++) {
      x.push_back(i);
    }

    BOOST_CHECK_EQUAL(x.size(), 16);

    int yInts[] = {3, 9, 10, 13, 5, 15, 11, 16, 14, 8, 7, 4, 12, 2, 6, 1};
    std::vector<int> y(yInts, yInts + sizeof(yInts) / sizeof(int));

    BOOST_CHECK_EQUAL(y.size(), 16);
    
    Bijection bijection = Bijection(x, y);
    BOOST_CHECK(&bijection);

    // Transform x to y.
    bool transformed = bijection.transform();
    BOOST_CHECK_EQUAL(transformed, true);
};
BOOST_AUTO_TEST_SUITE_END(); // datastructs_transformation_tests
