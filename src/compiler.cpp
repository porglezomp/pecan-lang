#include <iostream>
#include <fstream>

#include "tokenizer.hpp"

int main(int argc, char **argv)
{
  if (argc < 2) {
    std::cerr << "Usage: " << argv[0] << " file" << std::endl;
    return 1;
  }
  
  char *ifname = argv[1];
  std::ifstream file(ifname);
  auto tokenizer = Tokenizer(file);
  while (tokenizer.peek()->type != EOFTOKEN) {
    std::cout << tokenizer.peek()->show() << std::endl;
    tokenizer.advance();
  }
}
