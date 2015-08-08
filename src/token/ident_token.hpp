#ifndef PECAN_TOKEN_IDENT_TOKEN_HPP_INCLUDED
#define PECAN_TOKEN_IDENT_TOKEN_HPP_INCLUDED

#include "token.hpp"

#include <string>

class IdentToken : public Token {
  const std::string value;
public:
  virtual std::string show();
  IdentToken(int line, int col, std::string value)
    : Token(line, col, IDENTIFIER), value(value) { }

  virtual ~IdentToken() { }
};

#endif
