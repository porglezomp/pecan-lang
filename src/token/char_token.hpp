#ifndef PECAN_TOKEN_CHAR_TOKEN_HPP_INCLUDED
#define PECAN_TOKEN_CHAR_TOKEN_HPP_INCLUDED

#include "token.hpp"

#include <string>

class CharToken : public Token {
  char value;
public:
  virtual std::string show();
  CharToken(int line, int col, char value)
    : Token(line, col, CHARACTER), value(value) { }

  virtual ~CharToken() { }
};

#endif
