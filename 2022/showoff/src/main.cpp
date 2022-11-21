#include <iostream>

using namespace std;

enum IPV
{
  ipv4,
  ipv6,
};

int main()
{
  IPV address{ ipv4 } ;
  IPV someVar = IPV::ipv4; 
  cout << "The number for %d is: " << someVar << endl;

  return 0;
}
