#ifndef Bijection_H
#define Bijection_H
#include <iostream>
#include <cmath>

class Bijection 
{
    private:
      int order;
      int base;
      std::vector<int>& x;
      std::vector<int>& y;

    public:
      // Constructor.
      Bijection(std::vector<int>& x, std::vector<int>& y);
      bool transform();
};

#endif // Bijection_H
