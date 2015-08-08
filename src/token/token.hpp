#ifndef PECAN_TOKEN_TOKEN_HPP_INCLUDED
#define PECAN_TOKEN_TOKEN_HPP_INCLUDED

#include <string>

enum TokenType {
  INVALID,
  IDENTIFIER,
  OPERATOR,
  KEYWORD,
  CHARACTER,
  NUMERIC,
  EOFTOKEN,
};

class Token {
protected:
  const int line, col;
  Token(int line, int col, TokenType type)
    : line(line), col(col), type(type) { }
  
public:
  const TokenType type = INVALID;
  virtual std::string show() = 0;

  virtual ~Token() { }
};

#endif
