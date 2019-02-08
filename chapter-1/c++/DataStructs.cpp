#include "DataStructs.h"
#include <math.h>

// Public.
Bijection::Bijection(std::vector<int>& x, std::vector<int>& y): x(x), y(y) 
{
  order = 17;
  base = 3;
}

// Map f: X -> Y
bool Bijection::transform() 
{
  // Transformed Image of X.
  std::vector<int> image;

  // Transform each member of the set X.
  for (int eachX : this->x) {
      int powOutput = pow(this->base, eachX);
      int output = powOutput % this->order;

      image.push_back(output);
  }

  // Compare the image and set y for expected equality.
  for (int i = 1; i < 16; i++) {
    if (this->y[i] != image[i]) {
      return false;
    }
  }

  return true;
}
