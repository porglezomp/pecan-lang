#ifndef PECAN_TOKEN_IDENT_TOKEN_HPP_INCLUDED
#define PECAN_TOKEN_IDENT_TOKEN_HPP_INCLUDED

#include "token.hpp"

#include <string>

class IdentToken : public Token {
  const std::string value;
public:
  const TokenType type = IDENTIFIER;
  IdentToken(int line, int col, std::string value)
    : Token(line, col), value(value) { }
  virtual ~IdentToken() { }
};

#endif
