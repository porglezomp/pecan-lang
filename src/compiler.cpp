#include <iostream>
#include <fstream>

#include "lexer.hpp"

int main(int argc, char **argv)
{
  if (argc < 2) {
    std::cerr << "Usage: " << argv[0] << " file" << std::endl;
    return 1;
  }
  
  char *ifname = argv[1];
  std::ifstream file(ifname);
  auto lexer = Lexer(file);
  while (lexer.peek()->type != EOFTOKEN) {
    std::cout << lexer.peek()->show() << " ";
    lexer.advance();
  }
  std::cout << std::endl;
}

