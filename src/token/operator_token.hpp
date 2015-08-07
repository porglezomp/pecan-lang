#ifndef PECAN_TOKEN_OPERATOR_TOKEN_HPP_INCLUDED
#define PECAN_TOKEN_OPERATOR_TOKEN_HPP_INCLUDED

#include "token.hpp"

#include <string>

class OperatorToken : public Token {
  const std::string value;
public:
  const TokenType type = OPERATOR;
  OperatorToken(int line, int col, std::string value)
    : Token(line, col), value(value) { }
  virtual ~OperatorToken() { }
};

#endif
