#ifndef PECAN_TOKEN_OPERATOR_TOKEN_HPP_INCLUDED
#define PECAN_TOKEN_OPERATOR_TOKEN_HPP_INCLUDED

#include "token.hpp"

#include <string>

class OperatorToken : public Token {
  const std::string value;
public:
  virtual std::string show();
  OperatorToken(int line, int col, std::string value)
    : Token(line, col, OPERATOR), value(value) { }

  virtual ~OperatorToken() { }
};

#endif
